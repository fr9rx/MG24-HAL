//! CPU clock speed.
//!
//! SYSCLK runs from HFRCODPLL, and [`set_cpu_speed`] retunes that oscillator to
//! one of its factory-calibrated bands (RM Table 9.1). The calibration values
//! come from the DEVINFO page, so the resulting frequency is trimmed per chip
//! rather than nominal.
//!
//! ```ignore
//! use mg24_hal::{CpuConfig, clock::CpuSpeed};
//!
//! let dp = mg24_hal::init(CpuConfig::default().with_cpu_speed(CpuSpeed::Mhz38)).unwrap();
//! ```
//!
//! [`delay`](crate::delay) reads [`sysclk_hz`], so delays stay correct across a
//! speed change.

use core::sync::atomic::{AtomicU32, Ordering};

/// SYSCLK immediately after reset.
///
/// `CMU_SYSCLKCTRL.CLKSEL` comes up selecting HFRCODPLL, and HFRCODPLL's
/// default band is 19 MHz (RM Table 9.1, "19 MHz (default)"). Confirmed by
/// reading `CMU_SYSCLKCTRL` on a live part: it reads `0x2` = HFRCODPLL.
pub const DEFAULT_SYSCLK_HZ: u32 = 19_000_000;

/// `CMU_SYSCLKCTRL.CLKSEL` value selecting HFRCODPLL.
const CLKSEL_HFRCODPLL: u32 = 2;

static SYSCLK_HZ: AtomicU32 = AtomicU32::new(DEFAULT_SYSCLK_HZ);

/// The current SYSCLK frequency in Hz.
///
/// Tracks whatever [`set_cpu_speed`] last applied, so anything that needs to
/// convert time into core cycles can ask here instead of guessing.
pub fn sysclk_hz() -> u32 {
    SYSCLK_HZ.load(Ordering::Relaxed)
}

/// A factory-calibrated HFRCODPLL band.
///
/// # Why these are all safe at reset settings
///
/// Two reset defaults do the work. RM 11.3.5.1: "The system defaults to VSCALE2
/// out of reset", which covers EM0/EM1 operation up to 80 MHz. And
/// `MSC_READCTRL.MODE` resets to `WS2`, one more wait state than the `WS1` the
/// datasheet's table needs for 78 MHz at VSCALE2. Extra wait states only cost
/// speed, never correctness, so no EMU or MSC configuration is required for any
/// band here.
///
/// # Why there is no 78 MHz
///
/// 78 MHz is not an HFRCO band. The datasheet reaches it as "HFRCO w/ DPLL",
/// locking HFRCODPLL to the 39 MHz HFXO crystal at 2x. That needs HFXO startup
/// and DPLL lock, neither of which this crate does yet. The 80 MHz band the
/// hardware offers is deliberately left out: it is above the part's 78 MHz
/// maximum core frequency.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuSpeed {
    Mhz4,
    Mhz5,
    Mhz7,
    Mhz10,
    Mhz13,
    Mhz16,
    /// The reset default.
    Mhz19,
    Mhz20,
    Mhz26,
    Mhz32,
    Mhz38,
    Mhz48,
    Mhz56,
    /// The fastest band this crate offers. 78 MHz needs HFXO plus the DPLL.
    Mhz64,
}

impl CpuSpeed {
    /// The nominal frequency of this band, in Hz.
    pub const fn hz(self) -> u32 {
        match self {
            CpuSpeed::Mhz4 => 4_000_000,
            CpuSpeed::Mhz5 => 5_000_000,
            CpuSpeed::Mhz7 => 7_000_000,
            CpuSpeed::Mhz10 => 10_000_000,
            CpuSpeed::Mhz13 => 13_000_000,
            CpuSpeed::Mhz16 => 16_000_000,
            CpuSpeed::Mhz19 => 19_000_000,
            CpuSpeed::Mhz20 => 20_000_000,
            CpuSpeed::Mhz26 => 26_000_000,
            CpuSpeed::Mhz32 => 32_000_000,
            CpuSpeed::Mhz38 => 38_000_000,
            CpuSpeed::Mhz48 => 48_000_000,
            CpuSpeed::Mhz56 => 56_000_000,
            CpuSpeed::Mhz64 => 64_000_000,
        }
    }

    /// Which `DEVINFO_HFRCODPLLCALn` holds this band's trim (RM Table 9.1).
    /// The gaps in the numbering are the manual's, not a mistake.
    const fn cal_index(self) -> u8 {
        match self {
            CpuSpeed::Mhz4 => 0,
            CpuSpeed::Mhz5 => 1,
            CpuSpeed::Mhz7 => 3,
            CpuSpeed::Mhz10 => 4,
            CpuSpeed::Mhz13 => 6,
            CpuSpeed::Mhz16 => 7,
            CpuSpeed::Mhz19 => 8,
            CpuSpeed::Mhz20 => 9,
            CpuSpeed::Mhz26 => 10,
            CpuSpeed::Mhz32 => 11,
            CpuSpeed::Mhz38 => 12,
            CpuSpeed::Mhz48 => 13,
            CpuSpeed::Mhz56 => 14,
            CpuSpeed::Mhz64 => 15,
        }
    }

    /// This band's factory trim, read out of the DEVINFO page.
    fn calibration(self) -> u32 {
        // SAFETY: DEVINFO is a read-only factory page; reading it has no side
        // effects and needs no ownership.
        let di = unsafe { &*efr32mg24_pac::Devinfo::ptr() };

        match self.cal_index() {
            0 => di.hfrcodpllcal0().read().bits(),
            1 => di.hfrcodpllcal1().read().bits(),
            3 => di.hfrcodpllcal3().read().bits(),
            4 => di.hfrcodpllcal4().read().bits(),
            6 => di.hfrcodpllcal6().read().bits(),
            7 => di.hfrcodpllcal7().read().bits(),
            8 => di.hfrcodpllcal8().read().bits(),
            9 => di.hfrcodpllcal9().read().bits(),
            10 => di.hfrcodpllcal10().read().bits(),
            11 => di.hfrcodpllcal11().read().bits(),
            12 => di.hfrcodpllcal12().read().bits(),
            13 => di.hfrcodpllcal13().read().bits(),
            14 => di.hfrcodpllcal14().read().bits(),
            // Unreachable: `cal_index` only produces the values above.
            _ => di.hfrcodpllcal15().read().bits(),
        }
    }
}

/// Retunes HFRCODPLL to `speed` and points SYSCLK at it.
///
/// # Wait states
///
/// `MSC_READCTRL.MODE` is deliberately left alone. It resets to `WS2`, and the
/// datasheet's wait-state table allows `WS1` up to 40 MHz even on VSCALE1, so
/// two wait states already cover every band [`CpuSpeed`] offers. Trimming it to
/// the minimum would buy a little performance in exchange for a class of bug
/// that hangs the core, and RM 6.3.9's ordering rule only matters when the
/// setting actually changes.
pub fn set_cpu_speed(speed: CpuSpeed) -> Result<(), ClockError> {
    let calibration = speed.calibration();

    // SAFETY: single-core, and `init` hands out the peripherals once.
    unsafe {
        let cmu = &*efr32mg24_pac::CmuS::ptr();

        // HFRCO0's register interface has its own bus clock, and without it
        // every access below bus-faults. Enabling GPIO is not enough.
        cmu.clken0().modify(|_, w| w.hfrco0().set_bit());

        let hfrco = &*efr32mg24_pac::Hfrco0S::ptr();

        // RM 9.3.3: a write to CAL while a previous retune is still running is
        // queued rather than dropped, so wait it out before and after.
        wait_for_ready(hfrco)?;

        hfrco.cal().write(|w| w.bits(calibration));

        wait_for_ready(hfrco)?;

        // Reset already selects HFRCODPLL, but say so explicitly so this works
        // even if something else moved SYSCLK first.
        cmu.sysclkctrl()
            .modify(|r, w| w.bits((r.bits() & !0b111) | CLKSEL_HFRCODPLL));
    }

    SYSCLK_HZ.store(speed.hz(), Ordering::Relaxed);
    Ok(())
}

/// Something went wrong changing the clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockError {
    /// HFRCODPLL never reported the retune finished.
    Timeout,
}

/// Spins until HFRCODPLL is done retuning, but not forever.
///
/// A retune takes microseconds. The bound exists so that a mistake here — a
/// missing bus clock, a wedged oscillator — surfaces as an error the caller can
/// report instead of a silently hung core, which is a genuinely hard state to
/// diagnose on a board with no output.
fn wait_for_ready(hfrco: &efr32mg24_pac::hfrco0_s::RegisterBlock) -> Result<(), ClockError> {
    // Generous: far longer than a retune, far shorter than "hung forever".
    const MAX_SPINS: u32 = 1_000_000;

    for _ in 0..MAX_SPINS {
        if hfrco.status().read().freqbsy().bit_is_clear() {
            return Ok(());
        }
    }

    Err(ClockError::Timeout)
}
