#![no_std]

pub mod gpio;
pub(crate) mod pins;
use core::sync::atomic::{AtomicBool, Ordering};
use efr32mg24_pac::CmuS;
use pins::Pin;
pub mod delay;
pub use efr32mg24_pac as pac;

#[derive(Debug)]
pub enum PeripheralsErrors {
    AlreadyTaken,
}

static TAKEN: AtomicBool = AtomicBool::new(false);

#[allow(non_snake_case)]

pub struct CpuConfig {
    gpio_clock: bool,
}

impl Default for CpuConfig {
    fn default() -> Self {
        Self { gpio_clock: true }
    }
}

pub struct Pins {
    pub pc0: Pin<'C', 0>,
    pub pc1: Pin<'C', 1>,
    pub pc2: Pin<'C', 2>,
    pub pc3: Pin<'C', 3>,
    pub pc4: Pin<'C', 4>,
    pub pc5: Pin<'C', 5>,
    pub pc6: Pin<'C', 6>,
    pub pc7: Pin<'C', 7>,
    pub pc8: Pin<'C', 8>,
    pub pc9: Pin<'C', 9>,
    pub pb0: Pin<'B', 0>,
    pub pb1: Pin<'B', 1>,
    pub pb2: Pin<'B', 2>,
    pub pb3: Pin<'B', 3>,
    pub pb4: Pin<'B', 4>,
    pub pb5: Pin<'B', 5>,
    pub pa0: Pin<'A', 0>,
    pub pa1: Pin<'A', 1>,
    pub pa2: Pin<'A', 2>,
    pub pa3: Pin<'A', 3>,
    pub pa4: Pin<'A', 4>,
    pub pa5: Pin<'A', 5>,
    pub pa6: Pin<'A', 6>,
    pub pa7: Pin<'A', 7>,
    pub pa8: Pin<'A', 8>,
    pub pa9: Pin<'A', 9>,
    pub pd0: Pin<'D', 0>,
    pub pd1: Pin<'D', 1>,
    pub pd2: Pin<'D', 2>,
    pub pd3: Pin<'D', 3>,
    pub pd4: Pin<'D', 4>,
    pub pd5: Pin<'D', 5>,
}

pub struct Peripherals {
    pub pins: Pins,
}

pub fn init(config: CpuConfig) -> Result<Peripherals, PeripheralsErrors> {
    if TAKEN.swap(true, Ordering::AcqRel) {
        return Err(PeripheralsErrors::AlreadyTaken);
    }
    unsafe {
        let cmu = &*CmuS::ptr();
        if config.gpio_clock {
            cmu.clken0().modify(|r, w| w.bits(r.bits() | 1 << 26));
        }
    };
    Ok(Peripherals {
        pins: Pins {
            pc0: Pin::new(),
            pc1: Pin::new(),
            pc2: Pin::new(),
            pc3: Pin::new(),
            pc4: Pin::new(),
            pc5: Pin::new(),
            pc6: Pin::new(),
            pc7: Pin::new(),
            pc8: Pin::new(),
            pc9: Pin::new(),
            pb0: Pin::new(),
            pb1: Pin::new(),
            pb2: Pin::new(),
            pb3: Pin::new(),
            pb4: Pin::new(),
            pb5: Pin::new(),
            pa0: Pin::new(),
            pa1: Pin::new(),
            pa2: Pin::new(),
            pa3: Pin::new(),
            pa4: Pin::new(),
            pa5: Pin::new(),
            pa6: Pin::new(),
            pa7: Pin::new(),
            pa8: Pin::new(),
            pa9: Pin::new(),
            pd0: Pin::new(),
            pd1: Pin::new(),
            pd2: Pin::new(),
            pd3: Pin::new(),
            pd4: Pin::new(),
            pd5: Pin::new(),
        },
    })
}
