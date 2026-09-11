//! Procedural macros for [`mg24-hal`](https://crates.io/crates/mg24-hal).
//!
//! [`macro@main`] marks the application entry point, and [`macro@interrupt`]
//! marks an interrupt or exception handler.

use proc_macro::{Delimiter, TokenStream, TokenTree};

/// Every handler name `link.x` declares with `PROVIDE`.
///
/// A handler only runs if its symbol matches one of these exactly, so a typo
/// would otherwise compile into a function nothing ever calls — a silent
/// failure that is miserable to track down on hardware.
const HANDLERS: &[&str] = &[
    // Cortex-M33 exceptions.
    "NMI",
    "HardFault",
    "MemManage",
    "BusFault",
    "UsageFault",
    "SecureFault",
    "SVCall",
    "DebugMonitor",
    "PendSV",
    "SysTick",
    // EFR32MG24 device interrupts.
    "SMU_SECURE",
    "SMU_S_PRIVILEGED",
    "EMU",
    "TIMER0",
    "TIMER1",
    "TIMER2",
    "TIMER3",
    "TIMER4",
    "USART0_RX",
    "USART0_TX",
    "EUSART0_RX",
    "EUSART0_TX",
    "EUSART1_RX",
    "EUSART1_TX",
    "ICACHE0",
    "BURTC",
    "LETIMER0",
    "SYSCFG",
    "LDMA",
    "LFXO",
    "ULFRCO",
    "GPIO_ODD",
    "GPIO_EVEN",
    "I2C0",
    "I2C1",
    "EMUDG",
    "ACMP0",
    "ACMP1",
    "WDOG0",
    "WDOG1",
    "HFXO0",
    "HFRCO0",
    "HFRCOEM23",
    "CMU",
    "AES",
    "IADC",
    "MSC",
    "DPLL0",
    "PCNT0",
    "SW0",
    "SW1",
    "SW2",
    "SW3",
    "SEMBRX",
    "SEMBTX",
    "KEYSCAN",
];

/// Marks an interrupt or exception handler.
///
/// The function keeps its own name, which is what binds it to the vector table
/// slot — `link.x` aliases each slot to `DefaultHandler` with `PROVIDE`, and
/// exporting a matching symbol takes it over.
///
/// ```ignore
/// #[mg24_hal::interrupt]
/// fn GPIO_ODD() {
///     // ...
/// }
/// ```
///
/// The name is checked against the device's handler list, so a misspelling is a
/// compile error rather than a handler that never fires.
///
/// # Placement
///
/// Handlers go in `.ram_text` by default, which the reset handler copies out of
/// flash before `main` runs. RAM has no wait states, so entry latency stops
/// depending on `MSC_READCTRL.MODE`, and the handler keeps working while flash
/// is being erased or programmed.
///
/// Only the handler itself is moved. Anything it calls stays wherever the
/// compiler put it, so a handler that must survive a flash write has to avoid
/// calling into flash — or be small enough to inline what it needs.
///
/// Pass `flash` to leave it in flash and save the RAM:
///
/// ```ignore
/// #[mg24_hal::interrupt(flash)]
/// fn TIMER0() {
///     // ...
/// }
/// ```
#[proc_macro_attribute]
pub fn interrupt(args: TokenStream, input: TokenStream) -> TokenStream {
    match expand_interrupt(args, input) {
        Ok(tokens) => tokens,
        Err(message) => compile_error(&message),
    }
}

fn expand_interrupt(args: TokenStream, input: TokenStream) -> Result<TokenStream, String> {
    let in_ram = match args.into_iter().collect::<Vec<_>>().as_slice() {
        [] => true,
        [TokenTree::Ident(ident)] if ident.to_string() == "flash" => false,
        [TokenTree::Ident(ident)] if ident.to_string() == "ram" => true,
        _ => {
            return Err(
                "`#[mg24_hal::interrupt]` takes either no argument (the handler goes in RAM) or \
                 `flash` to leave it in flash"
                    .into(),
            );
        }
    };

    let tokens: Vec<TokenTree> = input.into_iter().collect();

    let fn_pos = tokens
        .iter()
        .position(|token| matches!(token, TokenTree::Ident(i) if i.to_string() == "fn"))
        .ok_or("`#[mg24_hal::interrupt]` must be applied to a function")?;

    let name = match tokens.get(fn_pos + 1) {
        Some(TokenTree::Ident(ident)) => ident.to_string(),
        _ => return Err("expected a handler name after `fn`".into()),
    };

    if !HANDLERS.contains(&name.as_str()) {
        return Err(format!(
            "`{name}` is not an EFR32MG24 interrupt or exception. The name must match the vector \
             table exactly, e.g. GPIO_ODD, GPIO_EVEN, TIMER0, LDMA. See link.x for the full list."
        ));
    }

    match tokens.get(fn_pos + 2) {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
            if !group.stream().is_empty() {
                return Err("an interrupt handler cannot take any arguments".into());
            }
        }
        _ => return Err("expected `()` after the handler name".into()),
    }

    let body = match tokens.last() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => group.clone(),
        _ => return Err("an interrupt handler must have a body".into()),
    };

    // Anything between `()` and the body would be a return type, and a handler
    // has nowhere to return a value to.
    let between: String = tokens[fn_pos + 3..tokens.len() - 1]
        .iter()
        .map(|token| token.to_string())
        .collect();

    if !between.trim().is_empty() {
        return Err("an interrupt handler cannot return a value".into());
    }

    // A per-handler subsection so the linker can still drop an unused one, and
    // so `--gc-sections` keeps the granularity it would have in .text.
    let placement = if in_ram {
        format!(r#"#[unsafe(link_section = ".ram_text.{name}")]"#)
    } else {
        String::new()
    };

    let mut output: TokenStream =
        format!(r#"{placement} #[unsafe(no_mangle)] pub extern "C" fn {name}()"#)
            .parse()
            .expect("the generated prefix is valid Rust");

    output.extend(std::iter::once(TokenTree::Group(body)));

    Ok(output)
}

/// Marks the application entry point.
///
/// The reset handler jumps here once `.data` and `.bss` are initialised, so
/// the function must never return.
///
/// ```ignore
/// #[mg24_hal::main]
/// fn main() -> ! {
///     loop {}
/// }
/// ```
///
/// It expands to a `__mg24_main` symbol, which is the name
/// `mg24_hal::rt::Reset` calls. Defining it twice, or not at all, is a link
/// error rather than a compile error.
#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    match expand(args, input) {
        Ok(tokens) => tokens,
        Err(message) => compile_error(&message),
    }
}

fn expand(args: TokenStream, input: TokenStream) -> Result<TokenStream, String> {
    if !args.is_empty() {
        return Err("`#[mg24_hal::main]` does not take any arguments".into());
    }

    let tokens: Vec<TokenTree> = input.into_iter().collect();

    // Expected shape: [attrs...] fn <name> () -> ! { .. }
    let fn_pos = tokens
        .iter()
        .position(|token| matches!(token, TokenTree::Ident(i) if i.to_string() == "fn"))
        .ok_or("`#[mg24_hal::main]` must be applied to a function")?;

    let name_pos = fn_pos + 1;
    if !matches!(tokens.get(name_pos), Some(TokenTree::Ident(_))) {
        return Err("expected a function name after `fn`".into());
    }

    let params_pos = name_pos + 1;
    match tokens.get(params_pos) {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
            if !group.stream().is_empty() {
                return Err("the entry point cannot take any arguments".into());
            }
        }
        _ => return Err("expected `()` after the function name".into()),
    }

    // The body is the trailing brace group. Re-emitting this exact group is
    // what keeps spans, and therefore diagnostics, pointing at the user's code.
    let body = match tokens.last() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => group.clone(),
        _ => return Err("the entry point must have a body".into()),
    };

    // Whatever sits between `()` and the body has to be the return type, and
    // the only one that makes sense for an entry point is `!`.
    let return_type: String = tokens[params_pos + 1..tokens.len() - 1]
        .iter()
        .map(|token| token.to_string())
        .collect();

    if return_type.replace(' ', "") != "->!" {
        return Err("the entry point must be diverging: `fn main() -> !`".into());
    }

    let mut output: TokenStream = r#"#[unsafe(no_mangle)] pub extern "C" fn __mg24_main() -> !"#
        .parse()
        .expect("the generated prefix is valid Rust");

    output.extend(std::iter::once(TokenTree::Group(body)));

    Ok(output)
}

/// Reports `message` at the macro invocation, rather than panicking and
/// showing the user a proc-macro backtrace.
fn compile_error(message: &str) -> TokenStream {
    format!("::core::compile_error!({message:?});")
        .parse()
        .expect("a compile_error! invocation is valid Rust")
}
