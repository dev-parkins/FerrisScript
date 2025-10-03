# Test Coverage Setup Notes

## Windows File Locking Issues with Tarpaulin

**Date**: October 2, 2025  
**Issue**: `cargo-tarpaulin` experiences file locking issues on Windows (OS error 32)

### Problem
Tarpaulin attempts to clean the build directory but encounters locked files, likely held by:
- rust-analyzer (VS Code extension)
- Other IDE processes
- Background compilation processes

### Attempted Solutions
1. ✅ Used `--skip-clean` flag - Still hit file locks during test execution
2. ❌ Default execution - Failed at cargo clean step

### Recommended Approaches for Windows

#### Option 1: Use llvm-cov (Native Rust Solution)
```powershell
# Install llvm-cov component
rustup component add llvm-tools-preview

# Install cargo-llvm-cov
cargo install cargo-llvm-cov

# Run coverage
cargo llvm-cov --workspace --html
cargo llvm-cov --workspace --lcov --output-path lcov.info
```

**Pros**:
- Native Rust tooling
- Better Windows compatibility
- No file locking issues
- Faster execution

**Cons**:
- Requires nightly features for some advanced options
- Different output format than tarpaulin

#### Option 2: Close IDE and Run Tarpaulin
```powershell
# 1. Close VS Code completely
# 2. Kill rust-analyzer if running
Get-Process rust-analyzer -ErrorAction SilentlyContinue | Stop-Process

# 3. Run tarpaulin
cargo tarpaulin --workspace --out Html --out Lcov --out Stdout --output-dir target/coverage
```

**Pros**:
- Uses tarpaulin as configured
- Consistent with tarpaulin.toml

**Cons**:
- Requires closing IDE
- Inconvenient for development workflow

#### Option 3: Use CI for Coverage (Recommended for Windows Dev)
Run coverage in CI (Linux environment) where file locking is not an issue, and focus local development on tests only.

```powershell
# Local development: Just run tests
cargo test --workspace

# Coverage: Let CI handle it (GitHub Actions on ubuntu-latest)
```

**Pros**:
- No local file locking issues
- CI generates reports
- Better for team collaboration

**Cons**:
- Can't generate coverage locally
- Slower feedback loop

### Decision for FerrisScript

**For this branch**: Will switch to `cargo-llvm-cov` for better Windows compatibility.

**Rationale**:
- Native Rust solution
- No file locking issues
- Can generate coverage locally
- Still integrates with CI
- Outputs LCOV format for compatibility

### Migration Steps
1. Install cargo-llvm-cov
2. Update coverage scripts to use llvm-cov
3. Update tarpaulin.toml → .cargo/config.toml (if needed)
4. Test local coverage generation
5. Update CI workflow
6. Document in README

---

*Note: This issue is tracked in development learnings for future reference.*
