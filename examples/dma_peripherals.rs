#![no_std]
#![no_main]

use mg24_hal::{
    rprintln_ts, rtt, timestamp,
    CpuConfig,
    delay::Delay,
    dma::{Dma, DmaConfig, DmaSize, DmaIncrement},
    gpio::{Output, OutputConfig, Level},
};

/// DMA with Peripheral Integration
///
/// This example demonstrates how DMA integrates with various peripherals
/// on the EFR32MG24:
///
/// Available DMA Peripherals (from RM section 21.3.14):
/// - I2C0: RX (0x5, 0x0), TX (0x5, 0x1)
/// - I2C1: RX (0x6, 0x0), TX (0x6, 0x1)
/// - USART0, USART1: RX/TX
/// - IADC0: Scan/Single modes
/// - TIMER0-4: Capture/Compare
/// - VDAC0/1: Channel 0/1
/// - MSC: Memory (for flash operations)
/// - PRS: Pattern Recognition System
///
/// DMA Request Sources (SOURCESEL/SIGSEL):
/// Each peripheral has specific SOURCESEL value and SIGSEL for different signals.
///
/// This example shows:
/// 1. Configuring multiple peripheral request sources
/// 2. Setting up data transfers from/to peripherals
/// 3. Handling peripheral-specific addressing
/// 4. Timing and synchronization with peripherals
///
/// Note: Actual peripheral operation requires those peripherals to be
/// initialized and configured separately.

#[mg24_hal::main]
fn main() -> ! {
    rtt::init();
    timestamp::init();
    rprintln_ts!("=== DMA Peripheral Integration ===");

    let dp = mg24_hal::init(
        CpuConfig::default()
            .with_i2c_clock(true)
            .with_ldma_clock(true)
    ).unwrap();
    let delay = Delay::new();

    let mut led = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());

    let mut dma = Dma::new();
    rprintln_ts!("DMA initialized");

    // ========== Available DMA Peripherals ==========
    rprintln_ts!("\n[Reference] Available DMA Peripheral Sources");
    rprintln_ts!("(From EFR32MG24 RM Section 21.3.14)\n");

    rprintln_ts!("I2C Peripherals:");
    rprintln_ts!("  I2C0: SOURCESEL=0x5");
    rprintln_ts!("    - SIGSEL=0x0: I2C0_DMA_RXDATAV (RX FIFO has data)");
    rprintln_ts!("    - SIGSEL=0x1: I2C0_DMA_TXBL (TX buffer available)");
    rprintln_ts!("  I2C1: SOURCESEL=0x6");
    rprintln_ts!("    - SIGSEL=0x0: I2C1_DMA_RXDATAV");
    rprintln_ts!("    - SIGSEL=0x1: I2C1_DMA_TXBL");

    rprintln_ts!("\nUSART Peripherals:");
    rprintln_ts!("  USART0: SOURCESEL=0x0");
    rprintln_ts!("    - SIGSEL=0x0: USART0_DMA_RXFL (RX FIFO level)");
    rprintln_ts!("    - SIGSEL=0x1: USART0_DMA_TXEMPTY (TX empty)");
    rprintln_ts!("  USART1: SOURCESEL=0x1 (similar signals)");

    rprintln_ts!("\nAnalog Peripherals:");
    rprintln_ts!("  IADC0: SOURCESEL=0xA");
    rprintln_ts!("    - SIGSEL=0x0: IADC0_DMA_IADC_SCAN");
    rprintln_ts!("    - SIGSEL=0x1: IADC0_DMA_IADC_SINGLE");
    rprintln_ts!("  VDAC0: SOURCESEL=0x11");
    rprintln_ts!("    - SIGSEL=0x0: VDAC0_DMA_CH0_REQ");
    rprintln_ts!("    - SIGSEL=0x1: VDAC0_DMA_CH1_REQ");

    rprintln_ts!("\nTimer Peripherals:");
    rprintln_ts!("  TIMER0-4: SOURCESEL=0xC-0xE");
    rprintln_ts!("    - SIGSEL=0x0-0x3: CC0-CC3 (Capture/Compare)");

    // ========== Example 1: I2C Peripheral Configuration Details ==========
    rprintln_ts!("\n[Example 1] I2C0 DMA Configuration Details");

    rprintln_ts!("I2C0 RX via DMA:");
    rprintln_ts!("  Register: I2C0_RXDATA at 0x4000_a00C");
    rprintln_ts!("  Width: 8-bit (byte) reads");
    rprintln_ts!("  Trigger: I2C controller has data ready");
    rprintln_ts!("  DMA Setup: SOURCESEL=0x5, SIGSEL=0x0");
    rprintln_ts!("  Config: SRCINC=NONE (fixed), DSTINC=ONE (auto-increment)");

    rprintln_ts!("\nI2C0 TX via DMA:");
    rprintln_ts!("  Register: I2C0_TXDATA at 0x4000_a008");
    rprintln_ts!("  Width: 8-bit (byte) writes");
    rprintln_ts!("  Trigger: I2C controller TX buffer ready");
    rprintln_ts!("  DMA Setup: SOURCESEL=0x5, SIGSEL=0x1");
    rprintln_ts!("  Config: SRCINC=ONE (auto-increment), DSTINC=NONE (fixed)");

    // Setup I2C0 RX on channel 0
    let mut rx_buffer = [0u8; 32];
    rprintln_ts!("\nConfiguring I2C0 RX on channel 0...");
    dma.i2c0_rx_slice(0, &mut rx_buffer);
    rprintln_ts!("✓ Channel 0: I2C0 RX ready");
    rprintln_ts!("  Waiting for I2C0 to have data...");

    // ========== Example 2: Peripheral Addressing Rules ==========
    rprintln_ts!("\n[Example 2] Peripheral Addressing Rules");

    rprintln_ts!("\nAddressing Modes (from RM 24.3.1.7):");
    rprintln_ts!("  - Initial descriptor: Always ABSOLUTE addressing");
    rprintln_ts!("  - Linked descriptors: Can use RELATIVE or ABSOLUTE");
    rprintln_ts!("  - For peripherals: Always use ABSOLUTE addressing");
    rprintln_ts!("    to register base + offset");

    rprintln_ts!("\nPeripheral Address Calculations:");
    rprintln_ts!("  I2C0 Base: 0x4000_a000");
    rprintln_ts!("    RXDATA offset: 0x00C → 0x4000_a00C");
    rprintln_ts!("    TXDATA offset: 0x008 → 0x4000_a008");
    rprintln_ts!("  I2C1 Base: 0x4000_a400");
    rprintln_ts!("    RXDATA offset: 0x00C → 0x4000_a40C");
    rprintln_ts!("    TXDATA offset: 0x008 → 0x4000_a408");

    // ========== Example 3: Transfer Size Selection ==========
    rprintln_ts!("\n[Example 3] Transfer Size Selection by Peripheral");

    rprintln_ts!("Size: BYTE (8-bit)");
    rprintln_ts!("  Use with: I2C, USART, UART, SPI byte mode");
    rprintln_ts!("  Example: I2C RXDATA/TXDATA are 8-bit");

    rprintln_ts!("Size: HALFWORD (16-bit)");
    rprintln_ts!("  Use with: ADC/DAC (16-bit samples), SPI 16-bit mode");
    rprintln_ts!("  Example: IADC results are typically 16-bit");

    rprintln_ts!("Size: WORD (32-bit)");
    rprintln_ts!("  Use with: Memory operations, 32-bit registers");
    rprintln_ts!("  Example: Timer CC registers, memory copies");

    // ========== Example 4: Trigger vs Software Request ==========
    rprintln_ts!("\n[Example 4] Trigger Types");

    rprintln_ts!("Hardware Trigger (Peripheral-driven):");
    rprintln_ts!("  - DMA waits for peripheral request signal");
    rprintln_ts!("  - Triggered by: RX FIFO data, TX ready, ADC result");
    rprintln_ts!("  - Set by: configure_request() with DmaRequest");
    rprintln_ts!("  - Use case: I2C, UART, ADC continuous data");

    rprintln_ts!("Software Trigger (CPU-driven):");
    rprintln_ts!("  - CPU explicitly starts transfer with start_transfer()");
    rprintln_ts!("  - Triggered by: dma.start_transfer(channel)");
    rprintln_ts!("  - Use case: Memory-to-memory, one-shot transfers");
    rprintln_ts!("  - Also called: Block Request (REQMODE=BLOCK in CTRL)");

    // ========== Example 5: Synchronization Points ==========
    rprintln_ts!("\n[Example 5] Multi-Peripheral Synchronization");

    rprintln_ts!("Scenario: Receive from I2C0, transmit via I2C1");
    rprintln_ts!("  Channel 0: I2C0 RX (peripheral-triggered)");
    rprintln_ts!("  Channel 1: I2C1 TX (peripheral-triggered)");

    // Setup I2C1 TX on channel 1
    let tx_buffer = [0x12, 0x34, 0x56, 0x78, 0xAB, 0xCD, 0xEF, 0x00];
    dma.i2c1_tx_slice(1, &tx_buffer);
    rprintln_ts!("✓ Channel 0: I2C0 RX (waiting for data)");
    rprintln_ts!("✓ Channel 1: I2C1 TX (waiting for TX ready)");

    rprintln_ts!("\nWhen to use DMA channels together:");
    rprintln_ts!("  ✓ Different peripherals: Can run in parallel");
    rprintln_ts!("  ✓ Same peripheral: Use different channels");
    rprintln_ts!("  ✓ No resource conflicts: LDMA handles arbitration");

    // ========== Example 6: Practical Configuration Guide ==========
    rprintln_ts!("\n[Example 6] Quick Configuration Reference");

    rprintln_ts!("For I2C/UART RX (receive into buffer):");
    rprintln_ts!("  - Size: BYTE");
    rprintln_ts!("  - Src Inc: NONE (peripheral register)");
    rprintln_ts!("  - Dst Inc: ONE (RAM buffer)");
    rprintln_ts!("  - Trigger: Peripheral RX data available");

    rprintln_ts!("For I2C/UART TX (send from buffer):");
    rprintln_ts!("  - Size: BYTE");
    rprintln_ts!("  - Src Inc: ONE (RAM buffer)");
    rprintln_ts!("  - Dst Inc: NONE (peripheral register)");
    rprintln_ts!("  - Trigger: Peripheral TX ready");

    rprintln_ts!("For Memory-to-Memory copy:");
    rprintln_ts!("  - Size: WORD (fastest)");
    rprintln_ts!("  - Src Inc: ONE");
    rprintln_ts!("  - Dst Inc: ONE");
    rprintln_ts!("  - Trigger: Software (start_transfer)");

    // ========== Example 7: DMA Channel Selection ==========
    rprintln_ts!("\n[Example 7] DMA Channel Selection");

    rprintln_ts!("EFR32MG24 LDMA has 8 independent channels (0-7)");
    rprintln_ts!("Arbitration modes (RM 24.3.6.1):");
    rprintln_ts!("  - Channels 0-(NUMFIXED-1): Fixed Priority");
    rprintln_ts!("  - Channels NUMFIXED-7: Round Robin");
    rprintln_ts!("  - Default: All fixed (NUMFIXED=24 > 8 channels)");

    rprintln_ts!("Channel selection strategy:");
    rprintln_ts!("  - Latency-critical: Use channels 0-2 (high priority)");
    rprintln_ts!("  - Background transfers: Use channels 5-7");
    rprintln_ts!("  - Real-time data: Use channels with fixed priority");

    // Poll for I2C transfers (with timeout)
    rprintln_ts!("\n[Status] Waiting for I2C transfers...");
    let mut timeout = 0;
    let max_timeout = 500;  // 500ms

    while (dma.is_transfer_done(0) == false || dma.is_transfer_done(1) == false)
        && timeout < max_timeout
    {
        timeout += 1;
        delay.delay_ms(1);
    }

    if timeout >= max_timeout {
        rprintln_ts!("✓ I2C transfers not triggered (devices not active)");
        rprintln_ts!("  (This is expected - I2C needs external devices)");
    } else {
        if dma.is_transfer_done(0) {
            rprintln_ts!("✓ I2C0 RX transfer complete");
            dma.clear_done_flag(0);
        }
        if dma.is_transfer_done(1) {
            rprintln_ts!("✓ I2C1 TX transfer complete");
            dma.clear_done_flag(1);
        }
    }

    // Success completion
    rprintln_ts!("\n=== Peripheral DMA Information Complete ===");
    rprintln_ts!("Key learning points:");
    rprintln_ts!("✓ Each peripheral has specific SOURCESEL/SIGSEL values");
    rprintln_ts!("✓ Addressing must match register offsets (absolute)");
    rprintln_ts!("✓ Size selection depends on peripheral register width");
    rprintln_ts!("✓ Fixed vs variable increment based on fixed peripherals");
    rprintln_ts!("✓ 8 channels available with configurable arbitration");
    rprintln_ts!("✓ Peripherals can trigger DMA or software can");

    for _ in 0..5 {
        led.set_level(Level::High);
        delay.delay_ms(80);
        led.set_level(Level::Low);
        delay.delay_ms(80);
    }

    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
