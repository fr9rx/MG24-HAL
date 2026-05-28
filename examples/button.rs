#![no_std]
#![no_main]

use cortex_m_rt::entry;
use mg24_hal::gpio::{Gpio, InputConfig, Pull};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let dp = mg24_hal::init().unwrap();
    let mut led = Gpio::output(dp.pins.pc1).unwrap();
    let config = InputConfig::new(Pull::Floating);
    let mut button = Gpio::input(dp.pins.pc3, config).unwrap();
    loop {
        if button.read() == false {
            led.write_high().unwrap();
        } else {
            led.write_low().unwrap();
        }
    }
}
