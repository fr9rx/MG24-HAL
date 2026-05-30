use crate::pins::{Input, Output, Pin, Unknown};
use efr32mg24_pac::GpioS;
use embedded_hal::digital::{
    ErrorType, InputPin as EhInput, OutputPin as EhOutput, StatefulOutputPin,
};

#[derive(Debug)]
pub enum GpioError {
    FailedToConfiguareGpio,
    FailedToSetGpio,
    FailedToSetPull,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    High,
    Low,
}

impl From<bool> for Level {
    fn from(value: bool) -> Self {
        if value { Level::High } else { Level::Low }
    }
}

impl embedded_hal::digital::Error for GpioError {
    fn kind(&self) -> embedded_hal::digital::ErrorKind {
        embedded_hal::digital::ErrorKind::Other
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Pull {
    Up,
    Down,
    Floating,
}

pub struct Gpio<const PORT: char, const PIN: u8, MODE> {
    _pin: Pin<PORT, PIN, MODE>,
}

impl<const PORT: char, const PIN: u8> Gpio<PORT, PIN, Output> {
    pub fn output(pin: Pin<PORT, PIN, Unknown>) -> Result<Self, GpioError> {
        unsafe {
            let gpio = &*GpioS::ptr();
            match (PORT, PIN) {
                ('A', 0..=7) => {
                    let bitfield = PIN * 4;
                    let mask: u32 = 0xF << bitfield;
                    let mode: u32 = 0x4;
                    let mut value: u32 = 0;
                    gpio.porta_model().modify(|r, w| {
                        value = (r.bits() & !mask) | (mode << bitfield);
                        w.bits(value)
                    });
                    if gpio.porta_model().read().bits() != value {
                        return Err(GpioError::FailedToConfiguareGpio);
                    }
                }
                ('A', 8..=15) => {
                    let bitfield = (PIN - 8) * 4;
                    let mask: u32 = 0xF << bitfield;
                    let mode: u32 = 0x4;
                    let mut value: u32 = 0;
                    gpio.porta_modeh().modify(|r, w| {
                        value = (r.bits() & !mask) | (mode << bitfield);
                        w.bits(value)
                    });
                    if gpio.porta_modeh().read().bits() != value {
                        return Err(GpioError::FailedToConfiguareGpio);
                    }
                }
                ('B', 0..=7) => {
                    let bitfield = PIN * 4;
                    let mask: u32 = 0xF << bitfield;
                    let mode: u32 = 0x4;
                    let mut value: u32 = 0;
                    gpio.portb_model().modify(|r, w| {
                        value = (r.bits() & !mask) | (mode << bitfield);
                        w.bits(value)
                    });
                    if gpio.portb_model().read().bits() != value {
                        return Err(GpioError::FailedToConfiguareGpio);
                    }
                }
                ('C', 0..=7) => {
                    let bitfield = PIN * 4;
                    let mask: u32 = 0xF << bitfield;
                    let mode: u32 = 0x4;
                    let mut value: u32 = 0;
                    gpio.portc_model().modify(|r, w| {
                        value = (r.bits() & !mask) | (mode << bitfield);
                        w.bits(value)
                    });
                    if gpio.portc_model().read().bits() != value {
                        return Err(GpioError::FailedToConfiguareGpio);
                    }
                }
                ('C', 8..=15) => {
                    let bitfield = (PIN - 8) * 4;
                    let mask: u32 = 0xF << bitfield;
                    let mode: u32 = 0x4;
                    let mut value: u32 = 0;
                    gpio.portc_modeh().modify(|r, w| {
                        value = (r.bits() & !mask) | (mode << bitfield);
                        w.bits(value)
                    });
                    if gpio.portc_modeh().read().bits() != value {
                        return Err(GpioError::FailedToConfiguareGpio);
                    }
                }
                ('D', 0..=7) => {
                    let bitfield = PIN * 4;
                    let mask: u32 = 0xF << bitfield;
                    let mode: u32 = 0x4;
                    let mut value: u32 = 0;
                    gpio.portd_model().modify(|r, w| {
                        value = (r.bits() & !mask) | (mode << bitfield);
                        w.bits(value)
                    });
                    if gpio.portd_model().read().bits() != value {
                        return Err(GpioError::FailedToConfiguareGpio);
                    }
                }

                _ => panic!("Invalid port/pin"),
            };
        }
        Ok(Self {
            _pin: pin.into_mode::<Output>(),
        })
    }

    pub fn write_high(&mut self) -> Result<(), GpioError> {
        unsafe {
            let gpio = &*GpioS::ptr();
            match PORT {
                'A' => {
                    gpio.porta_dout()
                        .modify(|r, w| w.bits(r.bits() | (1 << PIN)));
                    let value = 1 << PIN;
                    if gpio.porta_dout().read().bits() != value {
                        return Err(GpioError::FailedToSetGpio);
                    }
                }
                'B' => {
                    gpio.portb_dout()
                        .modify(|r, w| w.bits(r.bits() | (1 << PIN)));
                    let value = 1 << PIN;
                    if gpio.portb_dout().read().bits() != value {
                        return Err(GpioError::FailedToSetGpio);
                    }
                }
                'C' => {
                    gpio.portc_dout()
                        .modify(|r, w| w.bits(r.bits() | (1 << PIN)));
                    let value = 1 << PIN;
                    if gpio.portc_dout().read().bits() != value {
                        return Err(GpioError::FailedToSetGpio);
                    }
                }
                'D' => {
                    gpio.portd_dout()
                        .modify(|r, w| w.bits(r.bits() | (1 << PIN)));
                    let value = 1 << PIN;
                    if gpio.portd_dout().read().bits() != value {
                        return Err(GpioError::FailedToSetGpio);
                    }
                }
                _ => panic!("Failed"),
            }
        }
        Ok(())
    }

    pub fn write_low(&mut self) -> Result<(), GpioError> {
        unsafe {
            let gpio = &*GpioS::ptr();
            match PORT {
                'A' => {
                    gpio.porta_dout()
                        .modify(|r, w| w.bits(r.bits() & !(1 << PIN)));
                    if (gpio.porta_dout().read().bits() & (1 << PIN)) != 0 {
                        return Err(GpioError::FailedToSetGpio);
                    }
                }
                'B' => {
                    gpio.portb_dout()
                        .modify(|r, w| w.bits(r.bits() & !(1 << PIN)));
                    if (gpio.portb_dout().read().bits() & (1 << PIN)) != 0 {
                        return Err(GpioError::FailedToSetGpio);
                    }
                }
                'C' => {
                    gpio.portc_dout()
                        .modify(|r, w| w.bits(r.bits() & !(1 << PIN)));
                    if (gpio.portc_dout().read().bits() & (1 << PIN)) != 0 {
                        return Err(GpioError::FailedToSetGpio);
                    }
                }
                'D' => {
                    gpio.portd_dout()
                        .modify(|r, w| w.bits(r.bits() & !(1 << PIN)));
                    if (gpio.portd_dout().read().bits() & (1 << PIN)) != 0 {
                        return Err(GpioError::FailedToSetGpio);
                    }
                }
                _ => panic!("Failed"),
            }
        }
        Ok(())
    }

    pub fn write_toggle(&mut self) -> Result<(), GpioError> {
        unsafe {
            let gpio = &*GpioS::ptr();
            match PORT {
                'A' => {
                    gpio.porta_dout()
                        .modify(|r, w| w.bits(r.bits() ^ (1 << PIN)));
                    if (gpio.porta_dout().read().bits() & (1 << PIN)) != 0 {
                        return Err(GpioError::FailedToSetGpio);
                    }
                }
                'B' => {
                    gpio.portb_dout()
                        .modify(|r, w| w.bits(r.bits() ^ (1 << PIN)));
                    if (gpio.portb_dout().read().bits() & (1 << PIN)) != 0 {
                        return Err(GpioError::FailedToSetGpio);
                    }
                }
                'C' => {
                    gpio.portc_dout()
                        .modify(|r, w| w.bits(r.bits() ^ (1 << PIN)));
                    if (gpio.portc_dout().read().bits() & (1 << PIN)) != 0 {
                        return Err(GpioError::FailedToSetGpio);
                    }
                }
                'D' => {
                    gpio.portd_dout()
                        .modify(|r, w| w.bits(r.bits() ^ (1 << PIN)));
                    if (gpio.portd_dout().read().bits() & (1 << PIN)) != 0 {
                        return Err(GpioError::FailedToSetGpio);
                    }
                }
                _ => panic!("Failed"),
            }
        }
        Ok(())
    }

    pub fn write_level(&mut self, level: Level) -> Result<(), GpioError> {
        if level == Level::High {
            self.write_high()
        } else {
            self.write_low()
        }
    }

    pub fn is_set_high(&mut self) -> bool {
        unsafe {
            let gpio = &*GpioS::ptr();
            let dout = match PORT {
                'A' => gpio.porta_dout().read().bits(),
                'B' => gpio.portb_dout().read().bits(),
                'C' => gpio.portc_dout().read().bits(),
                'D' => gpio.portd_dout().read().bits(),
                _ => panic!("Failed"),
            };
            ((dout >> PIN) & 1) != 0
        }
    }

    pub fn is_set_low(&mut self) -> bool {
        unsafe {
            let gpio = &*GpioS::ptr();
            let dout = match PORT {
                'A' => gpio.porta_dout().read().bits(),
                'B' => gpio.portb_dout().read().bits(),
                'C' => gpio.portc_dout().read().bits(),
                'D' => gpio.portd_dout().read().bits(),
                _ => panic!("Failed"),
            };
            ((dout >> PIN) & 1) == 0
        }
    }
}

pub struct InputConfig {
    pub pull: Pull,
}

impl Default for InputConfig {
    fn default() -> Self {
        InputConfig {
            pull: Pull::Floating,
        }
    }
}

impl<const PORT: char, const PIN: u8> Gpio<PORT, PIN, Input> {
    pub fn input(pin: Pin<PORT, PIN, Unknown>, config: InputConfig) -> Result<Self, GpioError> {
        let (mode, out) = match config.pull {
            Pull::Floating => (0x1, !1),
            Pull::Up => (0x2, 1),
            Pull::Down => (0x2, !1),
        };

        unsafe {
            let gpio = &*GpioS::ptr();
            match (PORT, PIN) {
                ('A', 0..=7) => {
                    gpio.porta_dout()
                        .modify(|r, w| w.bits(r.bits() | out << PIN));
                    if gpio.porta_dout().read().bits() != (out << PIN) {
                        return Err(GpioError::FailedToSetPull);
                    }
                    let bitfield = PIN * 4;
                    let mask = 0xF << bitfield;
                    let mut value: u32 = 0;
                    gpio.porta_model().modify(|r, w| {
                        value = r.bits() & !mask | mode << bitfield;
                        w.bits(value)
                    });
                    if gpio.porta_model().read().bits() != value {
                        return Err(GpioError::FailedToSetGpio);
                    }
                }
                ('A', 8..=15) => {
                    gpio.porta_dout()
                        .modify(|r, w| w.bits(r.bits() | out << PIN));
                    if gpio.porta_dout().read().bits() != (out << PIN) {
                        return Err(GpioError::FailedToSetPull);
                    }
                    let bitfield = (PIN - 8) * 4;
                    let mask = 0xF << bitfield;
                    let mut value: u32 = 0;
                    gpio.porta_modeh().modify(|r, w| {
                        value = r.bits() & !mask | mode << bitfield;
                        w.bits(value)
                    });
                    if gpio.porta_modeh().read().bits() != value {
                        return Err(GpioError::FailedToSetGpio);
                    }
                }
                ('B', 0..=7) => {
                    gpio.portb_dout()
                        .modify(|r, w| w.bits(r.bits() | out << PIN));
                    if gpio.portb_dout().read().bits() != (out << PIN) {
                        return Err(GpioError::FailedToSetPull);
                    }
                    let bitfield = PIN * 4;
                    let mask = 0xF << bitfield;
                    let mut value: u32 = 0;
                    gpio.portb_model().modify(|r, w| {
                        value = r.bits() & !mask | mode << PIN;
                        w.bits(value)
                    });
                    if gpio.portb_model().read().bits() != value {
                        return Err(GpioError::FailedToSetGpio);
                    }
                }
                ('C', 0..=7) => {
                    gpio.portc_dout()
                        .modify(|r, w| w.bits(r.bits() | out << PIN));
                    if gpio.portc_dout().read().bits() != (out << PIN) {
                        return Err(GpioError::FailedToSetPull);
                    }
                    let bitfield = PIN * 4;
                    let mask = 0xF << bitfield;
                    let mut value: u32 = 0;
                    gpio.portc_model().modify(|r, w| {
                        value = r.bits() & !mask | mode << PIN;
                        w.bits(value)
                    });
                    if gpio.portc_model().read().bits() != value {
                        return Err(GpioError::FailedToSetGpio);
                    }
                }
                ('D', 0..=7) => {
                    gpio.portd_dout()
                        .modify(|r, w| w.bits(r.bits() | out << PIN));
                    if gpio.portd_dout().read().bits() != (out << PIN) {
                        return Err(GpioError::FailedToSetPull);
                    }
                    let bitfield = PIN * 4;
                    let mask = 0xF << bitfield;
                    let mut value: u32 = 0;
                    gpio.portd_model().modify(|r, w| {
                        value = r.bits() & !mask | mode << PIN;
                        w.bits(value)
                    });
                    if gpio.portd_model().read().bits() != value {
                        return Err(GpioError::FailedToSetGpio);
                    }
                }
                _ => {
                    panic!("Failed");
                }
            };
        }

        Ok(Self {
            _pin: pin.into_mode::<Input>(),
        })
    }

    pub fn read(&mut self) -> Level {
        unsafe {
            let gpio = &*GpioS::ptr();
            let din = match PORT {
                'A' => gpio.porta_din().read().bits(),
                'B' => gpio.portb_din().read().bits(),
                'C' => gpio.portc_din().read().bits(),
                'D' => gpio.portd_din().read().bits(),
                _ => panic!("Failed"),
            };
            Level::from(((din >> PIN) & 1) != 0)
        }
    }
}

impl<const PORT: char, const PIN: u8> ErrorType for Gpio<PORT, PIN, Output> {
    type Error = GpioError;
}

impl<const PORT: char, const PIN: u8> ErrorType for Gpio<PORT, PIN, Input> {
    type Error = GpioError;
}

impl<const PORT: char, const PIN: u8> EhOutput for Gpio<PORT, PIN, Output> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.write_high()
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.write_low()
    }
}

impl<const PORT: char, const PIN: u8> EhInput for Gpio<PORT, PIN, Input> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        if self.read() == Level::High {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        if self.read() == Level::High {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}

impl<const PORT: char, const PIN: u8> StatefulOutputPin for Gpio<PORT, PIN, Output> {
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.write_toggle()
    }
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.is_set_high())
    }
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.is_set_low())
    }
}
 