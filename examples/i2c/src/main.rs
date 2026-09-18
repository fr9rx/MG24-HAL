#![no_std]
#![no_main]

use mg24_hal::{
    rprintln, rtt,
    CpuConfig,
    delay::Delay,
    gpio::{Output, OutputConfig, Level},
    i2c::{I2c, I2cConfig, I2cSpeed, I2c0},
};

/// Common I2C slave addresses for testing
/// 0x50 = EEPROM (24C02)
/// 0x48 = Temperature sensor (LM75)
/// 0x68 = RTC (DS3231) or Accelerometer (MPU6050)
const SLAVE_ADDR: u8 = 0x50; // Default to EEPROM

#[mg24_hal::main]
fn main() -> ! {
    rtt::init();
    rprintln!("\n=== I2C Test Started ===");

    let dp = mg24_hal::init(CpuConfig::default().with_i2c_clock(true)).unwrap();
    let delay = Delay::new();

    // Setup LED for status indication
    let mut led = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());

    rprintln!("Configured I2C clock and pins");

    // Create I2C instance (I2C0 with standard mode)
    // I2C0: SDA=PC4, SCL=PC5 (from PINS.md)
    let mut i2c = I2c::<I2c0>::new(
        dp.pins.pc4,
        dp.pins.pc5,
        I2cConfig::default().with_speed(I2cSpeed::Standard),
    );

    rprintln!("I2C0 initialized at 100 kHz");
    rprintln!("Target slave address: 0x{:02X}", SLAVE_ADDR);

    // Test 1: Read device ID or first register
    rprintln!("\n--- Test 1: Read first register ---");
    let mut read_buf = [0u8; 4];

    match i2c.read(SLAVE_ADDR, &mut read_buf) {
        Ok(()) => {
            rprintln!("✓ Read successful!");
            rprintln!("Data: [0x{:02X}, 0x{:02X}, 0x{:02X}, 0x{:02X}]",
                     read_buf[0], read_buf[1], read_buf[2], read_buf[3]);
            led.set_level(Level::High);
            delay.delay_ms(100);
            led.set_level(Level::Low);
        }
        Err(e) => {
            rprintln!("✗ Read failed: {:?}", e);
        }
    }

    delay.delay_ms(500);

    // Test 2: Write and read (register write test)
    rprintln!("\n--- Test 2: Write then Read ---");
    let write_data = [0x00]; // Typically address 0x00 is the first register

    match i2c.write_read(SLAVE_ADDR, &write_data, &mut read_buf) {
        Ok(()) => {
            rprintln!("✓ Write-Read successful!");
            rprintln!("Wrote register 0x00, read: [0x{:02X}, 0x{:02X}, 0x{:02X}, 0x{:02X}]",
                     read_buf[0], read_buf[1], read_buf[2], read_buf[3]);
            led.set_level(Level::High);
            delay.delay_ms(100);
            led.set_level(Level::Low);
        }
        Err(e) => {
            rprintln!("✗ Write-Read failed: {:?}", e);
        }
    }

    delay.delay_ms(500);

    // Test 3: Sequential reads
    rprintln!("\n--- Test 3: Sequential Reads ---");
    for i in 0..3 {
        let mut buf = [0u8; 2];
        match i2c.read(SLAVE_ADDR, &mut buf) {
            Ok(()) => {
                rprintln!("Read {}: [0x{:02X}, 0x{:02X}]", i, buf[0], buf[1]);
                led.toggle();
            }
            Err(e) => {
                rprintln!("Read {} failed: {:?}", i, e);
                break;
            }
        }
        delay.delay_ms(200);
    }

    rprintln!("\n=== All I2C Tests Complete ===");
    rprintln!("If all tests passed, I2C is working correctly!");

    // Blink LED pattern to indicate completion
    for _ in 0..5 {
        led.set_level(Level::High);
        delay.delay_ms(100);
        led.set_level(Level::Low);
        delay.delay_ms(100);
    }

    loop {
        delay.delay_ms(1000);
    }
}
