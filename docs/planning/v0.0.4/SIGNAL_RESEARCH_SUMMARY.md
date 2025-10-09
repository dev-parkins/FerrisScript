# Signal Research Summary - v0.0.4

**Date**: October 8, 2025  
**Phase**: Step 5 Complete  
**Status**: ✅ RESEARCH SUCCESSFUL

---

## Executive Summary

Dynamic signal registration in godot-rust 0.4 is **fully supported** and **simpler than initially expected**. The API naturally fits FerrisScript's interpreted nature with untyped, runtime-registered signals.

---

## Key Discoveries

### 1. Simplified Registration API

**Critical Finding**: `add_user_signal()` only takes the signal NAME - no parameter types!

```rust
// Before (expected complexity):
self.base_mut().add_user_signal("health_changed", parameter_info_array);

// After (actual simplicity):
self.base_mut().add_user_signal("health_changed");  // Done!
```

**Impact**:
- ✅ No need to marshal parameter types at registration
- ✅ No complex PropertyInfo dictionaries
- ✅ Signals are inherently untyped (Variant-based)
- ✅ Simpler integration with FerrisScript

### 2. Working Prototype

**Location**: `crates/godot_bind/src/signal_prototype.rs`

**Status**: ✅ Compiles successfully

**Test Code**:
```rust
// Register
self.base_mut().add_user_signal("player_died");
self.base_mut().add_user_signal("health_changed");

// Emit (no params)
self.base_mut().emit_signal("player_died", &[]);

// Emit (with params)
let args = [Variant::from(100i32), Variant::from(75i32)];
self.base_mut().emit_signal("health_changed", &args);

// All FerrisScript types
let all_types = [
    Variant::from(42i32),           // i32
    Variant::from(3.14f32),         // f32
    Variant::from(true),            // bool
    Variant::from(GString::from("hello")),  // String
    Variant::from(Vector2::new(10.0, 20.0)), // Vector2
];
self.base_mut().emit_signal("all_types_signal", &all_types);
```

### 3. Type Conversions

**FerrisScript Value → Godot Variant** (implemented in prototype):

| FerrisScript Type | Godot Type | Conversion |
|-------------------|------------|------------|
| `Value::Int(i)` | `Variant(i32)` | `Variant::from(i)` |
| `Value::Float(f)` | `Variant(f32)` | `Variant::from(f)` |
| `Value::Bool(b)` | `Variant(bool)` | `Variant::from(b)` |
| `Value::String(s)` | `Variant(GString)` | `Variant::from(GString::from(s))` |
| `Value::Vector2{x,y}` | `Variant(Vector2)` | `Variant::from(Vector2::new(x, y))` |
| `Value::Nil` | `Variant::nil()` | `Variant::nil()` |

---

## API Documentation

### Registration

```rust
fn add_user_signal(&mut self, signal: impl AsArg<GString>)
```

- **Purpose**: Register a signal by name
- **Parameters**: Signal name only (no type information)
- **String Types**: `&str`, `String`, or `GString` all work
- **Example**: `node.add_user_signal("health_changed")`

### Emission

```rust
fn emit_signal(
    &mut self, 
    signal: impl AsArg<StringName>, 
    varargs: &[Variant]
) -> Error
```

- **Purpose**: Emit a registered signal with dynamic arguments
- **Parameters**:
  - `signal`: Signal name (StringName or &str)
  - `varargs`: Array of Variant arguments
- **Returns**: Godot Error code
- **Example**: `node.emit_signal("health_changed", &[Variant::from(100), Variant::from(75)])`

### Checking

```rust
fn has_signal(&self, signal: impl AsArg<StringName>) -> bool
```

- **Purpose**: Check if a signal is registered
- **Example**: `node.has_signal("health_changed")`

---

## Integration Architecture

### Flow Diagram

```
FerrisScript Code           Runtime                 Godot Binding
─────────────────           ───────                 ─────────────
signal health_changed(   →  TypeChecker validates   
    old: i32,                parameter types         
    new: i32);               (E301-E304 errors)     
                          
                          →  Runtime registers       
                             signal metadata         
                                                   →  FerrisScriptNode::ready()
                                                      for signal in signals {
                                                          add_user_signal(name)
                                                      }

emit_signal(             →  builtin_emit_signal()   
    "health_changed",        extracts name & args   
    100, 75);             
                          →  Calls callback with    
                             (name, &[Value])       
                                                   →  emit_signal_callback()
                                                      converts Values→Variants
                                                      node.emit_signal(name, variants)
                                                   
                                                   →  Godot engine emits signal
                                                      to connected callables
```

### Thread Safety

**Challenge**: `builtin_emit_signal()` runs in pure Rust runtime with no Godot node access.

**Solution**: Thread-local storage (same pattern as property getter/setter)

```rust
thread_local! {
    static NODE_INSTANCE: RefCell<Option<Gd<FerrisScriptNode>>> = RefCell::new(None);
}

fn emit_signal_callback(signal_name: &str, args: &[Value]) -> Result<(), String> {
    NODE_INSTANCE.with(|node| {
        let variants: Vec<Variant> = args.iter()
            .map(value_to_variant)
            .collect();
        
        if let Some(node_ref) = node.borrow_mut().as_mut() {
            node_ref.base_mut().emit_signal(signal_name, &variants);
            Ok(())
        } else {
            Err("Node not available".to_string())
        }
    })
}
```

---

## Implementation Checklist (Step 6)

### Phase 1: Runtime Callback Setup
- [ ] Add `SignalEmitter` callback type to `ferrisscript_runtime::Env`
- [ ] Implement `set_signal_emitter()` method
- [ ] Update `builtin_emit_signal()` to use callback

### Phase 2: Godot Binding Integration
- [ ] Add signal registration loop in `FerrisScriptNode::ready()`
- [ ] Implement `emit_signal_callback()` function
- [ ] Add thread-local node storage
- [ ] Copy `value_to_variant()` helper from prototype
- [ ] Connect callback in `call_script_function_with_self()`

### Phase 3: Testing
- [ ] Create test .ferris script with signals
- [ ] Add test scene in godot_test project
- [ ] Test signal emission from FerrisScript
- [ ] Test connection in Godot editor
- [ ] Test connection from GDScript
- [ ] Verify all FerrisScript types work

---

## Open Questions (Low Priority)

1. **Error Handling**: What happens if we emit an unregistered signal?
   - *Note*: Type checker prevents this at compile time (E302 error)

2. **Performance**: Is there overhead for dynamic signals vs static `#[signal]`?
   - *Note*: Unlikely to matter for scripting use case

3. **Signal Naming**: Do signal names need prefixes/namespacing?
   - *Note*: Not required, but good practice for clarity

---

## Conclusion

✅ **Research Phase Complete**

**Confidence Level**: **HIGH**

**Key Takeaway**: The godot-rust 0.4 API is a **perfect fit** for FerrisScript's interpreted signal system. No workarounds or hacks needed - the API was designed for exactly this use case.

**Next Step**: Proceed directly to Step 6 (Godot Binding Implementation)

**Estimated Complexity**: **Low** (thanks to simplified API)

---

## References

- **Prototype Code**: `crates/godot_bind/src/signal_prototype.rs`
- **Full Research**: `docs/planning/v0.0.4/SIGNAL_RESEARCH.md`
- **godot-rust Docs**: https://godot-rust.github.io/docs/gdext/master/
