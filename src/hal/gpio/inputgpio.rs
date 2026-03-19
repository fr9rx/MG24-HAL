use crate::ffi::cmu::cmu_wrap_enable_gpio;
use crate::ffi::gpio::{gpio_wrap_pin_cfg, gpio_wrap_pin_read};
use embedded_hal::digital::{ErrorType, InputPin as EhInput};

fn port_from_char(port: char) -> u32 {
    match port {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        _ => panic!("Invalid port"),
    }
}

pub enum Pull {
    Up,
    Down,
    Floating,
}

pub struct InputPin {
    _pin: (char, u8),
}

impl InputPin {
    pub fn new(pin: (char, u8), pull: Pull) -> Self {
        let port = port_from_char(pin.0) as u32;
        let pin_num = pin.1 as u32;
        let (mode, out) = match pull {
            Pull::Floating => (1, 0),
            Pull::Up => (2, 1),
            Pull::Down => (2, 0),
        };

        unsafe {
            cmu_wrap_enable_gpio();
            gpio_wrap_pin_cfg(port, pin_num, mode, out);
        }

        Self { _pin: pin }
    }

    pub fn read_high(&self) -> bool {
        unsafe { gpio_wrap_pin_read(port_from_char(self._pin.0) as u32, self._pin.1 as u32) != 0 }
    }

    pub fn read_low(&self) -> bool {
        !self.read_high()
    }
}

impl ErrorType for InputPin {
    type Error = core::convert::Infallible;
}

impl EhInput for InputPin {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.read_high())
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.read_low())
    }
}
