# FerrisScript Scripts

This directory contains helper scripts for development workflows.

## Quick Reference

| Script | Purpose | Platforms |
|--------|---------|-----------|
| `test.sh` / `test.ps1` | Run all tests | All |
| `run-tests.sh` / `run-tests.ps1` | Run test harness examples | All |
| `bench.sh` / `bench.ps1` | Run benchmarks | All |
| `format.sh` / `format.ps1` | Format code | All |
| `coverage.sh` / `coverage.ps1` | Generate coverage | All |
| `lint.sh` / `lint.ps1` | Run linting checks | All |
| `lint-docs.sh` / `lint-docs.ps1` | Lint documentation | All |
| `pre-push.sh` / `pre-push.ps1` | Pre-push validation | All |
| `install-git-hooks.sh` / `install-git-hooks.ps1` | Install Git hooks | All |
| `uninstall-git-hooks.sh` / `uninstall-git-hooks.ps1` | Uninstall Git hooks | All |

**Tip**: All scripts have both `.sh` (Bash) and `.ps1` (PowerShell) versions for cross-platform support.

---

## Development Scripts

### Test Runner

Runs all tests in the workspace (182 tests).

**PowerShell (Windows)**:

```powershell
.\scripts\test.ps1
```

**Bash (Linux/macOS)**:

```bash
./scripts/test.sh
```

**What It Does**:

- Runs `cargo test --workspace`
- Shows test results for all crates (compiler, runtime, godot_bind)
- Returns exit code 0 on success, non-zero on failure

**Use Cases**:

- Quick validation during development
- Pre-commit testing
- CI/CD integration

---

### Test Harness Runner

Runs FerrisScript examples through the headless Godot test harness.

**PowerShell (Windows)**:

```powershell
# Run a specific example
.\scripts\run-tests.ps1 -Script examples/node_query_basic.ferris -Verbose

# Run all examples matching a filter
.\scripts\run-tests.ps1 -All -Filter "node_query"

# Fast mode: skip rebuild if harness is already built
.\scripts\run-tests.ps1 -Script examples/hello.ferris -Fast

# Run all examples
.\scripts\run-tests.ps1 -All
```

**Bash (Linux/macOS)**:

```bash
# Run a specific example
./scripts/run-tests.sh --script examples/node_query_basic.ferris --verbose

# Run all examples matching a filter
./scripts/run-tests.sh --all --filter "node_query"

# Fast mode: skip rebuild
./scripts/run-tests.sh --script examples/hello.ferris --fast

# Run all examples
./scripts/run-tests.sh --all
```

**What It Does**:

- Builds test harness in release mode (unless `--fast`/`-Fast`)
- Runs FerrisScript examples through headless Godot
- Parses output for assertion markers (`✓`, `✗`, `○`)
- Reports test results with colored output
- Returns exit code 0 on success, non-zero on failure

**Command-Line Options**:

- `--script PATH` / `-Script PATH`: Run specific example file
- `--all` / `-All`: Run all examples in workspace
- `--filter PATTERN` / `-Filter PATTERN`: Filter examples by name pattern
- `--verbose` / `-Verbose`: Show detailed test output
- `--fast` / `-Fast`: Skip rebuild, use existing test harness binary

**Assertion Markers**:

Examples include print statements with markers:

- `✓` - Assertion passed (expected behavior confirmed)
- `✗` - Assertion failed (unexpected behavior detected)
- `○` - Informational (optional check, no failure if not present)

**Example Output**:

```
ℹ️  Building test harness in release mode...
✅ Build complete

ℹ️  Running: cargo run --release --bin ferris-test -- --script examples/node_query_basic.ferris --verbose

Running test: node_query_basic.ferris
Test result: PASS
  - ✓ Found Player node
  - ✓ Found UI node
  - ✓ Got parent node
  - ✓ Found OtherChild node

✅ All tests passed!
```

**Use Cases**:

- Testing examples against real Godot runtime
- Validating node query functionality
- Integration testing without manual Godot setup
- Pre-commit validation of examples

**See Also**:

- [docs/testing/PHASE_1_COMPLETION_REPORT.md](../docs/testing/PHASE_1_COMPLETION_REPORT.md) - Test harness architecture
- [docs/testing/PHASE_2_COMPLETION_REPORT.md](../docs/testing/PHASE_2_COMPLETION_REPORT.md) - Node query test coverage

---

### Benchmark Runner

Runs performance benchmarks for the compiler.

**PowerShell (Windows)**:

```powershell
.\scripts\bench.ps1
```

**Bash (Linux/macOS)**:

```bash
./scripts/bench.sh
```

**What It Does**:

- Runs `cargo bench --package ferrisscript_compiler`
- Executes lexer, parser, type checker benchmarks
- Saves results to `target/criterion/`
- Generates comparison reports

**Use Cases**:

- Performance regression testing
- Optimization validation
- Establishing baseline metrics

**Benchmark Results**:

- Lexer: 384 ns - 3.74 μs
- Parser: 600 ns - 7.94 μs
- Type Checker: 851 ns - 3.58 μs

See [docs/archive/v0.0.2/BENCHMARK_BASELINE.md](../docs/archive/v0.0.2/BENCHMARK_BASELINE.md) for detailed analysis.

---

### Code Formatter

Formats all Rust code in the workspace.

**PowerShell (Windows)**:

```powershell
.\scripts\format.ps1
```

**Bash (Linux/macOS)**:

```bash
./scripts/format.sh
```

**What It Does**:

- Runs `cargo fmt --all`
- Formats all `.rs` files according to project style
- Modifies files in-place

**Use Cases**:

- Pre-commit formatting
- Consistent code style
- Automatic style fixes

**Tip**: Run `cargo fmt -- --check` to verify formatting without modifying files (useful in CI).

---

### Code Linter

Runs cargo clippy with strict warning checks.

**PowerShell (Windows)**:

```powershell
.\scripts\lint.ps1
```

**Bash (Linux/macOS)**:

```bash
./scripts/lint.sh
```

**What It Does**:

- Runs `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- Catches common mistakes, anti-patterns, and potential bugs
- Treats all warnings as errors (strict mode)
- Returns exit code 0 on success, non-zero on failure

**Use Cases**:

- Pre-commit validation
- CI/CD quality gates
- Code review preparation
- Maintaining code quality standards

**Common Warnings**:

- Unused variables or imports
- Inefficient patterns
- Potential bugs (e.g., unwrap() on Option)
- Code style inconsistencies

**Fixing Warnings**:

Most warnings include suggestions. Example:

```rust
warning: unused variable: `x`
 --> src/main.rs:5:9
  |
5 |     let x = 42;
  |         ^ help: if this is intentional, prefix it with an underscore: `_x`
```

**Suppressing Warnings** (use sparingly):

```rust
#[allow(clippy::lint_name)]
fn my_function() {
    // Code that triggers warning
}
```

---

## Code Coverage

### Prerequisites

- **Rust toolchain** installed via rustup
- **cargo-llvm-cov** (auto-installed by scripts if missing)

### Usage

#### PowerShell (Windows)

```powershell
.\scripts\coverage.ps1
```

#### Bash (Linux/macOS)

```bash
./scripts/coverage.sh
```

### What It Does

1. Checks if `cargo-llvm-cov` is installed
2. Auto-installs `llvm-tools-preview` and `cargo-llvm-cov` if needed
3. Runs coverage analysis across all workspace crates
4. Generates HTML report in `target/coverage/html/index.html`
5. Generates LCOV report in `target/coverage/lcov.info`

### Viewing Reports

```powershell
# Windows
Invoke-Item target/coverage/html/index.html

# Linux
xdg-open target/coverage/html/index.html

# macOS
open target/coverage/html/index.html
```

### More Information

See [docs/DEVELOPMENT.md - Code Coverage](../docs/DEVELOPMENT.md#-code-coverage) for:

- Detailed coverage documentation
- Why we use llvm-cov vs tarpaulin
- Troubleshooting coverage issues
- Coverage goals and best practices

## Documentation Linting

### Prerequisites

- **Node.js** (v16 or higher): [Download here](https://nodejs.org/)
- **npm**: Comes with Node.js

### Installation

Run once to install dependencies:

```powershell
npm install
```

### Usage

```bash
# Check markdown formatting
npm run docs:lint

# Auto-fix markdown formatting issues
npm run docs:fix
```

**Note**: These npm scripts use the same markdownlint configuration as CI, ensuring consistency between local development and automated checks.

### What Gets Checked

- Heading styles and hierarchy
- List formatting
- Code block formatting
- Line length (relaxed to 120 chars)
- Trailing spaces
- And more... (see `.markdownlint.json`)

### Configuration

- **`.markdownlint.json`** - Markdown formatting rules

### CI Integration

Documentation linting runs automatically on:

- Pull requests (when `.md` files change)
- Pushes to main branch

See `.github/workflows/docs-lint.yml` for CI configuration.

### Troubleshooting

#### "Node.js is not installed"

Download and install from [nodejs.org](https://nodejs.org/)

#### "npm: command not found"

npm comes with Node.js. Restart your terminal after installing Node.js.

---

## Pre-commit Hooks

Automatically run quality checks before each commit.

### Installation

**PowerShell (Windows)**:

```powershell
.\scripts\install-git-hooks.ps1
```

**Bash (Linux/macOS)**:

```bash
./scripts/install-git-hooks.sh
```

### What It Does

Installs Git hooks that run before each commit and push:

**Pre-commit Hook**:

1. **Format Check**: Verifies code is formatted with `cargo fmt`
2. **Linting**: Runs `cargo clippy` with strict warnings
3. **Quick Tests**: Runs fast unit tests (skips slow integration tests)

**Pre-push Hook**:

1. **Markdown Linting**: Validates documentation formatting (only when `.md` files changed)

### Workflow

```bash
git add .
git commit -m "feat: add new feature"
# 🔍 Pre-commit hook runs automatically
# ✅ All checks pass → Commit proceeds
# ❌ Any check fails → Commit blocked, fix issues
```

### Bypassing Hooks (use sparingly)

For work-in-progress commits:

```bash
git commit --no-verify -m "WIP: experimenting"
git push --no-verify
```

### When to Bypass

- ✅ Experimental code (will revert)
- ✅ WIP commits (will clean up before PR)
- ✅ Debugging commits (temporary)
- ❌ NOT for PR commits (must pass checks)

### Troubleshooting

#### Hooks stop working

```bash
# Reinstall hooks
./scripts/install-git-hooks.sh

# Verify hook exists
cat .git/hooks/pre-commit
cat .git/hooks/pre-push
```

#### Hooks too slow

The pre-commit hook runs quick tests only (`cargo test --lib`). For WIP commits, you can bypass with `--no-verify`.

#### Uninstalling Hooks

**PowerShell (Windows)**:

```powershell
.\scripts\uninstall-git-hooks.ps1
```

**Bash (Linux/macOS)**:

```bash
./scripts/uninstall-git-hooks.sh
```

**What It Does**:

- Removes `.git/hooks/pre-commit` hook
- Removes `.git/hooks/pre-push` hook
- Shows confirmation for each hook removed
- Safe to run even if hooks aren't installed

**Manual Removal** (alternative):

```powershell
# PowerShell
Remove-Item .git/hooks/pre-commit -ErrorAction SilentlyContinue
Remove-Item .git/hooks/pre-push -ErrorAction SilentlyContinue
```

```bash
# Bash
rm -f .git/hooks/pre-commit
rm -f .git/hooks/pre-push
```

#### "Module not found"

Run `npm install` to install dependencies

#### Link check fails for external sites

Some sites may block automated requests. Check `.markdown-link-check.json` to configure retries or ignore specific domains.

## Testing FerrisScript Files

### test_ferris Example (Rust)

The compiler package includes a test utility for verifying FerrisScript files:

```bash
# Test any FerrisScript file
cargo run --example test_ferris -- examples/hello.ferris

# See error messages with source context
cargo run --example test_ferris -- examples/error_showcase.ferris
```

**What It Does:**

- Compiles the FerrisScript file
- Shows `✓ Compilation successful!` if valid
- Shows `✗ Compilation failed:` with detailed error message if invalid
- Error messages include:
  - Source context (±2 lines around error)
  - Visual pointer (^) at error location
  - Helpful hint explaining what's expected

**Use Cases:**

- Quick syntax validation
- Testing error message improvements
- Learning FerrisScript error handling
- Debugging script issues before loading in Godot

### test_error.ps1 (PowerShell - Legacy)

Located in `scripts/test_error.ps1`. See file for usage.

**Note**: The Rust example (`test_ferris`) is now the recommended approach as it's cross-platform and better integrated.

### Adding New Scripts

1. Create script in `scripts/` directory
2. Add entry to `package.json` scripts section (if npm-based)
3. Add VS Code task in `.vscode/tasks.json` (optional)
4. Document here in this README

## Future Scripts

Planned additions:

- `generate-toc.ps1` - Auto-generate table of contents
- `check-copyright.ps1` - Verify copyright headers
- `sync-versions.ps1` - Keep version numbers in sync
