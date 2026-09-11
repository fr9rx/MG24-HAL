//! Interrupt masking, via the `PRIMASK` register.
//!
//! This replaces the parts of `cortex_m::interrupt` the HAL relies on. The
//! GPIO code uses [`free`] to make its read-modify-write sequences atomic
//! against interrupt handlers touching the same port.

use core::sync::atomic::{Ordering, compiler_fence};

/// Masks all maskable interrupts.
#[inline]
pub fn disable() {
    unsafe {
        core::arch::asm!("cpsid i", options(nomem, nostack, preserves_flags));
    }
    // Nothing after this point may be hoisted above the mask.
    compiler_fence(Ordering::SeqCst);
}

/// Unmasks all maskable interrupts.
///
/// # Safety
///
/// Calling this inside a [`free`] closure breaks the critical section that
/// closure was promised.
#[inline]
pub unsafe fn enable() {
    // Nothing before this point may sink below the unmask.
    compiler_fence(Ordering::SeqCst);
    unsafe {
        core::arch::asm!("cpsie i", options(nomem, nostack, preserves_flags));
    }
}

/// Runs `f` with interrupts masked, then restores the previous mask state.
///
/// Nesting is safe: an inner `free` leaves interrupts masked on the way out if
/// they were already masked on the way in.
#[inline]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let primask: u32;
    unsafe {
        core::arch::asm!(
            "mrs {}, PRIMASK",
            out(reg) primask,
            options(nomem, nostack, preserves_flags),
        );
    }

    disable();
    let result = f();

    // Bit 0 set means interrupts were already masked before we got here, so
    // the caller still owns the mask and we leave it alone.
    if primask & 1 == 0 {
        unsafe { enable() };
    }

    result
}

// --------------------------------------------------------------------------
// Sharing state with a handler
// --------------------------------------------------------------------------

/// Shares a value between `main` and an interrupt handler.
///
/// Access runs inside [`free`], so the handler and `main` cannot be inside the
/// closure at the same time. Pair it with a [`core::cell::RefCell`] when the
/// value needs mutating — which is the usual case, since driver methods like
/// `clear_interrupt` take `&mut self`:
///
/// ```ignore
/// use core::cell::RefCell;
/// use mg24_hal::{gpio::Input, interrupt::Mutex};
///
/// static BUTTON: Mutex<RefCell<Option<Input<'C', 3>>>> = Mutex::new(RefCell::new(None));
///
/// // in main, after creating the pin:
/// BUTTON.lock(|b| b.borrow_mut().replace(button));
///
/// // in the handler:
/// BUTTON.lock(|b| {
///     if let Some(button) = b.borrow_mut().as_mut() {
///         button.clear_interrupt();
///     }
/// });
/// ```
pub struct Mutex<T> {
    inner: core::cell::UnsafeCell<T>,
}

// SAFETY: `lock` is the only way in, and it masks interrupts for the duration.
// On a single core that is enough to make access exclusive. `T: Send` because
// the value can be touched from a handler, which is a different context.
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    /// Wraps a value. `const`, so it can initialise a `static`.
    pub const fn new(value: T) -> Self {
        Mutex {
            inner: core::cell::UnsafeCell::new(value),
        }
    }

    /// Runs `f` with exclusive access to the value.
    ///
    /// Interrupts are masked for the duration, so keep the closure short.
    pub fn lock<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        free(|| {
            // SAFETY: interrupts are masked and this is a single-core part, so
            // no other context can be inside `lock` right now.
            f(unsafe { &*self.inner.get() })
        })
    }
}

// --------------------------------------------------------------------------
// NVIC
// --------------------------------------------------------------------------

/// NVIC Interrupt Set-Enable Registers.
const NVIC_ISER: *mut u32 = 0xE000_E100 as *mut u32;

/// NVIC Interrupt Clear-Enable Registers.
const NVIC_ICER: *mut u32 = 0xE000_E180 as *mut u32;

/// Even-numbered GPIO external interrupts.
pub const GPIO_EVEN: u8 = 26;

/// Odd-numbered GPIO external interrupts.
pub const GPIO_ODD: u8 = 25;

/// Enables an interrupt line in the NVIC.
///
/// Masking a peripheral's own interrupt is not enough on its own — the NVIC
/// line has to be enabled too, and this is what does it.
///
/// # Safety
///
/// The corresponding handler starts running as soon as the peripheral raises
/// it, which can preempt any critical section the caller is relying on.
#[inline]
pub unsafe fn enable_irq(irq: u8) {
    let index = (irq / 32) as usize;
    let bit = 1u32 << (irq % 32);
    // SAFETY: ISER is write-1-to-set, so this is a single word write that
    // cannot disturb other lines.
    unsafe { core::ptr::write_volatile(NVIC_ISER.add(index), bit) };
}

/// Disables an interrupt line in the NVIC.
#[inline]
pub fn disable_irq(irq: u8) {
    let index = (irq / 32) as usize;
    let bit = 1u32 << (irq % 32);
    // SAFETY: ICER is write-1-to-clear, so this is a single word write that
    // cannot disturb other lines.
    unsafe {
        core::ptr::write_volatile(NVIC_ICER.add(index), bit);
        // The architecture requires these before the disable is guaranteed to
        // have taken effect for subsequent instructions.
        core::arch::asm!("dsb", "isb", options(nomem, nostack, preserves_flags));
    }
}
