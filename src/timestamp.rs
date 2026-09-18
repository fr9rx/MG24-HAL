//! Timestamp support for RTT logging - milliseconds since boot.
//!
//! Provides ESP-IDF-style timestamps (milliseconds elapsed since CPU boot).
//!
//! ```ignore
//! use mg24_hal::{rprintln_ts, rtt, timestamp};
//!
//! rtt::init();
//! rprintln_ts!("System started");
//! // Output: [1234] System started
//! ```

use core::sync::atomic::{AtomicU32, Ordering};

/// Global millisecond counter (32-bit, wraps around after ~49 days)
static MILLIS: AtomicU32 = AtomicU32::new(0);

/// Initialize the timestamp system. Must be called once at startup.
pub fn init() {
    // Reset the counter to 0
    MILLIS.store(0, Ordering::SeqCst);
}

/// Get milliseconds elapsed since boot.
///
/// Returns 32-bit value that wraps after ~49 days.
pub fn millis_since_boot() -> u32 {
    MILLIS.load(Ordering::Relaxed)
}

/// Update the millisecond counter (typically called by SysTick handler or delay module).
/// Increments the counter by 1 millisecond.
pub fn tick_ms() {
    MILLIS.fetch_add(1, Ordering::Relaxed);
}

/// Manually set the millisecond counter to a specific value.
pub fn set_millis(ms: u32) {
    MILLIS.store(ms, Ordering::SeqCst);
}

/// Advance milliseconds by a specific amount.
pub fn advance_by_ms(ms: u32) {
    MILLIS.fetch_add(ms, Ordering::Relaxed);
}

/// Simple way to get monotonic time for timestamping logs.
/// Returns milliseconds since boot in a compact format.
pub fn format_timestamp(ms: u32) -> (u32, u32, u32, u32) {
    // Break down milliseconds into days, hours, minutes, seconds, and remaining millis
    let total_seconds = ms / 1000;
    let remaining_ms = ms % 1000;

    let seconds = total_seconds % 60;
    let total_minutes = total_seconds / 60;
    let minutes = total_minutes % 60;
    let total_hours = total_minutes / 60;
    let hours = total_hours % 24;
    let days = total_hours / 24;

    (days, hours, minutes, seconds * 1000 + remaining_ms)
}
