# Contributing Policy

Thank you for your interest in contributing to mg24-hal! This document outlines our contribution guidelines and standards.

## Core Principles

1. **Quality First**: All code must compile without warnings in release mode
2. **Professional Code**: No "illegal" patterns like `let _ =` for result ignoring
3. **Documentation**: Every feature must be documented before merging
4. **Testing**: All examples must work and compile cleanly
5. **Scalability**: Features must be designed with extensibility in mind

## Before You Start

Please review these files:
- `CLAUDE.md` - Project policies and instructions
- `CHANGELOG.md` - Recent changes and patterns
- `docs/` - Feature documentation

## Contribution Process

### 1. Feature Planning

Before implementing:
- Create an issue describing the feature
- Explain the use case and scope
- Reference relevant hardware documentation
- Get feedback on the approach

### 2. Implementation Guidelines

#### Code Style
- Follow Rust conventions and idioms
- Use meaningful variable names
- Keep functions focused and single-purpose
- Implement proper error handling with Result types

#### Safety
- **Never use `let _ =`** to ignore Results - this is forbidden
- Use `.ok()` for explicit result ignoring in examples
- Use proper error propagation with `?` operator
- Document unsafe blocks with explanations
- Unsafe blocks must wrap ONLY the unsafe operation

#### Testing
- All examples must compile in release mode without warnings
- Test on real hardware when possible
- Update existing examples if behavior changes
- Add new examples for new features

### 3. Documentation Requirements

#### For Every Feature

**Human API Documentation** (in docs/):
- Quick start guide with usage examples
- API reference with all types and methods
- Common patterns and best practices
- Troubleshooting section

**Agent API Documentation** (create if needed):
- Complete type signatures
- All error conditions
- Success/failure paths
- Hardware register details

#### Code Documentation
- Every public module has doc comments
- Every public type is documented
- Non-obvious behavior is explained
- Hardware references are cited

### 4. Commit Guidelines

#### Commit Message Format
```
feature/fix/docs: Brief one-line summary

Detailed explanation of changes in this commit.
List the files changed and what they do.

Closes #123 (if applicable)
```

#### Examples of Good Commit Messages
- `feature: Add UART driver with DMA support`
- `fix: Remove unnecessary unsafe blocks in I2C`
- `docs: Add error handling migration guide`

#### Before Committing
```bash
# No warnings in library
cargo build --release

# No warnings in examples
cargo build --release --examples

# All tests pass (if applicable)
cargo test --release
```

### 5. Pull Request Process

1. **Fork and branch** from `main` or `master`
2. **Make your changes** following guidelines above
3. **Test thoroughly** - all examples must compile
4. **Ensure zero warnings** - this is non-negotiable
5. **Update documentation** - docs/ folder must be current
6. **Update CHANGELOG.md** - document your changes
7. **Submit PR** with clear description of changes

### 6. Code Review

Pull requests will be reviewed for:
- ✅ No compiler warnings or errors
- ✅ Follows CLAUDE.md policies
- ✅ Documentation is complete
- ✅ API design is sound and extensible
- ✅ Examples demonstrate the feature
- ✅ Error handling is proper
- ✅ No "illegal" patterns (like `let _ =`)

## Breaking Changes

If your change breaks existing APIs:
1. Document in CHANGELOG.md
2. Provide migration examples in docs/
3. Consider deprecation warnings first
4. Discuss in issue before implementing
5. Increment minor version

## Release Process

After 2-3 features are merged:
1. Update version in Cargo.toml
2. Update CHANGELOG.md with release date
3. Run final testing: `cargo build --release --examples`
4. Tag the release: `git tag v2.x.x`
5. Publish to crates.io: `cargo publish`

## Banned Patterns

These patterns are **forbidden** and will be rejected:

### ❌ Do NOT use
```rust
let _ = some_function_returning_result();  // FORBIDDEN
```

### ✅ Use instead
```rust
some_function_returning_result().ok();     // Explicit ignoring
let result = some_function_returning_result()?;  // Propagate
```

## Getting Help

- **Questions**: Open a discussion issue
- **Bug Reports**: Open an issue with MRE
- **Features**: Start with a discussion
- **Documentation**: Clarify in docs/

## Code of Conduct

Be respectful, constructive, and professional. We're building embedded systems software that people depend on.

## Recognition

Contributors will be recognized in:
- CHANGELOG.md (with your name and contribution)
- GitHub contributors page
- Release notes

Thank you for helping make mg24-hal better! 🚀
