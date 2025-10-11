# Bundle 7 Implementation Plan - Property Hooks

**Date**: 2025-10-10  
**Status**: READY TO IMPLEMENT (Blocker Resolved)  
**Research Sources**: Claude 4.5 + GPT-5 API verification  
**Estimated Time**: 90 minutes (revised from 75 min)

---

## Executive Summary

Bundle 7 blocker **RESOLVED**. Both research sources confirm the correct API:

- **Methods**: `get_property()` and `set_property()` (NOT `get()` and `set()`)
- **Trait**: Methods available via `IObject` trait (inherited by `INode2D`)
- **Critical Addition**: Requires `#[class(tool)]` annotation for Inspector integration
- **Return Semantics**: `None`/`false` = fallback to Godot, `Some(value)`/`true` = we handled it

---

## Research Synthesis

### ✅ Consensus Points (Both Sources Agree)

| Aspect | Agreement |
|--------|-----------|
| **Method names** | `get_property()`, `set_property()` |
| **Signatures** | `get_property(&self, property: StringName) -> Option<Variant>` <br> `set_property(&mut self, property: StringName, value: Variant) -> bool` |
| **Return semantics** | `None`/`false` = fallback, `Some(value)`/`true` = handled |
| **API availability** | `get_property_list` requires Godot 4.3+ |
| **Integration point** | Override in `#[godot_api] impl INode2D` block |

### 🔍 Key Differences & Resolution

| Issue | Claude 4.5 | GPT-5 | Resolution |
|-------|-----------|-------|------------|
| **Trait source** | INode2D | IObject | IObject is base trait, INode2D inherits it. Both correct. |
| **Editor support** | Not mentioned | Requires `#[class(tool)]` | **CRITICAL**: Must add annotation for Inspector |
| **Testing approach** | Direct implementation | Start with logging stubs | **ADOPTED**: Phased approach safer |
| **Additional methods** | None | `validate_property`, `property_get_revert` | Defer to future (not needed for MVP) |

### 🎯 Final API Specification

```rust
#[derive(GodotClass)]
#[class(base=Node2D, tool)]  // ⬅️ ADD 'tool' annotation
pub struct FerrisScriptNode {
    // ... existing fields
}

#[godot_api]
impl INode2D for FerrisScriptNode {
    // ✅ Already implemented (Bundle 5)
    fn get_property_list(&mut self) -> Vec<PropertyInfo> { ... }
    
    // ⬅️ ADD THESE (Bundle 7)
    fn get_property(&self, property: StringName) -> Option<Variant> {
        // Called when Inspector reads property value
        // Return Some(value) = we handle it, None = fallback to Godot
    }
    
    fn set_property(&mut self, property: StringName, value: Variant) -> bool {
        // Called when Inspector writes property value
        // Return true = we handle it, false = fallback to Godot
    }
}
```

---

## Implementation Strategy (Phased Approach)

### Phase 1: Verification Stub (10 min) ✅ RECOMMENDED

**Objective**: Verify hooks are actually called by Inspector

**Code**:

```rust
#[derive(GodotClass)]
#[class(base=Node2D, tool)]  // ⬅️ ADD 'tool'
pub struct FerrisScriptNode {
    base: Base<Node2D>,
    // ... existing fields
}

#[godot_api]
impl INode2D for FerrisScriptNode {
    // ... existing methods ...
    
    fn get_property(&self, property: StringName) -> Option<Variant> {
        godot_print!("🔍 get_property() called for: {}", property);
        None  // Fallback to Godot for now
    }
    
    fn set_property(&mut self, property: StringName, value: Variant) -> bool {
        godot_print!("✏️ set_property() called for: {} = {:?}", property, value);
        false  // Fallback to Godot for now
    }
}
```

**Testing**:

1. Compile: `cargo build --package ferrisscript_godot_bind`
2. Open Godot Editor with test scene
3. Attach FerrisScriptNode to a Node2D
4. Load a script with @export properties
5. Interact with Inspector (read/write property)
6. **Expected**: Console shows "🔍 get_property()" and "✏️ set_property()" messages

**Success Criteria**:

- ✅ Hooks are called for exported properties
- ✅ Hooks are called when Inspector interacts
- ✅ Built-in properties (like `position`) still work

**If Hooks NOT Called**:

- Check Godot version >= 4.3 (for `get_property_list` support)
- Verify `#[class(tool)]` annotation present
- Check if property system priority issues (see GPT-5 note)

---

### Phase 2: Runtime Integration (35 min)

**Objective**: Connect hooks to runtime storage

**Code**:

```rust
fn get_property(&self, property: StringName) -> Option<Variant> {
    let prop_name = property.to_string();
    
    // Check if it's an exported property
    if let Some(env) = &self.env {
        if let Ok(value) = env.get_exported_property(&prop_name) {
            // Convert FerrisScript Value → Godot Variant
            return Some(value_to_variant(&value));
        }
    }
    
    // Not an exported property - let Godot handle built-in properties
    None
}

fn set_property(&mut self, property: StringName, value: Variant) -> bool {
    let prop_name = property.to_string();
    
    // Check if it's an exported property
    if let Some(env) = &mut self.env {
        // Convert Godot Variant → FerrisScript Value
        let fs_value = variant_to_value(&value);
        
        // from_inspector = true enables range clamping
        match env.set_exported_property(&prop_name, fs_value, true) {
            Ok(_) => return true,  // We handled it
            Err(e) => {
                // Log error but don't crash
                godot_error!("Failed to set property '{}': {}", prop_name, e);
                return false;  // Fallback to Godot
            }
        }
    }
    
    // Not an exported property - let Godot handle it
    false
}
```

**Testing**:

1. Compile and test in Godot Editor
2. Verify property reads show correct values
3. Verify property writes update runtime storage
4. Test range clamping (e.g., health 150 → clamped to 100)
5. Test all 8 types (i32, f32, bool, String, Vector2, Color, Rect2, Transform2D)

---

### Phase 3: Documentation & Error Handling (15 min)

**Add comprehensive documentation**:

```rust
/// Override get_property() to read FerrisScript exported properties from runtime storage
///
/// Called by Godot when Inspector or code reads a property value.
/// 
/// **Flow**:
/// 1. Inspector requests property value
/// 2. Check if property exists in runtime storage (env.get_exported_property)
/// 3. If found: Convert Value → Variant and return Some(variant)
/// 4. If not found: Return None (let Godot handle built-in properties)
///
/// **Return Semantics**:
/// - `Some(variant)` = We handled it, use this value
/// - `None` = Not our property, fallback to Godot's default handling
///
/// **Supported Types**: All 8 exportable types from Phase 5 Sub-Phase 2
fn get_property(&self, property: StringName) -> Option<Variant> {
    // ... implementation
}

/// Override set_property() to write FerrisScript exported properties to runtime storage
///
/// Called by Godot when Inspector or code writes a property value.
///
/// **Flow**:
/// 1. Inspector writes new property value
/// 2. Convert Variant → Value using variant_to_value()
/// 3. Call env.set_exported_property(name, value, from_inspector=true)
/// 4. from_inspector=true enables automatic range clamping
/// 5. Return true if successful, false if error or not our property
///
/// **Return Semantics**:
/// - `true` = We handled it, property updated successfully
/// - `false` = Not our property, fallback to Godot's default handling
///
/// **Range Clamping**: When from_inspector=true, values exceeding range hints
/// are automatically clamped (e.g., health 150 → clamped to 100 for range(0,100))
fn set_property(&mut self, property: StringName, value: Variant) -> bool {
    // ... implementation
}
```

**Add optional logging** (can be disabled later):

```rust
fn get_property(&self, property: StringName) -> Option<Variant> {
    let prop_name = property.to_string();
    
    if let Some(env) = &self.env {
        if let Ok(value) = env.get_exported_property(&prop_name) {
            #[cfg(feature = "debug_inspector")]
            godot_print!("Inspector reading '{}': {:?}", prop_name, value);
            
            return Some(value_to_variant(&value));
        }
    }
    
    None
}
```

---

### Phase 4: Testing & Validation (20 min)

**Test Matrix**:

| Test Case | Expected Behavior | Verification |
|-----------|-------------------|--------------|
| **Read property** | Inspector shows current value | Check Inspector display |
| **Write property** | Runtime storage updated | Check via script print() |
| **Range clamp** | Values clamped to hint range | Health 150 → 100 |
| **Type conversion** | All 8 types work correctly | Test each type |
| **Built-in props** | position, rotation still work | Move node in Inspector |
| **Script reload** | Properties persist after reload | Reload script, check values |
| **Error handling** | Errors logged, don't crash | Set invalid type |

**Manual Testing Steps**:

1. **Create test script** (`test_properties.ferris`):

```ferris
@export(range(0, 100, 1))
let mut health: i32 = 50;

@export
let mut speed: f32 = 5.5;

@export
let mut position: Vector2 = Vector2 { x: 0.0, y: 0.0 };

fn _ready() {
    print("Health:", health);
    print("Speed:", speed);
    print("Position:", position);
}
```

2. **Attach to Node2D** in Godot Editor
3. **Test Inspector reads**: Properties show default values (50, 5.5, Vector2(0,0))
4. **Test Inspector writes**: Change health to 75 in Inspector
5. **Run scene**: Console should show "Health: 75"
6. **Test range clamping**: Set health to 150 in Inspector
7. **Verify clamping**: Inspector should clamp to 100, console shows "Health: 100"
8. **Test built-in props**: Move node using position gizmo (should still work)

---

### Phase 5: Commit & Documentation (10 min)

**Commit Message**:

```
feat(godot): Bundle 7 - Property hooks (get/set) for Inspector integration (Checkpoint 3.9 COMPLETE)

**Bundle 7: Property Hooks** (90 min, Phase 5 Sub-Phase 3)

Changes:
1. Added #[class(tool)] annotation to FerrisScriptNode for editor support
2. Implemented get_property() override for Inspector property reads
3. Implemented set_property() override for Inspector property writes
4. Bidirectional sync between Inspector and runtime storage

Implementation Details:
- get_property(): Reads from env.get_exported_property(), converts Value → Variant
- set_property(): Writes to env.set_exported_property(), converts Variant → Value
- from_inspector=true enables automatic range clamping
- Returns None/false for non-exported properties (fallback to Godot)

Testing:
- All 8 property types functional (i32, f32, bool, String, Vector2, Color, Rect2, Transform2D)
- Range clamping verified (health 150 → clamped to 100)
- Built-in properties (position, rotation) still functional
- Inspector read/write operations working correctly

Checkpoint Status:
- ✅ Checkpoint 3.7 COMPLETE (Inspector display)
- ✅ Checkpoint 3.8 COMPLETE (Variant conversion)
- ✅ Checkpoint 3.9 COMPLETE (Property hooks) ← THIS BUNDLE
- ⏸️ Checkpoint 3.10 PENDING (Runtime sync & hot-reload)

Next: Bundle 8 - Runtime synchronization (notify_property_list_changed)
```

**Update Documentation**:

- Mark Bundle 7 complete in QUICK_REFERENCE.md
- Update SESSION_SUMMARY_BUNDLES_5-6.md with Bundle 7 status
- Update TODO list to mark Bundle 7 done

---

## Critical Implementation Notes

### 🚨 Required Changes

1. **Add `#[class(tool)]` annotation** to `FerrisScriptNode`:

```rust
#[derive(GodotClass)]
#[class(base=Node2D, tool)]  // ⬅️ ADD 'tool' here
pub struct FerrisScriptNode {
    // ... fields
}
```

2. **Verify Godot version** >= 4.3 for `get_property_list` support
   - If < 4.3: Properties may not appear in Inspector
   - Workaround: Use older property registration pattern (out of scope)

3. **Handle Option/Result correctly**:
   - `env` is `Option<Env>` - always check with `if let Some(env)`
   - `get_exported_property` returns `Result` - handle with `if let Ok(value)`
   - Never panic in property hooks (would crash Inspector)

4. **Conversion functions**:
   - `value_to_variant(&value)` - already exists from Bundle 6
   - `variant_to_value(&variant)` - already exists from Bundle 6
   - Both handle NaN/Infinity and type ordering correctly

### ⚠️ Potential Pitfalls

| Pitfall | Impact | Solution |
|---------|--------|----------|
| **Forgetting `#[class(tool)]`** | Hooks not called in editor | Add annotation to class |
| **Panicking in hooks** | Inspector crashes | Use `match`/`if let`, log errors |
| **Wrong return semantics** | Properties not handled | None/false = fallback, Some/true = handled |
| **env is None** | Properties not available | Check `if let Some(env)` before access |
| **Built-in prop priority** | Our hooks never called | Return None/false for unknown properties |

### 🔧 Debugging Tips

If hooks not working:

1. **Add logging**: Verify hooks are being called
2. **Check annotation**: `#[class(tool)]` must be present
3. **Check Godot version**: >= 4.3 required for full support
4. **Test in both contexts**: Editor (Inspector) and runtime (script)
5. **Verify property names**: Exact match between @export name and property parameter

---

## Time Breakdown (Revised)

| Phase | Task | Duration | Total |
|-------|------|----------|-------|
| 1 | Verification stub + testing | 10 min | 10 min |
| 2 | Runtime integration (get/set impl) | 35 min | 45 min |
| 3 | Documentation & error handling | 15 min | 60 min |
| 4 | Testing & validation | 20 min | 80 min |
| 5 | Commit & documentation update | 10 min | 90 min |

**Total**: 90 minutes (increased from 75 min due to phased approach)

**Justification**:

- Phased approach adds 15 min but significantly reduces risk
- Verification stub catches issues early
- Comprehensive testing ensures robustness

---

## Dependencies Confirmed

**Runtime Layer** (from Bundles 1-2):

- ✅ `Env.exported_properties: HashMap<String, Value>` - EXISTS
- ✅ `env.get_exported_property(name: &str) -> Result<Value, String>` - TESTED
- ✅ `env.set_exported_property(name: &str, value: Value, from_inspector: bool) -> Result<(), String>` - TESTED
- ✅ 10 comprehensive tests covering all scenarios

**Godot Layer** (from Bundles 4-6):

- ✅ `metadata_to_property_info()` - Bundle 4
- ✅ `get_property_list()` - Bundle 5
- ✅ `variant_to_value()` with NaN/Infinity handling - Bundle 6
- ✅ `value_to_variant()` with NaN/Infinity handling - Bundle 6

**All dependencies ready** - no blockers remaining.

---

## Success Criteria

### Functional Requirements

- ✅ Inspector can READ property values from runtime storage
- ✅ Inspector can WRITE property values to runtime storage
- ✅ Range hints are enforced (automatic clamping from Inspector)
- ✅ All 8 property types work correctly
- ✅ Built-in Node2D properties (position, rotation) still functional
- ✅ Errors logged gracefully (no crashes)

### Non-Functional Requirements

- ✅ Code well-documented with comprehensive comments
- ✅ No panics or unwraps in property hooks
- ✅ Phased implementation verified at each stage
- ✅ Test coverage for all property types
- ✅ Performance acceptable (no noticeable lag in Inspector)

---

## Post-Implementation Validation

After Bundle 7 complete, verify:

1. **Checkpoint 3.9 Status**: ✅ COMPLETE
2. **Test Count**: Still 554 passing (no regressions)
3. **Inspector Functionality**: Full read/write working
4. **Documentation**: All docs updated
5. **Commit Quality**: Clean commit message with details

---

## Next: Bundle 8 (Runtime Synchronization)

**Objective**: Implement `notify_property_list_changed()` for hot-reload

**Dependencies**: Bundle 7 property hooks functional

**Estimated**: 45 minutes

**Tasks**:

1. Implement `notify_property_list_changed()` call
2. Hook into script reload flow
3. Test hot-reload in Godot Editor
4. Complete Phase 5 Sub-Phase 3
5. Final documentation and commit

---

## Conclusion

Bundle 7 blocker **definitively resolved**. Both research sources confirm the API pattern with high confidence. The phased implementation approach ensures safe, verified integration while maintaining code quality.

**Ready to implement**: All technical details confirmed, dependencies ready, implementation path clear.

**Recommendation**: Proceed with Phase 1 (verification stub) first to confirm hooks work as expected, then implement full logic.
