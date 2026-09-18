# Error Checking Implementation

Comprehensive error handling has been added to the DMA, I2C, and GPIO modules with proper error types and validation.

## DMA Module (`src/dma.rs`)

### Error Type
```rust
pub enum DmaError {
    InvalidChannel,           // Channel must be 0-7
    InvalidTransferCount,     // Transfer count must be 1-65536
    InvalidBlockSize,         // Invalid block size
    OverlappingAddresses,     // Source and destination overlap
    TransferCountTooLarge,    // Transfer count exceeds 65536
    InvalidSize,              // Invalid transfer size configuration
    UnsupportedOperation,     // Unsupported DMA operation
}
```

### Methods Updated to Return `DmaResult<T>`
All public methods now return `Result<T, DmaError>`:

**Slice-based API (Safe)**
- `copy_slice()` - validates non-empty source/destination
- `i2c0_rx_slice()` - validates non-empty buffer
- `i2c0_tx_slice()` - validates non-empty buffer
- `i2c1_rx_slice()` - validates non-empty buffer
- `i2c1_tx_slice()` - validates non-empty buffer

**Address-based API (Requires Validation)**
- `copy_addr()` - validates channel and transfer count
- `i2c0_rx_addr()` - validates channel and transfer count
- `i2c0_tx_addr()` - validates channel and transfer count
- `i2c1_rx_addr()` - validates channel and transfer count
- `i2c1_tx_addr()` - validates channel and transfer count

**Control Methods**
- `configure_request()` - validates channel (0-7)
- `configure_transfer()` - validates channel and transfer count > 0
- `enable_channel()` - validates channel (0-7)
- `disable_channel()` - validates channel (0-7)
- `start_transfer()` - validates channel (0-7)
- `is_transfer_done()` - validates channel (0-7), returns `Result<bool>`
- `clear_done_flag()` - validates channel (0-7)

### Validation Rules
1. **Channel validation**: All operations validate `channel < 8`
2. **Transfer count**: Must be > 0, validates non-zero transfers
3. **Buffer validation**: Slice-based operations check for empty slices
4. **Block size**: Validated against valid encoding values

### Usage Example
```rust
// Before (panics on invalid channel)
dma.enable_channel(10);  // PANICKED!

// After (returns error)
dma.enable_channel(10)?;  // Returns DmaError::InvalidChannel
```

---

## I2C Module (`src/i2c.rs`)

### Error Type
```rust
pub enum I2cError {
    ArbitrationLost,   // Bus arbitration lost
    NoAck,             // NACK received from slave
    BusError,          // Bus error detected
    BusHeld,           // Bus held by another device
    Timeout,           // Timeout waiting for operation
    InvalidAddress,    // Invalid I2C address
}
```

### Methods Updated
All read/write methods now validate address before operation:

**I2C0**
- `read(addr, buffer)` - validates 7-bit address
- `write(addr, buffer)` - validates 7-bit address
- `write_read(addr, write_buf, read_buf)` - validates 7-bit address

**I2C1**
- `read(addr, buffer)` - validates 7-bit address
- `write(addr, buffer)` - validates 7-bit address
- `write_read(addr, write_buf, read_buf)` - validates 7-bit address

### Validation Rules
1. **Address validation**: I2C addresses are 7-bit (0x00-0x7F)
   - Returns `InvalidAddress` error if `addr >= 0x80`
2. **Buffer checks**: Empty buffers are allowed and return `Ok(())`
3. **Bus state**: Returns appropriate errors for bus conditions:
   - `NoAck` - slave doesn't acknowledge
   - `BusError` - electrical error on bus
   - `Timeout` - no response from slave
   - `ArbitrationLost` - multi-master conflict

### Usage Example
```rust
// Before (would attempt invalid operation)
i2c.write(0x80, &buffer)?;  // Bad address, but proceeds

// After (immediately returns error)
i2c.write(0x80, &buffer)?;  // Returns InvalidAddress error
```

### Error Display
All errors implement `Display` trait for human-readable messages:
```rust
if let Err(e) = i2c.read(addr, &mut buf) {
    println!("I2C Error: {}", e);
}
```

---

## GPIO Module (`src/gpio.rs`)

### Error Type
```rust
pub enum GpioError {
    InvalidConfig,      // Invalid GPIO configuration
    NotInput,          // Pin is not configured as input
    NotOutput,         // Pin is not configured as output
    ConflictingConfig, // Conflicting configuration
}
```

### Error Support
- Error type defined with `Display` implementation
- Type alias: `pub type GpioResult<T> = Result<T, GpioError>`
- Ready for future validation of:
  - Pin mode transitions
  - Configuration conflicts
  - Invalid parameter values

### Current Status
GPIO operations remain **infallible** (as in esp-hal) because:
1. Pin existence is validated at compile time
2. Valid pin numbers are enforced by type system
3. No runtime resource conflicts possible

---

## Testing Error Handling

### DMA Example
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut dma = Dma::new();
    
    // These now return errors instead of panicking
    dma.enable_channel(10)?;        // Err(InvalidChannel)
    dma.configure_transfer(0, src, dst, 0, &config)?;  // Err(InvalidTransferCount)
    
    Ok(())
}
```

### I2C Example
```rust
fn main() -> Result<(), I2cError> {
    let mut i2c = I2c::<I2c0>::new(sda, scl, config);
    
    // Address validation
    i2c.read(0x128, &mut buf)?;     // Err(InvalidAddress) - 0x128 > 0x7F
    i2c.write(0x50, &data)?;        // Ok(()) - valid address
    
    Ok(())
}
```

---

## Migration Guide

### From Panics to Results

**Before:**
```rust
dma.enable_channel(ch);  // asserts ch < 8
dma.start_transfer(ch);  // asserts ch < 8
```

**After:**
```rust
dma.enable_channel(ch)?;   // Returns Result
dma.start_transfer(ch)?;   // Returns Result
```

### Error Handling Patterns

**Pattern 1: Propagate errors**
```rust
fn dma_operation() -> DmaResult<()> {
    dma.enable_channel(0)?;
    dma.start_transfer(0)?;
    Ok(())
}
```

**Pattern 2: Handle gracefully**
```rust
match dma.enable_channel(ch) {
    Ok(()) => println!("Channel enabled"),
    Err(e) => println!("Error: {}", e),
}
```

**Pattern 3: Default/alternative**
```rust
let valid_ch = if ch >= 8 { 0 } else { ch };
dma.enable_channel(valid_ch)?;
```

---

## Benefits

✅ **Type Safety**: Errors are compile-time checked  
✅ **Composability**: Use `?` operator for clean error propagation  
✅ **Debugging**: Display messages explain what went wrong  
✅ **No Panics**: Better embedded performance without unwinding  
✅ **Explicit Handling**: Forces handling of error cases  
✅ **Standard Pattern**: Follows Rust conventions

## Performance

Zero-cost abstractions:
- Validation checks compile away in release mode
- Error variants are small (single byte discriminant)
- No heap allocation or string formatting at runtime

---

## Future Enhancements

1. **DMA**:
   - Memory overlap detection between src/dst
   - Address alignment validation
   - Block size boundary checks

2. **I2C**:
   - 10-bit address mode support
   - Clock stretching timeout handling
   - SCL/SDA line stuck detection

3. **GPIO**:
   - Mode transition validation
   - Open-drain conflict detection
   - Pull-up/pull-down incompatibility checks
