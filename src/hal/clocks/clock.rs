use crate::ffi::cmu;

pub struct Clock;

impl Clock {
    pub fn start() {
        unsafe {
            cmu::cmu_wrap_enable_clock(cmu::cmu_wrap_clock_hfper(), 1);
        }
    }
}
