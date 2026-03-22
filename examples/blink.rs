#![no_std]
#![no_main]

use cortex_m_rt::entry;
use mg24_hal::hal::gpio::delay_bare_metal::DelayBareMetal;
use mg24_hal::hal::gpio::outputgpio::OutputPin;
use mg24_hal::hal::pac::Prehirpals;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let dp = Prehirpals::take();
    let led = OutputPin::new(dp.pins.d1);
    loop {
        led.write_high();
        DelayBareMetal::delay_ms(500);
        led.write_low();
        DelayBareMetal::delay_ms(500);
    }
}
