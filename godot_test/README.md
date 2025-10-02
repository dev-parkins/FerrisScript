# Godot Test Project

This is a minimal Godot 4.x project used for testing the scripting language integration.

## 📁 Project Structure

```
godot_test/
├── project.godot          # Godot project configuration
├── rustyscript.gdextension # GDExtension configuration
├── test_scene.tscn        # Main test scene
└── scripts/               # Test scripts
    ├── hello.rscr         # Phase 6 test: _ready callback
    ├── move_test.rscr     # Phase 7 test: self.position modification
    ├── process_test.rscr  # Phase 7 test: _process callback
    └── bounce_test.rscr   # Phase 8 test: mutable state & control flow
```

## 🚀 Quick Start

### Building the Extension

From the repository root:
```bash
cargo build --package rustyscript_godot_bind
```

This creates the DLL that Godot loads via `rustyscript.gdextension`.

### Opening the Project

1. Open Godot 4.2+
2. Click "Import"
3. Navigate to `godot_test/project.godot`
4. Click "Import & Edit"

### Running Your First Test

1. Open `test_scene.tscn`
2. Press F5 to run
3. Check Output panel for: "Hello, Godot! RustyScript is working!"

## 🧪 Test Scripts Overview

| Script | Phase | Tests | Expected Behavior |
|--------|-------|-------|-------------------|
| hello.rscr | 6 | _ready callback | Prints "Hello, Godot!" |
| move_test.rscr | 7 | self.position modification | Node moves right continuously |
| process_test.rscr | 7 | _process callback | Prints "Delta" every frame |
| bounce_test.rscr | 8 | Mutable state & control flow | Node bounces between boundaries |

### Detailed Test Instructions

See individual test sections below for setup and validation steps.

## 📝 Test 1: hello.rscr (Phase 6)

**Purpose**: Validates script loading and _ready() execution.

**Setup:**
1. Add RustyScriptNode to scene
2. Set `script_path` to `res://scripts/hello.rscr`
3. Run scene (F5)

**Expected Output:**
```
Successfully loaded RustyScript: res://scripts/hello.rscr
Hello, Godot! RustyScript is working!
```

## 📝 Test 2: move_test.rscr (Phase 7)

**Purpose**: Validates _process() and self.position modification.

**Setup:**
1. Add RustyScriptNode to scene
2. Add child Sprite2D (for visualization)
3. Set `script_path` to `res://scripts/move_test.rscr`
4. Run scene (F5)

**Expected Behavior:**
- Node moves right at 50 pixels/second
- Movement is smooth without stuttering

## 📝 Test 3: bounce_test.rscr (Phase 8)

**Purpose**: Validates mutable state persistence and control flow.

**Setup:**
1. Add RustyScriptNode to scene
2. Add child Sprite2D with offset position
3. Set `script_path` to `res://scripts/bounce_test.rscr`
4. Run scene (F5)

**Expected Behavior:**
- Node bounces between x=-200 and x=200
- Direction reverses at boundaries
- Movement is smooth at 100 pixels/second

## 🐛 Troubleshooting

### Extension Not Loading
- Build the extension: `cargo build --package rustyscript_godot_bind`
- Verify DLL exists: `target/debug/rustyscript_godot_bind.dll`
- Check Godot console for errors

### Script Doesn't Run
- Verify `script_path` is set in Inspector
- Check path uses `res://` prefix
- Look for compilation errors in console

### Node Doesn't Move
- Ensure scene is running (F5)
- Check FPS is stable (View → Show FPS)
- Verify child sprite/visual element is present

## 📚 Additional Resources

- **Main Documentation**: See `../README.md`
- **Phase Testing Guides**: See `../docs/PHASE*_TESTING.md`
- **Development Checklist**: See `../docs/copilot-checklist.md`

---

**Last Updated**: October 1, 2025  
**Godot Version**: 4.5  
**Extension Version**: 0.0.1
