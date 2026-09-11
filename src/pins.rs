//! Pin tokens.
//!
//! A [`GpioPin`] is a zero-sized proof that a particular port/pin is not
//! already in use. [`crate::init`] hands out exactly one of each, and the
//! [`crate::gpio`] types consume it.
//!
//! The port and pin are const generics here so that a pin the chip does not
//! have is a compile error. Direction is not tracked — that is the job of
//! [`crate::gpio::Input`], [`crate::gpio::Output`] and [`crate::gpio::Flex`].

/// True for every port/pin the EFR32MG24 implements.
///
/// Ports A and C have MODEL and MODEH, so they reach pin 15. Ports B and D have
/// only MODEL (RM 23.5), so they stop at pin 7.
pub const fn is_supported(port: char, pin: u8) -> bool {
    match port {
        'A' | 'C' => pin < 16,
        'B' | 'D' => pin < 8,
        _ => false,
    }
}

/// A token for pin `PIN` on port `PORT`.
pub struct GpioPin<const PORT: char, const PIN: u8>;

impl<const PORT: char, const PIN: u8> GpioPin<PORT, PIN> {
    /// Rejects pins the chip does not have, at compile time.
    ///
    /// Forced by [`crate::gpio::Pin::degrade`], which is the only way into the
    /// driver types.
    pub(crate) const VALID: () = assert!(
        is_supported(PORT, PIN),
        "mg24-hal: no such pin on the EFR32MG24 (ports A and C have pins 0..=15, ports B and D have pins 0..=7)"
    );

    /// Only [`crate::init`] may mint these; handing them out freely would let
    /// an application configure the same pin twice.
    pub(crate) const fn new() -> Self {
        GpioPin
    }

    /// This pin's port letter.
    pub const fn port(&self) -> char {
        PORT
    }

    /// This pin's number within its port.
    pub const fn number(&self) -> u8 {
        PIN
    }
}
