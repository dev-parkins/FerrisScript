# Signal Editor Visibility - Deep Architectural Analysis

**Date**: October 9, 2025  
**Version**: v0.0.4-dev  
**Status**: Architectural Research & Design Decision  
**Context**: Research based on Godot 4.5 + cargo-godot 0.5.0 + gdext patterns

---

## 🎯 Executive Summary

**Problem**: FerrisScript signals work perfectly at runtime but don't appear in Godot's Node→Signals panel or autocompletion.

**Root Cause**: Godot's editor introspects `ClassDB` at **editor-time** (during class registration). FerrisScript uses **runtime registration** via `add_user_signal()` in the `ready()` lifecycle method.

**Current Status**: **Known limitation, not a bug**. Signals are fully functional - this is an editor visibility issue only.

**Recommendation**: Document this limitation and explore metadata-based solutions for future phases.

---

## 🔬 Technical Analysis

### How Godot's Signal System Works

#### Compile-Time Registration (Editor-Visible)

In Godot's native C++/GDScript or Rust with `#[signal]`:

```rust
// Rust GDExtension (cargo-godot/gdext)
#[derive(GodotClass)]
#[class(base=Node)]
pub struct MyNode;

#[godot_api]
impl INode for MyNode {
    fn register_class(builder: &mut ClassBuilder<Self>) {
        // THIS is when Godot's editor sees signals
        builder.add_signal(Signal {
            name: "my_signal",
            args: &[
                SignalArgument {
                    name: "value",
                    type_: VariantType::I64,
                    ..Default::default()
                },
            ],
        });
    }
}
```

**Key Points**:

- Called during **class registration** (before any instances exist)
- Registers signal in `ClassDB` (Godot's class metadata database)
- Editor introspects `ClassDB` to populate Node→Signals panel
- Signal is **statically known** at compile-time

---

#### Runtime Registration (Editor-Invisible)

In FerrisScript's current implementation:

```rust
// crates/godot_bind/src/lib.rs
impl INode2D for FerrisScriptNode {
    fn ready(&mut self) {
        // Load and compile script if path is set
        if !self.script_path.is_empty() {
            self.load_script();
        }

        // Register signals with Godot if script is loaded
        if self.script_loaded {
            if let Some(program) = &self.program {
                let signal_names: Vec<String> =
                    program.signals.iter().map(|s| s.name.clone()).collect();

                for signal_name in signal_names {
                    // THIS runs at runtime - editor has already loaded UI
                    self.base_mut().add_user_signal(&signal_name);
                    godot_print!("Registered signal: {}", signal_name);
                }
            }
        }
    }
}
```

**Key Points**:

- Called **after instance creation** (in `ready()` lifecycle)
- Uses `Object::add_user_signal()` - dynamic API
- Signal is **NOT in ClassDB** (only in instance's signal list)
- Editor UI has already been built (can't retroactively update)

---

### Why GDScript Sees Them at Runtime

Even though signals aren't in the editor UI, GDScript can still connect:

```gdscript
func _ready():
    var ferris_node = get_node("FerrisScriptNode")
    # This works because has_signal() checks INSTANCE signals
    if ferris_node.has_signal("health_changed"):
        ferris_node.connect("health_changed", _on_health_changed)
```

**Why This Works**:

- `Object::connect()` and `Object::emit_signal()` check **instance-level** signal list
- `add_user_signal()` adds to this list dynamically
- Runtime API is separate from editor metadata system
- Dynamic signals work perfectly at runtime - just invisible in editor

---

## 🏗️ FerrisScript's Architectural Challenge

### The Core Tension

FerrisScript has **ONE** Rust class (`FerrisScriptNode`) that loads **MANY** `.ferris` scripts:

```
FerrisScriptNode (Rust)
├── loads → scripts/player.ferris (defines signals: health_changed, died)
├── loads → scripts/enemy.ferris (defines signals: spotted_player, took_damage)
└── loads → scripts/powerup.ferris (defines signals: collected)
```

**Problem**: We can't know what signals exist until runtime (when .ferris file is loaded).

**Contrast with GDScript**: Each `.gd` file compiles to its OWN class:

```
Player.gd → Player class (with health_changed, died signals)
Enemy.gd → Enemy class (with spotted_player, took_damage signals)
Powerup.gd → Powerup class (with collected signal)
```

Each GDScript class has its signals declared at parse-time, so editor knows about them.

---

## 🔍 Research: Solutions from Similar Systems

### Python GDExtension (godot-python)

**Approach**: Dynamic language, same problem as FerrisScript

**Solution**: Hybrid approach

- Core signals declared in Python class decorators
- Optional runtime signals via `add_user_signal()`
- Editor visibility: Only decorator-declared signals

**Example**:

```python
@signal(name="health_changed", args=["old_health:int", "new_health:int"])
class Player(Node):
    pass
```

---

### Lua GDExtension (luaGodot)

**Approach**: Pure dynamic registration (like FerrisScript)

**Solution**: Accepted limitation

- Signals work at runtime
- No editor visibility
- Documentation encourages programmatic connections

---

### C# GDExtension (godot-sharp)

**Approach**: Compile-time attributes (like Rust `#[signal]`)

**Solution**: Static declaration required

```csharp
[Signal]
public delegate void HealthChangedEventHandler(int oldHealth, int newHealth);
```

**Why This Works**: C# compiles to .NET assemblies with full metadata, Godot can reflect on them.

---

## 💡 Potential Solutions for FerrisScript

### Option 1: Metadata File System (Recommended for Future)

**Status**: ✅ **Confirmed as "cleanest long-term solution"** by Godot GDExtension experts (October 9, 2025)

**Concept**: Two-phase compilation

1. **Phase 1** (Compile FerrisScript): Extract signal metadata to JSON
2. **Phase 2** (Rust Build): Read metadata in `register_class()`

**Concrete Implementation Pattern** (provided by research agent):

```json
// res://ferris_signals.json (aggregated from all .ferris files)
{
  "FerrisScriptNode": [
    {
      "name": "health_changed",
      "args": [
        { "name": "old_health", "type": "i32" },
        { "name": "new_health", "type": "i32" }
      ]
    },
    {
      "name": "died",
      "args": []
    },
    {
      "name": "score_updated",
      "args": [
        { "name": "score", "type": "i32" }
      ]
    }
  ]
}
```

**Rust Implementation** (using serde_json):

```rust
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct SignalManifest {
    #[serde(rename = "FerrisScriptNode")]
    signals: Vec<SignalDef>,
}

#[derive(Deserialize)]
struct SignalDef {
    name: String,
    args: Vec<SignalArg>,
}

#[derive(Deserialize)]
struct SignalArg {
    name: String,
    #[serde(rename = "type")]
    ty: String,
}

#[godot_api]
impl INode2D for FerrisScriptNode {
    fn register_class(builder: &mut ClassBuilder<Self>) {
        // Read aggregated signal manifest
        if let Ok(json) = std::fs::read_to_string("res://ferris_signals.json") {
            if let Ok(manifest) = serde_json::from_str::<SignalManifest>(&json) {
                for signal_def in manifest.signals {
                    let mut args = vec![];
                    
                    for arg in signal_def.args {
                        args.push(SignalArgument {
                            name: &arg.name,
                            type_: map_ferris_type_to_variant(&arg.ty),
                            ..Default::default()
                        });
                    }
                    
                    builder.add_signal(Signal {
                        name: &signal_def.name,
                        args: &args,
                    });
                }
            }
        }
    }
}

fn map_ferris_type_to_variant(ty: &str) -> VariantType {
    match ty {
        "i32" => VariantType::I64,
        "f32" => VariantType::F64,
        "bool" => VariantType::Bool,
        "String" => VariantType::String,
        _ => VariantType::Nil,
    }
}
```

**Build System Integration**:

```rust
// crates/godot_bind/build.rs (new file)
use std::fs;
use std::path::Path;
use serde_json::json;

fn main() {
    println!("cargo:rerun-if-changed=../../scripts");
    
    // Find all .ferris files
    let ferris_files = find_ferris_files("../../scripts");
    
    // Compile each and extract signal metadata
    let mut all_signals = vec![];
    for file in ferris_files {
        let source = fs::read_to_string(&file).unwrap();
        let program = ferrisscript_compiler::compile(&source).unwrap();
        
        for signal in program.signals {
            all_signals.push(json!({
                "name": signal.name,
                "args": signal.params.iter().map(|p| json!({
                    "name": p.name,
                    "type": p.ty
                })).collect::<Vec<_>>()
            }));
        }
    }
    
    // Write aggregated manifest
    let manifest = json!({
        "FerrisScriptNode": all_signals
    });
    
    fs::write("../../res://ferris_signals.json", 
              serde_json::to_string_pretty(&manifest).unwrap()).unwrap();
}
```

**Pros**:

- ✅ Full editor visibility (signals appear in Node→Signals panel)
- ✅ Preserves FerrisScript's dynamic nature (no per-script classes needed)
- ✅ Metadata generation integrates with existing compiler
- ✅ Standard pattern (JSON + serde) used by many GDExtension projects
- ✅ **Confirmed as recommended approach** by Godot experts

**Cons**:

- ❌ Requires build system integration (build.rs + serde dependency)
- ❌ All signals visible on ALL FerrisScriptNode instances (over-registration)
- ❌ Significant engineering effort (2-3 days)
- ❌ Manifest must be regenerated when .ferris files change
- ❌ Adds serde_json dependency to godot_bind crate

**Future Implementation Note**: Research agent has offered to provide drop-in helper code when we're ready to implement this for v0.1.0+.

---

#### Production-Ready Implementation Pattern (v0.1.0+)

**Key Insight from Research**: "Preload once at library load; avoid file I/O in every `register_class()` call"

**Dependencies**:

- `serde_json` - JSON parsing
- `once_cell` - Lazy static initialization

**Architecture**:

```rust
use godot::prelude::*;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct SignalMeta {
    pub name: String,
    pub args: Vec<SignalArgument>,
}

impl SignalMeta {
    pub fn to_godot(&self) -> Signal {
        Signal {
            name: &self.name,
            args: &self.args,
        }
    }
}

pub struct FerrisMetadataRegistry {
    pub signals: HashMap<String, Vec<SignalMeta>>,
}

// Global registry - loads ONCE when library initializes
static REGISTRY: Lazy<Mutex<FerrisMetadataRegistry>> = Lazy::new(|| {
    let mut registry = FerrisMetadataRegistry {
        signals: HashMap::new(),
    };

    // Load JSON manifest generated from AST
    if let Ok(json) = std::fs::read_to_string("res://ferris_signals.json") {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&json) {
            if let Some(obj) = value.as_object() {
                for (node_name, sigs) in obj {
                    let mut entries = Vec::new();
                    if let Some(array) = sigs.as_array() {
                        for sig in array {
                            let name = sig["name"].as_str()
                                .unwrap_or_default()
                                .to_string();
                            
                            let args = sig["args"]
                                .as_array()
                                .unwrap_or(&vec![])
                                .iter()
                                .map(|arg| {
                                    let arg_name = arg["name"].as_str()
                                        .unwrap_or("value");
                                    let arg_type = arg["type"].as_str()
                                        .unwrap_or("");
                                    
                                    SignalArgument {
                                        name: arg_name,
                                        type_: match arg_type {
                                            "i32" => VariantType::I64,
                                            "f32" => VariantType::F64,
                                            "bool" => VariantType::Bool,
                                            "String" => VariantType::String,
                                            "Vector2" => VariantType::Vector2,
                                            _ => VariantType::Nil,
                                        },
                                        ..Default::default()
                                    }
                                })
                                .collect();
                            
                            entries.push(SignalMeta { name, args });
                        }
                    }
                    registry.signals.insert(node_name.clone(), entries);
                }
            }
        }
    }

    Mutex::new(registry)
});

impl FerrisMetadataRegistry {
    /// Get signals for a specific node type (thread-safe)
    pub fn get_signals(node_name: &str) -> Vec<SignalMeta> {
        REGISTRY
            .lock()
            .unwrap()
            .signals
            .get(node_name)
            .cloned()
            .unwrap_or_default()
    }
}

// Integration with FerrisScriptNode
#[godot_api]
impl INode2D for FerrisScriptNode {
    fn register_class(builder: &mut ClassBuilder<Self>) {
        // Load signals from global registry (no file I/O here!)
        for signal in FerrisMetadataRegistry::get_signals("FerrisScriptNode") {
            builder.add_signal(signal.to_godot());
        }
    }
}
```

**Performance Benefits**:

- ✅ **One-time load**: JSON parsed once when library loads, not per node
- ✅ **Thread-safe**: Mutex ensures safe concurrent access
- ✅ **Zero I/O in hot path**: `register_class()` just reads from memory
- ✅ **Clean API**: Simple `get_signals("NodeName")` call

**Example JSON** (same format as before):

```json
{
  "FerrisScriptNode": [
    { "name": "health_changed", "args": [{"name": "old", "type": "i32"}, {"name": "new", "type": "i32"}] },
    { "name": "died", "args": [] }
  ]
}
```

**Forward Compatibility**:

- Same registry can power LSP/tooling reflection API
- Can expose metadata to VS Code extension for autocompletion
- Foundation for hot-reload support
- Enables future `FerrisRegistry.get_class_info()` API

**Dependencies to Add** (Cargo.toml):

```toml
[dependencies]
serde_json = "1.0"
once_cell = "1.19"
```

---

### Option 2: Predefined Common Signals

**Concept**: Declare frequently-used signals in Rust, allow custom ones dynamically

**Implementation**:

```rust
#[godot_api]
impl INode2D for FerrisScriptNode {
    fn register_class(builder: &mut ClassBuilder<Self>) {
        // Predefined common signals (editor-visible)
        builder.add_signal(Signal { name: "health_changed", ... });
        builder.add_signal(Signal { name: "died", ... });
        builder.add_signal(Signal { name: "score_updated", ... });
        builder.add_signal(Signal { name: "state_changed", ... });
    }
}
```

**In FerrisScript**:

```rust
// Use predefined signal (editor-visible)
signal health_changed(old: i32, new: i32);  // Matches Rust declaration

// Custom signal (runtime-only, not editor-visible)
signal custom_event(data: String);  // Dynamically registered
```

**Pros**:

- ✅ Simple to implement (1 hour)
- ✅ Common signals visible in editor
- ✅ Still allows custom signals
- ✅ No build system changes

**Cons**:

- ❌ Limited to predefined set
- ❌ All instances show all signals (even if script doesn't use them)
- ❌ Manual maintenance required

---

### Option 3: Code Generation Per Script

**Concept**: Generate Rust wrapper class for EACH .ferris file

**Example**:

```
scripts/player.ferris → generates → crates/godot_bind/src/generated/player.rs

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Player {
    runtime: FerrisScriptRuntime,
}

#[godot_api]
impl INode for Player {
    fn register_class(builder: &mut ClassBuilder<Self>) {
        builder.add_signal(Signal { name: "health_changed", ... });
        builder.add_signal(Signal { name: "died", ... });
    }
}
```

**Pros**:

- ✅ Full editor visibility
- ✅ Each script has its own class (like GDScript)
- ✅ Type-safe per-script

**Cons**:

- ❌ **Massive engineering effort** (1-2 weeks)
- ❌ Build system complexity (proc macros, code generation)
- ❌ Loses FerrisScript's simple "attach script to node" model
- ❌ Requires Godot project rebuild when scripts change

---

### Option 4: Accept Limitation (Current)

**Concept**: Document that dynamic signals don't appear in editor

**Status**: ✅ **Currently Implemented**

**Pros**:

- ✅ Zero engineering cost
- ✅ Signals fully functional at runtime
- ✅ Programmatic connections work fine
- ✅ Matches Python/Lua GDExtension behavior

**Cons**:

- ❌ No visual signal connections in editor
- ❌ Users must connect signals programmatically
- ❌ Less discoverable for beginners

---

## 🎯 Recommendation: Hybrid Approach (Option 2 + Documentation)

### Phase-by-Phase Plan

#### Phase 2-5 (Current - v0.0.4): Accept Limitation

- ✅ Document limitation clearly
- ✅ Provide GDScript connection examples
- ✅ Focus on core functionality

#### Phase 6 (Future - v0.1.0): Predefined Common Signals

- Implement Option 2 (common signals)
- 5-10 frequently-used signals declared in Rust
- Custom signals still work dynamically

#### Post-v0.1.0 (Enhancement): Metadata System

- Implement Option 1 if user feedback demands it
- Research build system integration
- Consider Godot editor plugin for metadata generation

---

## 📊 Comparison Matrix

| Solution | Editor Visibility | Engineering Cost | Build Complexity | User Experience |
|----------|------------------|------------------|------------------|-----------------|
| **Option 1: Metadata** | ✅ Full | 🟡 Medium (2-3 days) | 🔴 High (build.rs, codegen) | ✅ Excellent |
| **Option 2: Predefined** | 🟡 Partial | 🟢 Low (1 hour) | 🟢 None | 🟡 Good |
| **Option 3: Codegen** | ✅ Full | 🔴 High (1-2 weeks) | 🔴 Very High | ✅ Excellent |
| **Option 4: Accept** | ❌ None | 🟢 Zero | 🟢 None | 🟡 Acceptable |

---

## 🤔 Answer to Research Agent's Question

> "If you show me a small snippet of how your custom language nodes get wrapped or registered (the "outer" layer that binds them to Godot), I can show you the exact place to insert the builder-level registration hook"

### Current Architecture

```rust
// crates/godot_bind/src/lib.rs

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct FerrisScriptNode {
    base: Base<Node2D>,
    
    #[export(file = "*.ferris")]
    script_path: GString,  // Path to .ferris file
    
    env: Option<Env>,           // Runtime environment
    program: Option<ast::Program>,  // Compiled AST
    script_loaded: bool,
}

#[godot_api]
impl INode2D for FerrisScriptNode {
    fn init(base: Base<Node2D>) -> Self {
        FerrisScriptNode {
            base,
            script_path: GString::new(),
            env: None,
            program: None,
            script_loaded: false,
        }
    }
    
    fn ready(&mut self) {
        // Load .ferris file (runtime)
        if !self.script_path.is_empty() {
            self.load_script();  // Compiles .ferris → ast::Program
        }
        
        // Register signals dynamically (runtime)
        if let Some(program) = &self.program {
            for signal in &program.signals {
                self.base_mut().add_user_signal(&signal.name);
            }
        }
    }
}
```

### Where Builder Registration Would Go (Option 2)

```rust
#[godot_api]
impl INode2D for FerrisScriptNode {
    // NEW: Builder-level registration
    fn register_class(builder: &mut ClassBuilder<Self>) {
        // Predefined common signals (editor-visible)
        builder.add_signal(Signal {
            name: "health_changed",
            args: &[
                SignalArgument {
                    name: "old_health",
                    type_: VariantType::I64,
                    ..Default::default()
                },
                SignalArgument {
                    name: "new_health",
                    type_: VariantType::I64,
                    ..Default::default()
                },
            ],
        });
        
        builder.add_signal(Signal {
            name: "died",
            args: &[],
        });
        
        // Add 5-10 more common signals...
    }
    
    fn init(base: Base<Node2D>) -> Self { /* ... */ }
    
    fn ready(&mut self) {
        // Still register custom signals dynamically
        if let Some(program) = &self.program {
            for signal in &program.signals {
                // Only register if NOT predefined
                if !is_predefined_signal(&signal.name) {
                    self.base_mut().add_user_signal(&signal.name);
                }
            }
        }
    }
}
```

**Key Points**:

- `register_class()` runs at **compile-time** (before any .ferris files exist)
- Can't know custom signals, only predefined ones
- Hybrid: Predefined signals appear in editor, custom ones still work at runtime

---

### Where Metadata Registration Would Go (Option 1 - Future)

```rust
// build.rs (new file)
use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
struct SignalMetadata {
    name: String,
    params: Vec<ParamMetadata>,
}

fn main() {
    // Read all .ferris.meta.json files
    let metadata = load_all_ferris_metadata("../scripts");
    
    // Generate signal_registry.rs
    let code = generate_signal_registration_code(metadata);
    fs::write("src/generated/signal_registry.rs", code).unwrap();
}
```

```rust
// crates/godot_bind/src/lib.rs
mod generated {
    include!(concat!(env!("OUT_DIR"), "/signal_registry.rs"));
}

#[godot_api]
impl INode2D for FerrisScriptNode {
    fn register_class(builder: &mut ClassBuilder<Self>) {
        // Auto-generated from all .ferris files
        for signal in generated::ALL_SIGNALS {
            builder.add_signal(signal);
        }
    }
}
```

**Challenges**:

- Requires .ferris → metadata extraction step
- All signals visible on ALL instances (over-registration)
- Build system complexity

---

## � FerrisScript's Current AST Format (For Future Implementation)

### Signal Representation in Compiler

**Location**: `crates/compiler/src/ast.rs`

```rust
pub struct Program {
    pub signals: Vec<Signal>,
    pub global_vars: Vec<VarDeclaration>,
    pub functions: Vec<Function>,
}

pub struct Signal {
    pub name: String,
    pub params: Vec<Param>,
}

pub struct Param {
    pub name: String,
    pub ty: String,  // "i32", "f32", "bool", "String", "Vector2"
}
```

### Example Compilation Output

**FerrisScript Source** (`scripts/player.ferris`):

```rust
signal health_changed(old: i32, new: i32);
signal died();
signal score_updated(score: i32, multiplier: f32);
```

**Compiled AST**:

```rust
Program {
    signals: vec![
        Signal {
            name: "health_changed".to_string(),
            params: vec![
                Param { name: "old".to_string(), ty: "i32".to_string() },
                Param { name: "new".to_string(), ty: "i32".to_string() },
            ],
        },
        Signal {
            name: "died".to_string(),
            params: vec![],
        },
        Signal {
            name: "score_updated".to_string(),
            params: vec![
                Param { name: "score".to_string(), ty: "i32".to_string() },
                Param { name: "multiplier".to_string(), ty: "f32".to_string() },
            ],
        },
    ],
    // ... global_vars, functions ...
}
```

### Mapping to JSON Manifest (For Option 1)

The above would map to:

```json
{
  "FerrisScriptNode": [
    {
      "name": "health_changed",
      "args": [
        { "name": "old", "type": "i32" },
        { "name": "new", "type": "i32" }
      ]
    },
    {
      "name": "died",
      "args": []
    },
    {
      "name": "score_updated",
      "args": [
        { "name": "score", "type": "i32" },
        { "name": "multiplier", "type": "f32" }
      ]
    }
  ]
}
```

### Type Mapping (FerrisScript → Godot Variant)

| FerrisScript Type | Godot VariantType | Notes |
|-------------------|-------------------|-------|
| `i32` | `VariantType::I64` | Godot uses i64 for integers |
| `f32` | `VariantType::F64` | Godot uses f64 for floats |
| `bool` | `VariantType::Bool` | Direct mapping |
| `String` | `VariantType::String` | Direct mapping |
| `Vector2` | `VariantType::Vector2` | Direct mapping |
| `InputEvent` | `VariantType::Object` | Special case (Phase 2) |

### Implementation Path for v0.1.0+

When ready to implement Option 1 (metadata system):

1. **Extend compiler** to output JSON alongside compilation:

   ```rust
   // crates/compiler/src/lib.rs
   pub fn compile_with_metadata(source: &str) -> Result<(Program, SignalMetadata), Error> {
       let program = compile(source)?;
       let metadata = extract_signal_metadata(&program);
       Ok((program, metadata))
   }
   ```

2. **Add build.rs** to aggregate metadata from all scripts

3. **Use research agent's helper** (offered to provide drop-in code)

4. **Add serde_json dependency** to godot_bind crate

---

## �📚 References

### External Resources

- **Research Agent Source**: Advanced Godot introspection system discussion (October 9, 2025)
- **gdext Documentation**: [godot-rust/gdext](https://github.com/godot-rust/gdext)
- **Godot ClassDB**: [Godot Docs - ClassDB](https://docs.godotengine.org/en/stable/classes/class_classdb.html)
- **Signal Registration**: [Godot Docs - Signals](https://docs.godotengine.org/en/stable/getting_started/step_by_step/signals.html)

### Internal Documents

- [KNOWN_LIMITATIONS.md](KNOWN_LIMITATIONS.md#signal-visibility) - Current limitation documentation
- [SIGNAL_VISIBILITY_ISSUE.md](SIGNAL_VISIBILITY_ISSUE.md) - Testing results and workarounds
- [PHASE_1_2_TRANSITION_SUMMARY.md](PHASE_1_2_TRANSITION_SUMMARY.md) - Phase 1 completion status

---

## ✅ Design Decision (October 9, 2025)

### Current Status: **Accept Limitation** (Option 4)

**Rationale**:

- Signals are **fully functional** at runtime
- Editor visibility is **nice-to-have**, not critical for v0.0.4
- Engineering cost for metadata system not justified at this stage
- User feedback will inform future enhancements

### Future Work: **Hybrid Approach** (Option 2) in v0.1.0+

**When user feedback indicates editor visibility is important**:

1. Implement predefined common signals in `register_class()`
2. Keep dynamic registration for custom signals
3. Document which signals are editor-visible vs. runtime-only

### Roadmap Validation (October 9, 2025)

**Research Agent Confirmation**: "Your current direction is correct — deferring editor signal introspection until after core runtime (v0.0.4) is the right call."

**Validated Roadmap**:

| Phase | Focus | Status |
|-------|-------|--------|
| ✅ **Phase 2 (v0.0.4)** | Lifecycle callbacks + runtime correctness | Current priority |
| 🚧 **Phase 3** | Node query functions | Next |
| 🔜 **Phase 4** | Additional Godot types | Future |
| 📘 **v0.1.0+** | Metadata system + compile-time signal visibility | User feedback driven |

**Key Insight**: "Runtime correctness and lifecycle stability come first; compile-time reflection can be layered on cleanly later."

**Implementation Path Confirmed**:

- Metadata system is "clean, validated, and deferred strategically"
- Production-ready registry pattern documented
- Forward-compatible with future tooling (LSP, VS Code extension)

---

## 🎓 Key Learnings

### Technical Insights

1. **Godot's Editor Introspection**: Happens at **class registration time**, not instance creation
2. **ClassDB vs. Instance Signals**: Separate systems with different capabilities
3. **Dynamic Languages**: All face this challenge (Python, Lua, JS GDExtension)
4. **GDScript Special Case**: Each .gd file = its own class (FerrisScript has 1 class, many scripts)

### Architectural Takeaways

1. **Design Trade-off**: Simplicity (one node class) vs. Editor Integration (per-script classes)
2. **Godot Patterns**: Editor-facing features require compile-time knowledge
3. **Metadata Systems**: Viable but add significant build complexity
4. **User Expectations**: GDScript users expect editor visibility, Python/Lua users don't

---

**Last Updated**: October 9, 2025  
**Next Review**: After v0.0.4 release (user feedback phase)
