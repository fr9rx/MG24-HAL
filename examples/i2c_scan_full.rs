#![no_std]
#![no_main]

use mg24_hal::{
    rprintln, rtt,
    CpuConfig,
    delay::Delay,
    gpio::{Output, OutputConfig, Level},
    i2c::{I2c, I2cConfig, I2cSpeed, I2c0},
};

#[mg24_hal::main]
fn main() -> ! {
    rtt::init();
    rprintln!("\n=== I2C Full Range Bus Scanner ===");

    let dp = mg24_hal::init(CpuConfig::default().with_i2c_clock(true)).unwrap();
    let delay = Delay::new();

    // Setup LED for status indication
    let mut led = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());

    rprintln!("Configured I2C clock and pins");

    // Create I2C instance (I2C0 with standard mode)
    let mut i2c = I2c::<I2c0>::new(
        dp.pins.pc4,
        dp.pins.pc5,
        I2cConfig::default().with_speed(I2cSpeed::Standard),
    );

    rprintln!("I2C0 initialized at 100 kHz");
    rprintln!("\nScanning ALL I2C addresses 0x00-0x7F (128 total)...\n");
    rprintln!("Address ranges:");
    rprintln!("  0x00-0x07: Reserved for special commands");
    rprintln!("  0x08-0x77: Standard I2C addresses");
    rprintln!("  0x78-0x7F: Reserved addresses\n");

    let mut found_count = 0;
    let mut addresses = [0u8; 128];

    // Scan ALL possible 7-bit I2C addresses (0x00-0x7F)
    for addr in 0x00u8..=0x7F {
        // Try a quick write to see if device responds
        match i2c.write(addr, &[]) {
            Ok(()) => {
                rprintln!("Found device at 0x{:02X} ({})", addr, addr);
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

    rprintln!("\n=== Full Range Scan Complete ===");
    rprintln!("Total addresses scanned: 128");
    rprintln!("Found {} device(s):\n", found_count);

    if found_count > 0 {
        rprintln!("Devices found at:");
        for i in 0..found_count {
            let addr = addresses[i as usize];
            rprintln!("  0x{:02X} (decimal: {})", addr, addr);
        }

        // Blink LED success pattern
        for _ in 0..5 {
            led.set_level(Level::High);
            delay.delay_ms(100);
            led.set_level(Level::Low);
            delay.delay_ms(100);
        }
    } else {
        rprintln!("No devices found on I2C0 bus");
        rprintln!("\nDiagnostics:");
        rprintln!("  - Check pull-up resistors (typically 4.7kΩ on SDA/SCL)");
        rprintln!("  - Verify device power supply");
        rprintln!("  - Check pin connections (PC4=SDA, PC5=SCL)");
        rprintln!("  - Verify I2C device is not in sleep mode");
    }

    rprintln!("\nScanning finished. Device will loop forever.");
    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
