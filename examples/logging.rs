//! Logging over RTT.
//!
//! No pins and no wiring: RTT is a ring buffer in RAM that the debug probe
//! reads over the same SWD connection used to flash the chip.
//!
//! Run it with:
//!
//! ```text
//! probe-rs attach --chip EFR32MG24B220F1536IM48 <this binary>
//! ```

#![no_std]
#![no_main]

use mg24_hal::{
    CpuConfig, clock,
    delay::init_delay,
    gpio::{Level, Output, OutputConfig},
    rprintln, rtt,
};

#[mg24_hal::main]
fn main() -> ! {
    let dp = mg24_hal::init(CpuConfig::default()).unwrap();

    rtt::init();
    rprintln!("mg24-hal up, SYSCLK = {} Hz", clock::sysclk_hz());

    let mut led = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());
    let delay = init_delay();

    let mut ticks: u32 = 0;

    loop {
        led.toggle();
        rprintln!("tick {} led={:?}", ticks, led.output_level());
        ticks += 1;
        delay.delay_ms(500);
    }
}
