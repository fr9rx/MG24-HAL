# I2C API Reference

Complete documentation of the I2C (Inter-Integrated Circuit) module for both I2C0 and I2C1 peripherals.

## Module Overview

The I2C module provides a complete interface to the EFR32MG24's I2C peripherals with comprehensive error handling and address validation.

**Location**: `src/i2c.rs`  
**Types**: `I2c0<'d>`, `I2c1<'d>`, `I2cError`, `I2cConfig`

## I2C Initialization

### Creating an I2C Peripheral

```rust
use mg24_hal::i2c::{I2c0, I2cConfig};

let mut i2c = I2c0::new(
    dp.i2c0,
    dp.pins.pc0,  // SDA
    dp.pins.pc1,  // SCL
    I2cConfig::default()
        .with_speed(400_000)  // 400 kHz
);
```

### I2cConfig Builder Methods

| Method | Type | Default | Purpose |
|--------|------|---------|---------|
| `with_speed(u32)` | Frequency Hz | 100_000 | SCL clock frequency |
| `with_timeout(u32)` | Ticks | 0 | Timeout duration |

### Supported Speeds

Standard I2C frequencies:
- **100 kHz** (Standard Mode) - Default for compatibility
- **400 kHz** (Fast Mode) - Common industrial standard
- **1 MHz** (Fast Mode Plus) - High-speed transfers
- **Custom**: Any frequency supported by hardware clock tree

```rust
// Standard mode (100 kHz)
let i2c = I2c0::new(dp.i2c0, sda, scl, I2cConfig::default());

// Fast mode (400 kHz)
let i2c = I2c0::new(dp.i2c0, sda, scl, I2cConfig::default().with_speed(400_000));

// Custom speed
let i2c = I2c0::new(dp.i2c0, sda, scl, I2cConfig::default().with_speed(1_000_000));
```

## I2C Operations

### Reading Data (Receive)

```rust
pub fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), I2cError> {
    // Validate 7-bit address range (0x00-0x7F)
    // Read bytes into buffer, advancing SCL/SDA
    // Return error if bus problem detected
}
```

**Parameters**:
- `addr`: 7-bit I2C slave address (0x00-0x7F)
- `buffer`: Mutable byte slice to fill

**Returns**:
- `Ok(())` - All bytes received successfully
- `Err(I2cError)` - See error section below

**Example**:
```rust
let mut buffer = [0u8; 4];
i2c.read(0x68, &mut buffer)?;  // Read 4 bytes from address 0x68
```

### Writing Data (Transmit)

```rust
pub fn write(&mut self, addr: u8, data: &[u8]) -> Result<(), I2cError> {
    // Validate 7-bit address range (0x00-0x7F)
    // Write bytes from data, controlling SCL/SDA
    // Return error if bus problem detected
}
```

**Parameters**:
- `addr`: 7-bit I2C slave address (0x00-0x7F)
- `data`: Byte slice to transmit

**Returns**:
- `Ok(())` - All bytes transmitted successfully
- `Err(I2cError)` - See error section below

**Example**:
```rust
i2c.write(0x3C, &[0x40, 0x00])?;  // Write command to OLED
```

### Combined Write-Read (Repeated Start)

```rust
pub fn write_read(
    &mut self,
    addr: u8,
    write: &[u8],
    read: &mut [u8]
) -> Result<(), I2cError> {
    // Send START condition
    // Write data
    // Send REPEATED START (not STOP then START)
    // Read data
    // Send STOP
}
```

**Parameters**:
- `addr`: 7-bit I2C slave address (0x00-0x7F)
- `write`: Bytes to transmit first
- `read`: Mutable buffer for reply bytes

**Returns**:
- `Ok(())` - Operation completed successfully
- `Err(I2cError)` - See error section below

**Atomicity**: Uses I2C repeated START condition, preventing other masters from interleaving commands.

**Example**:
```rust
// Common pattern: write register address, read register value
let reg_addr = [0x3D];
let mut result = [0u8; 1];
i2c.write_read(0x68, &reg_addr, &mut result)?;
```

## Error Handling

### I2cError Enum

```rust
pub enum I2cError {
    InvalidAddress,      // 8-bit address invalid, must be 7-bit
    ArbitrationLost,     // Another master drove bus low
    NoAck,              // Slave didn't acknowledge address/data
    BusError,           // SDA/SCL state invalid
    BusHeld,            // Another master holds bus
    Timeout,            // Operation exceeded timeout
}
```

### Address Validation

7-bit addresses must be in range **0x00-0x7F**. The MSB (bit 7) is reserved for direction.

```rust
// ✅ VALID 7-bit addresses
i2c.read(0x3C, &mut buffer)?;   // 0x3C = 0011_1100
i2c.read(0x68, &mut buffer)?;   // 0x68 = 0110_1000

// ❌ INVALID 8-bit addresses
i2c.read(0x78, &mut buffer)?;   // ERROR: InvalidAddress (0x78 = 0111_1000, bit 7 set)
```

### Error Recovery

Most errors are unrecoverable and require bus reset or peripheral reinit:

```rust
match i2c.read(addr, &mut buffer) {
    Ok(()) => { /* success */ }
    Err(I2cError::InvalidAddress) => {
        // Address validation failed - check address is 7-bit
        eprintln!("Invalid I2C address: 0x{:02X}", addr);
    }
    Err(I2cError::NoAck) => {
        // Device not responding - check address, wiring
        eprintln!("Device at 0x{:02X} not responding", addr);
    }
    Err(I2cError::BusError) => {
        // Critical bus problem - may need reset
        eprintln!("I2C bus error - check SDA/SCL lines");
    }
    Err(e) => {
        eprintln!("I2C error: {:?}", e);
    }
}
```

## Embedded-HAL 1.0 Traits

### I2C Trait Implementation

```rust
use embedded_hal::i2c::I2c;

// All mg24-hal I2C peripherals implement embedded_hal::i2c::I2c
impl I2c for I2c0<'_> { }
impl I2c for I2c1<'_> { }
```

### ErrorType Trait

```rust
use embedded_hal::i2c::ErrorType;

impl ErrorType for I2c0<'_> {
    type Error = I2cError;
}

impl ErrorType for I2c1<'_> {
    type Error = I2cError;
}
```

## Common Patterns

### Device Discovery (Bus Scan)

```rust
pub fn i2c_scan(i2c: &mut I2c0) {
    rprintln!("Scanning I2C bus...");
    for addr in 0x00..=0x7F {
        let mut buffer = [0u8; 1];
        match i2c.read(addr, &mut buffer) {
            Ok(()) => rprintln!("  0x{:02X}: Device found", addr),
            Err(I2cError::NoAck) => { /* no device */ }
            Err(e) => rprintln!("  0x{:02X}: Error {:?}", addr, e),
        }
    }
}
```

### Sensor Register Reading

```rust
// Pattern: write register address, read result
pub fn read_sensor_reg(
    i2c: &mut I2c0,
    addr: u8,
    reg: u8
) -> Result<u8, I2cError> {
    let mut value = [0u8; 1];
    i2c.write_read(addr, &[reg], &mut value)?;
    Ok(value[0])
}

// Usage
let temperature = read_sensor_reg(&mut i2c, 0x68, 0x41)?;
```

### Multi-Byte Read

```rust
let mut buffer = [0u8; 6];  // Read 6 bytes
i2c.read(0x69, &mut buffer)?;
let accel_x = u16::from_le_bytes([buffer[0], buffer[1]]);
let accel_y = u16::from_le_bytes([buffer[2], buffer[3]]);
let accel_z = u16::from_le_bytes([buffer[4], buffer[5]]);
```

### Multi-Step Configuration

```rust
// Configure device with multiple register writes
i2c.write(0x3C, &[0x80, 0xAD])?;  // Set config register 1
i2c.write(0x3C, &[0x81, 0xD5])?;  // Set config register 2
i2c.write(0x3C, &[0x82, 0x00])?;  // Set control register

// Verify configuration
let mut status = [0u8; 1];
i2c.read(0x3C, &mut status)?;
rprintln!("Status: 0x{:02X}", status[0]);
```

## Hardware Constraints

### Bus Timing

The I2C clock (SCL) frequencies are dependent on the peripheral clock:

```
SCL Frequency = HFPERCLK / (DIV + 4)

Where DIV is programmable in CLKDIV register
```

### Pin Requirements

- **SDA (Serial Data)**: Must support open-drain output mode
- **SCL (Serial Clock)**: Must support open-drain output mode
- **Pull-ups**: 4.7kΩ pull-ups required on both SDA and SCL
- **Maximum Cable**: 400pF capacitance

### Electrical Characteristics

| Parameter | Min | Max | Unit |
|-----------|-----|-----|------|
| SDA/SCL High Voltage | 2.7 | 3.6 | V |
| SDA/SCL Low Voltage | 0.0 | 0.4 | V |
| Bus Capacitance | - | 400 | pF |

## Hardware Details (EFR32MG24)

- **I2C0 Base Address**: 0x4000_A000
- **I2C1 Base Address**: 0x4000_A400
- **Reference**: EFR32MG24 Reference Manual, Section 22

### Key Register Offsets

| Register | Offset | Purpose |
|----------|--------|---------|
| CTRL | 0x00 | Control register |
| CLKDIV | 0x04 | Clock division |
| STAT | 0x08 | Status flags |
| TXDATA | 0x0C | Transmit data |
| RXDATA | 0x10 | Receive data |

### STAT Register Bits

```
Bit 0: BUSY    - Bus in use
Bit 1: NACK    - No acknowledge received
Bit 2: ACK     - Acknowledge received
Bit 3: ARBLST  - Arbitration lost
Bit 4: TXC     - Transmit complete
Bit 5: RXDATAV - Receive data valid
```

## Performance Notes

- **Blocking Operations**: All operations block until completion or timeout
- **No Interrupts**: Current implementation is polling-based
- **DMA Support**: I2C can be used with LDMA for high-speed transfers
- **Buffer Size**: Maximum single transfer typically 256 bytes (hardware dependent)

## Debugging Tips

### Common Issues

| Symptom | Cause | Fix |
|---------|-------|-----|
| `InvalidAddress` error | Address > 0x7F | Use 7-bit address (0x00-0x7F) |
| `NoAck` error | Device not responding | Check address, wiring, power |
| `BusError` | SDA/SCL stuck | Check pull-ups, reset device |
| Slow transfers | SCL frequency too low | Increase speed in config |

### Verification

```rust
// Verify peripheral initialized
let i2c = I2c0::new(dp.i2c0, sda, scl, I2cConfig::default());

// Test single device
let mut buffer = [0u8; 1];
match i2c.read(0x68, &mut buffer) {
    Ok(()) => rprintln!("I2C working"),
    Err(e) => rprintln!("I2C error: {:?}", e),
}
```

---

**Version**: 2.0.0  
**Last Updated**: 2026-09-18
