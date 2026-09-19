#![no_std]
#![no_main]

use mg24_hal::{CpuConfig, delay::Delay};

#[mg24_hal::main]
fn main() -> ! {
    let _dp = mg24_hal::init(CpuConfig::default()).unwrap();
    let delay = Delay::new();

    loop {
        delay.delay_ms(1000);
    }
}
