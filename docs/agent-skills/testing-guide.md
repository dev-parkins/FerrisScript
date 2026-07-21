# Testing Guide

**Load this skill when adding or modifying tests**

## Testing Strategy Overview

FerrisScript uses a **4-layer testing strategy**:

```
┌─────────────────────────────────────────────┐
│   Layer 4: Manual Testing (Godot Editor)    │  ← Feature validation
├─────────────────────────────────────────────┤
│   Layer 3: Integration Tests (.ferris)      │  ← End-to-end behavior
├─────────────────────────────────────────────┤
│   Layer 2: GDExtension Tests (GDScript)     │  ← Godot bindings
├─────────────────────────────────────────────┤
│   Layer 1: Unit Tests (Rust)                │  ← Pure logic
└─────────────────────────────────────────────┘
```

**Current status:** 843+ tests, ~82% coverage

## Layer 1: Unit Tests (Rust)

### Location

Unit tests are co-located in each crate's `src/` directory using inline `#[cfg(test)]` modules:

```rust
// crates/compiler/src/parser.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_variable_declaration() {
        let source = "let x: i32 = 42;";
        let ast = Parser::new(source).parse().unwrap();
        assert_eq!(ast.declarations.len(), 1);
    }
}
```

### Naming Convention

```rust
// CORRECT — descriptive names
#[test]
fn test_type_mismatch_string_to_i32() { ... }

#[test]
fn test_parser_handles_nested_functions() { ... }

// WRONG — vague names
#[test]
fn test_types() { ... }

#[test]
fn test_parser() { ... }
```

**Pattern:** `test_<component>_<scenario>`

### Running Unit Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p ferrisscript_compiler

# Specific test
cargo test test_type_mismatch_string_to_i32 -- --exact

# With output
cargo test -- --nocapture
```

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_success_case() {
        let source = "let x: i32 = 42;";
        let result = compile(source);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_error_case() {
        let source = "let x: i32 = \"hello\";";
        let result = compile(source);
        assert!(result.is_err());
        
        let err = result.unwrap_err();
        assert_eq!(err.code, ErrorCode::E201);
        assert!(err.message.contains("type mismatch"));
    }
    
    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_unimplemented_feature() {
        // Test that unimplemented features panic with clear message
        unimplemented!("Feature X not yet supported");
    }
}
```

### Coverage Targets

| Crate | Target | Current |
|-------|--------|---------|
| compiler | 85%+ | ~85% |
| runtime | 85%+ | ~80% |
| godot_bind | 70%+ | ~70% |
| test_harness | 90%+ | ~90% |

### Running Coverage

```bash
# Local (cross-platform)
./scripts/coverage.sh  # Linux/macOS
.\scripts\coverage.ps1  # Windows

# Manual
cargo llvm-cov --workspace --html --output-dir target/coverage
```

## Layer 2: GDExtension Tests

### Location

GDExtension tests are in `crates/godot_bind/src/lib.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_value_variant_conversion() {
        let value = Value::I32(42);
        let variant = value_to_variant(&value);
        let back = variant_to_value(&variant);
        assert_eq!(back, Value::I32(42));
    }
}
```

### Limitations

Many GDExtension tests require Godot runtime and are marked `#[ignore]`:

```rust
#[test]
#[ignore]  // Requires Godot runtime
fn test_godot_node_creation() {
    // This test runs via integration tests instead
}
```

## Layer 3: Integration Tests (.ferris)

### Location

Integration tests are `.ferris` scripts in `godot_test/scripts/`:

```
godot_test/
├── project.godot
├── ferrisscript.gdextension
└── scripts/
    ├── signal_test.ferris
    ├── export_properties_test.ferris
    └── process_test.ferris
```

### Test Metadata Format

```ferris
// TEST: signal_test
// CATEGORY: integration
// DESCRIPTION: Test signal declaration and emission
// EXPECT: success
// ASSERT: Signal emitted successfully
// ASSERT: health_changed signal received

signal health_changed(new_health: i32);

let mut health: i32 = 100;

fn _ready() {
    health = health - 25;
    emit_signal("health_changed", health);
    print("Signal emitted successfully");
}
```

**Metadata fields:**

- `TEST:` — Test name (unique identifier)
- `CATEGORY:` — Test category (integration, unit, etc.)
- `DESCRIPTION:` — What the test validates
- `EXPECT:` — Expected outcome (success, failure, error)
- `ASSERT:` — Expected output markers (can have multiple)

### Running Integration Tests

```bash
# All integration tests
ferris-test --all

# Specific test
ferris-test --script godot_test/scripts/signal_test.ferris

# Filter by name
ferris-test --all --filter "signal"

# Verbose output
ferris-test --all --verbose

# JSON output (for CI)
ferris-test --all --format json > results.json
```

### Test Harness (ferris-test)

The test harness is defined in `crates/test_harness/`:

```rust
// crates/test_harness/src/main.rs
fn main() {
    let args = Cli::parse();
    
    let results = if args.all {
        runner.run_all_tests(&args.filter)
    } else if let Some(script) = &args.script {
        runner.run_single_test(script)
    } else {
        eprintln!("Specify --all or --script");
        std::process::exit(1);
    };
    
    match args.format {
        OutputFormat::Console => print_console(&results),
        OutputFormat::Json => print_json(&results),
        OutputFormat::JUnit => print_junit(&results),
    }
}
```

### Writing Integration Tests

**Basic test:**

```ferris
// TEST: hello_world
// CATEGORY: integration
// EXPECT: success
// ASSERT: Hello from FerrisScript!

fn _ready() {
    print("Hello from FerrisScript!");
}
```

**Test with assertions:**

```ferris
// TEST: variable_types
// CATEGORY: integration
// EXPECT: success
// ASSERT: x = 42
// ASSERT: y = 3.14

let x: i32 = 42;
let y: f32 = 3.14;

fn _ready() {
    print("x = ", x);
    print("y = ", y);
}
```

**Test expecting error:**

```ferris
// TEST: type_error
// CATEGORY: integration
// EXPECT: error
// ASSERT: E201

let x: i32 = "hello";  // Should fail with E201
```

## Layer 4: Manual Testing (Godot Editor)

### Setup

```bash
# 1. Build the extension
cargo build --package ferrisscript_godot_bind

# 2. Open Godot
# Import godot_test/project.godot

# 3. Create test scene
# Add FerrisScriptNode
# Set script_path in Inspector
# Run scene (F5)
```

### What to Test

- Inspector property editing
- Signal connection UI
- Hot reload behavior
- Visual output
- Performance in editor

## Test Checklist for New Features

When adding a new feature, ensure:

- [ ] **Unit tests** in relevant crate (`crates/*/src/*.rs`)
- [ ] **Integration test** in `godot_test/scripts/` (if Godot-related)
- [ ] **Error case tests** (what happens when feature is misused?)
- [ ] **Edge case tests** (empty input, boundary values, etc.)
- [ ] **Coverage maintained** (run `./scripts/coverage.sh`)
- [ ] **All tests pass** (`cargo test --workspace`)
- [ ] **Integration tests pass** (`ferris-test --all`)

## Common Testing Patterns

### Testing Compiler Errors

```rust
#[test]
fn test_undefined_variable_error() {
    let source = "fn test() { print(x); }";
    let result = compile(source);
    
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, ErrorCode::E202);
    assert!(err.message.contains("undefined variable"));
    assert!(err.hint.is_some());
}
```

### Testing Runtime Behavior

```rust
#[test]
fn test_variable_scoping() {
    let source = r#"
        let x: i32 = 10;
        fn test() -> i32 {
            let y: i32 = 20;
            return x + y;
        }
    "#;
    
    let mut runtime = Runtime::new(source);
    let result = runtime.execute().unwrap();
    assert_eq!(result, Value::I32(30));
}
```

### Testing Godot Integration

```ferris
// TEST: node_query
// CATEGORY: integration
// EXPECT: success
// ASSERT: Found node: Player

fn _ready() {
    if has_node("Player") {
        let player = get_node("Player");
        print("Found node: Player");
    }
}
```

## Debugging Failed Tests

### Unit Test Failures

```bash
# Run with output
cargo test test_name -- --nocapture

# Run specific crate
cargo test -p ferrisscript_compiler -- --nocapture
```

### Integration Test Failures

```bash
# Verbose output
ferris-test --script godot_test/scripts/test.ferris --verbose

# Check Godot console
# Open godot_test/ in Godot editor and run manually
```

### Coverage Gaps

```bash
# Generate HTML report
cargo llvm-cov --workspace --html --output-dir target/coverage

# Open report
xdg-open target/coverage/html/index.html  # Linux
open target/coverage/html/index.html      # macOS
```

Look for red lines (not covered) and add tests for those code paths.

## CI Testing

GitHub Actions runs:

```yaml
# .github/workflows/ci.yml
- name: Run tests
  run: cargo test --workspace

- name: Run integration tests
  run: ferris-test --all

- name: Check coverage
  run: cargo tarpaulin --workspace --out Xml
```

**PR requirements:**

- All unit tests pass
- All integration tests pass
- Coverage doesn't decrease by more than 2%

## Test Organization Tips

**Do:**

- Group related tests in the same `mod tests` block
- Use descriptive test names that explain the scenario
- Test both success and error cases
- Include edge cases (empty input, max values, etc.)
- Keep tests independent (no shared state)

**Don't:**

- Skip tests without a comment explaining why
- Write tests that depend on execution order
- Duplicate test logic (use helper functions)
- Ignore failing tests (fix or remove them)
- Write tests that are too broad (test one thing per test)
