//! Open-drain and open-source outputs.
//!
//! Open-drain drives low and releases high, so it needs a pull-up to reach a
//! high level — this is how a shared bus line like I2C SDA behaves. Open-source
//! (the MG24's wired-or) is the mirror image: it drives high and releases low.
//!
//! Wiring: nothing required. `into_flex()` reads the pad back, so you can see
//! an open-drain pin read low when something external holds it down.

#![no_std]
#![no_main]

use mg24_hal::{
    CpuConfig,
    delay::init_delay,
    gpio::{DriveMode, Level, Output, OutputConfig, Pull},
};

#[mg24_hal::main]
fn main() -> ! {
    let dp = mg24_hal::init(CpuConfig::default()).unwrap();
    let delay = init_delay();

    // Open-drain with the internal pull-up: high is released, not driven.
    let mut sda = Output::new(
        dp.pins.pc4,
        Level::High,
        OutputConfig::default()
            .with_drive_mode(DriveMode::OpenDrain)
            .with_pull(Pull::Up),
    );

    // Open-source with a pull-down: the opposite arrangement.
    let mut wired_or = Output::new(
        dp.pins.pc5,
        Level::Low,
        OutputConfig::default()
            .with_drive_mode(DriveMode::OpenSource)
            .with_pull(Pull::Down),
    );

    loop {
        sda.set_low();
        wired_or.set_high();
        delay.delay_ms(250);

        sda.set_high();
        wired_or.set_low();
        delay.delay_ms(250);
    }
}
