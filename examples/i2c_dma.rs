#![no_std]
#![no_main]

use mg24_hal::{
    rprintln_ts, rtt, timestamp,
    CpuConfig,
    delay::Delay,
    dma::Dma,
    gpio::{Output, OutputConfig, Level},
};

#[mg24_hal::main]
fn main() -> ! {
    rtt::init();
    timestamp::init();
    rprintln_ts!("=== I2C0 DMA RX Example ===");

    let dp = mg24_hal::init(
        CpuConfig::default()
            .with_i2c_clock(true)
            .with_ldma_clock(true)
    ).unwrap();
    let delay = Delay::new();

    let mut led = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());

    rprintln_ts!("I2C and DMA clocks enabled");

    let mut dma = Dma::new();
    rprintln_ts!("LDMA controller initialized");

    // Buffer for DMA transfer
    let mut rx_buffer = [0u8; 32];

    rprintln_ts!("Testing I2C0 RX via DMA");
    rprintln_ts!("Buffer: 0x{:08X}", &mut rx_buffer[0] as *mut u8 as u32);

    // One-liner I2C0 RX DMA setup - safe API accepts reference directly
    let _ = dma.i2c0_rx_slice(0, &mut rx_buffer);
    rprintln_ts!("DMA channel 0 configured and enabled for I2C0 RX");

    // In a real scenario, the peripheral (I2C) would request data
    // For this demo, we'll just show the DMA status

    let mut status_count = 0;
    loop {
        if status_count % 10 == 0 {
            let ch_status = dma.channel_status();
            let is_busy = dma.any_busy();
            rprintln_ts!("DMA Status: channels_active=0x{:02X}, any_busy={}", ch_status, is_busy);

            if dma.is_transfer_done(0).unwrap_or(false) {
                rprintln_ts!("DMA transfer completed!");
                let _ = dma.clear_done_flag(0);

                // Show first few bytes from buffer
                rprintln_ts!("RX Buffer (first 8 bytes):");
                for i in 0..8 {
                    rprintln_ts!("  [{}] = 0x{:02X}", i, rx_buffer[i]);
                }

                // Success pattern
                for _ in 0..5 {
                    led.set_level(Level::High);
                    delay.delay_ms(100);
                    led.set_level(Level::Low);
                    delay.delay_ms(100);
                }

                rprintln_ts!("DMA example complete!");
                break;
            }
        }

        status_count += 1;
        led.toggle();
        delay.delay_ms(100);

        // Timeout after 2 seconds
        if status_count > 20 {
            rprintln_ts!("DMA transfer timeout - peripheral may not have requested data");
            rprintln_ts!("(I2C device needs to request read for DMA to trigger)");
            break;
        }
    }

    rprintln_ts!("Device will loop forever");
    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
