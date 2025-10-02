# Hello World Example

**Difficulty**: Beginner  
**Concepts**: Functions, Print statements, Godot lifecycle hooks

## What This Demonstrates

This is the simplest FerrisScript example. It demonstrates:
- Defining a function
- Using the `_ready()` lifecycle hook
- Calling the `print()` builtin function
- Basic FerrisScript syntax

## The Code

```ferris
fn _ready() {
    print("Hello from FerrisScript!");
}
```

## Line-by-Line Explanation

### `fn _ready() {`

- `fn`: Keyword that declares a function
- `_ready`: Special function name recognized by Godot
  - Called **once** when the node enters the scene tree
  - Similar to GDScript's `_ready()` or Unity's `Start()`
- `()`: Parameter list (empty in this case - no parameters)
- `{`: Start of the function body

**Why `_ready` is special:**
Godot has a lifecycle system where nodes receive callbacks at specific times:
- `_ready()`: Called when node is added to scene (initialization)
- `_process(delta)`: Called every frame (animation/movement)
- `_physics_process(delta)`: Called every physics frame

FerrisScript recognizes these function names and connects them to Godot automatically.

### `print("Hello from FerrisScript!");`

- `print`: Builtin function that outputs text to the console
- `"Hello from FerrisScript!"`: String literal (text enclosed in double quotes)
- `;`: Semicolon ends the statement

**Where does the output go?**
- In Godot editor: **Output** panel at the bottom (View → Output)
- In exported game: Console output (varies by platform)

### `}`

Closes the function body.

## Running This Example

### Method 1: Standalone Test (Without Godot)

You can test FerrisScript compilation without Godot:

```powershell
# From the project root, run compiler tests:
cargo test --package ferrisscript_compiler test_compile_hello

# Or run all compiler tests:
cargo test --package ferrisscript_compiler
```

**Note**: This verifies the script compiles correctly but **does not execute** `print()` (requires Godot runtime).

**Why not `cargo run --example hello`?** The `.ferris` files are scripts for the FerrisScript language, not Rust examples. To run them, you need either:
- The Godot runtime (Method 2 below), or
- A standalone FerrisScript CLI (planned for v0.1.0)

### Method 2: In Godot (Recommended)

1. **Build the GDExtension**:
   ```powershell
   cargo build --release
   ```

2. **Copy the extension** to your Godot project:
   - Windows: `target/release/ferrisscript.dll`
   - Linux: `target/release/libferrisscript.so`
   - macOS: `target/release/libferrisscript.dylib`

3. **Create a Godot scene**:
   - Open Godot Editor
   - Create a new scene with a `Node2D` or any node
   - Attach a `FerrisScriptNode` (from the Add Node dialog)

4. **Configure the FerrisScriptNode**:
   - Select the `FerrisScriptNode` in the scene tree
   - In the Inspector, find the "Script Path" property
   - Set it to: `res://path/to/examples/hello.ferris` (adjust path as needed)

5. **Run the scene** (F5 or play button)

6. **Check the output**:
   - Look at the **Output** tab in Godot (bottom panel)
   - You should see: `Hello from FerrisScript!`

## Expected Output

```
Hello from FerrisScript!
```

This message will appear **once** when the scene starts (because `_ready()` is called once).

## Common Gotchas

### 1. Nothing Prints

**Problem**: You don't see any output.

**Solutions**:
- Check the **Output** tab in Godot (View → Output if hidden)
- Verify the `FerrisScriptNode` has the correct `script_path`
- Ensure the `.ferris` file path is relative to `res://` (Godot's resource root)
- Make sure you **ran the scene** (F5), not just opened it

### 2. "Failed to load script" Error

**Problem**: Godot can't find the `.ferris` file.

**Solutions**:
- Double-check the file path in the Inspector
- Use Godot's file browser (📁 button next to Script Path) to select the file
- Ensure the file is in your Godot project directory (under `res://`)

### 3. "Unknown function: print" Error

**Problem**: FerrisScript doesn't recognize `print`.

**Solutions**:
- This should not happen (bug in runtime)
- Verify you're using the latest build: `cargo build --release`
- Report an issue on GitHub if this persists

### 4. GDExtension Not Loading

**Problem**: Godot doesn't recognize `FerrisScriptNode`.

**Solutions**:
- Ensure you have a `.gdextension` file in your Godot project
- Check the file paths in the `.gdextension` file match your compiled library
- Restart Godot after building the extension
- See [TROUBLESHOOTING.md](../../docs/TROUBLESHOOTING.md) for detailed GDExtension debugging

## Variations to Try

### 1. Multiple Print Statements

```ferris
fn _ready() {
    print("Hello from FerrisScript!");
    print("This is line 2");
    print("This is line 3");
}
```

Each `print()` outputs on a new line.

### 2. Print Variables

```ferris
fn _ready() {
    let name = "FerrisScript";
    let version = 0.1;
    print("Language:", name);
    print("Version:", version);
}
```

`print()` can take multiple arguments separated by commas.

### 3. Print with Expressions

```ferris
fn _ready() {
    print("2 + 2 =", 2 + 2);
    print("10 * 5 =", 10 * 5);
}
```

Expressions are evaluated before printing.

### 4. Combine with `_process`

```ferris
fn _ready() {
    print("Scene started!");
}

fn _process(delta: f32) {
    print("Frame time:", delta);
}
```

**Warning**: This will print every frame (60+ times per second)! Your console will fill up quickly.

## Next Steps

After understanding this example:

1. **[Move Example](../move/README.md)**: Learn about `_process()` and animation
2. **[Bounce Example](../bounce/README.md)**: Explore conditionals and state
3. **[LANGUAGE_REFERENCE.md](../../docs/LANGUAGE_REFERENCE.md)**: Complete syntax guide

## Questions?

- **GitHub Issues**: [Report bugs or ask questions](https://github.com/dev-parkins/FerrisScript/issues)
- **GitHub Discussions**: [General questions and ideas](https://github.com/dev-parkins/FerrisScript/discussions)
- **Documentation**: [Full documentation](../../docs/)
