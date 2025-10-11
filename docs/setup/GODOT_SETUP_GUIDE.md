# FerrisScript Godot Setup Guide

**Last Updated**: October 8, 2025  
**FerrisScript Version**: v0.0.4-dev  
**Godot Compatibility**: 4.2+ (tested with 4.3+)

---

## 🎯 Quick Setup

### Prerequisites

- **Rust 1.70+** ([Install Rust](https://www.rust-lang.org/tools/install))
- **Godot 4.2+** ([Download Godot](https://godotengine.org/download))
  - **For Godot 4.3+**: Requires `godot = { version = "0.4", features = ["api-4-3"] }`
- **Git** (for cloning)

---

## 📦 Installation Steps

### 1. Clone Repository

```powershell
git clone https://github.com/dev-parkins/FerrisScript.git
cd FerrisScript
```

---

### 2. Build the GDExtension

```powershell
# For Godot 4.3+, ensure Cargo.toml has api-4-3 feature enabled
cargo build --package ferrisscript_godot_bind
```

**Expected Output**:

- `target/debug/ferrisscript_godot_bind.dll` (Windows)
- `target/debug/libferrisscript_godot_bind.so` (Linux)
- `target/debug/libferrisscript_godot_bind.dylib` (macOS)

---

### 3. Open Test Project in Godot

1. Launch **Godot 4.2+**
2. Go to **Project Manager** → **Import**
3. Select: `godot_test/project.godot`
4. Click **"Import & Edit"**

---

### 4. Verify GDExtension Loaded

- Open Godot's **Output** panel (bottom of editor)
- Look for:

  ```
  GDExtension loaded: res://ferrisscript.gdextension
  ```

- If you see errors like `classdb_register_extension_class5`, rebuild with `api-4-3` feature

---

### 5. Create Your First Script

Create `godot_test/scripts/my_script.ferris`:

```rust
fn _ready() {
    print("Hello from FerrisScript!");
}

fn _process(delta: f32) {
    self.position.x += 50.0 * delta;
}
```

---

### 6. Attach Script to Node

1. Add a **Node2D** to your scene
2. In the **Inspector**, look for the **Script** property
3. Click the script icon → **Load**
4. Select `res://scripts/my_script.ferris`
5. Run the scene (F5)

---

## 🐛 Troubleshooting

### Error: "classdb_register_extension_class5" not found

**Cause**: Godot 4.3+ requires `api-4-3` feature flag

**Fix**: Update `crates/godot_bind/Cargo.toml`:

```toml
[dependencies]
godot = { version = "0.4", features = ["api-4-3"] }
```

Then rebuild:

```powershell
cargo clean -p ferrisscript_godot_bind
cargo build --package ferrisscript_godot_bind
```

---

### Error: "GDExtension initialization failed"

**Check**:

1. DLL exists in `target/debug/`
2. `godot_test/ferrisscript.gdextension` points to correct path
3. Godot version matches gdext API version

---

### DLL Not Found

**Verify paths in** `godot_test/ferrisscript.gdextension`:

```ini
[libraries]
windows.debug.x86_64 = "res://../target/debug/ferrisscript_godot_bind.dll"
```

The `res://../target/` path is relative to `godot_test/` folder.

---

## 🔧 Advanced Configuration

### Building for Release

```powershell
cargo build --package ferrisscript_godot_bind --release
```

Update Godot to use release build in `ferrisscript.gdextension`:

```ini
windows.release.x86_64 = "res://../target/release/ferrisscript_godot_bind.dll"
```

---

### Godot Version Compatibility

| Godot Version | gdext Feature Flag | Cargo.toml |
|--------------|-------------------|-----------|
| 4.2.x        | (default)         | `godot = "0.4"` |
| 4.3.x        | `api-4-3`         | `godot = { version = "0.4", features = ["api-4-3"] }` |
| 4.4.x        | `api-4-4`         | `godot = { version = "0.4", features = ["api-4-4"] }` |

---

## 📚 Next Steps

- **Examples**: Check `examples/*.ferris` for sample code
- **Signals**: See `examples/signals.ferris` for event-driven programming
- **Type System**: Review `docs/ARCHITECTURE.md` for type reference
- **Error Codes**: Check `docs/ERROR_CODES.md` for debugging

---

## 🆘 Getting Help

- **Issues**: https://github.com/dev-parkins/FerrisScript/issues
- **Docs**: `docs/` folder in repository
- **Architecture**: `docs/ARCHITECTURE.md`

---

**Status**: ✅ Godot 4.3+ compatibility confirmed (October 8, 2025)
