# Phase 6: Godot Integration - Build and Test Guide

## Overview
This guide provides step-by-step instructions for building and testing the FerrisScript Godot integration.

## How FerrisScript Files Work in Godot

**Design Philosophy:**
- `.ferris` files are **asset files**, not Godot scripts
- They live inside your Godot project, just like textures, sounds, or JSON files
- Reference them using `res://` paths (e.g., `res://scripts/hello.ferris`)
- Godot's `FileAccess` API reads them at runtime
- Our extension compiles and executes them on-demand

**Why not use Godot's script system?**
- FerrisScript is a custom language, not GDScript/C#
- We want full control over compilation and execution
- This allows hot-reloading, custom error handling, and future optimizations

## Prerequisites

1. **Rust toolchain** (already installed)
2. **Godot 4.2 or higher** - Download from https://godotengine.org/download
3. **C++ compiler** (required by gdext):
   - Windows: Visual Studio 2019+ or MSVC build tools
   - Linux: GCC or Clang
   - macOS: Xcode Command Line Tools

## Build Instructions

### 1. Build the GDExtension Library

From the project root:

```powershell
# Debug build (faster compile, slower runtime)
cargo build --package FerrisScript_godot_bind

# Release build (slower compile, faster runtime)
cargo build --package FerrisScript_godot_bind --release
```

**Expected output:**
- Windows: `target/debug/FerrisScript_godot_bind.dll` (or `target/release/...`)
- Linux: `target/debug/libFerrisScript_godot_bind.so`
- macOS: `target/debug/libFerrisScript_godot_bind.dylib`

### 2. Verify the Build

Check that the library file exists:

```powershell
# Windows
ls target\debug\FerrisScript_godot_bind.dll

# Should show the file with timestamp
```

## Testing in Godot

### Option A: Using the Test Project (Recommended)

1. **Open Godot Editor**
   - Launch Godot 4.2+
   - Click "Import"
   - Navigate to `Y:\cpark\Projects\FerrisScript\godot_test\project.godot`
   - Click "Import & Edit"

2. **Verify Extension Loaded**
   - Check the Output panel (bottom of editor)
   - You should see: `GDExtension successfully loaded: res://FerrisScript.gdextension`
   - If you see errors, check the Troubleshooting section below

3. **Open Test Scene**
   - In the FileSystem dock, double-click `test_scene.tscn`
   - You should see a scene tree with:
     - TestScene (Node)
       - FerrisScriptTest (FerrisScriptNode)

4. **Inspect FerrisScriptNode**
   - Click on "FerrisScriptTest" node
   - In the Inspector panel, verify:
     - `Script Path` property is visible
     - Value is set to `res://scripts/hello.ferris`
   - **Note:** `.ferris` files are treated as assets, placed inside the Godot project like textures or sounds

5. **Run the Scene**
   - Click the "Play Scene" button (F6) or "Play" button (F5)
   - Check the Output panel

### Option B: Manual Scene Setup

1. **Open Godot and Create New Scene**
   - File → New Scene
   - Add a Node as root (rename to "TestScene")

2. **Add FerrisScriptNode**
   - Right-click on TestScene
   - Add Child Node
   - Search for "FerrisScriptNode"
   - If you don't see it, the extension isn't loaded properly

3. **Configure Script Path**
   - Select the FerrisScriptNode
   - In Inspector, find "Script Path" property
   - Set to: `res://scripts/hello.ferris`
   - **Important:** `.ferris` files should be placed inside your Godot project directory
   - Use `res://` paths just like any other Godot asset

4. **Save and Run**
   - Save scene as `test_scene.tscn`
   - Press F6 to run

## Acceptance Criteria

### ✅ **Success Criteria**

When you run the test scene, you should see:

**In Godot's Output Panel:**
```
Successfully loaded FerrisScript: res://scripts/hello.ferris
Hello, Godot! FerrisScript is working!
```

**Behavior Verification:**
1. ✅ No compilation errors when building the extension
2. ✅ Godot loads the extension without errors
3. ✅ FerrisScriptNode appears in "Create New Node" dialog
4. ✅ FerrisScriptNode has `script_path` property in Inspector
5. ✅ Setting `script_path` loads and compiles the .ferris file
6. ✅ Running the scene executes the `_ready()` function
7. ✅ `print("Hello, Godot!")` outputs to Godot console

### 🧪 **Extended Testing**

Test with different example files:

**Test 1: Branch Logic**
- Set `script_path` to `res://../examples/branch.ferris`
- Run scene
- Should see output from if/else branches

**Test 2: Global Variables**
- Set `script_path` to `res://../examples/bounce.ferris`
- Run scene
- Should initialize without errors (no output expected yet)

**Test 3: Functions**
- Set `script_path` to `res://../examples/functions.ferris`
- Run scene
- Should execute function definitions without errors

**Test 4: Error Handling**
- Set `script_path` to `res://../examples/type_error.ferris`
- Run scene
- Should see error message in console: "Type mismatch..."

**Test 5: Invalid Path**
- Set `script_path` to `res://nonexistent.ferris`
- Run scene
- Should see error: "Failed to read script file..."

**Test 6: Hot Reload**
- Run scene with hello.ferris
- Edit hello.ferris (change the message)
- In Godot, click the FerrisScriptNode
- In Inspector, find the "Reload Script" method (may be under "Methods")
- Call the method
- Verify new message appears in output

## Troubleshooting

### Extension Not Loading

**Error: "Can't open dynamic library"**
- Solution: Rebuild the extension with `cargo build --package FerrisScript_godot_bind`
- Verify the DLL/SO/DYLIB file exists in target/debug/
- Check that the path in `FerrisScript.gdextension` matches your build location

**Error: "No loader found for resource"**
- Solution: Ensure `FerrisScript.gdextension` is in the project root
- Verify the entry_symbol is correct: `gdext_rust_init`

**Error: "Entry symbol not found"**
- Solution: This usually means the Rust crate type isn't set correctly
- Verify `crate-type = ["cdylib"]` in `godot_bind/Cargo.toml`

### Script Not Loading

**Error: "Failed to read script file"**
- Check that the path is correct relative to Godot project
- Try absolute path: `Y:/cpark/Projects/FerrisScript/examples/hello.ferris`
- Verify file exists and has .ferris extension

**Error: "Failed to compile script"**
- Check the script for syntax errors
- Run the compiler tests to verify: `cargo test -p FerrisScript_compiler`
- Check error message for specific line/column

### Runtime Errors

**Error: "Error calling function '_ready'"**
- Check that the script defines a `_ready()` function
- Verify function signature is correct: `fn _ready() { ... }`
- Check runtime tests: `cargo test -p FerrisScript_runtime`

## Project Structure

```
FerrisScript/
├── FerrisScript.gdextension          # Extension manifest
├── crates/
│   └── godot_bind/
│       ├── Cargo.toml               # cdylib configuration
│       └── src/
│           └── lib.rs               # FerrisScriptNode implementation
├── examples/
│   ├── hello.ferris                   # Test script
│   ├── branch.ferris
│   ├── loop.ferris
│   └── ...
├── godot_test/                      # Test Godot project
│   ├── project.godot                # Godot project file
│   └── test_scene.tscn              # Test scene with FerrisScriptNode
└── target/
    └── debug/
        └── FerrisScript_godot_bind.dll  # Built extension (Windows)
```

## Next Steps After Phase 6

Once Phase 6 is verified working:

- **Phase 7**: Implement `_process()` callback with delta parameter
- **Phase 8**: Implement `self` binding for node property access
- **Phase 9**: Test `move.ferris` and `bounce.ferris` with full Godot integration

## Manual Testing Checklist

Copy this checklist to verify Phase 6 completion:

```
Phase 6 Testing Checklist
========================

Build Verification:
[ ] Cargo build completes without errors
[ ] DLL/SO/DYLIB file exists in target/debug/

Godot Integration:
[ ] Godot loads extension without errors
[ ] FerrisScriptNode appears in node list
[ ] script_path property visible in Inspector
[ ] Can set script_path to hello.ferris
[ ] Scene runs without crashing

Runtime Verification:
[ ] Output shows: "Successfully loaded FerrisScript: ..."
[ ] Output shows: "Hello, Godot!"
[ ] branch.ferris executes without errors
[ ] functions.ferris executes without errors
[ ] type_error.ferris shows error message
[ ] Invalid path shows error message

Advanced Features:
[ ] reload_script() method works
[ ] Can change script_path at runtime
[ ] Multiple FerrisScriptNode instances work independently

Date Tested: __________
Tester: __________
Godot Version: __________
Result: PASS / FAIL
Notes: _______________________________________________
```

## Reporting Issues

If acceptance criteria are not met, provide:

1. Godot version (Help → About)
2. Build output from `cargo build`
3. Godot console output (copy all errors/warnings)
4. Screenshot of Inspector showing FerrisScriptNode properties
5. Contents of hello.ferris being tested
6. Operating system and Rust version

## Success Indicators

✅ **Phase 6 is complete when:**
- Extension builds without errors
- Godot loads extension successfully
- hello.ferris executes and prints to console
- Error handling works (invalid files show errors)
- Documentation is clear and complete

🎉 **Ready for Phase 7 when all acceptance criteria pass!**

