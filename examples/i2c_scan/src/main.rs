#![no_std]
#![no_main]

use mg24_hal::{
    rprintln_ts, rtt, timestamp,
    CpuConfig,
    delay::Delay,
    gpio::{Output, OutputConfig, Level},
    i2c::{I2c, I2cConfig, I2cSpeed, I2c0},
};

/// Common I2C slave addresses for testing
/// 0x50 = EEPROM (24C02)
/// 0x48 = Temperature sensor (LM75)
/// 0x68 = RTC (DS3231) or Accelerometer (MPU6050)
#[allow(dead_code)]
const SLAVE_ADDR: u8 = 0x50; // Default to EEPROM

#[mg24_hal::main]
fn main() -> ! {
    rtt::init();
    timestamp::init();
    rprintln_ts!("=== I2C Bus Scanner Started ===");

    let dp = mg24_hal::init(CpuConfig::default().with_i2c_clock(true)).unwrap();
    let delay = Delay::new();

    // Setup LED for status indication
    let mut led = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());

    rprintln_ts!("Configured I2C clock and pins");

    // Create I2C instance (I2C0 with standard mode)
    // I2C0: SDA=PC4, SCL=PC5 (from PINS.md)
    let mut i2c = I2c::<I2c0>::new(
        dp.pins.pc4,
        dp.pins.pc5,
        I2cConfig::default().with_speed(I2cSpeed::Standard),
    );

    rprintln_ts!("I2C0 initialized at 100 kHz");
    rprintln_ts!("Scanning I2C addresses 0x08-0x77 (112 total)...");

    let mut found_count = 0;
    let mut addresses = [0u8; 120];

    // Scan valid I2C addresses (skip reserved ranges)
    // Valid addresses: 0x08-0x77 (7-bit addresses)
    for addr in 0x08u8..=0x77u8 {
        // Try a quick write to see if device responds
        // We send a STOP condition which shouldn't cause issues
        match i2c.write(addr, &[]) {
            Ok(()) => {
                rprintln_ts!("Found device at 0x{:02X}", addr);
                addresses[found_count as usize] = addr;
                found_count += 1;
                led.toggle();
                delay.delay_ms(100);
            }
            Err(_) => {
                // Device not found at this address
            }
        }
        delay.delay_ms(10);
    }

    rprintln_ts!("=== Scan Complete ===");
    rprintln_ts!("Found {} device(s):", found_count);

    if found_count > 0 {
        rprintln_ts!("\nAddresses with responding devices:");
        for i in 0..found_count {
            rprintln_ts!("  0x{:02X}", addresses[i as usize]);
        }

        // Blink LED success pattern
        for _ in 0..5 {
            led.set_level(Level::High);
            delay.delay_ms(100);
            led.set_level(Level::Low);
            delay.delay_ms(100);
        }
    } else {
        rprintln_ts!("No devices found on I2C0 bus");
        rprintln_ts!("Check:");
        rprintln_ts!("  - Pull-up resistors (typically 4.7kΩ on SDA/SCL)");
        rprintln_ts!("  - Device power supply");
        rprintln_ts!("  - Pin connections (PC4=SDA, PC5=SCL)");
    }

    rprintln_ts!("Scanning finished. Device will loop forever.");
    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
