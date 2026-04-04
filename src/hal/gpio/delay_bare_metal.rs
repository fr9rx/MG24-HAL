use crate::ffi::cmu;
use cortex_m::asm;

const FALLBACK_CORE_HZ: u32 = 19_000_000;

fn delay_cycles(mut cycles: u64) {
    while cycles != 0 {
        let chunk = core::cmp::min(cycles, u32::MAX as u64) as u32;
        asm::delay(chunk);
        cycles -= chunk as u64;
    }
}

pub fn delay_ms_at_hz(ms: u32, core_hz: u32) {
    let cycles_per_ms = (core_hz / 1_000) as u64;
    let total_cycles = cycles_per_ms.saturating_mul(ms as u64);
    delay_cycles(total_cycles);
}

fn core_clock_hz() -> u32 {
    let hz = unsafe { cmu::cmu_wrap_core_clock_hz() };
    if hz == 0 { FALLBACK_CORE_HZ } else { hz }
}

pub fn delay_ms(ms: u32) {
    delay_ms_at_hz(ms, core_clock_hz());
}
