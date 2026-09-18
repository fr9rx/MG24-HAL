# DMA (LDMA) API - Agent/Developer Specification

Complete technical specification for LDMA (Linked Direct Memory Access) module. Authoritative reference for agents implementing DMA features.

## Module Location
`src/dma.rs`

## Type Signatures

### Dma
Main LDMA controller.

```rust
pub struct Dma;

impl Dma {
    pub fn new() -> Self
    
    // Safe reference-based API
    pub fn copy_slice(
        &mut self,
        channel: usize,
        src: &[u8],
        dst: &mut [u8]
    ) -> Result<(), DmaError>
    
    pub fn i2c0_rx_slice(&mut self, channel: usize, dst: &mut [u8]) 
        -> Result<(), DmaError>
    pub fn i2c0_tx_slice(&mut self, channel: usize, src: &[u8])
        -> Result<(), DmaError>
    
    pub fn i2c1_rx_slice(&mut self, channel: usize, dst: &mut [u8])
        -> Result<(), DmaError>
    pub fn i2c1_tx_slice(&mut self, channel: usize, src: &[u8])
        -> Result<(), DmaError>
    
    pub fn uart0_rx_slice(&mut self, channel: usize, dst: &mut [u8])
        -> Result<(), DmaError>
    pub fn uart0_tx_slice(&mut self, channel: usize, src: &[u8])
        -> Result<(), DmaError>
    
    // Advanced address-based API
    pub fn copy_addr(
        &mut self,
        channel: usize,
        src: *const u8,
        dst: *mut u8,
        count: usize
    ) -> Result<(), DmaError>
    
    pub fn i2c0_rx_addr(&mut self, channel: usize, dst: *mut u8, count: usize)
        -> Result<(), DmaError>
    pub fn i2c0_tx_addr(&mut self, channel: usize, src: *const u8, count: usize)
        -> Result<(), DmaError>
    
    pub fn i2c1_rx_addr(&mut self, channel: usize, dst: *mut u8, count: usize)
        -> Result<(), DmaError>
    pub fn i2c1_tx_addr(&mut self, channel: usize, src: *const u8, count: usize)
        -> Result<(), DmaError>
    
    pub fn uart0_rx_addr(&mut self, channel: usize, dst: *mut u8, count: usize)
        -> Result<(), DmaError>
    pub fn uart0_tx_addr(&mut self, channel: usize, src: *const u8, count: usize)
        -> Result<(), DmaError>
    
    // Control
    pub fn enable_channel(&mut self, channel: usize) -> Result<(), DmaError>
    pub fn disable_channel(&mut self, channel: usize) -> Result<(), DmaError>
    pub fn is_transfer_done(&self, channel: usize) -> Result<bool, DmaError>
    pub fn clear_done_flag(&mut self, channel: usize) -> Result<(), DmaError>
    
    // Configuration
    pub fn configure_transfer(
        &mut self,
        channel: u8,
        src: u32,
        dst: u32,
        count: u16,
        config: &DmaConfig
    ) -> Result<(), DmaError>
    
    pub fn start_transfer(&mut self, channel: u8) -> Result<(), DmaError>
    pub fn channel_status(&self) -> u8
    pub fn any_busy(&self) -> bool
}
```

## Configuration Types

### DmaConfig
```rust
pub struct DmaConfig {
    size: DmaSize,
    src_inc: DmaIncrement,
    dst_inc: DmaIncrement,
    block_size: u16,
}

impl DmaConfig {
    pub fn default() -> Self
    pub fn with_size(self, size: DmaSize) -> Self
    pub fn with_src_inc(self, inc: DmaIncrement) -> Self
    pub fn with_dst_inc(self, inc: DmaIncrement) -> Self
    pub fn with_block_size(self, size: u16) -> Self
}
```

### DmaSize Enum
```rust
pub enum DmaSize {
    Byte,     // 8-bit transfers
    HalfWord, // 16-bit transfers
    Word,     // 32-bit transfers
}
```

### DmaIncrement Enum
```rust
pub enum DmaIncrement {
    None,  // No address increment
    One,   // Increment by 1 unit
    Two,   // Increment by 2 units
    Four,  // Increment by 4 units
}

// Units are per DmaSize:
// Byte mode:     One=1 byte, Two=2 bytes, Four=4 bytes
// HalfWord mode: One=2 bytes, Two=4 bytes, Four=8 bytes
// Word mode:     One=4 bytes, Two=8 bytes, Four=16 bytes
```

## Error Types

### DmaError Enum
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaError {
    /// Channel number outside 0-7
    InvalidChannel,
    
    /// Transfer count is 0 or exceeds 65535
    InvalidTransferCount,
    
    /// Block size invalid (must be power of 2 or <= 65535)
    InvalidBlockSize,
    
    /// Source and destination regions overlap
    OverlappingAddresses,
    
    /// Transfer count exceeds 16-bit limit (65535)
    TransferCountTooLarge,
    
    /// Size configuration invalid
    InvalidSize,
    
    /// Operation not supported on this channel/peripheral
    UnsupportedOperation,
}
```

**Error Conditions:**

| Error | Cause | Fix |
|-------|-------|-----|
| InvalidChannel | channel > 7 | Use 0-7 |
| InvalidTransferCount | count == 0 or > 65535 | Use 1-65535 bytes |
| OverlappingAddresses | src and dst overlap | Use non-overlapping regions |
| TransferCountTooLarge | count > 65535 | Split into multiple transfers |
| InvalidBlockSize | block_size invalid | Use power of 2 or <= 65535 |

## Hardware

### LDMA Base: 0x4008_0000

### Channel Register Layout

Each channel has registers at offset `0x500 + (ch * 0x10)`:

| Offset | Register | Width | Mode | Purpose |
|--------|----------|-------|------|---------|
| 0x500 | CH_CTRL | 32 | RW | Channel control |
| 0x504 | CH_CFG | 32 | RW | Channel config |
| 0x508 | CH_SRC | 32 | RW | Source address |
| 0x50C | CH_DST | 32 | RW | Destination address |
| 0x510 | CH_LEN | 32 | RW | Transfer length |

### Global Registers

| Offset | Register | Width | Mode | Purpose |
|--------|----------|-------|------|---------|
| 0x00 | CTRL | 32 | RW | Global control |
| 0x04 | STATUS | 32 | RO | Status |
| 0x08 | SYNC | 32 | RW | Sync bits |
| 0x20 | CHEN | 32 | RW | Channel enable |
| 0x24 | CHBUSY | 32 | RO | Channel busy |
| 0x28 | CHDONE | 32 | RW | Channel done flags |
| 0x2C | DBGHALT | 32 | RW | Debug halt |
| 0x30 | SWREQ | 32 | WO | Software requests |
| 0x34 | REQDIS | 32 | RW | Request disable |
| 0x38 | REQPEND | 32 | RO | Pending requests |
| 0x3C | LINKLOAD | 32 | WO | Link load |
| 0x40 | REQCLEAR | 32 | WO | Request clear |
| 0xE0 | IF | 32 | RO | Interrupt flags |
| 0xE4 | IFS | 32 | WO | Interrupt set |
| 0xE8 | IFC | 32 | WO | Interrupt clear |
| 0xEC | IEN | 32 | RW | Interrupt enable |

### CH_CTRL Register Bits (Per Channel)

```
[27:16] BLOCKSIZE - Units (count) per arbitration cycle
[13:8]  DSTINC    - Destination increment (0=none, 1=1, 2=2, 3=4)
[7:5]   RESERVED
[4:3]   SRCINC    - Source increment (0=none, 1=1, 2=2, 3=4)
[2:1]   SIZE      - Transfer size (0=byte, 1=halfword, 2=word)
[0]     EN        - Channel enable
```

### CH_CFG Register Bits

```
[7:5]   REQMODE   - Request mode (0=event, 1=block)
[4:0]   REQSEL    - Request signal selection (0-31)
```

### Transfer Count Limits

```
Maximum transfer: 65535 bytes (16-bit counter)
Block size: Arbitrary (but 1, 2, 4, 8, 16, 32, 64, 128 common)
Alignment: 1, 2, 4 bytes per config

For transfers > 65535 bytes: Split into multiple sequential transfers
```

## Peripheral Request Signals

### Request Signal Mapping
```
Signal 0:  I2C0_RXDATA
Signal 1:  I2C0_TXDATA
Signal 2:  I2C1_RXDATA
Signal 3:  I2C1_TXDATA
Signal 4:  UART0_RXDATA
Signal 5:  UART0_TXDATA
... others
```

**REQSEL in CH_CFG must match peripheral FIFO address:**
- I2C0 RX: REQSEL=0, address=0x4000_A010
- I2C0 TX: REQSEL=1, address=0x4000_A00C
- I2C1 RX: REQSEL=2, address=0x4000_A410
- I2C1 TX: REQSEL=3, address=0x4000_A40C
- UART0 RX: REQSEL=4
- UART0 TX: REQSEL=5

## Transfer Sequence

### Memory-to-Memory (copy_addr)
```
1. Set CH_SRC = source address
2. Set CH_DST = destination address
3. Set CH_LEN = transfer count
4. Set CH_CTRL = size, increments, enable
5. LDMA begins transfer automatically
6. Wait: is_transfer_done() returns true
7. Clear: clear_done_flag()
```

### Peripheral-to-Memory (i2c0_rx_addr)
```
1. Set CH_DST = destination address
2. Set CH_SRC = peripheral address (0x4000_A010 for I2C0 RXDATA)
3. Set CH_LEN = expected count
4. Set CH_CFG = REQSEL (0 for I2C0 RX)
5. Set CH_CTRL = size, SRCINC=none, DSTINC=1
6. Enable channel
7. Peripheral issues DMA requests when data available
8. LDMA pulls from peripheral, pushes to memory
9. Wait for completion
```

### Memory-to-Peripheral (i2c0_tx_addr)
```
1. Set CH_SRC = source address
2. Set CH_DST = peripheral address (0x4000_A00C for I2C0 TXDATA)
3. Set CH_LEN = count to transmit
4. Set CH_CFG = REQSEL (1 for I2C0 TX)
5. Set CH_CTRL = size, SRCINC=1, DSTINC=none
6. Enable channel
7. Peripheral requests data via DMA
8. LDMA pulls from memory, pushes to peripheral
9. Wait for completion
```

## Arbitration and Block Size

**Block Size Controls Arbitration Latency:**
```
Block Size = Units per arbitration cycle

Size 1:   Give up bus every 1 unit (lowest latency, lowest throughput)
Size 16:  Give up bus every 16 units (higher latency, higher throughput)
Size 256: Give up bus every 256 units (highest throughput, highest latency)
```

**Trade-off:**
- Small block size: Responsive to other bus masters, low latency
- Large block size: Higher throughput, possible latency for other masters

## Concurrency

### Multi-Channel Independence
```
Channels 0-7 are completely independent:
- Each has own source, destination, length, control
- Each can run simultaneously
- No interference between channels

Example: Channels 0, 1, 2 can transfer in parallel
- Channel 0: I2C0 RX (peripheral-to-memory)
- Channel 1: I2C1 TX (memory-to-peripheral)
- Channel 2: Memory copy (memory-to-memory)
```

### Channel Arbitration
- LDMA arbiter grants bus access in round-robin order
- All enabled channels get equal opportunity
- Block size determines hold time per arbitration

## Safety Guarantees

### Safe API (copy_slice, etc.)
- Automatically validates: no overlap between src/dst
- Uses rust references: guaranteed lifetime validity
- Returns error on validation failure

### Address-based API (copy_addr, etc.)
- **Unchecked**: Caller must ensure:
  - Addresses point to valid memory
  - Src and dst don't overlap (if not intended)
  - Regions are large enough for count
  - Addresses aligned per transfer size

## Constraints

| Parameter | Min | Max | Notes |
|-----------|-----|-----|-------|
| Channels | 0 | 7 | 8 independent channels |
| Count per transfer | 1 | 65,535 | 16-bit limit |
| Block size | 1 | 65,535 | Arbitration units |
| Address alignment | 1 byte | — | No alignment required |
| Source/Dest overlap | No | — | OverlappingAddresses error |

## Performance

### Throughput
```
Maximum: 32 MB/s (bus limited)
Typical: 20-30 MB/s (accounting for arbitration)
Minimum: 1 byte (no minimum transfer)
```

### Latency
- Setup: ~10-20 CPU cycles
- Start-to-first-transfer: < 1 microsecond
- Per-byte: < 100 nanoseconds

### CPU Overhead
- Initial setup: ~100 instructions
- Per transfer: 0 instructions (runs independently)
- Polling: Minimal (just register reads)

## Status Monitoring

### Channel Busy
```rust
pub fn any_busy(&self) -> bool
// Returns true if ANY channel is actively transferring
```

### Transfer Status Per Channel
```rust
pub fn is_transfer_done(&self, channel: usize) -> Result<bool, DmaError>
// Returns Ok(true) if transfer complete, Ok(false) if in progress
```

### All Channels Status
```rust
pub fn channel_status(&self) -> u8
// Returns bitmask: bit N set = channel N is enabled
```

## Interrupt Integration

### Interrupt Flags
- Register: LDMA->IF (read-only)
- Register: LDMA->IFS (write to set)
- Register: LDMA->IFC (write to clear)
- Register: LDMA->IEN (enable mask)

### IRQHandler Name
```rust
#[mg24_hal::interrupt]
fn LDMA() {
    // Called when LDMA issues interrupt
    // Check which channels are done via IF register
    // Clear flags via IFC register
}
```

## Deprecated API

### Backward-Compatibility Aliases
```rust
// These are deprecated, use copy_addr() instead
pub fn copy(&mut self, channel: u8, src: u32, dst: u32, count: u16)
pub fn i2c0_rx(&mut self, channel: u8, dst: u32, count: u16)
pub fn i2c0_tx(&mut self, channel: u8, src: u32, count: u16)
pub fn i2c1_rx(&mut self, channel: u8, dst: u32, count: u16)
pub fn i2c1_tx(&mut self, channel: u8, src: u32, count: u16)
```

All call modern `.ok()` idiom on underlying methods.

## Reference Manual

**EFR32MG24 Reference Manual:**
- Section 24: LDMA
- Section 24.2: Register Descriptions
- Section 24.3: Operational Details
- Section 24.4: Linked Descriptors (if used)

---

**Version**: 2.0.0  
**Last Updated**: 2026-09-18  
**Audience**: AI agents, developers, automated systems
