#![no_std]
#![no_main]

use mg24_hal::{
    rprintln_ts, rtt, timestamp,
    CpuConfig,
    delay::Delay,
    gpio::{Output, OutputConfig, Level},
    i2c::{I2c, I2cConfig, I2cSpeed, I2c0},
};

#[mg24_hal::main]
fn main() -> ! {
    rtt::init();
    timestamp::init();
    rprintln_ts!("=== Quick I2C Test ===\n");

    let dp = mg24_hal::init(CpuConfig::default().with_i2c_clock(true)).unwrap();
    let delay = Delay::new();

    // Enable IMU power
    let _imu_enable = Output::new(dp.pins.pd5, Level::High, OutputConfig::default());
    rprintln_ts!("✓ IMU power enabled");

    let mut led = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());

    // Test PC4/PC5
    rprintln_ts!("\nTesting I2C0 on PC4 (SDA) / PC5 (SCL)...");
    let mut i2c = I2c::<I2c0>::new(
        dp.pins.pc4,
        dp.pins.pc5,
        I2cConfig::default().with_speed(I2cSpeed::Standard),
    );

    let mut buf = [0u8; 1];
    match i2c.write_read(0x6A, &[0x0F], &mut buf) {
        Ok(()) => {
            rprintln_ts!("✓ SUCCESS! Read 0x{:02X} from address 0x6A", buf[0]);
            led.set_level(Level::High);
        }
        Err(e) => {
            rprintln_ts!("✗ Failed: {:?}", e);
        }
    }

    rprintln_ts!("\nDone. LED is on if success.");
    loop {
        delay.delay_ms(1000);
    }
}
