# MG24-HAL

A Rust Hardware Abstraction Layer (HAL) for the Silicon Labs EFR32MG24 
wireless SoC, targeting the Seeed Studio XIAO MG24.

Built with an ESP-RS inspired API style, backed by Silicon Labs EMLIB 
for battle-tested peripheral initialization, and fully compatible with 
the embedded-hal 1.0 ecosystem.

## Features

- `embedded-hal` 1.0 compatible — works with any community driver
- ESP-RS style API — clean, explicit, ownership-based
- EMLIB backed — Silicon Labs errata and register complexity handled
- `no_std` — bare metal, no OS required
- Type-safe peripheral ownership — compile time safety

## Hardware

Primary target: Seeed Studio XIAO MG24 (EFR32MG24B220F1536IM48)
- ARM Cortex-M33 @ 78MHz
- 1536KB Flash / 256KB RAM  
- 2.4GHz multiprotocol radio (BLE, Matter, Thread, Zigbee)

## Status

🚧 Early development — contributions welcome

| Peripheral | Status |
|---|---|
| GPIO Output | ✅ Working |
| GPIO Input  | 🔄 In Progress |
| Delay       | ⏳ Planned |
| UART        | ⏳ Planned |
| I2C         | ⏳ Planned |
| SPI         | ⏳ Planned |

## Usage
```rust
let p       = Peripherals::take();
let mut led = OutputPin::new(p.pins.on_board_led);

led.set_high();
led.toggle();
```

## License
