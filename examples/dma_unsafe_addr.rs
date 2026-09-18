#![no_std]
#![no_main]

use mg24_hal::{
    rprintln_ts, rtt, timestamp,
    CpuConfig,
    delay::Delay,
    dma::{Dma, DmaConfig, DmaSize, DmaIncrement},
    gpio::{Output, OutputConfig, Level},
};

/// Example: DMA with raw memory addresses
///
/// This example demonstrates how to use the address-based DMA API
/// when you have raw memory addresses (e.g., from peripherals or
/// memory-mapped regions).
///
/// Use case: When working with:
/// - Peripheral memory-mapped I/O addresses
/// - DMA chains using absolute addresses
/// - Hardware registers or external memory
/// - Addresses calculated at runtime
///
/// WARNING: Unsafe by nature - you must ensure:
/// - Addresses are valid and aligned
/// - No overlapping source/destination
/// - Correct address widths for the peripheral
/// - Memory is accessible during transfer

#[mg24_hal::main]
fn main() -> ! {
    rtt::init();
    timestamp::init();
    rprintln_ts!("=== DMA with Unsafe Memory Addresses ===");

    let dp = mg24_hal::init(
        CpuConfig::default().with_ldma_clock(true)
    ).unwrap();
    let delay = Delay::new();

    let mut led = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());

    // Example 1: Copy between absolute addresses
    rprintln_ts!("\n[Example 1] Copy between RAM regions");
    rprintln_ts!("Source region:      0x2000_0000");
    rprintln_ts!("Destination region: 0x2000_0100");

    // In real scenarios, these might come from:
    // - Peripheral memory-mapped addresses
    // - Linked descriptor lists in memory
    // - External SRAM regions
    let src_addr = 0x2000_0000u32;  // Example: RAM region 1
    let dst_addr = 0x2000_0100u32;  // Example: RAM region 2
    let copy_size = 256u16;         // 256 bytes

    let mut dma = Dma::new();

    // Use address-based API (unsafe - you validate correctness)
    let _ = dma.copy_addr(0, src_addr, dst_addr, copy_size);
    rprintln_ts!("DMA transfer started: {} bytes", copy_size);

    // Poll with timeout
    let mut timeout = 0;
    while !dma.is_transfer_done(0).unwrap_or(false) && timeout < 10000 {
        timeout += 1;
        delay.delay_us(10);
    }

    if dma.is_transfer_done(0).unwrap_or(false) {
        let _ = dma.clear_done_flag(0);
        rprintln_ts!("✓ Transfer complete!");
    } else {
        rprintln_ts!("✗ Transfer timeout!");
    }

    // Example 2: I2C0 with explicit peripheral addresses
    rprintln_ts!("\n[Example 2] I2C0 RX using explicit register address");
    rprintln_ts!("I2C0 RXDATA register: 0x4000_a00C");

    let rx_buffer = [0u8; 32];
    let buffer_addr = &rx_buffer[0] as *const u8 as u32;

    // When you know the exact I2C0 RXDATA register address
    let _ = dma.i2c0_rx_addr(1, buffer_addr, 32);
    rprintln_ts!("I2C0 RX configured on channel 1");
    rprintln_ts!("Waiting for I2C0 to request data...");

    // Timeout after 100ms
    timeout = 0;
    while !dma.is_transfer_done(1).unwrap_or(false) && timeout < 1000 {
        timeout += 1;
        delay.delay_ms(1);
    }

    if timeout >= 1000 {
        rprintln_ts!("No I2C0 data received (timeout)");
        rprintln_ts!("(Device must initiate I2C read for DMA to trigger)");
    } else {
        rprintln_ts!("✓ I2C0 data received via DMA");
    }

    // Example 3: Custom configuration with address-based API
    rprintln_ts!("\n[Example 3] Advanced: Custom transfer config");

    let config = DmaConfig::default()
        .with_size(DmaSize::Word)              // 32-bit transfers
        .with_src_inc(DmaIncrement::Four)      // Skip by 4 units
        .with_dst_inc(DmaIncrement::Two)       // Skip by 2 units
        .with_block_size(8);                   // 8 units per arbitration

    rprintln_ts!("Custom DMA config:");
    rprintln_ts!("  Size: Word (32-bit)");
    rprintln_ts!("  Src increment: 4 units");
    rprintln_ts!("  Dst increment: 2 units");
    rprintln_ts!("  Block size: 8 units");

    // Using custom config with address-based transfer
    let custom_src = 0x2000_0200u32;
    let custom_dst = 0x2000_0300u32;
    let _ = dma.configure_transfer(2, custom_src, custom_dst, 16, &config);
    let _ = dma.enable_channel(2);
    let _ = dma.start_transfer(2);
    rprintln_ts!("Custom transfer started on channel 2");

    // Wait for completion
    timeout = 0;
    while !dma.is_transfer_done(2).unwrap_or(false) && timeout < 10000 {
        timeout += 1;
        delay.delay_us(10);
    }

    if dma.is_transfer_done(2).unwrap_or(false) {
        let _ = dma.clear_done_flag(2);
        rprintln_ts!("✓ Custom transfer complete!");
    } else {
        rprintln_ts!("✗ Custom transfer timeout!");
    }

    // Example 4: Check DMA status
    rprintln_ts!("\n[Example 4] DMA Status Monitoring");
    let ch_status = dma.channel_status();
    rprintln_ts!("Channel status register: 0x{:02X}", ch_status);
    for i in 0..8 {
        if (ch_status & (1 << i)) != 0 {
            rprintln_ts!("  Channel {}: enabled", i);
        }
    }

    if dma.any_busy() {
        rprintln_ts!("DMA: At least one channel is busy");
    } else {
        rprintln_ts!("DMA: All channels idle");
    }

    // Success indicators
    rprintln_ts!("\n=== Examples Complete ===");
    for _ in 0..3 {
        led.set_level(Level::High);
        delay.delay_ms(100);
        led.set_level(Level::Low);
        delay.delay_ms(100);
    }

    rprintln_ts!("Device will loop forever");
    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
