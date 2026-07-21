# Godot Bindings

**Load this skill when working in `crates/godot_bind/`**

## Architecture Overview

```
src/
└── lib.rs  # GDExtension integration, node property access, lifecycle hooks
```

The godot_bind crate uses **gdext 0.5.4** to integrate FerrisScript with Godot 4.x via the GDExtension API.

## GDExtension Setup

### Entry Point

```rust
use godot::prelude::*;

struct FerrisScriptExtension;

#[gdextension]
unsafe impl ExtensionLibrary for FerrisScriptExtension {}
```

**Key points:**

- `#[gdextension]` macro generates the C entry point (`gdext_rust_init`)
- `ExtensionLibrary` trait handles initialization/shutdown
- Godot loads the compiled `.dll`/`.so`/`.dylib` at runtime

### FerrisScriptNode

The main node type that hosts FerrisScript:

```rust
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct FerrisScriptNode {
    base: Base<Node2D>,
    #[export]
    script_path: GString,
    
    runtime: Option<Runtime>,
    exported_properties: HashMap<String, PropertyExport>,
    signals: Vec<SignalDeclaration>,
}

#[godot_api]
impl INode2D for FerrisScriptNode {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            script_path: GString::new(),
            runtime: None,
            exported_properties: HashMap::new(),
            signals: Vec::new(),
        }
    }
    
    fn ready(&mut self) {
        // Load and compile the script
        let path = self.script_path.to_string();
        if path.is_empty() {
            godot_error!("No script path specified");
            return;
        }
        
        match self.load_script(&path) {
            Ok(runtime) => {
                self.runtime = Some(runtime);
                // Call _ready() if it exists
                if let Some(runtime) = &mut self.runtime {
                    let _ = runtime.call_function("_ready", &[]);
                }
            }
            Err(e) => {
                godot_error!("Failed to load script {}: {}", path, e);
            }
        }
    }
    
    fn process(&mut self, delta: f64) {
        if let Some(runtime) = &mut self.runtime {
            let _ = runtime.run_frame(delta as f32);
        }
    }
}
```

## Property Export System

### Declaring Exported Properties

```rust
// In FerrisScript:
@export let speed: f32 = 100.0;
@export(range, 0.0, 10.0) let health: f32 = 5.0;
@export(enum, "Idle", "Walk", "Run") let state: String = "Idle";
```

### Property Hint Generation

```rust
fn generate_property_hint(export: &PropertyExport) -> PropertyInfo {
    match &export.hint {
        PropertyHintType::None => PropertyInfo {
            type_name: export.type_name.clone(),
            hint: None,
        },
        PropertyHintType::Range { min, max } => PropertyInfo {
            type_name: export.type_name.clone(),
            hint: Some(format!("{}:{},{}", "range", min, max)),
        },
        PropertyHintType::Enum { variants } => PropertyInfo {
            type_name: export.type_name.clone(),
            hint: Some(format!("{}:{}", "enum", variants.join(","))),
        },
        PropertyHintType::File { patterns } => PropertyInfo {
            type_name: export.type_name.clone(),
            hint: Some(format!("{}:{}", "file", patterns.join(","))),
        },
    }
}
```

### Inspector Integration

```rust
#[godot_api]
impl FerrisScriptNode {
    #[func]
    fn get_property_list(&self) -> Vec<PropertyInfo> {
        let mut props = Vec::new();
        
        // Add script_path property
        props.push(PropertyInfo {
            name: "script_path".into(),
            type_name: "String".into(),
            hint: Some("file:*.ferris".into()),
        });
        
        // Add exported properties from script
        for (name, export) in &self.exported_properties {
            props.push(PropertyInfo {
                name: name.clone().into(),
                type_name: export.type_name.clone(),
                hint: export.hint_string(),
            });
        }
        
        props
    }
    
    #[func]
    fn get(&self, property: StringName) -> Option<Variant> {
        let name = property.to_string();
        
        if name == "script_path" {
            return Some(self.script_path.to_variant());
        }
        
        // Get value from runtime
        self.runtime.as_ref()
            .and_then(|rt| rt.get_variable(&name))
            .map(|v| value_to_variant(v))
    }
    
    #[func]
    fn set(&mut self, property: StringName, value: Variant) -> bool {
        let name = property.to_string();
        
        if name == "script_path" {
            self.script_path = value.to();
            return true;
        }
        
        // Set value in runtime
        if let Some(runtime) = &mut self.runtime {
            let ferris_value = variant_to_value(&value);
            return runtime.set_variable(&name, ferris_value).is_ok();
        }
        
        false
    }
}
```

## Signal System

### Declaring Signals

```rust
// In FerrisScript:
signal health_changed(new_health: i32);
signal player_died();
```

### Signal Registration

```rust
fn register_signals(&mut self) {
    for signal in &self.signals {
        let mut args = Vec::new();
        for param in &signal.parameters {
            args.push(PropertyInfo {
                name: param.name.clone().into(),
                type_name: param.type_name.clone(),
                hint: None,
            });
        }
        
        self.base_mut().add_user_signal(signal.name.clone().into(), args);
    }
}
```

### Emitting Signals

```rust
fn emit_signal_from_script(&mut self, signal_name: &str, args: &[Value]) {
    let variant_args: Vec<Variant> = args.iter()
        .map(|v| value_to_variant(v))
        .collect();
    
    self.base_mut().emit_signal(
        signal_name.into(),
        &variant_args.iter().collect::<Vec<_>>(),
    );
}
```

## Node Query Functions

```rust
#[godot_api]
impl FerrisScriptNode {
    #[func]
    fn get_node(&self, path: GString) -> Option<Gd<Node>> {
        self.base().get_node_or_null(path)
    }
    
    #[func]
    fn has_node(&self, path: GString) -> bool {
        self.base().has_node(path)
    }
    
    #[func]
    fn find_child(&self, name: GString) -> Option<Gd<Node>> {
        self.base().find_child(name, true, false)
    }
    
    #[func]
    fn get_parent(&self) -> Option<Gd<Node>> {
        self.base().get_parent()
    }
}
```

## Type Conversions

### Value → Variant

```rust
fn value_to_variant(value: &Value) -> Variant {
    match value {
        Value::I32(v) => v.to_variant(),
        Value::F32(v) => v.to_variant(),
        Value::Bool(v) => v.to_variant(),
        Value::String(v) => v.to_variant(),
        Value::Vector2 { x, y } => Vector2::new(*x, *y).to_variant(),
        Value::Color { r, g, b, a } => Color::from_rgb(*r, *g, *b).to_variant(),
        Value::Null => Variant::nil(),
        _ => Variant::nil(),
    }
}
```

### Variant → Value

```rust
fn variant_to_value(variant: &Variant) -> Value {
    match variant.get_type() {
        VariantType::INT => Value::I32(variant.to::<i64>() as i32),
        VariantType::FLOAT => Value::F32(variant.to::<f64>() as f32),
        VariantType::BOOL => Value::Bool(variant.to::<bool>()),
        VariantType::STRING => Value::String(variant.to::<GString>().to_string()),
        VariantType::VECTOR2 => {
            let v = variant.to::<Vector2>();
            Value::Vector2 { x: v.x, y: v.y }
        }
        VariantType::COLOR => {
            let c = variant.to::<Color>();
            Value::Color { r: c.r, g: c.g, b: c.b, a: c.a }
        }
        VariantType::NIL => Value::Null,
        _ => Value::Null,
    }
}
```

## Lifecycle Hooks

FerrisScript supports these Godot lifecycle callbacks:

```rust
fn _ready()                    // Called when node enters scene tree
fn _process(delta: f32)        // Called every frame
fn _physics_process(delta: f32) // Called every physics frame
fn _input(event: InputEvent)   // Called on input events
fn _enter_tree()               // Called when node is added to scene
fn _exit_tree()                // Called when node is removed from scene
```

**Implementation:**

```rust
#[godot_api]
impl INode2D for FerrisScriptNode {
    fn ready(&mut self) {
        self.load_script_and_call("_ready", &[]);
    }
    
    fn process(&mut self, delta: f64) {
        self.call_script_function("_process", &[Value::F32(delta as f32)]);
    }
    
    fn physics_process(&mut self, delta: f64) {
        self.call_script_function("_physics_process", &[Value::F32(delta as f32)]);
    }
    
    fn input(&mut self, event: Gd<InputEvent>) {
        let event_value = self.convert_input_event(&event);
        self.call_script_function("_input", &[event_value]);
    }
    
    fn enter_tree(&mut self) {
        self.call_script_function("_enter_tree", &[]);
    }
    
    fn exit_tree(&mut self) {
        self.call_script_function("_exit_tree", &[]);
    }
}
```

## Testing Godot Integration

### Unit Tests (Rust)

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
    
    #[test]
    fn test_vector2_conversion() {
        let value = Value::Vector2 { x: 10.0, y: 20.0 };
        let variant = value_to_variant(&value);
        let back = variant_to_value(&variant);
        assert_eq!(back, Value::Vector2 { x: 10.0, y: 20.0 });
    }
}
```

### Integration Tests (Godot)

```bash
# Run integration tests
ferris-test --all

# Run specific test
ferris-test --script godot_test/scripts/signal_test.ferris

# Verbose output
ferris-test --all --verbose
```

**Test script format:**

```ferris
// TEST: signal_test
// CATEGORY: integration
// EXPECT: success
// ASSERT: Signal emitted

signal test_signal(value: i32);

fn _ready() {
    emit_signal("test_signal", 42);
    print("Signal emitted");
}
```

## Common Tasks

### Adding a new Godot type

1. Add type to `Value` enum in runtime
2. Add conversion functions in godot_bind
3. Add type literal parsing in compiler
4. Add type checking rules
5. Add integration test

### Adding a new property hint

1. Add variant to `PropertyHintType` enum
2. Add parsing in compiler (`@export(hint, args)`)
3. Add hint string generation in godot_bind
4. Add validation in type checker
5. Add test

### Debugging GDExtension issues

```rust
// Enable verbose logging
godot_print!("Debug: {}", message);
godot_warn!("Warning: {}", message);
godot_error!("Error: {}", message);

// Check Godot console for output
```

**Common issues:**

- Extension not loading: Check `.gdextension` file paths
- Properties not showing: Verify `get_property_list()` returns correct data
- Signals not firing: Check signal registration in `ready()`

## Version Compatibility

**Current setup:**

- gdext: 0.5.4
- Godot API: 4.7 (via `api-4-7` feature)
- Minimum compatible: Godot 4.2

**Checking compatibility:**

```toml
# crates/godot_bind/Cargo.toml
[dependencies]
godot = { version = "0.5.4", features = ["api-4-7"] }
```

**If you get initialization errors:**

- Verify Godot version matches `api-4-X` feature
- Check `compatibility_minimum` in `.gdextension` file
- Rebuild with `cargo build --package ferrisscript_godot_bind`
