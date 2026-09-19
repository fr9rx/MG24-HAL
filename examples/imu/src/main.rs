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
const CTRL1_XL: u8 = 0x10;
const CTRL2_G: u8 = 0x11;
const ACCEL_X_L: u8 = 0x28;
const ACCEL_X_H: u8 = 0x29;
const ACCEL_Y_L: u8 = 0x2A;
const ACCEL_Y_H: u8 = 0x2B;
const ACCEL_Z_L: u8 = 0x2C;
const ACCEL_Z_H: u8 = 0x2D;
const GYRO_X_L: u8 = 0x22;
const GYRO_X_H: u8 = 0x23;
const GYRO_Y_L: u8 = 0x24;
const GYRO_Y_H: u8 = 0x25;
const GYRO_Z_L: u8 = 0x26;
const GYRO_Z_H: u8 = 0x27;

#[mg24_hal::main]
fn main() -> ! {
    rtt::init();
    timestamp::init();
    rprintln_ts!("=== LSM6DS3 IMU Test (XIAO MG24 Sense) ===");

    let dp = mg24_hal::init(CpuConfig::default().with_i2c_clock(true)).unwrap();
    let delay = Delay::new();

    // Setup LED for status indication
    let mut led = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());

    // Enable IMU power (PD5 must be driven HIGH)
    let mut imu_enable = Output::new(dp.pins.pd5, Level::High, OutputConfig::default());
    rprintln_ts!("IMU enable pin (PD5) set HIGH");
    delay.delay_ms(100);

    // Create I2C instance on PB2/PB3 (correct pins for XIAO MG24 Sense IMU)
    let mut i2c = I2c::<I2c0>::new(
        dp.pins.pb2,  // SDA
        dp.pins.pb3,  // SCL
        I2cConfig::default().with_speed(I2cSpeed::Standard),
    );

    rprintln_ts!("I2C0 initialized on PB2 (SDA) / PB3 (SCL)");
    delay.delay_ms(100);

    // Test I2C communication by reading WHO_AM_I register
    rprintln_ts!("\nTesting I2C communication...");
    let mut who_am_i = [0u8; 1];
    match i2c.write_read(LSM6DS3_ADDR, &[WHO_AM_I], &mut who_am_i) {
        Ok(()) => {
            rprintln_ts!("WHO_AM_I register read: 0x{:02X}", who_am_i[0]);
            if who_am_i[0] == 0x69 || who_am_i[0] == 0x6A {
                rprintln_ts!("✓ LSM6DS3 detected successfully!");
            } else {
                rprintln_ts!("✗ Unexpected WHO_AM_I value. Check connections.");
            }
        }
        Err(e) => {
            rprintln_ts!("✗ Failed to read WHO_AM_I: {:?}", e);
            rprintln_ts!("Check:");
            rprintln_ts!("  - Pull-up resistors on SDA/SCL");
            rprintln_ts!("  - Device power (PD5 enable pin)");
            rprintln_ts!("  - I2C address (0x6A)");
            loop {
                led.toggle();
                delay.delay_ms(500);
            }
        }
    }

    // Initialize accelerometer: 104 Hz, ±8g
    rprintln_ts!("\nInitializing IMU sensors...");
    let accel_config = [CTRL1_XL, 0x40]; // 104 Hz, ±8g
    if let Err(e) = i2c.write(LSM6DS3_ADDR, &accel_config) {
        rprintln_ts!("✗ Failed to configure accelerometer: {:?}", e);
    } else {
        rprintln_ts!("✓ Accelerometer configured");
    }

    // Initialize gyroscope: 104 Hz, ±245 dps
    let gyro_config = [CTRL2_G, 0x40]; // 104 Hz, ±245 dps
    if let Err(e) = i2c.write(LSM6DS3_ADDR, &gyro_config) {
        rprintln_ts!("✗ Failed to configure gyroscope: {:?}", e);
    } else {
        rprintln_ts!("✓ Gyroscope configured");
    }

    rprintln_ts!("\nStarting sensor readings...\n");
    delay.delay_ms(500);

    let mut read_count = 0;

    loop {
        // Read accelerometer data
        let mut accel_data = [0u8; 6];
        if i2c.write_read(LSM6DS3_ADDR, &[ACCEL_X_L], &mut accel_data).is_ok() {
            let accel_x = ((accel_data[1] as i16) << 8) | (accel_data[0] as i16);
            let accel_y = ((accel_data[3] as i16) << 8) | (accel_data[2] as i16);
            let accel_z = ((accel_data[5] as i16) << 8) | (accel_data[4] as i16);

            // Convert to g (at ±8g range, 1 LSB ≈ 0.244 mg)
            let ax_g = accel_x as f32 * 0.000244;
            let ay_g = accel_y as f32 * 0.000244;
            let az_g = accel_z as f32 * 0.000244;

            rprintln_ts!(
                "Accel [{}] X:{:7.2}g Y:{:7.2}g Z:{:7.2}g",
                read_count, ax_g, ay_g, az_g
            );
        } else {
            rprintln_ts!("✗ Failed to read accelerometer data");
        }

        // Read gyroscope data
        let mut gyro_data = [0u8; 6];
        if i2c.write_read(LSM6DS3_ADDR, &[GYRO_X_L], &mut gyro_data).is_ok() {
            let gyro_x = ((gyro_data[1] as i16) << 8) | (gyro_data[0] as i16);
            let gyro_y = ((gyro_data[3] as i16) << 8) | (gyro_data[2] as i16);
            let gyro_z = ((gyro_data[5] as i16) << 8) | (gyro_data[4] as i16);

            // Convert to dps (at ±245 dps range, 1 LSB ≈ 0.00875 dps)
            let gx_dps = gyro_x as f32 * 0.00875;
            let gy_dps = gyro_y as f32 * 0.00875;
            let gz_dps = gyro_z as f32 * 0.00875;

            rprintln_ts!(
                "Gyro       X:{:7.2}° Y:{:7.2}° Z:{:7.2}°/s",
                gx_dps, gy_dps, gz_dps
            );
        } else {
            rprintln_ts!("✗ Failed to read gyroscope data");
        }

        rprintln_ts!("");
        read_count += 1;

        led.toggle();
        delay.delay_ms(1000);

        if read_count >= 30 {
            rprintln_ts!("=== Test Complete ===");
            break;
        }
    }

    rprintln_ts!("IMU test finished. Looping forever.");
    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
