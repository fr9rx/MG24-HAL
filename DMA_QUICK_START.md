# DMA Quick Start Guide - One-Liner Operations

The DMA API has been streamlined for common operations. Most transfers can now be configured in **2-3 lines** of code.

## Memory-to-Memory Copy

**One-liner:**
```rust
dma.copy(0, src_addr, dst_addr, 256);  // Copy 256 bytes
while !dma.is_transfer_done(0) {}       // Wait for completion
```

**Full example:**
```rust
let mut dma = Dma::new();
let src_addr = &src_data[0] as *const u8 as u32;
let dst_addr = &mut dst_buffer[0] as *mut u8 as u32;

dma.copy(0, src_addr, dst_addr, 16);
while !dma.is_transfer_done(0) {}
dma.clear_done_flag(0);
```

## I2C0 Receive (DMA)

**One-liner:**
```rust
dma.i2c0_rx(0, &mut rx_buffer[0] as *mut u8 as u32, 32);
while !dma.is_transfer_done(0) {}
```

No need to configure request source, transfer size, or increments—all handled automatically.

## I2C0 Transmit (DMA)

**One-liner:**
```rust
dma.i2c0_tx(0, &tx_buffer[0] as *const u8 as u32, 32);
while !dma.is_transfer_done(0) {}
```

## I2C1 Receive (DMA)

**One-liner:**
```rust
dma.i2c1_rx(0, &mut rx_buffer[0] as *mut u8 as u32, 32);
while !dma.is_transfer_done(0) {}
```

## I2C1 Transmit (DMA)

**One-liner:**
```rust
dma.i2c1_tx(0, &tx_buffer[0] as *const u8 as u32, 32);
while !dma.is_transfer_done(0) {}
```

## Complete Example: Memory-to-Memory Transfer

```rust
#![no_std]
#![no_main]

use mg24_hal::{rprintln_ts, rtt, timestamp, CpuConfig, delay::Delay, dma::Dma};

#[mg24_hal::main]
fn main() -> ! {
    rtt::init();
    timestamp::init();
    let dp = mg24_hal::init(CpuConfig::default().with_ldma_clock(true)).unwrap();
    let delay = Delay::new();

    let src_data = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let mut dst_buffer = [0u8; 16];
    let mut dma = Dma::new();

    // ONE-LINER: Copy 16 bytes via DMA
    dma.copy(0, &src_data[0] as *const u8 as u32, &mut dst_buffer[0] as *mut u8 as u32, 16);
    
    // Wait and verify
    while !dma.is_transfer_done(0) { delay.delay_us(100); }
    rprintln_ts!("Transfer complete!");
    
    loop { delay.delay_ms(500); }
}
```

## Comparison: Before vs After

### Before (Complex)
```rust
let dma_config = DmaConfig::default()
    .with_channel(0)
    .with_size(DmaSize::Byte)
    .with_src_inc(DmaIncrement::None)
    .with_dst_inc(DmaIncrement::One)
    .with_block_size(1);

dma.configure_request(0, DmaRequest::I2c0Rx);
dma.configure_transfer(0, 0x4000_a00C, buf_addr, 32, &dma_config);
dma.enable_channel(0);
```

### After (Simple)
```rust
dma.i2c0_rx(0, buf_addr, 32);
```

## Available Quick Methods

All convenience methods follow this pattern:

```
dma.operation(channel, param1, param2);
while !dma.is_transfer_done(channel) {}
dma.clear_done_flag(channel);  // Optional but recommended
```

| Method | Parameters | Typical Use |
|--------|-----------|-------------|
| `copy()` | ch, src, dst, count | RAM-to-RAM transfers |
| `i2c0_rx()` | ch, dst, count | I2C0 read via DMA |
| `i2c0_tx()` | ch, src, count | I2C0 write via DMA |
| `i2c1_rx()` | ch, dst, count | I2C1 read via DMA |
| `i2c1_tx()` | ch, src, count | I2C1 write via DMA |

## Advanced Configuration (Still Available)

If you need custom transfer parameters:

```rust
let config = DmaConfig::default()
    .with_size(DmaSize::Word)
    .with_src_inc(DmaIncrement::Four)
    .with_dst_inc(DmaIncrement::Two);

dma.configure_transfer(0, src, dst, count, &config);
dma.enable_channel(0);
dma.start_transfer(0);
```

## Running Examples

```bash
# Memory-to-memory transfer
cargo build --example dma_m2m
probe-rs run --chip EFR32MG24B220F1536IM48 target/.../examples/dma_m2m

# I2C0 RX via DMA
cargo build --example i2c_dma
probe-rs run --chip EFR32MG24B220F1536IM48 target/.../examples/i2c_dma
```

## Key Features

✅ **Minimal Boilerplate** - Most operations in 1-3 lines
✅ **Type Safe** - All pointer conversions handled correctly
✅ **Well Documented** - Inline examples for each method
✅ **Flexible** - Advanced configuration still available when needed
✅ **Verified** - All register writes match reference manual exactly

## Status Checking

All DMA operations provide simple status methods:

```rust
// Check if specific channel completed
if dma.is_transfer_done(0) {
    dma.clear_done_flag(0);
}

// Check if any channel is active
if dma.any_busy() {
    // Still transferring
}

// Get all channel status (one byte, 8 channels)
let status = dma.channel_status();
```

## Error Handling

Always use timeouts when polling:

```rust
let mut timeout = 0;
while !dma.is_transfer_done(0) && timeout < 1000 {
    delay.delay_us(100);
    timeout += 1;
}

if timeout >= 1000 {
    rprintln!("DMA transfer timeout!");
}
```

## Performance Notes

- **Memory-to-Memory**: Up to 32-bit words per cycle
- **Peripheral Transfers**: Limited by I2C speed and FIFO depth
- **Power Efficiency**: LDMA runs independently, CPU can sleep
- **No CPU Overhead**: Zero interrupts needed for simple polling

## Next Steps

1. See `DMA_IMPLEMENTATION.md` for detailed register documentation
2. Check `examples/dma_m2m.rs` for complete M2M example
3. Check `examples/i2c_dma.rs` for I2C integration example
4. Review `src/dma.rs` for all available methods
