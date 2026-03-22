use core::sync::atomic::{AtomicBool, Ordering};

use crate::ffi::cmu;

static TAKEN: AtomicBool = AtomicBool::new(false);

pub struct Pins {
    pub d0: (char, u8),
    pub d1: (char, u8),
    pub d2: (char, u8),
    pub d3: (char, u8),
    pub d4: (char, u8),
    pub d5: (char, u8),
    pub d6: (char, u8),
    pub d7: (char, u8),
    pub d8: (char, u8),
    pub d9: (char, u8),
    pub d10: (char, u8),
    pub d11: (char, u8),
    pub d12: (char, u8),
    pub d13: (char, u8),
    pub d14: (char, u8),
    pub d15: (char, u8),
    pub d16: (char, u8),
    pub d17: (char, u8),
    pub d18: (char, u8),
    pub on_board_led: (char, u8),
}

pub struct Prehirpals {
    pub pins: Pins,
}

impl Prehirpals {
    pub fn take() -> Self {
        if TAKEN.swap(true, Ordering::Relaxed) {
            panic!("Prehirpals already taken");
        }

        unsafe {
            if cmu::cmu_wrap_init_78mhz() == 0 {
                panic!("Failed to initialize core clock to 78 MHz");
            }
        }

        Self {
            pins: Pins {
                d0: ('C', 0),
                d1: ('C', 1),
                d2: ('C', 2),
                d3: ('C', 3),
                d4: ('C', 4),
                d5: ('C', 5),
                d6: ('C', 6),
                d7: ('C', 7),
                d8: ('A', 3),
                d9: ('A', 4),
                d10: ('A', 5),
                d11: ('A', 9),
                d12: ('A', 8),
                d13: ('B', 2),
                d14: ('B', 3),
                d15: ('B', 0),
                d16: ('B', 1),
                d17: ('A', 0),
                d18: ('D', 2),
                on_board_led: ('A', 7),
            },
        }
    }
}
