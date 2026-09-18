#![no_std]
#![no_main]

use mg24_hal::{
    rprintln_ts, rtt, timestamp,
    CpuConfig,
    delay::Delay,
    dma::{Dma, DmaConfig, DmaRequest, DmaSize, DmaIncrement},
    gpio::{Output, OutputConfig, Level},
    i2c::{I2c, I2cConfig, I2cSpeed, I2c0},
};

#[mg24_hal::main]
fn main() -> ! {
    rtt::init();
    timestamp::init();
    rprintln_ts!("=== I2C DMA Example ===");

    let dp = mg24_hal::init(
        CpuConfig::default()
            .with_i2c_clock(true)
            .with_ldma_clock(true)
    ).unwrap();
    let delay = Delay::new();

    // Setup LED for status indication
    let mut led = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());

    rprintln_ts!("Configured I2C and DMA clocks");

    // Create I2C instance
    let mut i2c = I2c::<I2c0>::new(
        dp.pins.pc4,
        dp.pins.pc5,
        I2cConfig::default().with_speed(I2cSpeed::Standard),
    );

    rprintln_ts!("I2C0 initialized at 100 kHz");

    // Create DMA controller
    let mut dma = Dma::new();
    rprintln_ts!("LDMA controller initialized");

    // Configure DMA channel 0 for I2C0 RX
    let dma_config = DmaConfig::default()
        .with_channel(0)
        .with_size(DmaSize::Byte)
        .with_src_inc(DmaIncrement::None)  // Fixed address for I2C RX register
        .with_dst_inc(DmaIncrement::One)   // Increment destination
        .with_block_size(1);

    dma.configure_request(0, DmaRequest::I2c0Rx);
    rprintln_ts!("DMA channel 0 configured for I2C0 RX");

    // Create a buffer for DMA transfer (in RAM at specific address)
    let mut rx_buffer = [0u8; 32];

    rprintln_ts!("Testing I2C DMA functionality");
    rprintln_ts!("Buffer address: 0x{:08X}", &rx_buffer[0] as *const _ as u32);

    // Set up a 32-byte DMA transfer from I2C0 RXDATA (0x4000_a00C) to buffer
    // Note: This is a demonstration - actual I2C RX via DMA requires proper I2C
    // command sequence to read data from device
    dma.configure_transfer(
        0,
        0x4000_a00C,  // I2C0 RXDATA register
        &mut rx_buffer[0] as *mut _ as u32,
        32,
        &dma_config,
    );

    rprintln_ts!("DMA transfer configured: 32 bytes from I2C0 RX");

    // Enable and start the channel
    dma.enable_channel(0);
    rprintln_ts!("DMA channel 0 enabled");

    // In a real scenario, the peripheral (I2C) would request data
    // For this demo, we'll just show the DMA status

    let mut status_count = 0;
    loop {
        if status_count % 10 == 0 {
            let ch_status = dma.channel_status();
            let is_busy = dma.any_busy();
            rprintln_ts!("DMA Status: channels_active=0x{:02X}, any_busy={}", ch_status, is_busy);

            if dma.is_transfer_done(0) {
                rprintln_ts!("DMA transfer completed!");
                dma.clear_done_flag(0);

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
