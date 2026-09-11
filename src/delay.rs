//! Blocking delays driven by the Cortex-M33 SysTick timer.
//!
//! This replaces `cortex_m::delay::Delay` and talks to SysTick directly, so
//! nothing has to hand out ownership of the core peripherals first.

use core::ptr;

const SYST_CSR: *mut u32 = 0xE000_E010 as *mut u32;
const SYST_RVR: *mut u32 = 0xE000_E014 as *mut u32;
const SYST_CVR: *mut u32 = 0xE000_E018 as *mut u32;

const CSR_ENABLE: u32 = 1 << 0;
const CSR_CLKSOURCE: u32 = 1 << 2;
const CSR_COUNTFLAG: u32 = 1 << 16;

/// SysTick's reload register is 24 bits wide.
const MAX_RELOAD: u32 = 0x00FF_FFFF;

pub use crate::clock::DEFAULT_SYSCLK_HZ;

/// A blocking delay source.
///
/// Only one of these should be live at a time: they all drive the same SysTick
/// hardware, and a delay in an interrupt handler would corrupt one already
/// running underneath it.
pub struct Delay {
    sysclk_hz: u32,
}

impl Delay {
    /// Builds a delay for a core running at `sysclk_hz`.
    pub const fn new(sysclk_hz: u32) -> Self {
        Self { sysclk_hz }
    }

    /// Blocks for at least `us` microseconds.
    pub fn delay_us(&self, us: u32) {
        self.spin(self.ticks(us, 1_000_000));
    }

    /// Blocks for at least `ms` milliseconds.
    pub fn delay_ms(&self, ms: u32) {
        self.spin(self.ticks(ms, 1_000));
    }

    /// Blocks for at least `ns` nanoseconds.
    pub fn delay_ns(&self, ns: u32) {
        self.spin(self.ticks(ns, 1_000_000_000));
    }

    /// Converts `count` units of a `per_second`-Hz unit into core clock ticks.
    ///
    /// Rounds up, so a delay is never shorter than asked for. The intermediate
    /// product needs 64 bits: `u32::MAX` milliseconds at 19 MHz is about
    /// 8.1e16, far past what a `u32` holds.
    fn ticks(&self, count: u32, per_second: u32) -> u64 {
        (u64::from(count) * u64::from(self.sysclk_hz)).div_ceil(u64::from(per_second))
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

// These go through `spin`/`ticks` directly: the inherent methods share their
// names with the trait ones, and spelling out the private helpers keeps it
// obvious that this is not calling itself.
impl embedded_hal::delay::DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) {
        self.spin(self.ticks(ns, 1_000_000_000));
    }

    fn delay_us(&mut self, us: u32) {
        self.spin(self.ticks(us, 1_000_000));
    }

    fn delay_ms(&mut self, ms: u32) {
        self.spin(self.ticks(ms, 1_000));
    }
}

/// Builds a [`Delay`] for the clock the chip is actually running at.
///
/// Reads [`crate::clock::sysclk_hz`], so a delay built after
/// [`crate::init`] applied a [`crate::clock::CpuSpeed`] is calibrated for that
/// speed rather than the reset default.
pub fn init_delay() -> Delay {
    Delay::new(crate::clock::sysclk_hz())
}
