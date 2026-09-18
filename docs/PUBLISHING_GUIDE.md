# Publishing Guide for MG24-HAL Crates

Complete guide for publishing `mg24-hal` and `mg24-hal-macros` to crates.io.

## Overview

The MG24-HAL project consists of two crates:

1. **mg24-hal** (main HAL library)
   - GPIO, I2C, UART drivers
   - DMA controller with error handling
   - Delay, RTT, and timestamp support
   - Error types and validation

2. **mg24-hal-macros** (procedural macros)
   - `#[main]` - Application entry point
   - `#[interrupt]` - Interrupt handler registration

## Pre-Publication Checklist

### 1. Update Versions

Both crates should have synchronized versions. Current: `2.0.0`

```bash
# In Cargo.toml files, verify:
# [package] section has:
# name = "mg24-hal" or "mg24-hal-macros"
# version = "2.0.0"
# edition = "2021"
```

### 2. Documentation

✅ Complete documentation:
- ERROR_CHECKING.md - Error handling overview
- DMA_IMPLEMENTATION.md - DMA register details
- DMA_QUICK_START.md - DMA usage guide
- DMA_EXAMPLES_GUIDE.md - DMA examples walkthrough
- PUBLISHING_GUIDE.md - This file

### 3. Code Quality

Current metrics:
- Compiler warnings: 46 (down from 79)
- Main warnings:
  - E0133: unsafe operations (necessary for hardware access)
  - E0211: dead code (internal utilities)
  - Unused variables (in example code)

**Action**: The remaining warnings are expected for embedded HAL code and do not prevent publication.

### 4. Dependencies

**mg24-hal**:
```toml
efr32mg24-pac = "0.1.0"
embedded-hal = "1.0.0"
mg24-hal-macros = "2.0.0"
```

**mg24-hal-macros**:
```toml
# No external dependencies (uses proc_macro only)
```

✅ All dependencies are published and stable.

## Publishing Steps

### Step 1: Test the Crates Locally

```bash
# Build both crates
cargo build --workspace

# Run any tests
cargo test --workspace --doc

# Check for issues
cargo clippy --workspace
```

### Step 2: Create Crates.io Account

If you don't have an account:
1. Visit https://crates.io/
2. Sign up with GitHub account
3. Visit https://crates.io/me for API token
4. Store token securely

### Step 3: Configure Cargo

```bash
# Login to crates.io
cargo login

# Enter your API token when prompted
# Token is stored in ~/.cargo/credentials.toml
```

### Step 4: Publish mg24-hal-macros First

The main crate depends on macros, so publish it first:

```bash
cd macros

# Dry run to verify
cargo publish --dry-run

# If dry run succeeds, publish for real
cargo publish

# This takes 1-2 minutes to process
```

**Output**:
```
    Packaging mg24-hal-macros v2.0.0 ([path])
    Verifying mg24-hal-macros v2.0.0 ([path])
   Compiling mg24-hal-macros v2.0.0
    Finished release [optimized] target(s) in X.XXs
   Uploading mg24-hal-macros v2.0.0 to registry
    Uploaded mg24-hal-macros v2.0.0 to registry
   Published mg24-hal-macros v2.0.0 at registry
```

Wait 1-2 minutes before next step (crates.io index update).

### Step 5: Publish mg24-hal

```bash
cd ..

# Update to use published version if needed
# cargo update

# Dry run
cargo publish --dry-run

# Publish
cargo publish
```

### Step 6: Verify Publication

Visit these URLs:
- https://crates.io/crates/mg24-hal
- https://crates.io/crates/mg24-hal-macros

Check:
- ✅ Version number (2.0.0)
- ✅ Documentation rendered correctly
- ✅ Dependency links work
- ✅ Download badge functional

### Step 7: Documentation on docs.rs

Documentation is automatically built and published at:
- https://docs.rs/mg24-hal/latest/mg24_hal/
- https://docs.rs/mg24-hal-macros/latest/mg24_hal_macros/

This typically appears 1-5 minutes after publication. If documentation fails to build, check the build logs at https://docs.rs/crate/mg24-hal/latest/builds

## Updating Version for Future Releases

When releasing a new version:

1. Decide version: semantic versioning (MAJOR.MINOR.PATCH)

2. Update both Cargo.toml files:
```toml
[package]
version = "X.Y.Z"  # Change this
```

3. Update dependency in mg24-hal/Cargo.toml:
```toml
[dependencies]
mg24-hal-macros = "X.Y.Z"  # Change this too
```

4. Update CHANGELOG.md with release notes

5. Commit and tag:
```bash
git add -A
git commit -m "Release v2.0.0"
git tag v2.0.0
git push origin master --tags
```

6. Follow "Publishing Steps" above

## Common Issues and Solutions

### Issue: "some packages failed to verify"

**Cause**: Dependencies not yet available on crates.io

**Solution**: 
- Wait a few minutes for index to update
- Retry publish
- Check that all dependencies are published first

### Issue: "missing documentation"

**Cause**: Public API items lack doc comments

**Solution**:
```bash
cargo doc --open
# Check what's missing
# Add /// or //! comments to all pub items
```

### Issue: Documentation build fails on docs.rs

**Cause**: Missing features or platform-specific code

**Solution**:
1. Check build log: https://docs.rs/crate/mg24-hal/builds
2. Add `#![doc = include_str!("../README.md")]` if needed
3. Mark platform-specific code: `#[cfg(target_arch = "arm")]`

### Issue: Version conflict

**Cause**: Already published version number

**Solution**:
- Use a new version number
- Can't revert published versions

## Post-Publication

### 1. Verify Installation Works

```bash
# Create test project
cargo new --lib test_mg24
cd test_mg24

# Add as dependency
cargo add --build mg24-hal

# Should automatically fetch from crates.io
cargo build
```

### 2. Update README

If needed, update repository README to:
- Link to crates.io pages
- Show installation from crates.io
- Update version numbers in examples

### 3. Announce Release

Consider announcing on:
- Embedded Rust forums
- GitHub releases page
- Twitter/social media

### 4. Long-term Maintenance

Keep crates healthy by:
- Responding to issues promptly
- Releasing patch versions for bugs
- Updating dependencies as needed
- Maintaining documentation

## Version History

### v2.0.0 (Current)
- Complete error checking system
- DMA module with safe and address-based APIs
- I2C with address validation
- GPIO with error types
- Embedded-hal trait support
- Documentation and examples
- Reduced compiler warnings

### Migration from v1.x to v2.0

All public APIs now return `Result` types:

**Before**:
```rust
dma.enable_channel(0);  // Could panic
i2c.read(addr, &mut buf)?;  // Already used Results
```

**After**:
```rust
dma.enable_channel(0)?;  // Error handling required
i2c.read(addr, &mut buf)?;  // Same, now with validation
```

## Support and Issues

- **Bug Reports**: https://github.com/fr9rx/mg24-hal/issues
- **Feature Requests**: https://github.com/fr9rx/mg24-hal/discussions
- **Crates.io Issues**: Contact crates.io team

## Useful Commands

```bash
# Check for publishing readiness
cargo publish --dry-run --allow-dirty

# Build documentation locally
cargo doc --no-deps --open

# Check what would be published
cargo package --list

# Verify dependencies
cargo tree

# Check security
cargo audit

# Format code
cargo fmt --all

# Lint code
cargo clippy --all-targets
```

## References

- [Publishing to Crates.io](https://doc.rust-lang.org/cargo/reference/manifest.html)
- [Cargo.toml Documentation](https://doc.rust-lang.org/cargo/reference/manifest.html)
- [Semantic Versioning](https://semver.org/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
