use crate::ffi::pwm::*;

fn port_from_char(port: char) -> u32 {
    match port {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        _ => panic!("Invalid port"),
    }
}

pub enum PreScale {
    Div1 = 0,
    Div2 = 1,
    Div4 = 2,
    Div8 = 3,
    Div16 = 4,
    Div32 = 5,
    Div64 = 6,
    Div128 = 7,
    Div256 = 8,
    Div512 = 9,
    Div1024 = 10,
}

pub struct PwmConfig {
    pub prescale: PreScale,
    pub frequencyhz: u32,
    pub dutyprestart: u32,
    pub invertoutput: bool,
}

pub struct Pwm {
    _timer_number: u32,
    _channel: u32,
}

impl Pwm {
    pub fn new(gpio: (char, u8), timer_number: u32, channel: u32, config: PwmConfig) -> Self {
        if timer_number > 4 {
            panic!("Invalid Timer Number!!");
        }
        if channel > 4 {
            panic!("Invalid Channel Number!!");
        }
        if config.frequencyhz == 0 {
            panic!("Must Put a Proper Frequency Hz Number");
        }
        let value = if config.dutyprestart > 4095 {
            4095
        } else {
            config.dutyprestart
        };
        unsafe {
            PWM_XiaoMg24Configure(
                timer_number as u8,
                channel as u8,
                config.prescale as u32,
                config.frequencyhz,
                value as u16,
                port_from_char(gpio.0),
                gpio.1,
                config.invertoutput,
                true,
            );
        }
        return Self {
            _timer_number: timer_number,
            _channel: channel,
        };
    }

    pub fn write(&self, duty: u16) {
        let value = if duty > 4095 { 4095 } else { duty };
        unsafe {
            PWM_XiaoMg24WriteDuty(self._timer_number as u8, self._channel as u8, value);
        }
    }
}
