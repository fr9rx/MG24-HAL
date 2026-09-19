# MG24-HAL Examples

This directory contains independent Cargo projects demonstrating mg24-hal features. Each example is a **standalone project** that can be cloned and compiled independently.

## Using Examples

### Clone a Single Example

Instead of cloning the entire mg24-hal repository, you can clone just the example you want:

```bash
# Clone only the blink example
git clone --sparse --filter=blob:none https://github.com/fr9rx/mg24-hal.git
cd mg24-hal
git sparse-checkout set examples/blink
cd examples/blink
cargo build --release
```

### Or Clone Everything

```bash
git clone https://github.com/fr9rx/mg24-hal.git
cd mg24-hal/examples/blink
cargo build --release
```

### Build an Example

Each example is a complete Cargo project with its own `Cargo.toml`:

```bash
cd examples/blink
cargo run --release
```

## Available Examples

### GPIO Examples

- **blink** — Toggle LED with 500ms delay
- **button** — Read button input and control LED
- **flex** — Runtime pin direction changes
- **open_drain** — Open-drain output example
- **gpio_interrupt** — GPIO edge-triggered interrupt

### Timing

- **cpu_speed** — Configure CPU clock speed

### I2C Examples

- **i2c** — Basic I2C write/read operations
- **i2c_scan** — Scan I2C bus for devices
- **i2c_scan_full** — Full I2C bus scan with detailed reporting
- **i2c_dma** — I2C with DMA-assisted transfers

### DMA Examples

- **dma_m2m** — Memory-to-memory DMA transfer
- **dma_unsafe_addr** — DMA using raw addresses
- **dma_advanced** — Concurrent multi-channel transfers
- **dma_peripherals** — DMA with peripheral integration

### Miscellaneous

- **logging** — RTT logging over debug probe

## Example Structure

Each example follows this structure:

```
examples/blink/
├── Cargo.toml          # Project manifest with mg24-hal dependency
└── src/
    └── main.rs         # Example code
```

### Building the Example

```bash
cd examples/blink
cargo build --release
```

### Flashing to Device

Using probe-rs (recommended):

```bash
cd examples/blink
cargo flash --chip EFR32MG24 --release
```

Using other tools:

```bash
# With your favorite flashing tool
cargo build --release
# Flash target/thumbv7em-none-eabihf/release/mg24-hal-example-blink
```

## Dependencies

Each example depends on:
- **mg24-hal** (the HAL library)
- Standard Rust embedded libraries (embedded-hal, etc.)

The dependency is specified as a path in each `Cargo.toml`:

```toml
[dependencies]
mg24-hal = { version = "2.0.0", path = "../.." }
```

## Hardware Requirements

All examples target the **EFR32MG24** microcontroller:
- Silicon Labs EFR32MG24 evaluation board or XIAO MG24

### Pin Assignments

Examples use standard XIAO MG24 pins:
- **PA7**: Orange LED (blink, flex, etc.)
- **PC1**: LED (button)
- **PC3**: Button input
- **PC0/PC1**: I2C SDA/SCL
- Other pins as needed for specific examples

## Example Descriptions

### GPIO Examples

#### blink
Simple LED toggle at 500ms intervals. Best starting point.

```rust
loop {
    led.toggle();
    delay.delay_ms(500);
}
```

#### button
Read a button and mirror its state to LED.

#### open_drain
Demonstrates open-drain output mode (typically used for I2C, 1-wire).

#### gpio_interrupt
GPIO interrupt on falling edge with handler.

#### flex
Runtime-configurable pin direction switching.

### I2C Examples

#### i2c_scan
Scan I2C bus and report discovered addresses.

```
Scanning I2C bus...
  0x3C: Device found (SSD1306 OLED)
  0x68: Device found (MPU6050 IMU)
```

#### i2c
Basic I2C master read/write operations.

#### i2c_dma
Use LDMA for high-speed I2C transfers.

### DMA Examples

#### dma_m2m
Copy memory buffer using DMA (safe reference-based API).

```rust
dma.copy_slice(0, &src, &mut dst)?;
while !dma.is_transfer_done(0).unwrap_or(false) { }
```

#### dma_unsafe_addr
Advanced DMA using raw addresses (unchecked).

#### dma_advanced
Concurrent transfers on multiple channels.

## Workflow

### For Development

```bash
# Make a change to an example
cd examples/blink
nano src/main.rs

# Test the change
cargo build --release
cargo flash --chip EFR32MG24 --release
```

### For Learning

1. Start with `blink` (simplest)
2. Try `button` (add input handling)
3. Explore `i2c_scan` (I2C communication)
4. Experiment with `dma_m2m` (DMA transfers)

## Troubleshooting

### Compilation Error: "No such file"

Ensure you're in the correct directory:

```bash
cd examples/blink  # Not just examples/
cargo build --release
```

### Linking Error: "cannot find crate mg24_hal"

The example should automatically pull mg24-hal from the path. If it doesn't:

```bash
cd examples/blink
cargo clean
cargo build --release
```

### Device Not Found

Ensure:
- Debug probe connected (CMSIS-DAP, J-Link, etc.)
- `probe-rs` installed: `cargo install probe-rs-tools`
- Device recognized: `probe-rs list`

## Contributing Examples

To add a new example:

1. Create folder: `examples/my_feature/`
2. Add `Cargo.toml` with path dependency to mg24-hal
3. Create `src/main.rs` with example code
4. Ensure compiles: `cargo build --release`
5. Test on hardware
6. Submit PR with documentation

## Documentation

- **API Reference**: See `docs/GPIO_API.md`, `docs/I2C_API.md`, `docs/DMA_API.md`, `docs/DELAY_API.md`
- **Getting Started**: See project README
- **Contributing**: See `CONTRIBUTING_POLICY.md`

---

**Version**: 2.0.0  
**Last Updated**: 2026-09-18
