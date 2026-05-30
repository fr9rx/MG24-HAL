#![no_std]
#![no_main]

use cortex_m_rt::entry;
use mg24_hal::{
    CpuConfig,
    gpio::{Gpio, InputConfig, Level, Pull},
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let dp = mg24_hal::init(CpuConfig::default()).unwrap();
    let mut led = Gpio::output(dp.pins.pc1).unwrap();
    let mut config = InputConfig::default();
    config.pull = Pull::Up;
    let mut button = Gpio::input(dp.pins.pc3, config).unwrap();
    loop {
        if button.read() == Level::Low {
            led.write_high().unwrap();
        } else {
            led.write_low().unwrap();
        }
    }
}
