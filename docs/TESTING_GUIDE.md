# FerrisScript Testing Guide

**Single Source of Truth for All Testing Patterns**

**Last Updated**: 2025-10-10  
**Status**: Active - v0.0.4 Phase 5  
**Purpose**: Comprehensive guide to all testing approaches in FerrisScript

---

## Quick Navigation

- [Testing Philosophy](#testing-philosophy)
- [Test Architecture Overview](#test-architecture-overview)
- [Pattern 1: Unit Tests](#pattern-1-unit-tests-rust-only)
- [Pattern 2: Integration Tests (.ferris Scripts)](#pattern-2-integration-tests-ferris-scripts)
- [Pattern 3: GDExtension Testing](#pattern-3-gdextension-testing-godot-runtime)
- [Pattern 4: Benchmark Tests](#pattern-4-benchmark-tests)
- [Configuration](#configuration)
- [Running Tests](#running-tests)
- [CI/CD Integration](#cicd-integration)
- [Troubleshooting](#troubleshooting)

---

## Testing Philosophy

FerrisScript uses a **layered testing strategy** where each layer validates different concerns:

```
┌─────────────────────────────────────────────┐
│   Layer 4: Manual Testing (Godot Editor)   │  ← Feature validation
├─────────────────────────────────────────────┤
│   Layer 3: Integration Tests (.ferris)     │  ← End-to-end behavior
├─────────────────────────────────────────────┤
│   Layer 2: GDExtension Tests (GDScript)    │  ← Godot bindings
├─────────────────────────────────────────────┤
│   Layer 1: Unit Tests (Rust)               │  ← Pure logic
└─────────────────────────────────────────────┘
```

### Key Principles

1. **Test at the Right Layer**: Don't use integration tests for unit-testable logic
2. **Use Existing Infrastructure**: Leverage `test_harness` and `ferris-test.toml`
3. **Document Test Intent**: Every test should clearly state what it validates
4. **CI-Friendly**: All tests must run headlessly without GUI
5. **Fast Feedback**: Unit tests run in <1s, integration tests in <30s

---

## Test Architecture Overview

### Crate Structure

```
FerrisScript/
├── crates/
│   ├── compiler/          # Layer 1: Unit tests (543 tests)
│   │   └── src/
│   │       ├── lexer.rs             (tests inline)
│   │       ├── parser.rs            (tests inline)
│   │       ├── type_checker.rs      (tests inline)
│   │       └── error_code.rs        (tests inline)
│   │
│   ├── runtime/           # Layer 1: Unit tests (110 tests)
│   │   ├── src/lib.rs               (tests inline)
│   │   └── tests/
│   │       └── inspector_sync_test.rs
│   │
│   ├── godot_bind/        # Layer 1 + Layer 2
│   │   ├── src/lib.rs               (11 unit tests pass, 10 ignored*)
│   │   └── tests/
│   │       └── headless_integration.rs  (Layer 2: GDExtension tests)
│   │
│   └── test_harness/      # Layer 2 Infrastructure
│       ├── src/
│       │   ├── lib.rs
│       │   ├── main.rs              (ferris-test CLI)
│       │   ├── godot_cli.rs         (GodotRunner)
│       │   ├── output_parser.rs     (Marker parsing)
│       │   └── test_runner.rs       (TestHarness)
│       └── tests/
│           └── (self-tests)
│
├── godot_test/            # Layer 2 + Layer 3
│   ├── ferrisscript.gdextension
│   ├── scripts/
│   │   ├── *.ferris                 (Layer 3: Integration tests)
│   │   └── *.gd                     (Layer 2: GDExtension test runners)
│   └── tests/
│       └── generated/               (Auto-generated .tscn files)
│
├── ferris-test.toml       # Shared Configuration
└── docs/
    ├── TESTING_GUIDE.md              ← You are here
    └── planning/v0.0.4/
        ├── TESTING_STRATEGY_PHASE5.md    (Detailed strategy)
        ├── INTEGRATION_TESTS_REPORT.md   (Phase 5 results)
        └── INTEGRATION_TESTS_FIXES.md    (Bug fixes)
```

**Note**: The 10 ignored godot_bind tests require Godot runtime. They're covered by Layer 2 (GDExtension tests) and Layer 3 (integration tests). See [Why Some Tests Are Ignored](#why-some-tests-are-ignored).

---

## Pattern 1: Unit Tests (Rust Only)

**When to use**: Pure logic without Godot dependencies

**Location**: Inline `#[cfg(test)] mod tests` in source files

**Example**: Compiler type checking, runtime value operations

### Structure

```rust
// In crates/compiler/src/type_checker.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_range_hint_valid_i32() {
        let source = r#"
            @export @range(0, 100, 1)
            global health: i32 = 50;
        "#;
        
        let result = type_check(source);
        assert!(result.is_ok());
    }
}
```

### Running

```bash
# All unit tests
cargo test

# Specific crate
cargo test --package ferrisscript_compiler

# Specific test
cargo test test_export_range_hint_valid_i32

# With output
cargo test -- --nocapture
```

### Coverage

- ✅ **compiler**: 543 tests (lexer, parser, type checker, error system)
- ✅ **runtime**: 110 tests (execution, scoping, value operations)
- ✅ **godot_bind**: 11 tests (type mapping, API structure)
- ✅ **test_harness**: 38 tests (parser, output validation, scene builder)

**Total**: ~702 unit tests

---

## Pattern 2: Integration Tests (.ferris Scripts)

**When to use**: End-to-end validation of FerrisScript → Godot compilation and execution

**Location**: `godot_test/scripts/*.ferris`

**Infrastructure**: `test_harness` crate + `ferris-test.toml`

### How It Works

1. Write `.ferris` script with test metadata comments
2. Run via `ferris-test` CLI (uses `test_harness` crate)
3. CLI dynamically generates `.tscn` scene file
4. Godot runs headlessly with the scene
5. Output is parsed for `[PASS]`/`[FAIL]` markers
6. Results reported to console/JSON/TAP

### Example Test Script

```ferrisscript
// godot_test/scripts/export_properties_test.ferris
// @test-category: integration
// @test-name: Exported Properties with All Types and Hints
// @expect-pass

// Test exported properties
@export
global basic_int: i32 = 42;

@export @range(0, 100, 1)
global health: i32 = 100;

@export @enum("Small", "Medium", "Large")
global size: String = "Medium";

fn _ready() {
    print("[TEST_START]");
    
    // Test basic int
    if basic_int == 42 {
        print("[PASS] basic_int has correct value");
    } else {
        print("[FAIL] basic_int incorrect");
    }
    
    // Test range
    if health >= 0 && health <= 100 {
        print("[PASS] health within range");
    } else {
        print("[FAIL] health out of range");
    }
    
    print("[TEST_END]");
}
```

### Test Metadata Comments

```ferrisscript
// @test-category: integration | unit | feature | regression
// @test-name: Human-readable test description
// @expect-pass | @expect-error(E301) | @expect-error-demo
// @assert: condition description (optional, multiple allowed)
```

### Running Integration Tests

```bash
# Run single test
ferris-test --script godot_test/scripts/export_properties_test.ferris

# Run all tests
ferris-test --all

# Filter by name
ferris-test --all --filter "export"

# Verbose output
ferris-test --all --verbose

# JSON format (for CI)
ferris-test --all --format json > results.json
```

### Configuration (ferris-test.toml)

```toml
# Location: workspace root
godot_executable = "Y:\\cpark\\Projects\\Godot\\Godot_v4.5-dev4_win64.exe\\Godot_v4.5-dev4_win64_console.exe"
project_path = "./godot_test"
timeout_seconds = 30
output_format = "console"
verbose = true
```

**Environment Overrides**:
- `GODOT_BIN`: Override godot_executable
- `GODOT_PROJECT_PATH`: Override project_path
- `GODOT_TIMEOUT`: Override timeout_seconds

### Coverage

Current integration tests:
- ✅ `export_properties_test.ferris` - All 8 types, 4 hint types
- ✅ `clamp_on_set_test.ferris` - Range clamping behavior
- ✅ `signal_test.ferris` - Signal emission
- ✅ `process_test.ferris` - Lifecycle functions
- ✅ `node_query_*.ferris` - Scene tree queries
- ✅ `struct_literals_*.ferris` - Godot type construction
- ✅ `bounce_test.ferris`, `move_test.ferris`, `hello.ferris` - Examples

**Total**: 15+ integration tests

See: `docs/planning/v0.0.4/INTEGRATION_TESTS_REPORT.md` for detailed results

---

## Pattern 3: GDExtension Testing (Godot Runtime)

**When to use**: Testing Rust functions that construct Godot types (`GString`, `PropertyInfo`, etc.)

**Location**: `crates/{crate}/tests/headless_integration.rs` + `godot_test/scripts/*.gd`

**Why needed**: Some Rust functions require `godot::init()` which can't run in unit tests

### The Problem

```rust
// In crates/godot_bind/src/lib.rs

fn map_hint(hint: &ast::PropertyHint) -> PropertyHintInfo {
    match hint {
        ast::PropertyHint::Range { min, max, step } => {
            export_info_functions::export_range(  // ← Requires godot::init()
                *min as f64,
                *max as f64,
                Some(*step as f64),
                // ...
            )
        }
        // ...
    }
}

#[test]
#[ignore = "Requires Godot engine runtime"]
fn test_map_hint_range() {
    let hint = ast::PropertyHint::Range { min: 0.0, max: 100.0, step: 1.0 };
    let result = map_hint(&hint);  // ← FAILS: godot::init() not called
    assert_eq!(result.hint, PropertyHint::RANGE);
}
```

### The Solution: GDScript Test Runner

**Step 1**: Create GDScript test runner

```gdscript
# godot_test/scripts/godot_bind_tests.gd
extends Node

var passed_tests: int = 0
var failed_tests: int = 0

func _ready():
    print("[TEST_START]")
    
    test_basic_functionality()
    test_property_hint_enum()
    # ... more tests
    
    print("[SUMMARY] Total: %d, Passed: %d, Failed: %d" % 
          [passed_tests + failed_tests, passed_tests, failed_tests])
    print("[TEST_END]")
    
    get_tree().quit(failed_tests if failed_tests > 0 else 0)

func test_basic_functionality():
    run_test("Basic Node Creation", func():
        var node = Node.new()
        assert_not_null(node, "Node should be created")
        node.queue_free()
    )

func test_property_hint_enum():
    run_test("PropertyHint Enum Exists", func():
        # Validate that PropertyHint enum is accessible
        assert_equal(PropertyHint.NONE, 0, "PropertyHint.NONE should be 0")
        assert_equal(PropertyHint.RANGE, 1, "PropertyHint.RANGE should be 1")
    )

func run_test(test_name: String, test_func: Callable):
    print("[TEST] Running: %s" % test_name)
    var error = test_func.call()
    if error == null:
        print("[PASS] %s" % test_name)
        passed_tests += 1
    else:
        print("[FAIL] %s - %s" % [test_name, error])
        failed_tests += 1

func assert_equal(actual, expected, message: String):
    if actual != expected:
        return "%s (expected: %s, got: %s)" % [message, expected, actual]
    return null

func assert_not_null(value, message: String):
    if value == null:
        return message
    return null
```

**Step 2**: Create test scene

```
# godot_test/test_godot_bind.tscn
[gd_scene load_steps=2 format=3]

[ext_resource type="Script" path="res://scripts/godot_bind_tests.gd" id="1"]

[node name="GodotBindTests" type="Node"]
script = ExtResource("1")
```

**Step 3**: Create Rust integration test

```rust
// crates/godot_bind/tests/headless_integration.rs

use ferrisscript_test_harness::{TestConfig, TestOutput, GodotRunner};
use std::path::PathBuf;

fn get_test_config() -> Result<TestConfig, String> {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .parent().unwrap()
        .to_path_buf();
    
    let config_path = workspace_root.join("ferris-test.toml");
    
    let mut config = if config_path.exists() {
        TestConfig::from_file(&config_path)
            .map_err(|e| format!("Failed to load config: {}", e))?
    } else {
        TestConfig::default()
    };
    
    config = config.with_env_overrides();
    Ok(config)
}

#[test]
#[ignore = "Requires Godot executable - configure in ferris-test.toml"]
fn test_godot_headless_basic() {
    let config = get_test_config().expect("Failed to load config");
    
    let runner = GodotRunner::new(
        config.godot_executable,
        config.project_path,
        config.timeout_seconds,
    );
    
    let test_scene = PathBuf::from("test_godot_bind.tscn");
    let output = runner.run_headless(&test_scene)
        .expect("Failed to run Godot");
    
    // Parse [PASS]/[FAIL] markers
    let passed = output.stdout.contains("[PASS]") 
                 && !output.stdout.contains("[FAIL]");
    
    assert!(passed, "Test failed. Output:\n{}", output.stdout);
    assert_eq!(output.exit_code, 0);
}
```

### Running GDExtension Tests

```bash
# Run ignored tests (requires Godot configured in ferris-test.toml)
cargo test --package ferrisscript_godot_bind --test headless_integration -- --ignored --nocapture

# Or use environment override
GODOT_BIN=/path/to/godot cargo test --package ferrisscript_godot_bind --test headless_integration -- --ignored
```

### When to Add GDExtension Tests

Add GDExtension tests when you have Rust functions that:
1. Construct Godot types (`GString`, `PropertyInfo`, `Variant`, etc.)
2. Call Godot API functions
3. Need `godot::init()` to run
4. Can't be unit tested due to Godot runtime requirements

**Don't** add GDExtension tests for:
- Pure Rust logic (use unit tests)
- End-to-end `.ferris` script behavior (use integration tests)

### Coverage

Current GDExtension tests:
- ✅ Basic Godot functionality (Node creation, PropertyHint enum)
- ⏳ FerrisScriptTestNode (planned - will test map_hint(), metadata_to_property_info())

**Why 10 godot_bind tests are ignored**: They're low-level binding tests that require Godot runtime. The functionality IS tested via integration tests (`export_properties_test.ferris` validates all hint types work correctly). See [Why Some Tests Are Ignored](#why-some-tests-are-ignored).

---

## Pattern 4: Benchmark Tests

**When to use**: Performance regression detection

**Location**: `crates/compiler/benches/*.rs`

**Infrastructure**: Criterion.rs

### Example Benchmark

```rust
// crates/compiler/benches/parser_bench.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ferrisscript_compiler::parse;

fn bench_parse_hello(c: &mut Criterion) {
    let source = r#"
        fn _ready() {
            print("Hello, world!");
        }
    "#;
    
    c.bench_function("parse hello", |b| {
        b.iter(|| parse(black_box(source)))
    });
}

criterion_group!(benches, bench_parse_hello);
criterion_main!(benches);
```

### Running Benchmarks

```bash
# All benchmarks
cargo bench

# Specific benchmark
cargo bench --bench parser_bench

# With baseline comparison
cargo bench -- --baseline main
```

### Coverage

Current benchmarks:
- ✅ Lexer performance
- ✅ Parser performance
- ✅ Type checker performance
- ✅ Full pipeline performance

See: `docs/BENCHMARK_BASELINE.md` for baseline results

---

## Configuration

### ferris-test.toml (Workspace Root)

```toml
# Godot executable path (console version recommended for CI)
godot_executable = "Y:\\cpark\\Projects\\Godot\\Godot_v4.5-dev4_win64.exe\\Godot_v4.5-dev4_win64_console.exe"

# Godot project directory
project_path = "./godot_test"

# Test timeout in seconds
timeout_seconds = 30

# Output format: "console", "json", or "tap"
output_format = "console"

# Enable verbose output
verbose = true
```

### Environment Variables

Override config values with environment variables:

```bash
# Windows (PowerShell)
$env:GODOT_BIN = "C:\Path\To\Godot.exe"
$env:GODOT_PROJECT_PATH = "C:\Path\To\godot_test"
$env:GODOT_TIMEOUT = "60"

# Linux/Mac
export GODOT_BIN="/path/to/godot"
export GODOT_PROJECT_PATH="/path/to/godot_test"
export GODOT_TIMEOUT="60"
```

### VS Code tasks.json

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Test: Unit Tests",
      "type": "cargo",
      "command": "test",
      "group": {
        "kind": "test",
        "isDefault": true
      }
    },
    {
      "label": "Test: Integration Tests",
      "type": "shell",
      "command": "ferris-test",
      "args": ["--all"],
      "group": {
        "kind": "test",
        "isDefault": false
      }
    },
    {
      "label": "Test: GDExtension Tests",
      "type": "shell",
      "command": "cargo",
      "args": [
        "test",
        "--package", "ferrisscript_godot_bind",
        "--test", "headless_integration",
        "--", "--ignored", "--nocapture"
      ],
      "group": {
        "kind": "test",
        "isDefault": false
      }
    }
  ]
}
```

---

## Running Tests

### Quick Reference

```bash
# Layer 1: Unit tests (fast, <1s)
cargo test

# Layer 2: GDExtension tests (requires Godot, ~5-10s)
cargo test --test headless_integration -- --ignored --nocapture

# Layer 3: Integration tests (requires Godot, ~30s)
ferris-test --all

# Layer 4: Manual testing
# Open godot_test/project.godot in Godot Editor
```

### Common Workflows

**Pre-commit**: Run fast unit tests
```bash
cargo test
```

**Pre-push**: Run unit + integration tests
```bash
cargo test && ferris-test --all
```

**Feature validation**: Run all layers
```bash
cargo test && \
cargo test --test headless_integration -- --ignored --nocapture && \
ferris-test --all
```

**CI/CD**: All automated tests
```bash
# In GitHub Actions workflow
cargo test --all
ferris-test --all --format json > integration-results.json
cargo bench -- --baseline main
```

---

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Tests

on: [push, pull_request]

jobs:
  unit-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
      
      - name: Run unit tests
        run: cargo test --all

  integration-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Godot
        run: |
          wget https://downloads.tuxfamily.org/godotengine/4.3/Godot_v4.3-stable_linux.x86_64.zip
          unzip Godot_v4.3-stable_linux.x86_64.zip
          sudo mv Godot_v4.3-stable_linux.x86_64 /usr/local/bin/godot
          chmod +x /usr/local/bin/godot
      
      - name: Build GDExtension
        run: cargo build --release
      
      - name: Run integration tests
        env:
          GODOT_BIN: godot
        run: |
          cargo install --path crates/test_harness
          ferris-test --all --format json > results.json
      
      - name: Upload results
        uses: actions/upload-artifact@v3
        with:
          name: integration-results
          path: results.json
```

---

## Why Some Tests Are Ignored

### The 10 Ignored godot_bind Tests

**Location**: `crates/godot_bind/src/lib.rs`

**Tests**:
1. `test_map_hint_none`
2. `test_map_hint_range`
3. `test_map_hint_enum`
4. `test_map_hint_file_with_dots`
5. `test_map_hint_file_with_wildcards`
6. `test_map_hint_file_without_dots`
7. `test_metadata_basic_property`
8. `test_metadata_with_range_hint`
9. `test_metadata_with_enum_hint`
10. `test_metadata_with_file_hint`

**Why Ignored**: These tests call functions that construct Godot types (`GString`, `PropertyHintInfo`), which require `godot::init()`. This can't be called in Rust unit tests because:

1. Godot initialization is a one-time global operation
2. It requires the Godot engine to be running
3. Multiple tests calling `godot::init()` would conflict
4. Unit tests run in parallel, making initialization unsafe

**Are They Tested?**: YES! The functionality IS validated via:

1. **Integration Tests** (Layer 3): `export_properties_test.ferris` tests all 8 types and 4 hint types end-to-end
2. **GDExtension Tests** (Layer 2): `headless_integration.rs` can test these functions directly once `FerrisScriptTestNode` is added

**Should They Be Enabled?**: NO. They serve as documentation of the API but are redundant with higher-level tests. The ignore attribute correctly indicates these are low-level functions requiring Godot runtime.

**Alternative Approach**: If unit testing these functions is critical, they could be refactored to:
1. Extract pure logic into testable helper functions
2. Keep Godot type construction in thin wrappers
3. Unit test the helpers, integration test the wrappers

See: `docs/planning/v0.0.4/TESTING_STRATEGY_PHASE5.md` Section "godot_bind Tests (21 tests: 11 passing, 10 failing)"

---

## Troubleshooting

### "Godot executable not found"

**Problem**: Tests can't find Godot

**Solution**:
1. Check `ferris-test.toml` has correct `godot_executable` path
2. Or set `GODOT_BIN` environment variable
3. Ensure Godot 4.3+ is installed

```bash
# Windows
$env:GODOT_BIN = "C:\Godot\Godot_v4.3-stable_win64.exe"

# Linux
export GODOT_BIN="/usr/local/bin/godot"
```

### "Test timeout"

**Problem**: Test exceeds timeout_seconds

**Solution**:
1. Increase `timeout_seconds` in `ferris-test.toml`
2. Or set `GODOT_TIMEOUT` environment variable
3. Check if Godot is hanging (try `--verbose` flag)

### "Scene not found"

**Problem**: Godot can't find test scene

**Solution**:
1. Verify `project_path` points to `godot_test/`
2. Check scene file exists (`test_godot_bind.tscn`, `test_{name}.tscn`)
3. Ensure scene format is Godot 4.x compatible

### "GDExtension not loaded"

**Problem**: Godot can't load FerrisScript GDExtension

**Solution**:
1. Build GDExtension: `cargo build --release`
2. Check `godot_test/ferrisscript.gdextension` paths are correct
3. Verify `.dll`/`.so` exists in `target/release/`
4. Check Godot console output for load errors

### "Test marked as failed but output looks correct"

**Problem**: Parser misinterpreted output

**Solution**:
1. Ensure test uses `[TEST_START]`, `[PASS]`, `[FAIL]`, `[TEST_END]` markers
2. Check for extra `[FAIL]` markers in error messages
3. Run with `--verbose` to see full output
4. Verify exit code (0 = pass, non-zero = fail)

### "Tests pass locally but fail in CI"

**Problem**: Environment differences

**Solution**:
1. Check CI has Godot installed
2. Verify CI uses headless/console Godot variant
3. Ensure GDExtension is built before tests run
4. Check for absolute paths in tests (use config-based paths)

---

## Related Documentation

### Primary References

- **This Guide** - Single source of truth for testing patterns
- `docs/planning/v0.0.4/TESTING_STRATEGY_PHASE5.md` - Detailed strategy and analysis (1533 lines)
- `docs/planning/v0.0.4/INTEGRATION_TESTS_REPORT.md` - Phase 5 test results and findings
- `docs/planning/v0.0.4/INTEGRATION_TESTS_FIXES.md` - Bug fixes from integration testing

### Supporting Documentation

- `docs/HEADLESS_GODOT_SETUP.md` - GDExtension testing architecture (archival)
- `docs/RUNNING_HEADLESS_TESTS.md` - User guide (archival - superceded by this guide)
- `docs/BENCHMARK_BASELINE.md` - Performance baselines
- `docs/DEVELOPMENT.md` - General development guide

### Historical Documentation (Archive)

- `docs/archive/testing/TEST_HARNESS_TESTING_STRATEGY.md` - Phase 3 test harness design
- `docs/archive/testing/PHASE_3_COMPLETION_REPORT.md` - Phase 3 testing results

---

## Quick Start Checklist

Setting up testing for a new feature:

- [ ] **Unit Tests**: Add tests in `#[cfg(test)] mod tests` for pure logic
- [ ] **Integration Tests**: Create `.ferris` script in `godot_test/scripts/` if testing end-to-end behavior
- [ ] **GDExtension Tests**: Add GDScript test runner if testing Godot bindings requiring runtime
- [ ] **Benchmarks**: Add benchmark in `crates/*/benches/` if performance-critical
- [ ] **Documentation**: Update this guide if adding new patterns
- [ ] **CI**: Ensure tests run in CI pipeline (check `.github/workflows/`)

---

## Version History

- **v1.0** (2025-10-10): Initial comprehensive guide
  - Consolidated all testing patterns
  - Single source of truth established
  - Clear layer separation
  - Documented GDExtension testing pattern
  - Explained ignored tests

---

**Questions or Issues?** See `CONTRIBUTING.md` or open a GitHub issue.
