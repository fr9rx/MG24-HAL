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
    rprintln_ts!("=== DMA Memory-to-Memory Transfer ===");

    let dp = mg24_hal::init(
        CpuConfig::default().with_ldma_clock(true)
    ).unwrap();
    let delay = Delay::new();

    let mut led = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());

    rprintln_ts!("LDMA initialized");

    // Source data in RAM
    let src_data = [
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
        0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x12, 0x34,
    ];

    // Destination buffer
    let mut dst_buffer = [0u8; 16];

    let src_addr = &src_data[0] as *const u8 as u32;
    let dst_addr = &mut dst_buffer[0] as *mut u8 as u32;

    rprintln_ts!("Source:      0x{:08X}", src_addr);
    rprintln_ts!("Destination: 0x{:08X}", dst_addr);

    let mut dma = Dma::new();

    // One-liner M2M copy: 16 bytes from src to dst using DMA
    dma.copy(0, src_addr, dst_addr, 16);
    rprintln_ts!("DMA transfer started");

    // Wait for completion
    let mut timeout = 0;
    while !dma.is_transfer_done(0) && timeout < 1000 {
        timeout += 1;
        delay.delay_us(100);
    }

    if dma.is_transfer_done(0) {
        dma.clear_done_flag(0);
        rprintln_ts!("Transfer complete!");

        // Verify data
        let mut all_match = true;
        for i in 0..16 {
            if src_data[i] != dst_buffer[i] {
                all_match = false;
                rprintln_ts!("MISMATCH at [{}]: src=0x{:02X}, dst=0x{:02X}", i, src_data[i], dst_buffer[i]);
            }
        }

        if all_match {
            rprintln_ts!("SUCCESS: All 16 bytes copied correctly!");
            rprintln_ts!("Destination buffer contents:");
            for i in 0..16 {
                rprintln_ts!("  [{}] = 0x{:02X}", i, dst_buffer[i]);
            }

            // Success blink pattern
            for _ in 0..5 {
                led.set_level(Level::High);
                delay.delay_ms(100);
                led.set_level(Level::Low);
                delay.delay_ms(100);
            }
        }
    } else {
        rprintln_ts!("Transfer timeout!");
    }

    rprintln_ts!("Example complete");
    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
