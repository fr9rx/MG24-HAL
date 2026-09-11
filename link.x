/* Linker script for the EFR32MG24B220F1536IM48 (Seeed Studio XIAO MG24 Sense).
 *
 * This replaces cortex-m-rt's link.x + the separate memory.x. The chip is
 * fixed, so the memory map is inlined rather than split into its own file.
 */

MEMORY
{
  /* Main flash: 1536 KB */
  FLASH : ORIGIN = 0x08000000, LENGTH = 1536K

  /* SRAM: 256 KB */
  RAM   : ORIGIN = 0x20000000, LENGTH = 256K
}

/* The stack grows down from the top of RAM. */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

ENTRY(Reset);

/* Keep the vector table and its handlers even though nothing calls them. */
EXTERN(__RESET_VECTOR);
EXTERN(__EXCEPTIONS);
EXTERN(__INTERRUPTS);
EXTERN(DefaultHandler);

/* Cortex-M33 core exceptions. An application overrides one by defining a
 * `#[unsafe(no_mangle)] pub extern "C" fn <NAME>()` with the matching name. */
PROVIDE(NMI          = DefaultHandler);
PROVIDE(HardFault    = DefaultHandler);
PROVIDE(MemManage    = DefaultHandler);
PROVIDE(BusFault     = DefaultHandler);
PROVIDE(UsageFault   = DefaultHandler);
PROVIDE(SecureFault  = DefaultHandler);
PROVIDE(SVCall       = DefaultHandler);
PROVIDE(DebugMonitor = DefaultHandler);
PROVIDE(PendSV       = DefaultHandler);
PROVIDE(SysTick      = DefaultHandler);

/* Device interrupts (same names as the efr32mg24-pac Interrupt enum). */
PROVIDE(SMU_SECURE       = DefaultHandler);
PROVIDE(SMU_S_PRIVILEGED = DefaultHandler);
PROVIDE(EMU              = DefaultHandler);
PROVIDE(TIMER0           = DefaultHandler);
PROVIDE(TIMER1           = DefaultHandler);
PROVIDE(TIMER2           = DefaultHandler);
PROVIDE(TIMER3           = DefaultHandler);
PROVIDE(TIMER4           = DefaultHandler);
PROVIDE(USART0_RX        = DefaultHandler);
PROVIDE(USART0_TX        = DefaultHandler);
PROVIDE(EUSART0_RX       = DefaultHandler);
PROVIDE(EUSART0_TX       = DefaultHandler);
PROVIDE(EUSART1_RX       = DefaultHandler);
PROVIDE(EUSART1_TX       = DefaultHandler);
PROVIDE(ICACHE0          = DefaultHandler);
PROVIDE(BURTC            = DefaultHandler);
PROVIDE(LETIMER0         = DefaultHandler);
PROVIDE(SYSCFG           = DefaultHandler);
PROVIDE(LDMA             = DefaultHandler);
PROVIDE(LFXO             = DefaultHandler);
PROVIDE(ULFRCO           = DefaultHandler);
PROVIDE(GPIO_ODD         = DefaultHandler);
PROVIDE(GPIO_EVEN        = DefaultHandler);
PROVIDE(I2C0             = DefaultHandler);
PROVIDE(I2C1             = DefaultHandler);
PROVIDE(EMUDG            = DefaultHandler);
PROVIDE(ACMP0            = DefaultHandler);
PROVIDE(ACMP1            = DefaultHandler);
PROVIDE(WDOG0            = DefaultHandler);
PROVIDE(WDOG1            = DefaultHandler);
PROVIDE(HFXO0            = DefaultHandler);
PROVIDE(HFRCO0           = DefaultHandler);
PROVIDE(HFRCOEM23        = DefaultHandler);
PROVIDE(CMU              = DefaultHandler);
PROVIDE(AES              = DefaultHandler);
PROVIDE(IADC             = DefaultHandler);
PROVIDE(MSC              = DefaultHandler);
PROVIDE(DPLL0            = DefaultHandler);
PROVIDE(PCNT0            = DefaultHandler);
PROVIDE(SW0              = DefaultHandler);
PROVIDE(SW1              = DefaultHandler);
PROVIDE(SW2              = DefaultHandler);
PROVIDE(SW3              = DefaultHandler);
PROVIDE(SEMBRX           = DefaultHandler);
PROVIDE(SEMBTX           = DefaultHandler);
PROVIDE(KEYSCAN          = DefaultHandler);

SECTIONS
{
  /* The vector table must sit at the start of flash: the core fetches the
   * initial MSP from offset 0 and the reset vector from offset 4. */
  .vector_table ORIGIN(FLASH) :
  {
    __svector_table = .;

    /* Offset 0: initial stack pointer. */
    LONG(_stack_start);

    /* Offset 4: reset vector. */
    KEEP(*(.vector_table.reset_vector));

    /* Offsets 8..0x40: exceptions 2..15. */
    KEEP(*(.vector_table.exceptions));

    /* Offsets 0x40..: the 70 device interrupts. */
    KEEP(*(.vector_table.interrupts));

    __evector_table = .;
  } > FLASH

  .text : ALIGN(4)
  {
    __stext = .;
    *(.text .text.*);
    . = ALIGN(4);
    __etext = .;
  } > FLASH

  .rodata : ALIGN(4)
  {
    __srodata = .;
    *(.rodata .rodata.*);
    . = ALIGN(4);
    __erodata = .;
  } > FLASH

  /* Code that executes from RAM. Like .data it lives in RAM but is loaded
   * from flash, and the reset handler copies it across before main runs.
   *
   * Interrupt handlers go here: RAM has no wait states, so entry latency stops
   * depending on MSC_READCTRL.MODE, and the handler keeps working while flash
   * is being erased or programmed. SRAM is executable under the ARMv8-M
   * default memory map, so no MPU work is needed. */
  .ram_text : ALIGN(4)
  {
    . = ALIGN(4);
    __sramtext = .;
    *(.ram_text .ram_text.*);
    . = ALIGN(4);
    __eramtext = .;
  } > RAM AT > FLASH

  __siramtext = LOADADDR(.ram_text);

  /* Initialised statics: they live in RAM but are loaded from flash, and the
   * reset handler copies them across. */
  .data : ALIGN(4)
  {
    . = ALIGN(4);
    __sdata = .;
    *(.data .data.*);
    . = ALIGN(4);
    __edata = .;
  } > RAM AT > FLASH

  __sidata = LOADADDR(.data);

  /* Zero-initialised statics, zeroed by the reset handler. */
  .bss (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    __sbss = .;
    *(.bss .bss.*);
    *(COMMON);
    . = ALIGN(4);
    __ebss = .;
  } > RAM

  /* Statics that deliberately keep their value across a warm reset: neither
   * copied nor zeroed. */
  .uninit (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    __suninit = .;
    *(.uninit .uninit.*);
    . = ALIGN(4);
    __euninit = .;
  } > RAM

  /* Unwind tables are dead weight without a panic-unwind runtime. */
  /DISCARD/ :
  {
    *(.ARM.exidx);
    *(.ARM.exidx.*);
    *(.ARM.extab);
    *(.ARM.extab.*);
  }
}

/* 1 initial-SP word + 1 reset vector + 14 exceptions + 70 interrupts. */
ASSERT(__evector_table - __svector_table == (1 + 1 + 14 + 70) * 4,
  "BUG(mg24-hal): the vector table is the wrong size. Are src/rt.rs and link.x in sync?");

ASSERT(__sdata % 4 == 0 && __edata % 4 == 0,
  "BUG(mg24-hal): .data is not 4-byte aligned");

ASSERT(__sidata % 4 == 0,
  "BUG(mg24-hal): the load address of .data is not 4-byte aligned");

ASSERT(__sramtext % 4 == 0 && __eramtext % 4 == 0,
  "BUG(mg24-hal): .ram_text is not 4-byte aligned");

ASSERT(__siramtext % 4 == 0,
  "BUG(mg24-hal): the load address of .ram_text is not 4-byte aligned");

ASSERT(__sbss % 4 == 0 && __ebss % 4 == 0,
  "BUG(mg24-hal): .bss is not 4-byte aligned");

ASSERT(_stack_start % 8 == 0,
  "BUG(mg24-hal): the stack start is not 8-byte aligned (AAPCS requires it)");

ASSERT(__euninit <= _stack_start,
  "RAM overflowed: static data does not fit below the stack");
