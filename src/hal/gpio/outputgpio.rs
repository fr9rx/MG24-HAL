use crate::ffi::{cmu, gpio};

fn port_from_char(port: char) -> u32 {
    match port {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        _ => panic!("Invalid port"),
    }
}

pub struct OutputPin {
    _pin: (char, u8),
}

impl OutputPin {
    pub fn new(pin: (char, u8)) -> Self {
        unsafe {
            cmu::cmu_wrap_enable_gpio();
            gpio::gpio_wrap_pin_cfg(port_from_char(pin.0) as u32, pin.1 as u32, 4, 0);
        }
        Self { _pin: pin }
    }

    pub fn set_high(&self) {
        unsafe {
            gpio::gpio_wrap_pin_high(port_from_char(self._pin.0) as u32, self._pin.1 as u32);
        }
    }

    pub fn set_low(&self) {
        unsafe {
            gpio::gpio_wrap_pin_low(port_from_char(self._pin.0) as u32, self._pin.1 as u32);
        }
    }

    pub fn toggle(&self) {
        unsafe {
            gpio::gpio_wrap_pin_toggle(port_from_char(self._pin.0) as u32, self._pin.1 as u32);
        }
    }
}
