//! A GPIO HAL for the EFR32MG24 (Cortex-M33), built directly on
//! [`efr32mg24_pac`].
//!
//! The crate is self-contained: it brings its own runtime ([`rt`]), linker
//! script, panic handler ([`panic`]) and SysTick delays ([`delay`]), so an
//! application needs nothing beyond this crate and a linker flag.
//!
//! The [`gpio`] API follows `esp-hal`'s shape: [`gpio::Input`],
//! [`gpio::Output`] and [`gpio::Flex`], built from `with_*` config builders,
//! with infallible operations.
//!
//! ```ignore
//! #![no_std]
//! #![no_main]
//!
//! use mg24_hal::{
//!     CpuConfig,
//!     delay::Delay,
//!     gpio::{Level, Output, OutputConfig},
//! };
//!
//! #[mg24_hal::main]
//! fn main() -> ! {
//!     let dp = mg24_hal::init(CpuConfig::default()).unwrap();
//!     let mut led = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());
//!     let delay = Delay::new();
//!
//!     loop {
//!         led.toggle();
//!         delay.delay_ms(500);
//!     }
//! }
//! ```

#![no_std]

use core::sync::atomic::{AtomicBool, Ordering};
use efr32mg24_pac::CmuS;
use pins::GpioPin;

pub mod clock;
pub mod delay;
pub mod gpio;
pub mod i2c;
pub mod interrupt;
pub mod pins;
pub mod rt;
pub mod rtt;
pub mod timestamp;

#[cfg(feature = "panic-handler")]
pub mod panic;

pub use efr32mg24_pac as pac;

/// Marks the application entry point. See [`rt`] for what happens before it
/// runs.
///
/// ```ignore
/// #[mg24_hal::main]
/// fn main() -> ! {
///     loop {}
/// }
/// ```
pub use mg24_hal_macros::main;

/// Marks an interrupt or exception handler. See [`rt`] for the vector table.
///
/// ```ignore
/// #[mg24_hal::interrupt]
/// fn GPIO_ODD() {
///     // ...
/// }
/// ```
pub use mg24_hal_macros::interrupt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeripheralsErrors {
    AlreadyTaken,
    /// The requested CPU speed could not be applied. The clock is left as it
    /// was, so the chip keeps running.
    Clock(clock::ClockError),
}

static TAKEN: AtomicBool = AtomicBool::new(false);

/// What [`init`] should set up before handing over the peripherals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuConfig {
    /// Enables the GPIO bus clock. Without it every GPIO register reads back
    /// as zero and writes are dropped.
    pub gpio_clock: bool,

    /// Enables I2C0 and I2C1 clocks.
    pub i2c_clock: bool,

    /// Retunes SYSCLK to this speed. `None` leaves the reset clock alone,
    /// which is HFRCODPLL's 19 MHz default band.
    pub cpu_speed: Option<clock::CpuSpeed>,
}

impl CpuConfig {
    /// GPIO clock on, CPU speed left at the reset default.
    pub const fn new() -> Self {
        Self {
            gpio_clock: true,
            i2c_clock: false,
            cpu_speed: None,
        }
    }

    /// Sets the CPU speed [`init`] should apply.
    pub const fn with_cpu_speed(mut self, speed: clock::CpuSpeed) -> Self {
        self.cpu_speed = Some(speed);
        self
    }

    /// Sets whether [`init`] enables the GPIO bus clock.
    pub const fn with_gpio_clock(mut self, enable: bool) -> Self {
        self.gpio_clock = enable;
        self
    }

    /// Sets whether [`init`] enables the I2C clocks.
    pub const fn with_i2c_clock(mut self, enable: bool) -> Self {
        self.i2c_clock = enable;
        self
    }
}

impl Default for CpuConfig {
    fn default() -> Self {
        Self {
            gpio_clock: true,
            i2c_clock: false,
            cpu_speed: None,
        }
    }
}

/// The general-purpose pins.
///
/// # Board-specific pins
///
/// These are the chip's pins, and the HAL cannot know what your board wired
/// them to. On the XIAO MG24 several are spoken for — `pb4`/`pb5` drive the RF
/// antenna switch, `pa6` is the internal flash chip select, and `pd3`/`pd4`
/// are battery control and sense. Driving those will misbehave in ways that
/// have nothing to do with this crate. See `PINS.md`.
///
/// The two SWD pins are not here at all; see [`DebugPins`].
pub struct Pins {
    pub pc0: GpioPin<'C', 0>,
    pub pc1: GpioPin<'C', 1>,
    pub pc2: GpioPin<'C', 2>,
    pub pc3: GpioPin<'C', 3>,
    pub pc4: GpioPin<'C', 4>,
    pub pc5: GpioPin<'C', 5>,
    pub pc6: GpioPin<'C', 6>,
    pub pc7: GpioPin<'C', 7>,
    pub pc8: GpioPin<'C', 8>,
    pub pc9: GpioPin<'C', 9>,
    pub pb0: GpioPin<'B', 0>,
    pub pb1: GpioPin<'B', 1>,
    pub pb2: GpioPin<'B', 2>,
    pub pb3: GpioPin<'B', 3>,
    pub pb4: GpioPin<'B', 4>,
    pub pb5: GpioPin<'B', 5>,
    pub pa0: GpioPin<'A', 0>,
    pub pa3: GpioPin<'A', 3>,
    pub pa4: GpioPin<'A', 4>,
    pub pa5: GpioPin<'A', 5>,
    pub pa6: GpioPin<'A', 6>,
    pub pa7: GpioPin<'A', 7>,
    pub pa8: GpioPin<'A', 8>,
    pub pa9: GpioPin<'A', 9>,
    pub pd0: GpioPin<'D', 0>,
    pub pd1: GpioPin<'D', 1>,
    pub pd2: GpioPin<'D', 2>,
    pub pd3: GpioPin<'D', 3>,
    pub pd4: GpioPin<'D', 4>,
    pub pd5: GpioPin<'D', 5>,
}

/// The serial wire debug pins.
///
/// `PA01` is SWCLK and `PA02` is SWDIO (datasheet, alternate function table).
/// They come out of reset enabled, with a pull-down and pull-up respectively,
/// and they are how a debugger reaches the chip.
///
/// Reconfiguring them as ordinary GPIO fights the debug port for as long as
/// the program runs. The damage is not permanent — `GPIO_DBGROUTEPEN` cannot
/// be changed at all while a debugger is attached, and RM 23.3.9.3 says a
/// reset returns the debug pins to their enabled default state — but you will
/// lose the debug connection until then, which can make a board look bricked.
///
/// They are kept out of [`Pins`] so that using them has to be deliberate.
pub struct DebugPins {
    /// `PA01` — SWCLK.
    pub pa1: GpioPin<'A', 1>,
    /// `PA02` — SWDIO.
    pub pa2: GpioPin<'A', 2>,
}

pub struct Peripherals {
    pub pins: Pins,
    debug_pins: Option<DebugPins>,
}

impl Peripherals {
    /// Takes the SWD pins, so they can be used as ordinary GPIO.
    ///
    /// Returns `None` if they have already been taken.
    ///
    /// # Safety
    ///
    /// Reconfiguring SWCLK/SWDIO drops the debug connection until the next
    /// reset, which means no flashing and no debugging in the meantime. Read
    /// [`DebugPins`] before calling this, and leave yourself a way back — RM
    /// 23.3.9.3 suggests a delay at the start of `main` so a debugger can
    /// attach before the pins are repurposed.
    pub unsafe fn take_debug_pins(&mut self) -> Option<DebugPins> {
        self.debug_pins.take()
    }
}

/// Brings up the chip and hands over the peripherals.
///
/// Returns [`PeripheralsErrors::AlreadyTaken`] on the second and later calls,
/// so the pin tokens stay unique.
pub fn init(config: CpuConfig) -> Result<Peripherals, PeripheralsErrors> {
    if TAKEN.swap(true, Ordering::AcqRel) {
        return Err(PeripheralsErrors::AlreadyTaken);
    }

    // Clock first: everything timed afterwards should see the final SYSCLK.
    if let Some(speed) = config.cpu_speed {
        clock::set_cpu_speed(speed).map_err(PeripheralsErrors::Clock)?;
    }

    if config.gpio_clock {
        // SAFETY: we hold the one-shot token above, so nothing else is
        // touching CMU yet.
        unsafe {
            let cmu = &*CmuS::ptr();
            cmu.clken0().modify(|_, w| w.gpio().set_bit());
        }
    }

    if config.i2c_clock {
        // SAFETY: we hold the one-shot token above, so nothing else is
        // touching CMU yet.
        unsafe {
            let cmu = &*CmuS::ptr();
            cmu.clken0().modify(|_, w| {
                w.i2c0().set_bit()
                    .i2c1().set_bit()
            });
        }
    }

    Ok(Peripherals {
        pins: Pins {
            pc0: GpioPin::new(),
            pc1: GpioPin::new(),
            pc2: GpioPin::new(),
            pc3: GpioPin::new(),
            pc4: GpioPin::new(),
            pc5: GpioPin::new(),
            pc6: GpioPin::new(),
            pc7: GpioPin::new(),
            pc8: GpioPin::new(),
            pc9: GpioPin::new(),
            pb0: GpioPin::new(),
            pb1: GpioPin::new(),
            pb2: GpioPin::new(),
            pb3: GpioPin::new(),
            pb4: GpioPin::new(),
            pb5: GpioPin::new(),
            pa0: GpioPin::new(),
            pa3: GpioPin::new(),
            pa4: GpioPin::new(),
            pa5: GpioPin::new(),
            pa6: GpioPin::new(),
            pa7: GpioPin::new(),
            pa8: GpioPin::new(),
            pa9: GpioPin::new(),
            pd0: GpioPin::new(),
            pd1: GpioPin::new(),
            pd2: GpioPin::new(),
            pd3: GpioPin::new(),
            pd4: GpioPin::new(),
            pd5: GpioPin::new(),
        },
        debug_pins: Some(DebugPins {
            pa1: GpioPin::new(),
            pa2: GpioPin::new(),
        }),
    })
}
