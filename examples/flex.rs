//! A pin whose direction changes at runtime.
//!
//! `Flex` is the layer `Input` and `Output` are built on. It is what you want
//! for a half-duplex line — one wire that is driven for part of a cycle and
//! sensed for the rest.
//!
//! Here PC2 drives a pulse, then flips to an input to read the line back, and
//! PA7 mirrors whatever it read.
//!
//! Wiring: nothing required; PC2 with a pull-up reads high when nothing else
//! drives it.

#![no_std]
#![no_main]

use mg24_hal::{
    CpuConfig,
    delay::init_delay,
    gpio::{Flex, InputConfig, Level, Output, OutputConfig, Pull},
};

#[mg24_hal::main]
fn main() -> ! {
    let dp = mg24_hal::init(CpuConfig::default()).unwrap();
    let delay = init_delay();

    let mut led = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());

    let mut line = Flex::new(dp.pins.pc2);
    line.apply_output_config(&OutputConfig::default());
    line.apply_input_config(&InputConfig::default().with_pull(Pull::Up));

    loop {
        // Drive a short low pulse.
        line.set_output_enable(true);
        line.set_low();
        delay.delay_ms(10);

        // Release the line and sense it instead. Output wins over input while
        // both are on, so the driver has to come off first.
        line.set_output_enable(false);
        line.set_input_enable(true);
        delay.delay_ms(1);

        led.set_level(line.level());

        delay.delay_ms(250);
    }
}
