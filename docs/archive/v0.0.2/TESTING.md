# FerrisScript Testing Guide (v0.0.2) 🦀

**Version**: 0.0.2  
**Last Updated**: October 5, 2025  
**Status**: Active

---

## 📋 Overview

This guide covers testing practices, tools, and workflows for FerrisScript v0.0.2. It's intended for contributors and maintainers who want to write tests, understand test coverage, and maintain code quality.

**Key Testing Goals for v0.0.2**:

- **Line Coverage**: 70-75% (up from 65-70%)
- **Branch Coverage**: 55-60% (up from 50-55%)
- **Test Count**: 116 tests (up from 96)
- **All Tests Passing**: Required for all PRs

---

## 🚀 Quick Start

### Running All Tests

```bash
# Run all tests in workspace
cargo test --workspace

# Run tests with output
cargo test --workspace -- --nocapture

# Run specific test
cargo test test_name

# Run tests for specific crate
cargo test --package ferrisscript_compiler
cargo test --package ferrisscript_runtime
```

### Expected Output

```
running 116 tests
test compiler::tests::test_empty_script ... ok
test compiler::tests::test_hello_world ... ok
test runtime::tests::test_arithmetic ... ok
...
test result: ok. 116 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 📝 Writing Tests

### Test Organization

Tests are organized in two ways:

1. **Unit Tests**: In `mod tests` blocks within source files
2. **Integration Tests**: In `tests/` directories (future)

**Current Structure**:

```
crates/
  compiler/
    src/
      lexer.rs          # Contains `mod tests` for lexer
      parser.rs         # Contains `mod tests` for parser
      type_checker.rs   # Contains `mod tests` for type checker
  runtime/
    src/
      lib.rs            # Contains `mod tests` for runtime
```

### Test Naming Conventions

Follow these patterns for consistency:

```rust
#[test]
fn test_<feature>_<scenario>() {
    // Example: test_lexer_empty_file()
    // Example: test_parser_function_declaration()
    // Example: test_type_checker_type_mismatch()
}

#[test]
fn test_<feature>_<edge_case>() {
    // Example: test_lexer_long_identifier()
    // Example: test_parser_deeply_nested_expressions()
}

#[test]
fn test_<feature>_error_<condition>() {
    // Example: test_type_checker_error_undefined_variable()
    // Example: test_parser_error_missing_semicolon()
}
```

### Unit Test Template

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_basic() {
        // Arrange: Set up test data
        let input = "test input";
        
        // Act: Perform operation
        let result = function_under_test(input);
        
        // Assert: Verify result
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_value);
    }
    
    #[test]
    fn test_feature_error_condition() {
        // Arrange
        let input = "invalid input";
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected error message"));
    }
}
```

### Testing Best Practices

#### 1. Test One Thing Per Test

❌ **Bad** - Multiple assertions unrelated:

```rust
#[test]
fn test_everything() {
    assert!(lexer_works());
    assert!(parser_works());
    assert!(runtime_works());
}
```

✅ **Good** - Focused test:

```rust
#[test]
fn test_lexer_tokenizes_keywords() {
    let tokens = lex("fn let mut");
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].kind, TokenKind::Fn);
    assert_eq!(tokens[1].kind, TokenKind::Let);
    assert_eq!(tokens[2].kind, TokenKind::Mut);
}
```

#### 2. Use Descriptive Test Names

❌ **Bad**:

```rust
#[test]
fn test1() { ... }

#[test]
fn test_parser() { ... }
```

✅ **Good**:

```rust
#[test]
fn test_parser_function_with_multiple_parameters() { ... }

#[test]
fn test_lexer_string_with_escape_sequences() { ... }
```

#### 3. Test Error Cases

Every error path should have at least one test:

```rust
#[test]
fn test_type_checker_error_type_mismatch() {
    let code = r#"
        fn main() {
            let x: i32 = "string";  // Type error
        }
    "#;
    
    let result = compile(code);
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    assert!(error.contains("Type mismatch"));
    assert!(error.contains("expected i32, found String"));
}
```

#### 4. Test Edge Cases

Common edge cases to test:

- **Empty inputs**: Empty files, empty strings, empty arrays
- **Boundary values**: Min/max integers, very long identifiers
- **Special characters**: Unicode, escape sequences, whitespace
- **Nested structures**: Deeply nested expressions, long call chains
- **Comments**: Comments-only files, comments in unusual positions

Example:

```rust
#[test]
fn test_parser_empty_file() {
    let result = parse("");
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.functions.len(), 0);
    assert_eq!(program.global_vars.len(), 0);
}

#[test]
fn test_lexer_long_identifier() {
    let long_name = "a".repeat(10000);
    let tokens = lex(&long_name);
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Identifier);
}
```

#### 5. Use Test Fixtures for Common Data

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Test fixture: Common test data
    fn sample_program() -> &'static str {
        r#"
        fn main() {
            let x: i32 = 42;
            print(x);
        }
        "#
    }
    
    #[test]
    fn test_parser_with_fixture() {
        let code = sample_program();
        let result = parse(code);
        assert!(result.is_ok());
    }
}
```

---

## 📊 Test Coverage

### Current Coverage (v0.0.2)

**Estimated Coverage** (as of manual analysis):

- **Line Coverage**: 70-75%
- **Branch Coverage**: 55-60%
- **Test Count**: 116 tests
- **Crates**:
  - `ferrisscript_compiler`: ~80% coverage
  - `ferrisscript_runtime`: ~75% coverage
  - `ferrisscript_godot_bind`: ~20% coverage (minimal testing)

**Target Coverage** (v0.0.2 goal):

- **Line Coverage**: 80%
- **Branch Coverage**: 70%

See [TEST_COVERAGE_ANALYSIS.md](TEST_COVERAGE_ANALYSIS.md) for detailed gap analysis.

### Generating Coverage Reports

#### Using cargo-llvm-cov (Local Development)

```bash
# Install cargo-llvm-cov (one-time setup)
cargo install cargo-llvm-cov

# Generate HTML coverage report
cargo llvm-cov --html --workspace

# Open report (Windows)
start target/llvm-cov/html/index.html

# Open report (Linux)
xdg-open target/llvm-cov/html/index.html

# Open report (macOS)
open target/llvm-cov/html/index.html
```

#### Using Cross-Platform Scripts

```bash
# Windows PowerShell
.\scripts\coverage.ps1

# Linux/macOS Bash
./scripts/coverage.sh
```

These scripts:

1. Check for required tools (cargo-llvm-cov or cargo-tarpaulin)
2. Run coverage analysis across all workspace crates
3. Generate HTML report in `target/coverage/html/index.html`
4. Generate LCOV report in `target/coverage/lcov.info`

See [scripts/README.md](../../scripts/README.md) for more details.

### Coverage Goals by Module

| Module | Current | Target (v0.0.2) | Target (v0.1.0) |
|--------|---------|-----------------|-----------------|
| Lexer | 80% | 85% | 90% |
| Parser | 75% | 80% | 90% |
| Type Checker | 70% | 80% | 90% |
| Runtime | 75% | 80% | 90% |
| Godot Bind | 20% | 30% | 70% |
| **Overall** | **70-75%** | **80%** | **90%** |

---

## 🐛 Testing Error Messages

FerrisScript v0.0.2 significantly improved error messages. Test them thoroughly:

### Error Message Testing Template

```rust
#[test]
fn test_error_message_includes_context() {
    let code = r#"
    fn main() {
        let x: i32 = "oops";
    }
    "#;
    
    let result = compile(code);
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    
    // Verify error includes:
    // 1. Line number
    assert!(error.contains("line 3"));
    
    // 2. Error message
    assert!(error.contains("Type mismatch"));
    
    // 3. Expected vs actual
    assert!(error.contains("expected i32"));
    assert!(error.contains("found String"));
    
    // 4. Helpful hint (if applicable)
    assert!(error.contains("Hint:"));
}
```

### Error Message Quality Checklist

For all error tests, verify:

- ✅ **Line number included** (`line 3, column 12`)
- ✅ **Source context shown** (`let x: i32 = "oops";` with visual indicator)
- ✅ **Clear error description** ("Type mismatch: expected i32, found String")
- ✅ **Helpful hint** (when applicable, e.g., "Hint: String literals must be of type String")

---

## 🧪 Test Categories

### 1. Core Functionality Tests

Test the happy path for all features:

```rust
#[test]
fn test_compiler_hello_world() {
    let code = r#"
        fn _ready() {
            print("Hello, World!");
        }
    "#;
    
    let result = compile(code);
    assert!(result.is_ok());
}
```

### 2. Edge Case Tests

Test boundary conditions and unusual inputs:

```rust
#[test]
fn test_lexer_empty_file() { ... }

#[test]
fn test_lexer_comments_only() { ... }

#[test]
fn test_lexer_long_identifier() { ... }

#[test]
fn test_parser_deeply_nested_expressions() { ... }
```

**v0.0.2 Edge Case Coverage**:

- ✅ Empty script files (4 tests)
- ✅ Scripts with only comments (5 tests)
- ✅ Long variable names (6 tests)
- ⏸️ Very large number literals (deferred)
- ⏸️ Deeply nested expressions (deferred)

### 3. Error Condition Tests

Test that invalid inputs produce appropriate errors:

```rust
#[test]
fn test_type_checker_undefined_variable() { ... }

#[test]
fn test_parser_missing_semicolon() { ... }

#[test]
fn test_lexer_invalid_token() { ... }
```

### 4. Integration Tests (Future)

Test interactions between components:

```rust
// tests/integration_test.rs (planned for v0.0.3+)
#[test]
fn test_full_pipeline_hello_world() {
    let code = load_example("hello.ferris");
    let compiled = compile(code).expect("Compilation failed");
    let result = execute(compiled).expect("Execution failed");
    assert_eq!(result.exit_code, 0);
}
```

### 5. Performance Tests (Benchmarks)

Benchmarks are in `benches/` directories (using criterion):

```bash
# Run benchmarks
cargo bench --workspace

# Run specific benchmark
cargo bench --bench lexer_bench
```

**Current Benchmarks** (v0.0.2):

- Lexer: 384 ns - 3.74 μs
- Parser: 600 ns - 7.94 μs
- Type Checker: 851 ns - 3.58 μs
- Runtime: 1.05 μs per function call

See [BENCHMARK_BASELINE.md](BENCHMARK_BASELINE.md) for detailed results.

---

## 🔧 Testing Workflow

### Before Creating a PR

1. **Run all tests**:

   ```bash
   cargo test --workspace
   ```

2. **Run clippy**:

   ```bash
   cargo clippy --workspace --tests -- -D warnings
   ```

3. **Format code**:

   ```bash
   cargo fmt --all
   ```

4. **Check coverage** (optional but recommended):

   ```bash
   .\scripts\coverage.ps1  # Windows
   ./scripts/coverage.sh   # Linux/macOS
   ```

5. **Run benchmarks** (if performance-sensitive changes):

   ```bash
   cargo bench --workspace
   ```

### During Development

- **Run tests frequently**: After each significant change
- **Write tests first**: TDD approach encouraged
- **Test error paths**: Don't just test happy paths
- **Keep tests fast**: Unit tests should run in milliseconds

### After Merging

- **Monitor CI**: Ensure all platforms pass
- **Review coverage reports**: Check for coverage regressions
- **Update docs**: If test patterns change

---

## 🎯 Testing Checklist

Use this checklist when adding new features:

### Feature Implementation Checklist

- [ ] **Happy path test**: Basic functionality works
- [ ] **Edge case tests**: Empty inputs, boundary values, special characters
- [ ] **Error tests**: Invalid inputs produce appropriate errors
- [ ] **Error messages**: Include line numbers, context, hints
- [ ] **Documentation**: Rustdoc comments for public APIs
- [ ] **Integration**: Feature works with other components
- [ ] **Performance**: No significant performance regression

### PR Testing Checklist

- [ ] All tests pass: `cargo test --workspace`
- [ ] No clippy warnings: `cargo clippy --workspace --tests`
- [ ] Code formatted: `cargo fmt --all --check`
- [ ] Coverage maintained: Coverage hasn't decreased
- [ ] Benchmarks stable: No unexpected performance changes
- [ ] Documentation updated: README, CHANGELOG, etc.

---

## 🐞 Troubleshooting Tests

### Tests Are Failing

1. **Check error messages**: Read test output carefully
2. **Run specific test**: `cargo test test_name -- --nocapture`
3. **Check recent changes**: `git diff`
4. **Verify dependencies**: `cargo build`

### Tests Are Slow

1. **Profile tests**: Use `cargo test -- --nocapture --test-threads=1`
2. **Check for heavy operations**: File I/O, network calls
3. **Use mocks**: Mock external dependencies
4. **Parallelize**: Ensure tests can run in parallel

### Coverage Not Generating

See [COVERAGE_SETUP_NOTES.md](../COVERAGE_SETUP_NOTES.md) for:

- Platform-specific coverage setup
- Troubleshooting cargo-llvm-cov
- Troubleshooting cargo-tarpaulin
- CI coverage integration

---

## 📚 Additional Resources

### Documentation

- [CONTRIBUTING.md](../../CONTRIBUTING.md) - Contribution guidelines
- [TEST_COVERAGE_ANALYSIS.md](TEST_COVERAGE_ANALYSIS.md) - Detailed coverage analysis
- [BENCHMARK_BASELINE.md](BENCHMARK_BASELINE.md) - Performance benchmarks
- [COVERAGE_SETUP_NOTES.md](../COVERAGE_SETUP_NOTES.md) - Coverage tool setup

### Test Examples

Study existing tests for patterns:

- `crates/compiler/src/lexer.rs` - Lexer tests (tokenization)
- `crates/compiler/src/parser.rs` - Parser tests (AST generation)
- `crates/compiler/src/type_checker.rs` - Type checker tests
- `crates/runtime/src/lib.rs` - Runtime tests (execution)

### Testing Tools

- **cargo test**: Built-in test runner
- **cargo-llvm-cov**: Coverage reports (recommended)
- **cargo-tarpaulin**: Alternative coverage tool (Linux)
- **criterion**: Benchmarking framework
- **cargo-watch**: Auto-run tests on file changes

---

## 🔮 Future Testing Plans

### v0.0.3 (Editor Experience Alpha)

- Add integration tests in `tests/` directory
- Increase coverage to 85%+
- Add property-based tests with proptest
- Enhanced error message testing

### v0.0.4 (Godot API Expansion)

- Godot integration tests
- End-to-end tests with Godot project
- Increase godot_bind coverage to 70%+

### v0.1.0 (MVP Release)

- Comprehensive test suite (90%+ coverage)
- Performance regression tests
- Cross-platform CI testing (Linux, Windows, macOS)
- Stress tests for large scripts

---

## 📝 Version History

- **v0.0.2** (October 2025): Initial testing guide, 116 tests, 70-75% coverage
- **v0.0.1** (September 2025): 96 tests, basic coverage

---

**Questions or Issues?**

- See [FAQ.md](../FAQ.md) for common testing questions
- See [TROUBLESHOOTING.md](../TROUBLESHOOTING.md) for debugging help
- Open an issue: [GitHub Issues](https://github.com/dev-parkins/FerrisScript/issues)
