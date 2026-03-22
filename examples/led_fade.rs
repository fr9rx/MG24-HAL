#![no_std]
#![no_main]

use cortex_m_rt::entry;
use mg24_hal::hal::pac::Prehirpals;
use mg24_hal::hal::pwm::outputpwm::{PreScale, Pwm, PwmConfig};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let dp = Prehirpals::take();
    let led = Pwm::new(
        dp.pins.d1,
        0,
        0,
        PwmConfig {
            prescale: PreScale::Div16,
            frequencyhz: 1190,
            dutyprestart: 0,
            invertoutput: false,
        },
    );
    loop {
        for v in 0..4095 {
            led.write(v);
        }

        for v in 4095..0 {
            led.write(v);
        }
    }
}
