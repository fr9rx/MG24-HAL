#![no_std]
#![no_main]

use mg24_hal::{CpuConfig, i2c::{I2c0, I2cConfig}};

#[mg24_hal::main]
fn main() -> ! {
    let dp = mg24_hal::init(CpuConfig::default()).unwrap();

    let mut i2c = I2c0::new(
        dp.i2c0,
        dp.pins.pc0,  // SDA
        dp.pins.pc1,  // SCL
        I2cConfig::default().with_speed(400_000),
    );

    // Scan I2C bus for devices
    loop {
        for addr in 0x00..=0x7F {
            let mut buffer = [0u8; 1];
            if i2c.read(addr, &mut buffer).is_ok() {
                // Device found at address addr
                // Do something with it
            }
        }
    }
}
