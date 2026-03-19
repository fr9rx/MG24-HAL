unsafe extern "C" {
    pub fn gpio_wrap_pin_cfg(port: u32, pin: u32, mode: u32, out: u32);
    pub fn gpio_wrap_pin_high(port: u32, pin: u32);
    pub fn gpio_wrap_pin_low(port: u32, pin: u32);
    pub fn gpio_wrap_pin_toggle(port: u32, pin: u32);
}
