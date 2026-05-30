#![no_std]
#![no_main]

use cortex_m_rt::entry;
use mg24_hal::{CpuConfig, delay::init_delay, gpio::Gpio};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let dp = mg24_hal::init(CpuConfig::default()).unwrap();
    let mut led = Gpio::output(dp.pins.pa7).unwrap();
    let mut delay = init_delay();
    let delay_value = 500;
    loop {
        led.write_toggle().unwrap();
        delay.delay_ms(delay_value);
    }
}
