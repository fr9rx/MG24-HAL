#![no_std]
#![no_main]

use mg24_hal::{
    CpuConfig,
    delay::init_delay,
    gpio::{Level, Output, OutputConfig},
};

#[mg24_hal::main]
fn main() -> ! {
    let dp = mg24_hal::init(CpuConfig::default()).unwrap();

    // PA07 is the onboard orange LED on the XIAO MG24.
    let mut led = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());
    let delay = init_delay();

    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
