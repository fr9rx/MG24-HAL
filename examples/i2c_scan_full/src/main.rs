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
    rprintln_ts!("=== I2C Full Range Bus Scanner ===");

    let dp = mg24_hal::init(CpuConfig::default().with_i2c_clock(true)).unwrap();
    let delay = Delay::new();

    // Setup LED for status indication
    let mut led = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());

    rprintln_ts!("Configured I2C clock and pins");

    // Create I2C instance (I2C0 with standard mode)
    let mut i2c = I2c::<I2c0>::new(
        dp.pins.pc4,
        dp.pins.pc5,
        I2cConfig::default().with_speed(I2cSpeed::Standard),
    );

    rprintln_ts!("I2C0 initialized at 100 kHz");
    rprintln_ts!("Scanning ALL I2C addresses 0x00-0x7F (128 total)");
    rprintln_ts!("Address ranges:");
    rprintln_ts!("  0x00-0x07: Reserved for special commands");
    rprintln_ts!("  0x08-0x77: Standard I2C addresses");
    rprintln_ts!("  0x78-0x7F: Reserved addresses");

    let mut found_count = 0;
    let mut addresses = [0u8; 128];

    // Scan ALL possible 7-bit I2C addresses (0x00-0x7F)
    for addr in 0x00u8..=0x7F {
        // Try a quick write to see if device responds
        match i2c.write(addr, &[]) {
            Ok(()) => {
                rprintln_ts!("Found device at 0x{:02X} ({})", addr, addr);
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

    rprintln_ts!("=== Full Range Scan Complete ===");
    rprintln_ts!("Total addresses scanned: 128");
    rprintln_ts!("Found {} device(s):", found_count);

    if found_count > 0 {
        rprintln_ts!("Devices found at:");
        for i in 0..found_count {
            let addr = addresses[i as usize];
            rprintln_ts!("  0x{:02X} (decimal: {})", addr, addr);
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
        rprintln_ts!("Diagnostics:");
        rprintln_ts!("  - Check pull-up resistors (typically 4.7kΩ on SDA/SCL)");
        rprintln_ts!("  - Verify device power supply");
        rprintln_ts!("  - Check pin connections (PC4=SDA, PC5=SCL)");
        rprintln_ts!("  - Verify I2C device is not in sleep mode");
    }

    rprintln_ts!("Scanning finished. Device will loop forever.");
    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
