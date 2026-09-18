# GPIO API Reference

Complete documentation of the GPIO (General Purpose Input/Output) module following esp-hal conventions.

## Module Overview

The GPIO module provides type-safe access to GPIO pins with compile-time validation and runtime configuration flexibility.

**Location**: `src/gpio.rs`  
**Types**: `GpioPin<PORT, PIN>`, `Input<'d>`, `Output<'d>`, `Flex<'d>`

## Type Safety

Each pin is identified by port and pin number at compile time:

```rust
// Compile error: pin doesn't exist
let invalid = dp.pins.pa99;

// OK: pin PA7 exists
let led = dp.pins.pa7;
```

## Input Mode

### Creating an Input Pin

```rust
use mg24_hal::gpio::{Input, InputConfig, Pull};

let button = Input::new(
    dp.pins.pc3,
    InputConfig::default()
        .with_pull(Pull::Up)
        .with_filter(true)  // Enable glitch filter
);
```

### InputConfig Builder Methods

| Method | Type | Default | Purpose |
|--------|------|---------|---------|
| `with_pull(Pull)` | `Pull::Up`, `Pull::Down`, `Pull::None` | `None` | Pull-up/down configuration |
| `with_filter(bool)` | `bool` | `false` | Enable 2-stage glitch filter |

### Reading Input State

```rust
// Read current pin level
let is_high = button.is_high();  // -> bool
let is_low = button.is_low();    // -> bool

// Alternative names
let level = button.is_set();     // High == true
```

### Edge Detection (Interrupts)

```rust
use mg24_hal::gpio::Event;

// Setup interrupt handler first
static BUTTON: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));

#[mg24_hal::interrupt]
fn GPIO_ODD() {
    BUTTON.lock(|slot| {
        if let Some(pin) = slot.borrow_mut().as_mut() {
            pin.clear_interrupt();  // Must clear to avoid re-entering
        }
    });
}

// Configure listening
button.listen(Event::FallingEdge);  // Also: RisingEdge, AnyEdge

// Clear interrupt flag after handling
button.clear_interrupt();
```

### Pin Constraints

- **Per-pin interrupts**: External interrupt `n` belongs to pin `n` on ANY port
- **Conflict avoidance**: Only one pin with number `n` can listen at a time (different ports)
- **Handler naming**: Must match vector table exactly (checked at compile time)

## Output Mode

### Creating an Output Pin

```rust
use mg24_hal::gpio::{Output, OutputConfig, Level, DriveMode};

let led = Output::new(
    dp.pins.pa7,
    Level::Low,  // Initial level
    OutputConfig::default()
        .with_drive_mode(DriveMode::PushPull)
        .with_pull(Pull::None)
        .with_slew_rate(SlewRate::Default)
);
```

### OutputConfig Builder Methods

| Method | Type | Default | Purpose |
|--------|------|---------|---------|
| `with_drive_mode(DriveMode)` | PushPull, OpenDrain, OpenSource | PushPull | Output type |
| `with_pull(Pull)` | Pull::Up, Pull::Down, Pull::None | None | Pull-up/down |
| `with_slew_rate(SlewRate)` | 0-7 (per-port setting) | Default | Slew rate control |

### Output Operations

```rust
// Set/clear output
led.set_high();
led.set_low();

// Toggle
led.toggle();

// Get current state
let current = led.is_set();  // -> bool
```

### Drive Modes

#### Push-Pull (Default)
```rust
.with_drive_mode(DriveMode::PushPull)
// Can drive high (3.3V) and low (GND)
// Standard output mode
```

#### Open-Drain
```rust
.with_drive_mode(DriveMode::OpenDrain)
// Can pull low only, external pull-up required
// Use for I2C SDA/SCL
```

#### Open-Source / Wired-Or
```rust
.with_drive_mode(DriveMode::OpenSource)
// Can pull high only, external pull-down required
// Use for active-high signaling
```

### Slew Rate (Per-Port)

Slew rate is a **port-wide setting** affecting all pins on that port:

```rust
// Affects all PA pins
pa_pin.set_slew_rate(SlewRate::Fast);  // Faster edge, more EMI
pa_pin.set_slew_rate(SlewRate::Slow);  // Slower edge, less EMI
```

Values: 0 (slowest) to 7 (fastest)

## Flexible Runtime Mode

The `Flex` pin allows runtime direction changes:

```rust
use mg24_hal::gpio::Flex;

let mut pin = Flex::new(dp.pins.pc6);

// Configure as input
pin.apply_input_config(InputConfig::default().with_pull(Pull::Up));

// Later: reconfigure as output
pin.apply_output_config(OutputConfig::default());

// Enable/disable individual directions
pin.set_input_enable(true);
pin.set_output_enable(true);
```

## Level Representation

```rust
use mg24_hal::gpio::Level;

// Create from bool
let level = Level::from(true);   // High
let level = Level::from(false);  // Low

// Convert to bool
let is_high: bool = level.into();
```

## Embedded-HAL 1.0 Traits

### OutputPin Trait

```rust
use embedded_hal::digital::OutputPin;

// All GPIO outputs implement OutputPin
led.set_low().ok();    // Infallible operations
led.set_high().ok();
```

### InputPin Trait

```rust
use embedded_hal::digital::InputPin;

// Infallible reads
let state = button.is_high().ok();
```

### ErrorType Trait

```rust
use embedded_hal::digital::ErrorType;

// GPIO has no error type
type Error = core::convert::Infallible;
```

## Pin Port-Number Map

| Port | Pins |
|------|------|
| PA | 0-9 |
| PB | 0-4 |
| PC | 0-5 |
| PD | 0-3 |
| PE | 0-2 |
| PF | 0-1 |
| PG | 0 |
| PH | 0-1 |

**DebugPins** (unsafe, debugger only):
- PA1: SWCLK
- PA2: SWDIO

## Common Patterns

### Debounced Button
```rust
let button = Input::new(
    dp.pins.pc3,
    InputConfig::default()
        .with_pull(Pull::Up)
        .with_filter(true)  // Hardware glitch filter
);

loop {
    if button.is_low() {
        delay.delay_ms(20);  // Software debounce
        if button.is_low() {
            // Button press confirmed
        }
    }
}
```

### Interrupt-Driven Button
```rust
static BUTTON: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));

#[mg24_hal::interrupt]
fn GPIO_ODD() {
    BUTTON.lock(|slot| {
        if let Some(btn) = slot.borrow_mut().as_mut() {
            btn.clear_interrupt();
            // Handle button press
        }
    });
}

// In main:
let mut button = Input::new(dp.pins.pc3, InputConfig::default().with_pull(Pull::Up));
button.listen(Event::FallingEdge);
interrupt::free(|| {
    BUTTON.lock(|slot| slot.borrow_mut().replace(button));
});
```

### Multi-LED Control
```rust
let mut led1 = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());
let mut led2 = Output::new(dp.pins.pa6, Level::Low, OutputConfig::default());

loop {
    led1.toggle();
    led2.toggle();
    delay.delay_ms(500);
}
```

## Error Handling

GPIO operations are **infallible** by design:

```rust
// No Result type - operations cannot fail
led.set_high();  // Not a Result

// When using embedded-hal traits, use .ok() for type compatibility
use embedded_hal::digital::OutputPin;
led.set_high().ok();  // Converts Infallible to Ok(())
```

## Performance Notes

- **Zero-cost abstractions**: Type erasure (degrade) compiles away
- **Compile-time checks**: Pin existence verified at build time
- **Inline operations**: Pin reads/writes inline to single register access
- **No dynamic dispatch**: Even with AnyPin, operations inline

## Hardware Details (EFR32MG24)

- **Register base**: Port-dependent (GPIO_PORT* registers)
- **Pin configuration**: MODE register (4 bits per pin)
- **Input/output**: IN/OUT registers (1 bit per pin)
- **Interrupt**: Single external interrupt per pin number

Reference: EFR32MG24 Reference Manual, Section 23.3 (GPIO)

---

**Version**: 2.0.0  
**Last Updated**: 2026-09-18
