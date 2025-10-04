# FerrisScript Scripts

This directory contains helper scripts for development workflows.

## Quick Reference

| Script | Purpose | Platforms |
|--------|---------|-----------|
| `test.sh` / `test.ps1` | Run all tests | All |
| `bench.sh` / `bench.ps1` | Run benchmarks | All |
| `format.sh` / `format.ps1` | Format code | All |
| `coverage.sh` / `coverage.ps1` | Generate coverage | All |
| `lint-docs.sh` / `lint-docs.ps1` | Lint documentation | All |
| `pre-push.sh` / `pre-push.ps1` | Pre-push validation | All |

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

See [docs/v0.0.2/BENCHMARK_BASELINE.md](../docs/v0.0.2/BENCHMARK_BASELINE.md) for detailed analysis.

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
