#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use mg24_hal::hal::gpio::delay_bare_metal::delay_ms;
use mg24_hal::hal::gpio::pindriver::PinDriver;
use mg24_hal::hal::pac::Prehirpals;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let dp = Prehirpals::take().unwrap();
    let mut led = PinDriver::output(dp.pins.GPIO1).unwrap();
    loop {
        led.write_high().unwrap();
        delay_ms(500);
        led.set_low().unwrap();
        delay_ms(500);
    }
}
