# Bounce Example

**Difficulty**: Intermediate  
**Concepts**: Global variables, Mutability, Conditionals, State management, Boundary checks

## What This Demonstrates

This example shows a bouncing animation with boundary checks. It demonstrates:

- Global variables (`let mut dir`)
- Conditional statements (`if`)
- State management (tracking direction)
- Boundary detection (preventing off-screen movement)
- More complex frame-by-frame logic

## The Code

```ferris
let mut dir: f32 = 1.0;

fn _process(delta: f32) {
    self.position.x += dir * 100.0 * delta;

    if self.position.x > 10.0 {
        dir = -1.0;
    }
    if self.position.x < -10.0 {
        dir = 1.0;
    }
}
```

## Line-by-Line Explanation

### `let mut dir: f32 = 1.0;`

This is a **global variable** declaration.

#### `let`

- Declares a variable (like `var` in JavaScript or `let` in Rust)
- Variables must be declared before use (no implicit creation)

#### `mut`

- Makes the variable **mutable** (can be changed after initialization)
- Without `mut`, the variable would be **immutable** (constant)
- Compare:

  ```ferris
  let x = 5;      // Immutable: x cannot be changed
  let mut y = 5;  // Mutable: y can be reassigned
  ```

**Why is `mut` needed here?**  
We reassign `dir` in the `if` statements (`dir = -1.0` and `dir = 1.0`). Without `mut`, the type checker would reject the script with an error:

```
Cannot assign to immutable variable 'dir'
```

#### `: f32`

- Type annotation (optional in FerrisScript)
- `f32`: 32-bit floating point number
- Could also write `let mut dir = 1.0;` (type inferred from `1.0`)

#### `= 1.0`

- Initializes `dir` to `1.0` (positive direction, moving right)
- In FerrisScript, variables **must be initialized** when declared

#### Global vs Local Variables

**This variable is global** because it's declared **outside any function**.

- **Global**: Declared at the top level (before `fn`)
  - Persists across function calls
  - Retains its value between frames
  - Used for state that needs to survive

- **Local**: Declared inside a function (after `fn _process() {`)
  - Destroyed when function ends
  - Recreated every function call
  - Used for temporary calculations

**Example of local (WRONG for this use case):**

```ferris
fn _process(delta: f32) {
    let mut dir = 1.0;  // WRONG: Resets to 1.0 every frame!
    self.position.x += dir * 100.0 * delta;
    // ... direction changes have no effect
}
```

This would **always move right** because `dir` is reset to `1.0` every frame.

### `fn _process(delta: f32) {`

Same as the [move example](../move/README.md):

- Called every frame
- `delta`: Time since last frame (seconds)

### `self.position.x += dir * 100.0 * delta;`

Similar to the move example, but with a **direction multiplier**.

#### `dir *`

- Multiplies the speed by the direction
- If `dir = 1.0`: moves right (positive X)
- If `dir = -1.0`: moves left (negative X)

#### `100.0`

- Speed in pixels per second (faster than the move example's `50.0`)

#### Complete movement calculation

**Moving right (`dir = 1.0`):**

- `1.0 * 100.0 * 0.016` = `1.6 pixels per frame` (at 60 FPS)
- **100 pixels per second** to the right

**Moving left (`dir = -1.0`):**

- `-1.0 * 100.0 * 0.016` = `-1.6 pixels per frame`
- **100 pixels per second** to the left

### `if self.position.x > 10.0 {`

First boundary check: **right edge**

#### `if`

- Conditional statement: executes the block only if the condition is true
- Similar to `if` in most programming languages

#### `self.position.x > 10.0`

- Condition: "Is the X position greater than 10?"
- `>`: Greater-than comparison operator
- Returns `true` or `false` (boolean value)

**Why 10.0?**  
This is an arbitrary boundary. In a real game:

- Use the screen width (e.g., `screen_width / 2` for center-based coordinates)
- Use the node's size (e.g., `sprite.width / 2` to account for the node's own width)
- Use world boundaries (e.g., tilemap edges)

**This example uses small boundaries for demonstration purposes.**

#### `dir = -1.0;`

- **Reassignment**: Sets `dir` to `-1.0` (reverse direction)
- This is allowed because `dir` was declared with `mut`
- Next frame, the node will move **left** instead of **right**

#### `}`

Closes the `if` block.

### `if self.position.x < -10.0 {`

Second boundary check: **left edge**

#### `< -10.0`

- Condition: "Is the X position less than -10?"
- `<`: Less-than comparison operator
- `-10.0`: Negative boundary (left side)

**Coordinate system reminder:**

- `x = 0`: Center
- `x > 0`: Right of center
- `x < 0`: Left of center

#### `dir = 1.0;`

- Reverses direction to **right** (positive)
- Next frame, the node will move **right** again

### `}`

Closes the `_process` function.

## How It Works (Step by Step)

Let's trace the execution over several frames:

### Frame 1 (Initial State)

- `dir = 1.0` (moving right)
- `self.position.x = 0.0` (center)
- Movement: `0.0 + (1.0 * 100.0 * 0.016)` = `1.6`
- New position: `1.6`
- No boundary hit (not > 10.0 or < -10.0)
- Direction unchanged

### Frames 2-6

- Continues moving right
- Position: `3.2`, `4.8`, `6.4`, `8.0`, `9.6`
- No boundary hit yet

### Frame 7

- `self.position.x = 9.6 + 1.6` = `11.2`
- **Boundary hit!** `11.2 > 10.0` is `true`
- `dir` changes to `-1.0` (reverse)

### Frame 8

- `dir = -1.0` (moving left now)
- Movement: `11.2 + (-1.0 * 100.0 * 0.016)` = `11.2 - 1.6` = `9.6`
- New position: `9.6`
- No boundary hit (not < -10.0, and now not > 10.0)

### Frames 9-20

- Continues moving left
- Position: `8.0`, `6.4`, ... `-8.0`, `-9.6`

### Frame 21

- `self.position.x = -9.6 - 1.6` = `-11.2`
- **Boundary hit!** `-11.2 < -10.0` is `true`
- `dir` changes to `1.0` (reverse)

### Frame 22+

- Starts moving right again
- Cycle repeats forever

**Result**: The node bounces back and forth between X positions `-10.0` and `10.0`.

## Running This Example

### Setup

Same as the [move example](../move/README.md):

1. Build GDExtension: `cargo build --release`
2. Add `FerrisScriptNode` to scene
3. Set `script_path` to `res://path/to/examples/bounce.ferris`
4. Add a `Sprite2D` child (so you can see the bouncing)

### Expected Behavior

- Node starts at center (or your initial position)
- Moves right at 100 pixels/second
- Reverses at X = 10.0
- Moves left at 100 pixels/second
- Reverses at X = -10.0
- Repeats forever

**Visual effect**: Looks like a ball bouncing between two walls.

## Common Gotchas

### 1. Node Doesn't Reverse Direction

**Problem**: Node moves past the boundaries.

**Solutions**:

- Check that `dir` is declared **global** (before `fn _process`)
- Verify `mut` keyword is present (`let mut dir`)
- Add `print(dir)` to debug: see if `dir` changes
- Add `print(self.position.x)` to see current position

**Example debugging:**

```ferris
fn _process(delta: f32) {
    print("Position:", self.position.x, "Direction:", dir);
    // ... rest of code
}
```

### 2. Node Reverses Too Early or Too Late

**Problem**: Boundaries don't match expectations.

**Solution**: Adjust the boundary values:

```ferris
if self.position.x > 500.0 {  // Right edge of 1000px-wide screen
    dir = -1.0;
}
if self.position.x < -500.0 {  // Left edge
    dir = 1.0;
}
```

**Common screen widths:**

- 1920 (Full HD): Use ±960 for center-based coordinates
- 1280 (HD): Use ±640
- 800 (small window): Use ±400

### 3. Node "Sticks" at Boundary

**Problem**: Node oscillates rapidly at the boundary.

**Cause**: The boundary check happens **after** movement, so the node can overshoot.

**Example at 60 FPS:**

- Frame N: `position.x = 9.6`, moves to `11.2` (overshoots 10.0)
- Boundary hit, `dir = -1.0`
- Frame N+1: `position.x = 11.2`, moves to `9.6`
- No boundary hit (9.6 < 10.0)
- Frame N+2: `position.x = 9.6`, moves to `11.2`
- Boundary hit again...

**Solution** (if needed): Check boundary **before** applying movement:

```ferris
if self.position.x + dir * 100.0 * delta > 10.0 {
    dir = -1.0;
}
// ... then apply movement
```

However, for most use cases, the current code works fine (overshooting by 1-2 pixels is imperceptible).

### 4. Movement Feels Robotic

**Expected!** This example has instant direction changes (no easing or acceleration).

**Improvements** (future variations):

- Add acceleration/deceleration
- Use easing functions (smoothstep, sine wave)
- Add rotation to match direction

### 5. "Cannot assign to immutable variable 'dir'" Error

**Problem**: Compilation fails.

**Solution**: Add `mut` to the variable declaration:

```ferris
let mut dir: f32 = 1.0;  // ✅ Correct (mutable)
let dir: f32 = 1.0;      // ❌ Wrong (immutable)
```

FerrisScript enforces mutability at compile time (like Rust). This prevents accidental modifications.

## Variations to Try

### 1. Vertical Bouncing

```ferris
let mut dir: f32 = 1.0;

fn _process(delta: f32) {
    self.position.y += dir * 100.0 * delta;

    if self.position.y > 300.0 {
        dir = -1.0;
    }
    if self.position.y < -300.0 {
        dir = 1.0;
    }
}
```

Bounces up and down instead of left and right.

### 2. Diagonal Bouncing

```ferris
let mut dir_x: f32 = 1.0;
let mut dir_y: f32 = 1.0;

fn _process(delta: f32) {
    self.position.x += dir_x * 100.0 * delta;
    self.position.y += dir_y * 100.0 * delta;

    if self.position.x > 400.0 { dir_x = -1.0; }
    if self.position.x < -400.0 { dir_x = 1.0; }
    if self.position.y > 300.0 { dir_y = -1.0; }
    if self.position.y < -300.0 { dir_y = 1.0; }
}
```

Bounces like a DVD screensaver (independent X and Y directions).

### 3. Variable Speed

```ferris
let mut dir: f32 = 1.0;
let speed: f32 = 50.0;  // Start slow

fn _process(delta: f32) {
    self.position.x += dir * speed * delta;

    if self.position.x > 10.0 {
        dir = -1.0;
    }
    if self.position.x < -10.0 {
        dir = 1.0;
    }
}
```

Change `speed` to adjust how fast it bounces.

### 4. Count Bounces

```ferris
let mut dir: f32 = 1.0;
let mut bounces: int = 0;

fn _process(delta: f32) {
    self.position.x += dir * 100.0 * delta;

    if self.position.x > 10.0 {
        dir = -1.0;
        bounces += 1;
        print("Bounces:", bounces);
    }
    if self.position.x < -10.0 {
        dir = 1.0;
        bounces += 1;
        print("Bounces:", bounces);
    }
}
```

Prints the total number of bounces.

### 5. Pause After Each Bounce

```ferris
let mut dir: f32 = 1.0;
let mut pause_timer: f32 = 0.0;

fn _process(delta: f32) {
    if pause_timer > 0.0 {
        pause_timer -= delta;
        return;  // Don't move while paused
    }

    self.position.x += dir * 100.0 * delta;

    if self.position.x > 10.0 {
        dir = -1.0;
        pause_timer = 0.5;  // Pause for 0.5 seconds
    }
    if self.position.x < -10.0 {
        dir = 1.0;
        pause_timer = 0.5;
    }
}
```

Pauses for half a second at each boundary.

### 6. Bounce with Rotation

```ferris
let mut dir: f32 = 1.0;

fn _process(delta: f32) {
    self.position.x += dir * 100.0 * delta;
    // TODO: Rotation not yet supported in FerrisScript v0.0.1
    // self.rotation += dir * 2.0 * delta;  // Future feature

    if self.position.x > 10.0 { dir = -1.0; }
    if self.position.x < -10.0 { dir = 1.0; }
}
```

**Note**: Rotation is not yet implemented in FerrisScript v0.0.1. See [v0.1.0-ROADMAP.md](../../docs/v0.1.0-ROADMAP.md) for planned features.

## Real-World Use Cases

This pattern (boundary checks + direction reversal) is useful for:

- **Platform game enemies**: Patrol back and forth on a platform
- **Breakout/Pong**: Ball bouncing off walls
- **UI animations**: Slider moving back and forth
- **Screen savers**: DVD logo, bouncing ball
- **Health bar animations**: Pulsing effect
- **Camera shake**: Oscillate camera position

## Performance Considerations

This example is still **very efficient**:

- 4 arithmetic operations per frame (`+`, `*`, two comparisons)
- 2 conditional branches (CPU branch prediction handles these well)
- No memory allocation

**Comparison to GDScript:**  
Roughly equivalent performance to:

```gdscript
var dir = 1.0

func _process(delta):
    position.x += dir * 100.0 * delta
    if position.x > 10.0:
        dir = -1.0
    if position.x < -10.0:
        dir = 1.0
```

## Next Steps

After understanding this example:

1. **Try the variations** above to explore conditionals and state
2. **Read [ARCHITECTURE.md](../../docs/ARCHITECTURE.md)**: Learn about the language design and how features like global variables work internally
3. **Explore [v0.1.0-ROADMAP.md](../../docs/v0.1.0-ROADMAP.md)**: See what's planned for future releases
4. **Build your own**: Try making a simple game (Pong, Breakout, etc.)

## Questions?

- **GitHub Issues**: [Report bugs or ask questions](https://github.com/dev-parkins/FerrisScript/issues)
- **GitHub Discussions**: [General questions and ideas](https://github.com/dev-parkins/FerrisScript/discussions)
- **Documentation**: [Full documentation](../../docs/)
