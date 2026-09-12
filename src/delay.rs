//! Blocking delays driven by the Cortex-M33 SysTick timer.
//!
//! ```ignore
//! use mg24_hal::delay::Delay;
//!
//! let delay = Delay::new();
//! delay.delay_ms(500);
//! ```
//!
//! [`Delay`] is zero-sized and reads [`crate::clock::sysclk_hz`] on every call,
//! so one built before a [`crate::clock::CpuSpeed`] change stays correct
//! afterwards.

use core::ptr;

pub use crate::clock::DEFAULT_SYSCLK_HZ;

const SYST_CSR: *mut u32 = 0xE000_E010 as *mut u32;
const SYST_RVR: *mut u32 = 0xE000_E014 as *mut u32;
const SYST_CVR: *mut u32 = 0xE000_E018 as *mut u32;

const CSR_ENABLE: u32 = 1 << 0;
const CSR_CLKSOURCE: u32 = 1 << 2;
const CSR_COUNTFLAG: u32 = 1 << 16;

/// SysTick's reload register is 24 bits wide.
const MAX_RELOAD: u32 = 0x00FF_FFFF;

/// A blocking delay source.
///
/// Only one should be spinning at a time: they all drive the same SysTick
/// hardware, so a delay inside an interrupt handler would corrupt one already
/// running underneath it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Delay;

impl Delay {
    /// Builds a delay for whatever speed the core is running at.
    pub const fn new() -> Self {
        Delay
    }

    /// Blocks for at least `ms` milliseconds.
    pub fn delay_millis(&self, ms: u32) {
        self.spin(self.ticks(ms, 1_000));
    }

    /// Blocks for at least `us` microseconds.
    pub fn delay_micros(&self, us: u32) {
        self.spin(self.ticks(us, 1_000_000));
    }

    /// Blocks for at least `ns` nanoseconds.
    pub fn delay_nanos(&self, ns: u32) {
        self.spin(self.ticks(ns, 1_000_000_000));
    }

    /// Blocks for at least `ms` milliseconds. Alias of [`Delay::delay_millis`].
    pub fn delay_ms(&self, ms: u32) {
        self.delay_millis(ms);
    }

    /// Blocks for at least `us` microseconds. Alias of [`Delay::delay_micros`].
    pub fn delay_us(&self, us: u32) {
        self.delay_micros(us);
    }

    /// Blocks for at least `ns` nanoseconds. Alias of [`Delay::delay_nanos`].
    pub fn delay_ns(&self, ns: u32) {
        self.delay_nanos(ns);
    }

    /// Converts `count` units of a `per_second`-Hz unit into core clock ticks.
    ///
    /// Rounds up, so a delay is never shorter than asked for. The intermediate
    /// product needs 64 bits: `u32::MAX` milliseconds at 64 MHz is about
    /// 2.7e17, far past what a `u32` holds.
    fn ticks(&self, count: u32, per_second: u32) -> u64 {
        let hz = u64::from(crate::clock::sysclk_hz());
        (u64::from(count) * hz).div_ceil(u64::from(per_second))
    }

    /// Counts down `ticks` core clock cycles, in chunks SysTick can hold.
    fn spin(&self, mut ticks: u64) {
        while ticks > 0 {
            // A reload value of N counts N+1 cycles, so one pass covers up to
            // MAX_RELOAD + 1 ticks.
            let chunk = ticks.min(u64::from(MAX_RELOAD) + 1) as u32;
            ticks -= u64::from(chunk);

            unsafe {
                ptr::write_volatile(SYST_RVR, chunk - 1);
                // Writing CVR clears it *and* clears COUNTFLAG, so the poll
                // below cannot see a flag left over from the previous chunk.
                ptr::write_volatile(SYST_CVR, 0);
                ptr::write_volatile(SYST_CSR, CSR_CLKSOURCE | CSR_ENABLE);

                while ptr::read_volatile(SYST_CSR) & CSR_COUNTFLAG == 0 {}

                ptr::write_volatile(SYST_CSR, 0);
            }
        }
    }
}

// The trait methods take `&mut self` while the inherent ones take `&self`.
// Inherent methods win at a call site, so `delay.delay_ms(..)` on a shared
// binding keeps working and this impl is what generic drivers get.
impl embedded_hal::delay::DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) {
        Delay::delay_nanos(self, ns);
    }

    fn delay_us(&mut self, us: u32) {
        Delay::delay_micros(self, us);
    }

    fn delay_ms(&mut self, ms: u32) {
        Delay::delay_millis(self, ms);
    }
}
