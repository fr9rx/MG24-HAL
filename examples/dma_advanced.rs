#![no_std]
#![no_main]

use mg24_hal::{
    rprintln_ts, rtt, timestamp,
    CpuConfig,
    delay::Delay,
    dma::{Dma, DmaConfig, DmaSize, DmaIncrement},
    gpio::{Output, OutputConfig, Level},
};

/// Advanced DMA Examples
///
/// This example demonstrates advanced DMA features:
/// 1. Multiple concurrent DMA channels
/// 2. Custom transfer configurations
/// 3. Packed/unpacked data transfers
/// 4. Different block sizes for timing control
/// 5. Status monitoring and synchronization
///
/// Reference Manual: EFR32MG24 Chapter 24 - LDMA
///
/// Advanced scenarios:
/// - Scatter-gather operations (multiple buffers)
/// - Real-time data acquisition
/// - Peripheral communication chains
/// - Memory-to-memory format conversion
/// - Synchronized multi-channel transfers

#[mg24_hal::main]
fn main() -> ! {
    rtt::init();
    timestamp::init();
    rprintln_ts!("=== Advanced DMA Examples ===");

    let dp = mg24_hal::init(
        CpuConfig::default().with_ldma_clock(true)
    ).unwrap();
    let delay = Delay::new();

    let mut led = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());

    // ========== Example 1: Packed/Unpacked Data Transfer ==========
    rprintln_ts!("\n[Example 1] Packed/Unpacked Data Conversion");
    rprintln_ts!("Scenario: Convert word-aligned bytes to packed bytes");
    rprintln_ts!("Source (word-aligned): [XX, YY, 00, ZZ, ...]");
    rprintln_ts!("Destination (packed):  [XX, YY, ZZ, ...]");

    // Configuration for unpacking word-aligned bytes to contiguous bytes
    // SIZE = BYTE, SRCINC = 4 (skip 4 bytes), DSTINC = 1 (contiguous)
    let unpack_config = DmaConfig::default()
        .with_size(DmaSize::Byte)
        .with_src_inc(DmaIncrement::Four)      // Skip 4 byte addresses per transfer
        .with_dst_inc(DmaIncrement::One)       // Write contiguously
        .with_block_size(4);

    let mut dma = Dma::new();
    let src_word_aligned = 0x2000_0000u32;
    let dst_packed = 0x2000_0200u32;

    dma.configure_transfer(0, src_word_aligned, dst_packed, 32, &unpack_config).ok();
    dma.enable_channel(0).ok();
    dma.start_transfer(0).ok();
    rprintln_ts!("Channel 0: Unpacking transfer started");
    rprintln_ts!("  Src increment: 4 units (word-aligned)");
    rprintln_ts!("  Dst increment: 1 unit (packed)");

    delay.delay_ms(10);
    if dma.is_transfer_done(0).unwrap_or(false) {
        dma.clear_done_flag(0).ok();
        rprintln_ts!("✓ Unpack transfer complete");
    }

    // ========== Example 2: Multiple Concurrent Channels ==========
    rprintln_ts!("\n[Example 2] Multiple Concurrent DMA Channels");
    rprintln_ts!("Starting 3 independent transfers simultaneously");

    let config_ch1 = DmaConfig::default()
        .with_size(DmaSize::Word)
        .with_block_size(4);

    let config_ch2 = DmaConfig::default()
        .with_size(DmaSize::HalfWord)
        .with_block_size(8);

    let config_ch3 = DmaConfig::default()
        .with_size(DmaSize::Byte)
        .with_block_size(16);

    // Channel 1: 32-bit transfers
    dma.configure_transfer(1, 0x2000_0300u32, 0x2000_0400u32, 64, &config_ch1).ok();
    dma.enable_channel(1).ok();
    dma.start_transfer(1).ok();
    rprintln_ts!("Channel 1: Word transfers (32-bit), block size 4");

    // Channel 2: 16-bit transfers
    dma.configure_transfer(2, 0x2000_0500u32, 0x2000_0600u32, 128, &config_ch2).ok();
    dma.enable_channel(2).ok();
    dma.start_transfer(2).ok();
    rprintln_ts!("Channel 2: HalfWord transfers (16-bit), block size 8");

    // Channel 3: 8-bit transfers
    dma.configure_transfer(3, 0x2000_0700u32, 0x2000_0800u32, 256, &config_ch3).ok();
    dma.enable_channel(3).ok();
    dma.start_transfer(3).ok();
    rprintln_ts!("Channel 3: Byte transfers (8-bit), block size 16");

    // Monitor all channels simultaneously
    rprintln_ts!("Monitoring concurrent transfers...");
    let mut ch1_done = false;
    let mut ch2_done = false;
    let mut ch3_done = false;
    let mut iteration = 0;

    while (!ch1_done || !ch2_done || !ch3_done) && iteration < 1000 {
        if !ch1_done && dma.is_transfer_done(1).unwrap_or(false) {
            dma.clear_done_flag(1).ok();
            ch1_done = true;
            rprintln_ts!("  ✓ Channel 1 complete");
        }
        if !ch2_done && dma.is_transfer_done(2).unwrap_or(false) {
            dma.clear_done_flag(2).ok();
            ch2_done = true;
            rprintln_ts!("  ✓ Channel 2 complete");
        }
        if !ch3_done && dma.is_transfer_done(3).unwrap_or(false) {
            dma.clear_done_flag(3).ok();
            ch3_done = true;
            rprintln_ts!("  ✓ Channel 3 complete");
        }

        delay.delay_us(100);
        iteration += 1;
    }

    if iteration >= 1000 {
        rprintln_ts!("Transfer timeout (1000 iterations)");
    } else {
        rprintln_ts!("All concurrent transfers complete!");
    }

    // ========== Example 3: Different Block Sizes ==========
    rprintln_ts!("\n[Example 3] Block Size Effects on Arbitration");
    rprintln_ts!("Block size controls how many units per arbitration cycle");

    let sizes = [1u16, 2, 4, 8, 16];
    let size_names = ["1", "2", "4", "8", "16"];

    for (idx, &size) in sizes.iter().enumerate() {
        let config = DmaConfig::default()
            .with_size(DmaSize::Word)
            .with_block_size(size);

        rprintln_ts!("Channel {}: Block size = {}", idx % 4, size_names[idx]);

        if idx < 4 {
            let ch = idx as u8;
            let _ = dma.configure_transfer(
                ch,
                0x2000_1000u32 + (idx as u32 * 0x100),
                0x2000_2000u32 + (idx as u32 * 0x100),
                32,
                &config,
            );
            dma.enable_channel(ch).ok();
            dma.start_transfer(ch).ok();
        }
    }

    rprintln_ts!("Block size = 1: 1 unit per arbitration (low latency)");
    rprintln_ts!("Block size = 16: 16 units per arbitration (high throughput)");
    rprintln_ts!("Choose based on latency vs throughput tradeoff");

    // Wait for first 4 channels
    delay.delay_ms(50);
    for ch in 0..4 {
        if dma.is_transfer_done(ch).unwrap_or(false) {
            dma.clear_done_flag(ch).ok();
        }
    }

    // ========== Example 4: DMA Status Analysis ==========
    rprintln_ts!("\n[Example 4] Advanced Status Monitoring");

    let ch_status = dma.channel_status();
    rprintln_ts!("Channel Status Byte: 0x{:02X}", ch_status);
    rprintln_ts!("Bits set = channels enabled:");

    for i in 0..8 {
        if (ch_status & (1 << i)) != 0 {
            rprintln_ts!("  Bit {}: Channel {} is ENABLED", i, i);
        }
    }

    let busy = dma.any_busy();
    rprintln_ts!("Any channel busy? {}", if busy { "YES" } else { "NO" });

    // ========== Example 5: Transfer Rate Optimization ==========
    rprintln_ts!("\n[Example 5] Transfer Rate Optimization");
    rprintln_ts!("Different configurations for different use cases");

    rprintln_ts!("\nLow-Latency Mode:");
    rprintln_ts!("  Size: Byte, BlockSize: 1");
    rprintln_ts!("  Use: Real-time responses needed");

    rprintln_ts!("\nHigh-Throughput Mode:");
    rprintln_ts!("  Size: Word, BlockSize: 128");
    rprintln_ts!("  Use: Large bulk transfers");

    rprintln_ts!("\nBalanced Mode:");
    rprintln_ts!("  Size: HalfWord, BlockSize: 8");
    rprintln_ts!("  Use: Mixed workloads");

    // ========== Example 6: Timing-Critical Synchronization ==========
    rprintln_ts!("\n[Example 6] Timing-Critical Multi-Channel Sync");
    rprintln_ts!("Start multiple transfers and wait for all to complete");

    let channels = [4u8, 5, 6, 7];
    let config = DmaConfig::default().with_size(DmaSize::Word);

    for (i, &ch) in channels.iter().enumerate() {
        let _ = dma.configure_transfer(
            ch,
            0x2000_3000u32 + (i as u32 * 0x100),
            0x2000_4000u32 + (i as u32 * 0x100),
            16,
            &config,
        );
        dma.enable_channel(ch).ok();
        dma.start_transfer(ch).ok();
    }

    rprintln_ts!("All channels started simultaneously");

    // Busy-wait for all to complete (in real code, use interrupts)
    let start_ms = timestamp::millis_since_boot();
    while dma.any_busy() {
        // Timeout after 1 second
        if timestamp::millis_since_boot() - start_ms > 1000 {
            rprintln_ts!("Timeout waiting for transfers");
            break;
        }
        delay.delay_us(100);
    }

    let elapsed = timestamp::millis_since_boot() - start_ms;
    rprintln_ts!("All transfers complete (took {}ms)", elapsed);

    // Clear all done flags
    for &ch in channels.iter() {
        if dma.is_transfer_done(ch).unwrap_or(false) {
            dma.clear_done_flag(ch).ok();
        }
    }

    // Success pattern
    rprintln_ts!("\n=== All Advanced Examples Complete ===");
    for _ in 0..5 {
        led.set_level(Level::High);
        delay.delay_ms(80);
        led.set_level(Level::Low);
        delay.delay_ms(80);
    }

    rprintln_ts!("Key takeaways:");
    rprintln_ts!("✓ DMA supports packed/unpacked data format conversion");
    rprintln_ts!("✓ Multiple channels can run concurrently");
    rprintln_ts!("✓ Block size tunes latency vs throughput");
    rprintln_ts!("✓ Status monitoring enables synchronization");
    rprintln_ts!("✓ Transfers are independent and overlappable");

    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
