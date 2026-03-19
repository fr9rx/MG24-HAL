use cortex_m::asm;

pub struct DelayBareMetal;

impl DelayBareMetal {
    pub fn delay_ms(ms: u32) {
        asm::delay(ms * 78_000)
    }
}
