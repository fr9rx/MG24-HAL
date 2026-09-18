//! GPIO driver, shaped after the `esp-hal` API.
//!
//! Three pin types, layered the way `esp-hal` layers them: [`Flex`] carries
//! every operation, and [`Input`] / [`Output`] are thin typed wrappers over it.
//! Like `esp-hal`, they are parameterised only by a lifetime — the port and pin
//! are erased into an [`AnyPin`] when the driver takes the pin, so
//! `Input<'static>` is a type you can name in a `static`.
//!
//! ```ignore
//! use mg24_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
//!
//! let mut led = Output::new(dp.pins.pa7, Level::Low, OutputConfig::default());
//! let button = Input::new(dp.pins.pc3, InputConfig::default().with_pull(Pull::Up));
//!
//! led.set_level(button.level());
//! ```
//!
//! # Differences from `esp-hal`
//!
//! - Operations are infallible, as in `esp-hal`. A pin that does not exist is
//!   rejected at compile time, so there is no error left to return.
//! - `esp-hal`'s `DriveStrength` has no equivalent here: the EFR32MG24 has no
//!   per-pin drive strength. The closest control is [`SlewRate`], which is
//!   **per port**, not per pin.
//! - The MG24 also offers open-source (wired-or) outputs, which `esp-hal` has
//!   no name for. They live under [`DriveMode::OpenSource`].
//!
//! Register details follow the EFR32xG24 Reference Manual, chapter 23.

use core::marker::PhantomData;
use core::fmt;

use crate::interrupt;
use crate::pins::GpioPin;
use embedded_hal::digital::{
    ErrorType, InputPin as EhInput, OutputPin as EhOutput, StatefulOutputPin,
};

/// GPIO operation errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioError {
    /// Invalid pin configuration
    InvalidConfig,
    /// Pin is not an input
    NotInput,
    /// Pin is not an output
    NotOutput,
    /// Conflicting configuration
    ConflictingConfig,
}

impl fmt::Display for GpioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GpioError::InvalidConfig => write!(f, "Invalid GPIO configuration"),
            GpioError::NotInput => write!(f, "Pin is not configured as input"),
            GpioError::NotOutput => write!(f, "Pin is not configured as output"),
            GpioError::ConflictingConfig => write!(f, "Conflicting GPIO configuration"),
        }
    }
}

pub type GpioResult<T> = Result<T, GpioError>;

// --------------------------------------------------------------------------
// Raw addresses for the bit-access aliases.
//
// RM 4.2.4.3: every register with bit-access has _SET at TARGET + 0x1000,
// _CLR at + 0x2000 and _TGL at + 0x3000. Writing a 1 acts on that bit, a 0
// leaves it alone, and the whole thing is one bus access with no
// read-modify-write. That is what makes the DOUT operations below atomic
// against interrupt handlers without masking anything.
// --------------------------------------------------------------------------

const GPIO_S_BASE: usize = 0x4003_C000;
const ALIAS_SET: usize = 0x1000;
const ALIAS_CLR: usize = 0x2000;
const ALIAS_TGL: usize = 0x3000;

/// RM 23.5: GPIO_IF offset.
const IF_OFFSET: usize = 0x420;

// --------------------------------------------------------------------------
// MODE field encodings, RM 23.6.3.
// --------------------------------------------------------------------------

const MODE_DISABLED: u32 = 0;
const MODE_INPUT: u32 = 1;
const MODE_INPUT_PULL: u32 = 2;
const MODE_INPUT_PULL_FILTER: u32 = 3;
const MODE_PUSH_PULL: u32 = 4;
const MODE_WIRED_OR: u32 = 6;
const MODE_WIRED_OR_PULL_DOWN: u32 = 7;
const MODE_WIRED_AND: u32 = 8;
const MODE_WIRED_AND_FILTER: u32 = 9;
const MODE_WIRED_AND_PULL_UP: u32 = 10;
const MODE_WIRED_AND_PULL_UP_FILTER: u32 = 11;

// --------------------------------------------------------------------------
// Public configuration types.
// --------------------------------------------------------------------------

/// A digital level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Low,
    High,
}

impl From<bool> for Level {
    fn from(value: bool) -> Self {
        if value { Level::High } else { Level::Low }
    }
}

impl From<Level> for bool {
    fn from(value: Level) -> Self {
        value == Level::High
    }
}

impl core::ops::Not for Level {
    type Output = Level;

    fn not(self) -> Level {
        match self {
            Level::Low => Level::High,
            Level::High => Level::Low,
        }
    }
}

/// Internal pull resistor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Pull {
    #[default]
    None,
    Up,
    Down,
}

/// How an output drives the pad.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DriveMode {
    /// Drives both rails.
    #[default]
    PushPull,
    /// Drives low only, releases high. Needs a pull-up to reach a high level.
    OpenDrain,
    /// Drives high only, releases low. The MG24 calls this wired-or; `esp-hal`
    /// has no equivalent.
    OpenSource,
}

/// Output slew rate limit, RM 23.6.2 (`GPIO_PORTn_CTRL.SLEWRATE`).
///
/// Higher is faster. The slowest setting also limits drive current to roughly
/// 1 mA.
///
/// # This is per port, not per pin
///
/// `SLEWRATE` lives in `GPIO_PORTn_CTRL`, so setting it through one pin's
/// [`OutputConfig`] changes it for **every pin on that port**. This is the
/// closest analogue to `esp-hal`'s per-pin `DriveStrength`, but it is not the
/// same thing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlewRate(u8);

impl Default for SlewRate {
    /// The reset value, not the slowest setting.
    fn default() -> Self {
        SlewRate::DEFAULT
    }
}

impl SlewRate {
    /// The reset value, 4.
    pub const DEFAULT: SlewRate = SlewRate(4);

    /// Slowest edges, and a drive current limited to about 1 mA.
    pub const SLOWEST: SlewRate = SlewRate(0);

    /// Fastest edges.
    pub const FASTEST: SlewRate = SlewRate(7);

    /// Builds a slew rate from a raw 0..=7 setting.
    ///
    /// Values above 7 saturate rather than corrupting the neighbouring fields.
    pub const fn new(value: u8) -> Self {
        SlewRate(if value > 7 { 7 } else { value })
    }

    /// The raw 0..=7 setting.
    pub const fn get(self) -> u8 {
        self.0
    }
}

/// Which edge an [`Input`] or [`Flex`] reports.
///
/// The MG24's external interrupts are edge sensitive only; `esp-hal`'s
/// `LowLevel` and `HighLevel` have no hardware equivalent here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    RisingEdge,
    FallingEdge,
    AnyEdge,
}

/// Input pin configuration.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InputConfig {
    pull: Pull,
    filter: bool,
}

impl InputConfig {
    /// Floating, no glitch filter.
    pub const fn new() -> Self {
        InputConfig {
            pull: Pull::None,
            filter: false,
        }
    }

    /// Sets the internal pull resistor.
    pub const fn with_pull(mut self, pull: Pull) -> Self {
        self.pull = pull;
        self
    }

    /// Enables the glitch filter.
    ///
    /// RM 23.3.10.1 recommends this for any pin driving an external interrupt,
    /// since the edge detection is asynchronous and therefore noise sensitive.
    pub const fn with_filter(mut self, filter: bool) -> Self {
        self.filter = filter;
        self
    }

    /// The configured pull.
    pub const fn pull(&self) -> Pull {
        self.pull
    }

    /// Whether the glitch filter is on.
    pub const fn filter(&self) -> bool {
        self.filter
    }

    /// The MODE encoding, and what DOUT has to be for it.
    const fn mode_and_dout(&self) -> (u32, bool) {
        match (self.pull, self.filter) {
            // MODE_INPUT reads DOUT as "enable the filter".
            (Pull::None, false) => (MODE_INPUT, false),
            (Pull::None, true) => (MODE_INPUT, true),
            // The INPUTPULL modes read DOUT as the pull direction.
            (Pull::Up, false) => (MODE_INPUT_PULL, true),
            (Pull::Down, false) => (MODE_INPUT_PULL, false),
            (Pull::Up, true) => (MODE_INPUT_PULL_FILTER, true),
            (Pull::Down, true) => (MODE_INPUT_PULL_FILTER, false),
        }
    }
}

/// Output pin configuration.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OutputConfig {
    drive_mode: DriveMode,
    pull: Pull,
    slew_rate: Option<SlewRate>,
    filter: bool,
}

impl OutputConfig {
    /// Push-pull, no pull resistor, port slew rate left alone.
    pub const fn new() -> Self {
        OutputConfig {
            drive_mode: DriveMode::PushPull,
            pull: Pull::None,
            slew_rate: None,
            filter: false,
        }
    }

    /// Sets the drive mode.
    pub const fn with_drive_mode(mut self, drive_mode: DriveMode) -> Self {
        self.drive_mode = drive_mode;
        self
    }

    /// Sets the pull resistor.
    ///
    /// Only meaningful for the modes that release a rail: [`Pull::Up`] with
    /// [`DriveMode::OpenDrain`], and [`Pull::Down`] with
    /// [`DriveMode::OpenSource`]. A push-pull output drives both rails, so the
    /// hardware has no pull setting for it and this is ignored.
    pub const fn with_pull(mut self, pull: Pull) -> Self {
        self.pull = pull;
        self
    }

    /// Sets the port's slew rate. See [`SlewRate`] — this affects every pin on
    /// the port, not just this one. Leaving it unset keeps the port as it is.
    pub const fn with_slew_rate(mut self, slew_rate: SlewRate) -> Self {
        self.slew_rate = Some(slew_rate);
        self
    }

    /// Enables the glitch filter on the open-drain modes that support it.
    pub const fn with_filter(mut self, filter: bool) -> Self {
        self.filter = filter;
        self
    }

    /// The configured drive mode.
    pub const fn drive_mode(&self) -> DriveMode {
        self.drive_mode
    }

    /// The configured pull.
    pub const fn pull(&self) -> Pull {
        self.pull
    }

    /// The configured slew rate, if this config sets one.
    pub const fn slew_rate(&self) -> Option<SlewRate> {
        self.slew_rate
    }

    /// Whether the glitch filter is on.
    pub const fn filter(&self) -> bool {
        self.filter
    }

    /// The MODE encoding for this combination.
    const fn mode(&self) -> u32 {
        match self.drive_mode {
            DriveMode::PushPull => MODE_PUSH_PULL,
            DriveMode::OpenSource => match self.pull {
                Pull::Down => MODE_WIRED_OR_PULL_DOWN,
                _ => MODE_WIRED_OR,
            },
            DriveMode::OpenDrain => match (self.pull, self.filter) {
                (Pull::Up, false) => MODE_WIRED_AND_PULL_UP,
                (Pull::Up, true) => MODE_WIRED_AND_PULL_UP_FILTER,
                (_, true) => MODE_WIRED_AND_FILTER,
                (_, false) => MODE_WIRED_AND,
            },
        }
    }
}

// --------------------------------------------------------------------------
// AnyPin: a pin with its identity erased, and the register access layer.
// --------------------------------------------------------------------------

/// A pin identified at runtime rather than in the type.
///
/// This is what the driver types hold, which is why they need only a lifetime.
/// Every register access in this module goes through here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnyPin {
    port: char,
    number: u8,
}

impl AnyPin {
    /// This pin's port letter.
    pub const fn port(self) -> char {
        self.port
    }

    /// This pin's number within its port.
    pub const fn number(self) -> u8 {
        self.number
    }

    /// This pin's bit mask within its port's registers.
    #[inline]
    const fn mask(self) -> u32 {
        1 << self.number
    }

    /// RM 23.5: GPIO_PORTn_DOUT offsets.
    #[inline]
    const fn dout_offset(self) -> usize {
        match self.port {
            'A' => 0x040,
            'B' => 0x070,
            'C' => 0x0A0,
            // Unreachable for any other port: `GpioPin::VALID` rejects them.
            _ => 0x0D0,
        }
    }

    /// Writes `mode` into this pin's 4-bit MODE field.
    ///
    /// MODEL covers pins 0..=7 and MODEH pins 8..=15, so a pin always owns the
    /// nibble at its position within that half. This is a genuine
    /// read-modify-write (the nibbles of seven other pins share the register),
    /// so it runs with interrupts masked.
    fn set_mode(self, mode: u32) {
        let shift = (self.number % 8) * 4;
        let mask: u32 = 0xF << shift;
        let value: u32 = mode << shift;

        interrupt::free(|| unsafe {
            let gpio = &*efr32mg24_pac::GpioS::ptr();

            // This PAC's `modify` returns the written word, so the trailing
            // semicolon is what keeps every match arm at `()`.
            macro_rules! set_nibble {
                ($reg:ident) => {{
                    gpio.$reg()
                        .modify(|r, w| w.bits((r.bits() & !mask) | value));
                }};
            }

            match (self.port, self.number) {
                ('A', 0..=7) => set_nibble!(porta_model),
                ('A', 8..=15) => set_nibble!(porta_modeh),
                ('B', 0..=7) => set_nibble!(portb_model),
                ('C', 0..=7) => set_nibble!(portc_model),
                ('C', 8..=15) => set_nibble!(portc_modeh),
                ('D', 0..=7) => set_nibble!(portd_model),
                // Unreachable: `GpioPin::VALID` rejects everything else.
                _ => {}
            }
        });
    }

    /// Sets the port's slew rate, RM 23.6.2.
    fn set_slew_rate(self, slew_rate: SlewRate) {
        let value = (slew_rate.get() as u32) << 4;
        let mask: u32 = 0b111 << 4;

        interrupt::free(|| unsafe {
            let gpio = &*efr32mg24_pac::GpioS::ptr();

            macro_rules! set_slew {
                ($reg:ident) => {{
                    gpio.$reg()
                        .modify(|r, w| w.bits((r.bits() & !mask) | value));
                }};
            }

            match self.port {
                'A' => set_slew!(porta_ctrl),
                'B' => set_slew!(portb_ctrl),
                'C' => set_slew!(portc_ctrl),
                'D' => set_slew!(portd_ctrl),
                _ => {}
            }
        });
    }

    /// One atomic write to a DOUT bit-access alias. No read-modify-write, so
    /// no masking needed.
    #[inline]
    fn write_dout_alias(self, alias: usize) {
        let address = GPIO_S_BASE + alias + self.dout_offset();
        // SAFETY: a fixed, valid peripheral address; the alias semantics mean
        // only this pin's bit is touched.
        unsafe { core::ptr::write_volatile(address as *mut u32, self.mask()) };
    }

    /// Reads DOUT: the level the pin is being driven to.
    fn read_dout(self) -> bool {
        unsafe {
            let gpio = &*efr32mg24_pac::GpioS::ptr();
            let bits = match self.port {
                'A' => gpio.porta_dout().read().bits(),
                'B' => gpio.portb_dout().read().bits(),
                'C' => gpio.portc_dout().read().bits(),
                _ => gpio.portd_dout().read().bits(),
            };
            bits & self.mask() != 0
        }
    }

    /// Reads DIN: the level actually present on the pad.
    fn read_din(self) -> bool {
        unsafe {
            let gpio = &*efr32mg24_pac::GpioS::ptr();
            let bits = match self.port {
                'A' => gpio.porta_din().read().bits(),
                'B' => gpio.portb_din().read().bits(),
                'C' => gpio.portc_din().read().bits(),
                _ => gpio.portd_din().read().bits(),
            };
            bits & self.mask() != 0
        }
    }

    /// RM 4.2.4.4.6: writing straight to an INTFLAG register does nothing.
    /// The flag can only be cleared through the `_CLR` alias.
    #[inline]
    fn clear_interrupt_bit(self) {
        let address = GPIO_S_BASE + ALIAS_CLR + IF_OFFSET;
        // SAFETY: a fixed, valid peripheral address.
        unsafe { core::ptr::write_volatile(address as *mut u32, self.mask()) };
    }
}

/// A pin that can be handed to a driver type.
///
/// `esp-hal` spells this the same way: a trait implemented by the concrete pin
/// types, with `degrade` erasing the identity into an [`AnyPin`].
pub trait Pin {
    /// Erases the port and pin from the type.
    fn degrade(self) -> AnyPin;
}

impl<const PORT: char, const PIN: u8> Pin for GpioPin<PORT, PIN> {
    fn degrade(self) -> AnyPin {
        // Forces the compile-time check that this pin exists. Every route into
        // the driver types passes through here.
        let () = Self::VALID;

        AnyPin {
            port: PORT,
            number: PIN,
        }
    }
}

impl Pin for AnyPin {
    fn degrade(self) -> AnyPin {
        self
    }
}

// --------------------------------------------------------------------------
// Flex: every operation lives here.
// --------------------------------------------------------------------------

/// A pin whose direction is decided at runtime.
///
/// Equivalent to `esp-hal`'s `Flex`. [`Input`] and [`Output`] are wrappers
/// around this.
pub struct Flex<'d> {
    pin: AnyPin,
    // The MG24 packs direction and drive mode into one MODE field, so unlike a
    // chip with separate output-enable and input-enable bits, the driver has to
    // remember both configurations and recompute MODE whenever either changes.
    input_config: InputConfig,
    output_config: OutputConfig,
    input_enabled: bool,
    output_enabled: bool,
    _lifetime: PhantomData<&'d mut ()>,
}

impl<'d> Flex<'d> {
    /// Takes the pin, leaving the hardware as it is.
    ///
    /// Nothing is driven or sensed until [`Flex::set_output_enable`] or
    /// [`Flex::set_input_enable`] turns a direction on.
    pub fn new(pin: impl Pin + 'd) -> Self {
        Flex {
            pin: pin.degrade(),
            input_config: InputConfig::new(),
            output_config: OutputConfig::new(),
            input_enabled: false,
            output_enabled: false,
            _lifetime: PhantomData,
        }
    }

    /// The pin this driver was built from.
    pub fn pin(&self) -> AnyPin {
        self.pin
    }

    /// Recomputes MODE from whichever direction is currently enabled.
    ///
    /// Output wins over input, because a pin driving the pad is also readable
    /// through DIN.
    fn update_mode(&mut self) {
        if self.output_enabled {
            self.pin.set_mode(self.output_config.mode());
        } else if self.input_enabled {
            let (mode, dout) = self.input_config.mode_and_dout();
            // Under the input modes DOUT selects the pull, so it has to be
            // right before the mode switch.
            self.pin
                .write_dout_alias(if dout { ALIAS_SET } else { ALIAS_CLR });
            self.pin.set_mode(mode);
        } else {
            self.pin.set_mode(MODE_DISABLED);
        }
    }

    // ---- configuration ----

    /// Applies an input configuration.
    ///
    /// Takes effect immediately if the pin is currently an input; otherwise it
    /// is remembered for the next [`Flex::set_input_enable`].
    ///
    /// # Note
    ///
    /// While the pin is an input, DOUT holds the pull direction rather than an
    /// output level, so switching to input loses whatever level an output was
    /// driving. That is the hardware's doing, not the driver's.
    pub fn apply_input_config(&mut self, config: &InputConfig) {
        self.input_config = *config;
        self.update_mode();
    }

    /// Applies an output configuration.
    ///
    /// Takes effect immediately if the pin is currently an output; otherwise it
    /// is remembered for the next [`Flex::set_output_enable`]. The slew rate,
    /// if the config sets one, is applied straight away — it belongs to the
    /// whole port, not this pin.
    pub fn apply_output_config(&mut self, config: &OutputConfig) {
        self.output_config = *config;
        if let Some(slew_rate) = config.slew_rate {
            self.pin.set_slew_rate(slew_rate);
        }
        self.update_mode();
    }

    /// Turns the input buffer on or off.
    pub fn set_input_enable(&mut self, enable_input: bool) {
        self.input_enabled = enable_input;
        self.update_mode();
    }

    /// Turns the output driver on or off.
    ///
    /// With both directions off the pin goes to MODE DISABLED: nothing driven,
    /// nothing sensed.
    pub fn set_output_enable(&mut self, enable_output: bool) {
        self.output_enabled = enable_output;
        self.update_mode();
    }

    // ---- output ----

    /// Drives the pin high.
    #[inline]
    pub fn set_high(&mut self) {
        self.pin.write_dout_alias(ALIAS_SET);
    }

    /// Drives the pin low.
    #[inline]
    pub fn set_low(&mut self) {
        self.pin.write_dout_alias(ALIAS_CLR);
    }

    /// Drives the pin to `level`.
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        match level {
            Level::High => self.set_high(),
            Level::Low => self.set_low(),
        }
    }

    /// Inverts the driven level.
    #[inline]
    pub fn toggle(&mut self) {
        self.pin.write_dout_alias(ALIAS_TGL);
    }

    /// Whether the pin is being driven high.
    pub fn is_set_high(&self) -> bool {
        self.pin.read_dout()
    }

    /// Whether the pin is being driven low.
    pub fn is_set_low(&self) -> bool {
        !self.pin.read_dout()
    }

    /// The level the pin is being driven to.
    pub fn output_level(&self) -> Level {
        Level::from(self.pin.read_dout())
    }

    // ---- input ----

    /// Whether the pad reads high.
    pub fn is_high(&self) -> bool {
        self.pin.read_din()
    }

    /// Whether the pad reads low.
    pub fn is_low(&self) -> bool {
        !self.pin.read_din()
    }

    /// The level on the pad.
    pub fn level(&self) -> Level {
        Level::from(self.pin.read_din())
    }

    // ---- interrupts ----
    //
    // RM 23.3.10.1: external interrupt n takes its pin from
    // EXTIPSEL[n] (port) and EXTIPINSEL[n] (offset within a group of four),
    // where the group base is 4 * int(n / 4). This driver uses n == pin number,
    // which always satisfies that and gives a stable pin-to-interrupt mapping.
    //
    // The consequence: interrupt n belongs to pin n, whichever port it is on.
    // Two pins with the same number cannot both listen at once.

    /// Starts reporting `event` on this pin.
    ///
    /// This enables the NVIC line too, so the handler fires as soon as the edge
    /// arrives — there is no separate step to forget.
    ///
    /// # Define the handler first
    ///
    /// Every vector defaults to `DefaultHandler`, which spins. Enabling an
    /// interrupt with no handler of your own therefore parks the core on the
    /// first edge. Write the handler before calling this:
    ///
    /// ```ignore
    /// #[mg24_hal::interrupt]
    /// fn GPIO_ODD() {
    ///     // clear the flag here, or it re-enters forever
    /// }
    /// ```
    ///
    /// Odd-numbered pins arrive on `GPIO_ODD`, even-numbered ones on
    /// `GPIO_EVEN`; [`Flex::interrupt`] reports the number.
    pub fn listen(&mut self, event: Event) {
        let n = self.pin.number;
        let port_index: u32 = match self.pin.port {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => 3,
        };
        // RM 23.3.10.1: offset = pin - base, and base = 4 * int(n / 4).
        let pin_offset = (n % 4) as u32;

        let (rise, fall) = match event {
            Event::RisingEdge => (true, false),
            Event::FallingEdge => (false, true),
            Event::AnyEdge => (true, true),
        };

        let pin = self.pin;

        interrupt::free(|| unsafe {
            let gpio = &*efr32mg24_pac::GpioS::ptr();

            // Both EXTIPSEL and EXTIPINSEL pack one 2-bit value every 4 bits.
            let shift = (n % 8) * 4;
            let field_mask: u32 = 0xF << shift;

            if n < 8 {
                gpio.extipsell()
                    .modify(|r, w| w.bits((r.bits() & !field_mask) | (port_index << shift)));
                gpio.extipinsell()
                    .modify(|r, w| w.bits((r.bits() & !field_mask) | (pin_offset << shift)));
            } else {
                gpio.extipselh()
                    .modify(|r, w| w.bits((r.bits() & !field_mask) | (port_index << shift)));
                gpio.extipinselh()
                    .modify(|r, w| w.bits((r.bits() & !field_mask) | (pin_offset << shift)));
            }

            let bit = 1u32 << n;

            gpio.extirise().modify(|r, w| {
                w.bits(if rise {
                    r.bits() | bit
                } else {
                    r.bits() & !bit
                })
            });
            gpio.extifall().modify(|r, w| {
                w.bits(if fall {
                    r.bits() | bit
                } else {
                    r.bits() & !bit
                })
            });

            // Drop anything latched while we were reconfiguring.
            pin.clear_interrupt_bit();

            gpio.ien().modify(|r, w| w.bits(r.bits() | bit));
        });

        // Arming the pin is pointless while the NVIC line is masked, and
        // making the caller work out odd-versus-even themselves is a step to
        // get wrong. SAFETY: the caller asked for this interrupt.
        unsafe {
            interrupt::enable_irq(if n.is_multiple_of(2) {
                interrupt::GPIO_EVEN
            } else {
                interrupt::GPIO_ODD
            })
        };
    }

    /// Stops reporting events on this pin.
    pub fn unlisten(&mut self) {
        let bit = 1u32 << self.pin.number;

        interrupt::free(|| unsafe {
            let gpio = &*efr32mg24_pac::GpioS::ptr();
            gpio.ien().modify(|r, w| w.bits(r.bits() & !bit));
            gpio.extirise().modify(|r, w| w.bits(r.bits() & !bit));
            gpio.extifall().modify(|r, w| w.bits(r.bits() & !bit));
        });
    }

    /// Whether this pin is currently reporting events.
    pub fn is_listening(&self) -> bool {
        unsafe {
            let gpio = &*efr32mg24_pac::GpioS::ptr();
            gpio.ien().read().bits() & self.pin.mask() != 0
        }
    }

    /// Whether this pin's interrupt flag is set.
    pub fn is_interrupt_set(&self) -> bool {
        unsafe {
            let gpio = &*efr32mg24_pac::GpioS::ptr();
            gpio.if_().read().bits() & self.pin.mask() != 0
        }
    }

    /// Clears this pin's interrupt flag.
    pub fn clear_interrupt(&mut self) {
        self.pin.clear_interrupt_bit();
    }

    /// The external interrupt number this pin uses.
    ///
    /// Even numbers are delivered on [`crate::interrupt::GPIO_EVEN`], odd ones
    /// on [`crate::interrupt::GPIO_ODD`].
    pub const fn interrupt(&self) -> u8 {
        self.pin.number
    }
}

// --------------------------------------------------------------------------
// Output
// --------------------------------------------------------------------------

/// An output pin.
pub struct Output<'d> {
    flex: Flex<'d>,
}

impl<'d> Output<'d> {
    /// Configures the pin as an output, driving `initial_level` from the start.
    pub fn new(pin: impl Pin + 'd, initial_level: Level, config: OutputConfig) -> Self {
        let mut flex = Flex::new(pin);
        // Settle DOUT before the mode switch, so the pin never drives the
        // wrong level even briefly.
        flex.set_level(initial_level);
        flex.apply_output_config(&config);
        flex.set_output_enable(true);
        Output { flex }
    }

    /// Re-applies an output configuration, keeping the current level.
    pub fn apply_config(&mut self, config: &OutputConfig) {
        self.flex.apply_output_config(config);
    }

    /// Drives the pin high.
    #[inline]
    pub fn set_high(&mut self) {
        self.flex.set_high();
    }

    /// Drives the pin low.
    #[inline]
    pub fn set_low(&mut self) {
        self.flex.set_low();
    }

    /// Drives the pin to `level`.
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        self.flex.set_level(level);
    }

    /// Inverts the driven level.
    #[inline]
    pub fn toggle(&mut self) {
        self.flex.toggle();
    }

    /// Whether the pin is being driven high.
    pub fn is_set_high(&self) -> bool {
        self.flex.is_set_high()
    }

    /// Whether the pin is being driven low.
    pub fn is_set_low(&self) -> bool {
        self.flex.is_set_low()
    }

    /// The level the pin is being driven to.
    pub fn output_level(&self) -> Level {
        self.flex.output_level()
    }

    /// The pin this driver was built from.
    pub fn pin(&self) -> AnyPin {
        self.flex.pin()
    }

    /// Gives up the typed wrapper and returns the underlying [`Flex`].
    ///
    /// This is the way to read the pad itself — useful on an open-drain output,
    /// where something else can hold the line low while DOUT says high.
    pub fn into_flex(self) -> Flex<'d> {
        self.flex
    }
}

// --------------------------------------------------------------------------
// Input
// --------------------------------------------------------------------------

/// An input pin.
pub struct Input<'d> {
    flex: Flex<'d>,
}

impl<'d> Input<'d> {
    /// Configures the pin as an input.
    pub fn new(pin: impl Pin + 'd, config: InputConfig) -> Self {
        let mut flex = Flex::new(pin);
        flex.apply_input_config(&config);
        flex.set_input_enable(true);
        Input { flex }
    }

    /// Re-applies an input configuration.
    pub fn apply_config(&mut self, config: &InputConfig) {
        self.flex.apply_input_config(config);
    }

    /// Whether the pad reads high.
    pub fn is_high(&self) -> bool {
        self.flex.is_high()
    }

    /// Whether the pad reads low.
    pub fn is_low(&self) -> bool {
        self.flex.is_low()
    }

    /// The level on the pad.
    pub fn level(&self) -> Level {
        self.flex.level()
    }

    /// Starts reporting `event`. See [`Flex::listen`].
    pub fn listen(&mut self, event: Event) {
        self.flex.listen(event);
    }

    /// Stops reporting events.
    pub fn unlisten(&mut self) {
        self.flex.unlisten();
    }

    /// Whether this pin's interrupt flag is set.
    pub fn is_interrupt_set(&self) -> bool {
        self.flex.is_interrupt_set()
    }

    /// Clears this pin's interrupt flag.
    pub fn clear_interrupt(&mut self) {
        self.flex.clear_interrupt();
    }

    /// The external interrupt number this pin uses.
    pub const fn interrupt(&self) -> u8 {
        self.flex.interrupt()
    }

    /// The pin this driver was built from.
    pub fn pin(&self) -> AnyPin {
        self.flex.pin()
    }

    /// Gives up the typed wrapper and returns the underlying [`Flex`].
    pub fn into_flex(self) -> Flex<'d> {
        self.flex
    }
}

// --------------------------------------------------------------------------
// embedded-hal 1.0
// --------------------------------------------------------------------------

impl ErrorType for Output<'_> {
    type Error = core::convert::Infallible;
}

impl ErrorType for Input<'_> {
    type Error = core::convert::Infallible;
}

impl ErrorType for Flex<'_> {
    type Error = core::convert::Infallible;
}

impl EhOutput for Output<'_> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Output::set_high(self);
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Output::set_low(self);
        Ok(())
    }
}

impl StatefulOutputPin for Output<'_> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.flex.is_set_high())
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.flex.is_set_low())
    }

    fn toggle(&mut self) -> Result<(), Self::Error> {
        Output::toggle(self);
        Ok(())
    }
}

impl EhInput for Input<'_> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.flex.is_high())
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.flex.is_low())
    }
}

impl EhOutput for Flex<'_> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Flex::set_high(self);
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Flex::set_low(self);
        Ok(())
    }
}

impl StatefulOutputPin for Flex<'_> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Flex::is_set_high(self))
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Flex::is_set_low(self))
    }

    fn toggle(&mut self) -> Result<(), Self::Error> {
        Flex::toggle(self);
        Ok(())
    }
}

impl EhInput for Flex<'_> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Flex::is_high(self))
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Flex::is_low(self))
    }
}
