use cortex_m::Peripherals;
use cortex_m::delay::Delay;

static CPU_HZ: u32 = 20_000_000;

pub fn init_delay() -> Delay {
    let cp = Peripherals::take().unwrap();
    Delay::new(cp.SYST, CPU_HZ)
}
