//! A GPIO edge interrupt.
//!
//! PC3 is an input with a pull-up and the glitch filter on (RM 23.3.10.1 asks
//! for the filter on any pin driving an external interrupt). A falling edge —
//! a button pulling the line to ground — fires the handler, which toggles the
//! LED on PA7.
//!
//! Wiring: a button between PC3 and ground.
//!
//! # How the pin reaches a handler
//!
//! External interrupt `n` belongs to pin `n` (see `Flex::listen`). PC3 is pin 3,
//! which is odd, so it lands in `GPIO_ODD`. An even-numbered pin would arrive in
//! `GPIO_EVEN` instead. `listen` enables the NVIC line for you.

#![no_std]
#![no_main]

use core::cell::RefCell;

use mg24_hal::{
    CpuConfig,
    gpio::{Event, Input, InputConfig, Level, Output, OutputConfig, Pull},
    interrupt::{self, Mutex},
    rprintln, rtt,
};

/// Shared with the handler so it can clear the flag through the driver rather
/// than by poking registers.
static BUTTON: Mutex<RefCell<Option<Input<'static>>>> = Mutex::new(RefCell::new(None));

/// Owned by the handler, which is the only thing that touches the LED.
static LED: Mutex<RefCell<Option<Output<'static>>>> = Mutex::new(RefCell::new(None));

#[mg24_hal::interrupt]
fn GPIO_ODD() {
    BUTTON.lock(|slot| {
        if let Some(button) = slot.borrow_mut().as_mut() {
            // Leave this out and the handler re-enters forever.
            button.clear_interrupt();
        }
    });

    LED.lock(|slot| {
        if let Some(led) = slot.borrow_mut().as_mut() {
            led.toggle();
            // Note this calls into flash: core::fmt does not fit in .ram_text
            // alongside the handler, so this particular handler would not
            // survive a flash erase. Fine here, worth knowing.
            rprintln!("button -> led={:?}", led.output_level());
        }
    });
}

#[mg24_hal::main]
fn main() -> ! {
    let dp = mg24_hal::init(CpuConfig::default()).unwrap();

    rtt::init();
    rprintln!("waiting for a falling edge on PC3");

    let led = Output::new(dp.pins.pc1, Level::Low, OutputConfig::default());

    let mut button = Input::new(
        dp.pins.pc3,
        InputConfig::default().with_pull(Pull::Up).with_filter(true),
    );

    // `listen` enables the NVIC line, so an edge could be serviced the moment
    // it returns. Masking interrupts across the handover means the handler
    // cannot run before both statics are populated.
    interrupt::free(|| {
        button.listen(Event::FallingEdge);
        LED.lock(|slot| slot.borrow_mut().replace(led));
        BUTTON.lock(|slot| slot.borrow_mut().replace(button));
    });

    loop {
        // All the work happens in the handler.
        core::hint::spin_loop();
    }
}
