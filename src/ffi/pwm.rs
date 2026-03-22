unsafe extern "C" {
    pub fn PWM_XiaoMg24Configure(
        timerNumber: u8,
        channel: u8,
        prescale: u32,
        frequencyHz: u32,
        dutyPermille: u16,
        port: u32,
        pin: u8,
        invertOutput: bool,
        enableNow: bool,
    ) -> bool;
    pub fn PWM_XiaoMg24WriteDuty(timerNumber: u8, channel: u8, dutyPermille: u16) -> bool;
}
