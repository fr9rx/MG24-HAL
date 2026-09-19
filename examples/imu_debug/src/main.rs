#![no_std]
#![no_main]

use mg24_hal::{
    rprintln_ts, rtt, timestamp,
    CpuConfig,
    delay::Delay,
    gpio::{Output, OutputConfig, Level},
    i2c::{I2c, I2cConfig, I2cSpeed, I2c0},
};

const LSM6DS3_ADDR: u8 = 0x6A;
const WHO_AM_I: u8 = 0x0F;

#[mg24_hal::main]
fn main() -> ! {
    rtt::init();
    timestamp::init();
    rprintln_ts!("=== I2C IMU Debug - Finding correct pins ===\n");

    let dp = mg24_hal::init(CpuConfig::default().with_i2c_clock(true)).unwrap();
    let delay = Delay::new();

    // Enable IMU power (PD5)
    let _imu_enable = Output::new(dp.pins.pd5, Level::High, OutputConfig::default());
    rprintln_ts!("✓ IMU power enabled (PD5 = HIGH)");
    delay.delay_ms(100);

    // Try PC4/PC5 first (standard mg24-hal pins)
    rprintln_ts!("\n[Test 1] Trying I2C0 on PC4 (SDA) / PC5 (SCL)...");
    let mut i2c_pc = I2c::<I2c0>::new(
        dp.pins.pc4,
        dp.pins.pc5,
        I2cConfig::default().with_speed(I2cSpeed::Standard),
    );

    let mut who = [0u8; 1];
    match i2c_pc.write_read(LSM6DS3_ADDR, &[WHO_AM_I], &mut who) {
        Ok(()) => {
            rprintln_ts!("✓ SUCCESS on PC4/PC5! WHO_AM_I = 0x{:02X}", who[0]);
            if who[0] == 0x69 || who[0] == 0x6A {
                rprintln_ts!("✓ LSM6DS3 detected correctly!");
            }
        }
        Err(e) => {
            rprintln_ts!("✗ Failed on PC4/PC5: {:?}", e);
        }
    }

    delay.delay_ms(500);

    // Try PB2/PB3 (forum suggested pins)
    rprintln_ts!("\n[Test 2] Trying I2C0 on PB2 (SDA) / PB3 (SCL)...");
    let mut i2c_pb = I2c::<I2c0>::new(
        dp.pins.pb2,
        dp.pins.pb3,
        I2cConfig::default().with_speed(I2cSpeed::Standard),
    );

    let mut who = [0u8; 1];
    match i2c_pb.write_read(LSM6DS3_ADDR, &[WHO_AM_I], &mut who) {
        Ok(()) => {
            rprintln_ts!("✓ SUCCESS on PB2/PB3! WHO_AM_I = 0x{:02X}", who[0]);
            if who[0] == 0x69 || who[0] == 0x6A {
                rprintln_ts!("✓ LSM6DS3 detected correctly!");
            }
        }
        Err(e) => {
            rprintln_ts!("✗ Failed on PB2/PB3: {:?}", e);
        }
    }

    rprintln_ts!("\n=== Debug Complete ===");
    rprintln_ts!("If neither worked, check:");
    rprintln_ts!("  - I2C pull-up resistors (4.7k on SDA/SCL)");
    rprintln_ts!("  - IMU address (0x6A)");
    rprintln_ts!("  - Power supply to IMU");
    rprintln_ts!("  - Board connections");

    loop {
        delay.delay_ms(1000);
    }
}
