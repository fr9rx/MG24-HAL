# DMA Quick Start Guide - Safe One-Liner Operations

The DMA API has been streamlined for common operations. Most transfers can now be configured in **2-3 lines** of code with **zero unsafe pointer casts**.

## Safe API (Recommended)

Pass references directly - the API extracts addresses safely:

### Memory-to-Memory Copy

**One-liner (Safe):**
```rust
dma.copy_slice(0, &src_data, &mut dst_buffer);  // No pointer casting!
while !dma.is_transfer_done(0) {}
```

### I2C0 Receive

**One-liner (Safe):**
```rust
dma.i2c0_rx_slice(0, &mut rx_buffer);
while !dma.is_transfer_done(0) {}
```

### I2C0 Transmit

**One-liner (Safe):**
```rust
dma.i2c0_tx_slice(0, &tx_buffer);
while !dma.is_transfer_done(0) {}
```

### I2C1 Receive

**One-liner (Safe):**
```rust
dma.i2c1_rx_slice(0, &mut rx_buffer);
while !dma.is_transfer_done(0) {}
```

### I2C1 Transmit

**One-liner (Safe):**
```rust
dma.i2c1_tx_slice(0, &tx_buffer);
while !dma.is_transfer_done(0) {}
```

## Address-Based API (When Needed)

If you already have raw addresses or need advanced control:

```rust
// Use _addr variants when you have pointers
dma.copy_addr(0, src_addr, dst_addr, 256);
dma.i2c0_rx_addr(0, dst_addr, 32);
dma.i2c0_tx_addr(0, src_addr, 32);
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

    // ONE-LINER: Safe API - no pointer casting needed!
    dma.copy_slice(0, &src_data, &mut dst_buffer);
    
    // Wait and verify
    while !dma.is_transfer_done(0) { delay.delay_us(100); }
    rprintln_ts!("Transfer complete!");
    
    loop { delay.delay_ms(500); }
}
```

## Comparison: Before vs After

### Before (6-7 lines with boilerplate)
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

### After (1 line, safe, no casting)
```rust
dma.i2c0_rx_slice(0, &mut rx_buffer);
```

## Available Quick Methods

### Safe Methods (Take References - Recommended)

| Method | Parameters | Typical Use |
|--------|-----------|-------------|
| `copy_slice()` | ch, &src, &mut dst | RAM-to-RAM transfers |
| `i2c0_rx_slice()` | ch, &mut dst | I2C0 read via DMA |
| `i2c0_tx_slice()` | ch, &src | I2C0 write via DMA |
| `i2c1_rx_slice()` | ch, &mut dst | I2C1 read via DMA |
| `i2c1_tx_slice()` | ch, &src | I2C1 write via DMA |

### Address Methods (Raw Pointers)

| Method | Parameters | When to Use |
|--------|-----------|-------------|
| `copy_addr()` | ch, src_addr, dst_addr, count | Already have addresses |
| `i2c0_rx_addr()` | ch, dst_addr, count | Advanced use cases |
| `i2c0_tx_addr()` | ch, src_addr, count | Advanced use cases |
| `i2c1_rx_addr()` | ch, dst_addr, count | Advanced use cases |
| `i2c1_tx_addr()` | ch, src_addr, count | Advanced use cases |

All safe methods automatically:
- Extract addresses from references
- Determine transfer count from slice length
- Configure peripheral request sources
- Set transfer size and increments
- Enable channels

## Safety Improvements

### Safe API (No Casting)
```rust
// Recommended - compiler extracts address safely
let src_data = [1, 2, 3, 4, 5];
let mut dst_buffer = [0u8; 5];
dma.copy_slice(0, &src_data, &mut dst_buffer);
```

### Address API (When Needed)
```rust
// Use only if you have raw addresses from elsewhere
let src_addr = 0x2000_0000u32;
let dst_addr = 0x2000_0100u32;
dma.copy_addr(0, src_addr, dst_addr, 256);
```

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
# Memory-to-memory transfer (safe API)
cargo build --example dma_m2m
probe-rs run --chip EFR32MG24B220F1536IM48 target/.../examples/dma_m2m

# I2C0 RX via DMA (safe API)
cargo build --example i2c_dma
probe-rs run --chip EFR32MG24B220F1536IM48 target/.../examples/i2c_dma
```

## Key Features

✅ **Zero Unsafe Pointer Casts** - Safe references for common operations
✅ **Minimal Boilerplate** - Most operations in 1-3 lines
✅ **Automatic Configuration** - No need to specify sizes or increments
✅ **Slice Length Support** - Transfer count derived from slice length
✅ **Type Safe** - Rust's type system ensures safety
✅ **Flexible** - Address-based API still available when needed
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
- **Zero Overhead Abstractions**: Safe API compiles to same code as unsafe

## Migration from Old API

Old API still works but is deprecated:
```rust
// Old (deprecated but still works)
dma.copy(0, src_addr, dst_addr, 256);

// New (recommended)
dma.copy_slice(0, &src_data, &mut dst_buffer);
```

## Next Steps

1. See `DMA_IMPLEMENTATION.md` for detailed register documentation
2. Check `examples/dma_m2m.rs` for complete M2M example
3. Check `examples/i2c_dma.rs` for I2C integration example
4. Review `src/dma.rs` for all available methods
