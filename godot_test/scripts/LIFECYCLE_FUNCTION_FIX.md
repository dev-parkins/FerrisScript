# Lifecycle Function Fix - Optional Callbacks

## Problem

FerrisScript's Godot bindings were **unconditionally** calling all lifecycle functions (`_ready`, `_process`, `_physics_process`, `_input`, `_enter_tree`, `_exit_tree`) on every frame, even when these functions weren't defined in the FerrisScript file.

### Errors Observed

```
Error[E415]: Undefined function: _physics_process
Error[E415]: Undefined function: _process  
Error[E415]: Undefined function: _input
```

### Additional Issue

Variables in FerrisScript are **immutable by default** (like Rust). Attempting to reassign a variable causes:

```
Error[E400]: Cannot assign to immutable variable 'result'
```

## Root Cause

The `lib.rs` Godot bindings were calling lifecycle functions without checking if they exist:

```rust
fn process(&mut self, delta: f64) {
    if self.script_loaded {
        // ❌ Always tries to call, even if function doesn't exist
        self.call_script_function_with_self("_process", &[delta_value]);
    }
}
```

## Solution

### 1. Check Function Existence Before Calling

Updated all lifecycle callbacks to check if the function exists using `env.get_function()`:

```rust
fn process(&mut self, delta: f64) {
    if self.script_loaded {
        if let Some(env) = &self.env {
            // ✅ Only call if function exists
            if env.get_function("_process").is_some() {
                let delta_value = Value::Float(delta as f32);
                self.call_script_function_with_self("_process", &[delta_value]);
            }
        }
    }
}
```

**Applied to ALL lifecycle functions:**
- `_ready()` - ✅ Now optional!
- `_process(delta: f32)`
- `_physics_process(delta: f32)`
- `_input(event: InputEvent)`
- `_enter_tree()`
- `_exit_tree()`

### 2. Fixed Immutability Issue in Test File

**Before (BROKEN):**
```ferris
let result: i32 = 0;
if y > 40 {
    result = 1;  // ❌ Cannot reassign!
} else {
    result = -1;
}
```

**After (FIXED):**
```ferris
// Test the condition directly instead of storing in variable
assert_test(y > 40);
```

**Why:** FerrisScript variables are immutable (no `mut` keyword exists yet). If expressions aren't supported either, so we simplified the test.

## Files Changed

### `crates/godot_bind/src/lib.rs`
- ✅ Added function existence checks to `ready()` for `_ready()`
- ✅ Added function existence checks to `process()` for `_process()`
- ✅ Added function existence checks to `physics_process()` for `_physics_process()`
- ✅ Added function existence checks to `input()` for `_input()`
- ✅ Added function existence checks to `enter_tree()` for `_enter_tree()`
- ✅ Added function existence checks to `exit_tree()` for `_exit_tree()`

### `godot_test/scripts/v004_phase2_test.ferris`
- ✅ Removed variable reassignment in Test 3
- ✅ Changed to direct condition testing

## Current Status

✅ **Lifecycle functions are now optional** - FerrisScript files can define only the callbacks they need
✅ **No more E415 errors** for undefined lifecycle functions
✅ **Test file compiles successfully** with 4 functions
✅ **DLL rebuilt and copied** to `godot_test/` at 12:59:23

## Testing in Godot

1. **Close Godot completely** (check Task Manager)
2. **Clean cache**: `Remove-Item "godot_test\.godot" -Recurse -Force`
3. **Open Godot** and load the test scene
4. **Expected output**: Only "PASS" messages, no lifecycle errors

## Language Design Notes

### Optional vs Required Lifecycle Functions

**Optional (implement if needed):**
- `_process(delta: f32)` - Called every frame
- `_physics_process(delta: f32)` - Called every physics frame (60Hz)
- `_input(event: InputEvent)` - Called on input events
- `_enter_tree()` - Called when added to scene tree
- `_exit_tree()` - Called when removed from scene tree

**Commonly Used:**
- `_ready()` - Called when node is ready (initialization)

### Variable Immutability

- FerrisScript variables are **immutable by default** (like Rust)
- No `mut` keyword exists yet (Phase 2 limitation)
- Workarounds:
  - Declare separate variables instead of reassigning
  - Test conditions directly instead of storing results
  - Use function return values

### If Expressions Not Supported

FerrisScript currently only supports **if statements**, not **if expressions**:

❌ **Not Supported:**
```ferris
let result = if condition { 1 } else { -1 };
```

✅ **Supported:**
```ferris
let result = 0;
if condition {
    // But can't reassign result here!
}
```

**Future Enhancement:** Consider adding if expressions or mutable variables in a later phase.

## Impact

This fix makes FerrisScript's Godot integration much more flexible:

1. **Minimal Scripts Work** - Simple scripts with just `_ready()` no longer error
2. **Performance Optimization** - Godot won't waste time calling undefined functions
3. **Better Error Messages** - Real errors stand out instead of being buried in lifecycle noise
4. **Cleaner Code** - Scripts only define the callbacks they actually use

## Version

- **Fixed in:** v0.0.4-dev (post-Phase 2)
- **Build:** 2025-10-09 12:59:23
- **Affects:** All FerrisScript Godot integration
