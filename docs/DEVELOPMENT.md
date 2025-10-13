# FerrisScript 🦀 Development Guide

This guide covers development setup, workflows, and contribution guidelines for FerrisScript.

---

## 🚀 Quick Start

### Prerequisites

- **Rust 1.70+** (install via [rustup](https://rustup.rs/))
  - Currently tested on Rust 1.90.0 (stable)
  - Uses Rust 2021 edition
- **Godot 4.2+** (for testing GDExtension integration)
- **Git** for version control

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/dev-parkins/FerrisScript.git
cd FerrisScript

# Build all crates
cargo build --workspace

# Run all tests (96 tests)
cargo test --workspace

# Build optimized release
cargo build --workspace --release
```

### Quick Commands

```bash
# Build specific crate
cargo build -p ferrisscript_compiler
cargo build -p ferrisscript_runtime
cargo build -p ferrisscript_godot_bind

# Run tests with output
cargo test --workspace -- --show-output

# Check code formatting
cargo fmt --all -- --check

# Run clippy linter (strict - treats warnings as errors)
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Generate documentation
cargo doc --workspace --open
```

### Cross-Platform Builds

FerrisScript supports building for multiple platforms. The CI automatically builds native binaries for Linux, macOS, and Windows.

**Native Build (Recommended)**:

```bash
# Build for your current platform (always works)
cargo build --workspace --release
```

**Cross-Compilation Setup**:

⚠️ **Note**: Cross-compilation from Windows requires platform-specific linkers and is complex to set up. For most development, use native builds or rely on CI for multi-platform builds.

If you need to verify target compatibility locally:

```bash
# Install target platform support
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Verify compilation (may fail at linking stage without proper linkers)
cargo build --workspace --release --target x86_64-unknown-linux-gnu
```

**CI Builds**: The GitHub Actions workflow automatically builds native binaries for:

- Linux x86_64 (`x86_64-unknown-linux-gnu`)
- Windows x86_64 (`x86_64-pc-windows-msvc`)
- macOS x86_64 (`x86_64-apple-darwin`)

See `.github/workflows/ci.yml` for the full build matrix.

---

## 📁 Project Structure

```
ferrisscript/
├── .github/workflows/       # CI/CD automation
├── crates/
│   ├── compiler/            # Lexer, parser, type checker (69 tests)
│   ├── runtime/             # AST interpreter (26 tests)
│   └── godot_bind/          # GDExtension integration (1 test)
├── docs/
│   ├── archive/             # Version-specific documentation
│   └── DEVELOPMENT.md       # This file
├── examples/                # 11 example .ferris scripts
├── godot_test/              # Test Godot project
├── ARCHITECTURE.md          # Technical design documentation
├── LICENSE                  # MIT License
├── README.md                # Main project documentation
└── RELEASE_NOTES.md         # v0.0.1 release information
```

### Crate Responsibilities

| Crate | Purpose | Lines of Code | Tests |
|-------|---------|---------------|-------|
| `ferrisscript_compiler` | Tokenization, parsing, type checking | ~1,500 | 69 |
| `ferrisscript_runtime` | AST interpretation, execution engine | ~1,200 | 26 |
| `ferrisscript_godot_bind` | Godot 4.x GDExtension integration | ~800 | 1 |

---

## 🔧 Development Workflow

### 1. Choose a Task

Check the roadmap in [ARCHITECTURE.md](ARCHITECTURE.md) or create an issue for new features.

### 2. Create a Branch

Follow our **branch naming convention** to get the right PR template automatically:

```bash
# For bug fixes → Bug Fix PR template
git checkout -b bugfix/your-bug-description
# or
git checkout -b fix/parser-null-pointer

# For new features → Feature PR template
git checkout -b feature/your-feature-name
# or
git checkout -b feat/async-loading

# For documentation → Documentation PR template
git checkout -b docs/add-api-examples
# or
git checkout -b doc/update-readme
```

**Why Branch Naming Matters:**

- 🤖 **Auto-applies PR templates**: Correct template based on branch prefix
- 📋 **Better organization**: Easy to identify PR types at a glance
- ✅ **GitHub Copilot friendly**: Automated PRs get proper templates

See [CONTRIBUTING.md](../CONTRIBUTING.md#branch-naming-convention) for full details.

### 3. Make Changes

- Write code following Rust conventions
- Add tests for new functionality
- Update documentation if needed

### 4. Test Your Changes

**📚 See [testing/README.md](testing/README.md) for comprehensive testing documentation**

```bash
# Run all tests (843+ tests across all layers)
cargo test --workspace

# Run specific test types
cargo test -p ferrisscript_compiler    # Unit tests (compiler)
cargo test -p ferrisscript_runtime     # Unit tests (runtime)
ferris-test --all                      # Integration tests (.ferris scripts)

# Run specific test
cargo test test_compile_hello

# Check formatting and linting
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

**Testing Layers**:

1. **Unit Tests (Rust)** - Pure logic testing (compiler/runtime)
2. **Integration Tests (.ferris)** - End-to-end testing with Godot
3. **GDExtension Tests** - Godot bindings requiring runtime
4. **Benchmark Tests** - Performance measurement

**Quick Links**:

- [Testing Guide](testing/TESTING_GUIDE.md) - Complete testing patterns ⭐ **START HERE**
- [Test Matrices](testing/README.md#test-matrices) - Systematic coverage tracking
- [Test Harness](testing/TEST_HARNESS_TESTING_STRATEGY.md) - ferris-test CLI architecture

### 4.5. Validate Documentation Changes

**If you modified any `.md` files**, always run documentation checks before committing:

```bash
# Option 1: VS Code Task (Recommended)
# Install dependencies (first time only)
npm install

# Check markdown formatting
npm run docs:lint

# Auto-fix formatting issues
npm run docs:fix
```

**Why This Matters**:

- ⏱️ **Faster PR reviews**: Catch formatting issues before CI runs
- **Consistent style**: Maintain professional documentation quality
- ✅ **CI will pass**: Same checks run in CI, but you catch them early

**Common Issues Caught**:

- Missing blank lines around headings
- Inconsistent list formatting
- Trailing whitespace
- Code blocks without language specifiers

**Link Checking**: Broken links are automatically checked in CI. To check links manually:

```bash
# Check specific file
npx markdown-link-check your-file.md

# Check with config (quieter output)
npx markdown-link-check your-file.md --config .markdown-link-check.json -q
```

See [../scripts/README.md](../scripts/README.md) for full documentation linting guide.

### 5. Commit with Conventional Commits

```bash
# Feature additions
git commit -m "feat(compiler): add support for array types"

# Bug fixes
git commit -m "fix(runtime): handle division by zero correctly"

# Documentation
git commit -m "docs: update README with new examples"

# Tests
git commit -m "test(parser): add tests for field access"

# Maintenance
git commit -m "chore: update dependencies to latest versions"
```

### 6. Push and Create PR

```bash
# Push your branch (use the actual branch name you created)
git push origin bugfix/your-bug-description
# or
git push origin feature/your-feature-name
# or
git push origin docs/your-doc-update

# Then create a pull request on GitHub
```

**What Happens Next:**

1. 🤖 Our automation detects your branch name
2. 📋 Appropriate PR template is auto-applied to your PR
3. ✍️ Fill out the template sections (marked with `<!-- ... -->` comments)
4. ✅ CI runs tests and checks
5. 👀 Maintainers review and provide feedback

**For GitHub Copilot Users:**
If you're using GitHub Copilot to create PRs automatically, the branch naming convention ensures your automated PRs get the correct template applied!

---

## 🧪 Testing

### Test Organization

- **Unit tests**: In each module (e.g., `lexer.rs`, `parser.rs`)
- **Integration tests**: In `tests/` directories
- **Example tests**: Validate `.ferris` files compile correctly

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p ferrisscript_compiler

# Specific module
cargo test --lib lexer::tests

# With output (see println! statements)
cargo test -- --show-output --nocapture

# Single test
cargo test test_compile_hello -- --exact
```

### Adding Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_feature() {
        let result = your_function();
        assert_eq!(result, expected_value);
    }
}
```

---

## � Code Coverage

FerrisScript maintains high test coverage to ensure code quality and catch regressions early. This section explains how to generate, view, and interpret coverage reports.

### Overview: Two Coverage Tools

We use different coverage tools for different environments:

| Tool | Environment | Purpose | Platform |
|------|-------------|---------|----------|
| **cargo-llvm-cov** | Local Development | Interactive development, Windows-compatible | Windows, macOS, Linux |
| **cargo-tarpaulin** | CI/CD (GitHub Actions) | Automated coverage in CI pipelines | Linux only |

**Why two tools?**

- **cargo-llvm-cov**: Native LLVM-based coverage, no file locking issues on Windows, faster local execution
- **cargo-tarpaulin**: Mature CI integration, generates Codecov-compatible reports, but has Windows file locking issues

See [infrastructure/COVERAGE_SETUP_NOTES.md](infrastructure/COVERAGE_SETUP_NOTES.md) for the technical investigation that led to this approach.

### Running Coverage Locally

#### Quick Method: Use Coverage Scripts

```bash
# PowerShell (Windows)
.\scripts\coverage.ps1

# Bash (Linux/macOS)
./scripts/coverage.sh
```

These scripts will:

1. Check if `cargo-llvm-cov` is installed (auto-install if missing)
2. Run coverage analysis on all workspace crates
3. Generate HTML and LCOV reports in `target/coverage/`
4. Display report locations

#### Manual Method: Direct Commands

```bash
# First time setup
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov

# Generate HTML report (human-readable)
cargo llvm-cov --workspace --html --output-dir target/coverage

# Generate LCOV report (for tools like VSCode extensions)
cargo llvm-cov --workspace --lcov --output-path target/coverage/lcov.info

# Generate both formats at once
cargo llvm-cov --workspace --html --output-dir target/coverage
cargo llvm-cov --workspace --lcov --output-path target/coverage/lcov.info

# Coverage for specific crate
cargo llvm-cov -p ferrisscript_compiler --html

# Coverage with test output visible
cargo llvm-cov --workspace --html -- --nocapture
```

### Viewing Coverage Reports

**HTML Report** (most user-friendly):

```powershell
# Windows
Invoke-Item target/coverage/html/index.html

# Linux
xdg-open target/coverage/html/index.html

# macOS
open target/coverage/html/index.html
```

The HTML report shows:

- Overall coverage percentage
- Per-file coverage breakdown
- Line-by-line coverage visualization (green = covered, red = not covered)
- Branch coverage information

**LCOV Report** (for tool integration):

The `target/coverage/lcov.info` file can be used by:

- VS Code extensions (e.g., "Coverage Gutters")
- CI/CD systems (e.g., Codecov, Coveralls)
- Code review tools

### Understanding Coverage Metrics

Coverage reports show several metrics:

| Metric | Description | Target |
|--------|-------------|--------|
| **Line Coverage** | % of code lines executed by tests | 80%+ |
| **Branch Coverage** | % of conditional branches tested | 75%+ |
| **Function Coverage** | % of functions called by tests | 90%+ |

**Color Coding in HTML Reports**:

- 🟢 **Green**: Line is covered by tests
- 🔴 **Red**: Line is not covered by tests
- 🟡 **Yellow**: Line is partially covered (some branches not tested)
- ⚪ **Gray**: Line is not executable (comments, blank lines)

### Coverage in CI/CD

**GitHub Actions Workflow**:

Coverage is automatically generated on every push to `main` and in pull requests:

```yaml
# .github/workflows/ci.yml
coverage:
  name: Code Coverage
  runs-on: ubuntu-latest
  steps:
    - name: Generate coverage
      run: cargo tarpaulin --workspace --out Xml --output-dir coverage
    
    - name: Upload to Codecov
      uses: codecov/codecov-action@v4
```

**Why Tarpaulin in CI?**

- Well-established in Rust CI pipelines
- Generates Cobertura XML for Codecov
- No file locking issues on Linux runners
- GitHub Actions uses `ubuntu-latest` (Linux)

**Viewing CI Coverage**:

- Check the "Code Coverage" job in GitHub Actions
- View detailed reports on Codecov (if configured)
- Coverage badge in README.md (future)

### Coverage Configuration

**tarpaulin.toml**:

```toml
[tool]
out = ["Html", "Lcov", "Stdout"]
output-dir = "target/coverage"
workspace = true
timeout = "5m"
follow-exec = true
count = true
fail-under = 0  # Currently no enforcement, will increase as coverage improves

[report]
branches = true
lines = true
```

**What's Excluded**:

- Test files (`*/tests/*`, `*_test.rs`)
- Generated code
- External dependencies

### Coverage Goals and Thresholds

**Current Status**:

- **Test Count**: 116+ tests (and growing)
- **Coverage**: Actively tracked via cargo-llvm-cov locally and cargo-tarpaulin in CI
- **Enforcement**: No minimum threshold yet (`fail-under = 0`)
- **Historical Baseline**: See version-specific archives for baseline snapshots

**Target Goals**:

- **Overall coverage**: 80% line coverage
- **Compiler crate**: 85%+ (critical path)
- **Runtime crate**: 85%+ (critical path)
- **Godot bind**: 70%+ (harder to test, requires Godot)

**For New Code**:

- All new features should include tests
- Aim for 80%+ coverage on modified files
- Test both success and error paths
- Include edge cases and boundary conditions

### Troubleshooting Coverage Issues

#### Issue: "cargo-llvm-cov not found"

**Solution**:

```bash
# Install llvm-tools-preview component
rustup component add llvm-tools-preview

# Install cargo-llvm-cov
cargo install cargo-llvm-cov

# Verify installation
cargo llvm-cov --version
```

#### Issue: Windows File Locking (OS Error 32)

**Solution**: Use `cargo-llvm-cov` instead of `cargo-tarpaulin`:

```powershell
# Don't use tarpaulin on Windows
# cargo tarpaulin  # ❌ May fail with file locks

# Use llvm-cov instead
cargo llvm-cov --workspace --html  # ✅ Works on Windows
```

**Why this happens**:

- Tarpaulin tries to clean the build directory
- rust-analyzer or VS Code may lock files
- llvm-cov doesn't have this issue

**If you must use tarpaulin on Windows**:

```powershell
# Close VS Code and all IDEs
# Kill rust-analyzer process
Get-Process rust-analyzer -ErrorAction SilentlyContinue | Stop-Process

# Then run tarpaulin
cargo tarpaulin --workspace --skip-clean
```

#### Issue: Coverage Seems Low

**Check**:

1. **Are tests running?** `cargo test --workspace` should show test execution
2. **Are tests in the right place?** Tests should be in `#[cfg(test)]` modules or `tests/` directories
3. **Are features enabled?** Some code may be behind feature flags
4. **Are examples included?** Example files aren't included in coverage by default

**Improve coverage**:

```rust
// Add tests for error paths
#[test]
#[should_panic(expected = "expected error message")]
fn test_error_handling() {
    // Test code that should panic
}

// Test edge cases
#[test]
fn test_boundary_conditions() {
    // Test empty input, maximum values, etc.
}
```

#### Issue: CI Coverage Differs from Local

**This is normal**:

- CI uses tarpaulin (Linux)
- Local uses llvm-cov (cross-platform)
- Different coverage implementations may have small differences (1-3%)

**If differences are large (>5%)**:

- Check if CI is running all tests: `cargo test --workspace`
- Verify both are using `--workspace` flag
- Check for platform-specific code (`#[cfg(windows)]`, etc.)

### Advanced Coverage Techniques

**Exclude Code from Coverage**:

```rust
// Exclude specific function
#[cfg(not(tarpaulin_include))]
fn debug_only_function() {
    // ...
}

// Exclude block
#[cfg(not(tarpaulin))]
{
    // Debug-only code
}
```

**Coverage for Specific Test**:

```bash
# Run coverage for specific test
cargo llvm-cov --test integration_tests --html
```

**Coverage with Clean Build**:

```bash
# Clean previous coverage data
cargo llvm-cov clean

# Generate fresh coverage
cargo llvm-cov --workspace --html
```

### Best Practices

✅ **Do**:

- Run coverage locally before submitting PRs
- Add tests for new features immediately
- Test both success and error paths
- Include edge cases and boundary conditions
- Use descriptive test names
- Keep coverage above 80% for critical code

❌ **Don't**:

- Use tarpaulin on Windows (use llvm-cov)
- Commit coverage reports to Git (they're in `.gitignore`)
- Optimize for 100% coverage at the expense of code quality
- Write tests just to increase coverage without testing behavior
- Ignore low coverage warnings in CI

---

## �📝 Code Style

### Rust Conventions

- Use `rustfmt` for formatting: `cargo fmt --all`
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use descriptive variable names
- Add doc comments for public APIs

### Documentation

```rust
/// Compiles FerrisScript source code into an AST.
///
/// # Arguments
/// * `source` - The FerrisScript source code as a string
///
/// # Returns
/// * `Ok(Program)` - Successfully compiled AST
/// * `Err(String)` - Compilation error message
///
/// # Examples
/// ```
/// let source = "fn _ready() { print(\"Hello\"); }";
/// let ast = compile(source)?;
/// ```
pub fn compile(source: &str) -> Result<Program, String> {
    // ...
}
```

---

## �️ Rust Edition Explained

### What is a Rust Edition?

Rust uses an **edition system** (2015, 2018, 2021) to introduce backwards-incompatible changes without breaking existing code. Think of it like:

- **Python 2 vs Python 3** (but less painful)
- **C++11, C++14, C++17** standards

### FerrisScript Uses 2021 Edition

```toml
[package]
edition = "2021"  # In all Cargo.toml files
```

**Benefits of 2021 Edition**:

- **Disjoint capture in closures**: Closures only borrow fields they use
- **Panic message consistency**: Better error messages
- **IntoIterator for arrays**: Can iterate `[1, 2, 3]` directly
- **New preludes**: `TryFrom`, `TryInto` available automatically
- **Cargo resolver v2**: Better dependency resolution

**Backwards Compatibility**: Code compiled with edition 2021 works with libraries using 2015/2018. The edition only affects **how your code is compiled**, not the ABI.

### Should We Upgrade?

**Current**: 2021 edition (latest)  
**Recommendation**: ✅ Keep 2021 - it's the latest stable edition

---

## 🔄 Dependency Management

### Updating Dependencies

```bash
# Check for outdated dependencies
cargo outdated

# Update all dependencies to latest compatible versions
cargo update

# Update specific dependency
cargo update -p gdext

# Update to breaking versions (edit Cargo.toml first)
cargo build
```

### Adding Dependencies

```toml
# In crates/compiler/Cargo.toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

---

## 🐛 Debugging

### Debug Prints

```rust
// Use dbg! macro for debugging
let result = dbg!(some_calculation());

// Use eprintln! for errors
eprintln!("Error: {}", error_message);
```

### Godot Integration Debugging

```rust
// In godot_bind/src/lib.rs
godot_print!("Debug info: {}", value);
godot_warn!("Warning: {}", warning);
godot_error!("Error: {}", error);
```

### LLDB/GDB Debugging

```bash
# Build with debug symbols
cargo build

# Run with debugger
rust-lldb target/debug/your_binary
# or
rust-gdb target/debug/your_binary
```

---

## 📊 Performance Profiling

```bash
# Build with release optimizations
cargo build --release

# Profile with perf (Linux)
perf record target/release/your_binary
perf report

# Benchmark (if you add benches/)
cargo bench
```

---

## 🤝 Contributing

### Commit Message Convention

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**:

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style changes (formatting, no logic change)
- `refactor`: Code refactoring
- `test`: Adding/updating tests
- `chore`: Maintenance tasks
- `ci`: CI/CD changes

**Examples**:

```bash
feat(compiler): add array type support
fix(runtime): handle division by zero
docs: update ARCHITECTURE.md with new patterns
test(parser): add tests for function calls
chore: update gdext to 0.2.0
```

### Code Review Checklist

Before submitting a PR:

- [ ] All tests pass (`cargo test --workspace`)
- [ ] Code is formatted (`cargo fmt --all`)
- [ ] No clippy warnings (strict mode: `cargo clippy --workspace --all-targets --all-features -- -D warnings`)
- [ ] Documentation updated if needed
- [ ] Documentation linting passes (`npm run docs:lint`)
- [ ] Commit messages follow convention
- [ ] PR description explains the change

---

## 📚 Resources

### FerrisScript Docs

- [ARCHITECTURE.md](ARCHITECTURE.md) - Technical design and decisions
- [RELEASE_NOTES.md](../RELEASE_NOTES.md) - Release information
- [docs/archive/](archive/) - Version-specific development docs

### Rust Learning

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### Compiler/Interpreter Resources

- [Crafting Interpreters](https://craftinginterpreters.com/) - Excellent book
- [Pratt Parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
- [Rust Compiler Development Guide](https://rustc-dev-guide.rust-lang.org/)

### Godot + Rust

- [Godot 4.x Documentation](https://docs.godotengine.org/en/stable/)
- [gdext Book](https://godot-rust.github.io/book/)
- [GDExtension Documentation](https://docs.godotengine.org/en/stable/tutorials/scripting/gdextension/)

---

## 📄 License

FerrisScript is licensed under the [MIT License](../LICENSE)

MIT (to be added)
