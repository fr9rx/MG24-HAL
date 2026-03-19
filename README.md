# MG24-HAL

A Rust Hardware Abstraction Layer (HAL) for the Silicon Labs EFR32MG24 
wireless SoC, targeting the Seeed Studio XIAO MG24 Sense. Built with 
an ESP-RS inspired API style, backed by Silicon Labs EMLIB for 
battle-tested peripheral initialization, and fully compatible with the 
embedded-hal 1.0 ecosystem.

This HAL is designed to be readable, teachable, and usable as learning material 
for embedded Rust developers.

---

## Hardware Target

**Seeed Studio XIAO MG24 Sense**
**Chip: EFR32MG24B220F1536IM48**

| Feature | Detail |
|---|---|
| Core | ARM Cortex-M33 @ 78MHz |
| FPU | Yes — FPv5-SP-D16 |
| TrustZone | Yes — hardware security |
| Flash | 1536KB |
| RAM | 256KB |
| Radio | 2.4GHz multiprotocol |
| Protocols | BLE 5.3, Matter, Thread, Zigbee 3.0 |
| Package | 48-pin QFN |
| Board | Seeed Studio XIAO form factor |

---

## Why This HAL Exists

The EFR32MG24 is a powerful wireless SoC with no existing Rust 
support for Series 2 devices. All prior Rust work on Silicon Labs 
chips targeted Series 0 and Series 1 devices, last updated in 2023 
and largely inactive. This HAL is pioneer territory — the first 
production-quality Rust HAL for the EFR32 Series 2 family.

---

## Design Philosophy

### ESP-RS Style API
The API is modeled after the ESP-RS ecosystem (esp-hal), which 
represents the current gold standard for embedded Rust HAL design. 
Peripherals are owned, consumed, and type-safe:
```rust
let dp = Peripherals::take();
let mut led = OutputPin::new(dp.pins.on_board_led);
let button  = InputPin::new(dp.pins.d0, Pull::Up);

led.set_high();
led.toggle();

if button.is_low() {
    led.set_high();
}
```

### EMLIB Backed
Rather than fighting raw registers from scratch, this HAL uses Silicon 
Labs' battle-tested EMLIB C library as the hardware layer. EMLIB 
handles errata workarounds, register sequencing, and chip-specific 
initialization that would take months to reverse-engineer from the 
reference manual alone.

The EMLIB C functions are wrapped in thin, safe Rust wrappers with 
all `unsafe` quarantined in a single FFI layer. The public API is 
entirely safe Rust.

### embedded-hal 1.0 Compatible
Every peripheral driver implements the appropriate embedded-hal 1.0 
traits. This means any community driver — SSD1306 OLED displays, 
BME280 sensors, SPI flash chips — works with this HAL out of the box 
with zero modification.

### no_std
Pure bare metal. No OS, no heap, no runtime dependencies beyond 
cortex-m and cortex-m-rt.

---

## How It Was Built — The Full Story

### Step 1 — Finding the Right PAC
The first challenge was finding the correct Peripheral Access Crate. 
The EFR32MG24 comes in two variants — A-series and B-series — and 
the XIAO MG24 uses the B220 variant specifically. Using the wrong 
SVD file (A020 instead of B220) results in incorrect register 
addresses and a bricked board. The correct SVD was obtained from the 
official Silicon Labs Gecko SDK DFP pack version 2025.6.2 which 
contains all 41 EFR32MG24 device variants.

Reference: `EFR32MG24B220F1536IM48.svd`

### Step 2 — EMLIB Source Files
EMLIB source files were obtained from the official Silicon Labs 
Gecko SDK repository on GitHub:
```
official emlib github repo emlib/inc/     — header files
official emlib github repo emlib/src/     — C source files
gecko_sdk/platform/Device/SiliconLabs/EFR32MG24/Include/  — chip headers
gecko_sdk/platform/CMSIS/Core/Include/   — ARM CMSIS headers
```

**Critical finding during development:** The first EMLIB files 
obtained were version 3.20.0 from 2012 — written for EFM32 Series 1 
chips under the original Energy Micro company name before Silicon Labs 
acquisition. These files are 13 years old and completely incompatible 
with the MG24 Series 2 register layout.

The Series 2 MG24 uses fundamentally different register names:

| Series 1 (broken) | Series 2 MG24 (correct) |
|---|---|
| `GPIO->IFC` | `GPIO->IF` |
| `CMU->HFPERCLKEN0` | `CMU->CLKEN0_SET / CLKEN0_CLR` |
| Standard DOUT write | Atomic DOUTSET/DOUTCLR/DOUTTGL |

The correct files come from gecko_sdk branch `gsdk_4.4` which 
targets Series 2 devices.

### Step 3 — Build System
The build system uses the `cc` crate in `build.rs` to compile EMLIB 
C sources into a static library linked into the Rust binary. Key 
details:

- Compiler: `arm-none-eabi-gcc`
- Target: `thumbv8m.main-none-eabihf` (Cortex-M33 with hard FPU)
- Chip define: `EFR32MG24B220F1536IM48`
- `cc` must be in `[build-dependencies]` not `[dependencies]`

### Step 4 — C Wrappers
EMLIB functions fall into two categories — non-inline functions that 
live in `.c` files and can be called via `extern "C"`, and 
`__STATIC_INLINE` functions defined only in headers that cannot be 
extern'd directly. For the inline functions, thin C wrapper files 
were written

### Step 5 — Clock Management
The MG24 requires explicit clock enabling for every peripheral before 
its registers can be accessed. The CMU (Clock Management Unit) 
`ClockEnable` function from Series 1 EMLIB used different register 
paths than the MG24. A minimal Series 2 compatible implementation was 
written:
```c
void CMU_ClockEnable(CMU_Clock_TypeDef clock, bool enable) {
    uint32_t bit = (clock >> CMU_EN_BIT_POS) & CMU_EN_BIT_MASK;
    if (enable) { CMU->CLKEN0_SET = 1UL << bit; }
    else        { CMU->CLKEN0_CLR = 1UL << bit; }
}
```

The GPIO clock must be enabled before any GPIO register access. 
This is done automatically inside `OutputPin::new()` and 
`InputPin::new()` so users never have to think about it.

### Step 6 — HardFault Debugging
During development a HardFault occurred at `GPIO_PinModeSet` at 
address `0x5003c300`. Root cause analysis:

1. GPIO clock was not enabled — register writes to unclocked 
   peripherals trigger precise data bus faults on Cortex-M33
2. GPIO registers were locked — PinModeSet writing locked 
   registers triggers a bus fault

Fix: enable clock first, unlock GPIO, then configure pin. This 
sequence is now enforced automatically in the HAL.

### Step 7 — First Blink
After resolving all build and runtime issues, `cargo run --example blink` 
successfully flashed the XIAO MG24 via probe-rs and the onboard 
orange LED blinked. First proof of life on real hardware.

---

## Project Structure
```
mg24-hal/
├── build.rs                         ← compiles EMLIB C into static library
├── memory.x                         ← MG24 flash/RAM layout for linker
├── Cargo.toml                       ← dependencies and build dependencies
├── Cargo.lock
├── .gitignore
├── .cargo/
│   └── config.toml                  ← target, linker flags, probe-rs runner
├── wrapper/
│   ├── gpio_wrap.h                  ← GPIO C wrapper declarations
│   ├── gpio_wrap.c                  ← GPIO C wrapper implementations
│   ├── cmu_wrap.h                   ← CMU clock C wrapper declarations
│   ├── cmu_wrap.c                   ← CMU clock C wrapper implementations
│   ├── CMSIS/                       ← ARM CMSIS headers (not inside emlib)
│   │   └── Core/
│   │       └── Include/
│   │           ├── core_cm33.h
│   │           ├── cmsis_gcc.h
│   │           ├── cmsis_compiler.h
│   │           └── cmsis_version.h
│   ├── device/                      ← chip specific headers (not inside emlib)
│   │   └── EFR32MG24/
│   │       └── Include/
│   │           ├── em_device.h
│   │           ├── EFR32MG24B220F1536IM48.h
│   │           ├── efr32mg24_gpio.h
│   │           ├── efr32mg24_cmu.h
│   │           └── system_efr32mg24.h
│   └── emlib/
│       ├── inc/                     ← EMLIB headers from gecko_sdk gsdk_4.4
│       │   ├── em_gpio.h
│       │   ├── em_cmu.h
│       │   ├── em_bitband.h
│       │   ├── em_assert.h
│       │   ├── em_bus.h
│       │   └── em_core.h
│       └── src/                     ← EMLIB sources patched for Series 2
│           ├── em_gpio.c
│           └── em_cmu.c             ← CMU_ClockEnable patched for MG24
├── hal/
│   ├── mod.rs                       ← crate root, no_std, module exports
│   ├── peripherals.rs               ← Peripherals::take() singleton
│   ├── clock.rs                     ← CMU clock management, SYSCLK constants
│   ├── delay.rs                     ← cortex_m asm delay implementation
│   └── gpio/
│       ├── mod.rs                   ← OutputPin, implementations
│       └── ffi.rs                   ← extern "C" FFI bindings to C wrappers
└── examples/
    ├── blink.rs                     ← onboard LED blink — first proof of life
```

---

## Pin Mapping — XIAO MG24

| Arduino | Port | Pin | Alternate Function |
|---|---|---|---|
| D0  | C | 0 | A0 |
| D1  | C | 1 | A1 |
| D2  | C | 2 | A2 |
| D3  | C | 3 | A3 |
| D4  | C | 4 | A4 / SDA0 |
| D5  | C | 5 | A5 / SCL0 |
| D6  | C | 6 | A6 / TX0 |
| D7  | C | 7 | A7 / RX0 |
| D8  | A | 3 | A8 / SCK0 |
| D9  | A | 4 | A9 / MISO0 |
| D10 | A | 5 | A10 / MOSI0 |
| D11 | A | 9 | A11 |
| D12 | A | 8 | A12 |
| D13 | B | 2 | A13 / SCL1 |
| D14 | B | 3 | A14 / SDA1 |
| D15 | B | 0 | A15 / MOSI1 |
| D16 | B | 1 | A16 / MISO1 |
| D17 | A | 0 | A17 / SCK1 |
| D18 | D | 2 | A18 |
| LED | A | 7 | Onboard orange LED |

**Internal pins — never use as GPIO:**
- `PB04` — RF antenna switch
- `PB05` — RF switch power
- `PA06` — internal flash CS
- `PD03` — battery control
- `PD04` — battery ADC

---

## Peripheral Status

### Core
| Peripheral | Status | Notes |
|---|---|---|
| GPIO Output | ✅ Working | set_high, set_low, toggle |
| GPIO Input  | ✅ Working | floating, pull-up, pull-down |
| Clock (CMU) | ✅ Working | GPIO clock, peripheral clock |
| Delay       | ⏳ Planned | cortex_m::asm::delay backed |

### Communication
| Peripheral | Status | Notes |
|---|---|---|
| UART / USART | ⏳ Planned | blocking TX first |
| I2C          | ⏳ Planned | embedded-hal I2c trait |
| SPI          | ⏳ Planned | embedded-hal SpiBus trait |

### Advanced
| Peripheral | Status | Notes |
|---|---|---|
| ADC (IADC)   | ⏳ Planned | |
| PWM (TIMER)  | ⏳ Planned | |
| RTC          | ⏳ Planned | |
| DMA          | ⏳ Planned | |
| Low Power    | ⏳ Planned | EM1/EM2/EM3 sleep modes |

### Radio Stack
| Protocol | Status | Notes |
|---|---|---|
| BLE 5.3  | ⏳ Planned | via BGAPI / Silicon Labs stack |
| Matter   | ⏳ Planned | requires Thread + BLE |
| Thread   | ⏳ Planned | OpenThread integration |
| Zigbee 3.0 | ⏳ Planned | via GSDK Zigbee stack |
| RAIL     | ⏳ Planned | low-level radio abstraction |

---

## Getting Started

### Prerequisites
```bash
# Rust embedded target
rustup target add thumbv8m.main-none-eabihf

# ARM toolchain for linking
# Ubuntu/Debian:
sudo apt install gcc-arm-none-eabi

# Windows:
# Download from https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads

# Flashing tool
cargo install probe-rs-tools --locked
```

### Build and Flash
```bash
# build the blink example
cargo build --example blink

# flash and run on connected XIAO MG24
cargo run --example blink
```

### Verify Your Board Is Detected
```bash
probe-rs list
# should show:
# Seeed Studio XIAO MG24 (Sense) CMSIS-DAP
```

---

## Related Projects

| Project | Description |
|---|---|
| [EFR32MG2X-RS](https://github.com/bitscrafts/EFR32MG2X-RS) | Another MG24 HAL attempt — PAC generation reference |
| [esp-hal](https://github.com/esp-rs/esp-hal) | API style inspiration |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Trait definitions |

---

## License

Licensed under:
- MIT License (LICENSE-MIT)
---

## Acknowledgments

- Silicon Labs for the EFR32MG24 and EMLIB
- The Rust Embedded Working Group for embedded-hal and tooling
- ESP-RS team for the API design inspiration
