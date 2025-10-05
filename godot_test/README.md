# Godot Test Project

This is a minimal Godot 4.x project used for testing the scripting language integration.

## 📁 Project Structure

```
godot_test/
├── project.godot          # Godot project configuration
├── ferrisscript.gdextension # GDExtension configuration
├── test_scene.tscn        # Main test scene
└── scripts/               # Test scripts
    ├── hello.ferris         # Phase 6 test: _ready callback
    ├── move_test.ferris     # Phase 7 test: self.position modification
    ├── process_test.ferris  # Phase 7 test: _process callback
    └── bounce_test.ferris   # Phase 8 test: mutable state & control flow
```

## 🚀 Quick Start

### Building the Extension

From the repository root:

```bash
cargo build --package FerrisScript_godot_bind
```

This creates the DLL that Godot loads via `ferrisscript.gdextension`.

### Opening the Project

1. Open Godot 4.2+
2. Click "Import"
3. Navigate to `godot_test/project.godot`
4. Click "Import & Edit"

### Running Your First Test

1. Open `test_scene.tscn`
2. Press F5 to run
3. Check Output panel for: "Hello, Godot! FerrisScript is working!"

## 🧪 Test Scripts Overview

| Script | Phase | Tests | Expected Behavior |
|--------|-------|-------|-------------------|
| hello.ferris | 6 | _ready callback | Prints "Hello, Godot!" |
| move_test.ferris | 7 | self.position modification | Node moves right continuously |
| process_test.ferris | 7 | _process callback | Prints "Delta" every frame |
| bounce_test.ferris | 8 | Mutable state & control flow | Node bounces between boundaries |

### Detailed Test Instructions

See individual test sections below for setup and validation steps.

## 📝 Test 1: hello.ferris (Phase 6)

**Purpose**: Validates script loading and _ready() execution.

**Setup:**

1. Add FerrisScriptNode to scene
2. Set `script_path` to `res://scripts/hello.ferris`
3. Run scene (F5)

**Expected Output:**

```
Successfully loaded FerrisScript: res://scripts/hello.ferris
Hello, Godot! FerrisScript is working!
```

## 📝 Test 2: move_test.ferris (Phase 7)

**Purpose**: Validates _process() and self.position modification.

**Setup:**

1. Add FerrisScriptNode to scene
2. Add child Sprite2D (for visualization)
3. Set `script_path` to `res://scripts/move_test.ferris`
4. Run scene (F5)

**Expected Behavior:**

- Node moves right at 50 pixels/second
- Movement is smooth without stuttering

## 📝 Test 3: bounce_test.ferris (Phase 8)

**Purpose**: Validates mutable state persistence and control flow.

**Setup:**

1. Add FerrisScriptNode to scene
2. Add child Sprite2D with offset position
3. Set `script_path` to `res://scripts/bounce_test.ferris`
4. Run scene (F5)

**Expected Behavior:**

- Node bounces between x=-200 and x=200
- Direction reverses at boundaries
- Movement is smooth at 100 pixels/second

## 🐛 Troubleshooting

### Extension Not Loading

- Build the extension: `cargo build --package FerrisScript_godot_bind`
- Verify DLL exists: `target/debug/FerrisScript_godot_bind.dll`
- Check Godot console for errors

### Script Doesn't Run

- Verify `script_path` is set in Inspector
- Check path uses `res://` prefix
- Look for compilation errors in console

### Node Doesn't Move

- Ensure scene is running (F5)
- Check FPS is stable (View → Show FPS)
- Verify child sprite/visual element is present

## ✏️ Adding New Test Scripts

### Creating a New Test Script

1. **Create the `.ferris` file** in `scripts/` directory:

   ```bash
   # Example: Create a new rotation test
   touch godot_test/scripts/rotation_test.ferris
   ```

2. **Write your FerrisScript code**:

   ```rust
   fn _ready() {
       print("Rotation test initialized!");
   }
   
   fn _process(delta: f32) {
       // Rotate the node
       self.rotation = self.rotation + (1.0 * delta);
   }
   ```

3. **Add the script to a scene**:
   - Open `test_scene.tscn` in Godot
   - Add a new Node2D or use existing FerrisScriptNode
   - Set `script_path` to `res://scripts/rotation_test.ferris`

4. **Test the script**:
   - Press F5 to run the scene
   - Verify expected behavior in output/visually
   - Check for errors in Godot console

### Test Script Template

Use this template for new test scripts:

```rust
// Test: [Brief description]
// Expected: [Expected behavior]

fn _ready() {
    print("Test initialized: [test name]");
    // Setup code here
}

fn _process(delta: f32) {
    // Per-frame logic here
}
```

### Testing Best Practices

- **Start simple**: Test one feature at a time
- **Print debug info**: Use `print()` to verify values
- **Test incrementally**: Build complexity gradually
- **Document expectations**: Add comments explaining expected behavior
- **Clean up**: Remove or comment out debug prints when test passes

### Common Test Patterns

**Testing Position**:

```rust
fn _process(delta: f32) {
    self.position.x = self.position.x + (50.0 * delta);
    print("Position: ");
    print(self.position.x);
}
```

**Testing State**:

```rust
fn _ready() {
    let mut counter: i32 = 0;
}

fn _process(delta: f32) {
    counter = counter + 1;
    if counter > 60 {
        print("One second passed!");
        counter = 0;
    }
}
```

**Testing Conditionals**:

```rust
fn _process(delta: f32) {
    if self.position.x > 100.0 {
        print("Reached target position!");
    } else {
        self.position.x = self.position.x + (50.0 * delta);
    }
}
```

## 📚 Additional Resources

- **Main Documentation**: See `../README.md`
- **Testing Guide**: See `../docs/v0.0.2/TESTING.md`
- **Example Scripts**: See `../examples/` directory
- **Contributing**: See `../CONTRIBUTING.md`

---

**Last Updated**: October 5, 2025  
**Godot Version**: 4.2+  
**FerrisScript Version**: 0.0.2
