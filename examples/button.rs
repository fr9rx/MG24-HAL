#![no_std]
#![no_main]

use cortex_m_rt::entry;
use mg24_hal::hal::clocks::clock::Clock;
use mg24_hal::hal::gpio::inputgpio::{InputPin, Pull};
use mg24_hal::hal::gpio::outputgpio::OutputPin;
use mg24_hal::hal::pac::Prehirpals;
use panic_halt as _;

#[entry]
fn main() -> ! {
    Clock::start();
    let dp = Prehirpals::take();
    let led = OutputPin::new(dp.pins.d1);
    let button = InputPin::new(dp.pins.d3, Pull::Up);
    loop {
        if button.read_low() {
            led.write_high();
        } else {
            led.write_low();
        }
    }
}
