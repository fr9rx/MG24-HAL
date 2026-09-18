# Changelog

All notable changes to the mg24-hal project are documented in this file.

## [2.0.0] - 2026-09-18

### Released
- Published mg24-hal v2.0.0 to crates.io
- Published mg24-hal-macros v2.0.0 to crates.io

### Added

#### DMA Module (LDMA - Linked Direct Memory Access)
- Complete implementation of 8 independent DMA channels
- Safe reference-based API: `copy_slice()`, `i2c0_rx_slice()`, etc.
- Address-based API for advanced use cases: `copy_addr()`, `configure_transfer()`
- Memory-to-memory, peripheral-to-memory, and memory-to-peripheral transfers
- Full integration with I2C, UART, ADC peripherals
- Packed/unpacked data format conversion capabilities
- Multi-channel concurrent transfer support
- Complete register verification against EFR32MG24 reference manual

#### Error Handling System
- `DmaError` with 7 variants: InvalidChannel, InvalidTransferCount, InvalidBlockSize, OverlappingAddresses, TransferCountTooLarge, InvalidSize, UnsupportedOperation
- `I2cError` enhancements: Address validation for 7-bit addressing, ArbitrationLost, NoAck, BusError, BusHeld, Timeout, InvalidAddress
- `GpioError` for future configuration validation
- All public APIs return Result types instead of panicking

#### Embedded-HAL 1.0 Integration
- GPIO: ErrorType, InputPin, OutputPin, StatefulOutputPin
- Delay: DelayNs trait
- I2C: ErrorType, Error trait with kind mapping, address validation

#### Documentation
- DMA_QUICK_START.md: One-liner examples and API reference
- DMA_IMPLEMENTATION.md: Architecture and register details
- DMA_EXAMPLES_GUIDE.md: Progressive examples from beginner to advanced
- ERROR_CHECKING.md: Comprehensive error handling guide
- PUBLISHING_GUIDE.md: Crates.io publication process
- SEPARATE_MACROS_REPO.md: Guide for separate macros repository
- v2.0.0_RELEASE_NOTES.md: Complete release notes
- Updated README.md with v2.0.0 features

#### Examples
- dma_m2m.rs: Memory-to-memory transfers (safe API)
- dma_unsafe_addr.rs: Address-based API usage
- dma_advanced.rs: Concurrent multi-channel transfers
- dma_peripherals.rs: Peripheral integration reference
- i2c_dma.rs: I2C with DMA support
- i2c_scan.rs: I2C bus scanning
- All other existing examples updated for error handling

### Changed

#### API Breaking Changes
- All DMA methods now return `Result<T, DmaError>`
- I2C address validation rejects invalid 8-bit addresses
- All Result-returning methods must be handled or explicitly ignored with `.ok()`

#### Code Quality
- Removed 46 compiler warnings from library
- Removed unnecessary unsafe blocks from I2C register operations
- Proper inline unsafe blocks for hardware register writes only
- Zero compiler warnings in release builds

#### Testing & Verification
- All examples compile without warnings
- Dry-run verification passed before publication
- Both crates published successfully to crates.io

### Fixed

#### Compiler Warnings (46 → 0)
- Removed unnecessary unsafe blocks in I2c0/I2c1 methods
- Fixed register read/write safety patterns
- Removed unused Result warnings from examples
- Added allow(dead_code) for intentionally unused fields

#### Example Compilation
- Fixed Result handling in all DMA examples
- Updated is_transfer_done() calls to use unwrap_or(false)
- Replaced let _ = patterns with .ok() idiom per policy

### Documentation Files

#### Root Level
- CHANGELOG.md: This file, commit history with dates and changes
- CONTRIBUTING_POLICY.md: Human contributor guidelines
- AI_CONTRIBUTING_POLICY.md: AI agent contribution guidelines
- CLAUDE.md: Project policies and instructions (checked into repo)

#### docs/ Folder
- README.md: Main feature documentation
- DMA_QUICK_START.md: Quick reference for DMA API
- DMA_IMPLEMENTATION.md: Architecture and register documentation
- DMA_EXAMPLES_GUIDE.md: Progressive example walkthrough
- ERROR_CHECKING.md: Error handling patterns and migration guide
- PUBLISHING_GUIDE.md: Publication process for crates.io
- SEPARATE_MACROS_REPO.md: Guide for separate macros repository

#### .gitignore
- internal_docs/: Folder for copyrighted/non-committable documentation

## Previous Releases

### [1.0.0] - 2026-09
Initial release with GPIO, I2C, and basic peripherals support.

---

## Maintenance Notes

- **Commit Messages**: Follow format "feature/fix: description" with clear changeset
- **Documentation**: Every feature must be documented in docs/ folder
- **Testing**: All examples must compile without warnings in release mode
- **Publication**: After 2-3 features, republish to crates.io
- **API Changes**: Document breaking changes in release notes
- **Code Quality**: Zero compiler warnings required before commits
