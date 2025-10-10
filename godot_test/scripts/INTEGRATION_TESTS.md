# FerrisScript Exported Properties - Integration Tests

**Phase 5 Sub-Phase 3** - Runtime & Godot Integration Tests  
**Last Updated**: October 10, 2025

---

## Overview

This directory contains comprehensive integration tests for FerrisScript's exported property system, covering all 8 exportable types, 4 hint types, and the clamp-on-set policy.

---

## Test Files

### 1. `export_properties_test.ferris`

**Purpose**: Test all exportable types and hint types

**Coverage**:

- ✅ All 8 exportable types: i32, f32, bool, String, Vector2, Color, Rect2, Transform2D
- ✅ All 4 hint types: None, Range, Enum, File
- ✅ Multiple properties with different configurations
- ✅ Runtime property access and modification
- ✅ Property interaction functions

**Test Cases**:

| Test # | Description | Properties Tested |
|--------|-------------|-------------------|
| 1 | Basic exported properties (no hints) | basic_int, basic_float, basic_bool, basic_string |
| 2 | Range hints (int and float) | health, speed, temperature |
| 3 | Enum hints | size, color_name |
| 4 | File hints | texture_path, resource_path |
| 5 | Godot struct types | position, color, bounds, transform |
| 6 | Multiple properties with different hints | rotation_degrees, animation_state, opacity |
| 7 | Script logic using exported properties | All properties accessed in _ready() |
| 8 | Property interaction functions | set/get functions for health, size, position |

**How to Run**:

1. Open Godot project: `godot_test/project.godot`
2. Create a Node2D scene
3. Attach FerrisScriptNode script
4. Load `export_properties_test.ferris` as the script source
5. Run scene and check console output
6. Open Inspector to verify all properties visible with correct types/hints

**Expected Output**:

```
=== Exported Properties Test ===
Basic Int: 42
Basic Float: 3.14
Basic Bool: true
Basic String: Hello FerrisScript
Health: 100
Speed: 5.0
...
=== All Properties Accessible ===
```

---

### 2. `clamp_on_set_test.ferris`

**Purpose**: Test clamp-on-set policy (Inspector clamps, script warns)

**Coverage**:

- ✅ Range clamping behavior for i32 and f32
- ✅ Script sets allow out-of-range (with warning)
- ✅ Inspector sets would clamp automatically
- ✅ Negative range handling (-50 to 50)
- ✅ NaN/Infinity rejection (at API boundary)

**Test Cases**:

| Test # | Property | Test Value | Expected Behavior |
|--------|----------|------------|-------------------|
| 1 | health (0-100) | 150 | Script: Allow + warn, Inspector: Clamp to 100 |
| 2 | health (0-100) | -20 | Script: Allow + warn, Inspector: Clamp to 0 |
| 3 | speed (0.0-10.0) | 15.5 | Script: Allow + warn, Inspector: Clamp to 10.0 |
| 4 | speed (0.0-10.0) | -3.2 | Script: Allow + warn, Inspector: Clamp to 0.0 |
| 5 | altitude (-50 to 50) | -25 | Script: Allow (valid), Inspector: Allow |
| 6 | altitude (-50 to 50) | 75 | Script: Allow + warn, Inspector: Clamp to 50 |

**How to Run**:

1. Open Godot project: `godot_test/project.godot`
2. Create a Node2D scene
3. Attach FerrisScriptNode script
4. Load `clamp_on_set_test.ferris` as the script source
5. Run scene - automated tests execute in _ready()
6. **Manual Inspector Test**:
   - Select FerrisScriptNode in scene tree
   - In Inspector, set `player_health` to 150
   - Verify it clamps to 100
   - Set `player_health` to -20
   - Verify it clamps to 0

**Expected Console Output**:

```
=== Inspector Clamp-on-Set Test ===
Initial values:
  Health: 75
  Speed: 5.0
  Altitude: 0

--- Testing Script Sets (Should Warn, Not Clamp) ---
Test 1: Setting health to 150 (above max 100)
Warning: Property 'player_health' set to 150, outside range 0-100
  Result: 150
Test 2: Setting health to -20 (below min 0)
Warning: Property 'player_health' set to -20, outside range 0-100
  Result: -20
...
```

---

### 3. `property_test_helper.gd`

**Purpose**: GDScript helper to verify PropertyInfo from Godot's perspective

**Coverage**:

- ✅ get_property_list() returns correct PropertyInfo structures
- ✅ PropertyInfo fields match expected types and hints
- ✅ Property get/set works correctly
- ✅ Inspector clamping verification (manual test)
- ✅ All 8 types verified
- ✅ All 4 hints verified

**Test Functions**:

| Function | Purpose | Automated |
|----------|---------|-----------|
| `test_property_list()` | Verify PropertyInfo structure | ✅ Yes |
| `test_property_get_set()` | Test property access | ✅ Yes |
| `test_inspector_clamping()` | Verify clamp behavior | ⚠️ Manual |
| `test_all_types()` | Verify 8 exportable types | ✅ Yes |
| `test_all_hints()` | Verify 4 hint types | ✅ Yes |
| `run_all_tests()` | Run all tests | ✅ Yes |
| `print_all_properties()` | Debug utility | ℹ️ Utility |

**How to Run**:

1. Open Godot project
2. Create a Node2D scene
3. Add two children:
   - FerrisScriptNode (with `export_properties_test.ferris`)
   - Node2D (with `property_test_helper.gd`)
4. Run scene
5. Tests execute automatically in _ready()

**Expected Output**:

```
=== GDScript PropertyInfo Integration Test ===

--- Test 1: Property List Verification ---
Total properties: 47
FerrisScript exported properties found: 18

Property: basic_int
  Type: 2 (INT)
  Hint: 0 (NONE)
  Hint String: 
  Usage: 7
  ✓ Verified

Property: health
  Type: 2 (INT)
  Hint: 1 (RANGE)
  Hint String: 0,100,1
  Usage: 7
  ✓ Verified

...

--- Test 2: Property Get/Set Verification ---
Set basic_int to 100, got: 100
Set position to (10, 20), got: (10, 20)
✓ Get/Set working correctly

...

=== GDScript Tests Complete ===
```

---

## Test Execution Workflow

### Quick Test (5 minutes)

1. Open `godot_test/project.godot` in Godot Editor
2. Create new scene: Node2D root
3. Add FerrisScriptNode child
4. Set script to `export_properties_test.ferris`
5. Press F5 to run - verify console output
6. Stop scene, select FerrisScriptNode
7. Verify Inspector shows all properties with hints

### Full Test Suite (15 minutes)

1. **Basic Export Test** (5 min):
   - Run `export_properties_test.ferris`
   - Verify all property types in Inspector
   - Test range sliders, enum dropdowns, file pickers
   - Modify properties in Inspector, save scene, reload
   - Verify persistence

2. **Clamp Policy Test** (5 min):
   - Run `clamp_on_set_test.ferris`
   - Check console for warning messages
   - Manually test Inspector clamping
   - Verify script sets don't clamp (just warn)

3. **PropertyInfo Integration Test** (5 min):
   - Run scene with both FerrisScript and GDScript nodes
   - Verify all automated tests pass
   - Check detailed PropertyInfo output
   - Run `print_all_properties()` from debugger

### Regression Test Checklist

After making changes to exported property system:

- [ ] All 8 types visible in Inspector
- [ ] All 4 hint types display correctly (slider, dropdown, file picker, none)
- [ ] Range hints show min/max/step in Inspector
- [ ] Enum hints show dropdown with correct values
- [ ] File hints show file picker with correct extensions
- [ ] Properties can be get/set from GDScript
- [ ] Inspector sets clamp to range (i32/f32 with range hint)
- [ ] Script sets warn but don't clamp (out-of-range values)
- [ ] NaN/Infinity rejected for float range properties
- [ ] Negative ranges work correctly (-100 to 100)
- [ ] Property values persist in saved scenes
- [ ] Properties accessible from _ready() and _process()
- [ ] PropertyInfo structure matches Godot conventions
- [ ] Console shows no errors or warnings (except clamp warnings)

---

## Known Limitations

### Current Scope (Phase 5 Sub-Phase 3)

**Implemented**:

- ✅ All 8 exportable types
- ✅ All 4 hint types
- ✅ Per-instance property storage
- ✅ Get/set with clamp-on-set policy
- ✅ Variant conversion (Value ↔ Variant)
- ✅ PropertyInfo generation (if Checkpoint 3.7 complete)

**Not Yet Implemented** (Future):

- ⏸️ Property groups/subgroups in Inspector
- ⏸️ Custom property icons
- ⏸️ Property tooltips/documentation strings
- ⏸️ Property revert to default button
- ⏸️ Property animation (AnimationPlayer integration)
- ⏸️ Export flags (STORAGE, NO_EDITOR, etc.)
- ⏸️ Resource and Node type exports (e.g., `Texture2D`, `PackedScene`)

### Test Environment Constraints

**Godot Version**: Tests designed for Godot 4.x with godot-rust 0.4.0

**Limitations**:

1. **Unit Tests**: Cannot test Godot types (Variant, Vector2, etc.) in unit test environment
   - Solution: Integration tests in godot_test/ with actual Godot runtime
2. **Inspector Tests**: Require manual interaction (automated UI testing not available)
   - Solution: GDScript helper verifies PropertyInfo, manual Inspector verification
3. **Clamping**: Inspector clamping is automatic, hard to test programmatically
   - Solution: Test script sets (warn-only) automatically, Inspector sets manually

---

## Test Results Log

### Last Run: [DATE]

**Environment**:

- Godot Version: 4.x
- FerrisScript Version: 0.0.4 (Phase 5 Sub-Phase 3)
- godot-rust Version: 0.4.0

**Results**:

| Test Suite | Status | Tests Passed | Tests Failed | Duration |
|------------|--------|--------------|--------------|----------|
| export_properties_test.ferris | ⏳ Pending | - | - | - |
| clamp_on_set_test.ferris | ⏳ Pending | - | - | - |
| property_test_helper.gd | ⏳ Pending | - | - | - |

**Notes**:

- _Update this section after running tests_

---

## Troubleshooting

### Issue: Properties not visible in Inspector

**Symptoms**: FerrisScript exports defined, but Inspector shows no custom properties

**Possible Causes**:

1. `get_property_list()` not overridden in FerrisScriptNode
2. PropertyInfo not generated correctly
3. Script compilation failed
4. Node not attached to scene

**Solutions**:

1. Verify Checkpoint 3.7 implemented (get_property_list override)
2. Check console for compilation errors
3. Verify script loads without errors in _ready()
4. Re-attach script to node in Inspector

---

### Issue: Range hints not showing slider

**Symptoms**: Range properties show text input instead of slider

**Possible Causes**:

1. Hint string format incorrect
2. PropertyHint not set to RANGE
3. Type mismatch (String instead of i32/f32)

**Solutions**:

1. Verify hint_string format: "min,max,step"
2. Check PropertyInfo.hint_info.hint == PropertyHint::RANGE
3. Ensure variant_type matches property type (INT or FLOAT)

---

### Issue: Inspector clamps when it shouldn't

**Symptoms**: Script sets are clamped (expected: warn only)

**Possible Causes**:

1. from_inspector flag not working correctly
2. Clamp logic applied to all sets
3. Runtime set_exported_property called with wrong flag

**Solutions**:

1. Check runtime set_exported_property implementation
2. Verify from_inspector flag passed correctly
3. Review clamp_if_range() logic (should only trigger if from_inspector=true)

---

### Issue: Console warnings not appearing

**Symptoms**: Out-of-range script sets succeed but no warning printed

**Possible Causes**:

1. warn_if_out_of_range() not called
2. Console output suppressed
3. Range check logic incorrect

**Solutions**:

1. Verify set_exported_property calls warn_if_out_of_range when from_inspector=false
2. Check Godot console visibility (View → Output)
3. Add debug print statements to warn_if_out_of_range()

---

## Contributing

When adding new exported property features:

1. **Add Test Case**: Create new .ferris test file or extend existing
2. **Update Test Matrix**: Add to test coverage matrix in this README
3. **Document Behavior**: Explain expected behavior in comments
4. **Run All Tests**: Verify no regressions in existing tests
5. **Update Checklist**: Add to regression test checklist

---

## References

### Related Documentation

- `docs/planning/v0.0.4/SUB_PHASE_3_IMPLEMENTATION_LOG.md` - Implementation details
- `docs/planning/v0.0.4/PROPERTYINFO_RESEARCH.md` - PropertyInfo API research
- `docs/planning/v0.0.4/PHASE_4_COMPLETION_REPORT.md` - PropertyMetadata design

### Godot Documentation

- [PropertyInfo](https://docs.godotengine.org/en/stable/classes/class_%40globalscope.html#class-globalscope-propertyinfo)
- [PropertyHint](https://docs.godotengine.org/en/stable/classes/class_%40globalscope.html#enum-globalscope-propertyhint)
- [PropertyUsageFlags](https://docs.godotengine.org/en/stable/classes/class_%40globalscope.html#enum-globalscope-propertyusageflags)
- [get_property_list()](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-method-get-property-list)

### godot-rust Examples

- `godot-rust/gdext/itest/rust/src/object_tests/get_property_list_test.rs`
- `godot-rust/gdext/itest/rust/src/object_tests/property_test.rs`

---

**Test Suite Version**: 1.0  
**Status**: Ready for Checkpoint 3.7 completion  
**Next Update**: After Checkpoint 3.8 (End-to-End Integration)
