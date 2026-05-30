# `mg24-hal`

> **Pure Rust, type‑state GPIO HAL for the EFR32MG24 (Cortex‑M33)**  
> Built directly on [`efr32mg24-pac`](https://crates.io/crates/efr32mg24-pac).  
> No C, no EMLIB, no bindings.

## Notice
for now i dont have the board i am wating to get it so this code is expermintal so please for any one who tried this hal to conatct me and tell me if it worked or not i will providde links to my platforms and if the code bricked you baord so sorry to hear that 

- **WhatsApp**: 0776708542
- **Gmail**: fahadhiyari0@gmail.com
- **instagram**: fr9rx

## 🎯 Hardware Target

- **Chip**: EFR32MG24B220F1536IM48  
- **Board**: Seeed Studio XIAO MG24 Sense  
- **Core**: ARM Cortex‑M33 @ 78 MHz, FPU, TrustZone  
- **Memory**: 1536 KB flash, 256 KB RAM  

## ✅ Currently Supported

| Peripheral | Features |
|------------|----------|
| GPIO       | Output (`write_high`, `write_low`, `write_toggle`), Input (`read` with pull‑up/down/floating) |
| `embedded-hal` 1.0 | `OutputPin`, `InputPin`, `StatefulOutputPin` traits |
| Type‑state pins | Zero‑cost, compile‑time safe (`Unknown` → `Output` / `Input`) |

## 📦 Prerequisites

```bash
# Rust embedded target for Cortex‑M33 with FPU
rustup target add thumbv8m.main-none-eabihf

# ARM toolchain (for linking, if not already present)
# Ubuntu/Debian:
sudo apt install gcc-arm-none-eabi

# Flashing tool
cargo install probe-rs-tools --locked

```
## 🔨 Building & Flashing

```bash
# Clone the repository
git clone https://github.com/fr9rx/mg24-hal
cd mg24-hal

# Build an example
cargo build --example blink

# Flash and run on a connected XIAO MG24
cargo run --example blink

```
