# mg24-hal-macros

Procedural macros for the mg24-hal embedded HAL for Silicon Labs EFR32MG24 microcontroller.

[![Crates.io](https://img.shields.io/crates/v/mg24-hal-macros.svg)](https://crates.io/crates/mg24-hal-macros)
[![Documentation](https://docs.rs/mg24-hal-macros/badge.svg)](https://docs.rs/mg24-hal-macros/)
[![License](https://img.shields.io/crates/l/mg24-hal-macros.svg)](LICENSE)

## Features

- **`#[main]`** - Mark the application entry point
- **`#[interrupt]`** - Register interrupt and exception handlers with compile-time validation

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
mg24-hal-macros = "2.0"
```

### Entry Point

```rust
#[mg24_hal::main]
fn main() -> ! {
    // Application code here
    loop {}
}
```

### Interrupt Handler

```rust
#[mg24_hal::interrupt]
fn GPIO_ODD() {
    // Handle GPIO odd-numbered pins interrupt
}
```

## Features

- **Compile-time validation** of handler names against device specification
- **RAM placement** by default for fast interrupt latency
- **Optional flash placement** to conserve RAM: `#[mg24_hal::interrupt(flash)]`
- **No dependencies** on syn/quote - uses only proc_macro for minimal overhead

## Documentation

Full documentation is available at https://docs.rs/mg24-hal-macros/

## Related

- Main HAL: [mg24-hal](https://crates.io/crates/mg24-hal)
- Documentation: [docs.rs/mg24-hal](https://docs.rs/mg24-hal/)

## License

Licensed under the MIT license (LICENSE or http://opensource.org/licenses/MIT)
