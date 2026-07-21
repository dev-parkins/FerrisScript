# Checkpoint 3.7 & 3.8 Execution Plan: Inspector Property Integration

**Date**: October 10, 2025  
**Phase**: Phase 5 Sub-Phase 3 - Runtime & Godot Integration  
**Status**: READY FOR EXECUTION  
**Estimated Time**: 3.5-4.0 hours (API verification + implementation + testing)

---

## Executive Summary

This execution plan consolidates feedback from three review documents to provide a complete, actionable roadmap for implementing bidirectional Inspector↔Runtime property synchronization in FerrisScript.

**Key Changes from Original Plan**:

1. ✅ Added **mandatory API verification phase** (from Feedback2)
2. ✅ Expanded Checkpoint 3.8 to include **property hooks** `get()`/`set()` (from Feedback1 & Feedback2)
3. ✅ Added **NaN/Infinity handling** for float conversions (from BIDIRECTIONAL_SYNC_EXAMPLE)
4. ✅ Incorporated **type conversion ordering** to avoid bool→int ambiguity (from BIDIRECTIONAL_SYNC_EXAMPLE)
5. ✅ Added **runtime synchronization** with `notify_property_list_changed()` (from all feedback)
6. ✅ Integrated **export_info_functions helpers** for robust hint strings (from Feedback1)
7. ✅ Split implementation into **atomic, testable bundles** following established workflow

**Sources**:

- `PROPERTYINFO_RESEARCH.md` v1.1 (original research + Feedback1 integration)
- `PROPERTYINFO_RESEARCH_FEEDBACK2.md` (second peer review)
- `BIDIRECTIONAL_SYNC_EXAMPLE.md` (technical reference implementation)

---

## Table of Contents

1. [Pre-Implementation Requirements](#pre-implementation-requirements)
2. [Bundle Breakdown](#bundle-breakdown)
3. [Detailed Implementation Steps](#detailed-implementation-steps)
4. [Testing Strategy](#testing-strategy)
5. [Risk Mitigation](#risk-mitigation)
6. [Success Criteria](#success-criteria)
7. [Rollback Plan](#rollback-plan)

---

## Pre-Implementation Requirements

### ✅ Already Complete

| Requirement | Status | Evidence |
|-------------|--------|----------|
| Per-instance property storage | ✅ | `Env.exported_properties: HashMap<String, Value>` |
| Runtime get/set methods | ✅ | `get_exported_property()`, `set_exported_property()` |
| Clamp-on-set policy | ✅ | Implemented in Bundle 2, tested |
| Variant conversion (partial) | ✅ | `value_to_variant()` exists for signals |
| Metadata extraction | ✅ | `compile()` populates `Program.property_metadata` |
| Integration test suite | ✅ | 3 test files (~500 LOC) ready |

### 🔄 Pending Critical Items

| Item | Why Critical | Estimated Time | Source |
|------|-------------|----------------|--------|
| API version verification | Avoid compilation failures | 15 min | Feedback2 |
| Enhanced variant_to_value() | Type ambiguity resolution | 20 min | BIDIRECTIONAL_SYNC_EXAMPLE |
| NaN/Infinity handling | Inspector crash prevention | 10 min | BIDIRECTIONAL_SYNC_EXAMPLE |
| Property hooks (get/set) | Core Inspector integration | 60 min | Feedback1 & Feedback2 |
| Runtime sync notification | Inspector updates | 20 min | All feedback sources |

---

## Bundle Breakdown

Following established workflow pattern: **Small, atomic, fully-tested bundles**.

### Bundle 4: API Verification & PropertyInfo Generation (Checkpoint 3.7 Part 1)

**Objective**: Verify godot-rust API availability and implement property list display.

**Estimated Time**: 75 minutes

**Files Modified**:

- `crates/godot_bind/src/lib.rs` (add imports, helper functions, test module)

**Deliverables**:

1. API verification tests (confirm `ClassId::none()` vs `::invalid()`)
2. Type mapping function (`map_type_to_variant`)
3. Hint mapping function (`map_hint`) with `export_info_functions`
4. PropertyInfo conversion function (`metadata_to_property_info`)
5. Unit tests for all helpers
6. Documentation of API variant used

**Success Criteria**:

- ✅ All unit tests pass
- ✅ API verification documents correct ClassId method
- ✅ Type mapping covers all 8 exportable types
- ✅ Hint mapping uses helper functions (not manual strings)
- ✅ No compilation errors

---

### Bundle 5: Inspector Property List Display (Checkpoint 3.7 Part 2)

**Objective**: Make properties visible in Godot Inspector (read-only initially).

**Estimated Time**: 45 minutes

**Files Modified**:

- `crates/godot_bind/src/lib.rs` (override `get_property_list()`)

**Deliverables**:

1. `get_property_list()` override in `INode2D` impl
2. Integration with `Program.property_metadata`
3. Manual Inspector verification test

**Success Criteria**:

- ✅ Properties appear in Inspector
- ✅ Correct types displayed (INT, FLOAT, etc.)
- ✅ Hints displayed correctly (range sliders, enum dropdowns, file pickers)
- ✅ All 653 existing tests still pass (regression check)

---

### Bundle 6: Enhanced Variant Conversion (Pre-Checkpoint 3.8)

**Objective**: Robust bidirectional Variant↔Value conversion with edge case handling.

**Estimated Time**: 45 minutes

**Files Modified**:

- `crates/godot_bind/src/lib.rs` (enhance `variant_to_value()`)
- `crates/runtime/src/lib.rs` (if Value→Variant needs updates)

**Deliverables**:

1. Enhanced `variant_to_value()` with ordered type checking
2. NaN/Infinity handling in `value_to_variant()`
3. Transform2D component extraction
4. Round-trip conversion unit tests

**Success Criteria**:

- ✅ Bool→int ambiguity resolved (bool checked first)
- ✅ NaN/Infinity converts to 0.0 (no crashes)
- ✅ All 8 types round-trip successfully
- ✅ 16+ unit tests pass (from BIDIRECTIONAL_SYNC_EXAMPLE)

---

### Bundle 7: Property Value Hooks (Checkpoint 3.8 Part 1)

**Objective**: Inspector can read and write property values.

**Estimated Time**: 75 minutes

**Files Modified**:

- `crates/godot_bind/src/lib.rs` (override `get()` and `set()`)

**Deliverables**:

1. `get()` override - reads from `Env.exported_properties`
2. `set()` override - writes to `Env.exported_properties` with clamping
3. Proper return values (`Option<Variant>`, `bool`)
4. Integration tests (Inspector edit → runtime reflects change)

**Success Criteria**:

- ✅ Inspector reads current property values
- ✅ Inspector edits update runtime
- ✅ Clamp-on-set applies to Inspector edits
- ✅ Non-exported properties still work (return None/false)
- ✅ All integration tests pass

---

### Bundle 8: Runtime Synchronization (Checkpoint 3.8 Part 2)

**Objective**: Runtime changes update Inspector display.

**Estimated Time**: 45 minutes

**Files Modified**:

- `crates/godot_bind/src/lib.rs` (add sync notification)
- `crates/runtime/src/lib.rs` (optional: call notification on assignment)

**Deliverables**:

1. `notify_property_list_changed()` implementation
2. Call notification after reload/script changes
3. (Optional) Call notification on exported property assignment
4. Hot-reload integration test

**Success Criteria**:

- ✅ Hot-reload updates Inspector
- ✅ Script property changes reflected in Inspector
- ✅ No performance regression (notification not spammed)

---

## Detailed Implementation Steps

### Bundle 4: API Verification & PropertyInfo Generation

#### Step 4.1: Add Imports (5 min)

**Source**: PROPERTYINFO_RESEARCH v1.1, Feedback2

```rust
// At top of crates/godot_bind/src/lib.rs
use godot::meta::{PropertyInfo, PropertyHintInfo, ClassId};
use godot::global::{PropertyHint, PropertyUsageFlags};
use godot::builtin::VariantType;
use godot::register::property::export_info_functions; // CRITICAL: For hint helpers
```

**Test**: `cargo check --package ferrisscript_godot_bind`

---

#### Step 4.2: API Verification Tests (15 min)

**Source**: Feedback2 (critical gap identified)

```rust
#[cfg(test)]
mod api_verification {
    use super::*;
    
    #[test]
    fn verify_classid_api() {
        // Try ClassId::none() first (most common in 0.4.0)
        let _class_id = ClassId::none();
        
        // If above fails, document and try:
        // let _class_id = ClassId::invalid();
        // let _class_id = ClassId::of::<()>();
        
        println!("ClassId API verified: using ClassId::none()");
    }
    
    #[test]
    fn verify_property_usage_flags() {
        let flags = PropertyUsageFlags::DEFAULT 
            | PropertyUsageFlags::EDITOR 
            | PropertyUsageFlags::STORAGE;
        
        println!("Usage flags: {:?}", flags);
        assert!(flags.contains(PropertyUsageFlags::EDITOR));
    }
    
    #[test]
    fn verify_export_info_functions() {
        let hint_info = export_info_functions::export_range(
            0.0, 100.0, Some(1.0),
            false, false, false, false, false, false, None
        );
        
        assert_eq!(hint_info.hint, PropertyHint::RANGE);
        println!("export_range hint_string: {}", hint_info.hint_string);
    }
}
```

**Run**: `cargo test --package ferrisscript_godot_bind api_verification`

**Document**: In commit message or implementation log, note which ClassId variant worked.

---

#### Step 4.3: Type Mapping Function (10 min)

**Source**: PROPERTYINFO_RESEARCH v1.1

```rust
/// Map FerrisScript type name to Godot VariantType
fn map_type_to_variant(type_name: &str) -> VariantType {
    match type_name {
        "i32" => VariantType::INT,
        "f32" => VariantType::FLOAT,
        "bool" => VariantType::BOOL,
        "String" => VariantType::STRING,
        "Vector2" => VariantType::VECTOR2,
        "Color" => VariantType::COLOR,
        "Rect2" => VariantType::RECT2,
        "Transform2D" => VariantType::TRANSFORM2D,
        _ => {
            godot_warn!("Unknown FerrisScript type '{}', defaulting to NIL", type_name);
            VariantType::NIL
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_map_type_to_variant_all_types() {
        assert_eq!(map_type_to_variant("i32"), VariantType::INT);
        assert_eq!(map_type_to_variant("f32"), VariantType::FLOAT);
        assert_eq!(map_type_to_variant("bool"), VariantType::BOOL);
        assert_eq!(map_type_to_variant("String"), VariantType::STRING);
        assert_eq!(map_type_to_variant("Vector2"), VariantType::VECTOR2);
        assert_eq!(map_type_to_variant("Color"), VariantType::COLOR);
        assert_eq!(map_type_to_variant("Rect2"), VariantType::RECT2);
        assert_eq!(map_type_to_variant("Transform2D"), VariantType::TRANSFORM2D);
        assert_eq!(map_type_to_variant("UnknownType"), VariantType::NIL);
    }
}
```

**Test**: `cargo test --package ferrisscript_godot_bind test_map_type_to_variant`

---

#### Step 4.4: Hint Mapping Function (20 min)

**Source**: PROPERTYINFO_RESEARCH v1.1 + Feedback2 (use helpers)

```rust
/// Map FerrisScript PropertyHint to Godot PropertyHintInfo
/// Uses export_info_functions for robust, platform-compatible hint strings
fn map_hint(hint: &ast::PropertyHint) -> PropertyHintInfo {
    match hint {
        ast::PropertyHint::None => PropertyHintInfo {
            hint: PropertyHint::NONE,
            hint_string: GString::new(),
        },
        
        ast::PropertyHint::Range { min, max, step } => {
            // ✅ Use helper function (not manual string formatting)
            export_info_functions::export_range(
                *min as f64,
                *max as f64,
                Some(*step as f64),
                false, // or_greater
                false, // or_less
                false, // exp
                false, // radians_as_degrees
                false, // degrees
                false, // hide_slider
                None,  // suffix (e.g., Some("px".to_string()))
            )
        },
        
        ast::PropertyHint::Enum { values } => PropertyHintInfo {
            hint: PropertyHint::ENUM,
            // Format: "Value1,Value2,Value3"
            hint_string: GString::from(values.join(",")),
        },
        
        ast::PropertyHint::File { extensions } => {
            // Add wildcard prefix if missing
            let formatted: Vec<String> = extensions
                .iter()
                .map(|ext| {
                    if ext.starts_with("*.") {
                        ext.clone()
                    } else if ext.starts_with('.') {
                        format!("*{}", ext)
                    } else {
                        format!("*.{}", ext)
                    }
                })
                .collect();
            
            PropertyHintInfo {
                hint: PropertyHint::FILE,
                // ⚠️ Use semicolons (not commas) for Windows compatibility
                hint_string: GString::from(formatted.join(";")),
            }
        },
    }
}

#[cfg(test)]
mod tests {
    // ... existing tests ...
    
    #[test]
    fn test_map_hint_range() {
        let hint = ast::PropertyHint::Range {
            min: 0.0,
            max: 100.0,
            step: 1.0,
        };
        let hint_info = map_hint(&hint);
        
        assert_eq!(hint_info.hint, PropertyHint::RANGE);
        // Hint string format validated by export_info_functions
        println!("Range hint string: {}", hint_info.hint_string);
    }
    
    #[test]
    fn test_map_hint_enum() {
        let hint = ast::PropertyHint::Enum {
            values: vec!["Low".to_string(), "Medium".to_string(), "High".to_string()],
        };
        let hint_info = map_hint(&hint);
        
        assert_eq!(hint_info.hint, PropertyHint::ENUM);
        assert_eq!(hint_info.hint_string.to_string(), "Low,Medium,High");
    }
    
    #[test]
    fn test_map_hint_file_extensions() {
        let hint = ast::PropertyHint::File {
            extensions: vec!["png".to_string(), ".jpg".to_string(), "*.gif".to_string()],
        };
        let hint_info = map_hint(&hint);
        
        assert_eq!(hint_info.hint, PropertyHint::FILE);
        let hint_str = hint_info.hint_string.to_string();
        
        // All should have wildcard prefix
        assert!(hint_str.contains("*.png"));
        assert!(hint_str.contains("*.jpg"));
        assert!(hint_str.contains("*.gif"));
        
        // Should use semicolons
        assert!(hint_str.contains(";"));
        assert!(!hint_str.contains(","));
    }
}
```

**Test**: `cargo test --package ferrisscript_godot_bind test_map_hint`

---

#### Step 4.5: PropertyInfo Conversion Function (15 min)

**Source**: PROPERTYINFO_RESEARCH v1.1

```rust
/// Convert FerrisScript PropertyMetadata to Godot PropertyInfo
fn metadata_to_property_info(metadata: &ast::PropertyMetadata) -> PropertyInfo {
    PropertyInfo {
        variant_type: map_type_to_variant(&metadata.type_name),
        
        // Use result from API verification (Step 4.2)
        class_id: ClassId::none(), // or ClassId::invalid() if documented
        
        property_name: StringName::from(&metadata.name),
        hint_info: map_hint(&metadata.hint),
        
        // CRITICAL: Must include EDITOR and STORAGE for Inspector visibility
        usage: PropertyUsageFlags::DEFAULT 
            | PropertyUsageFlags::EDITOR 
            | PropertyUsageFlags::STORAGE,
    }
}

#[cfg(test)]
mod tests {
    // ... existing tests ...
    
    #[test]
    fn test_metadata_to_property_info_basic() {
        let metadata = ast::PropertyMetadata {
            name: "health".to_string(),
            type_name: "i32".to_string(),
            hint: ast::PropertyHint::None,
            default_value: Some("100".to_string()),
        };
        
        let info = metadata_to_property_info(&metadata);
        
        assert_eq!(info.variant_type, VariantType::INT);
        assert_eq!(info.property_name.to_string(), "health");
        assert_eq!(info.hint_info.hint, PropertyHint::NONE);
        assert!(info.usage.contains(PropertyUsageFlags::DEFAULT));
        assert!(info.usage.contains(PropertyUsageFlags::EDITOR));
        assert!(info.usage.contains(PropertyUsageFlags::STORAGE));
    }
    
    #[test]
    fn test_metadata_to_property_info_with_range() {
        let metadata = ast::PropertyMetadata {
            name: "speed".to_string(),
            type_name: "f32".to_string(),
            hint: ast::PropertyHint::Range {
                min: 0.0,
                max: 10.0,
                step: 0.5,
            },
            default_value: Some("5.0".to_string()),
        };
        
        let info = metadata_to_property_info(&metadata);
        
        assert_eq!(info.variant_type, VariantType::FLOAT);
        assert_eq!(info.property_name.to_string(), "speed");
        assert_eq!(info.hint_info.hint, PropertyHint::RANGE);
    }
}
```

**Test**: `cargo test --package ferrisscript_godot_bind test_metadata_to_property_info`

---

#### Step 4.6: Commit Bundle 4 (5 min)

**Checkpoint**:

```bash
cargo test --package ferrisscript_godot_bind
cargo test --package ferrisscript_runtime
cargo test --package ferrisscript_compiler
```

**Expected**: All tests pass (653 total + new unit tests)

**Commit Message**:

```
Bundle 4: API Verification & PropertyInfo Generation (Checkpoint 3.7 Part 1)

- Add PropertyInfo/PropertyHint/PropertyUsageFlags imports
- Implement API verification tests (ClassId::none() confirmed)
- Implement map_type_to_variant() for 8 exportable types
- Implement map_hint() using export_info_functions helpers
- Implement metadata_to_property_info() conversion
- Add 12+ unit tests for all helper functions

Testing: All 653+ tests pass
Files: crates/godot_bind/src/lib.rs (~150 LOC added)
Time: ~75 minutes
Source: PROPERTYINFO_RESEARCH v1.1 + Feedback2
```

---

### Bundle 5: Inspector Property List Display

#### Step 5.1: Override get_property_list (15 min)

**Source**: PROPERTYINFO_RESEARCH v1.1

```rust
#[godot_api]
impl INode2D for FerrisScriptRunner {
    // ... existing methods (ready, process, etc.) ...
    
    /// Called by Godot to get list of properties for Inspector display
    /// Returns fresh PropertyInfo objects generated from static Program metadata
    fn get_property_list(&mut self) -> Vec<PropertyInfo> {
        // Get static metadata from compiled Program
        if let Some(program) = &self.compiled_program {
            program.property_metadata
                .iter()
                .map(metadata_to_property_info)
                .collect()
        } else {
            // No program loaded yet - return empty list
            Vec::new()
        }
    }
}
```

**Test**: Compilation only at this stage

```bash
cargo build --package ferrisscript_godot_bind
```

---

#### Step 5.2: Manual Inspector Test (20 min)

**Source**: Integration test suite created earlier

**Steps**:

1. Open Godot project (`godot_test/`)
2. Open `test_scene.tscn` or create new test scene
3. Add `FerrisScriptRunner` node
4. Set `script_path` to `"res://scripts/bounce_test.ferris"` (has exported properties)
5. Select node in Scene tree
6. **Verify Inspector shows**:
   - `health` (INT, range 0-100 with slider)
   - `speed` (FLOAT)
   - `jump_strength` (FLOAT, range 0-20 with slider)
   - `gravity` (FLOAT, range 0-1000 with slider)
7. **Take screenshot** or note verification in log

**Expected Behavior**:

- ✅ Properties appear under "FerrisScript Properties" section
- ✅ Correct types displayed
- ✅ Range sliders functional
- ⚠️ **Values show default (NOT current runtime values)** - this is expected for Bundle 5

---

#### Step 5.3: Regression Test (5 min)

**Run full test suite**:

```bash
cargo test
```

**Expected**: All 653+ tests still pass (no breakage)

---

#### Step 5.4: Commit Bundle 5 (5 min)

**Commit Message**:

```
Bundle 5: Inspector Property List Display (Checkpoint 3.7 Part 2)

- Override get_property_list() in INode2D impl
- Generate fresh PropertyInfo from Program.property_metadata
- Integrate with existing compiled_program field

Testing:
- Manual Inspector verification: Properties visible with correct types/hints
- All 653+ tests pass (regression check)
- Properties READ-ONLY at this stage (values in Bundle 7)

Files: crates/godot_bind/src/lib.rs (~20 LOC added)
Time: ~45 minutes
Source: PROPERTYINFO_RESEARCH v1.1
Status: Checkpoint 3.7 COMPLETE ✅
```

---

### Bundle 6: Enhanced Variant Conversion

#### Step 6.1: Enhance value_to_variant with NaN/Infinity Handling (15 min)

**Source**: BIDIRECTIONAL_SYNC_EXAMPLE (critical for Inspector stability)

```rust
/// Convert FerrisScript Value to Godot Variant
/// CRITICAL: Handles NaN/Infinity to prevent Inspector crashes
fn value_to_variant(value: &Value) -> Variant {
    match value {
        Value::Int(i) => Variant::from(*i),
        
        Value::Float(f) => {
            // ⚠️ CRITICAL: NaN/Infinity handling
            if f.is_nan() || f.is_infinite() {
                godot_warn!("Converting NaN/Infinity to 0.0 for Godot compatibility");
                Variant::from(0.0_f32)
            } else {
                Variant::from(*f)
            }
        },
        
        Value::Bool(b) => Variant::from(*b),
        Value::String(s) => Variant::from(s.as_str()),
        
        // Godot built-in types (already implemented in signal system)
        Value::Vector2 { x, y } => Variant::from(Vector2::new(*x, *y)),
        Value::Color { r, g, b, a } => Variant::from(Color::from_rgba(*r, *g, *b, *a)),
        Value::Rect2 { x, y, width, height } => Variant::from(Rect2::new(*x, *y, *width, *height)),
        
        Value::Transform2D { origin_x, origin_y, rotation, scale_x, scale_y } => {
            // Reconstruct Transform2D from components
            let mut transform = Transform2D::IDENTITY;
            transform = transform.rotated(*rotation);
            transform = transform.scaled(Vector2::new(*scale_x, *scale_y));
            transform = transform.translated(Vector2::new(*origin_x, *origin_y));
            Variant::from(transform)
        },
        
        // Fallback for unsupported types
        _ => {
            godot_warn!("Unsupported value type for Variant conversion: {:?}", value);
            Variant::nil()
        }
    }
}
```

**Note**: If `value_to_variant()` already exists from signal system, **modify** it to add NaN/Infinity handling.

---

#### Step 6.2: Implement Enhanced variant_to_value (20 min)

**Source**: BIDIRECTIONAL_SYNC_EXAMPLE (ordered type checking)

```rust
/// Convert Godot Variant to FerrisScript Value
/// CRITICAL: Type checking order prevents bool→int ambiguity
fn variant_to_value(variant: &Variant) -> Value {
    // ⚠️ Order matters! Try specific types before generic ones
    
    // 1. Try bool FIRST (before int, since bools can convert to int)
    if let Ok(b) = variant.try_to::<bool>() {
        return Value::Bool(b);
    }
    
    // 2. Try int
    if let Ok(i) = variant.try_to::<i32>() {
        return Value::Int(i);
    }
    
    // 3. Try float (after int to avoid precision loss)
    if let Ok(f) = variant.try_to::<f32>() {
        return Value::Float(f);
    }
    
    // 4. Try string
    if let Ok(s) = variant.try_to::<GString>() {
        return Value::String(s.to_string());
    }
    
    // 5. Try Godot built-in types
    if let Ok(v) = variant.try_to::<Vector2>() {
        return Value::Vector2 { x: v.x, y: v.y };
    }
    
    if let Ok(c) = variant.try_to::<Color>() {
        return Value::Color { r: c.r, g: c.g, b: c.b, a: c.a };
    }
    
    if let Ok(r) = variant.try_to::<Rect2>() {
        return Value::Rect2 {
            x: r.position.x,
            y: r.position.y,
            width: r.size.x,
            height: r.size.y,
        };
    }
    
    if let Ok(t) = variant.try_to::<Transform2D>() {
        // Extract components from Transform2D
        let origin = t.origin;
        let rotation = t.rotation();
        let scale = t.scale();
        
        return Value::Transform2D {
            origin_x: origin.x,
            origin_y: origin.y,
            rotation,
            scale_x: scale.x,
            scale_y: scale.y,
        };
    }
    
    // Fallback
    godot_warn!("Could not convert Variant to Value: {:?}", variant);
    Value::Nil
}
```

**Note**: This may already exist from Bundle 3. If so, **verify** type checking order and add Transform2D extraction if missing.

---

#### Step 6.3: Round-Trip Conversion Tests (10 min)

**Source**: BIDIRECTIONAL_SYNC_EXAMPLE (16 comprehensive tests)

```rust
#[cfg(test)]
mod variant_conversion_tests {
    use super::*;
    
    #[test]
    fn test_round_trip_primitives() {
        // Int
        let val_int = Value::Int(42);
        let var_int = value_to_variant(&val_int);
        let back_int = variant_to_value(&var_int);
        assert_eq!(val_int, back_int);
        
        // Float
        let val_float = Value::Float(3.14);
        let var_float = value_to_variant(&val_float);
        let back_float = variant_to_value(&var_float);
        assert_eq!(val_float, back_float);
        
        // Bool (CRITICAL: test ambiguity resolution)
        let val_bool = Value::Bool(true);
        let var_bool = value_to_variant(&val_bool);
        let back_bool = variant_to_value(&var_bool);
        assert_eq!(val_bool, back_bool);
        
        // String
        let val_str = Value::String("Hello".to_string());
        let var_str = value_to_variant(&val_str);
        let back_str = variant_to_value(&var_str);
        assert_eq!(val_str, back_str);
    }
    
    #[test]
    fn test_round_trip_vector2() {
        let val = Value::Vector2 { x: 10.0, y: 20.0 };
        let var = value_to_variant(&val);
        let back = variant_to_value(&var);
        
        match back {
            Value::Vector2 { x, y } => {
                assert!((x - 10.0).abs() < 0.001);
                assert!((y - 20.0).abs() < 0.001);
            }
            _ => panic!("Expected Vector2, got {:?}", back),
        }
    }
    
    #[test]
    fn test_round_trip_color() {
        let val = Value::Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 };
        let var = value_to_variant(&val);
        let back = variant_to_value(&var);
        
        match back {
            Value::Color { r, g, b, a } => {
                assert!((r - 1.0).abs() < 0.001);
                assert!((g - 0.5).abs() < 0.001);
                assert!((b - 0.0).abs() < 0.001);
                assert!((a - 1.0).abs() < 0.001);
            }
            _ => panic!("Expected Color, got {:?}", back),
        }
    }
    
    #[test]
    fn test_nan_infinity_handling() {
        // NaN should convert to 0.0
        let val_nan = Value::Float(f32::NAN);
        let var_nan = value_to_variant(&val_nan);
        let back_nan = variant_to_value(&var_nan);
        assert_eq!(back_nan, Value::Float(0.0));
        
        // Infinity should convert to 0.0
        let val_inf = Value::Float(f32::INFINITY);
        let var_inf = value_to_variant(&val_inf);
        let back_inf = variant_to_value(&var_inf);
        assert_eq!(back_inf, Value::Float(0.0));
        
        // Negative infinity
        let val_neg_inf = Value::Float(f32::NEG_INFINITY);
        let var_neg_inf = value_to_variant(&val_neg_inf);
        let back_neg_inf = variant_to_value(&var_neg_inf);
        assert_eq!(back_neg_inf, Value::Float(0.0));
    }
    
    #[test]
    fn test_transform2d_round_trip() {
        let val = Value::Transform2D {
            origin_x: 100.0,
            origin_y: 200.0,
            rotation: 1.57, // ~90 degrees
            scale_x: 2.0,
            scale_y: 3.0,
        };
        
        let var = value_to_variant(&val);
        let back = variant_to_value(&var);
        
        match back {
            Value::Transform2D { origin_x, origin_y, rotation, scale_x, scale_y } => {
                assert!((origin_x - 100.0).abs() < 0.1);
                assert!((origin_y - 200.0).abs() < 0.1);
                assert!((rotation - 1.57).abs() < 0.1);
                assert!((scale_x - 2.0).abs() < 0.1);
                assert!((scale_y - 3.0).abs() < 0.1);
            }
            _ => panic!("Expected Transform2D, got {:?}", back),
        }
    }
}
```

**Test**: `cargo test --package ferrisscript_godot_bind variant_conversion`

---

#### Step 6.4: Commit Bundle 6

**Commit Message**:

```
Bundle 6: Enhanced Variant Conversion (Pre-Checkpoint 3.8)

- Add NaN/Infinity handling in value_to_variant() (prevents Inspector crashes)
- Enhance variant_to_value() with ordered type checking (resolves bool→int ambiguity)
- Add Transform2D component extraction
- Add 16+ comprehensive round-trip conversion tests

Testing: All 653+ tests pass + 16 new conversion tests
Files: crates/godot_bind/src/lib.rs (~120 LOC added/modified)
Time: ~45 minutes
Source: BIDIRECTIONAL_SYNC_EXAMPLE
Critical: NaN/Infinity handling prevents production crashes
```

---

### Bundle 7: Property Value Hooks

#### Step 7.1: Implement get() Override (30 min)

**Source**: Feedback1, Feedback2, BIDIRECTIONAL_SYNC_EXAMPLE

```rust
#[godot_api]
impl INode2D for FerrisScriptRunner {
    // ... existing methods ...
    
    /// Called by Godot when Inspector reads a property value
    /// CRITICAL: Must be fast - called frequently by Inspector
    /// Returns None to let Godot handle non-exported properties normally
    fn get(&self, property: StringName) -> Option<Variant> {
        let prop_name = property.to_string();
        
        // Check if it's an exported property
        if let Some(env) = &self.env {
            match env.get_exported_property(&prop_name) {
                Ok(value) => {
                    // Convert FerrisScript Value to Godot Variant
                    return Some(value_to_variant(&value));
                }
                Err(_) => {
                    // Not an exported property - normal behavior
                    // (This is expected for Godot's built-in properties like position, rotation, etc.)
                }
            }
        }
        
        // Return None to let Godot handle the property normally
        None
    }
}
```

**Test**: Requires Inspector interaction (manual test in Step 7.3)

---

#### Step 7.2: Implement set() Override (30 min)

**Source**: Feedback1, Feedback2, BIDIRECTIONAL_SYNC_EXAMPLE

```rust
#[godot_api]
impl INode2D for FerrisScriptRunner {
    // ... existing methods ...
    
    /// Called by Godot when Inspector changes a property value
    /// CRITICAL: Must return true to indicate we handled it, false otherwise
    fn set(&mut self, property: StringName, value: Variant) -> bool {
        let prop_name = property.to_string();
        
        // Check if it's an exported property
        if let Some(env) = &mut self.env {
            // Convert Godot Variant to FerrisScript Value
            let fs_value = variant_to_value(&value);
            
            // Set with from_inspector=true (enables clamping for Inspector edits)
            match env.set_exported_property(&prop_name, fs_value, true) {
                Ok(_) => {
                    // Successfully handled - return true
                    return true;
                }
                Err(e) => {
                    // Log error but don't crash the Editor
                    godot_error!("Failed to set property '{}': {}", prop_name, e);
                    return false;
                }
            }
        }
        
        // Not an exported property - return false to let Godot handle it
        false
    }
}
```

**Test**: Requires Inspector interaction (manual test in Step 7.3)

---

#### Step 7.3: Manual Integration Test (15 min)

**Source**: Integration test suite (INTEGRATION_TESTS.md)

**Steps**:

1. Open Godot project
2. Open `test_scene.tscn`
3. Select `FerrisScriptRunner` node
4. **Test Reading**:
   - Verify `health` shows current runtime value (e.g., 100)
   - Verify `speed` shows current runtime value (e.g., 5.0)
5. **Test Writing**:
   - Change `health` to 50 in Inspector
   - Run scene (`F5`)
   - In script's `_ready()`, add: `print("Health:", health);`
   - Verify console output: `Health: 50`
6. **Test Clamping**:
   - Change `health` to 150 in Inspector
   - Verify it clamps to 100 (max value from `@range(0, 100, 1)`)
   - Run scene and verify: `Health: 100`
7. **Test Non-Exported Properties**:
   - Try to access built-in Node2D properties (e.g., `position`)
   - Verify they still work normally (not broken by our overrides)

**Expected**: All tests pass

---

#### Step 7.4: Commit Bundle 7

**Commit Message**:

```
Bundle 7: Property Value Hooks (Checkpoint 3.8 Part 1)

- Implement get() override: Inspector reads from Env.exported_properties
- Implement set() override: Inspector writes to Env.exported_properties with clamping
- Proper return values (Option<Variant>, bool) for Godot interop
- Non-exported properties still work (return None/false for fallback)

Testing:
- Manual Inspector integration test: Read/write/clamp verified
- All 653+ tests pass (regression check)
- Bidirectional sync functional

Files: crates/godot_bind/src/lib.rs (~60 LOC added)
Time: ~75 minutes
Source: Feedback1 + Feedback2 + BIDIRECTIONAL_SYNC_EXAMPLE
Status: Checkpoint 3.8 Part 1 COMPLETE ✅
```

---

### Bundle 8: Runtime Synchronization

#### Step 8.1: Add Inspector Notification Method (15 min)

**Source**: All feedback documents

```rust
impl FerrisScriptRunner {
    /// Notify Godot Inspector that property list has changed
    /// Call this after hot-reload or when properties are added/removed at runtime
    fn notify_inspector_refresh(&mut self) {
        // Try direct method first (godot-rust 0.5.0+)
        // self.notify_property_list_changed();
        
        // Fallback for godot-rust 0.4.0:
        // Use call() to invoke the built-in method
        self.base_mut().call(
            "notify_property_list_changed".into(),
            &[]
        );
    }
}
```

**Test**: Compilation only at this stage

---

#### Step 8.2: Integrate with Reload (15 min)

**Source**: Feedback1 (runtime sync gap)

```rust
impl FerrisScriptRunner {
    /// Reload script from source (modified to notify Inspector)
    pub fn reload_script(&mut self, source: &str) -> Result<(), String> {
        // ... existing reload logic ...
        
        // Recompile
        let program = compile(source)?;
        
        // Reinitialize environment with new metadata
        let mut env = Env::new();
        env.initialize_properties(&program.property_metadata)?;
        
        self.compiled_program = Some(program);
        self.env = Some(env);
        
        // ✅ NEW: Notify Inspector to refresh property list
        self.notify_inspector_refresh();
        
        Ok(())
    }
}
```

**Test**: Hot-reload test (Step 8.3)

---

#### Step 8.3: Hot-Reload Integration Test (15 min)

**Source**: Integration test suite

**Steps**:

1. Open Godot project and run `test_scene.tscn`
2. While game is running, edit `bounce_test.ferris`:

   ```ferrisscript
   // Add new exported property
   @export
   global new_property: i32 = 42;
   ```

3. Save file (triggers hot-reload)
4. **Verify**:
   - Inspector updates without restarting scene
   - New property `new_property` appears in Inspector
   - Existing properties still work
5. Change property type:

   ```ferrisscript
   @export
   global health: f32 = 100.0; // Changed from i32 to f32
   ```

6. **Verify**:
   - Inspector updates to show FLOAT instead of INT
   - No crashes or errors

**Expected**: Inspector stays synchronized with script changes

---

#### Step 8.4: Commit Bundle 8

**Commit Message**:

```
Bundle 8: Runtime Synchronization (Checkpoint 3.8 Part 2)

- Implement notify_inspector_refresh() with 0.4.0 fallback
- Integrate with reload_script() to auto-notify on changes
- Add hot-reload integration test

Testing:
- Hot-reload test: Add/remove/change properties → Inspector updates
- All 653+ tests pass (regression check)
- Full bidirectional sync complete

Files: crates/godot_bind/src/lib.rs (~30 LOC added)
Time: ~45 minutes
Source: Feedback1 + Feedback2
Status: Checkpoint 3.8 COMPLETE ✅
Status: Sub-Phase 3 COMPLETE ✅
```

---

## Testing Strategy

### Regression Testing (After Each Bundle)

**Command**: `cargo test`

**Expected**: All 653+ tests pass

**If Failure**: Do NOT proceed to next bundle until fixed.

---

### Unit Testing (Bundles 4, 6)

**Focus**: Helper functions in isolation

**Tests**:

- Type mapping (9 tests: 8 types + NIL)
- Hint mapping (4 tests: None, Range, Enum, File)
- PropertyInfo conversion (3 tests: basic, with range, with enum)
- Variant conversion (16 tests: round-trip, NaN/Infinity, all types)

**Coverage**: ~90% of new code

---

### Manual Integration Testing (Bundles 5, 7, 8)

**Source**: `INTEGRATION_TESTS.md` + new hot-reload test

**Tests**:

1. **Property Display** (Bundle 5):
   - Properties visible in Inspector
   - Correct types and hints
2. **Property Read/Write** (Bundle 7):
   - Inspector reads runtime values
   - Inspector edits update runtime
   - Clamping applies correctly
3. **Runtime Sync** (Bundle 8):
   - Hot-reload updates Inspector
   - Property changes reflected
   - No crashes on type changes

**Time**: ~60 minutes total across bundles

---

### Automated Integration Testing (Post-Implementation)

**Source**: `export_properties_test.ferris`, `clamp_on_set_test.ferris`

**Run After Bundle 8 Complete**:

1. Open `godot_test/` project
2. Run `property_test_helper.gd` test suite
3. Verify all assertions pass

**Expected**: 25+ automated checks pass

---

## Risk Mitigation

### Risk 1: ClassId API Compilation Failure

**Probability**: Medium (20%)  
**Impact**: High (blocks Checkpoint 3.7)  
**Source**: Feedback2

**Mitigation**:

- Bundle 4 Step 4.2 verifies API **before** implementation
- Document correct variant in commit message
- If `ClassId::none()` fails, try `ClassId::invalid()` or `ClassId::of::<()>()`

**Rollback**: Revert Bundle 4, research godot-rust version

---

### Risk 2: Inspector Crashes on NaN/Infinity

**Probability**: Low (5%) - mitigated by Bundle 6  
**Impact**: High (production crash)  
**Source**: BIDIRECTIONAL_SYNC_EXAMPLE

**Mitigation**:

- Bundle 6 Step 6.1 adds explicit NaN/Infinity handling
- Test with edge cases in Step 6.3

**Rollback**: None needed - handled proactively

---

### Risk 3: Property Hooks Not Called by Godot

**Probability**: Low (10%)  
**Impact**: High (Inspector non-functional)  
**Source**: Feedback2

**Mitigation**:

- Bundle 7 Step 7.3 verifies hooks work in actual Inspector
- If hooks not called, investigate godot-rust trait implementation

**Rollback**: Revert Bundle 7, investigate INode2D trait documentation

---

### Risk 4: Hot-Reload Breaks Inspector State

**Probability**: Medium (15%)  
**Impact**: Medium (poor UX, not crash)  
**Source**: Feedback1

**Mitigation**:

- Bundle 8 Step 8.3 tests hot-reload explicitly
- `notify_inspector_refresh()` forces Inspector update

**Rollback**: Remove notification call, document limitation

---

### Risk 5: Performance Regression

**Probability**: Low (5%)  
**Impact**: Medium (slower Inspector)  
**Source**: General concern

**Mitigation**:

- `get()` is fast (HashMap lookup)
- PropertyInfo generated on-demand (not cached)
- Notification only called on reload (not every frame)

**Monitoring**: If performance issue emerges, profile with Godot profiler

---

## Success Criteria

### Checkpoint 3.7 Success (After Bundle 5)

- ✅ Properties appear in Godot Inspector
- ✅ Correct types displayed (INT, FLOAT, STRING, etc.)
- ✅ Hints functional (range sliders, enum dropdowns, file pickers)
- ✅ All 653+ tests pass (no regression)
- ✅ API verification documented

---

### Checkpoint 3.8 Success (After Bundle 8)

- ✅ Inspector reads current property values
- ✅ Inspector edits update runtime
- ✅ Clamp-on-set applies to Inspector edits
- ✅ Runtime changes update Inspector
- ✅ Hot-reload functional without restart
- ✅ All 653+ tests pass (no regression)
- ✅ All automated integration tests pass

---

### Sub-Phase 3 Complete

- ✅ All success criteria above met
- ✅ Manual verification completed
- ✅ Documentation updated (implementation log)
- ✅ No known blockers
- ✅ Ready for Phase 6 (Advanced Features)

---

## Rollback Plan

### Per-Bundle Rollback

If any bundle fails testing:

1. **Identify failure point** (compilation, unit test, integration test)
2. **Attempt fix** (max 30 minutes)
3. **If no fix**: `git revert HEAD` (rollback bundle commit)
4. **Document issue** in implementation log
5. **Research solution** before retry

---

### Full Checkpoint Rollback

If Checkpoint 3.7 or 3.8 cannot be completed:

1. **Revert all bundles** for that checkpoint
2. **Preserve API verification results** (Bundle 4 tests)
3. **Create issue** documenting blocker
4. **Re-evaluate approach** with additional research

---

### Critical Rollback Trigger

**If any of these occur**, STOP and rollback:

- ❌ Godot Editor crashes when selecting FerrisScript node
- ❌ Existing tests fail after bundle commit
- ❌ Inspector shows corrupted property values
- ❌ Hot-reload causes scene corruption

**Do NOT** proceed until root cause identified and fixed.

---

## Time Estimates

### Bundle-by-Bundle Breakdown

| Bundle | Time | Cumulative | Critical Path |
|--------|------|------------|---------------|
| Bundle 4: API Verification & PropertyInfo | 75 min | 1.25 hrs | Yes |
| Bundle 5: Inspector Display | 45 min | 2.0 hrs | Yes |
| Bundle 6: Variant Conversion | 45 min | 2.75 hrs | Yes |
| Bundle 7: Property Hooks | 75 min | 4.0 hrs | Yes |
| Bundle 8: Runtime Sync | 45 min | 4.75 hrs | No (optional feature) |

**Total Estimated Time**: 4.75 hours (285 minutes)

**Buffer Time**: +25% = 6.0 hours total

---

### Milestone Timeline

| Milestone | Hours | Date (if started Oct 10) |
|-----------|-------|--------------------------|
| Checkpoint 3.7 Complete | 2.0 | Oct 10 (afternoon) |
| Checkpoint 3.8 Part 1 | 4.0 | Oct 10 (evening) |
| Checkpoint 3.8 Complete | 4.75 | Oct 11 (morning) |
| Sub-Phase 3 Complete | 6.0 | Oct 11 (noon) |

---

## Appendix A: API Verification Reference

**From Feedback2**: Must verify before implementation.

### ClassId Variants to Try

```rust
// Try in order:
1. ClassId::none()        // Most common in 0.4.0
2. ClassId::invalid()     // Some 0.4.0 builds
3. ClassId::of::<()>()    // Fallback
```

### PropertyUsageFlags Verification

```rust
// Verify DEFAULT doesn't auto-include EDITOR/STORAGE
let flags = PropertyUsageFlags::DEFAULT 
    | PropertyUsageFlags::EDITOR 
    | PropertyUsageFlags::STORAGE;

assert!(flags.contains(PropertyUsageFlags::EDITOR));
assert!(flags.contains(PropertyUsageFlags::STORAGE));
```

---

## Appendix B: Code Integration Points

### Files Modified

| File | Bundles | LOC Added | LOC Modified |
|------|---------|-----------|--------------|
| `crates/godot_bind/src/lib.rs` | 4, 5, 6, 7, 8 | ~350 | ~50 |
| `crates/runtime/src/lib.rs` | 6 (optional) | ~0 | ~20 |

**Total**: ~400 LOC added/modified

---

## Appendix C: Feedback Source Mapping

| Concern | Feedback Source | Bundle | Step |
|---------|----------------|--------|------|
| Missing property hooks | Feedback1, Feedback2 | 7 | 7.1-7.2 |
| API version uncertainty | Feedback2 | 4 | 4.2 |
| Inspector sync missing | Feedback1, Feedback2 | 8 | 8.1-8.2 |
| Hint string fragility | Feedback1 | 4 | 4.4 |
| NaN/Infinity crashes | BIDIRECTIONAL_SYNC_EXAMPLE | 6 | 6.1 |
| Type conversion ambiguity | BIDIRECTIONAL_SYNC_EXAMPLE | 6 | 6.2 |
| Transform2D extraction | BIDIRECTIONAL_SYNC_EXAMPLE | 6 | 6.2 |
| FFI boundary complexity | BIDIRECTIONAL_SYNC_EXAMPLE | 6, 7 | All |

---

## Appendix D: Next Steps After Completion

### Immediate (Day 1)

1. ✅ Update `SUB_PHASE_3_IMPLEMENTATION_LOG.md` with final status
2. ✅ Run full integration test suite (`INTEGRATION_TESTS.md`)
3. ✅ Take screenshots of Inspector for documentation
4. ✅ Update `PHASE_TRACKING.md` (Sub-Phase 3 → 100%)

### Near-Term (Week 1)

1. 📝 Create `PHASE_5_COMPLETION_REPORT.md`
2. 📝 Document lessons learned
3. 📝 Update `v0.0.2-CHECKLIST.md`
4. 🎯 Begin Phase 6 planning (if in scope)

### Long-Term (Post-v0.0.2)

1. 🔄 Plan godot-rust 0.5.0 migration
2. 🔄 Add property groups/subgroups support
3. 🔄 Implement property delta storage (persistence)
4. 🔄 Add signature hash for type change detection

---

**Document Version**: 1.0  
**Author**: GitHub Copilot (AI Assistant)  
**Review Sources**: PROPERTYINFO_RESEARCH v1.1, PROPERTYINFO_RESEARCH_FEEDBACK2, BIDIRECTIONAL_SYNC_EXAMPLE  
**Status**: READY FOR EXECUTION  
**Estimated Completion**: 6.0 hours (with buffer)
