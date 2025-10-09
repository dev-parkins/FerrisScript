# FerrisScript Immutability Limitation

## Critical Issue: No Mutable Variables Yet

**FerrisScript v0.0.4-dev does NOT support mutable variables.**

### What This Means

All variables are **immutable by default** (like Rust's `let` without `mut`):

```ferris
let x: i32 = 10;
x = 20;  // ❌ Error[E400]: Cannot assign to immutable variable 'x'
```

### Impact on Loops

**While loops are essentially unusable** without mutable variables:

```ferris
// ❌ BROKEN - Cannot update counter
let i: i32 = 0;
while i < 5 {
    print("Iteration");
    i = i + 1;  // ERROR: Cannot assign to immutable variable 'i'
}

// ❌ BROKEN - Cannot accumulate sum
let sum: i32 = 0;
while sum < 100 {
    sum = sum + 10;  // ERROR: Cannot assign to immutable variable 'sum'
}
```

### Current Workarounds

1. **Avoid loops that need counters**
2. **Use fixed-iteration patterns** (if possible)
3. **Skip loop tests entirely** (as we did in v004_phase2_test.ferris)

### What Works

✅ **Immutable variable declarations:**
```ferris
let x: i32 = 42;
let y: i32 = x + 8;
let name: String = "Ferris";
```

✅ **Function parameters (effectively immutable):**
```ferris
fn add(a: i32, b: i32) -> i32 {
    return a + b;  // Can't modify a or b
}
```

✅ **If statements (but not if expressions):**
```ferris
if condition {
    print("True branch");
} else {
    print("False branch");
}
```

✅ **Simple while loops (no counter updates):**
```ferris
// This works, but runs forever!
while true {
    print("Loop");
}
```

### What Doesn't Work

❌ **Variable reassignment:**
```ferris
let x = 10;
x = 20;  // ERROR
```

❌ **Counter-based loops:**
```ferris
let i = 0;
while i < 10 {
    i = i + 1;  // ERROR
}
```

❌ **Accumulator patterns:**
```ferris
let sum = 0;
sum = sum + value;  // ERROR
```

❌ **If expressions:**
```ferris
let result = if x > 0 { 1 } else { -1 };  // ERROR: Expected statement
```

## Test File Adjustments

### v004_phase2_test.ferris - Test 4 Skipped

**Original (BROKEN):**
```ferris
// Test 4: Loop Execution
print("Test 4: Loop Execution");
let sum: i32 = 0;
let i: i32 = 0;
while i < 5 {
    sum = sum + i;  // ❌ Cannot reassign
    i = i + 1;      // ❌ Cannot reassign
}
assert_test(sum == 10);
```

**Fixed (SKIPPED):**
```ferris
// Test 4: Loop Execution
print("Test 4: Loop Execution (skipped - requires mutable variables)");
// Note: While loops require mutable variables to update counters
// This feature will be added in a future phase
// Expected: sum of 0+1+2+3+4 = 10
```

### test_blank_line.ferris - Added _ready()

**Original (BROKEN):**
```ferris
fn test() {
    print("hello");
}
// ❌ No _ready() function - Godot won't call anything
```

**Fixed (WORKING):**
```ferris
fn test() {
    print("hello");
}

fn _ready() {
    test();  // ✅ Godot calls this automatically
}
```

## Future Enhancement

### Phase 3 or Later: Add Mutable Variables

**Option 1: Rust-style `mut` keyword**
```ferris
let mut i: i32 = 0;  // Mutable variable
while i < 5 {
    i = i + 1;  // ✅ Now allowed
}
```

**Option 2: Different keyword**
```ferris
var i: i32 = 0;  // JavaScript/TypeScript style
while i < 5 {
    i = i + 1;
}
```

**Option 3: All variables mutable by default**
```ferris
let i: i32 = 0;  // Mutable by default (like C/JavaScript)
while i < 5 {
    i = i + 1;
}
```

### Required Changes

1. **Lexer:** Add `mut` or `var` token (if using keywords)
2. **Parser:** Parse mutability modifier in variable declarations
3. **AST:** Add `is_mutable: bool` field to `VarDecl`
4. **Type Checker:** Track mutability in environment
5. **Runtime:** Allow reassignment for mutable variables only
6. **Error Messages:** Update E400 to suggest using `mut`

## Current Status

- ✅ v004_phase2_test.ferris: Test 4 skipped, 6 tests run
- ✅ test_blank_line.ferris: Added `_ready()` function
- ✅ Both files compile successfully
- ⚠️ **While loops with counters WILL NOT WORK** until mutability is added

## Recommendation

**For v0.0.4 release:** Document this limitation clearly in README and examples.

**For v0.1.0 planning:** Prioritize mutable variables as a core language feature.

## Related Files

- `v004_phase2_test.ferris` - Test suite with loop test skipped
- `test_blank_line.ferris` - Simple test file with `_ready()` added
- `LIFECYCLE_FUNCTION_FIX.md` - Optional lifecycle functions documentation
- `v004_phase2_test_FIXES.md` - Previous iteration fixes

## Build Info

- **Date:** 2025-10-09
- **Version:** v0.0.4-dev (post-Phase 2)
- **Status:** Language limitation, not a bug
