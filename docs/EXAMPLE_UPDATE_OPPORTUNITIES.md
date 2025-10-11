# Example Update Opportunities

## Purpose

This document tracks opportunities to update the `examples/` directory as new features are added to FerrisScript. Examples serve as the primary showcase of FerrisScript's capabilities and should be kept current to demonstrate:

1. **New language features** - When adding new syntax or semantics
2. **Enhanced error messages** - When improving developer experience
3. **Runtime capabilities** - When adding new runtime features
4. **Godot integration** - When expanding GDExtension bindings

**Goal**: Maintain examples as a living demonstration of FerrisScript's best capabilities, making it easy for users to learn and evaluate the language.

---

## Current Examples Status (v0.0.2)

### Core Examples

**`examples/hello.ferris`**

- **Status**: ✅ Up-to-date
- **Demonstrates**: Basic function declaration, string literals, `print()` built-in
- **Features Used**: Functions, strings, built-in functions

**`examples/move.ferris`**

- **Status**: ✅ Up-to-date  
- **Demonstrates**: Godot integration, Vector2 type, field access, arithmetic, `_process()` lifecycle
- **Features Used**: Godot types, field access, operators, lifecycle methods

**`examples/bounce.ferris`**

- **Status**: ✅ Up-to-date
- **Demonstrates**: Global mutable state, control flow (if/while), compound assignment, floating-point math
- **Features Used**: Global variables, if statements, while loops, compound assignment (`+=`, `*=`)

**`examples/functions.ferris`**

- **Status**: ✅ Up-to-date
- **Demonstrates**: Function parameters, return types, type annotations, arithmetic
- **Features Used**: Function parameters, return types, type inference

**`examples/branch.ferris`**

- **Status**: ✅ Up-to-date
- **Demonstrates**: Conditional logic, if/else statements, boolean conditions
- **Features Used**: If/else, comparison operators, boolean expressions

**`examples/loop.ferris`**

- **Status**: ✅ Up-to-date
- **Demonstrates**: While loops, mutable state, compound assignment
- **Features Used**: While loops, mutable variables, counter patterns

**`examples/match.ferris`**

- **Status**: ⚠️ Future feature
- **Note**: Match expressions not yet implemented

**`examples/collections.ferris`**

- **Status**: ⚠️ Future feature
- **Note**: Arrays/collections not yet implemented

**`examples/scene.ferris`**

- **Status**: ⚠️ Future feature
- **Note**: Advanced Godot features not yet implemented

**`examples/reload.ferris`**

- **Status**: ⚠️ Future feature
- **Note**: Hot reload capabilities not yet fully implemented

**`examples/type_error.ferris`**

- **Status**: ✅ Up-to-date  
- **Demonstrates**: Type errors (intentional errors for testing)
- **Features Used**: Type mismatches, invalid operations

---

## Phase 3 Enhancement Opportunities (v0.0.2)

### Enhanced Error Messages with Source Context

**Status**: ✅ Completed (PR #13)  
**Feature**: All 38 compiler errors now display:

- ±2 lines of source code context
- Visual pointer (^) at exact error location
- Helpful hints explaining what's expected

**Example Update Opportunities**:

#### 1. Create Interactive Error Demonstration

**File**: `examples/error_showcase.ferris` (NEW)

**Purpose**: Show users how helpful FerrisScript's error messages are

**Content Ideas**:

```ferris
// This example demonstrates FerrisScript's helpful error messages
// Uncomment sections below to see different error types

// --- Type Mismatch Error ---
// fn test_types() {
//     let x: i32 = true;  // Error shows: "Value type bool cannot be coerced to i32"
// }

// --- Undefined Variable Error ---
// fn test_undefined() {
//     let x = undefined_var;  // Error shows: "Variable must be declared before use"
// }

// --- If Condition Error ---
// fn test_condition() {
//     if 5 {  // Error shows: "Condition must evaluate to a boolean value"
//         print("hello");
//     }
// }

// --- Binary Operation Error ---
// fn test_binary() {
//     let x = 5 + true;  // Error shows: "Arithmetic operations require i32 or f32 types"
// }

// Main function that works correctly
fn _ready() {
    print("Uncomment error examples above to see helpful messages!");
}
```

**Benefit**: New users can quickly learn about error message quality without triggering real compilation failures.

#### 2. Add Error Recovery Examples to Documentation

**File**: `examples/README.md` (UPDATE)

**Addition**: Section showing before/after of error messages:

```markdown
## Error Messages

FerrisScript provides helpful error messages with source context:

**Example Error**:
```ferris
fn test() {
    let x: i32 = true;
}
```

**Output**:

```
Type mismatch in let binding 'x': expected i32, found bool at line 2, column 9

 1 | fn test() {
 2 |     let x: i32 = true;
   |              ^ Value type bool cannot be coerced to i32
 3 | }
```

The error shows:

- ± lines of source context
- Visual pointer (^) at error location
- Helpful hint explaining the issue

```

#### 3. Update Example READMEs with Error Examples

**Files**: `examples/hello/README.md`, `examples/move/README.md`, etc.

**Addition**: "Common Mistakes" section showing:
- What happens if you misspell a function name
- What happens if you use the wrong type
- How error messages help you fix it

**Example for `hello/README.md`**:
```markdown
### Common Mistakes

**Typo in function name**:
```ferris
fn _redy() {  // Typo: should be _ready
    print("Hello!");
}
```

FerrisScript will tell you: "No entry point '_ready' function found"

**Wrong type for print()**:

```ferris
fn _ready() {
    print(42);  // Error: print expects string
}
```

Error message shows: "Function 'print' argument 0 has wrong type: expected string, found i32"

```

---

## Future Feature Opportunities (v0.1.0+)

### Arrays/Collections

**When Available**: v0.1.0 (planned)

**Example Updates Needed**:

1. **Update `examples/collections.ferris`** (currently placeholder)
   - Create array with literal syntax
   - Access array elements by index
   - Show array bounds checking errors
   - Demonstrate iteration patterns

2. **Add Array Example to `examples/functions.ferris`**
   - Function that takes array parameter
   - Function that returns array
   - Show type inference for array types

3. **New Example**: `examples/array_bounce.ferris`
   - Track multiple bouncing objects using array
   - Show array indexing in game loop
   - Demonstrate performance with collections

**Error Message Opportunities**:
- Out-of-bounds access errors with helpful context
- Array type mismatch errors (e.g., `let arr: [i32] = [true, false]`)
- Show "Array must be initialized with consistent types" hints

### For Loops

**When Available**: v0.1.0 (planned)

**Example Updates Needed**:

1. **Update `examples/loop.ferris`**
   - Add for loop syntax alongside while loop
   - Show range-based iteration
   - Compare while vs. for loop patterns

2. **Update `examples/bounce.ferris`**
   - Could use for loop to update multiple bouncing objects
   - Show cleaner iteration syntax

3. **New Example**: `examples/iteration.ferris`
   - Demonstrate all iteration patterns (while, for, range)
   - Show break/continue (if implemented)
   - Performance comparison notes

**Error Message Opportunities**:
- "For loop requires iterable expression" hint
- "Range must be numeric types" guidance

### Match Expressions

**When Available**: v0.1.0 (planned)

**Example Updates Needed**:

1. **Update `examples/match.ferris`** (currently placeholder)
   - Pattern matching on integers
   - Pattern matching on enums (if implemented)
   - Show exhaustiveness checking

2. **Update `examples/branch.ferris`**
   - Add match expression example alongside if/else
   - Show when match is more elegant than if chains

3. **New Example**: `examples/state_machine.ferris`
   - Game state machine using match
   - Show transitions between game states
   - Demonstrate Godot integration with match

**Error Message Opportunities**:
- "Match must be exhaustive" with missing patterns shown
- "Match arm pattern type mismatch" with type guidance
- "Unreachable match arm" warnings

### Structs/User-Defined Types

**When Available**: v0.2.0 (future)

**Example Updates Needed**:

1. **New Example**: `examples/struct_basics.ferris`
   - Define struct with fields
   - Create instances
   - Access/modify fields
   - Show struct methods (if implemented)

2. **Update `examples/move.ferris`**
   - Define custom Position/Velocity structs
   - Show struct composition
   - Demonstrate typed game objects

3. **New Example**: `examples/game_entities.ferris`
   - Player struct with health, position, etc.
   - Enemy struct with AI state
   - Show struct-based game architecture

**Error Message Opportunities**:
- "Struct field 'x' does not exist on type 'Player'" with available fields listed
- "Cannot access private field" with visibility hints
- "Missing fields in struct initialization" with required fields shown

### Enums/Sum Types

**When Available**: v0.2.0 (future)

**Example Updates Needed**:

1. **New Example**: `examples/enum_basics.ferris`
   - Define enum with variants
   - Pattern match on enum
   - Show enum with associated data (if supported)

2. **New Example**: `examples/game_state.ferris`
   - GameState enum (Menu, Playing, Paused, GameOver)
   - State machine using match on enum
   - Godot integration with enum states

**Error Message Opportunities**:
- "Enum variant 'Foo' does not exist" with similar variants suggested
- "Pattern match not exhaustive" with missing variants listed

### Closures/Lambdas

**When Available**: v0.3.0 (future)

**Example Updates Needed**:

1. **New Example**: `examples/closures.ferris`
   - Define closure capturing variables
   - Pass closures as function parameters
   - Show closure use in callbacks (if Godot supports)

2. **Update `examples/collections.ferris`**
   - Use closures for map/filter operations
   - Show functional programming patterns

**Error Message Opportunities**:
- "Closure captures mutable variable" ownership hints
- "Cannot move captured variable" borrow checker guidance

### Traits/Interfaces

**When Available**: v0.3.0+ (future)

**Example Updates Needed**:

1. **New Example**: `examples/traits.ferris`
   - Define trait with methods
   - Implement trait for types
   - Show trait bounds in functions

2. **New Example**: `examples/polymorphism.ferris`
   - Different game entities implementing common trait
   - Show dynamic dispatch (if supported)

**Error Message Opportunities**:
- "Type 'Foo' does not implement trait 'Bar'" with missing methods listed
- "Trait method signature mismatch" with expected/actual shown

---

## Maintenance Workflow

### When Adding New Features

1. **Check This Document**: Look for planned example updates related to the feature
2. **Update Examples**: Create or modify examples to showcase the feature
3. **Add Error Demonstrations**: Show helpful error messages for common mistakes
4. **Update READMEs**: Add "Common Mistakes" sections if applicable
5. **Test Examples**: Run examples through compiler to verify they work
6. **Update This Document**: Mark opportunities as completed, add new opportunities if discovered

### When Improving Error Messages

1. **Check Examples**: See if existing examples can demonstrate the improvement
2. **Add Error Showcase**: Consider adding to `examples/error_showcase.ferris`
3. **Update Documentation**: Show before/after in READMEs if significant improvement

### When Adding Runtime Features

1. **Update Godot Examples**: Ensure `move.ferris`, `bounce.ferris` showcase new capabilities
2. **Create Godot-Specific Examples**: Add to `godot_test/scripts/` directory
3. **Document GDExtension Bindings**: Update example READMEs with new API surface

---

## Example Quality Standards

### All Examples Should:

- ✅ Compile without errors (unless demonstrating errors)
- ✅ Include descriptive comments explaining what's happening
- ✅ Follow FerrisScript style guide (when established)
- ✅ Have accompanying README.md (for complex examples)
- ✅ Demonstrate realistic use cases (not just syntax)
- ✅ Work in Godot (for Godot-specific examples)

### Example READMEs Should Include:

- **What It Does**: Brief description of functionality
- **Key Concepts**: FerrisScript features demonstrated
- **How to Run**: Instructions for compiling/running
- **Expected Output**: What the user should see
- **Common Mistakes**: Errors users might make and how to fix them

---

## Tracking Progress

| Feature | Version | Examples Needed | Status | Notes |
|---------|---------|----------------|--------|-------|
| Enhanced Errors | v0.0.2 | error_showcase.ferris, README updates | ⏸️ Optional | Phase 3 complete |
| Arrays | v0.1.0 | collections.ferris, array_bounce.ferris | ⏸️ Planned | Awaiting feature |
| For Loops | v0.1.0 | loop.ferris update, iteration.ferris | ⏸️ Planned | Awaiting feature |
| Match | v0.1.0 | match.ferris, state_machine.ferris | ⏸️ Planned | Awaiting feature |
| Structs | v0.2.0 | struct_basics.ferris, game_entities.ferris | ⏸️ Planned | Future |
| Enums | v0.2.0 | enum_basics.ferris, game_state.ferris | ⏸️ Planned | Future |
| Closures | v0.3.0 | closures.ferris updates | ⏸️ Planned | Future |
| Traits | v0.3.0+ | traits.ferris, polymorphism.ferris | ⏸️ Planned | Future |

---

## Related Documents

- [v0.1.0-ROADMAP.md](./v0.1.0-ROADMAP.md) - Planned features for v0.1.0
- [EDGE_CASE_ERROR_HANDLING_PLAN.md](./EDGE_CASE_ERROR_HANDLING_PLAN.md) - Error message improvements
- [examples/README.md](../examples/README.md) - Current examples overview

---

**Last Updated**: January 2025  
**Version**: v0.0.2  
**Status**: Living document - update as features are added
