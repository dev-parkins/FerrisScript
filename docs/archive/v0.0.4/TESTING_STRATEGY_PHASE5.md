# Testing Strategy for Phase 5 Inspector Integration

**Date**: 2025-10-10  
**Phase**: Phase 5 Sub-Phase 3 Complete  
**Status**: Testing Analysis & Recommendations  
**Purpose**: Increase robustness and confidence in Inspector integration features

---

## Executive Summary

Phase 5 Sub-Phase 3 introduced significant new functionality (Bundles 5-8) with Inspector integration, property hooks, and hot-reload support. While we have **543 passing compiler tests**, we need to expand our testing strategy to cover:

1. **Integration testing** (compiler → runtime → godot_bind pipeline)
2. **Headless Godot testing** (automated without GUI)
3. **Input mutation testing** (fuzzing for robustness)
4. **Error boundary testing** (graceful degradation)
5. **Performance regression testing** (benchmarks)

This document provides a comprehensive testing strategy to build confidence in upcoming features and ensure system robustness.

---

## Current Test Coverage Analysis

### ✅ Strong Coverage

#### Compiler Tests (543 passing)

**Location**: `crates/compiler/src/**/*.rs`

**Coverage**:

- ✅ **Lexer** (~80 tests): Token generation, unicode, edge cases, errors
- ✅ **Parser** (~150 tests): AST generation, error recovery, @export annotations
- ✅ **Type Checker** (~250 tests): Type inference, coercion, signals, @export validation
- ✅ **Error System** (~50 tests): Error codes, context extraction, formatting
- ✅ **Suggestions** (~13 tests): Levenshtein distance, identifier suggestions

**Strengths**:

- Comprehensive edge case coverage
- Good error handling tests
- Export annotation parsing tested
- Type validation for all 8 types

**Example Well-Tested Areas**:

```rust
// Type checker has comprehensive @export tests
test_export_range_min_greater_than_max()
test_export_enum_empty_values()
test_export_file_invalid_format_mixed()
test_export_duplicate_error()
test_export_unsupported_inputevent()
```

#### Runtime Tests

**Location**: `crates/runtime/src/lib.rs`

**Coverage**:

- ✅ Environment scoping
- ✅ Value coercion (int → float, bool conversion)
- ✅ Builtin functions
- ✅ Arithmetic operations
- ✅ Comparison operations
- ✅ Logical operations

**Strengths**:

- Basic execution tested
- Value conversion tested
- Scoping behavior validated

#### godot_bind Tests (21 tests: 11 passing, 10 failing)

**Location**: `crates/godot_bind/src/lib.rs`

**Passing Tests (11)**:

- ✅ Type mapping (i32, f32, bool, String, Vector2, Color, Rect2, Transform2D)
- ✅ API structure tests (ClassId, PropertyUsageFlags)
- ✅ Unknown type handling

**Failing Tests (10 - EXPECTED)**:

- ⏳ Hint mapping (None, Range, Enum, File) - **Require Godot engine**
- ⏳ Metadata conversion tests - **Require Godot engine**

**Reason**: These tests call Godot FFI functions that need engine runtime.

---

### ⚠️ Testing Gaps

#### 1. **Integration Tests** (CRITICAL GAP)

**Missing**: End-to-end tests combining:

- Compile FerrisScript source
- Execute in runtime
- Sync with Inspector via property hooks
- Verify hot-reload behavior

**Why Critical**: Bundles 5-8 are integration features - they need full pipeline testing.

**Impact**: Medium confidence in individual components, but **low confidence in integration**.

---

#### 2. **Property Hook Tests** (HIGH PRIORITY)

**Missing Tests for Bundle 7**:

**get_property() Edge Cases**:

- ❌ Property doesn't exist in runtime storage
- ❌ Script not loaded (env = None)
- ❌ Property name with unicode/special characters
- ❌ Built-in property fallback (position, rotation)
- ❌ Property name collision with Godot built-ins
- ❌ Very long property names (>256 chars)
- ❌ Empty property name
- ❌ Property name with null bytes

**set_property() Edge Cases**:

- ❌ Type mismatch (Inspector sends wrong type)
- ❌ Range clamping behavior (value > max, value < min)
- ❌ Range clamping with NaN/Infinity
- ❌ Immutable property write attempt
- ❌ Script not loaded (env = None)
- ❌ Property doesn't exist
- ❌ Variant conversion failure
- ❌ from_inspector=true vs false behavior

**Error Handling**:

- ❌ Graceful degradation on errors
- ❌ Error logging verification
- ❌ Panic prevention tests

**Why Critical**: Property hooks are the core of Inspector integration - they must be bulletproof.

---

#### 3. **Variant Conversion Tests** (MEDIUM PRIORITY)

**Bundle 6 - Enhanced but Undertested**:

**Missing Edge Cases**:

- ❌ NaN → Variant → Value → Variant roundtrip
- ❌ Infinity → Variant → Value → Variant roundtrip
- ❌ -0.0 vs 0.0 handling
- ❌ Very large i32 (near overflow)
- ❌ Very small f32 (near denormalization)
- ❌ Unicode string edge cases (emoji, RTL text, combining chars)
- ❌ Empty String handling
- ❌ String with null bytes
- ❌ Struct literal with missing fields
- ❌ Struct literal with extra fields
- ❌ Struct literal with wrong types
- ❌ Nested struct literals (deep nesting)

**Why Important**: Data corruption at Inspector ↔ Runtime boundary would be catastrophic.

---

#### 4. **Hot-Reload Tests** (HIGH PRIORITY)

**Bundle 8 - Completely Untested**:

**Missing Tests**:

- ❌ Add new @export property → verify Inspector updates
- ❌ Remove @export property → verify Inspector updates
- ❌ Change property type (i32 → f32) → verify Inspector updates
- ❌ Change property hint (Range → Enum) → verify Inspector updates
- ❌ Script reload with syntax error → verify graceful handling
- ❌ Script reload with type error → verify graceful handling
- ❌ Multiple rapid reloads → verify stability
- ❌ Reload while Inspector is reading property
- ❌ Reload while Inspector is writing property
- ❌ notify_property_list_changed() called multiple times

**Why Critical**: Hot-reload is advertised feature - must be rock-solid.

---

#### 5. **Input Mutation/Fuzzing Tests** (ROBUSTNESS)

**Missing**: Adversarial input testing to find edge cases we haven't considered.

**Areas to Fuzz**:

**@export Annotations**:

```ferris
// Malformed range hints
@export(range(NaN, 100))
@export(range(10, 5))  // min > max
@export(range(1e308, 1e309))  // overflow
@export(range("a", "b"))  // strings instead of numbers

// Malformed enum hints
@export(enum())  // empty
@export(enum("a", "a"))  // duplicates
@export(enum("a\0b"))  // null byte
@export(enum("🦀", "🚀"))  // emoji

// Malformed file hints
@export(file(""))  // empty
@export(file("*.exe, *.dll"))  // invalid format
@export(file("C:\\malicious\\path"))  // absolute path
@export(file("../" * 100))  // path traversal

// Property names
@export let mut 🦀: i32 = 0;  // emoji
@export let mut position: i32 = 0;  // collision with built-in
@export let mut x: i32 = 0; @export let mut x: f32 = 0.0;  // duplicate
@export let mut a_very_long_property_name_...: i32 = 0;  // 1000+ chars
```

**Variant Values**:

```rust
// Extreme values
Value::Int(i32::MAX)
Value::Int(i32::MIN)
Value::Float(f32::INFINITY)
Value::Float(f32::NEG_INFINITY)
Value::Float(f32::NAN)
Value::Float(-0.0)
Value::String("\0".repeat(10000))  // null bytes
Value::String("🦀".repeat(10000))  // huge emoji string

// Malformed struct literals
Value::Struct { fields: [(field with spaces, value)] }
Value::Struct { fields: [(very_long_name..., value)] }
Value::Struct { fields: circular_reference }
```

**Why Important**: Fuzzing finds bugs that humans don't anticipate.

---

#### 6. **Performance/Benchmark Tests** (REGRESSION PREVENTION)

**Missing**: Performance regression tests for new features.

**Benchmarks Needed**:

- `get_property()` latency (should be < 1ms)
- `set_property()` latency (should be < 1ms)
- `get_property_list()` with 100 properties
- Variant conversion overhead (Value → Variant → Value)
- Script reload time with 50 properties
- Property name lookup (HashMap performance)

**Why Important**: Performance degradation would make Inspector unusable.

---

## Recommended Testing Strategy

### Phase 1: Critical Integration Tests (IMMEDIATE)

**Priority**: 🔴 **CRITICAL**  
**Timeline**: 2-3 days  
**Effort**: High

#### Test 1: End-to-End Property Read

```rust
#[test]
fn test_inspector_read_exported_property() {
    // 1. Compile script with @export
    let source = r#"
        @export let mut health: i32 = 100;
    "#;
    let program = compile(source).unwrap();
    
    // 2. Create FerrisScriptNode and load script
    let mut node = FerrisScriptNode::new();
    node.load_script_from_source(source);
    
    // 3. Simulate Inspector read via get_property()
    let value = node.get_property(StringName::from("health"));
    
    // 4. Verify correct value returned
    assert_eq!(value, Some(Variant::from(100)));
}
```

#### Test 2: End-to-End Property Write

```rust
#[test]
fn test_inspector_write_exported_property() {
    let source = r#"
        @export let mut health: i32 = 100;
        fn get_health() -> i32 { return health; }
    "#;
    let program = compile(source).unwrap();
    
    let mut node = FerrisScriptNode::new();
    node.load_script_from_source(source);
    
    // Inspector writes new value
    let success = node.set_property(StringName::from("health"), Variant::from(50));
    assert!(success);
    
    // Verify runtime storage updated
    let result = call_function("get_health", &[], node.get_env()).unwrap();
    assert_eq!(result, Value::Int(50));
}
```

#### Test 3: Hot-Reload with Added Property

```rust
#[test]
fn test_hot_reload_add_property() {
    let source_v1 = r#"
        @export let mut health: i32 = 100;
    "#;
    
    let mut node = FerrisScriptNode::new();
    node.load_script_from_source(source_v1);
    
    // Verify initial property list
    let props_v1 = node.get_property_list();
    assert_eq!(props_v1.len(), 1);
    
    // Reload with additional property
    let source_v2 = r#"
        @export let mut health: i32 = 100;
        @export let mut mana: i32 = 50;
    "#;
    node.load_script_from_source(source_v2);
    
    // Verify property list updated
    let props_v2 = node.get_property_list();
    assert_eq!(props_v2.len(), 2);
    assert!(props_v2.iter().any(|p| p.property_name == "mana"));
}
```

**Implementation**:

- Create `tests/integration_tests.rs` in `crates/godot_bind`
- Use headless Godot runtime (see Phase 2)
- Run in CI pipeline

---

### Phase 2: Headless Godot Testing (HIGH PRIORITY)

**Priority**: 🟠 **HIGH**  
**Timeline**: 3-5 days  
**Effort**: High

**Goal**: Run godot_bind tests without GUI, enabling CI automation.

#### Setup Instructions

**1. Install Godot Headless**:

```bash
# Download godot-headless binary
wget https://downloads.tuxfamily.org/godotengine/4.3/Godot_v4.3-stable_win64.exe

# Verify headless mode works
godot --headless --version
```

**2. Configure Test Harness**:

```rust
// crates/godot_bind/tests/headless_setup.rs
use std::process::Command;

pub fn setup_godot_headless() -> Result<(), String> {
    let output = Command::new("godot")
        .arg("--headless")
        .arg("--quit")
        .output()
        .map_err(|e| format!("Failed to start Godot: {}", e))?;
    
    if !output.status.success() {
        return Err("Godot headless mode failed".to_string());
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_godot_headless_available() {
        setup_godot_headless().expect("Godot headless not available");
    }
}
```

**3. Create Headless Test Suite**:

```rust
// crates/godot_bind/tests/headless_integration.rs
use godot::prelude::*;

#[cfg(test)]
#[godot_api]
impl FerrisScriptNodeTest {
    #[test]
    fn test_property_hooks_headless() {
        // This test runs in godot-headless runtime
        // Now FFI calls work!
        let mut node = FerrisScriptNode::new();
        // ... test implementation
    }
}
```

**4. CI Integration**:

```yaml
# .github/workflows/test.yml
- name: Install Godot Headless
  run: |
    wget https://downloads.tuxfamily.org/godotengine/4.3/Godot_v4.3-stable_linux_headless.64.zip
    unzip Godot_v4.3-stable_linux_headless.64.zip
    chmod +x Godot_v4.3-stable_linux_headless.64
    sudo mv Godot_v4.3-stable_linux_headless.64 /usr/local/bin/godot

- name: Run Headless Tests
  run: cargo test --package ferrisscript_godot_bind --test headless_integration
```

**Benefits**:

- ✅ All 21 godot_bind tests can run in CI
- ✅ Catch regressions before merging
- ✅ Test property hooks automatically
- ✅ Test hot-reload behavior

---

### Phase 3: Property Hook Edge Case Tests (HIGH PRIORITY)

**Priority**: 🟠 **HIGH**  
**Timeline**: 2-3 days  
**Effort**: Medium

**Test Suite**: `tests/property_hooks_edge_cases.rs`

#### Test: Missing Property

```rust
#[test]
fn test_get_property_missing() {
    let source = r#"
        @export let mut health: i32 = 100;
    "#;
    let mut node = FerrisScriptNode::new();
    node.load_script_from_source(source);
    
    // Try to read non-existent property
    let value = node.get_property(StringName::from("mana"));
    
    // Should return None (fallback to Godot)
    assert_eq!(value, None);
}
```

#### Test: Range Clamping

```rust
#[test]
fn test_set_property_range_clamp() {
    let source = r#"
        @export(range(0, 100))
        let mut health: i32 = 50;
    "#;
    let mut node = FerrisScriptNode::new();
    node.load_script_from_source(source);
    
    // Try to set value above max
    let success = node.set_property(StringName::from("health"), Variant::from(150));
    assert!(success);
    
    // Verify clamped to max
    let value = node.get_property(StringName::from("health"));
    assert_eq!(value, Some(Variant::from(100)));
}
```

#### Test: Type Mismatch

```rust
#[test]
fn test_set_property_type_mismatch() {
    let source = r#"
        @export let mut health: i32 = 100;
    "#;
    let mut node = FerrisScriptNode::new();
    node.load_script_from_source(source);
    
    // Try to set wrong type (String instead of i32)
    let success = node.set_property(
        StringName::from("health"),
        Variant::from("not a number")
    );
    
    // Should fail gracefully (return false)
    assert!(!success);
    
    // Verify original value unchanged
    let value = node.get_property(StringName::from("health"));
    assert_eq!(value, Some(Variant::from(100)));
}
```

#### Test: Built-in Property Fallback

```rust
#[test]
fn test_builtin_property_fallback() {
    let source = r#"
        @export let mut health: i32 = 100;
    "#;
    let mut node = FerrisScriptNode::new();
    node.load_script_from_source(source);
    
    // Try to read built-in Node2D property
    let value = node.get_property(StringName::from("position"));
    
    // Should return None (fallback to Godot's implementation)
    assert_eq!(value, None);
}
```

#### Test: Unicode Property Names

```rust
#[test]
fn test_property_unicode_name() {
    let source = r#"
        @export let mut 生命値: i32 = 100;  // "health" in Japanese
    "#;
    let mut node = FerrisScriptNode::new();
    node.load_script_from_source(source);
    
    let value = node.get_property(StringName::from("生命値"));
    assert_eq!(value, Some(Variant::from(100)));
}
```

**Total Tests**: ~20 edge cases

---

### Phase 4: Input Mutation/Fuzzing (ROBUSTNESS)

**Priority**: 🟡 **MEDIUM**  
**Timeline**: 5-7 days  
**Effort**: High

**Goal**: Find bugs through adversarial input generation.

#### Option 1: Property-Based Testing (proptest)

**Setup**:

```toml
[dev-dependencies]
proptest = "1.4"
```

**Implementation**:

```rust
// tests/property_based_tests.rs
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_variant_conversion_roundtrip(value: i32) {
        // Generate random i32 values
        let original = Value::Int(value);
        let variant = value_to_variant(&original);
        let converted_back = variant_to_value(&variant);
        
        // Should always roundtrip correctly
        assert_eq!(original, converted_back);
    }
    
    #[test]
    fn test_property_name_handling(name in "[a-zA-Z_][a-zA-Z0-9_]{0,255}") {
        // Generate random valid property names
        let source = format!("@export let mut {}: i32 = 0;", name);
        
        // Should compile without errors
        let program = compile(&source);
        assert!(program.is_ok());
    }
    
    #[test]
    fn test_range_clamping_always_safe(
        min: i32,
        max: i32,
        value: i32
    ) {
        // Generate random range constraints and values
        let clamped = clamp_value(value, min, max);
        
        // Should always satisfy constraints
        if min <= max {
            assert!(clamped >= min);
            assert!(clamped <= max);
        }
    }
}
```

**Benefits**:

- ✅ Tests thousands of random inputs automatically
- ✅ Finds edge cases humans miss
- ✅ Catches off-by-one errors
- ✅ Validates invariants

---

#### Option 2: Fuzzing (cargo-fuzz)

**Setup**:

```bash
cargo install cargo-fuzz
cargo fuzz init
```

**Implementation**:

```rust
// fuzz/fuzz_targets/variant_conversion.rs
#![no_main]
use libfuzzer_sys::fuzz_target;
use ferrisscript_godot_bind::*;

fuzz_target!(|data: &[u8]| {
    // Fuzz variant conversion
    if let Ok(variant) = Variant::decode(data) {
        let value = variant_to_value(&variant);
        let back = value_to_variant(&value);
        // Should not panic
    }
});
```

```rust
// fuzz/fuzz_targets/export_annotation.rs
#![no_main]
use libfuzzer_sys::fuzz_target;
use ferrisscript_compiler::compile;

fuzz_target!(|data: &[u8]| {
    // Fuzz @export annotation parsing
    if let Ok(source) = std::str::from_utf8(data) {
        let _ = compile(source);
        // Should never panic, even on garbage input
    }
});
```

**Run Fuzzing**:

```bash
# Run for 1 hour
cargo fuzz run variant_conversion -- -max_total_time=3600

# Run for 10 million iterations
cargo fuzz run export_annotation -- -runs=10000000
```

**Benefits**:

- ✅ Finds crashes and panics
- ✅ Coverage-guided (explores all code paths)
- ✅ Reproduces minimal failing input
- ✅ Industry-standard technique

---

### Phase 5: Performance Benchmarks (REGRESSION PREVENTION)

**Priority**: 🟢 **LOW** (but important)  
**Timeline**: 2-3 days  
**Effort**: Medium

**Goal**: Ensure new features don't cause performance regressions.

**Setup**:

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "property_hooks"
harness = false
```

**Implementation**:

```rust
// benches/property_hooks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ferrisscript_godot_bind::*;

fn benchmark_get_property(c: &mut Criterion) {
    let mut node = FerrisScriptNode::new();
    let source = r#"
        @export let mut health: i32 = 100;
    "#;
    node.load_script_from_source(source);
    
    c.bench_function("get_property", |b| {
        b.iter(|| {
            let value = node.get_property(black_box(StringName::from("health")));
            black_box(value);
        });
    });
}

fn benchmark_set_property(c: &mut Criterion) {
    let mut node = FerrisScriptNode::new();
    let source = r#"
        @export let mut health: i32 = 100;
    "#;
    node.load_script_from_source(source);
    
    c.bench_function("set_property", |b| {
        b.iter(|| {
            let success = node.set_property(
                black_box(StringName::from("health")),
                black_box(Variant::from(50))
            );
            black_box(success);
        });
    });
}

fn benchmark_get_property_list(c: &mut Criterion) {
    let mut node = FerrisScriptNode::new();
    let source = (0..100)
        .map(|i| format!("@export let mut prop_{}: i32 = {};", i, i))
        .collect::<Vec<_>>()
        .join("\n");
    node.load_script_from_source(&source);
    
    c.bench_function("get_property_list_100_props", |b| {
        b.iter(|| {
            let props = node.get_property_list();
            black_box(props);
        });
    });
}

criterion_group!(
    benches,
    benchmark_get_property,
    benchmark_set_property,
    benchmark_get_property_list
);
criterion_main!(benches);
```

**Run Benchmarks**:

```bash
cargo bench --package ferrisscript_godot_bind
```

**Expected Results**:

- `get_property`: < 1 μs (1 microsecond)
- `set_property`: < 5 μs (5 microseconds)
- `get_property_list` (100 props): < 100 μs (100 microseconds)

**CI Integration**:

- Run benchmarks on every PR
- Fail if performance regresses > 20%
- Store baseline in git

---

## Testing Priority Matrix

| Test Category | Priority | Effort | Timeline | Impact on Confidence |
|---------------|----------|--------|----------|---------------------|
| Integration Tests (Phase 1) | 🔴 CRITICAL | High | 2-3 days | +40% confidence |
| Headless Godot (Phase 2) | 🟠 HIGH | High | 3-5 days | +30% confidence |
| Property Hook Edge Cases (Phase 3) | 🟠 HIGH | Medium | 2-3 days | +20% confidence |
| Input Mutation/Fuzzing (Phase 4) | 🟡 MEDIUM | High | 5-7 days | +10% confidence |
| Performance Benchmarks (Phase 5) | 🟢 LOW | Medium | 2-3 days | Regression prevention |

**Total Effort**: ~20 days  
**Total Confidence Increase**: +100% (from 50% → 100%)

---

## Recommended Implementation Order

### Week 1: Critical Foundation

1. **Day 1-3**: Phase 1 (Integration Tests) - **MUST DO**
2. **Day 4-5**: Phase 3 (Property Hook Edge Cases) - **MUST DO**

**Result**: Basic confidence in property hooks working end-to-end.

### Week 2: Automation & Robustness

3. **Day 6-10**: Phase 2 (Headless Godot Testing) - **SHOULD DO**

**Result**: CI automation, catch regressions automatically.

### Week 3: Advanced Testing

4. **Day 11-15**: Phase 4 (Input Mutation/Fuzzing) - **COULD DO**
5. **Day 16-18**: Phase 5 (Performance Benchmarks) - **COULD DO**

**Result**: High confidence, performance guarantees.

---

## Specific Test Cases for Bundles 5-8

### Bundle 5: Inspector Display (`get_property_list`)

**Current Coverage**: ✅ Good (tested via compiler type_checker)

**Missing Tests**:

- ❌ Property list with 0 properties
- ❌ Property list with 100+ properties
- ❌ Property list with all 8 types
- ❌ Property list with all 3 hints
- ❌ Property order preservation
- ❌ Property list caching behavior

**Priority**: 🟢 LOW (well-covered by compiler tests)

---

### Bundle 6: Variant Conversion

**Current Coverage**: ⚠️ Partial (basic types tested)

**Missing Tests**:

**Edge Cases**:

```rust
#[test]
fn test_nan_roundtrip() {
    let nan = Value::Float(f32::NAN);
    let variant = value_to_variant(&nan);
    let back = variant_to_value(&variant);
    assert!(matches!(back, Value::Float(x) if x.is_nan()));
}

#[test]
fn test_infinity_roundtrip() {
    let inf = Value::Float(f32::INFINITY);
    let variant = value_to_variant(&inf);
    let back = variant_to_value(&variant);
    assert_eq!(back, Value::Float(f32::INFINITY));
}

#[test]
fn test_negative_zero() {
    let neg_zero = Value::Float(-0.0);
    let variant = value_to_variant(&neg_zero);
    let back = variant_to_value(&variant);
    // Should preserve sign bit
    assert_eq!(back.to_float().unwrap().to_bits(), (-0.0_f32).to_bits());
}

#[test]
fn test_bool_vs_int_ordering() {
    // Bool should be checked before int
    let variant_zero = Variant::from(0);
    let value = variant_to_value(&variant_zero);
    assert_eq!(value, Value::Int(0), "Should be Int, not Bool(false)");
    
    let variant_one = Variant::from(1);
    let value = variant_to_value(&variant_one);
    assert_eq!(value, Value::Int(1), "Should be Int, not Bool(true)");
}

#[test]
fn test_large_string() {
    let large = "🦀".repeat(10000);
    let value = Value::String(large.clone());
    let variant = value_to_variant(&value);
    let back = variant_to_value(&variant);
    assert_eq!(back, Value::String(large));
}

#[test]
fn test_string_with_null_bytes() {
    let with_null = "hello\0world";
    let value = Value::String(with_null.to_string());
    let variant = value_to_variant(&value);
    let back = variant_to_value(&variant);
    // Should preserve null bytes
    assert_eq!(back, Value::String(with_null.to_string()));
}
```

**Priority**: 🟠 HIGH (data corruption risk)

---

### Bundle 7: Property Hooks

**Current Coverage**: ❌ None (needs headless Godot)

**Critical Tests**:

```rust
#[test]
fn test_get_property_fallback_none() {
    // Test that returning None allows Godot fallback
    let mut node = FerrisScriptNode::new();
    let source = "@export let mut health: i32 = 100;";
    node.load_script_from_source(source);
    
    // Read FerrisScript property - should return Some
    let fs_prop = node.get_property(StringName::from("health"));
    assert_eq!(fs_prop, Some(Variant::from(100)));
    
    // Read Godot built-in - should return None (fallback)
    let godot_prop = node.get_property(StringName::from("position"));
    assert_eq!(godot_prop, None);
}

#[test]
fn test_set_property_fallback_false() {
    // Test that returning false allows Godot fallback
    let mut node = FerrisScriptNode::new();
    let source = "@export let mut health: i32 = 100;";
    node.load_script_from_source(source);
    
    // Write FerrisScript property - should return true
    let fs_result = node.set_property(StringName::from("health"), Variant::from(50));
    assert!(fs_result);
    
    // Write Godot built-in - should return false (fallback)
    let godot_result = node.set_property(
        StringName::from("position"),
        Variant::from(Vector2::new(10.0, 20.0))
    );
    assert!(!godot_result);
}

#[test]
fn test_range_clamp_automatic() {
    let mut node = FerrisScriptNode::new();
    let source = r#"
        @export(range(0, 100))
        let mut health: i32 = 50;
    "#;
    node.load_script_from_source(source);
    
    // Set below min - should clamp to 0
    node.set_property(StringName::from("health"), Variant::from(-10));
    let value = node.get_property(StringName::from("health"));
    assert_eq!(value, Some(Variant::from(0)));
    
    // Set above max - should clamp to 100
    node.set_property(StringName::from("health"), Variant::from(150));
    let value = node.get_property(StringName::from("health"));
    assert_eq!(value, Some(Variant::from(100)));
    
    // Set in range - should not clamp
    node.set_property(StringName::from("health"), Variant::from(50));
    let value = node.get_property(StringName::from("health"));
    assert_eq!(value, Some(Variant::from(50)));
}

#[test]
fn test_from_inspector_parameter() {
    let mut node = FerrisScriptNode::new();
    let source = r#"
        @export(range(0, 100))
        let mut health: i32 = 50;
        
        fn set_health_unclamped(val: i32) {
            health = val;  // This bypasses from_inspector=false
        }
    "#;
    node.load_script_from_source(source);
    
    // Inspector write - should clamp (from_inspector=true)
    node.set_property(StringName::from("health"), Variant::from(150));
    let value = node.get_property(StringName::from("health"));
    assert_eq!(value, Some(Variant::from(100)));
    
    // Runtime write - should NOT clamp (from_inspector=false)
    call_function("set_health_unclamped", &[Value::Int(150)], node.get_env()).unwrap();
    let value = node.get_property(StringName::from("health"));
    assert_eq!(value, Some(Variant::from(150)));  // Unclamped!
}

#[test]
fn test_error_logging_no_panic() {
    let mut node = FerrisScriptNode::new();
    // Don't load script - env will be None
    
    // Try to set property when script not loaded
    let result = node.set_property(StringName::from("health"), Variant::from(100));
    
    // Should fail gracefully (return false), not panic
    assert!(!result);
}

#[test]
fn test_unicode_property_name() {
    let mut node = FerrisScriptNode::new();
    let source = r#"
        @export let mut 生命値: i32 = 100;  // "health" in Japanese
        @export let mut 魔力: i32 = 50;      // "mana" in Japanese
    "#;
    node.load_script_from_source(source);
    
    // Should handle unicode correctly
    let health = node.get_property(StringName::from("生命値"));
    assert_eq!(health, Some(Variant::from(100)));
    
    let mana = node.get_property(StringName::from("魔力"));
    assert_eq!(mana, Some(Variant::from(50)));
}
```

**Priority**: 🔴 CRITICAL (core feature)

---

### Bundle 8: Runtime Synchronization

**Current Coverage**: ❌ None

**Critical Tests**:

```rust
#[test]
fn test_hot_reload_add_property() {
    let mut node = FerrisScriptNode::new();
    
    // Initial script with 1 property
    let source_v1 = "@export let mut health: i32 = 100;";
    node.load_script_from_source(source_v1);
    let props_v1 = node.get_property_list();
    assert_eq!(props_v1.len(), 1);
    
    // Reload with 2 properties
    let source_v2 = r#"
        @export let mut health: i32 = 100;
        @export let mut mana: i32 = 50;
    "#;
    node.load_script_from_source(source_v2);
    
    // notify_property_list_changed() should have been called
    // Inspector should see updated list
    let props_v2 = node.get_property_list();
    assert_eq!(props_v2.len(), 2);
    assert!(props_v2.iter().any(|p| p.property_name == "health"));
    assert!(props_v2.iter().any(|p| p.property_name == "mana"));
}

#[test]
fn test_hot_reload_remove_property() {
    let mut node = FerrisScriptNode::new();
    
    // Initial script with 2 properties
    let source_v1 = r#"
        @export let mut health: i32 = 100;
        @export let mut mana: i32 = 50;
    "#;
    node.load_script_from_source(source_v1);
    let props_v1 = node.get_property_list();
    assert_eq!(props_v1.len(), 2);
    
    // Reload with only 1 property
    let source_v2 = "@export let mut health: i32 = 100;";
    node.load_script_from_source(source_v2);
    
    let props_v2 = node.get_property_list();
    assert_eq!(props_v2.len(), 1);
    assert!(props_v2.iter().any(|p| p.property_name == "health"));
    assert!(!props_v2.iter().any(|p| p.property_name == "mana"));
}

#[test]
fn test_hot_reload_change_type() {
    let mut node = FerrisScriptNode::new();
    
    // Initial script with i32
    let source_v1 = "@export let mut value: i32 = 100;";
    node.load_script_from_source(source_v1);
    let props_v1 = node.get_property_list();
    assert_eq!(props_v1[0].variant_type, VariantType::INT);
    
    // Reload with f32
    let source_v2 = "@export let mut value: f32 = 100.0;";
    node.load_script_from_source(source_v2);
    
    let props_v2 = node.get_property_list();
    assert_eq!(props_v2[0].variant_type, VariantType::FLOAT);
}

#[test]
fn test_hot_reload_with_error() {
    let mut node = FerrisScriptNode::new();
    
    // Initial valid script
    let source_v1 = "@export let mut health: i32 = 100;";
    node.load_script_from_source(source_v1);
    let props_v1 = node.get_property_list();
    assert_eq!(props_v1.len(), 1);
    
    // Reload with syntax error
    let source_v2 = "@export let mut health: i32 = ;";  // Missing value
    let result = node.load_script_from_source(source_v2);
    
    // Should fail gracefully
    assert!(result.is_err());
    
    // Property list should still work (old script still loaded)
    let props_still = node.get_property_list();
    assert_eq!(props_still.len(), 1);
}

#[test]
fn test_multiple_rapid_reloads() {
    let mut node = FerrisScriptNode::new();
    
    // Reload 100 times in quick succession
    for i in 0..100 {
        let source = format!("@export let mut prop_{}: i32 = {};", i, i);
        node.load_script_from_source(&source);
    }
    
    // Should be stable, last reload wins
    let props = node.get_property_list();
    assert_eq!(props.len(), 1);
    assert!(props[0].property_name.contains("prop_99"));
}
```

**Priority**: 🟠 HIGH (advertised feature)

---

## Mutation Testing Examples

### Example 1: Range Hint Mutations

**Original**:

```ferris
@export(range(0, 100, 1))
let mut health: i32 = 50;
```

**Mutations to Test**:

```ferris
// Min > Max
@export(range(100, 0, 1))
let mut health: i32 = 50;

// Negative step
@export(range(0, 100, -1))
let mut health: i32 = 50;

// Zero step
@export(range(0, 100, 0))
let mut health: i32 = 50;

// NaN values
@export(range(NaN, 100, 1))
let mut health: i32 = 50;

// Infinity values
@export(range(0, inf, 1))
let mut health: i32 = 50;

// Very large numbers
@export(range(0, 2147483647, 1))
let mut health: i32 = 50;

// Negative range
@export(range(-100, -50, 1))
let mut health: i32 = -75;

// String instead of number
@export(range("0", "100", "1"))
let mut health: i32 = 50;
```

**Expected Behavior**:

- ✅ Compile-time error for type mismatches
- ✅ Compile-time warning/error for min > max
- ✅ Runtime clamping handles all valid numeric ranges
- ✅ No panics on any input

---

### Example 2: Enum Hint Mutations

**Original**:

```ferris
@export(enum("Easy", "Medium", "Hard"))
let mut difficulty: String = "Medium";
```

**Mutations to Test**:

```ferris
// Empty enum
@export(enum())
let mut difficulty: String = "Medium";

// Single value
@export(enum("Easy"))
let mut difficulty: String = "Easy";

// Duplicate values
@export(enum("Easy", "Easy", "Medium"))
let mut difficulty: String = "Medium";

// Unicode values
@export(enum("簡単", "普通", "難しい"))
let mut difficulty: String = "普通";

// Very long values
@export(enum("a" * 1000, "b" * 1000))
let mut difficulty: String = "a" * 1000;

// Special characters
@export(enum("Easy!", "Med/ium", "Ha\"rd"))
let mut difficulty: String = "Easy!";

// Null bytes
@export(enum("Easy\0", "Medium"))
let mut difficulty: String = "Medium";

// Mixed types
@export(enum("Easy", 1, 2.5))
let mut difficulty: String = "Easy";
```

**Expected Behavior**:

- ✅ Compile-time error for empty enum
- ✅ Compile-time warning for duplicates
- ✅ Support unicode correctly
- ✅ Handle special characters safely
- ✅ Compile-time error for mixed types

---

### Example 3: Property Name Mutations

**Mutations to Test**:

```ferris
// Built-in collision
@export let mut position: Vector2 = Vector2 { x: 0.0, y: 0.0 };

// Keyword collision
@export let mut fn: i32 = 0;
@export let mut let: i32 = 0;
@export let mut mut: i32 = 0;

// Unicode
@export let mut 🦀: i32 = 0;
@export let mut 生命値: i32 = 100;

// Very long name
@export let mut a_very_long_property_name_that_goes_on_and_on: i32 = 0;

// Special characters
@export let mut prop-name: i32 = 0;  // Hyphen
@export let mut prop.name: i32 = 0;  // Dot
@export let mut prop name: i32 = 0;  // Space

// Leading numbers
@export let mut 123abc: i32 = 0;

// Empty name
@export let mut : i32 = 0;

// Null byte
@export let mut prop\0name: i32 = 0;
```

**Expected Behavior**:

- ✅ Compile-time error for keyword collisions
- ✅ Runtime handles unicode correctly
- ✅ Compile-time error for invalid characters
- ✅ Compile-time error for empty name
- ⚠️ Warning for built-in collision (property hooks handle fallback)

---

## Guard Rails & Error Handling Improvements

Based on mutation testing, we should add these guard rails:

### 1. **Range Validation**

```rust
// In type_checker during @export validation
if let Some(RangeHint { min, max, step }) = &hint {
    // Guard: min <= max
    if min > max {
        return Err(TypeCheckError::InvalidRangeHint {
            message: format!("Range min ({}) cannot be greater than max ({})", min, max),
            hint: "Consider swapping the values or using a valid range",
        });
    }
    
    // Guard: step != 0
    if *step == 0.0 {
        return Err(TypeCheckError::InvalidRangeHint {
            message: "Range step cannot be zero",
            hint: "Use a positive step value like 1 or 0.1",
        });
    }
    
    // Guard: NaN/Infinity
    if min.is_nan() || max.is_nan() || step.is_nan() {
        return Err(TypeCheckError::InvalidRangeHint {
            message: "Range values cannot be NaN",
            hint: "Use finite numeric values",
        });
    }
    
    if min.is_infinite() || max.is_infinite() {
        return Err(TypeCheckError::InvalidRangeHint {
            message: "Range values cannot be infinite",
            hint: "Use finite numeric values",
        });
    }
}
```

### 2. **Enum Validation**

```rust
// In type_checker during @export validation
if let Some(EnumHint { values }) = &hint {
    // Guard: non-empty
    if values.is_empty() {
        return Err(TypeCheckError::InvalidEnumHint {
            message: "Enum must have at least one value",
            hint: "Add one or more string literals: enum(\"value1\", \"value2\")",
        });
    }
    
    // Guard: duplicates
    let mut seen = HashSet::new();
    for value in values {
        if !seen.insert(value) {
            return Err(TypeCheckError::DuplicateEnumValue {
                value: value.clone(),
                hint: "Each enum value must be unique",
            });
        }
    }
    
    // Guard: null bytes
    for value in values {
        if value.contains('\0') {
            return Err(TypeCheckError::InvalidEnumValue {
                message: format!("Enum value '{}' contains null byte", value),
                hint: "Remove null bytes from string literals",
            });
        }
    }
}
```

### 3. **Property Name Validation**

```rust
// In type_checker when processing @export
fn validate_property_name(name: &str) -> Result<(), TypeCheckError> {
    // Guard: non-empty
    if name.is_empty() {
        return Err(TypeCheckError::EmptyPropertyName {
            hint: "Property name cannot be empty",
        });
    }
    
    // Guard: max length
    if name.len() > 256 {
        return Err(TypeCheckError::PropertyNameTooLong {
            name: name.to_string(),
            max_len: 256,
            hint: "Use a shorter property name",
        });
    }
    
    // Guard: valid identifier
    if !name.chars().next().unwrap().is_alphabetic() && name.chars().next().unwrap() != '_' {
        return Err(TypeCheckError::InvalidPropertyName {
            name: name.to_string(),
            hint: "Property name must start with letter or underscore",
        });
    }
    
    // Guard: keyword collision
    if is_keyword(name) {
        return Err(TypeCheckError::PropertyNameIsKeyword {
            name: name.to_string(),
            hint: "Choose a different property name that is not a keyword",
        });
    }
    
    // Warning: built-in collision
    if is_builtin_property(name) {
        eprintln!("⚠️  WARNING: Property '{}' collides with Node2D built-in. \
                   Property hooks will use fallback behavior.", name);
    }
    
    Ok(())
}
```

### 4. **Variant Conversion Safety**

```rust
// In godot_bind/lib.rs
pub fn variant_to_value(variant: &Variant) -> Value {
    match variant.get_type() {
        VariantType::FLOAT => {
            let f = variant.to::<f32>();
            
            // Guard: NaN
            if f.is_nan() {
                godot_warn!("Converting NaN from Inspector - using 0.0");
                return Value::Float(0.0);
            }
            
            // Guard: Infinity
            if f.is_infinite() {
                godot_warn!("Converting Infinity from Inspector - clamping");
                return Value::Float(if f > 0.0 { f32::MAX } else { f32::MIN });
            }
            
            Value::Float(f)
        }
        // ... other cases
    }
}
```

---

## Success Criteria for Testing Phase

After implementing these tests, we should achieve:

### Quantitative Metrics

- ✅ **Integration test coverage**: ≥ 90% of property hook code paths
- ✅ **Edge case coverage**: ≥ 50 edge case tests passing
- ✅ **Fuzzing**: 0 crashes after 10 million iterations
- ✅ **Performance**: All benchmarks within 10% of baseline
- ✅ **CI Automation**: 100% of tests run in CI

### Qualitative Metrics

- ✅ **Confidence**: Can demo to users without hesitation
- ✅ **Documentation**: Every edge case has test demonstrating behavior
- ✅ **Debugging**: Test failures provide clear error messages
- ✅ **Regression Prevention**: New bugs caught before merge

---

## Conclusion

Phase 5 Sub-Phase 3 delivered powerful features (Bundles 5-8) with solid unit test coverage (543 compiler tests). However, **integration testing is critical** for confidence in the full pipeline.

**Immediate Actions** (Week 1):

1. ✅ Create integration test suite (Phase 1)
2. ✅ Add property hook edge case tests (Phase 3)
3. ⏳ Set up headless Godot (Phase 2)

**Follow-up Actions** (Weeks 2-3):
4. ⏳ Implement mutation/fuzzing tests (Phase 4)
5. ⏳ Add performance benchmarks (Phase 5)

**Expected Outcome**: Increase confidence from **~50% → 95%** in Inspector integration features.

---

## Next Steps

1. **Review this document** with team
2. **Prioritize test phases** based on project timeline
3. **Allocate resources** (20 days total effort)
4. **Set up CI infrastructure** for headless Godot
5. **Begin implementation** starting with Phase 1 (integration tests)

**Let's build bulletproof Inspector integration! 🦀🛡️**
