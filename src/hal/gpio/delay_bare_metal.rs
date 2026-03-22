use cortex_m::asm;
use crate::ffi::cmu;

pub struct DelayBareMetal;

impl DelayBareMetal {
    const FALLBACK_CORE_HZ: u32 = 19_000_000;

    pub fn delay_ms(ms: u32) {
        Self::delay_ms_at_hz(ms, Self::core_clock_hz());
    }

    pub fn delay_ms_at_hz(ms: u32, core_hz: u32) {
        let cycles_per_ms = (core_hz / 1_000) as u64;
        let total_cycles = cycles_per_ms.saturating_mul(ms as u64);
        Self::delay_cycles(total_cycles);
    }

    fn delay_cycles(mut cycles: u64) {
        while cycles != 0 {
            let chunk = core::cmp::min(cycles, u32::MAX as u64) as u32;
            asm::delay(chunk);
            cycles -= chunk as u64;
        }
    }

    fn core_clock_hz() -> u32 {
        let hz = unsafe { cmu::cmu_wrap_core_clock_hz() };
        if hz == 0 {
            Self::FALLBACK_CORE_HZ
        } else {
            hz
        }
    }
}
