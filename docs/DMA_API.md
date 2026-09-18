# DMA (LDMA) API Reference

Complete documentation of the LDMA (Linked Direct Memory Access) module with 8 independent channels for high-speed memory and peripheral transfers.

## Module Overview

The DMA module provides safe reference-based and address-based APIs for configuring the EFR32MG24's 8 LDMA channels.

**Location**: `src/dma.rs`  
**Types**: `Dma`, `DmaError`, `DmaTransferConfig`

**Key Concept**: LDMA is a **separate DMA engine** outside CPU—transfers run independently after configuration.

## DMA Controller Initialization

### Creating a DMA Controller

```rust
use mg24_hal::dma::Dma;

let dma = Dma::new(dp.ldma);

// Now has access to 8 independent channels: 0-7
```

### DMA Architecture

```
EFR32MG24 LDMA (8 channels)
├─ Channel 0: Independent descriptor list
├─ Channel 1: Independent descriptor list
├─ ...
├─ Channel 7: Independent descriptor list
└─ Each channel: DMA_REQ_SIGNAL (per-peripheral)
```

## Safe Reference-Based API

The safe API accepts Rust references and validates memory regions automatically.

### Memory-to-Memory Transfer

```rust
pub fn copy_slice(
    &mut self,
    channel: usize,
    src: &[u8],
    dst: &mut [u8]
) -> Result<(), DmaError> {
    // Validate: src and dst don't overlap
    // Validate: same length
    // Validate: channel 0-7
    // Configure transfer
}
```

**Parameters**:
- `channel`: DMA channel 0-7
- `src`: Source data slice (immutable reference)
- `dst`: Destination buffer (mutable reference)

**Returns**:
- `Ok(())` - Transfer configured successfully
- `Err(DmaError)` - See error section

**Example**:
```rust
let source = [1, 2, 3, 4];
let mut dest = [0u8; 4];

dma.copy_slice(0, &source, &mut dest)?;

// Wait for completion
while !dma.is_transfer_done(0).unwrap_or(false) {
    // Poll or sleep
}

assert_eq!(dest, [1, 2, 3, 4]);
```

### I2C Receive (Peripheral to Memory)

```rust
pub fn i2c0_rx_slice(
    &mut self,
    channel: usize,
    dst: &mut [u8]
) -> Result<(), DmaError> {
    // Configure channel to receive from I2C0 RXDATA
    // Write into destination buffer
}
```

**Example**:
```rust
let mut i2c_data = [0u8; 32];
dma.i2c0_rx_slice(0, &mut i2c_data)?;

// Perform I2C read
i2c.read(addr, &mut i2c_data)?;

// Wait for DMA completion
while !dma.is_transfer_done(0).unwrap_or(false) { }
```

### I2C Transmit (Memory to Peripheral)

```rust
pub fn i2c0_tx_slice(
    &mut self,
    channel: usize,
    src: &[u8]
) -> Result<(), DmaError> {
    // Configure channel to transmit to I2C0 TXDATA
    // Read from source buffer
}
```

**Example**:
```rust
let config = [0x80, 0xAD, 0xD5, 0x00];
dma.i2c0_tx_slice(0, &config)?;

// Perform I2C write
i2c.write(addr, &config)?;

// Wait for completion
while !dma.is_transfer_done(0).unwrap_or(false) { }
```

### UART RX/TX

```rust
// Receive from UART0 into memory
pub fn uart0_rx_slice(&mut self, channel: usize, dst: &mut [u8]) 
    -> Result<(), DmaError> { }

// Transmit from memory to UART0
pub fn uart0_tx_slice(&mut self, channel: usize, src: &[u8])
    -> Result<(), DmaError> { }
```

## Advanced Address-Based API

For scenarios where references aren't available (external memory, computed addresses), use the address-based API:

```rust
pub fn copy_addr(
    &mut self,
    channel: usize,
    src_addr: *const u8,
    dst_addr: *mut u8,
    count: usize
) -> Result<(), DmaError> {
    // Validate: src and dst don't overlap (if not disjoint)
    // Validate: count > 0 and <= 16K
    // Validate: addresses aligned
    // Configure transfer
}
```

**Parameters**:
- `channel`: DMA channel 0-7
- `src_addr`: Source address (must be valid)
- `dst_addr`: Destination address (must be writable)
- `count`: Number of bytes to transfer

**Returns**:
- `Ok(())` - Transfer configured
- `Err(DmaError::OverlappingAddresses)` - Regions overlap
- `Err(DmaError::InvalidTransferCount)` - count invalid

**Safety Requirements**:
- Addresses must point to valid, accessible memory
- Region sizes must match count
- Overlapping transfers must use non-overlapping regions

**Example**:
```rust
// Transfer from external flash to RAM
const FLASH_BASE: usize = 0x0E000000;
const RAM_BASE: usize = 0x20000000;

let src = FLASH_BASE as *const u8;
let dst = RAM_BASE as *mut u8;

dma.copy_addr(0, src, dst, 256)?;
```

## Transfer Status and Control

### Check Transfer Completion

```rust
pub fn is_transfer_done(&self, channel: usize) -> Result<bool, DmaError> {
    // Returns Ok(true) if transfer finished
    // Returns Ok(false) if transfer in progress
    // Returns Err if channel invalid
}
```

**Example**:
```rust
// Wait with timeout
let timeout = 1_000_000;
let mut wait = 0;
while !dma.is_transfer_done(0).unwrap_or(false) && wait < timeout {
    wait += 1;
}

if wait >= timeout {
    eprintln!("DMA timeout");
}
```

### Enable/Disable Channel

```rust
pub fn enable_channel(&mut self, channel: usize) -> Result<(), DmaError> { }
pub fn disable_channel(&mut self, channel: usize) -> Result<(), DmaError> { }
```

## Error Types

### DmaError Enum

```rust
pub enum DmaError {
    InvalidChannel,          // channel > 7
    InvalidTransferCount,    // count == 0 or > 65535
    InvalidBlockSize,        // blocksize invalid
    OverlappingAddresses,    // src and dst overlap
    TransferCountTooLarge,   // count > LDMA maximum
    InvalidSize,            // size not power of 2
    UnsupportedOperation,   // operation not available
}
```

### Error Handling

```rust
match dma.copy_slice(0, &src, &mut dst) {
    Ok(()) => rprintln!("DMA configured"),
    Err(DmaError::InvalidChannel) => {
        rprintln!("Channel must be 0-7");
    }
    Err(DmaError::OverlappingAddresses) => {
        rprintln!("Source and destination overlap");
    }
    Err(e) => {
        rprintln!("DMA error: {:?}", e);
    }
}
```

## Channel Limitations

| Constraint | Limit | Notes |
|-----------|-------|-------|
| Channels | 8 (0-7) | One per channel, independent |
| Max Transfer | 65,535 bytes | 16-bit counter |
| Alignment | 1, 2, 4 bytes | Per transfer width |
| Overlapping | Not allowed | Use separate channels |

## Concurrent Transfers

Each of the 8 channels is fully independent—use multiple channels for concurrent transfers:

```rust
// Transfer 4 independent regions simultaneously
dma.copy_slice(0, &src1, &mut dst1)?;  // Channel 0
dma.copy_slice(1, &src2, &mut dst2)?;  // Channel 1
dma.copy_slice(2, &src3, &mut dst3)?;  // Channel 2
dma.copy_slice(3, &src4, &mut dst4)?;  // Channel 3

// All 4 transfers run in parallel on hardware
loop {
    let done1 = dma.is_transfer_done(0).unwrap_or(false);
    let done2 = dma.is_transfer_done(1).unwrap_or(false);
    let done3 = dma.is_transfer_done(2).unwrap_or(false);
    let done4 = dma.is_transfer_done(3).unwrap_or(false);
    
    if done1 && done2 && done3 && done4 {
        break;
    }
}
```

## Peripheral Integration

### I2C DMA Transfer

```rust
// DMA-assisted I2C data reception
let mut i2c_buffer = [0u8; 64];

// Configure DMA to receive from I2C0
dma.i2c0_rx_slice(0, &mut i2c_buffer)?;

// Initiate I2C read operation
i2c.read(device_addr, &mut i2c_buffer)?;

// DMA and I2C work in parallel
while !dma.is_transfer_done(0).unwrap_or(false) {
    // Optionally do other work
}

// Data now in i2c_buffer via DMA
```

### UART DMA Transfer

```rust
let mut uart_rx_buffer = [0u8; 128];

// DMA receives from UART0
dma.uart0_rx_slice(0, &mut uart_rx_buffer)?;

// Perform UART receive
// DMA moves data from UART RXDATA -> RAM

while !dma.is_transfer_done(0).unwrap_or(false) { }
```

## Common Patterns

### Fill Pattern (Repeated Value)

```rust
// To fill a buffer with a repeated value, use copy_slice from source:
let pattern = [0xAA, 0xBB, 0xCC, 0xDD];
let mut large_buffer = [0u8; 1024];

// Copy pattern to buffer (may need loop for full buffer)
dma.copy_slice(0, &pattern, &mut large_buffer[0..4])?;
```

### Chained Transfers

```rust
// Sequential transfers on same channel (not linked descriptor list)
// Transfer 1
dma.copy_slice(0, &src1, &mut dst1)?;
while !dma.is_transfer_done(0).unwrap_or(false) { }

// Transfer 2
dma.copy_slice(0, &src2, &mut dst2)?;
while !dma.is_transfer_done(0).unwrap_or(false) { }
```

### High-Throughput Data Pipeline

```rust
// Ping-pong buffer pattern using 2 channels
let mut buffer1 = [0u8; 512];
let mut buffer2 = [0u8; 512];
let source = [0u8; 1024];

// Start receiving into buffer1 on channel 0
dma.copy_slice(0, &source[0..512], &mut buffer1)?;

loop {
    // Process buffer1 while buffer2 receives
    dma.copy_slice(1, &source[512..1024], &mut buffer2)?;
    
    // Wait for buffer2
    while !dma.is_transfer_done(1).unwrap_or(false) { }
    
    // Swap
    // ...
}
```

## Performance Characteristics

### Throughput

- **Peak bandwidth**: 32 MB/s (typical for EFR32MG24)
- **CPU overhead**: Minimal (setup only)
- **Latency**: < 1 microsecond per transfer

### Power Considerations

- **Active transfer**: Full current (typical 10-30 mA)
- **Idle**: No current when disabled
- **Sleep modes**: Disable DMA before entering deep sleep

## Hardware Details (EFR32MG24)

### LDMA Base Address
- **Address**: 0x4008_0000
- **Reference**: EFR32MG24 Reference Manual, Section 24

### Channel Configuration

| Register | Offset | Per-Channel | Purpose |
|----------|--------|-------------|---------|
| CH_CTRL | 0x500 | Yes | Channel control |
| CH_CFG | 0x504 | Yes | Channel configuration |
| CH_SRC | 0x508 | Yes | Source address |
| CH_DST | 0x50C | Yes | Destination address |
| CH_LEN | 0x510 | Yes | Transfer length |

### LDMA Interrupt Sources

```
DONE[0..7]     - Channel 0-7 transfer complete
ERROR          - Error detected (address, mode)
REQPEND[0..15] - Pending DMA requests
```

## Debugging Tips

| Issue | Cause | Fix |
|-------|-------|-----|
| Transfer doesn't complete | Incorrect addresses | Verify src/dst validity |
| Invalid channel error | Channel > 7 | Use 0-7 |
| Overlapping error | Regions overlap | Use separate channels |
| Slow transfer | Peripheral bottleneck | Check peripheral speed |

## Integration with IRQ Handler

```rust
static DMA_STATUS: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

#[mg24_hal::interrupt]
fn LDMA() {
    // DMA complete - set flag
    interrupt::free(|cs| {
        DMA_STATUS.borrow_mut() = true;
    });
}

// In main:
dma.copy_slice(0, &src, &mut dst)?;

// Wait for interrupt-driven completion
interrupt::free(|cs| {
    while !*DMA_STATUS.borrow() { }
    *DMA_STATUS.borrow_mut() = false;
});
```

---

**Version**: 2.0.0  
**Last Updated**: 2026-09-18
