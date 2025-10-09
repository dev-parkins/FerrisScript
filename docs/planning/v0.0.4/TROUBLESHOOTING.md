# Troubleshooting FerrisScriptNode in Godot

## Issue: FerrisScriptNode not appearing in Godot's node list

### ✅ Verified Working Configuration

- **DLL Location**: `godot_test/ferrisscript_godot_bind.dll` (4.26 MB, built 10/9/2025 12:43:28 PM)
- **Extension File**: `godot_test/ferrisscript.gdextension` (configured for local DLL)
- **Export Symbol**: `gdext_rust_init` ✓ (verified via dumpbin)
- **Class Name**: `FerrisScriptNode` (derives from `Node2D`)

### 🔧 Solution Steps

1. **Close Godot Completely**
   - Make sure ALL Godot instances are closed
   - Check Task Manager if needed (look for `Godot_v4.x.exe`)

2. **Clean Cache**
   - Delete the `godot_test/.godot/` folder
   - This clears all cached extension data
   - Script provided: `rebuild_godot_extension.ps1`

3. **Verify Files**
   ```powershell
   # Check DLL exists
   ls godot_test\ferrisscript_godot_bind.dll
   
   # Check extension file
   ls godot_test\ferrisscript.gdextension
   ```

4. **Restart Godot**
   - Open Godot Editor
   - Open the `godot_test` project
   - Check Output panel for extension loading messages
   - Look for "GDExtension successfully loaded" message

5. **Find FerrisScriptNode**
   - Click the `+` button in Scene panel
   - Search for "FerrisScript"
   - Should appear under Node2D hierarchy
   - Full path: `Node2D > FerrisScriptNode`

### 📋 Common Issues

#### Extension Not Loading
**Symptoms**: No messages about FerrisScript in Output panel

**Solutions**:
- Check `ferrisscript.gdextension` file is in `godot_test/` directory
- Verify DLL path in `.gdextension` matches actual DLL location
- Check for error messages in Godot's Output panel
- Try `Project > Reload Current Project`

#### DLL Version Mismatch
**Symptoms**: "Failed to load extension" or "Incompatible version"

**Solutions**:
- Rebuild with `cargo build --package ferrisscript_godot_bind --release`
- Copy DLL: `Copy-Item target\release\ferrisscript_godot_bind.dll godot_test\ -Force`
- Delete `.godot/` folder
- Restart Godot

#### Wrong Base Class
**Symptoms**: Node appears but has unexpected properties

**Note**: `FerrisScriptNode` inherits from `Node2D` (has position, rotation, scale)
- If you need a pure `Node`, modify `crates/godot_bind/src/lib.rs` line 87
- Change `#[class(base=Node2D)]` to `#[class(base=Node)]`
- Rebuild

### 🚀 Quick Rebuild Script

Run `rebuild_godot_extension.ps1`:
```powershell
.\rebuild_godot_extension.ps1
```

This will:
1. Check if Godot is running (warns you to close it)
2. Clean `.godot/` cache
3. Rebuild the DLL
4. Copy to `godot_test/`
5. Verify all files

### 📝 Extension Configuration

Current `ferrisscript.gdextension`:
```ini
[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.1
reloadable = true

[libraries]
windows.debug.x86_64 =   "res://ferrisscript_godot_bind.dll"
windows.release.x86_64 = "res://ferrisscript_godot_bind.dll"
# ... (other platforms)
```

### 🔍 Debugging Steps

1. **Check Godot Output Panel**
   - Look for "GDExtension" messages
   - Look for "FerrisScript" messages
   - Note any error messages

2. **Verify DLL**
   ```powershell
   # Check DLL exists and is recent
   Get-Item godot_test\ferrisscript_godot_bind.dll | Select Name, LastWriteTime, Length
   ```

3. **Check Extension Registration**
   ```powershell
   # After opening Godot once, check this file
   Get-Content godot_test\.godot\extension_list.cfg
   # Should contain: res://ferrisscript.gdextension
   ```

4. **Test with Simple Scene**
   - Create new scene
   - Add Node2D as root
   - Try to add FerrisScriptNode as child
   - If it appears, the extension is working!

### ✨ Using FerrisScriptNode

Once the node is available:

1. **Add to Scene**
   - Add FerrisScriptNode to your scene tree
   - In Inspector, set "Script Path" property
   - Example: `res://scripts/v004_phase2_test.ferris`

2. **Script Requirements**
   - Scripts must have functions (no top-level executable code)
   - Call `run_tests()` or similar from `_ready()`
   - Example:
     ```ferris
     fn _ready() {
         run_tests();
     }
     
     fn run_tests() {
         print("Tests running!");
     }
     ```

3. **Run Scene**
   - Press F5 or click Play button
   - Check Output panel for script output
   - Any `print()` calls will appear in Godot's Output

### 🆘 Still Not Working?

If FerrisScriptNode still doesn't appear:

1. Check Godot version compatibility (needs 4.1+)
2. Verify you're on Windows x64 (or rebuild for your platform)
3. Check for antivirus blocking the DLL
4. Try building in debug mode:
   ```powershell
   cargo build --package ferrisscript_godot_bind
   # Update .gdextension to point to target/debug/ferrisscript_godot_bind.dll
   ```
5. Check Godot's log file in `%APPDATA%\Godot\app_userdata\`

### 📚 Reference

- **FerrisScript Documentation**: `docs/`
- **Godot GDExtension Docs**: https://docs.godotengine.org/en/stable/tutorials/scripting/gdextension/index.html
- **gdext (Rust bindings)**: https://godot-rust.github.io/
