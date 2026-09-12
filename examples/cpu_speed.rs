//! Running the core at a chosen speed.
//!
//! `CpuSpeed` retunes HFRCODPLL to one of its factory-calibrated bands, using
//! the per-chip trim values from the DEVINFO page. `Delay` reads the
//! configured speed back, so the blink below is the same wall-clock rate at
//! every band — only the number of cycles behind it changes.
//!
//! Change `CpuSpeed::Mhz64` to `Mhz4` and the LED should keep blinking at the
//! same rate. That is the real test: the tick count scales with the requested
//! frequency, so the wall-clock rate only stays put if the hardware actually
//! retuned. A blink that comes out twice as slow means the delay scaled but the
//! oscillator did not.
//!
//! Wiring: none. The LED is the onboard one on PA7.

#![no_std]
#![no_main]

use mg24_hal::{
    CpuConfig,
    clock::{self, CpuSpeed},
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
};

#[mg24_hal::main]
fn main() -> ! {
    // The fastest band this crate offers; they run from 4 MHz up.
    let dp = mg24_hal::init(CpuConfig::default().with_cpu_speed(CpuSpeed::Mhz64)).unwrap();

    // Built after init, so it is calibrated for the new clock.
    let delay = Delay::new();

    let mut led = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());

    // `sysclk_hz` reports what the HAL applied, so this only catches a clock
    // change that was rejected outright — which `init` would already have
    // returned an error for. Whether the oscillator truly retuned is the blink
    // rate's job to show.
    let period = if clock::sysclk_hz() >= CpuSpeed::Mhz64.hz() {
        500
    } else {
        125
    };

    loop {
        led.toggle();
        delay.delay_ms(period);
    }
}
