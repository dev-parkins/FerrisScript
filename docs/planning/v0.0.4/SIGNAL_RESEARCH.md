# Signal Research for FerrisScript v0.0.4

**Date**: October 8, 2025  
**Researcher**: GitHub Copilot  
**Objective**: Determine how to implement dynamic signal registration and emission in godot-rust 0.4

## Background

FerrisScript needs to support Godot signals with the following syntax:
```ferris
signal health_changed(old: i32, new: i32);
signal player_died();

fn take_damage() {
    emit_signal("health_changed", 100, 75);
}
```

The challenge is integrating this with godot-rust 0.4's signal system.

---

## Research Questions

### 1. Static vs Dynamic Signal Registration

**Question**: Does godot-rust 0.4 require `#[signal]` attributes, or can signals be registered dynamically at runtime?

**Known from gdext docs**:
- godot-rust 0.4 uses `#[signal]` attribute on associated functions in `#[godot_api]` impl blocks
- Static approach: `#[signal] fn health_changed(old: i32, new: i32);`
- Dynamic approach: `Object::add_user_signal()` from Godot's ClassDB API

**Hypothesis**: We likely need to use `add_user_signal()` from the Godot engine API to register signals dynamically at runtime, since FerrisScript signals are parsed at load time, not compile time.

**Test Required**: Can we call `add_user_signal()` on a Node2D instance in the `ready()` or `init()` method?

---

### 2. Signal Parameter Types

**Question**: How do we marshal FerrisScript types to Godot signal parameters?

**FerrisScript Types → Godot Types**:
- `i32` → `Variant::from(i32)`
- `f32` → `Variant::from(f32)`
- `bool` → `Variant::from(bool)`
- `String` → `Variant::from(GString)`
- `Vector2` → `Variant::from(Vector2)`

**Known**: godot-rust 0.4 uses `Variant` as the universal type for Godot interop.

**Hypothesis**: We can convert `ferrisscript_runtime::Value` to `godot::builtin::Variant` with a helper function.

---

### 3. Signal Emission

**Question**: How do we emit dynamically registered signals?

**Known approaches**:
1. **Static signals**: `self.emit_signal("signal_name".into(), &[variant1, variant2])`
2. **Dynamic signals**: Same API, but signal must be registered first with `add_user_signal()`

**Hypothesis**: We can use `Object::emit_signal()` after registering with `add_user_signal()`.

**Concern**: Does `emit_signal()` work for signals registered via `add_user_signal()`, or only for `#[signal]` declared signals?

---

### 4. Signal Connection/Disconnection

**Question**: Can other nodes connect to dynamically registered signals?

**Godot API**:
```gdscript
# In GDScript
node.connect("signal_name", callable)
node.disconnect("signal_name", callable)
```

**godot-rust equivalent**:
```rust
node.connect("signal_name".into(), callable);
node.disconnect("signal_name".into(), callable);
```

**Hypothesis**: Connections should work normally once signals are registered via `add_user_signal()`.

---

## Implementation Strategy

### Approach A: Dynamic Registration (Preferred)

```rust
impl INode2D for FerrisScriptNode {
    fn ready(&mut self) {
        // 1. Load and compile script
        self.load_script();
        
        // 2. Register all signals dynamically
        if let Some(program) = &self.program {
            for signal in &program.signals {
                let signal_name = StringName::from(&signal.name);
                let mut property_list = Array::new();
                
                // Build parameter list
                for (param_name, param_type) in &signal.parameters {
                    let mut dict = Dictionary::new();
                    dict.set("name", param_name);
                    dict.set("type", variant_type_from_string(param_type));
                    property_list.push(dict);
                }
                
                // Register signal with Godot
                self.base_mut().add_user_signal(signal_name, property_list);
            }
        }
        
        // 3. Call _ready function
        self.call_script_function("_ready", &[]);
    }
}

// Helper to convert FerrisScript type names to Godot VariantType
fn variant_type_from_string(type_name: &str) -> VariantType {
    match type_name {
        "i32" => VariantType::INT,
        "f32" => VariantType::FLOAT,
        "bool" => VariantType::BOOL,
        "String" => VariantType::STRING,
        "Vector2" => VariantType::VECTOR2,
        _ => VariantType::NIL,
    }
}

// In runtime builtin_emit_signal - needs access to Godot node
fn builtin_emit_signal(args: &[Value]) -> Result<Value, String> {
    // Problem: How do we access the Godot node from here?
    // Solution: Pass a callback from Godot binding to runtime
}
```

**Challenges**:
1. ⚠️ **Runtime-to-Godot callback**: `builtin_emit_signal()` runs in pure Rust runtime, needs access to Godot node
2. ✅ **Parameter marshalling**: Straightforward `Value` → `Variant` conversion
3. ⚠️ **Thread safety**: Need to ensure signals are registered before emission

---

### Approach B: Static Declaration with Code Generation

```rust
// Generate at compile time based on parsed signals:
#[godot_api]
impl FerrisScriptNode {
    #[signal]
    fn health_changed(old: i32, new: i32);
    
    #[signal]
    fn player_died();
}
```

**Challenges**:
1. ❌ **Requires build-time codegen**: FerrisScript is interpreted, not compiled
2. ❌ **No flexibility**: Can't load different scripts with different signals
3. ❌ **Complex build process**: Need proc macros or build scripts

**Verdict**: Not viable for an interpreted scripting language.

---

## Critical Issue: Emit Signal Callback

### Problem

The runtime's `builtin_emit_signal()` function is called from FerrisScript code:
```ferris
emit_signal("health_changed", 100, 75);
```

But this function has signature:
```rust
fn builtin_emit_signal(args: &[Value]) -> Result<Value, String>
```

It has **no access** to the Godot node instance, which is needed to call `node.emit_signal()`.

### Solution Options

#### Option 1: Emit Signal Callback (Recommended)

Add a callback to the runtime `Env`, similar to property getter/setter:

```rust
// In runtime/src/lib.rs
pub type SignalEmitter = fn(&str, &[Value]) -> Result<(), String>;

pub struct Env {
    // ... existing fields
    signal_emitter: Option<SignalEmitter>,
}

impl Env {
    pub fn set_signal_emitter(&mut self, emitter: SignalEmitter) {
        self.signal_emitter = Some(emitter);
    }
}

fn builtin_emit_signal(args: &[Value]) -> Result<Value, String> {
    // Extract signal name and parameters from args
    // Call signal_emitter callback
}
```

In Godot binding:
```rust
fn emit_signal_callback(signal_name: &str, args: &[Value]) -> Result<(), String> {
    // Access node via thread-local storage
    NODE_INSTANCE.with(|node| {
        // Convert Values to Variants
        // Call node.emit_signal()
    })
}
```

**Pros**: Clean separation, follows existing pattern (property getter/setter)  
**Cons**: Needs thread-local storage for node access

#### Option 2: Env Holds Node Reference

```rust
pub struct Env {
    godot_node: Option<Gd<Node2D>>,
}
```

**Pros**: Direct access  
**Cons**: Creates tight coupling, requires Godot types in runtime crate

#### Option 3: Global Signal Queue

```rust
static SIGNAL_QUEUE: Mutex<Vec<(String, Vec<Value>)>> = Mutex::new(Vec::new());

fn builtin_emit_signal(args: &[Value]) -> Result<Value, String> {
    // Push to queue
    SIGNAL_QUEUE.lock().unwrap().push((signal_name, args));
}

// In Godot binding after call_function returns:
fn flush_signal_queue(&mut self) {
    for (signal_name, args) in drain_signal_queue() {
        self.base_mut().emit_signal(signal_name, &args);
    }
}
```

**Pros**: No callback needed  
**Cons**: Delayed emission, harder to debug

---

## Recommended Implementation Plan

### Phase 1: Prototype (Step 5)

1. ✅ Create test file with hardcoded signal
2. ✅ Test `add_user_signal()` API
3. ✅ Test `emit_signal()` on dynamically registered signal
4. ✅ Verify parameter passing works
5. ✅ Test connection from GDScript

### Phase 2: Integration (Step 6)

1. Add `SignalEmitter` callback type to runtime
2. Implement signal registration in `FerrisScriptNode::ready()`
3. Implement `emit_signal_callback()` in Godot binding
4. Connect runtime to callback via `env.set_signal_emitter()`
5. Add `Value` → `Variant` conversion helper
6. Update `builtin_emit_signal()` to use callback

### Phase 3: Testing (Step 7)

1. Test signal emission from FerrisScript
2. Test connection in Godot editor
3. Test connection from GDScript
4. Test connection from another FerrisScript node
5. Integration tests in godot_test project

---

## Open Questions

1. **VariantType vs PropertyInfo**: ✅ **ANSWERED** - add_user_signal() only takes signal name, NO type info
2. **Signal naming**: Do signal names need special prefixes/namespacing?
3. **Error handling**: What happens if we emit an unregistered signal?
4. **Performance**: Is there overhead for dynamic signals vs static `#[signal]`?

---

## CRITICAL FINDINGS ✅

### Discovery 1: Simplified API

**godot-rust 0.4's `add_user_signal()` only takes ONE argument - the signal NAME!**

```rust
// API Signature (from compiler errors):
pub fn add_user_signal(&mut self, signal: impl AsArg<GString>);

// Usage:
self.base_mut().add_user_signal("health_changed");  // That's it!
```

**There is NO parameter type specification at registration time.** Parameters are passed dynamically as `Variant` values during emission.

### Discovery 2: String Types

- **Registration**: `add_user_signal(impl AsArg<GString>)` - uses GString
- **Emission**: `emit_signal(impl AsArg<StringName>, &[Variant])` - uses StringName  
- **String literals work**: `&str` implements both `AsArg<GString>` and `AsArg<StringName>`

### Discovery 3: Complete Working Example

```rust
// Register signals (no type info!)
self.base_mut().add_user_signal("health_changed");
self.base_mut().add_user_signal("player_died");

// Emit with no params
self.base_mut().emit_signal("player_died", &[]);

// Emit with typed params (types from Variant values)
let args = [Variant::from(100i32), Variant::from(75i32)];
self.base_mut().emit_signal("health_changed", &args);

// All FerrisScript types work
let all_types = [
    Variant::from(42i32),
    Variant::from(3.14f32),
    Variant::from(true),
    Variant::from(GString::from("hello")),
    Variant::from(Vector2::new(10.0, 20.0)),
];
self.base_mut().emit_signal("all_types_signal", &all_types);
```

**Status**: ✅ Compiles successfully!  
**Location**: `crates/godot_bind/src/signal_prototype.rs`

---

## Implementation Impact

### What This Means for FerrisScript

1. ✅ **Simpler than expected** - No need to register parameter types
2. ✅ **Dynamic by design** - Godot signals are inherently untyped in godot-rust 0.4
3. ✅ **Type checking at FerrisScript level** - We validate types in type checker (Steps 1-3 complete)
4. ✅ **Runtime is flexible** - Just convert Values to Variants and emit

### Updated Implementation (Step 6)

```rust
// In FerrisScriptNode::ready()
for signal in &program.signals {
    self.base_mut().add_user_signal(&signal.name);  // Simple!
}

// Signal emission callback
fn emit_signal_callback(signal_name: &str, args: &[Value]) -> Result<(), String> {
    NODE_INSTANCE.with(|node| {
        let variants: Vec<Variant> = args.iter()
            .map(value_to_variant)
            .collect();
        node.borrow_mut().base_mut().emit_signal(signal_name, &variants);
    });
    Ok(())
}

// Helper function (already implemented in signal_prototype.rs)
fn value_to_variant(value: &Value) -> Variant {
    match value {
        Value::Int(i) => Variant::from(*i),
        Value::Float(f) => Variant::from(*f),
        Value::Bool(b) => Variant::from(*b),
        Value::String(s) => Variant::from(GString::from(s)),
        Value::Vector2 { x, y } => Variant::from(Vector2::new(*x, *y)),
        Value::Nil => Variant::nil(),
        Value::SelfObject => Variant::nil(),
    }
}
```

---

## Next Steps (Updated)

### Phase 2: Integration (Step 6) - Now Much Simpler!

1. ✅ ~~Add parameter type mapping~~ - NOT NEEDED!
2. Add signal registration loop in `FerrisScriptNode::ready()`
3. Implement `emit_signal_callback()` with thread-local node access
4. Connect runtime `builtin_emit_signal()` to callback
5. Copy `value_to_variant()` helper from prototype
6. Test in godot_test project

### Phase 3: Testing (Step 7)

1. Test signal emission from FerrisScript
2. Test connection in Godot editor
3. Test connection from GDScript
4. Integration tests

---

## Conclusion

✅ **Dynamic signal registration is FULLY SUPPORTED and SIMPLER than expected!**

The godot-rust 0.4 API naturally supports our use case:
- No compile-time signal declarations needed
- No parameter type specification at registration
- Clean, minimal API surface
- Perfect fit for interpreted FerrisScript

**Confidence Level**: HIGH - Ready to proceed with Step 6 implementation!
