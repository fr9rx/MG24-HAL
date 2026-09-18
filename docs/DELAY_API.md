# Delay API Reference

Complete documentation of the Delay module for time-based operations with embedded-hal 1.0 support.

## Module Overview

The Delay module provides delay and timing operations using the SysTick timer, with support for millisecond and microsecond granularity.

**Location**: `src/delay.rs`  
**Types**: `Delay`

## Delay Initialization

### Creating a Delay Instance

```rust
use mg24_hal::delay::Delay;

let delay = Delay::new();

// Delay is ready to use immediately
// Uses SysTick timer automatically
```

### No Parameters Required

Unlike some HAL implementations, `mg24_hal::Delay` doesn't require clock frequency or other parameters:

```rust
// ESP-hal style: simple one-liner
let delay = Delay::new();

// Ready to use with ms/us delays
delay.delay_ms(500);
delay.delay_us(1000);
```

## Delay Operations

### Millisecond Delays

```rust
pub fn delay_ms(&self, ms: u32) {
    // Wait for ms milliseconds
    // Resolution: 1 ms
}
```

**Parameters**:
- `ms`: Duration in milliseconds (0-4,294,967,295)

**Example**:
```rust
let delay = Delay::new();

// Wait 1 second
delay.delay_ms(1000);

// Wait 100ms (debounce delay)
delay.delay_ms(100);

// Wait 5 seconds
delay.delay_ms(5000);
```

### Microsecond Delays

```rust
pub fn delay_us(&self, us: u32) {
    // Wait for us microseconds
    // Resolution: varies with clock frequency
}
```

**Parameters**:
- `us`: Duration in microseconds (0-4,294,967,295)

**Example**:
```rust
// Wait 500 microseconds
delay.delay_us(500);

// Wait 10 microseconds (signal timing)
delay.delay_us(10);

// Wait 1 second in microseconds
delay.delay_us(1_000_000);
```

## Embedded-HAL 1.0 Traits

### DelayNs Trait

The Delay type implements the `embedded_hal::delay::DelayNs` trait:

```rust
use embedded_hal::delay::DelayNs;

pub fn toggle_with_hal_delay(pin: &mut impl StatefulOutputPin, delay: &mut impl DelayNs) {
    pin.toggle().ok();
    delay.delay_ms(500);
    pin.toggle().ok();
    delay.delay_ms(500);
}

// Works with any DelayNs implementation
let mut delay = Delay::new();
toggle_with_hal_delay(&mut led, &mut delay);
```

### Trait Methods

```rust
impl DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32);  // Nanosecond delay
    fn delay_us(&mut self, us: u32);  // Microsecond delay (1000x ns)
    fn delay_ms(&mut self, ms: u32);  // Millisecond delay (1000x us)
}
```

## Common Patterns

### Blink LED Loop

```rust
let delay = Delay::new();
let mut led = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());

loop {
    led.set_high();
    delay.delay_ms(500);
    
    led.set_low();
    delay.delay_ms(500);
}
```

### Button Debouncing

```rust
let delay = Delay::new();
let button = Input::new(
    dp.pins.pc3,
    InputConfig::default().with_pull(Pull::Up)
);

loop {
    if button.is_low() {
        delay.delay_ms(20);  // Hardware debounce time
        if button.is_low() {
            // Button confirmed pressed
            rprintln!("Button pressed!");
        }
    }
}
```

### Initialization Sequencing

```rust
let delay = Delay::new();

// Power up device
power_pin.set_high();

// Wait for startup time per datasheet
delay.delay_ms(100);

// Initialize I2C communication
let mut i2c = I2c0::new(/* ... */);
let device_id = read_device_id(&mut i2c)?;

rprintln!("Device ID: 0x{:04X}", device_id);
```

### PWM-Style Signal Generation

```rust
let delay = Delay::new();
let mut pin = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());

let period_ms = 10;
let duty_cycle = 75;  // 75%

let on_time = period_ms * duty_cycle / 100;
let off_time = period_ms - on_time;

loop {
    pin.set_high();
    delay.delay_ms(on_time);
    
    pin.set_low();
    delay.delay_ms(off_time);
}
```

### Timing-Critical Operations

```rust
// Measure time between button presses
let delay = Delay::new();
let button = Input::new(/* ... */);

let mut last_press = 0u32;
loop {
    if button.is_low() {
        delay.delay_ms(20);  // Debounce
        if button.is_low() {
            let now = get_tick_count();  // Your tick counter
            let interval = now - last_press;
            rprintln!("Button interval: {} ms", interval);
            last_press = now;
            
            // Wait for release
            while button.is_low() {
                delay.delay_ms(10);
            }
        }
    }
}
```

## Hardware Details (EFR32MG24)

### SysTick Timer

The Delay implementation uses the ARM Cortex-M SysTick timer:

```
SysTick Clock = HFCLK / (HFCLK_FREQUENCY_HZ / 1_000_000)

For EFR32MG24:
- HFCLK = 39 MHz (typical)
- 1 microsecond = 39 ticks
- 1 millisecond = 39,000 ticks
```

### Timer Characteristics

| Property | Value |
|----------|-------|
| Type | ARM SysTick (24-bit) |
| Maximum Delay | ~429 seconds (24-bit counter) |
| Minimum Resolution | < 1 microsecond |
| Frequency | HFCLK / prescaler |

### Register Base

- **Address**: 0xE000_E000 (Part of ARM Cortex-M4F core)
- **Reference**: ARM Cortex-M4 Technical Reference Manual

## Accuracy and Precision

### Millisecond Delays

```rust
delay.delay_ms(1000)
```

**Accuracy**: ±1 millisecond  
**Resolution**: 1 millisecond granularity  
**Use Case**: General delays, debouncing, periodic operations

### Microsecond Delays

```rust
delay.delay_us(1000)
```

**Accuracy**: ±10-100 microseconds (depends on system load)  
**Resolution**: < 1 microsecond (depends on clock frequency)  
**Use Case**: Timing-critical operations, bus protocols

### Nanosecond Delays

```rust
use embedded_hal::delay::DelayNs;
let mut delay = Delay::new();
delay.delay_ns(1000)  // Delay 1 microsecond
```

**Note**: Nanosecond delays are converted to microseconds; true nanosecond accuracy not achievable on this platform.

## Performance Notes

### CPU Usage

- **Busy-wait implementation**: Uses CPU cycles during delay
- **No idle support**: CPU cannot sleep during delay (see low-power alternatives)
- **Overhead**: ~10 clock cycles per millisecond

### Alternative: Low-Power Delays

For battery-powered applications, consider timer interrupts instead of busy-wait:

```rust
// NOT busy-wait, but interrupt-driven
#[mg24_hal::interrupt]
fn TIMER0() {
    // Delay complete, resume
}

// This requires more complex setup but saves power
```

## Common Issues

### Delay Seems Too Long/Short

**Cause**: Incorrect HFCLK frequency assumption  
**Fix**: Verify actual clock frequency in clock configuration

```rust
// If delays are 2x too long, HFCLK might be half expected
// Check EMU (Energy Management Unit) clock settings
```

### Interference with Other Operations

**Cause**: Busy-wait blocks all other operations  
**Fix**: Use interrupts for critical timing, or split delays:

```rust
let delay = Delay::new();

// Instead of single 5s delay blocking everything:
for _ in 0..50 {
    delay.delay_ms(100);
    // Interleave other operations here
    check_sensors();
    process_data();
}
```

### SysTick Handler Conflicts

**Cause**: Another part of code uses SysTick interrupt  
**Fix**: Coordinate timer usage or use dedicated timer module

## Integration with Other Modules

### With I2C Bus Operations

```rust
let delay = Delay::new();
let mut i2c = I2c0::new(/* ... */);

loop {
    // Attempt I2C operation
    match i2c.read(addr, &mut buffer) {
        Ok(()) => { /* success */ }
        Err(I2cError::Timeout) => {
            // Retry after delay
            delay.delay_ms(100);
        }
        Err(e) => rprintln!("Error: {:?}", e),
    }
}
```

### With GPIO Polling

```rust
let delay = Delay::new();
let button = Input::new(/* ... */);

loop {
    if button.is_high() {
        delay.delay_ms(500);
        if button.is_high() {
            // Long press detected
        }
    }
}
```

### With DMA Transfers

```rust
let delay = Delay::new();
let mut dma = Dma::new(/* ... */);

dma.copy_slice(0, &src, &mut dst).ok();

// Poll with timeout
let mut wait = 0;
loop {
    if dma.is_transfer_done(0).unwrap_or(false) {
        break;
    }
    delay.delay_ms(1);
    wait += 1;
    if wait > 1000 {
        rprintln!("DMA timeout");
        break;
    }
}
```

## Embedded-HAL Compatibility

### Using Delay with Generic Code

```rust
use embedded_hal::delay::DelayNs;

// Generic function works with any DelayNs type
pub fn initialize_sensor<D: DelayNs>(delay: &mut D) {
    delay.delay_ms(100);  // Power-up time
}

// Call with mg24-hal Delay
let mut delay = mg24_hal::delay::Delay::new();
initialize_sensor(&mut delay);
```

### Trait Bounds in Libraries

```rust
// Library code accepting any delay
pub struct MyDriver<D: DelayNs> {
    delay: D,
}

impl<D: DelayNs> MyDriver<D> {
    pub fn new(delay: D) -> Self {
        Self { delay }
    }
}

// Use in application
let delay = Delay::new();
let driver = MyDriver::new(delay);
```

---

**Version**: 2.0.0  
**Last Updated**: 2026-09-18
