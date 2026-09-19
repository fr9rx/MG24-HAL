#![no_std]
#![no_main]

use mg24_hal::{
    CpuConfig,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
};

#[mg24_hal::main]
fn main() -> ! {
    let dp = mg24_hal::init(CpuConfig::default()).unwrap();

    let mut led = Output::new(dp.pins.pc1, Level::Low, OutputConfig::default());
    let button = Input::new(dp.pins.pc3, InputConfig::default().with_pull(Pull::Up));

    loop {
        // Button is pulled up, reads low when pressed
        led.set_level(match button.level() {
            Level::Low => Level::High,
            Level::High => Level::Low,
        });
    }
}
