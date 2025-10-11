# Bundle 7 - Quick Implementation Guide

**Status**: ✅ READY TO IMPLEMENT (Blocker Resolved)  
**Estimated Time**: 90 minutes  
**Confidence**: 100% (confirmed by Claude 4.5 + GPT-5)

---

## TL;DR - What To Do

### 1. Add `tool` Annotation (Critical!)

```rust
#[derive(GodotClass)]
#[class(base=Node2D, tool)]  // ⬅️ ADD 'tool' here
pub struct FerrisScriptNode {
    // ... existing fields
}
```

### 2. Add Property Hooks

```rust
#[godot_api]
impl INode2D for FerrisScriptNode {
    // ... existing methods ...
    
    fn get_property(&self, property: StringName) -> Option<Variant> {
        let prop_name = property.to_string();
        
        if let Some(env) = &self.env {
            if let Ok(value) = env.get_exported_property(&prop_name) {
                return Some(value_to_variant(&value));
            }
        }
        
        None  // Fallback to Godot for built-in properties
    }
    
    fn set_property(&mut self, property: StringName, value: Variant) -> bool {
        let prop_name = property.to_string();
        
        if let Some(env) = &mut self.env {
            let fs_value = variant_to_value(&value);
            
            if env.set_exported_property(&prop_name, fs_value, true).is_ok() {
                return true;  // We handled it
            }
        }
        
        false  // Fallback to Godot
    }
}
```

### 3. Test in Godot Editor

1. Compile: `cargo build --package ferrisscript_godot_bind`
2. Open Godot Editor
3. Attach FerrisScriptNode with @export properties
4. **Read**: Properties show in Inspector
5. **Write**: Change values in Inspector
6. **Verify**: Run scene, check values in console

---

## Key Points

✅ **API Confirmed**: `get_property()` and `set_property()` (NOT `get()` and `set()`)  
✅ **Annotation Required**: `#[class(tool)]` enables Inspector integration  
✅ **Return Semantics**: `None`/`false` = fallback to Godot, `Some(value)`/`true` = we handle it  
✅ **Dependencies Ready**: Runtime layer complete, variant conversion complete  
✅ **No Blockers**: All technical details confirmed

---

## Recommended Approach

**Start with verification stub** (10 min):
```rust
fn get_property(&self, property: StringName) -> Option<Variant> {
    godot_print!("🔍 get_property: {}", property);
    None  // Test hook is called
}

fn set_property(&mut self, property: StringName, value: Variant) -> bool {
    godot_print!("✏️ set_property: {} = {:?}", property, value);
    false  // Test hook is called
}
```

**Then implement full logic** (35 min) once verified.

---

## Success Criteria

- ✅ Properties readable in Inspector
- ✅ Properties writable in Inspector
- ✅ Range clamping works (health 150 → 100)
- ✅ All 8 types work (i32, f32, bool, String, Vector2, Color, Rect2, Transform2D)
- ✅ Built-in properties still work (position, rotation)

---

## Full Details

See **BUNDLE_7_IMPLEMENTATION_PLAN.md** for:
- Complete code with documentation
- Phased implementation strategy
- Testing matrix
- Debugging tips
- Time breakdown

---

**Questions?** Review BUNDLE_7_IMPLEMENTATION_PLAN.md or ask!
