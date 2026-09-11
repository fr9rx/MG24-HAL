//! Logging over RTT.
//!
//! RTT is a ring buffer in RAM that the debug probe reads over SWD while the
//! core keeps running. It needs no pins, no UART and no extra wiring — the same
//! SWD connection used to flash the chip carries the output.
//!
//! ```ignore
//! use mg24_hal::{rprintln, rtt};
//!
//! rtt::init();
//! rprintln!("temperature = {}", degrees);
//! ```
//!
//! Then on the host:
//!
//! ```text
//! probe-rs attach --chip EFR32MG24B220F1536IM48 target/.../your-binary
//! ```
//!
//! # How the probe finds it
//!
//! The host scans RAM for the 16-byte string `SEGGER RTT` and reads the control
//! block that follows. [`init`] writes that marker **last**, after every field
//! is valid, so a probe that happens to scan mid-initialisation cannot latch
//! onto a half-built block.
//!
//! # Cost
//!
//! [`BUFFER_SIZE`] bytes plus about 50 bytes of control block, all in `.bss`,
//! so it costs RAM but no flash beyond the code. Nothing is sent and nothing
//! blocks when no probe is attached.

use core::cell::UnsafeCell;
use core::fmt;
use core::ptr;
use core::sync::atomic::{Ordering, compiler_fence};

use crate::interrupt;

/// Size of the target-to-host ring buffer.
pub const BUFFER_SIZE: usize = 1024;

/// Size of the host-to-target ring buffer. Nothing reads from it yet; it exists
/// because the RTT layout expects a down channel to be present.
const DOWN_BUFFER_SIZE: usize = 16;

/// Drop whole messages that do not fit rather than blocking.
///
/// Blocking would wedge the core whenever a probe stops draining, which is a
/// bad trade for logging. Writing a partial message instead would corrupt the
/// line, so a message either lands whole or not at all.
const FLAG_NO_BLOCK_SKIP: u32 = 0;

/// One direction of one channel, laid out exactly as the probe expects.
#[repr(C)]
struct Buffer {
    name: *const u8,
    buffer: *mut u8,
    size: u32,
    /// Written by the target, read by the probe.
    write: u32,
    /// Written by the probe, read by the target.
    read: u32,
    flags: u32,
}

#[repr(C)]
struct ControlBlock {
    id: [u8; 16],
    max_up: i32,
    max_down: i32,
    up: Buffer,
    down: Buffer,
}

/// Wrapper that makes a `static` of raw pointers legal.
///
/// Access is confined to this module and serialised by [`interrupt::free`].
struct Shared<T>(UnsafeCell<T>);

// SAFETY: every access below runs inside `interrupt::free` on a single core.
unsafe impl<T> Sync for Shared<T> {}

static CHANNEL_NAME: [u8; 9] = *b"Terminal\0";

/// Starts zeroed, so the whole thing lands in `.bss` and costs no flash. The
/// zeroed `id` also means an uninitialised block can never be mistaken for a
/// live one.
static CONTROL_BLOCK: Shared<ControlBlock> = Shared(UnsafeCell::new(ControlBlock {
    id: [0; 16],
    max_up: 0,
    max_down: 0,
    up: Buffer {
        name: ptr::null(),
        buffer: ptr::null_mut(),
        size: 0,
        write: 0,
        read: 0,
        flags: 0,
    },
    down: Buffer {
        name: ptr::null(),
        buffer: ptr::null_mut(),
        size: 0,
        write: 0,
        read: 0,
        flags: 0,
    },
}));

static UP_BUFFER: Shared<[u8; BUFFER_SIZE]> = Shared(UnsafeCell::new([0; BUFFER_SIZE]));
static DOWN_BUFFER: Shared<[u8; DOWN_BUFFER_SIZE]> = Shared(UnsafeCell::new([0; DOWN_BUFFER_SIZE]));

/// Prepares the control block so a probe can find it.
///
/// Call once, before the first [`rprintln!`]. Calling it again is harmless but
/// resets the buffer, which will confuse an already-attached probe.
pub fn init() {
    interrupt::free(|| unsafe {
        let cb = CONTROL_BLOCK.0.get();

        (*cb).max_up = 1;
        (*cb).max_down = 1;

        (*cb).up = Buffer {
            name: CHANNEL_NAME.as_ptr(),
            buffer: UP_BUFFER.0.get().cast::<u8>(),
            size: BUFFER_SIZE as u32,
            write: 0,
            read: 0,
            flags: FLAG_NO_BLOCK_SKIP,
        };

        (*cb).down = Buffer {
            name: CHANNEL_NAME.as_ptr(),
            buffer: DOWN_BUFFER.0.get().cast::<u8>(),
            size: DOWN_BUFFER_SIZE as u32,
            write: 0,
            read: 0,
            flags: FLAG_NO_BLOCK_SKIP,
        };

        // Everything above must be visible before the marker appears, or a
        // probe scanning RAM could find the ID and read garbage behind it.
        compiler_fence(Ordering::SeqCst);

        let id = b"SEGGER RTT\0\0\0\0\0\0";
        for (i, byte) in id.iter().enumerate() {
            ptr::write_volatile((&raw mut (*cb).id).cast::<u8>().add(i), *byte);
        }
    });
}

/// How much room is left before the write pointer would catch the read pointer.
///
/// One byte is always left free: a full and an empty buffer would otherwise
/// both show `write == read`.
unsafe fn available(write: u32, read: u32) -> u32 {
    if read > write {
        read - write - 1
    } else {
        BUFFER_SIZE as u32 - write + read - 1
    }
}

/// Writes `bytes` into the ring buffer, or drops them if they do not fit.
fn write_bytes(bytes: &[u8]) {
    interrupt::free(|| unsafe {
        let cb = CONTROL_BLOCK.0.get();

        // Not initialised: nothing to write into.
        if (*cb).up.buffer.is_null() {
            return;
        }

        let mut write = ptr::read_volatile(&raw const (*cb).up.write);
        let read = ptr::read_volatile(&raw const (*cb).up.read);

        if bytes.len() as u32 > available(write, read) {
            return;
        }

        let buffer = (*cb).up.buffer;
        for byte in bytes {
            ptr::write_volatile(buffer.add(write as usize), *byte);
            write = (write + 1) % BUFFER_SIZE as u32;
        }

        // The bytes must be in the buffer before the probe is told about them.
        compiler_fence(Ordering::SeqCst);
        ptr::write_volatile(&raw mut (*cb).up.write, write);
    });
}

/// The RTT terminal, as a [`fmt::Write`] sink.
pub struct Terminal;

impl fmt::Write for Terminal {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_bytes(s.as_bytes());
        Ok(())
    }
}

/// Writes pre-formatted arguments. The macros below are the usual way in.
pub fn write_fmt(args: fmt::Arguments<'_>) {
    use fmt::Write;
    // `Terminal`'s `write_str` cannot fail, so there is nothing to handle.
    let _ = Terminal.write_fmt(args);
}

/// Logs without a trailing newline.
#[macro_export]
macro_rules! rprint {
    ($($arg:tt)*) => {
        $crate::rtt::write_fmt(::core::format_args!($($arg)*))
    };
}

/// Logs with a trailing newline.
#[macro_export]
macro_rules! rprintln {
    () => {
        $crate::rtt::write_fmt(::core::format_args!("\n"))
    };
    ($($arg:tt)*) => {
        $crate::rtt::write_fmt(::core::format_args!("{}\n", ::core::format_args!($($arg)*)))
    };
}
