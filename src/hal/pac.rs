use crate::hal::pin::Pin;
use core::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
pub enum PrephipalsErros {
    AlreadyTaken,
}

static TAKEN: AtomicBool = AtomicBool::new(false);

#[allow(non_snake_case)]

pub struct Pins {
    pub GPIO0: Pin<'C', 0>,
    pub GPIO1: Pin<'C', 1>,
    pub GPIO2: Pin<'C', 2>,
    pub GPIO3: Pin<'C', 3>,
    pub GPIO4: Pin<'C', 4>,
    pub GPIO5: Pin<'C', 5>,
    pub GPIO6: Pin<'C', 6>,
    pub GPIO7: Pin<'C', 7>,
    pub GPIO8: Pin<'A', 3>,
    pub GPIO9: Pin<'A', 4>,
    pub GPIO10: Pin<'A', 5>,
    pub GPIO11: Pin<'A', 9>,
    pub GPIO12: Pin<'A', 8>,
    pub GPIO13: Pin<'B', 2>,
    pub GPIO14: Pin<'B', 3>,
    pub GPIO15: Pin<'B', 0>,
    pub GPIO16: Pin<'B', 1>,
    pub GPIO17: Pin<'A', 0>,
    pub GPIO18: Pin<'D', 2>,
    pub GPIO19: Pin<'A', 7>,
}

pub struct Prehirpals {
    pub pins: Pins,
}

impl Prehirpals {
    pub fn take() -> Result<Self, PrephipalsErros> {
        if TAKEN.swap(true, Ordering::Relaxed) {
            return Err(PrephipalsErros::AlreadyTaken);
        }

        Ok(Self {
            pins: Pins {
                GPIO0: Pin::new(),
                GPIO1: Pin::new(),
                GPIO2: Pin::new(),
                GPIO3: Pin::new(),
                GPIO4: Pin::new(),
                GPIO5: Pin::new(),
                GPIO6: Pin::new(),
                GPIO7: Pin::new(),
                GPIO8: Pin::new(),
                GPIO9: Pin::new(),
                GPIO10: Pin::new(),
                GPIO11: Pin::new(),
                GPIO12: Pin::new(),
                GPIO13: Pin::new(),
                GPIO14: Pin::new(),
                GPIO15: Pin::new(),
                GPIO16: Pin::new(),
                GPIO17: Pin::new(),
                GPIO18: Pin::new(),
                GPIO19: Pin::new(),
            },
        })
    }
}
