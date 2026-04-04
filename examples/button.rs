#![no_std]
#![no_main]

use cortex_m_rt::entry;
use mg24_hal::hal::gpio::pindriver::{InputConfig, PinDriver, Pull};
use mg24_hal::hal::pac::Prehirpals;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let dp = Prehirpals::take().unwrap();
    let mut led = PinDriver::output(dp.pins.GPIO1).unwrap();
    let config = InputConfig::new(Pull::Up);
    let mut button = PinDriver::input(dp.pins.GPIO3, config).unwrap();
    loop {
        if button.read().unwrap() == false {
            led.write_high().unwrap();
        } else {
            led.write_low().unwrap();
        }
    }
}
