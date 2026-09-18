# LDMA (Linked DMA) Implementation for EFR32MG24

## Overview

This document describes the DMA module implementation for the EFR32MG24 microcontroller, providing direct access to the LDMA (Linked Direct Memory Access) controller with full support for I2C integration.

## Architecture

The LDMA controller in the EFR32MG24 features:

- **8 Independent Channels** (CH0-CH7) for concurrent DMA operations
- **Multiple Transfer Types**: Memory-to-Memory, Memory-to-Peripheral, Peripheral-to-Memory, Peripheral-to-Peripheral
- **Flexible Configuration**: Programmable data sizes, address increments, and block sizes
- **Request Sources**: Hardware triggers from peripherals (I2C, UART, ADC, etc.) or software triggers
- **Arbitration Modes**: Fixed priority or round-robin scheduling

## Module Structure (src/dma.rs)

### Core Components

#### `DmaRequest` Enum
Defines available DMA request sources, specifically for I2C:
- `I2c0Rx` (SOURCESEL=0x5, SIGSEL=0x0) - I2C0 receive FIFO data valid
- `I2c0Tx` (SOURCESEL=0x5, SIGSEL=0x1) - I2C0 transmit buffer level
- `I2c1Rx` (SOURCESEL=0x6, SIGSEL=0x0) - I2C1 receive FIFO data valid
- `I2c1Tx` (SOURCESEL=0x6, SIGSEL=0x1) - I2C1 transmit buffer level

#### `DmaSize` Enum
Transfer unit sizes:
- `Byte` (0) - 8-bit transfers
- `HalfWord` (1) - 16-bit transfers
- `Word` (2) - 32-bit transfers

#### `DmaIncrement` Enum
Address increment modes per transfer:
- `One` (0) - Increment by 1 unit
- `Two` (1) - Increment by 2 units
- `Four` (2) - Increment by 4 units
- `None` (3) - Fixed address (for FIFOs)

#### `DmaConfig` Builder
Configuration builder following esp-hal patterns:

```rust
let config = DmaConfig::default()
    .with_channel(0)
    .with_request(DmaRequest::I2c0Rx)
    .with_size(DmaSize::Byte)
    .with_src_inc(DmaIncrement::None)
    .with_dst_inc(DmaIncrement::One)
    .with_block_size(1);
```

#### `Dma` Struct
Main controller interface providing:
- `new()` - Enable LDMA module
- `configure_request()` - Connect channel to peripheral trigger
- `configure_transfer()` - Set up source, destination, and count
- `enable_channel()` - Mark channel as active
- `disable_channel()` - Disable channel
- `start_transfer()` - Trigger software-initiated transfer
- `is_transfer_done()` - Poll completion status
- `clear_done_flag()` - Clear interrupt flag after completion
- `any_busy()` - Check if any channel is active
- `channel_status()` - Get status of all 8 channels

## Register Mapping

All register accesses verified against EFR32MG24 Reference Manual Chapter 24.

### Key Registers

| Register | Offset | Purpose | Reference |
|----------|--------|---------|-----------|
| LDMA_EN | 0x004 | Module enable/disable | 24.7.2 |
| LDMA_CTRL | 0x008 | Fixed priority arbitration setup | 24.7.3 |
| LDMA_CHEN | 0x024 | Channel enable register | 24.7.10 |
| LDMA_CHDIS | 0x028 | Channel disable register | 24.7.11 |
| LDMA_CHSTATUS | 0x02C | Channel enabled status | 24.7.12 |
| LDMA_CHBUSY | 0x030 | Channel busy status | 24.7.13 |
| LDMA_CHDONE | 0x034 | Channel done interrupt flag | 24.7.14 |
| LDMA_SWREQ | 0x03C | Software transfer request | 24.7.16 |
| LDMA_STATUS | 0x00C | DMA status (ANYBUSY bit 0) | 24.7.4 |
| LDMA_CHx_CFG | 0x05C + (ch*0x20) | Channel configuration | 24.7.23 |
| LDMA_CHx_CTRL | 0x064 + (ch*0x20) | Descriptor control | 24.7.25 |
| LDMA_CHx_SRC | 0x068 + (ch*0x20) | Source address | 24.7.26 |
| LDMA_CHx_DST | 0x06C + (ch*0x20) | Destination address | 24.7.27 |
| LDMA_CHx_LINK | 0x070 + (ch*0x20) | Link address | 24.7.28 |
| LDMAXBAR_CHx_REQSEL | 0x4001_2004 + (ch*4) | Peripheral request source | 24.9.2 |

## I2C Integration

The DMA module provides seamless integration with I2C transfers:

### I2C DMA Request Sources

From EFR32MG24 Reference Manual Section 21.3.14:

**I2C0:**
- RX Request: SOURCESEL=0x5 (I2C0), SIGSEL=0x0 (I2C0_DMA_RXDATAV)
  - Triggers when I2C0 RXDATA register has data available
- TX Request: SOURCESEL=0x5 (I2C0), SIGSEL=0x1 (I2C0_DMA_TXBL)
  - Triggers when I2C0 transmit buffer is available

**I2C1:**
- RX Request: SOURCESEL=0x6 (I2C1), SIGSEL=0x0 (I2C1_DMA_RXDATAV)
- TX Request: SOURCESEL=0x6 (I2C1), SIGSEL=0x1 (I2C1_DMA_TXBL)

### I2C Peripheral Addresses

- **I2C0 RXDATA**: 0x4000_a00C - 8-bit receive data register
- **I2C0 TXDATA**: 0x4000_a008 - 8-bit transmit data register
- **I2C1 RXDATA**: 0x4000_a400 + 0x00C - 8-bit receive data register
- **I2C1 TXDATA**: 0x4000_a400 + 0x008 - 8-bit transmit data register

## Usage Example

### Basic I2C RX via DMA

```rust
use mg24_hal::{
    CpuConfig, dma::{Dma, DmaConfig, DmaRequest, DmaSize, DmaIncrement}
};

// Initialize with DMA clock enabled
let dp = mg24_hal::init(
    CpuConfig::default()
        .with_i2c_clock(true)
        .with_ldma_clock(true)
)?;

// Create and configure DMA controller
let mut dma = Dma::new();

// Configure DMA channel 0 for I2C0 RX (32-byte transfer)
let config = DmaConfig::default()
    .with_channel(0)
    .with_size(DmaSize::Byte)
    .with_src_inc(DmaIncrement::None)  // I2C FIFO = fixed address
    .with_dst_inc(DmaIncrement::One)   // Increment destination buffer
    .with_block_size(1);

// Set up peripheral request source
dma.configure_request(0, DmaRequest::I2c0Rx);

// Configure transfer (source=I2C0 RXDATA, destination=buffer, count=32)
let mut rx_buffer = [0u8; 32];
dma.configure_transfer(
    0,
    0x4000_a00C,  // I2C0 RXDATA register
    &mut rx_buffer[0] as *mut _ as u32,
    32,
    &config,
);

// Enable and start transfer
dma.enable_channel(0);
dma.start_transfer(0);

// Poll for completion
while !dma.is_transfer_done(0) {
    // Wait...
}

dma.clear_done_flag(0);
```

## Hardware Details

### Clock Configuration

The LDMA controller requires the LDMA clock to be enabled before use:

```rust
// In CpuConfig
let dp = mg24_hal::init(
    CpuConfig::default()
        .with_ldma_clock(true)
)?;
```

This sets the LDMA bit in CMU_CLKEN0 register (Reference Manual 9.5.6).

### Channel Configuration

The LDMA_CHx_CTRL register fields configured by `configure_transfer()`:

- **DSTMODE (bit 31)**: Destination addressing mode (always absolute)
- **SRCMODE (bit 30)**: Source addressing mode (always absolute)
- **DSTINC (bits 29:28)**: Destination increment per transfer
- **SIZE (bits 27:26)**: Unit data transfer size
- **SRCINC (bits 25:24)**: Source increment per transfer
- **IGNORESREQ (bit 23)**: Ignore single requests (disabled)
- **DECLOOPCNT (bit 22)**: Decrement loop counter (disabled)
- **REQMODE (bit 21)**: Request mode - BLOCK mode for single transfers
- **DONEIEN (bit 20)**: Interrupt enable on completion
- **BLOCKSIZE (bits 19:16)**: Units per arbitration cycle
- **BYTESWAP (bit 15)**: Endian byte swap (disabled)
- **XFERCNT (bits 14:4)**: Transfer count (value written is count - 1)
- **STRUCTTYPE (bits 1:0)**: Structure type (TRANSFER = 0)

### Transfer Completion

The DMA hardware sets the corresponding bit in LDMA_CHDONE when a transfer descriptor completes. Software must:

1. Poll or wait for interrupt
2. Clear the CHDONE flag by writing 1 to that bit
3. Configure next transfer if chaining operations

## Design Decisions

### Register Access Method

Uses direct pointer arithmetic for register access rather than relying on PAC functions, allowing:

1. **Full Control**: Direct verification against reference manual
2. **Minimal Dependencies**: No additional PAC abstractions needed
3. **Performance**: No abstraction overhead
4. **Transparency**: Register writes match reference manual exactly

Example for LDMA_CHx_CTRL at offset 0x64 for channel N:
```rust
let ctrl_ptr = (0x4001_2000 + 0x64 + (channel as usize * 0x20)) as *mut u32;
core::ptr::write_volatile(ctrl_ptr, ctrl_value);
```

### Builder Pattern

`DmaConfig` follows esp-hal conventions for intuitive API:

```rust
config
    .with_channel(0)
    .with_request(DmaRequest::I2c0Rx)
    .with_size(DmaSize::Byte)
```

### Block Size Encoding

The `block_size_bits()` helper maps user-friendly block sizes to hardware encodings:

| Size | Encoding | Units per Arbitration |
|------|----------|----------------------|
| 1    | 0x0      | 1 |
| 2    | 0x1      | 2 |
| 3    | 0x2      | 3 |
| 4    | 0x3      | 4 |
| 6    | 0x4      | 6 |
| 8    | 0x5      | 8 |
| 16   | 0x7      | 16 |
| 32   | 0x9      | 32 |
| 64   | 0xA      | 64 |
| 128  | 0xB      | 128 |
| 256  | 0xC      | 256 |
| 512  | 0xD      | 512 |
| 1024 | 0xE      | 1024 |

Reference: EFR32MG24 RM 24.7.25 BLOCKSIZE field

## Example Programs

### i2c_dma.rs

Demonstrates:
- LDMA module initialization
- Channel configuration for I2C0 RX
- Transfer setup with 32-byte buffer
- Status polling and completion detection
- LED feedback for status indication

Run with:
```bash
cargo build --example i2c_dma
probe-rs run --chip EFR32MG24B220F1536IM48 target/.../examples/i2c_dma
```

## Testing

The DMA module has been verified to:

1. **Initialize correctly** - LDMA module enables via EN register
2. **Configure channels** - Request sources set properly in LDMAXBAR_CHx_REQSEL
3. **Set up transfers** - Source/destination/count registers written correctly
4. **Poll status** - Done flags and busy bits read accurately
5. **Compile with examples** - No linker or runtime errors

## Future Enhancements

Potential additions:

1. **Linked Descriptors** - Support descriptor chaining for complex transfers
2. **Interrupt Handlers** - LDMA interrupt integration
3. **Error Handling** - Detection and handling of AHB errors
4. **Scatter-Gather** - Support for multiple non-contiguous buffers
5. **Loop Transfers** - Repeating transfer sequences with LOOPCNT

## References

- EFR32MG24 Reference Manual, Chapter 24: LDMA - Linked DMA
- Section 21.3.14: I2C DMA Support
- Section 9.5.6: CMU Clock Enable Register

## Register Verification Checklist

All registers and fields verified against EFR32MG24 Reference Manual Rev. 0.5:

- [x] LDMA_EN (24.7.2) - Module enable
- [x] LDMA_CTRL (24.7.3) - Fixed priority channels
- [x] LDMA_CHEN (24.7.10) - Channel enable
- [x] LDMA_CHDIS (24.7.11) - Channel disable
- [x] LDMA_CHSTATUS (24.7.12) - Channel status
- [x] LDMA_CHBUSY (24.7.13) - Busy status
- [x] LDMA_CHDONE (24.7.14) - Done flag
- [x] LDMA_SWREQ (24.7.16) - Software request
- [x] LDMA_STATUS (24.7.4) - Overall status
- [x] LDMA_CHx_CFG (24.7.23) - Channel config
- [x] LDMA_CHx_CTRL (24.7.25) - Descriptor control
- [x] LDMA_CHx_SRC (24.7.26) - Source address
- [x] LDMA_CHx_DST (24.7.27) - Destination address
- [x] LDMA_CHx_LINK (24.7.28) - Link address
- [x] LDMAXBAR_CHx_REQSEL (24.9.2) - Request selection
- [x] I2C Request Sources (21.3.14) - I2C0/I2C1 RX/TX
