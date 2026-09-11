//! Minimal Cortex-M33 runtime for the EFR32MG24.
//!
//! This replaces `cortex-m-rt`. It provides the vector table, the reset
//! handler (which initialises `.data`/`.bss` and enables the FPU), and a
//! catch-all [`DefaultHandler`] that every exception and interrupt is aliased
//! to by `link.x`.
//!
//! # Defining the entry point
//!
//! ```ignore
//! #[mg24_hal::main]
//! fn main() -> ! {
//!     loop {}
//! }
//! ```
//!
//! # Overriding a handler
//!
//! `link.x` aliases every handler to `DefaultHandler` with `PROVIDE`, so an
//! application takes one over just by exporting a symbol with the same name:
//!
//! ```ignore
//! #[unsafe(no_mangle)]
//! pub extern "C" fn GPIO_EVEN() {
//!     // ...
//! }
//! ```

use core::ptr;

unsafe extern "C" {
    // Section boundaries, all supplied by `link.x`.
    static mut __sbss: u32;
    static mut __ebss: u32;
    static mut __sdata: u32;
    static mut __edata: u32;
    static __sidata: u32;
    static mut __sramtext: u32;
    static mut __eramtext: u32;
    static __siramtext: u32;

    // The application entry point, defined by the `entry!` macro.
    fn __mg24_main() -> !;
}

/// Address of the SCB Coprocessor Access Control Register.
const SCB_CPACR: *mut u32 = 0xE000_ED88 as *mut u32;

/// Grants full access to CP10 and CP11, the two coprocessor slots the FPU
/// lives in. The Cortex-M33 traps every floating-point instruction until this
/// is set, and the `thumbv8m.main-none-eabihf` target emits them freely.
#[inline(always)]
unsafe fn enable_fpu() {
    unsafe {
        let cpacr = ptr::read_volatile(SCB_CPACR);
        ptr::write_volatile(SCB_CPACR, cpacr | (0b1111 << 20));
        // The architecture requires a barrier pair before any FP instruction
        // is allowed to depend on the new CPACR value.
        core::arch::asm!("dsb", "isb", options(nomem, nostack, preserves_flags));
    }
}

/// The reset handler: the first thing that runs after power-on or reset.
///
/// # Safety
///
/// Only the hardware reset vector may call this.
#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Reset() -> ! {
    unsafe {
        // Do this first: everything below is compiled for a target with an
        // FPU, so the compiler is free to use FP registers at any point.
        enable_fpu();

        // Zero `.bss`.
        let mut dst = &raw mut __sbss;
        let end = &raw mut __ebss;
        while dst < end {
            ptr::write_volatile(dst, 0);
            dst = dst.add(1);
        }

        // Copy `.data` from its load address in flash to its home in RAM.
        let mut dst = &raw mut __sdata;
        let end = &raw mut __edata;
        let mut src = &raw const __sidata;
        while dst < end {
            ptr::write_volatile(dst, ptr::read_volatile(src));
            dst = dst.add(1);
            src = src.add(1);
        }

        // Copy `.ram_text` the same way. This has to happen before anything
        // can call into it, which means before interrupts are ever enabled.
        let mut dst = &raw mut __sramtext;
        let end = &raw mut __eramtext;
        let mut src = &raw const __siramtext;
        while dst < end {
            ptr::write_volatile(dst, ptr::read_volatile(src));
            dst = dst.add(1);
            src = src.add(1);
        }

        // The instructions just written went through the data path, so the
        // instruction side has to be told to refetch before they are executed.
        core::arch::asm!("dsb", "isb", options(nomem, nostack, preserves_flags));

        __mg24_main()
    }
}

/// Runs whenever an exception or interrupt fires that the application has not
/// taken over. Spins, so a debugger can catch it with the core still stopped
/// at the fault.
#[unsafe(no_mangle)]
pub extern "C" fn DefaultHandler() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

/// One vector table slot: either a handler address or a reserved zero word.
#[repr(C)]
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

// Every one of these resolves to `DefaultHandler` via `PROVIDE` in `link.x`
// unless the application exports its own symbol with the same name.
unsafe extern "C" {
    fn NMI();
    fn HardFault();
    fn MemManage();
    fn BusFault();
    fn UsageFault();
    fn SecureFault();
    fn SVCall();
    fn DebugMonitor();
    fn PendSV();
    fn SysTick();

    fn SMU_SECURE();
    fn SMU_S_PRIVILEGED();
    fn EMU();
    fn TIMER0();
    fn TIMER1();
    fn TIMER2();
    fn TIMER3();
    fn TIMER4();
    fn USART0_RX();
    fn USART0_TX();
    fn EUSART0_RX();
    fn EUSART0_TX();
    fn EUSART1_RX();
    fn EUSART1_TX();
    fn ICACHE0();
    fn BURTC();
    fn LETIMER0();
    fn SYSCFG();
    fn LDMA();
    fn LFXO();
    fn ULFRCO();
    fn GPIO_ODD();
    fn GPIO_EVEN();
    fn I2C0();
    fn I2C1();
    fn EMUDG();
    fn ACMP0();
    fn ACMP1();
    fn WDOG0();
    fn WDOG1();
    fn HFXO0();
    fn HFRCO0();
    fn HFRCOEM23();
    fn CMU();
    fn AES();
    fn IADC();
    fn MSC();
    fn DPLL0();
    fn PCNT0();
    fn SW0();
    fn SW1();
    fn SW2();
    fn SW3();
    fn SEMBRX();
    fn SEMBTX();
    fn KEYSCAN();
}

/// Exception 1: the reset vector, at offset 4 of the vector table.
#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".vector_table.reset_vector")]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

/// Exceptions 2..=15, at offsets 8..0x40 of the vector table.
#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".vector_table.exceptions")]
pub static __EXCEPTIONS: [Vector; 14] = [
    Vector { handler: NMI },       //  2
    Vector { handler: HardFault }, //  3
    Vector { handler: MemManage }, //  4
    Vector { handler: BusFault },  //  5
    Vector {
        handler: UsageFault,
    }, //  6
    Vector {
        handler: SecureFault,
    }, //  7
    Vector { reserved: 0 },        //  8
    Vector { reserved: 0 },        //  9
    Vector { reserved: 0 },        // 10
    Vector { handler: SVCall },    // 11
    Vector {
        handler: DebugMonitor,
    }, // 12
    Vector { reserved: 0 },        // 13
    Vector { handler: PendSV },    // 14
    Vector { handler: SysTick },   // 15
];

/// The 70 device interrupts, at offsets 0x40.. of the vector table. Indices
/// match the `efr32mg24_pac::Interrupt` discriminants; the gaps are reserved.
#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".vector_table.interrupts")]
pub static __INTERRUPTS: [Vector; 70] = [
    Vector {
        handler: SMU_SECURE,
    }, //  0
    Vector {
        handler: SMU_S_PRIVILEGED,
    }, //  1
    Vector { reserved: 0 },        //  2
    Vector { handler: EMU },       //  3
    Vector { handler: TIMER0 },    //  4
    Vector { handler: TIMER1 },    //  5
    Vector { handler: TIMER2 },    //  6
    Vector { handler: TIMER3 },    //  7
    Vector { handler: TIMER4 },    //  8
    Vector { handler: USART0_RX }, //  9
    Vector { handler: USART0_TX }, // 10
    Vector {
        handler: EUSART0_RX,
    }, // 11
    Vector {
        handler: EUSART0_TX,
    }, // 12
    Vector {
        handler: EUSART1_RX,
    }, // 13
    Vector {
        handler: EUSART1_TX,
    }, // 14
    Vector { reserved: 0 },        // 15
    Vector { handler: ICACHE0 },   // 16
    Vector { handler: BURTC },     // 17
    Vector { handler: LETIMER0 },  // 18
    Vector { handler: SYSCFG },    // 19
    Vector { reserved: 0 },        // 20
    Vector { handler: LDMA },      // 21
    Vector { handler: LFXO },      // 22
    Vector { reserved: 0 },        // 23
    Vector { handler: ULFRCO },    // 24
    Vector { handler: GPIO_ODD },  // 25
    Vector { handler: GPIO_EVEN }, // 26
    Vector { handler: I2C0 },      // 27
    Vector { handler: I2C1 },      // 28
    Vector { handler: EMUDG },     // 29
    Vector { reserved: 0 },        // 30
    Vector { reserved: 0 },        // 31
    Vector { reserved: 0 },        // 32
    Vector { reserved: 0 },        // 33
    Vector { reserved: 0 },        // 34
    Vector { reserved: 0 },        // 35
    Vector { reserved: 0 },        // 36
    Vector { reserved: 0 },        // 37
    Vector { reserved: 0 },        // 38
    Vector { reserved: 0 },        // 39
    Vector { handler: ACMP0 },     // 40
    Vector { handler: ACMP1 },     // 41
    Vector { handler: WDOG0 },     // 42
    Vector { handler: WDOG1 },     // 43
    Vector { handler: HFXO0 },     // 44
    Vector { handler: HFRCO0 },    // 45
    Vector { handler: HFRCOEM23 }, // 46
    Vector { handler: CMU },       // 47
    Vector { handler: AES },       // 48
    Vector { handler: IADC },      // 49
    Vector { handler: MSC },       // 50
    Vector { handler: DPLL0 },     // 51
    Vector { reserved: 0 },        // 52
    Vector { reserved: 0 },        // 53
    Vector { handler: PCNT0 },     // 54
    Vector { handler: SW0 },       // 55
    Vector { handler: SW1 },       // 56
    Vector { handler: SW2 },       // 57
    Vector { handler: SW3 },       // 58
    Vector { reserved: 0 },        // 59
    Vector { reserved: 0 },        // 60
    Vector { reserved: 0 },        // 61
    Vector { reserved: 0 },        // 62
    Vector { reserved: 0 },        // 63
    Vector { reserved: 0 },        // 64
    Vector { handler: SEMBRX },    // 65
    Vector { handler: SEMBTX },    // 66
    Vector { reserved: 0 },        // 67
    Vector { reserved: 0 },        // 68
    Vector { handler: KEYSCAN },   // 69
];
