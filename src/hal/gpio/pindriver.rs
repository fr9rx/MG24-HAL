use crate::{
    ffi::{
        cmu::cmu_wrap_enable_gpio,
        gpio::{
            gpio_wrap_pin_cfg, gpio_wrap_pin_high, gpio_wrap_pin_low, gpio_wrap_pin_read,
            gpio_wrap_pin_toggle,
        },
    },
    hal::pin::{Input, Output, Pin, Unknown, port_num},
};
use embedded_hal::digital::{ErrorType, InputPin as EhInput, OutputPin as EhOutput};

#[derive(Debug)]
pub enum GpioError {
    InitFailed,
}

impl embedded_hal::digital::Error for GpioError {
    fn kind(&self) -> embedded_hal::digital::ErrorKind {
        embedded_hal::digital::ErrorKind::Other
    }
}

#[derive(Clone, Copy)]
pub enum Pull {
    Up,
    Down,
    Floating,
}

pub struct PinDriver<const PORT: char, const PIN: u8, MODE> {
    _pin: Pin<PORT, PIN, MODE>,
}

impl<const PORT: char, const PIN: u8> PinDriver<PORT, PIN, Output> {
    pub fn output(pin: Pin<PORT, PIN, Unknown>) -> Result<Self, GpioError> {
        unsafe {
            cmu_wrap_enable_gpio();
            gpio_wrap_pin_cfg(port_num(PORT), PIN as u32, 4, 0);
        }
        Ok(Self {
            _pin: pin.into_mode::<Output>(),
        })
    }

    pub fn write_high(&mut self) -> Result<(), GpioError> {
        unsafe {
            gpio_wrap_pin_high(port_num(PORT), PIN as u32);
        }
        Ok(())
    }

    pub fn write_low(&mut self) -> Result<(), GpioError> {
        unsafe {
            gpio_wrap_pin_low(port_num(PORT), PIN as u32);
        }
        Ok(())
    }

    pub fn write_toggle(&mut self) -> Result<(), GpioError> {
        unsafe {
            gpio_wrap_pin_toggle(port_num(PORT), PIN as u32);
        }
        Ok(())
    }

    pub fn is_set_high(&mut self) -> Result<bool, GpioError> {
        Ok(unsafe { gpio_wrap_pin_read(port_num(PORT), PIN as u32) } != 0)
    }

    pub fn is_set_low(&mut self) -> Result<bool, GpioError> {
        Ok(!self.is_set_high().unwrap())
    }
}

pub struct InputConfig {
    _pull: Pull,
}

impl InputConfig {
    pub fn new(pull: Pull) -> Self {
        Self { _pull: pull }
    }

    pub fn read_pull(&mut self) -> Pull {
        self._pull
    }
}

impl<const PORT: char, const PIN: u8> PinDriver<PORT, PIN, Input> {
    pub fn input(pin: Pin<PORT, PIN, Unknown>, config: InputConfig) -> Result<Self, GpioError> {
        let (mode, out) = match config._pull {
            Pull::Floating => (1, 0),
            Pull::Up => (2, 1),
            Pull::Down => (2, 0),
        };

        unsafe {
            cmu_wrap_enable_gpio();
            gpio_wrap_pin_cfg(port_num(PORT), PIN as u32, mode, out);
        }

        Ok(Self {
            _pin: pin.into_mode::<Input>(),
        })
    }

    pub fn read(&mut self) -> Result<bool, GpioError> {
        Ok(unsafe { gpio_wrap_pin_read(port_num(PORT), PIN as u32) } != 0)
    }
}

impl<const PORT: char, const PIN: u8> ErrorType for PinDriver<PORT, PIN, Output> {
    type Error = GpioError;
}

impl<const PORT: char, const PIN: u8> ErrorType for PinDriver<PORT, PIN, Input> {
    type Error = GpioError;
}

impl<const PORT: char, const PIN: u8> EhOutput for PinDriver<PORT, PIN, Output> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.write_high().unwrap();
        Ok(())
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.write_low().unwrap();
        Ok(())
    }
}

impl<const PORT: char, const PIN: u8> EhInput for PinDriver<PORT, PIN, Input> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        if self.read().unwrap() {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        if self.read().unwrap() {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}
