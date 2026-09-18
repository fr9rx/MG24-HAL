# Setting Up mg24-hal-macros as Separate Repository

Complete guide to create and maintain `mg24-hal-macros` as a separate GitHub repository.

## Current Status

✅ mg24-hal-macros published on crates.io
✅ README.md created for macros crate
✅ All code ready for separate repo

## Steps to Create Separate Repository

### 1. Create New Repository on GitHub

1. Go to https://github.com/new
2. Create new repository:
   - **Repository name**: `mg24-hal-macros`
   - **Description**: "Procedural macros for mg24-hal"
   - **Visibility**: Public
   - **Initialize**: No (we'll push existing code)
   - Click "Create repository"

3. Note the repository URL: `https://github.com/fr9rx/mg24-hal-macros.git`

### 2. Create Worktree for Separate Repo

```powershell
cd C:\Users\LENOVO\Projects\MG24-HAL

# Create a new worktree for the macros repo
git worktree add -b macros-main ..\mg24-hal-macros-git main
```

### 3. Filter and Copy Macros-Only History

```powershell
cd ..\mg24-hal-macros-git

# Filter git history to only include macros directory
git filter-branch --subdirectory-filter macros -- --all

# Remove filter-branch backups
git reflog expire --expire=now --all
git gc --aggressive --prune=now
```

### 4. Add Remote and Push to New Repository

```powershell
git remote add origin https://github.com/fr9rx/mg24-hal-macros.git

# Push all branches and tags
git push -u origin main
git push --tags
```

### 5. Verify New Repository

Check https://github.com/fr9rx/mg24-hal-macros:
- ✅ All commits present
- ✅ README.md visible
- ✅ src/lib.rs present
- ✅ Cargo.toml present
- ✅ Tags visible

### 6. Clean Up Worktree

```powershell
cd C:\Users\LENOVO\Projects\MG24-HAL

# Remove the temporary worktree
git worktree remove ..\mg24-hal-macros-git

# Optional: Remove backup references
git reflog expire --all
```

## Repository Structure

```
mg24-hal-macros/
├── Cargo.toml          # Package configuration
├── src/
│   └── lib.rs          # Macro implementations
└── README.md           # Documentation
```

## File Configuration

### Cargo.toml Highlights

```toml
[package]
name = "mg24-hal-macros"
version = "2.0.0"
edition = "2021"
description = "Procedural macros for mg24-hal"
repository = "https://github.com/fr9rx/mg24-hal-macros"

[lib]
proc-macro = true

# No external dependencies!
[dependencies]
```

### README.md Structure

✅ Already created with:
- Crates.io badge
- Documentation badge
- License badge
- Feature descriptions
- Usage examples
- Related links

## Maintenance Strategy

### Publishing Updates

When releasing a new version:

1. Update version in both repositories:
   ```toml
   # In mg24-hal-macros/Cargo.toml
   version = "X.Y.Z"
   
   # In mg24-hal/Cargo.toml (dependency)
   mg24-hal-macros = "X.Y.Z"
   ```

2. Publish macros first:
   ```powershell
   cd mg24-hal-macros
   cargo publish
   ```

3. Publish main crate:
   ```powershell
   cd mg24-hal
   cargo publish
   ```

### Keep Repos in Sync

Two options:

**Option A: Monorepo Source of Truth (Recommended)**
- Keep macros in `mg24-hal/macros/`
- Use git subtree to push to separate repo:
  ```powershell
  git subtree push --prefix macros https://github.com/fr9rx/mg24-hal-macros.git main
  ```

**Option B: Separate Repo Source**
- Maintain macros in separate repo
- Pull updates into mg24-hal as dependency
- More complex, but clearer separation

### Recommended: Hybrid Approach

1. Keep macros in `mg24-hal/macros/` (single source of truth)
2. Use git subtree to push to separate repository:
   ```powershell
   # After committing changes to macros/
   git subtree push --prefix macros origin macros-main
   ```
3. Users can clone either:
   - Full HAL: `git clone https://github.com/fr9rx/mg24-hal`
   - Just macros: `git clone https://github.com/fr9rx/mg24-hal-macros`

## Documentation Updates

Update repository descriptions on GitHub:

**mg24-hal**:
- "EFR32MG24 HAL with DMA, I2C, GPIO, error handling"
- Website: https://docs.rs/mg24-hal/
- Topics: `embedded`, `hal`, `efr32`, `cortex-m33`

**mg24-hal-macros**:
- "Procedural macros for mg24-hal entry point and interrupts"
- Website: https://docs.rs/mg24-hal-macros/
- Topics: `embedded`, `macros`, `efr32`, `procedural-macro`

## CI/CD Considerations

### GitHub Actions for Separate Repo

Create `.github/workflows/ci.yml`:

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test
      - run: cargo doc --no-deps

  publish:
    needs: test
    if: startsWith(github.ref, 'refs/tags/v')
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo publish --token ${{ secrets.CARGO_TOKEN }}
```

## Linking Repositories

Update READMEs to cross-link:

**mg24-hal/README.md**:
```markdown
## Procedural Macros

Macros are in a [separate repository](https://github.com/fr9rx/mg24-hal-macros):
- [`mg24-hal-macros`](https://crates.io/crates/mg24-hal-macros) on crates.io
```

**mg24-hal-macros/README.md**:
```markdown
## Main HAL

Part of the [mg24-hal](https://github.com/fr9rx/mg24-hal) project.
```

## Troubleshooting

### Issue: "remote already exists"

```powershell
git remote rm origin
git remote add origin https://github.com/fr9rx/mg24-hal-macros.git
```

### Issue: "filter-branch is slow"

For large histories, use `git-filter-repo`:

```powershell
pip install git-filter-repo
git-filter-repo --subdirectory-filter macros
```

### Issue: Lost commits after filter-branch

Don't worry - they're in reflog:

```powershell
git reflog
# Find commit hash
git reset --hard <commit-hash>
```

## Final Checklist

After creating the separate repository:

- [ ] Repository created on GitHub
- [ ] Code pushed with history
- [ ] README visible and formatted correctly
- [ ] Tags pushed
- [ ] Crates.io links updated
- [ ] Cross-links in both READMEs
- [ ] GitHub repository description set
- [ ] Topics/keywords added
- [ ] License file visible
- [ ] CI/CD configured (if desired)
- [ ] Branch protection rules set (if desired)

## Timeline

- **5 min**: Create GitHub repository
- **10 min**: Filter and push history  
- **5 min**: Update documentation and links
- **Total**: ~20 minutes

---

## Quick Reference

```powershell
# Create new repo, filter history, and push
git worktree add -b macros-main ..\macros-repo main
cd ..\macros-repo
git filter-branch --subdirectory-filter macros -- --all
git remote add origin https://github.com/fr9rx/mg24-hal-macros.git
git push -u origin main --all --tags
cd ..\mg24-hal
git worktree remove ..\macros-repo
```

Done! The macros crate is now a separate, independently cloneable repository.
