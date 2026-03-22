unsafe extern "C" {
    pub fn cmu_wrap_enable_clock(clock: u32, enable: u32);
    pub fn cmu_wrap_clock_gpio() -> u32;
    pub fn cmu_wrap_enable_gpio();
    pub fn cmu_wrap_core_clock_hz() -> u32;
    pub fn cmu_wrap_init_78mhz() -> u32;
}
