//! The default panic handler.
//!
//! Enabled by the `panic-handler` feature, which is on by default. Turn it off
//! (`default-features = false`) to supply your own `#[panic_handler]`, or to
//! use a crate like `panic-probe`.

use core::panic::PanicInfo;

/// Masks interrupts and spins.
///
/// Interrupts are masked first so a pending handler cannot carry on running
/// against whatever invalid state caused the panic. `rust_begin_unwind` stays
/// a real, non-inlined symbol, so it makes a convenient debugger breakpoint.
#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    crate::interrupt::disable();
    loop {
        core::hint::spin_loop();
    }
}
