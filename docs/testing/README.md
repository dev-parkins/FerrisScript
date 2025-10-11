# FerrisScript Testing Documentation

**Single Source of Truth for Testing Procedures, Patterns, and Tracking**

**Last Updated**: October 10, 2025  
**Status**: Active  
**Purpose**: Central hub for test discoverability and comprehensive testing guidance

---

## 🚀 Quick Start

**New to testing FerrisScript?** Start here:

1. **[TESTING_GUIDE.md](TESTING_GUIDE.md)** - Complete guide to all testing patterns
2. Choose your testing layer:
   - **Unit Tests** → See [Unit Testing (Runtime)](#unit-testing-runtime) or [Unit Testing (Compiler)](#unit-testing-compiler)
   - **Integration Tests** → See [Integration Testing (.ferris Scripts)](#integration-testing-ferris-scripts)
   - **GDExtension Tests** → See [Headless Testing (Godot Runtime)](#headless-testing-godot-runtime)
   - **Performance Tests** → See [Benchmark Testing](#benchmark-testing)
3. Run the tests: `cargo test --workspace` or `ferris-test --all`

**Looking for specific test coverage?** → See [Test Matrices](#-test-matrices)

---

## 📚 Testing Framework Overview

FerrisScript uses a **4-layer testing strategy** where each layer validates different concerns:

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

### Test Suite Summary (v0.0.4)

| Test Type | Count | Location | Run Command |
|-----------|-------|----------|-------------|
| **Unit Tests (Compiler)** | 543 | `crates/compiler/src/` | `cargo test -p ferrisscript_compiler` |
| **Unit Tests (Runtime)** | 110 | `crates/runtime/src/` | `cargo test -p ferrisscript_runtime` |
| **Unit Tests (GDExtension)** | 11 pass, 10 ignored | `crates/godot_bind/src/` | `cargo test -p ferrisscript_godot_bind` |
| **Integration Tests** | 15+ | `godot_test/scripts/*.ferris` | `ferris-test --all` |
| **Benchmark Tests** | 8 suites | `crates/*/benches/` | `cargo bench` |
| **Total** | **843+** | All layers | `cargo test --workspace` |

**Coverage**: ~82% (last updated: 2025-10-10)

---

## 🔍 Testing Types & Procedures

### Unit Testing (Compiler)

**What**: Tests lexer, parser, type checker, and code generator logic  
**When**: Testing pure compilation logic without runtime execution  
**Location**: `crates/compiler/src/` (inline `#[cfg(test)] mod tests`)  

**Documentation**:

- [TESTING_GUIDE.md - Pattern 1](TESTING_GUIDE.md#pattern-1-unit-tests-rust-only)
- [DEVELOPMENT.md - Running Tests](../DEVELOPMENT.md#run-tests)

**Quick Start**:

```bash
# Run all compiler tests (543 tests)
cargo test -p ferrisscript_compiler

# Run specific test
cargo test -p ferrisscript_compiler test_parse_assignment

# Run with output
cargo test -p ferrisscript_compiler -- --show-output
```

**Example Test**:

```rust
#[test]
fn test_parse_assignment() {
    let source = "let x = 42;";
    let result = Parser::parse(source);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().statements.len(), 1);
}
```

---

### Unit Testing (Runtime)

**What**: Tests AST interpreter, runtime execution, and node callbacks  
**When**: Testing runtime behavior without Godot GDExtension  
**Location**: `crates/runtime/src/` and `crates/runtime/tests/`  

**Documentation**:

- [TESTING_GUIDE.md - Pattern 1](TESTING_GUIDE.md#pattern-1-unit-tests-rust-only)
- [DEVELOPMENT.md - Test Structure](../DEVELOPMENT.md#test-structure)

**Quick Start**:

```bash
# Run all runtime tests (110 tests)
cargo test -p ferrisscript_runtime

# Run specific test
cargo test -p ferrisscript_runtime test_call_get_node_function

# Run with output
cargo test -p ferrisscript_runtime -- --show-output
```

**Example Test**:

```rust
#[test]
fn test_call_get_node_function() {
    let mut runtime = Runtime::new();
    runtime.set_node_callback(|path| Ok(MockNode::new(path)));
    
    let result = runtime.call_function("get_node", vec!["Player"]);
    assert!(result.is_ok());
}
```

---

### Integration Testing (.ferris Scripts)

**What**: End-to-end testing of .ferris scripts running in Godot  
**When**: Testing complete feature workflows with real Godot runtime  
**Location**: `godot_test/scripts/*.ferris`  
**Tool**: `ferris-test` CLI (from `crates/test_harness`)

**Documentation**:

- [TESTING_GUIDE.md - Pattern 2](TESTING_GUIDE.md#pattern-2-integration-tests-ferris-scripts)
- [TEST_HARNESS_TESTING_STRATEGY.md](TEST_HARNESS_TESTING_STRATEGY.md) - Test harness design

**Quick Start**:

```bash
# Run all integration tests (15+ tests)
ferris-test --all

# Run specific test
ferris-test --script godot_test/scripts/export_properties_test.ferris

# Filter by name
ferris-test --all --filter "signal"

# Verbose output
ferris-test --all --verbose

# JSON format (for CI)
ferris-test --all --format json > results.json
```

**Example Test** (`godot_test/scripts/signal_test.ferris`):

```ferrisscript
// TEST: signal_emission
// CATEGORY: integration
// EXPECT: success
// ASSERT: Signal emitted correctly

export fn _ready() {
    print("[TEST_START]");
    
    signal health_changed(i32, i32);
    emit_signal("health_changed", 100, 80);
    
    print("[PASS] Signal emitted successfully");
    print("[TEST_END]");
}
```

**Configuration**: `ferris-test.toml` in workspace root

---

### Headless Testing (Godot Runtime)

**What**: Tests Rust code that requires Godot runtime initialization  
**When**: Testing GDExtension bindings, Godot type construction, PropertyInfo generation  
**Location**: `crates/*/tests/headless_integration.rs` + `godot_test/scripts/*.gd`  

**Documentation**:

- [TESTING_GUIDE.md - Pattern 3](TESTING_GUIDE.md#pattern-3-gdextension-testing-godot-runtime)
- [TESTING_GUIDE.md - Why Some Tests Are Ignored](TESTING_GUIDE.md#why-some-tests-are-ignored)

**Quick Start**:

```bash
# Run headless tests (requires Godot)
cargo test -p ferrisscript_godot_bind test_headless_integration

# Note: Some tests are marked #[ignore] because they require Godot runtime
# These are tested via integration tests with ferris-test instead
```

**Example Test**:

```rust
#[test]
#[ignore = "requires Godot runtime - tested via ferris-test"]
fn test_export_range_property() {
    godot::init();
    
    let hint = PropertyHint::Range { min: 0, max: 100, step: 1 };
    let result = map_hint(&hint);
    
    assert_eq!(result.hint_string(), "0,100,1");
}
```

**Why Ignored?** Many GDExtension tests require `godot::init()` which can't run in standard unit tests. These are covered by integration tests instead. See [TESTING_GUIDE.md - Why Some Tests Are Ignored](TESTING_GUIDE.md#why-some-tests-are-ignored) for details.

---

### Benchmark Testing

**What**: Performance benchmarks using Criterion.rs  
**When**: Measuring compiler/runtime performance, regression detection  
**Location**: `crates/*/benches/`  

**Documentation**:

- [TESTING_GUIDE.md - Pattern 4](TESTING_GUIDE.md#pattern-4-benchmark-tests)
- [BENCHMARK_BASELINE.md](../BENCHMARK_BASELINE.md) - Performance baselines

**Quick Start**:

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench compilation

# Run with baseline comparison
cargo bench -- --save-baseline main
cargo bench -- --baseline main
```

**Benchmark Suites**:

- `compilation` - Full pipeline benchmarks
- `lexer` - Tokenization performance
- `parser` - AST construction performance
- `type_checker` - Type checking performance
- `execution` - Runtime execution performance

**Baselines**: See [BENCHMARK_BASELINE.md](../BENCHMARK_BASELINE.md) for v0.0.4 performance targets

---

## 📊 Test Matrices

Test matrices provide **systematic tracking** of test scenarios across all test types. Use these to:

- Identify coverage gaps
- Plan new test cases
- Track testing progress for specific features
- Target areas for improvement

### Node Queries & Signals

**Files**:

- [TEST_MATRIX_NODE_QUERIES_SIGNALS.md](TEST_MATRIX_NODE_QUERIES_SIGNALS.md) - Systematic test scenario tracking
- [TEST_COVERAGE_ANALYSIS_NODE_QUERIES_SIGNALS.md](TEST_COVERAGE_ANALYSIS_NODE_QUERIES_SIGNALS.md) - Detailed coverage analysis

**Coverage Summary** (as of v0.0.4):

| Feature Category | Total Scenarios | Tested | Coverage | Priority Gaps |
|------------------|----------------|--------|----------|---------------|
| **Node Query Functions** | 45 | 23 | 51% | Relative/absolute paths, Unicode paths, edge cases |
| **Signal System** | 30 | 18 | 60% | Error propagation, signal validation, performance |
| **Overall** | 75 | 41 | 55% | Cross-cutting concerns (security, performance) |

**Status Legend**: ✅ PASS | ⚠️ PARTIAL | ❌ TODO | 🚧 IN PROGRESS | 💥 FAIL

**Quick Links**:

- [Node Query Test Matrix](TEST_MATRIX_NODE_QUERIES_SIGNALS.md#node-query-tests) - `get_node()`, `get_parent()`, `has_node()`, `find_child()`
- [Signal Test Matrix](TEST_MATRIX_NODE_QUERIES_SIGNALS.md#signal-tests) - `signal`, `emit_signal`, error cases
- [Coverage Analysis](TEST_COVERAGE_ANALYSIS_NODE_QUERIES_SIGNALS.md#coverage-matrix) - Detailed analysis by test type

### Future Test Matrices

**Planned** (as features are added):

- `TEST_MATRIX_ARRAYS_COLLECTIONS.md` - Array operations, for loops, iteration (v0.0.6)
- `TEST_MATRIX_GODOT_API.md` - Godot API bindings, node lifecycle (v0.0.7)
- `TEST_MATRIX_TYPE_SYSTEM.md` - Extended types, casting, type safety (v0.2.0)
- `TEST_MATRIX_ARITHMETIC_SAFETY.md` - Checked/saturating/wrapping methods (v0.3.0)

**Want to add a test matrix?** Follow the pattern in [TEST_MATRIX_NODE_QUERIES_SIGNALS.md](TEST_MATRIX_NODE_QUERIES_SIGNALS.md) and update this README.

---

## 🛠️ Test Harness & Tooling

### ferris-test CLI

**Purpose**: Run .ferris integration tests headlessly with Godot  
**Location**: `crates/test_harness/src/main.rs`  
**Configuration**: `ferris-test.toml` in workspace root

**Features**:

- Headless Godot execution
- Test metadata parsing (TEST, CATEGORY, EXPECT, ASSERT)
- Output marker parsing ([TEST_START], [PASS], [FAIL], [TEST_END])
- Multiple output formats (console, JSON, JUnit)
- Parallel test execution
- Timeout handling
- CI/CD integration

**Documentation**:

- [TESTING_GUIDE.md - Pattern 2](TESTING_GUIDE.md#pattern-2-integration-tests-ferris-scripts)
- [TEST_HARNESS_TESTING_STRATEGY.md](TEST_HARNESS_TESTING_STRATEGY.md) - Architecture and design

**Usage Examples**:

```bash
# Basic usage
ferris-test --all

# Filter tests
ferris-test --all --filter "export"

# JSON output for CI
ferris-test --all --format json > results.json

# Verbose debugging
ferris-test --all --verbose

# Single test
ferris-test --script godot_test/scripts/signal_test.ferris
```

**Configuration** (`ferris-test.toml`):

```toml
godot_executable = "path/to/godot.exe"
project_path = "./godot_test"
timeout_seconds = 30
output_format = "console"
verbose = false
```

**Environment Overrides**:

- `GODOT_BIN` - Override Godot executable path
- `GODOT_PROJECT_PATH` - Override project path
- `GODOT_TIMEOUT` - Override timeout (seconds)

---

## 🔗 Related Documentation

### Core Documentation

- **[TESTING_GUIDE.md](TESTING_GUIDE.md)** - Complete testing patterns and procedures ⭐ **START HERE**
- **[DEVELOPMENT.md](../DEVELOPMENT.md)** - Development workflow (includes testing setup)
- **[CONTRIBUTING.md](../CONTRIBUTING.md)** - Contribution guidelines (includes testing requirements)
- **[ARCHITECTURE.md](../ARCHITECTURE.md)** - System architecture (includes test layer design)

### Testing Strategy & Analysis

- [TEST_HARNESS_TESTING_STRATEGY.md](TEST_HARNESS_TESTING_STRATEGY.md) - Test harness architecture
- [TEST_COVERAGE_ANALYSIS_NODE_QUERIES_SIGNALS.md](TEST_COVERAGE_ANALYSIS_NODE_QUERIES_SIGNALS.md) - Coverage analysis
- [TEST_MATRIX_NODE_QUERIES_SIGNALS.md](TEST_MATRIX_NODE_QUERIES_SIGNALS.md) - Test scenario tracking

### Version-Specific Documentation

- **v0.0.4**:
  - [TESTING_STRATEGY_PHASE5.md](../planning/v0.0.4/TESTING_STRATEGY_PHASE5.md) - Detailed Phase 5 strategy (1533 lines)
  - [INTEGRATION_TESTS_REPORT.md](../planning/v0.0.4/INTEGRATION_TESTS_REPORT.md) - Phase 5 test results
  - [INTEGRATION_TESTS_FIXES.md](../planning/v0.0.4/INTEGRATION_TESTS_FIXES.md) - Bug fixes from testing

### Performance & Coverage

- [BENCHMARK_BASELINE.md](../BENCHMARK_BASELINE.md) - Performance baselines and targets
- [COVERAGE_SETUP_NOTES.md](../COVERAGE_SETUP_NOTES.md) - Code coverage setup (tarpaulin)

---

## ✅ Testing Checklist for New Features

Use this checklist when adding new functionality:

### 1. Unit Tests (Required)

- [ ] Add tests in `#[cfg(test)] mod tests` for pure logic
- [ ] Test happy path (valid inputs)
- [ ] Test error cases (invalid inputs)
- [ ] Test edge cases (empty, null, boundary values)
- [ ] Verify error messages are clear and actionable

### 2. Integration Tests (Required for User-Facing Features)

- [ ] Create `.ferris` script in `godot_test/scripts/`
- [ ] Add test metadata (TEST, CATEGORY, EXPECT, ASSERT)
- [ ] Use output markers ([TEST_START], [PASS], [FAIL], [TEST_END])
- [ ] Test end-to-end workflow
- [ ] Verify behavior in Godot runtime

### 3. GDExtension Tests (If Applicable)

- [ ] Add tests in `crates/*/tests/headless_integration.rs` if requiring Godot runtime
- [ ] Mark as `#[ignore]` with reason if Godot init required
- [ ] Ensure coverage via integration tests instead

### 4. Benchmarks (If Performance-Critical)

- [ ] Add benchmark in `crates/*/benches/`
- [ ] Compare against baseline
- [ ] Document performance expectations

### 5. Documentation

- [ ] Update [TESTING_GUIDE.md](TESTING_GUIDE.md) if new pattern added
- [ ] Add test matrix entry if systematic tracking needed
- [ ] Update this README if new test type added

### 6. CI/CD

- [ ] Verify tests run in CI pipeline (check `.github/workflows/`)
- [ ] Ensure tests are headless (no GUI dependencies)
- [ ] Test passes on all platforms (Windows, Linux, macOS)

---

## 🚨 Troubleshooting

### Common Issues

**Problem**: "Tests pass locally but fail in CI"

- **Solution**: Check CI has Godot installed, uses headless variant, builds GDExtension first

**Problem**: "GDExtension not loaded in tests"

- **Solution**: Build with `cargo build --release`, verify `ferrisscript.gdextension` paths

**Problem**: "ferris-test command not found"

- **Solution**: Run `cargo build --release` in test_harness crate, or use `cargo run --bin ferris-test -- --all`

**Problem**: "Test marked as failed but output looks correct"

- **Solution**: Ensure markers ([TEST_START], [PASS], [FAIL], [TEST_END]) are present, run with `--verbose`

**Problem**: Copilot suggests creating duplicate testing infrastructure

- **Solution**: ⚠️ **Testing infrastructure already exists!** Point to [TESTING_GUIDE.md](TESTING_GUIDE.md) and this README

**Full Troubleshooting**: See [TESTING_GUIDE.md - Troubleshooting](TESTING_GUIDE.md#troubleshooting)

---

## 📈 Testing Metrics & Goals

### Current State (v0.0.4)

- **Total Tests**: 843+ (unit + integration + benchmarks)
- **Code Coverage**: ~82%
- **Integration Tests**: 15+ end-to-end scenarios
- **Test Execution Time**: <30s for full suite
- **CI Status**: All tests passing

### Goals (v0.1.0)

- **Total Tests**: 200+ (focused on quality over quantity)
- **Code Coverage**: 80%+ (excluding bindings)
- **Integration Tests**: 30+ scenarios
- **Test Matrices**: Systematic tracking for all major features
- **Documentation**: 100% of testing patterns documented

### Continuous Improvement

- **Add test matrices** for new features as they're developed
- **Track coverage gaps** using analysis documents
- **Improve test harness** based on [TEST_HARNESS_TESTING_STRATEGY.md](TEST_HARNESS_TESTING_STRATEGY.md)
- **Update this README** as testing practices evolve

---

## 💡 Testing Best Practices

1. **Test at the Right Layer**: Don't use integration tests for unit-testable logic
2. **Use Existing Infrastructure**: Leverage `ferris-test` and `test_harness` instead of creating new tools
3. **Document Test Intent**: Every test should clearly state what it validates
4. **Make Tests CI-Friendly**: All tests must run headlessly without GUI
5. **Optimize for Fast Feedback**: Unit tests <1s, integration tests <30s
6. **Track Coverage Systematically**: Use test matrices to identify gaps
7. **Update Documentation**: Keep [TESTING_GUIDE.md](TESTING_GUIDE.md) and this README in sync

---

## 🔄 Document Maintenance

**Update Frequency**: After each version release or when testing patterns change

**Review Triggers**:

- New testing pattern added
- New test type introduced
- Test coverage targets changed
- Test harness upgraded
- CI/CD pipeline modified

**Ownership**: Project Lead (solo dev) + Community Contributors

**Last Review**: October 10, 2025 (v0.0.4 completion)

---

**Questions or Issues?** See [CONTRIBUTING.md](../CONTRIBUTING.md) or open a GitHub issue.

**Want to improve testing?** Check [TEST_HARNESS_TESTING_STRATEGY.md](TEST_HARNESS_TESTING_STRATEGY.md) for enhancement opportunities.
