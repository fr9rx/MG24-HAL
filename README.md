# `mg24-hal`

> **Pure Rust HAL for the EFR32MG24 (Cortex‑M33), shaped after `esp-hal`**  
> Built directly on [`efr32mg24-pac`](https://crates.io/crates/efr32mg24-pac).  
> No C, no EMLIB, no bindings — and no `cortex-m-rt`: the crate brings its own
> runtime, linker script, panic handler and RTT logging.

## 🎯 Hardware Target

- **Chip**: EFR32MG24B220F1536IM48  
- **Board**: Seeed Studio XIAO MG24 Sense  
- **Core**: ARM Cortex‑M33, FPU, TrustZone  
- **Memory**: 1536 KB flash, 256 KB RAM  

## ✅ Currently Supported

The GPIO API follows `esp-hal`'s shape: `Input<'d>` / `Output<'d>` / `Flex<'d>`,
built from `with_*` config builders, with infallible operations.

The pin types mirror `esp-hal`'s three-layer arrangement: `GpioPin<PORT, PIN>`
carries the identity in the type so a nonexistent pin is a compile error, the
`Pin` trait's `degrade()` erases it into an `AnyPin`, and the drivers hold that
— which is why they are parameterised only by a lifetime. `Input<'static>` is a
type you can name in a `static`, which is what makes sharing one with an
interrupt handler tolerable.

| Peripheral | Features |
|------------|----------|
| GPIO output | Push‑pull, open‑drain, open‑source (wired‑or), pull‑up/down, per‑port slew rate |
| GPIO input | Pull‑up / pull‑down / floating, optional glitch filter |
| GPIO interrupts | `listen` / `unlisten` on rising, falling or any edge; `listen` enables the NVIC line too |
| `Flex` | Direction chosen at runtime, `apply_input_config` / `apply_output_config` / `set_input_enable` / `set_output_enable` |
| Handlers | `#[mg24_hal::interrupt]`, name checked at compile time, placed in RAM |
| Sharing | `interrupt::Mutex` for handing a driver to a handler |
| `embedded-hal` 1.0 | `OutputPin`, `InputPin`, `StatefulOutputPin`, `DelayNs` |
| Pins | `GpioPin<PORT, PIN>` tokens; a pin the chip lacks is a compile error |
| Runtime | Vector table, reset handler, `.data`/`.bss`/`.ram_text` init, FPU enable |
| Delay | Blocking, SysTick‑driven |
| Clock | HFRCODPLL bands, 4–64 MHz, factory‑trimmed from DEVINFO |
| Logging | RTT over SWD — no pins, no UART, no dependencies |
| Panic handler | Masks interrupts and spins (optional) |

Not implemented: EM4 wakeup and PRS routing, which need energy‑mode and PRS
support this crate does not have yet.

### Differences from `esp-hal`

- `esp-hal`'s `DriveStrength` has no equivalent — the EFR32MG24 has no per‑pin
  drive strength. The nearest control is `SlewRate`, which is **per port**.
- `Event` has only `RisingEdge` / `FallingEdge` / `AnyEdge`. The MG24's external
  interrupts are edge sensitive, so `LowLevel` / `HighLevel` have no hardware
  equivalent.
- esp-hal registers handlers with `set_interrupt_handler`; this crate uses the
  vector table, so handlers are named functions marked `#[mg24_hal::interrupt]`.
- No `async wait_for_*` (no executor) and no `split` / `peripheral_input` (the
  MG24 has no GPIO matrix).
- External interrupt `n` belongs to pin `n` on whichever port (RM 23.3.10.1), so
  two pins with the same number cannot both listen at once.

### ⚠️ SWD pins

`PA01` (SWCLK) and `PA02` (SWDIO) are **not** in `Pins`. They live in
`DebugPins`, reachable only through `unsafe Peripherals::take_debug_pins()`.
Repurposing them costs you the debugger until the next reset.

## 📦 Prerequisites

```bash
# Rust embedded target for Cortex-M33 with FPU
rustup target add thumbv8m.main-none-eabihf

# Flashing tool
cargo install probe-rs-tools --locked
```

No ARM GCC toolchain is needed — `rust-lld` links the firmware.

## 🚀 Usage

```rust
#![no_std]
#![no_main]

use mg24_hal::{
    CpuConfig,
    delay::init_delay,
    gpio::{Level, Output, OutputConfig},
};

#[mg24_hal::main]
fn main() -> ! {
    let dp = mg24_hal::init(CpuConfig::default()).unwrap();
    let mut led = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());
    let delay = init_delay();

    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
```

An input with a pull-up, and an open-drain output:

```rust
use mg24_hal::gpio::{DriveMode, Input, InputConfig, Output, OutputConfig, Level, Pull};

let button = Input::new(dp.pins.pc3, InputConfig::default().with_pull(Pull::Up));

let sda = Output::new(
    dp.pins.pc4,
    Level::High,
    OutputConfig::default()
        .with_drive_mode(DriveMode::OpenDrain)
        .with_pull(Pull::Up),
);
```

Edge interrupts:

```rust
use core::cell::RefCell;
use mg24_hal::{
    gpio::{Event, Input, InputConfig, Pull},
    interrupt::{self, Mutex},
};

// Hand the pin to the handler so it can clear the flag through the driver.
static BUTTON: Mutex<RefCell<Option<Input<'static>>>> = Mutex::new(RefCell::new(None));

#[mg24_hal::interrupt]
fn GPIO_ODD() {
    BUTTON.lock(|slot| {
        if let Some(button) = slot.borrow_mut().as_mut() {
            button.clear_interrupt(); // or it re-enters forever
        }
    });
}

let mut button = Input::new(
    dp.pins.pc3,
    InputConfig::default().with_pull(Pull::Up).with_filter(true),
);

interrupt::free(|| {
    button.listen(Event::FallingEdge); // enables the NVIC line for you
    BUTTON.lock(|slot| slot.borrow_mut().replace(button));
});
```

Pin `n` uses external interrupt `n`, so odd-numbered pins land in `GPIO_ODD` and
even ones in `GPIO_EVEN`. `#[mg24_hal::interrupt]` checks the name against the
vector table, so a typo is a compile error instead of a handler that never runs.

Your application needs one linker flag. In `.cargo/config.toml`:

```toml
[build]
target = "thumbv8m.main-none-eabihf"
rustflags = ["-C", "link-arg=-Tlink.x"]
```

`mg24-hal`'s build script puts `link.x` on the linker search path, so there is
no `memory.x` to copy and nothing else to configure.

## 🔨 Building & Flashing

```bash
git clone https://github.com/fr9rx/mg24-hal
cd mg24-hal

cargo build --example blink --release

# Flash and run on a connected XIAO MG24
cargo run --example blink --release
```

Build in release for anything you actually flash. The pin identity is erased to
a runtime `AnyPin`, and it is cross-crate inlining plus constant propagation
that folds it back to a constant — which is why `[profile.release]` sets
`lto = true` and `codegen-units = 1`. Without LTO the same images are roughly
four times larger.

| Example | Release image |
|---|---|
| `blink` | 1,416 B |
| `button` | 1,428 B |
| `open_drain` | 1,544 B |
| `flex` | 1,608 B |
| `gpio_interrupt` | 2,408 B |
| `cpu_speed` | 2,592 B |
| `logging` | 4,240 B |

(vector table + `.text` + `.rodata`. `logging` is the outlier because
`core::fmt` drags in formatting machinery — the argument for `defmt` if that
ever matters.)

## ⚙️ Cargo Features

| Feature | Default | Effect |
|---|---|---|
| `panic-handler` | on | Provides the `#[panic_handler]`. Turn it off (`default-features = false`) to supply your own, or to use `panic-probe`. |

## 🔌 Interrupt Handlers

Every exception and device interrupt defaults to a spin loop. Take one over by
naming a handler after it — see `link.x` for the full list:

```rust
#[mg24_hal::interrupt]
fn GPIO_EVEN() {
    // ...
}
```

The name is checked against the vector table at compile time. Since an
unhandled interrupt lands in the spinning `DefaultHandler`, write the handler
*before* calling `listen`, or the first edge parks the core.

### Handlers run from RAM

`#[mg24_hal::interrupt]` puts the handler in `.ram_text`, which the reset
handler copies out of flash before `main` runs. RAM has no wait states, so entry
latency stops depending on `MSC_READCTRL.MODE`, and the handler keeps working
while flash is being erased or programmed.

Only the handler itself moves. Anything it calls stays where the compiler put
it, so a handler that has to survive a flash write must avoid calling into
flash — or be small enough that what it needs gets inlined.

Pass `flash` to leave it in flash and get the RAM back:

```rust
#[mg24_hal::interrupt(flash)]
fn TIMER0() {
    // ...
}
```

The cost is only what you use: `gpio_interrupt`'s handler is 212 B of RAM, and
`blink`, which defines no handlers, links a zero-length `.ram_text`.

## 📝 Logging

RTT is a ring buffer in RAM that the debug probe reads over SWD while the core
keeps running. It needs no pins, no UART and no extra wiring — the same
connection used to flash the chip carries the output.

```rust
use mg24_hal::{rprintln, rtt};

rtt::init();
rprintln!("SYSCLK = {} Hz", mg24_hal::clock::sysclk_hz());
```

```bash
probe-rs attach --chip EFR32MG24B220F1536IM48 --rtt-scan-memory \
    target/thumbv8m.main-none-eabihf/release/examples/logging
```

```text
mg24-hal up, SYSCLK = 19000000 Hz
tick 0 led=High
tick 1 led=Low
```

`--rtt-scan-memory` is required: probe-rs finds the control block by scanning
RAM for its marker, and without the flag it reports nothing.

Messages are dropped rather than truncated or blocked when the buffer is full,
so a line either arrives whole or not at all, and the core never stalls waiting
for a host that is not draining. Cost is 1,116 B of `.bss` and no flash beyond
the code. `rtt::Terminal` implements `core::fmt::Write` if you want `write!`.

## ⏱️ CPU Speed

SYSCLK runs from HFRCODPLL, which comes up in its 19 MHz band. `CpuSpeed`
retunes it to any other factory-calibrated band, using the per-chip trim values
from the DEVINFO page:

```rust
use mg24_hal::{CpuConfig, clock::CpuSpeed, delay::init_delay};

let dp = mg24_hal::init(CpuConfig::default().with_cpu_speed(CpuSpeed::Mhz64)).unwrap();
let delay = init_delay(); // reads clock::sysclk_hz(), so it is calibrated for 64 MHz
```

Bands: 4, 5, 7, 10, 13, 16, 19, 20, 26, 32, 38, 48, 56, 64 MHz.

**No EMU or MSC setup is needed for any of them.** RM 11.3.5.1: "The system
defaults to VSCALE2 out of reset", which covers EM0/EM1 up to 80 MHz. And
`MSC_READCTRL.MODE` resets to `WS2` — one *more* wait state than the `WS1` the
datasheet's table requires at 78 MHz. Extra wait states cost speed, never
correctness, so both reset values are left alone.

**Why there is no 78 MHz.** 78 MHz is not an HFRCO band. The datasheet reaches
it as "HFRCO w/ DPLL": HFRCODPLL locked to the 39 MHz HFXO crystal at 2×. That
needs HFXO startup and DPLL lock, which this crate does not do yet. The 80 MHz
band the hardware offers is left out on purpose — it is above the part's 78 MHz
maximum core frequency.

## 📋 Examples

| Example | Shows | Wiring |
|---|---|---|
| `blink` | Push-pull output, SysTick delay | none |
| `button` | Input with pull-up driving an output | button on PC3 |
| `open_drain` | `DriveMode::OpenDrain` and `OpenSource` | none |
| `gpio_interrupt` | `#[interrupt]`, `Mutex`, `clear_interrupt` | button on PC3 |
| `flex` | Runtime direction changes on one pin | none |
| `cpu_speed` | `CpuSpeed` (64 MHz) + delay recalibration | none |
| `logging` | RTT logging over SWD | none |

```bash
cargo run --example blink
```
