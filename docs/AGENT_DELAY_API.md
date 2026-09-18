# Delay API - Agent/Developer Specification

Complete technical specification for Delay module. Authoritative reference for agents implementing Delay features.

## Module Location
`src/delay.rs`

## Type Signatures

### Delay
SysTick-based delay implementation.

```rust
pub struct Delay;

impl Delay {
    pub fn new() -> Self
    // Creates new Delay instance
    // Initializes SysTick timer internally
    // Ready to use immediately
    
    pub fn delay_ms(&self, ms: u32)
    // Busy-wait delay in milliseconds
    // Blocks CPU for entire duration
    
    pub fn delay_us(&self, us: u32)
    // Busy-wait delay in microseconds
    // Blocks CPU for entire duration
}
```

## Embedded-HAL 1.0 Traits

### DelayNs Trait Implementation
```rust
pub trait DelayNs {
    fn delay_ns(&mut self, ns: u32);
    fn delay_us(&mut self, us: u32);
    fn delay_ms(&mut self, ms: u32);
}

impl DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) { /* convert to us */ }
    fn delay_us(&mut self, us: u32) { /* busy-wait */ }
    fn delay_ms(&mut self, ms: u32) { /* busy-wait */ }
}
```

## Hardware

### SysTick Timer
ARM Cortex-M4F core peripheral (not on LDMA/GPIO buses).

**Base Address**: 0xE000_E000 (System Control Block)

### SysTick Registers

| Address | Register | Width | Mode | Purpose |
|---------|----------|-------|------|---------|
| 0xE000_E010 | SYST_CSR | 32 | RW | Control and Status |
| 0xE000_E014 | SYST_RVR | 32 | RW | Reload Value |
| 0xE000_E018 | SYST_CVR | 32 | RW | Current Value |
| 0xE000_E01C | SYST_CALIB | 32 | RO | Calibration |

### SYST_CSR Register Bits

```
Bit 16: COUNTFLAG - Timer counted to zero (read clears)
Bit 2:  CLKSOURCE - Clock source (0=refclk, 1=processor)
Bit 1:  TICKINT   - Interrupt enable (0=disabled, 1=enabled)
Bit 0:  ENABLE    - Counter enable (0=disabled, 1=running)
```

### SYST_RVR (Reload Value)

```
Bits [23:0]: RELOAD - Value to reload on underflow
             Valid: 0 to 16,777,215 (24-bit)
```

### SYST_CVR (Current Value)

```
Bits [23:0]: CURRENT - Current timer value
             Counts down from RELOAD to 0
```

## Timing Calculations

### Clock Frequency
```
For EFR32MG24 (CLKSOURCE=1, processor clock):
Clock = HFCLK = 39 MHz (typical)

Timer period:
1 tick = 1 / 39MHz = 25.64 nanoseconds
1 microsecond = 39 ticks (25.64ns * 39 ≈ 1000ns)
1 millisecond = 39,000 ticks
```

### Reload Value Calculation
```
To delay N microseconds:
RELOAD = N * (HFCLK_MHz)

Examples for 39 MHz:
1 μs  → RELOAD = 39
10 μs → RELOAD = 390
100 μs → RELOAD = 3,900
1 ms → RELOAD = 39,000
10 ms → RELOAD = 390,000
```

### Maximum Delay
```
24-bit counter maximum: 16,777,215 ticks

At 39 MHz:
Max delay = 16,777,215 / 39,000,000 = 0.43 seconds

For delays > 0.43s: Use multiple SysTick cycles or loop
For delays >> 1 second: Recommended to use timer module instead
```

## Implementation Details

### Busy-Wait Loop
```rust
pub fn delay_ms(&self, ms: u32) {
    for _ in 0..ms {
        // Reload SysTick for 1ms delay
        while !SYST.did_underflow() { }
    }
}
```

**Characteristics:**
- Each loop iteration waits for 1 millisecond
- CPU clock gate OFF (draws full current)
- Cannot sleep or context-switch during delay
- Accurate to ±1 millisecond

### Granularity
```
Millisecond delays: 1 ms resolution
Microsecond delays: 1 us resolution (approximately)

NOTE: Nanosecond delays not truly supported (converted to μs)
```

## Accuracy and Resolution

### Millisecond Delays
```
Resolution: 1 ms
Accuracy: ±1 ms
Clock source: HFCLK
Overhead: ~5-10 CPU cycles per call
```

### Microsecond Delays
```
Resolution: ~25.6 ns (depends on HFCLK)
Accuracy: ±50-100 ns
Clock source: HFCLK
Overhead: ~50-100 CPU cycles per call
```

### Nanosecond Delays
```
Resolution: Cannot achieve true nanosecond accuracy
Conversion: ns → us → RELOAD value
Minimum practical: ~100 ns (due to function call overhead)

Note: DelayNs trait allows calling delay_ns(), but resolution
      is actually microseconds. For true nanosecond accuracy,
      use inline assembly or DSP operations.
```

## Interrupt Behavior

### SysTick Interrupt
```
If TICKINT = 1: Generates interrupt on count = 0
Handler name: SysTick (standard ARM name)

Typical usage: Interrupt-driven instead of busy-wait
```

### mg24-hal Implementation
```rust
// Current implementation: Busy-wait, no interrupt
// TICKINT = 0 (interrupts disabled)
// Could be extended to interrupt-driven if needed
```

## Systick Initialization

### Default State
```
CLKSOURCE = 1 (processor clock, 39 MHz)
ENABLE = 0 (disabled initially)
TICKINT = 0 (no interrupt)
RELOAD = varies per delay call
```

### Startup Sequence
```
1. RELOAD ← delay value in ticks
2. CVR ← 0 (start from 0)
3. CSR ← enable bit (start counting)
4. Poll COUNTFLAG until set
5. CSR ← clear enable bit (stop)
```

## Performance Characteristics

### CPU Usage During Delay
```
During delay: 100% CPU utilization
Power: Full operating current (no reduction)
Register accesses: 3-4 per iteration

For power-critical applications: Use timer/interrupt instead
```

### Overhead
```
Function call: ~20 CPU cycles
Register setup: ~5 CPU cycles
Loop per millisecond: ~3-4 CPU cycles
```

### Throughput
```
Can service thousands of delays per second
No queue/buffering needed (immediate execution)
Blocking operation (CPU stalled)
```

## Embedded-HAL Trait Bounds

### Generic Code
```rust
pub fn initialize<D: DelayNs>(delay: &mut D) {
    delay.delay_ms(100);  // Works with any DelayNs
}
```

### Trait Requirements
```rust
// Any type T where:
// - T implements DelayNs
// - delay_ms(u32) method available

// Can be:
// - mg24_hal::Delay (SysTick)
// - Any other HAL's Delay
// - Mock/test implementation
```

## Constraints and Limitations

| Constraint | Value | Notes |
|-----------|-------|-------|
| Max single delay | ~429 ms | 24-bit limit at 39 MHz |
| Min resolution | 26 ns | SysTick tick period |
| CPU blocking | 100% | No power saving |
| Nesting | Allowed | Multiple delays in sequence |
| Interrupts | Block-able | Interrupt disables underflow detection |
| Accuracy | ±1-100 ns | Depends on HFCLK jitter |

## Register Access Patterns

### Reading Current Tick Count
```rust
let current = SYST.cvr().read().bits();  // Returns 24-bit value
```

### Checking Underflow
```rust
let underflowed = (SYST.csr().read().bits() & (1 << 16)) != 0;
```

### Starting New Delay
```rust
SYST.rvr().write(|w| w.bits(reload_value));  // Set reload
SYST.cvr().write(|w| w.bits(0));             // Clear counter
SYST.csr().modify(|_, w| w.enable().set_bit()); // Start
```

## Power Considerations

### Energy Consumption
```
During busy-wait delay:
- Full CPU frequency (39 MHz)
- Full operating current (~10-20 mA)
- No power gating

Total energy for 1s delay at 39 MHz:
≈ 39 MHz * 1s * I = High power consumption

Alternative for power-critical: Use timer interrupt + sleep
```

### Low-Power Alternative
```
Instead of busy-wait in main:

// Use TIMER0 + interrupt
#[timer0_interrupt]
fn TIMER0() {
    // Delay complete - resume
}

// Enter sleep mode
sleep();  // CPU off, woken by timer
```

## Clock Source Selection

### CLKSOURCE Bit Options
```
0: Reference clock (48 kHz typical) - Lower power, slower
1: Processor clock (39 MHz typical) - Higher power, accurate

mg24-hal uses: CLKSOURCE = 1 (processor clock)
Reason: Accurate microsecond timing needed
```

### Alternative: Reference Clock
```
For low-power delays:
Switch CLKSOURCE to 0 (48 kHz)

Timer runs slower: 1 tick = 20.8 μs
Max delay: ~349 seconds
Accuracy: ±20 μs (lower resolution)
```

## Debugging

### Check SysTick Status
```rust
// Is timer enabled?
let enabled = (SYST.csr().read().bits() & 1) != 0;

// Did it underflow?
let underflowed = (SYST.csr().read().bits() & (1 << 16)) != 0;

// Current tick count?
let count = SYST.cvr().read().bits();
```

### Verify HFCLK
```
Expected: 39 MHz
Check via: CMU_STATUS, HFCLKSEL bits
Actual frequency determines timing accuracy
```

## Reference Manual

**ARM Cortex-M4 Technical Reference Manual:**
- Section 8.3: SysTick Timer
- Register description and bit layout
- Timing examples

**EFR32MG24 Reference Manual:**
- Section 25: EMU (clock configuration affects HFCLK)
- HFCLK frequency and dividers

---

**Version**: 2.0.0  
**Last Updated**: 2026-09-18  
**Audience**: AI agents, developers, automated systems
