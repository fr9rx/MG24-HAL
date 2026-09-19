# mg24-generate

Generate new mg24-hal projects quickly and easily — similar to `esp-generate` for ESP-IDF.

## Installation

```bash
cargo install mg24-generate
```

## Usage

### Create a blank project

```bash
mg24-generate my-project
cd my-project
cargo build --release
cargo run --release  # Flash to device with probe-rs
```

### Create from template

```bash
# Blink example
mg24-generate blink-project --template blink

# Button input example
mg24-generate button-project --template button

# I2C example
mg24-generate i2c-project --template i2c

# DMA example
mg24-generate dma-project --template dma
```

## Features

- ✅ Creates standalone mg24-hal projects
- ✅ Includes Cargo.toml with proper dependencies
- ✅ Provides .cargo/config.toml with probe-rs runner
- ✅ Multiple project templates (blank, blink, button, i2c, dma)
- ✅ Ready to build and flash immediately
- 🔄 TUI mode (coming soon): Interactive project generation with retro ncurses-style interface

## Generated Project Structure

```
my-project/
├── Cargo.toml              # Project manifest with mg24-hal dependency
├── src/
│   └── main.rs            # Project code
├── .cargo/
│   └── config.toml        # Build configuration with probe-rs runner
└── .gitignore             # Standard Rust/embedded ignore patterns
```

## Building

```bash
cd my-project
cargo build --release
```

## Flashing

With probe-rs connected:

```bash
cargo run --release
```

Or manually:

```bash
cargo build --release
probe-rs run --chip EFR32MG24B220F1536IM48 target/thumbv8m.main-none-eabihf/release/my-project
```

## Available Templates

- **blank** (default): Minimal project with initialization
- **blink**: LED blinking example
- **button**: Button input with LED control
- **i2c**: I2C bus communication
- **dma**: DMA memory transfer

## Help

```bash
mg24-generate --help
```

## Future Enhancements

A TUI mode with interactive project configuration is planned, featuring a retro ncurses-style interface similar to Linux menuconfig. This will allow users to:
- Select project name interactively
- Browse and choose from available templates
- Preview template descriptions
- Confirm generation before creating

---

**Version**: 0.3.0
