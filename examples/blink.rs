#![no_std]
#![no_main]

use cortex_m_rt::entry;
use mg24_hal::hal::clocks::clock::Clock;
use mg24_hal::hal::gpio::delay_bare_metal::DelayBareMetal;
use mg24_hal::hal::gpio::outputgpio::OutputPin;
use mg24_hal::hal::pac::Prehirpals;
use panic_halt as _;

#[entry]
fn main() -> ! {
    Clock::start();
    let dp = Prehirpals::take();
    let buzzer = OutputPin::new(dp.pins.d0);
    loop {
        buzzer.set_high();
        DelayBareMetal::delay_ms(5);
        buzzer.set_low();
        DelayBareMetal::delay_ms(5);
    }
}
