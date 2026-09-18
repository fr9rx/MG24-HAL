# I2C API - Agent/Developer Specification

Complete technical specification for I2C module. Authoritative reference for agents implementing I2C features.

## Module Location
`src/i2c.rs`

## Type Signatures

### I2c0<'d> and I2c1<'d>
I2C peripheral wrappers for two independent controllers.

```rust
pub struct I2c0<'d> {
    _marker: PhantomData<&'d ()>,
}

pub struct I2c1<'d> {
    _marker: PhantomData<&'d ()>,
}

impl<'d> I2c0<'d> {
    pub fn new(
        _i2c: I2C0,
        sda: impl Into<AnyPin>,
        scl: impl Into<AnyPin>,
        config: I2cConfig,
    ) -> Self
    
    pub fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), I2cError>
    pub fn write(&mut self, addr: u8, data: &[u8]) -> Result<(), I2cError>
    pub fn write_read(
        &mut self,
        addr: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), I2cError>
}

// I2c1 has identical interface
```

## Configuration

### I2cConfig
```rust
pub struct I2cConfig {
    speed: u32,      // Hz
    timeout: u32,    // Ticks
}

impl I2cConfig {
    pub fn default() -> Self  // 100_000 Hz, 0 timeout
    pub fn with_speed(self, speed: u32) -> Self
    pub fn with_timeout(self, timeout: u32) -> Self
}
```

**Standard Speeds:**
```
100_000  Hz (Standard Mode, I2C)
400_000  Hz (Fast Mode, Fm)
1_000_000 Hz (Fast Mode Plus, Fm+)
```

## Error Types

### I2cError Enum
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I2cError {
    /// Address is not valid 7-bit (MSB set)
    InvalidAddress,
    
    /// Arbitration lost: another master drove bus low
    ArbitrationLost,
    
    /// Slave did not acknowledge (NACK)
    NoAck,
    
    /// Invalid bus state (SDA/SCL not as expected)
    BusError,
    
    /// Bus held by another master (can't acquire)
    BusHeld,
    
    /// Operation timeout
    Timeout,
}
```

**Error Conditions:**

| Error | Cause | Recovery |
|-------|-------|----------|
| InvalidAddress | addr > 0x7F | Use 7-bit address (0x00-0x7F only) |
| NoAck | Slave not responding | Check: power, address, wiring |
| ArbitrationLost | Multi-master conflict | Release bus, retry |
| BusError | SDA/SCL line issue | Check pull-ups (4.7kΩ typical) |
| BusHeld | Another master active | Wait, then retry |
| Timeout | Operation exceeded limit | Check clock, device responsiveness |

## Address Validation

### 7-bit Addressing
- **Valid range**: 0x00 - 0x7F (7 bits)
- **MSB (bit 7)**: Reserved for direction (0=write, 1=read)
- **Rejection**: Any address with bit 7 set returns `InvalidAddress`

```rust
// Examples
0x3C  // Valid: 0011_1100
0x68  // Valid: 0110_1000
0x78  // INVALID: 0111_1000 (bit 7 set)
0xFF  // INVALID: 1111_1111 (multiple bits set)
```

## Hardware Registers

### I2C0 Base: 0x4000_A000
### I2C1 Base: 0x4000_A400

| Offset | Register | Width | Mode | Purpose |
|--------|----------|-------|------|---------|
| 0x00 | CTRL | 32 | RW | Control register |
| 0x04 | CLKDIV | 32 | RW | Clock divider |
| 0x08 | STAT | 32 | RO | Status flags |
| 0x0C | TXDATA | 32 | WO | Transmit data |
| 0x10 | RXDATA | 32 | RO | Receive data |
| 0x14 | TXDATAE | 32 | RW | TX data expand |
| 0x18 | RXDATAE | 32 | RO | RX data expand |
| 0x1C | ACT | 32 | RW | Active register |

### STAT Register Bits

```
Bit 0: BUSY     - I2C bus busy (START issued, STOP not issued)
Bit 1: NACK     - No acknowledge received (set by HW)
Bit 2: ACK      - Acknowledge received (set by HW)
Bit 3: ARBLST   - Arbitration lost (set by HW)
Bit 4: TXC      - Transmit complete (set by HW)
Bit 5: RXDATAV  - Receive data valid (set by HW)
Bit 6: TXBL     - Transmit buffer level (1=empty)
Bit 7: RXFL     - Receive full (1=data available)
```

### CTRL Register Key Bits

```
Bit 0: EN       - Enable I2C (0=disabled, 1=enabled)
Bit 1: ABORT    - Abort I2C operation
Bit 2: ACKDIS   - Disable automatic ACK (0=auto ACK)
Bit 3: CLKSEN   - Clock sense enable (for FM+ mode)
Bit 4: AUTOSN   - Auto-send NACK
Bit 5: AUTOSE   - Auto-send END
Bit 6: AUTOSTP  - Auto-send STOP
```

## Transfer Sequences

### Write Operation (Master TX)
```
1. START condition
2. Send address + Write bit (0)
3. Wait for ACK/NACK
4. Transmit each data byte
5. Wait ACK after each byte
6. STOP condition
```

### Read Operation (Master RX)
```
1. START condition
2. Send address + Read bit (1)
3. Wait for ACK/NACK
4. Receive each data byte
5. Send ACK after each byte (except last)
6. Send NACK after last byte
7. STOP condition
```

### Write-Read (Repeated START)
```
1. START condition
2. Send address + Write bit (0)
3. Wait for ACK
4. Transmit write data
5. Wait ACK after each byte
6. REPEATED START (no STOP)
7. Send address + Read bit (1)
8. Wait for ACK
9. Receive read data
10. STOP condition
```

## Timing Characteristics

### Clock Divider Formula
```
SCL_freq = HFPERCLK / (CLKDIV + 4)

Where:
- HFPERCLK = Peripheral clock (39 MHz typical for EFR32MG24)
- CLKDIV = Value in CLKDIV register
- +4 = Minimum divider constant
```

### Typical Divider Values
```
For 39 MHz HFPERCLK:
100 kHz:  CLKDIV = 385  (39M/(389+4) = 100k)
400 kHz:  CLKDIV = 92   (39M/(96+4) = 400k)
1 MHz:    CLKDIV = 35   (39M/(39+4) = 976k)
```

### Bus Hold Time
- Minimum SDA low: 4 SCL periods
- Minimum SCL low: 4 SCL periods
- Setup time (SDA before SCL): 0.1-0.25 µs

## Pin Requirements

### Pin Modes
- SDA: Must support open-drain output mode
- SCL: Must support open-drain output mode

### Electrical Characteristics

| Parameter | Min | Typ | Max | Unit |
|-----------|-----|-----|-----|------|
| Vdd | 2.7 | 3.3 | 3.6 | V |
| VinHigh | 0.7*Vdd | — | Vdd | V |
| VinLow | 0 | — | 0.3*Vdd | V |
| Pull-up resistance | — | 4.7 | 10 | kΩ |
| Bus capacitance | — | — | 400 | pF |
| Max frequency | — | 100k/400k/1M | — | Hz |

### Pull-up Calculation
```
R_pullup = (Vdd - V_low) / I_max

For 3.3V bus, 3mA max sink:
R = (3.3 - 0.4) / 0.003 = 967Ω minimum
Typical: 4.7kΩ (pulls to 3.3V)
```

## Embedded-HAL 1.0 Traits

### I2c Trait
```rust
pub trait I2c {
    type Error;
    
    fn write(&mut self, addr: u8, write: &[u8]) -> Result<(), Self::Error>;
    fn read(&mut self, addr: u8, read: &mut [u8]) -> Result<(), Self::Error>;
    fn write_read(
        &mut self,
        addr: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error>;
}

// mg24-hal I2c0/I2c1 implement this trait
```

### ErrorType Trait
```rust
pub trait ErrorType {
    type Error;
}

impl ErrorType for I2c0<'_> {
    type Error = I2cError;
}

impl ErrorType for I2c1<'_> {
    type Error = I2cError;
}
```

## State Machine

### Write/Read State Machine
```
IDLE → (START) → ADDR_TX → (ACK?) → 
  → DATA_TX → (ACK?) → ... → (STOP) → IDLE

IDLE → (START) → ADDR_RX → (ACK?) → 
  → DATA_RX → (ACK) → ... → (NACK) → (STOP) → IDLE
```

**State Transitions:**
- START: Set CTRL.START, master acquires bus
- ADDR: Shift 8-bit address+direction into TXDATA
- ACK/NACK: Read STAT.ACK/NACK bits
- DATA: Read/write RXDATA/TXDATA registers
- STOP: Set CTRL.STOP, release bus

## Initialization Sequence

```rust
1. Configure peripheral clock (I2C0/I2C1 in CMU)
2. Configure pins (SDA/SCL as open-drain)
3. Set CLKDIV for desired SCL frequency
4. Enable I2C: CTRL.EN = 1
5. Ready for operations
```

## Constraints and Limitations

1. **7-bit Addressing Only**: 10-bit addressing not supported
2. **Master Mode Only**: Slave mode not implemented
3. **Blocking Operations**: No DMA/interrupt transfer support (but I2C can work with LDMA)
4. **Single Address**: Cannot listen on multiple addresses (master only)
5. **No Clock Stretching**: Assumes slave responds quickly
6. **Buffer Sizes**: Limited by RXDATAE register (typically 256 bytes max per operation)

## Error Recovery

### NoAck Error
- Slave did not acknowledge address or data
- Possible causes: Wrong address, slave powered off, communication issue
- Recovery: Check address, verify slave is responding

### BusError
- SDA/SCL in unexpected state
- Possible causes: Line stuck low, external driver, noise
- Recovery: May require power cycle or bus reset

### ArbitrationLost
- Multi-master scenario (unexpected in typical use)
- Recovery: Release ownership, wait, retry

## Performance Notes

### Throughput
```
100 kHz: 12.5 kB/s max (including overhead)
400 kHz: 50 kB/s max
1 MHz: 125 kB/s max
```

### Latency
- START to first bit: ~4 SCL periods
- Byte transfer: 9 SCL periods (8 data + 1 ACK)
- Repeated START: ~3 SCL periods

## Reference Manual

**EFR32MG24 Reference Manual:**
- Section 22: I2C
- Section 22.3: Register Descriptions
- Section 22.4: Functional Description
- Electrical characteristics in Section 3

---

**Version**: 2.0.0  
**Last Updated**: 2026-09-18  
**Audience**: AI agents, developers, automated systems
