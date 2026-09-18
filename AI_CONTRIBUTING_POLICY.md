# AI Contributing Policy

This document provides guidelines for AI agents (like Claude) contributing to mg24-hal.

## Core Constraints

### Code Quality Mandates
1. **Zero Compiler Warnings**: Every commit must compile cleanly in release mode
2. **No "Illegal" Patterns**: The `let _ =` pattern is **forbidden** in HAL code
3. **Explicit Error Handling**: Use `.ok()` for explicit ignoring, never silent drops
4. **Inline Unsafe**: Only wrap the unsafe operation, not the entire function

### Style Requirements
1. No AI-generated comments indicating authorship (e.g., "// AI generated", "// Created by AI")
2. Professional documentation only - no verbose explanations of obvious code
3. Follow existing code patterns in the repository
4. Use meaningful variable names, not AI-generated placeholders

## Pre-Commit Checklist

Before any commit, an AI agent MUST verify:

```bash
# 1. Library builds cleanly
cargo build --release
# Output must show: Finished `release`... with NO warnings

# 2. All examples compile without warnings
cargo build --release --examples
# Output must show: Finished `release`... with NO warnings

# 3. Check for forbidden patterns
grep -r "let _ =" src/ examples/
# Output must be EMPTY (no results)
```

If any of these fail, the commit must NOT proceed.

## Commit Message Standards

### Format
```
feature/fix/docs: Concise one-line summary

- List specific changes made
- Reference files modified
- Explain why changes needed
- Keep factual and professional

Closing: Include author attribution line at end
```

### Attribution
Every commit MUST include at the end:
```
Co-Authored-By: Claude Haiku 4.5 <noreply@anthropic.com>
```
Or replace with appropriate AI model version.

### Example
```
feature: Add DMA memory-to-memory transfer support

- Implement copy_addr() method in dma.rs
- Add DmaError::OverlappingAddresses validation
- Create dma_m2m.rs example
- Document in DMA_QUICK_START.md

Validates hardware constraints per EFR32MG24 RM section 24.3.
All examples compile without warnings.

Co-Authored-By: Claude Haiku 4.5 <noreply@anthropic.com>
```

## API Documentation Standard

When adding new features, create documentation in **2 files**:

### 1. Human Documentation (docs/)
**Filename**: `FEATURE_QUICK_START.md` or similar

Content:
```markdown
# Feature Name

Brief one-sentence description.

## Quick Example
```rust
// Minimal working code
```

## API Reference
- Method signatures
- Parameter descriptions
- Return types and errors
- Common patterns

## Common Pitfalls
- What often goes wrong
- How to debug
- Reference to full docs
```

### 2. Agent/Developer Documentation
**Filename**: `docs/API_<FEATURE>.md` (if complex) or in code doc comments

Content:
- Every public type's complete signature
- All error variants and when they occur
- Register addresses and bit fields (for hardware)
- Hardware constraints from reference manual
- Success/failure paths
- Examples of each operation

## Error Handling Policy

### Public API Methods
```rust
// ✅ CORRECT: All public methods return Result
pub fn do_something(&mut self) -> Result<(), MyError> {
    // Validate inputs
    // Perform operation
    // Return Ok(()) or Err
}

// ❌ WRONG: Ignoring errors silently
pub fn do_something(&mut self) {
    let _ = risky_operation();  // FORBIDDEN
}
```

### Example Code
```rust
// ✅ ACCEPTABLE: In examples, use .ok() for truly non-critical errors
dma.enable_channel(0).ok();  // Non-critical, demo code

// ✅ BETTER: Handle errors explicitly
match dma.enable_channel(0) {
    Ok(()) => rprintln!("Enabled"),
    Err(e) => rprintln!("Error: {}", e),
}

// ✅ BEST: Use ? for proper error propagation (if main returns Result)
dma.enable_channel(0)?;
```

## Hardware Documentation Requirements

For any feature touching hardware registers:

1. **Reference Manual Citation**: Cite section number and page
   ```rust
   // RM 24.3.1: LDMA Configuration
   ```

2. **Register Addresses**: Document precisely
   ```rust
   const I2C0_RXDATA: u32 = 0x4000_A00C;  // Base 0x4000_A000 + offset 0xC
   ```

3. **Bit Fields**: Show important flags
   ```rust
   // CTRL register bit layout:
   // [0]: AUTOREQ - Hardware request trigger
   // [1]: REQMODE - 0=Event, 1=Block
   ```

4. **Constraints**: Document hardware limitations
   ```rust
   // Max transfer count: 16 bits (0-65535)
   // Min block size: 1, Max: 2048
   ```

## Testing Standard

### Examples Must Compile
```bash
# Every example must compile without warnings
cargo build --release --examples
```

### No Silent Failures
- Example code must demonstrate success/failure clearly
- Use rprintln!() or similar for status output
- Invalid scenarios should show error messages

## Review Checklist

Before submitting any change, verify:

- [ ] No `let _ =` patterns anywhere
- [ ] All public APIs return Result types
- [ ] Zero compiler warnings in release mode
- [ ] Examples compile and demonstrate feature
- [ ] Documentation complete in docs/ folder
- [ ] CHANGELOG.md updated
- [ ] Commit message includes author attribution
- [ ] No AI indicators in code or comments
- [ ] Professional and clear code style
- [ ] Unsafe blocks properly scoped and documented

## Repository Navigation

Key files to understand:
- `CLAUDE.md` - Master project policies
- `src/lib.rs` - Public API surface
- `src/*.rs` - Feature modules
- `examples/*.rs` - Usage examples
- `docs/*.md` - User documentation
- `CHANGELOG.md` - Change history

## When to Ask for Help

Consult the repository owner when:
- Unclear if pattern/approach violates policy
- Unsure about hardware register usage
- Deciding on API design (might conflict with existing patterns)
- Creating new error types or breaking changes
- Documenting complex hardware behavior

## Integration with CLAUDE.md

This policy is supplementary to `CLAUDE.md`. When both apply:
- More specific rule in this file takes precedence for AI
- More specific rule in `CLAUDE.md` applies to all
- When unclear, err on side of quality

## Example Review

### ✅ ACCEPTABLE PR
```
feature: Add RTC driver

Files: src/rtc.rs, examples/rtc_example.rs, docs/RTC_QUICK_START.md

- Implements RTC initialization and alarm support
- Full error handling with RtcError enum
- Hardware validated against RM section 27
- Examples compile without warnings
- Documentation complete

Builds cleanly: cargo build --release
Examples clean: cargo build --release --examples
```

### ❌ REJECTED PR
```
feature: Add RTC driver

- Added RTC support
- Let _ = some_function(); // Suppresses errors quietly
- No documentation
- Warnings in release build
- Examples don't compile
```

## Success Metrics

A contribution is successful when:
1. ✅ Compiles without warnings (`cargo build --release`)
2. ✅ Examples compile without warnings (`cargo build --release --examples`)
3. ✅ All public APIs return Result types
4. ✅ Documentation complete and professional
5. ✅ No "illegal" patterns present
6. ✅ Commit message proper and attributed
7. ✅ CHANGELOG.md updated
8. ✅ Code follows repository style

---

**Version**: 1.0  
**Last Updated**: 2026-09-18  
**Applies To**: AI agents and automated systems
