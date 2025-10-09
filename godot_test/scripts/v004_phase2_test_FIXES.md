# v004_phase2_test.ferris - Fixes Applied

## Issues Found and Fixed

### 1. Function Signatures Didn't Match Calls

**Problem**: Function definitions had no parameters, but were being called with arguments.

#### Fixed: `assert_test`

```ferris
// ❌ Before (wrong)
fn assert_test() {
    if true {  // Always true!
        print("PASS");
    }
}

// Call site was: assert_test(x == 42)  // ERROR: expects 0 args, found 1

// ✅ After (correct)
fn assert_test(cond: bool) {
    if cond {  // Now uses the parameter
        print("PASS");
    } else {
        print("FAIL");
    }
}
```

#### Fixed: `add`

```ferris
// ❌ Before (wrong)
fn add() {
    return 1 + 2;  // Always returns 3!
}

// Call site was: add(5, 7)  // ERROR: expects 0 args, found 2

// ✅ After (correct)
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```

### 2. Empty Lifecycle Functions Removed

**Problem**: Empty `_physics_process()` and `_input()` stubs were causing errors.

```ferris
// ❌ Removed (causing errors)
fn _physics_process() {  // ERROR: requires 1 parameter
    
}

fn _input(event: InputEvent) {  // Empty function not needed
    
}
```

**Solution**: Removed these empty functions. Only define lifecycle callbacks if you actually use them.

### 3. Fixed `_ready()` Implementation

```ferris
// ❌ Before (empty)
fn _ready() {
    
}

// ✅ After (calls tests)
fn _ready() {
    run_tests();
}
```

## Current File Structure

```ferris
// Helper function for test assertions
fn assert_test(cond: bool) { ... }

// Main test suite
fn run_tests() {
    // Test 1-7 here
}

// Helper function for Test 5
fn add(a: i32, b: i32) -> i32 { ... }

// Godot lifecycle callback - entry point
fn _ready() {
    run_tests();
}
```

## About `-> void` Return Type

**You mentioned**: "`-> void` not working"

**Explanation**: FerrisScript doesn't support explicit `-> void` syntax. Instead:

- ✅ **Implicit void**: Functions without return type are automatically `void`

  ```ferris
  fn assert_test(cond: bool) {  // Implicitly returns void
      print("test");
  }
  ```

- ✅ **Explicit return type**: Use `-> i32`, `-> f32`, `-> bool`

  ```ferris
  fn add(a: i32, b: i32) -> i32 {  // Explicitly returns i32
      return a + b;
  }
  ```

- ❌ **Cannot write `-> void`**: This is not recognized

  ```ferris
  fn test() -> void {  // ERROR: Unknown type 'void'
      print("test");
  }
  ```

This matches how Rust and many other languages work - void is implicit for functions without a return type.

## Supported Types

Currently, FerrisScript supports these types:

- `i32` - 32-bit integer
- `f32` - 32-bit float
- `bool` - Boolean
- `String` - String (limited support)
- `Vector2` - Godot Vector2
- `Node` - Godot Node
- `InputEvent` - Godot InputEvent

**Not supported yet**: `str` (string literals are treated as dynamic)

## Lifecycle Function Signatures

If you define these lifecycle functions, they **must** have the correct signature:

```ferris
// ✅ Correct lifecycle signatures
fn _ready() {
    // No parameters, called once when node enters scene tree
}

fn _process(delta: f32) {
    // Called every frame, delta = time since last frame
}

fn _physics_process(delta: f32) {
    // Called at fixed timestep (60 FPS by default)
}

fn _input(event: InputEvent) {
    // Called for every input event
}

fn _enter_tree() {
    // Called when node enters scene tree
}

fn _exit_tree() {
    // Called when node exits scene tree
}
```

**Important**: Only define lifecycle functions if you actually use them. Empty functions are not needed.

## Testing in Godot

1. Make sure Godot is completely closed
2. Delete `.godot/` folder if it exists
3. Open Godot and load `godot_test/project.godot`
4. Add a FerrisScriptNode to your scene
5. Set Script Path: `res://scripts/v004_phase2_test.ferris`
6. Run the scene (F5)
7. Check Output panel - you should see:

   ```
   Test 1: Variable Assignment and Retrieval
   PASS
   Test 2: Arithmetic Operations
   PASS
   ...
   All v0.0.4 Phase 2 tests completed.
   ```

## Status

✅ All syntax errors fixed
✅ File compiles successfully (4 functions)
✅ Ready to test in Godot
