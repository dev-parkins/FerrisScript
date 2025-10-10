Perfect — you’re now moving into the *editor integration phase*, which is the point where FerrisScript starts becoming a **first-class Godot citizen** instead of “just another scripting backend.”

Below is a **complete plan + setup guide** for building a **Godot Editor Plugin (Extension)** that serves as the *UI layer* for FerrisScript inside the Godot Editor.

It’s tailored specifically for your case: **FerrisScript = compile-time, Rust-powered scripting language**, targeting **Godot 4.5** and using **cargo-godot 0.5.0** for runtime bindings.

---

```markdown
# 🧩 FerrisScript Editor Extension Setup Guide (Godot 4.5)

## 🎯 Goal

Create a **Godot Editor Plugin** that provides a visual UI layer for FerrisScript inside the editor.  
This includes:
- Inspector integration for `.ferris` scripts and nodes  
- Build, reload, and diagnostic UI  
- Code intelligence hooks (LSP / compiler diagnostics display)
- Signal/property metadata visualization  

---

## 🧱 1. Godot Extension Types Overview

In Godot 4.x, you can extend the editor via **Editor Plugins**.  
There are **three levels** you can use — you’ll likely combine all three:

| Level | Type | Purpose |
|--------|------|----------|
| 🧩 **EditorPlugin (GDScript/C#)** | Editor UI panels, dock tabs, menus, and interactions. | Used for main FerrisScript panel and file inspectors. |
| ⚙️ **EditorInspectorPlugin** | Custom rendering of properties in Inspector. | Used to show FerrisScript exports, signals, etc. |
| 🧠 **Native Extension (Rust via GDExtension)** | Low-level logic or binding access. | Used for communicating with FerrisScript runtime/compiler. |

The pattern:  
Rust runtime backend (compiler bridge) ↔ EditorPlugin (UI in GDScript/C#).

---

## 📁 2. Project Structure

Recommended directory layout:

```

project_root/
│
├─ addons/
│  └─ ferrisscript/
│     ├─ plugin.cfg
│     ├─ plugin.gd
│     ├─ ferris_panel.tscn
│     ├─ ferris_panel.gd
│     ├─ icons/
│     └─ styles/
│
├─ ferris_runtime/          # Rust GDExtension crate
│  ├─ src/lib.rs
│  └─ Cargo.toml
│
├─ ferris_lsp/              # LSP bridge (optional)
└─ ferris_tooling/          # CLI tools (cargo ferris, etc.)

````

---

## ⚙️ 3. Plugin Manifest (`plugin.cfg`)

Godot requires a simple configuration file:

```ini
[plugin]
name="FerrisScript Integration"
description="FerrisScript editor integration for Godot"
author="FerrisScript Team"
version="0.1.0"
script="plugin.gd"
````

---

## 🧩 4. Core Plugin Script (`plugin.gd`)

This is the main entry point for your extension.

```gdscript
extends EditorPlugin

var ferris_panel

func _enter_tree():
    # Load custom dock
    ferris_panel = preload("res://addons/ferrisscript/ferris_panel.tscn").instantiate()
    add_control_to_dock(DOCK_SLOT_RIGHT_UL, ferris_panel)
    
    # Register FerrisScript file type
    add_custom_type(
        "FerrisScript",
        "Script",
        preload("res://addons/ferrisscript/ferris_script.gd"),
        preload("res://addons/ferrisscript/icons/ferris_icon.svg")
    )

func _exit_tree():
    remove_control_from_docks(ferris_panel)
    remove_custom_type("FerrisScript")
```

---

## 🪟 5. Ferris Panel UI (`ferris_panel.tscn`)

Create a simple **Dock Panel UI** for build/run/debug controls.

**Scene structure example:**

```
PanelContainer
└─ VBoxContainer
   ├─ Label ("FerrisScript Tools")
   ├─ Button ("Build Ferris Scripts")
   ├─ Button ("Reload Runtime")
   ├─ Button ("Run Headless Tests")
   └─ RichTextLabel (log output)
```

**`ferris_panel.gd` example:**

```gdscript
extends PanelContainer

@onready var log = $VBoxContainer/RichTextLabel

func _ready():
    log.text = "[FerrisScript] Ready"
    $VBoxContainer/Button.connect("pressed", _on_build_pressed)

func _on_build_pressed():
    log.text += "\nRunning cargo ferris build..."
    var output = []
    var exit_code = OS.execute("cargo", ["ferris", "build"], output, true)
    log.text += "\n" + output.join("\n")
    if exit_code == 0:
        log.text += "\n✅ Build complete."
    else:
        log.text += "\n❌ Build failed."
```

---

## 🔌 6. Inspector Integration (Optional, but Powerful)

You can add an **Inspector Plugin** to show FerrisScript-specific metadata:

**`ferris_inspector_plugin.gd`:**

```gdscript
extends EditorInspectorPlugin

func can_handle(object):
    return object is FerrisScriptNode

func parse_property(object, type, path, hint, hint_text, usage):
    if path == "signals":
        add_custom_control(Label.new())
        get_last_added_control().text = "Signals: " + str(object.get_ferris_signals())
```

Register it inside your main plugin:

```gdscript
func _enter_tree():
    var insp = preload("res://addons/ferrisscript/ferris_inspector_plugin.gd").new()
    add_inspector_plugin(insp)
```

---

## 🧠 7. Rust Backend (Native GDExtension)

You’ll use **cargo-godot** to create your backend:

**`ferris_runtime/src/lib.rs`**

```rust
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct FerrisBridge;

#[godot_api]
impl FerrisBridge {
    #[func]
    fn build_project(&self) -> String {
        // Example: call Cargo build internally or spawn a process
        "FerrisScript build complete!".to_string()
    }

    #[func]
    fn get_signals(&self, path: String) -> Vec<String> {
        vec!["on_ready".into(), "on_update".into()]
    }
}

#[godot_api]
impl IRefCounted for FerrisBridge {}
```

Expose this in your plugin (GDScript):

```gdscript
var bridge = FerrisBridge.new()
print(bridge.build_project())
```

---

## 🧩 8. Hot Reload Support

Add a watcher (Rust-side or GDScript-side) to rebuild/reload on file changes.

**Option 1 (Rust):** Use `notify` crate in GDExtension backend.
**Option 2 (GDScript):**

```gdscript
func _process(_delta):
    for file in Directory.list_dir_begin("res://scripts"):
        if file.ends_with(".ferris") and OS.get_unix_time() - get_last_mod(file) < 3:
            OS.execute("cargo", ["ferris", "build"])
```

---

## 🧮 9. Testing & Debug Integration

Once your headless testing system (from the earlier task) is available, integrate it via button:

```gdscript
func _on_run_tests_pressed():
    log.text += "\nRunning integration tests..."
    var output = []
    OS.execute("cargo", ["ferris", "test", "--headless"], output, true)
    log.text += "\n" + output.join("\n")
```

---

## 🪞 10. Extending with LSP / Code Intelligence

Later phases can include:

- **LSP Bridge:** Connect your `ferris-lsp` server to Godot’s code editor via `EditorPlugin` socket APIs.
- **Inline Diagnostics:** Parse `cargo ferris check` output → highlight lines in script editor.

---

## 🧭 11. Development & Debug Cycle

1. Enable your plugin:

   - In Godot Editor → Project → Project Settings → Plugins → “FerrisScript Integration” → Enable.
2. Test the dock panel.
3. Run commands (`cargo ferris build`, etc.).
4. Connect it with your Rust backend for signals/inspector reflection.
5. Iterate toward deeper UI (logs, errors, type trees).

---

## ✅ Deliverables Summary

| Component                    | Purpose                          | Language   |
| ---------------------------- | -------------------------------- | ---------- |
| `plugin.cfg`                 | Registers plugin                 | INI        |
| `plugin.gd`                  | Core plugin entry                | GDScript   |
| `ferris_panel.tscn/gd`       | Main dock UI                     | GDScript   |
| `ferris_inspector_plugin.gd` | Custom inspector integration     | GDScript   |
| `ferris_runtime`             | Native backend (compiler bridge) | Rust       |
| `cargo ferris`               | CLI automation                   | Rust       |
| Headless test runner         | Runtime validation               | Rust/Godot |

---

## 💡 Next Steps

- [ ] Scaffold `addons/ferrisscript/` plugin structure
- [ ] Integrate Rust `FerrisBridge` via cargo-godot
- [ ] Add dock panel buttons for build/test/reload
- [ ] Add Inspector metadata panel for FerrisScriptNode
- [ ] Integrate headless test runner CLI
- [ ] Add runtime reload + build logs in dock
- [ ] Prepare for LSP integration in later phase (v0.9+)

---

### 🧠 Design Philosophy

> “FerrisScript’s editor integration should *feel native* — not like a plugin running beside Godot, but as if Godot itself now speaks Ferris.”

---

**Result:**
Once complete, you’ll have a full-featured, visually integrated **FerrisScript Development Hub inside Godot**, backed by Rust — handling compilation, reflection, testing, and diagnostics all from the editor.

```
