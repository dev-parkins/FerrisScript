# Move Example

**Difficulty**: Beginner  
**Concepts**: Frame-by-frame updates, Delta time, Property access, Arithmetic operators

## What This Demonstrates

This example shows a basic animation in Godot. It demonstrates:

- Using the `_process(delta)` lifecycle hook
- Accessing node properties (`self.position.x`)
- Performing arithmetic with delta time
- Modifying a node's position every frame

## The Code

```ferris
fn _process(delta: f32) {
    self.position.x += 50.0 * delta;
}
```

## Line-by-Line Explanation

### `fn _process(delta: f32) {`

- `fn`: Declares a function
- `_process`: Special function name for Godot's frame update callback
  - Called **every frame** (typically 60 times per second)
  - Used for animations, movement, and input handling
- `delta: f32`: Parameter declaration
  - `delta`: Time elapsed since the last frame (in seconds)
  - `: f32`: Type annotation (32-bit floating point number)
  - Typically `delta` ≈ 0.016 seconds (1/60 for 60 FPS)

**Why `_process` instead of `_physics_process`?**

- `_process(delta)`: Runs every visual frame (60 FPS or monitor refresh rate)
- `_physics_process(delta)`: Runs every physics frame (fixed 60 FPS by default)

Use `_process` for visual effects and non-physics movement (like this example).  
Use `_physics_process` for physics-based movement (collisions, velocity, etc.).

### `self.position.x += 50.0 * delta;`

This is the core of the animation. Let's break it down into parts:

#### `self`

- Refers to the **current Godot node** that this script is attached to
- In FerrisScript, `self` is a special variable (like `this` in C++/C# or `self` in Python)
- You can access properties and methods of the node through `self`

#### `.position`

- Accesses the `position` property of the node
- `position` is a `Vector2` (2D coordinate: `x` and `y`)
- All `Node2D` subclasses have this property (Sprite2D, CharacterBody2D, etc.)

#### `.x`

- Accesses the `x` component of the `Vector2`
- `x` is the horizontal position (left-right)
- `y` would be the vertical position (up-down, not used here)

#### `+=`

- Compound assignment operator: "add to and assign"
- `a += b` is shorthand for `a = a + b`
- Here: "add something to `self.position.x` and store the result back"

#### `50.0 * delta`

- Multiplies `50.0` (pixels per second) by `delta` (seconds per frame)
- Result: **pixels to move this frame**

**Example calculation:**

- Speed: 50 pixels/second
- Delta: 0.016 seconds (one frame at 60 FPS)
- Movement: 50 × 0.016 = **0.8 pixels per frame**
- Over 60 frames (1 second): 0.8 × 60 = **50 pixels**

**Why multiply by delta?**  
Without `delta`, movement would be **framerate-dependent**:

- 60 FPS: moves 50 pixels per frame = 3000 pixels/second
- 30 FPS: moves 50 pixels per frame = 1500 pixels/second

With `delta`, movement is **framerate-independent**:

- 60 FPS: moves 0.8 pixels per frame = 50 pixels/second
- 30 FPS: moves 1.6 pixels per frame = 50 pixels/second

This ensures the animation looks the same regardless of performance.

#### `;`

Ends the statement.

## Running This Example

### Prerequisites

- Godot project with FerrisScript GDExtension installed (see [hello example](../hello/README.md))
- A scene with a `Node2D` or `Sprite2D` node

### Steps

1. **Attach the script**:
   - Add a `FerrisScriptNode` to your scene
   - Set `script_path` to `res://path/to/examples/move.ferris`

2. **Add a visual element** (optional but recommended):
   - Add a `Sprite2D` as a child of the `FerrisScriptNode`
   - Assign a texture (any image) to the `Sprite2D`
   - This lets you **see** the movement

3. **Run the scene** (F5)

4. **Observe**:
   - The node moves to the right at **50 pixels per second**
   - It will continue forever (no boundary checks)

## Expected Behavior

- **Initial position**: Whatever you set in the editor (default: `(0, 0)`)
- **Movement**: Steady rightward motion
- **Speed**: 50 pixels per second
- **Direction**: Always right (positive X)

After ~20 seconds, the node will have moved ~1000 pixels off-screen (typical screen width: 1920 pixels).

## Common Gotchas

### 1. Node Moves Too Fast or Too Slow

**Problem**: Movement doesn't feel right.

**Solution**: Adjust the speed constant:

```ferris
// Slower (10 pixels/second)
self.position.x += 10.0 * delta;

// Faster (200 pixels/second)
self.position.x += 200.0 * delta;
```

Try values between `10.0` and `500.0` to find what feels good.

### 2. Node Doesn't Move

**Problem**: No visible movement.

**Solutions**:

- Ensure the node is a `Node2D` or subclass (has a `position` property)
- Check that the node is **visible** (not hidden or off-screen)
- Add a `print(self.position.x)` to verify position is changing
- Make sure `_process()` is being called (add `print("Frame!")` temporarily)

### 3. Node Moves Vertically Instead

**Problem**: Node moves up/down instead of left/right.

**Solution**: You modified `y` instead of `x`. Use:

```ferris
self.position.y += 50.0 * delta;  // Vertical (down is positive)
```

**Godot's coordinate system:**

- `+X`: Right
- `-X`: Left
- `+Y`: Down (not up!)
- `-Y`: Up

### 4. Movement is Choppy/Stuttery

**Problem**: Animation isn't smooth.

**Solutions**:

- Check framerate (Godot shows FPS in the top-right during play)
- Ensure V-Sync is enabled (Project Settings → Display → Window → V-Sync Mode → Enabled)
- Use `_process` (not `_physics_process`) for visual animations
- Close other programs that might be using CPU/GPU

### 5. Node Moves Off-Screen and Disappears

**Expected behavior!** This example has no boundary checks.  
See the [Bounce Example](../bounce/README.md) to learn how to add boundaries.

## Variations to Try

### 1. Move Left Instead

```ferris
fn _process(delta: f32) {
    self.position.x -= 50.0 * delta;  // Negative = left
}
```

### 2. Move Vertically

```ferris
fn _process(delta: f32) {
    self.position.y += 50.0 * delta;  // Down
}

// Or move up:
fn _process(delta: f32) {
    self.position.y -= 50.0 * delta;  // Up
}
```

### 3. Move Diagonally

```ferris
fn _process(delta: f32) {
    self.position.x += 50.0 * delta;  // Right
    self.position.y += 50.0 * delta;  // Down
}
```

### 4. Variable Speed

```ferris
let mut speed: f32 = 50.0;

fn _process(delta: f32) {
    self.position.x += speed * delta;
    speed += 10.0 * delta;  // Accelerate over time
}
```

This makes the node speed up as it moves (like a car accelerating).

### 5. User-Controlled Speed

```ferris
let speed: f32 = 100.0;

fn _process(delta: f32) {
    // TODO: Add input handling (future FerrisScript feature)
    // For now, speed is constant
    self.position.x += speed * delta;
}
```

**Note**: Input handling is not yet implemented in FerrisScript v0.0.1. See [v0.1.0-ROADMAP.md](../../docs/v0.1.0-ROADMAP.md) for planned features.

### 6. Print Position (Debugging)

```ferris
fn _process(delta: f32) {
    self.position.x += 50.0 * delta;
    
    // Print every 60 frames (once per second at 60 FPS)
    let frame_count = self.position.x / 50.0;
    if frame_count % 1.0 < delta {
        print("Position X:", self.position.x);
    }
}
```

This prints the X position approximately once per second.

## Physics vs Visual Movement

### Use `_process` when

- Animating sprites (position, rotation, scale)
- UI animations (fading, sliding)
- Non-gameplay effects (particles, shaders)
- Following the mouse cursor

### Use `_physics_process` when

- Applying forces/velocity
- Handling collisions
- Character controllers
- Projectiles
- Anything that interacts with physics

This example uses `_process` because:

- No physics interactions (no collisions, no velocity)
- Simple visual movement (like a scrolling background)

## Performance Considerations

This example is **very efficient**:

- Only two arithmetic operations per frame (`*` and `+=`)
- No memory allocation
- No function calls (besides `_process` itself)

Even with 1000+ nodes running this script, performance would be excellent.

**Comparison to GDScript:**  
This FerrisScript version is roughly **equivalent** in performance to:

```gdscript
func _process(delta):
    position.x += 50.0 * delta
```

## Next Steps

After understanding this example:

1. **[Bounce Example](../bounce/README.md)**: Add boundaries and conditionals
2. **[ARCHITECTURE.md](../../docs/ARCHITECTURE.md)**: Learn about variables, types, operators, and how `self.position` works internally
3. **[v0.1.0-ROADMAP.md](../../docs/v0.1.0-ROADMAP.md)**: See what's planned for future releases

## Questions?

- **GitHub Issues**: [Report bugs or ask questions](https://github.com/dev-parkins/FerrisScript/issues)
- **GitHub Discussions**: [General questions and ideas](https://github.com/dev-parkins/FerrisScript/discussions)
- **Documentation**: [Full documentation](../../docs/)
