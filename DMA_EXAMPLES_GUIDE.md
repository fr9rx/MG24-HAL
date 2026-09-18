# DMA Examples Guide - Complete Overview

Comprehensive guide to all DMA examples with progressively advanced features.

## Example Progression

```
dma_m2m.rs
  ↓ (safe references)
dma_unsafe_addr.rs
  ↓ (raw addresses)
dma_advanced.rs
  ↓ (multiple channels, custom configs)
dma_peripherals.rs
  ↓ (peripheral integration reference)
```

## 1. Basic: Safe Memory-to-Memory (dma_m2m.rs)

**Difficulty:** Beginner  
**Time to run:** ~5 seconds  
**Files:** examples/dma_m2m.rs

### What it demonstrates:
- Safe reference-based API (no pointer casting)
- Simple memory copy via DMA
- 16-byte transfer with verification
- Status polling with timeout
- LED feedback

### Key concepts:
```rust
// Copy using safe references
dma.copy_slice(0, &src_data, &mut dst_buffer);

// Poll for completion
while !dma.is_transfer_done(0) {}

// Verify data matches
for i in 0..16 {
    assert_eq!(src_data[i], dst_buffer[i]);
}
```

### When to use:
- ✅ Learning the basics
- ✅ Simple RAM-to-RAM copies
- ✅ Bulk memory operations
- ✅ Production code (safe API)

### Success criteria:
- DMA transfer completes without error
- LED blinks 5 times
- Data verification shows all bytes match

---

## 2. Intermediate: Unsafe Address-Based API (dma_unsafe_addr.rs)

**Difficulty:** Intermediate  
**Time to run:** ~2 seconds  
**Files:** examples/dma_unsafe_addr.rs

### What it demonstrates:
- Address-based DMA API (when you have raw pointers)
- Multiple DMA examples in one program
- Custom DMA configuration with different settings
- Peripheral register addressing (I2C0 RXDATA)
- DMA status checking and monitoring

### Key concepts:
```rust
// Example 1: Raw address copy
dma.copy_addr(0, src_addr, dst_addr, 256);

// Example 2: Peripheral register access
let rxdata_addr = 0x4000_a00C;  // I2C0 RXDATA
dma.i2c0_rx_addr(1, rxdata_addr, 32);

// Example 3: Custom configuration
let config = DmaConfig::default()
    .with_size(DmaSize::Word)
    .with_src_inc(DmaIncrement::Four)
    .with_dst_inc(DmaIncrement::Two);
dma.configure_transfer(2, src, dst, count, &config);
```

### When to use:
- ✅ When you have raw hardware addresses
- ✅ Peripheral memory-mapped I/O
- ✅ Custom transfer configurations
- ⚠️ Requires careful address validation
- ⚠️ Unsafe - must verify correctness manually

### Success criteria:
- 3 examples run without error
- Status checks show correct information
- Timeouts handled gracefully for missing peripherals

### Use case example:
```rust
// Copying between fixed RAM regions
let region1 = 0x2000_0000u32;
let region2 = 0x2000_0100u32;
dma.copy_addr(0, region1, region2, 256);
```

---

## 3. Advanced: Multiple Concurrent Channels (dma_advanced.rs)

**Difficulty:** Advanced  
**Time to run:** ~1 second  
**Files:** examples/dma_advanced.rs

### What it demonstrates:
- Packed/unpacked data conversion (format conversion via DMA)
- Multiple concurrent DMA channels
- Different block sizes for different use cases
- 8-bit, 16-bit, 32-bit transfers simultaneously
- Complex status monitoring
- Timing optimization strategies

### Key concepts:

#### Packed/Unpacked Conversion
```rust
// Convert word-aligned bytes to packed format
// Source (word-aligned): [XX, YY, 00, ZZ, ...]
// Destination (packed):  [XX, YY, ZZ, ...]

let config = DmaConfig::default()
    .with_size(DmaSize::Byte)
    .with_src_inc(DmaIncrement::Four)  // Skip 4 bytes per transfer
    .with_dst_inc(DmaIncrement::One);  // Write contiguously
```

#### Concurrent Channels
```rust
// Channel 1: 32-bit transfers, 4 units per block
dma.configure_transfer(1, src1, dst1, 64, config1);

// Channel 2: 16-bit transfers, 8 units per block
dma.configure_transfer(2, src2, dst2, 128, config2);

// Channel 3: 8-bit transfers, 16 units per block
dma.configure_transfer(3, src3, dst3, 256, config3);

// All run simultaneously!
dma.enable_channel(1);
dma.enable_channel(2);
dma.enable_channel(3);
```

#### Block Size Effects
```rust
// Block size controls units per arbitration cycle
// Low block size (1-2):
//   - Low latency
//   - Better for real-time
//   - Lower throughput

// High block size (64-128):
//   - High throughput
//   - Longer latency
//   - Better for bulk transfers
```

### When to use:
- ✅ Multiple parallel transfers needed
- ✅ Format conversion (packed/unpacked)
- ✅ Optimizing for specific latency/throughput tradeoff
- ✅ Time-critical multi-channel sync
- ✅ Complex data transformations via DMA

### Success criteria:
- All 3 concurrent transfers complete
- Different sizes (8-bit, 16-bit, 32-bit) work together
- Status monitoring shows correct arbitration
- Packed/unpacked conversion data is correct

### Real-world example:
```rust
// Receive packed ADC data in 16-bit format
// Convert to 32-bit format in real-time via DMA
dma.configure_transfer(ch, adc_source, ram_dest, count, config);
```

---

## 4. Reference: Peripheral Integration (dma_peripherals.rs)

**Difficulty:** Reference/Educational  
**Time to run:** ~1 second  
**Files:** examples/dma_peripherals.rs

### What it demonstrates:
- DMA request source table for all EFR32MG24 peripherals
- I2C peripheral addressing and configuration
- Transfer size selection rules
- Hardware vs software trigger mechanisms
- Channel arbitration and priority
- Multi-peripheral synchronization
- Configuration quick reference

### Key learning sections:

#### 1. Peripheral Sources
```
SOURCESEL values:
0x0-0x1: USART0, USART1
0x5-0x6: I2C0, I2C1
0xA: IADC0
0xC-0xE: TIMER0-4
0x11-0x12: VDAC0, VDAC1
```

#### 2. I2C Addressing
```
I2C0:
  Base: 0x4000_a000
  RXDATA: 0x4000_a00C
  TXDATA: 0x4000_a008

I2C1:
  Base: 0x4000_a400
  RXDATA: 0x4000_a40C
  TXDATA: 0x4000_a408
```

#### 3. Trigger Types
```
Hardware Trigger (Peripheral-driven):
- Peripheral signals data ready
- Example: I2C has byte in FIFO

Software Trigger (CPU-driven):
- CPU explicitly starts transfer
- Example: start_transfer() call
```

#### 4. Configuration Rules
```
For I2C RX:
  Size: BYTE
  Src Inc: NONE
  Dst Inc: ONE
  Trigger: Peripheral RX

For I2C TX:
  Size: BYTE
  Src Inc: ONE
  Dst Inc: NONE
  Trigger: Peripheral TX
```

### When to use:
- 📖 Reference during implementation
- 📖 Understanding peripheral DMA support
- 📖 Selecting correct SOURCESEL/SIGSEL values
- 📖 Configuration quick lookup

### No hardware needed:
This example runs without external devices - it's purely informational.

---

## Running the Examples

### Build single example
```bash
cargo build --example dma_m2m
cargo build --example dma_unsafe_addr
cargo build --example dma_advanced
cargo build --example dma_peripherals
```

### Run example
```bash
probe-rs run --chip EFR32MG24B220F1536IM48 target/thumbv8m.main-none-eabihf/debug/examples/dma_m2m
```

### Build all examples
```bash
cargo build --examples
```

### View RTT output
```bash
probe-rs attach --chip EFR32MG24B220F1536IM48
```

---

## DMA API Quick Reference

### Safe Slice-Based (Recommended)
```rust
dma.copy_slice(ch, &src, &mut dst);
dma.i2c0_rx_slice(ch, &mut buffer);
dma.i2c0_tx_slice(ch, &buffer);
dma.i2c1_rx_slice(ch, &mut buffer);
dma.i2c1_tx_slice(ch, &buffer);
```

### Address-Based (When Needed)
```rust
dma.copy_addr(ch, src_addr, dst_addr, count);
dma.i2c0_rx_addr(ch, dst_addr, count);
dma.i2c0_tx_addr(ch, src_addr, count);
dma.i2c1_rx_addr(ch, dst_addr, count);
dma.i2c1_tx_addr(ch, src_addr, count);
```

### Custom Configuration
```rust
let config = DmaConfig::default()
    .with_size(DmaSize::Word)
    .with_src_inc(DmaIncrement::Four)
    .with_dst_inc(DmaIncrement::One)
    .with_block_size(8);

dma.configure_transfer(ch, src, dst, count, &config);
dma.enable_channel(ch);
dma.start_transfer(ch);
```

### Status Checking
```rust
if dma.is_transfer_done(ch) { dma.clear_done_flag(ch); }
if dma.any_busy() { /* Still transferring */ }
let status = dma.channel_status();
```

---

## Feature Matrix

| Feature | dma_m2m | dma_unsafe_addr | dma_advanced | dma_peripherals |
|---------|---------|-----------------|--------------|-----------------|
| Safe API | ✅ | ⚠️ | ⚠️ | ✅ |
| Raw addresses | ❌ | ✅ | ✅ | ❌ |
| Single channel | ✅ | ✅ | ✅ | ✅ |
| Multi-channel | ❌ | ❌ | ✅ | ✅ |
| Custom config | ❌ | ✅ | ✅ | ❌ |
| I2C integration | ❌ | ✅ | ❌ | ✅ |
| Format conversion | ❌ | ❌ | ✅ | ❌ |
| Status monitoring | ✅ | ✅ | ✅ | ✅ |
| Hardware devices needed | ❌ | ⚠️ | ❌ | ❌ |
| Educational | ⭐ | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |

---

## Troubleshooting

### Transfer doesn't complete
- ✅ Verify addresses are valid
- ✅ Check timeout value isn't too short
- ✅ Ensure peripheral is initialized (for I2C)
- ✅ Check DMA channel isn't already in use

### Data corruption
- ✅ Verify src and dst don't overlap
- ✅ Check transfer count matches buffer size
- ✅ Ensure addresses are correctly aligned
- ✅ Verify no CPU writes during transfer

### Multiple transfers conflict
- ✅ Use different channels (0-7 available)
- ✅ Wait for previous transfer to complete
- ✅ Check LDMA clock is enabled

### Pointer casting errors
- ✅ Use safe slice-based API (copy_slice, i2c0_rx_slice)
- ✅ If using addresses: cast as `as *const u8 as u32`

---

## Next Steps

1. Start with **dma_m2m** - understand the basics
2. Move to **dma_unsafe_addr** - learn address API
3. Study **dma_advanced** - master concurrent transfers
4. Reference **dma_peripherals** - for peripheral integration

Then check:
- **DMA_QUICK_START.md** - API quick reference
- **DMA_IMPLEMENTATION.md** - Register details
- **src/dma.rs** - Full API documentation

---

## Key Takeaways

✅ **Safety**: Prefer slice-based API (copy_slice, i2c0_rx_slice)  
✅ **Flexibility**: Address-based API when needed  
✅ **Power**: 8 independent channels with flexible arbitration  
✅ **Integration**: Built for EFR32MG24 peripherals  
✅ **Performance**: Zero-overhead abstractions  
✅ **Documentation**: Register-verified implementation  

The DMA module is production-ready for:
- Memory-to-memory copies
- I2C/UART/SPI transfers
- Peripheral data acquisition
- Format conversion
- High-performance systems
