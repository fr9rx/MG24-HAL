use cortex_m::asm;

const FALLBACK_CORE_HZ: u32 = 19_000_000;

pub fn delay_ms(ms: u32) {
    asm::delay(ms * FALLBACK_CORE_HZ);
}
