# GPIO API - Agent/Developer Specification

Complete technical specification for GPIO module. This is the authoritative reference for agents implementing GPIO features.

## Module Location
`src/gpio.rs`

## Type Signatures

### GpioPin<PORT, PIN>
Base GPIO pin type with phantom port and pin numbers.

```rust
pub struct GpioPin<PORT: GpioPort, PIN: GpioPin> {
    // Phantom type parameters encode port/pin at compile time
    _port: PhantomData<PORT>,
    _pin: PhantomData<PIN>,
}
```

### Input<'d>
Input pin mode.

```rust
pub struct Input<'d> {
    pin: AnyPin,
    _marker: PhantomData<&'d ()>,
}

impl<'d> Input<'d> {
    pub fn new(pin: impl Into<AnyPin>, config: InputConfig) -> Self
    pub fn is_high(&self) -> bool
    pub fn is_low(&self) -> bool
    pub fn is_set(&self) -> bool  // Alias for is_high
    pub fn level(&self) -> Level
    pub fn listen(&mut self, event: Event)
    pub fn clear_interrupt(&mut self)
}
```

### Output<'d>
Output pin mode.

```rust
pub struct Output<'d> {
    pin: AnyPin,
    level: Level,
    _marker: PhantomData<&'d ()>,
}

impl<'d> Output<'d> {
    pub fn new(pin: impl Into<AnyPin>, level: Level, config: OutputConfig) -> Self
    pub fn set_high(&mut self)
    pub fn set_low(&mut self)
    pub fn set_level(&mut self, level: Level)
    pub fn is_set(&self) -> bool
    pub fn toggle(&mut self)
    pub fn level(&self) -> Level
}
```

### Flex<'d>
Flexible pin mode allowing runtime direction changes.

```rust
pub struct Flex<'d> {
    pin: AnyPin,
    _marker: PhantomData<&'d ()>,
}

impl<'d> Flex<'d> {
    pub fn new(pin: impl Into<AnyPin>) -> Self
    pub fn apply_input_config(&mut self, config: &InputConfig)
    pub fn apply_output_config(&mut self, config: &OutputConfig)
    pub fn set_input_enable(&mut self, enable: bool)
    pub fn set_output_enable(&mut self, enable: bool)
    pub fn is_high(&self) -> bool
    pub fn is_low(&self) -> bool
    pub fn set_high(&mut self)
    pub fn set_low(&mut self)
    pub fn level(&self) -> Level
}
```

## Configuration Types

### InputConfig
```rust
pub struct InputConfig {
    pull: Pull,
    filter: bool,
}

impl InputConfig {
    pub fn default() -> Self  // pull=None, filter=false
    pub fn with_pull(self, pull: Pull) -> Self
    pub fn with_filter(self, filter: bool) -> Self
}
```

**Pull enum:**
```rust
pub enum Pull {
    None,   // No pull-up/down
    Up,     // Internal pull-up
    Down,   // Internal pull-down
}
```

### OutputConfig
```rust
pub struct OutputConfig {
    drive_mode: DriveMode,
    pull: Pull,
    slew_rate: SlewRate,
}

impl OutputConfig {
    pub fn default() -> Self  // PushPull, None pull, default slew
    pub fn with_drive_mode(self, mode: DriveMode) -> Self
    pub fn with_pull(self, pull: Pull) -> Self
    pub fn with_slew_rate(self, rate: SlewRate) -> Self
}
```

**DriveMode enum:**
```rust
pub enum DriveMode {
    PushPull,    // Normal CMOS output
    OpenDrain,   // Can only pull low
    OpenSource,  // Can only pull high
}
```

**SlewRate type:**
```rust
pub type SlewRate = u8;  // 0-7, higher = faster
```

### Level
```rust
pub enum Level {
    High,
    Low,
}

impl From<bool> for Level {
    fn from(high: bool) -> Self
}

impl Into<bool> for Level {
    fn into(self) -> bool
}
```

### Event
```rust
pub enum Event {
    FallingEdge,
    RisingEdge,
    AnyEdge,
}
```

## Pin Port-Pin Mapping

| Port | Base Register | Pins | Notes |
|------|---------------|------|-------|
| PA | 0x4004_C000 | 0-9 | 10 pins |
| PB | 0x4004_C030 | 0-4 | 5 pins |
| PC | 0x4004_C060 | 0-5 | 6 pins |
| PD | 0x4004_C090 | 0-3 | 4 pins |
| PE | 0x4004_C0C0 | 0-2 | 3 pins |
| PF | 0x4004_C0F0 | 0-1 | 2 pins |
| PG | 0x4004_C120 | 0 | 1 pin |
| PH | 0x4004_C150 | 0-1 | 2 pins |

**Debug Pins (unsafe, debugger only):**
- PA1: SWCLK (0x4004_C004 + 0x04)
- PA2: SWDIO (0x4004_C004 + 0x08)

## Register Layout Per Port

Base: `GPIO_PORT<X>` where X ∈ {A..H}

| Offset | Register | Width | Bits | Purpose |
|--------|----------|-------|------|---------|
| 0x00 | CTRL | 32 | — | Port control (slew rate, etc.) |
| 0x04 | MODEL | 32 | [0..31] | Pin 0-3 mode (4 bits each) |
| 0x08 | MODEH | 32 | [0..31] | Pin 4-7 mode (4 bits each) |
| 0x0C | DOUT | 32 | [0..15] | Data output |
| 0x10 | DIN | 32 | [0..15] (RO) | Data input |
| 0x14 | DOUTTGL | 32 | — | Toggle DOUT |

**Pin MODE values (4 bits each):**
```
0x0: Disabled
0x1: Input (PULLUP if Pull::Up)
0x2: Reserved
0x3: Output
0x4: Output OpenDrain
0x5: Output OpenSource
... other special functions
```

## Interrupt System

### External Interrupts
Each pin number (0-15) has **one** external interrupt across all ports:
- Pin 0 on any port uses EXTI0
- Pin 1 on any port uses EXTI1
- ...
- Pin 15 on any port uses EXTI15

**Constraint**: Only ONE pin with a given number can listen across all ports simultaneously.

### Interrupt Flags
Register: `GPIO->IF` (0x4004_C1F4, read/write)
Register: `GPIO->IFC` (0x4004_C1F8, write-clear)

Bit layout: One bit per pin (0-15)

```rust
// Clear interrupt flag for pin N
gpio.ifc.write(|w| w.bits(1 << N));
```

### Interrupt Enable
Register: `GPIO->IEN` (0x4004_C1FC)

## Embedded-HAL 1.0 Trait Implementations

### InputPin
```rust
pub trait InputPin {
    type Error = Infallible;
    
    fn is_high(&mut self) -> Result<bool, Self::Error>;
    fn is_low(&mut self) -> Result<bool, Self::Error>;
}

// All Input<'d> implement InputPin
```

### OutputPin
```rust
pub trait OutputPin {
    type Error = Infallible;
    
    fn set_low(&mut self) -> Result<(), Self::Error>;
    fn set_high(&mut self) -> Result<(), Self::Error>;
}

// All Output<'d> implement OutputPin
```

### StatefulOutputPin
```rust
pub trait StatefulOutputPin: OutputPin {
    fn is_set_high(&mut self) -> Result<bool, Self::Error>;
    fn is_set_low(&mut self) -> Result<bool, Self::Error>;
}

// All Output<'d> implement StatefulOutputPin
```

### ErrorType
```rust
pub trait ErrorType {
    type Error;
}

// GPIO error type is core::convert::Infallible (cannot fail)
```

## Hardware Constraints

### Pin Count Per Port
- **PA/PC/PD/PE**: Standard pins, all IO capable
- **PB**: Limited pins (0-4 only)
- **PF/PH**: Very limited (0-1 or 0-2)
- **PG**: Single pin (0 only)

### Electrical Characteristics
| Parameter | Min | Typ | Max | Unit |
|-----------|-----|-----|-----|------|
| Vdd | 2.7 | 3.3 | 3.6 | V |
| VinHigh | 0.7*Vdd | — | Vdd | V |
| VinLow | 0 | — | 0.3*Vdd | V |
| Output High | 0.8*Vdd | — | Vdd | V |
| Output Low | 0 | — | 0.2*Vdd | V |
| Drive Strength | — | Typical | 4-20 | mA |

### Slew Rate Control
Slew rate is a **per-port setting**, not per-pin:
- Register: `GPIO_PORT<X>->CTRL`
- Bits: [14..10] = SLEWRATE (5 bits, values 0-7)
- Setting affects ALL pins on that port

```
Value 0: Slowest slew rate (minimizes EMI)
Value 7: Fastest slew rate (maximum frequency)
```

## Key Implementation Details

### Type Erasure (AnyPin)
Pins are type-erased to `AnyPin` at runtime:
- Compile-time: `GpioPin<PA, P7>` (static, zero-cost)
- Runtime: `AnyPin` (wrapped in enum, single register base)
- Degrade pattern: Compile-time becomes runtime when needed

### Safe References
Once a safe reference `&gpio_port` is obtained via `unsafe { &*PORT }`:
- Operations on reference are safe (no additional unsafe needed)
- Register reads/writes via reference are valid memory access
- No need to wrap each operation in unsafe

### Pin Initialization
Pin must be:
1. Configured via MODE register before use
2. Only one Output or Input per pin (not both)
3. Cannot change mode during interrupt handling
4. Pull-up/down controlled via MODE bits

## Validation Rules

1. **Pin Existence**: Checked at compile-time (type system)
2. **Pin Availability**: Runtime - pin must not be already taken
3. **Drive Mode Validity**: OpenDrain/OpenSource require pull-up (external)
4. **Interrupt Conflicts**: Only one pin number per interrupt handler

## Error Handling

GPIO operations are **infallible by design**:
- Type system prevents invalid operations
- All configurations are valid at runtime
- No Result types for GPIO methods

**Exception**: When using embedded-hal traits, operations return `Result<T, Infallible>`:
```rust
use embedded_hal::digital::OutputPin;
led.set_high().ok();  // Converts Infallible to Ok(())
```

## Performance

### Compile-Time Zero Cost
- Type parameters optimized away
- Phantom markers have zero size
- Inlines to single register access

### Register Access Timing
- GPIO register read/write: ~2-3 CPU cycles
- Toggle via DOUTTGL: Same as write
- No CPU locks required (GPIO is independent peripheral)

## Reference Manual

**EFR32MG24 Reference Manual:**
- Section 23: GPIO
- Section 23.3: Register Descriptions
- Section 23.3.1-23.3.10: Detailed register layout
- Electrical characteristics in Section 3

---

**Version**: 2.0.0  
**Last Updated**: 2026-09-18  
**Audience**: AI agents, developers, automated systems
