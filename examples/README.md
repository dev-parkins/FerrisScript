# FerrisScript Examples

This directory contains example scripts demonstrating FerrisScript's features and capabilities.

## 🧪 TEST Metadata (v0.0.4+)

**All `.ferris` files now include standardized TEST headers** for headless test runner integration:

```ferris
// TEST: test_name
// CATEGORY: unit|integration
// DESCRIPTION: Brief description
// EXPECT: success|error
// ASSERT: Expected output lines
```

**Benefits**:

- Automated test discovery and validation
- Consistent documentation structure
- Headless testing support
- CI/CD integration ready

**See**: [`docs/testing/TESTING_GUIDE.md`](../docs/testing/TESTING_GUIDE.md) for full testing patterns

## ⭐ Inspector Testing (v0.0.4)

**NEW**: Test files for Inspector integration with `@export` properties!

### ✅ Recommended: Minimal Test (WORKS)

- **Test File**: `inspector_minimal.ferris` - Simple working test with 7 properties
- **Guide**: `INSPECTOR_MINIMAL_TEST_GUIDE.md` - Step-by-step testing instructions
- **Status**: ✅ Compiles successfully, ready for testing

### ⚠️ Comprehensive Test (Has Issues)

- **Test File**: `inspector_test.ferris` - Complete test suite with 20+ properties (in godot_test/scripts/)
- **Status**: ⚠️ Parser issues - use minimal test instead
- **Full Guide**: `INSPECTOR_TEST_GUIDE.md` - Detailed testing instructions
- **Quick Ref**: `INSPECTOR_QUICK_REF.md` - Quick reference

**Get Started**: See [Inspector Minimal Test Guide](INSPECTOR_MINIMAL_TEST_GUIDE.md) for testing instructions.

---

## Quick Start

### Testing Examples (Without Godot)

To verify an example compiles correctly and see error messages:

```bash
# Test any FerrisScript file
cargo run --example test_ferris -- examples/hello.ferris

# See what error messages look like
cargo run --example test_ferris -- examples/error_showcase.ferris

# Or run the test suite for all examples
cargo test --package ferrisscript_compiler test_compile
```

### Running Examples (In Godot)

To actually execute examples, you need to run them in Godot with the FerrisScript GDExtension. See the "Running Examples in Godot" section below for setup instructions.

## Available Examples

### Basic Examples

#### `hello.ferris` - Hello World

**Demonstrates**: Basic function declaration, string literals, `print()` built-in

The classic "Hello, World!" program showing the simplest valid FerrisScript program.

```ferris
fn _ready() {
    print("Hello from FerrisScript!");
}
```

See `hello/README.md` for more details and common mistakes.

---

#### `functions.ferris` - Function Basics

**Demonstrates**: Function parameters, return types, type annotations, arithmetic

Shows how to define functions with parameters and return values.

```ferris
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
```

---

#### `branch.ferris` - Conditional Logic

**Demonstrates**: If/else statements, boolean conditions, comparison operators

Shows how to use conditional statements to control program flow.

---

#### `loop.ferris` - While Loops

**Demonstrates**: While loops, mutable state, compound assignment

Shows how to use while loops for iteration and counter patterns.

---

### Godot Integration Examples

#### `move.ferris` - Basic Movement

**Demonstrates**: Godot lifecycle methods, Vector2, field access, `_process()`

A simple script that moves a Godot node horizontally. Shows how FerrisScript integrates with Godot's game loop.

See `move/README.md` for Godot setup instructions.

---

#### `bounce.ferris` - Bouncing Ball

**Demonstrates**: Global mutable state, control flow, compound assignment, game logic

A bouncing ball simulation showing how to manage game state and respond to boundaries.

See `bounce/README.md` for more details.

---

### Advanced Examples

#### `error_showcase.ferris` - Error Message Demonstration

**Demonstrates**: FerrisScript's helpful error messages with source context

An interactive example showing how FerrisScript provides helpful error messages. Uncomment different sections to see various error types with:

- Source context (±2 lines around the error)
- Visual pointer (^) showing exact error location  
- Helpful hints explaining what's expected

**How to Use:**

1. **See it compile successfully**:

   ```bash
   cargo run --example test_ferris -- examples/error_showcase.ferris
   ```

   Output: `✓ Compilation successful!`

2. **See error messages in action**:
   - Open `error_showcase.ferris`
   - Uncomment any error section (e.g., lines 71-73 for "Type Mismatch Error")
   - Run the test again:

     ```bash
     cargo run --example test_ferris -- examples/error_showcase.ferris
     ```

   - See the error with source context, pointer, and helpful hint!

3. **Learn by reading**: Browse through the commented examples to understand different error types without needing to trigger them.

---

### Future Examples (Placeholders)

These examples demonstrate planned features:

- `collections.ferris` - Arrays and collections (v0.1.0+)
- `match.ferris` - Pattern matching (v0.1.0+)
- `scene.ferris` - Advanced Godot integration
- `reload.ferris` - Hot reload capabilities
- `type_error.ferris` - Type system demonstration

---

## Error Messages

FerrisScript provides helpful error messages with source context to help you fix issues quickly.

### Example Error Output

When you make a mistake, FerrisScript shows you exactly what went wrong:

**Code with Error:**

```ferris
fn test() {
    let x: i32 = true;  // Type mismatch
}
```

**Error Output:**

```
Type mismatch in let binding 'x': expected i32, found bool at line 2, column 18

 1 | fn test() {
 2 |     let x: i32 = true;
   |                  ^ Value type bool cannot be coerced to i32
 3 | }
```

**What the error shows:**

1. **Error Message**: Clear description of what went wrong
2. **Location**: Line 2, column 18
3. **Source Context**: ±2 lines of code around the error
4. **Visual Pointer**: `^` pointing to the exact problem location
5. **Helpful Hint**: "Value type bool cannot be coerced to i32"

### Common Error Types

#### Type Mismatch Errors

```ferris
let x: i32 = "hello";  // Error: Can't assign string to i32
```

The error message will show:

- What type was expected (i32)
- What type was found (string)
- A hint about type coercion rules

---

#### Undefined Variable Errors

```ferris
let x = undefined_var;  // Error: Variable not declared
```

The error message will show:

- Which variable is undefined
- Where you tried to use it
- A hint: "Variable must be declared before use"

---

#### Control Flow Errors

```ferris
if 5 {  // Error: Condition must be bool
    print("test");
}
```

The error message will show:

- That the condition must be a boolean
- What type was found (i32)
- A hint: "Condition must evaluate to a boolean value (true or false)"

---

#### Function Call Errors

```ferris
print(42);  // Error: Wrong argument type
```

The error message will show:

- Function name and argument number
- Expected type (string)
- Found type (i32)
- A hint about the correct argument type

---

### Tips for New Users

1. **Read the Full Error**: Don't just look at the first line - the source context and hints are valuable!

2. **Check the Pointer**: The `^` character points to exactly where the error occurred.

3. **Follow the Hints**: Error hints explain what's expected and often suggest how to fix the issue.

4. **Try error_showcase.ferris**: Uncomment different error sections to learn about common mistakes.

5. **Common Mistakes READMEs**: Check example-specific READMEs (like `hello/README.md`) for common errors in that context.

---

## Example Structure

Each example can have its own directory with additional documentation:

```
examples/
  hello.ferris          - Standalone example
  hello/                - Example with additional docs
    README.md           - Detailed documentation
    common_mistakes.md  - Common errors for this example
```

For complex examples or Godot-specific examples, check the subdirectory for setup instructions and troubleshooting.

---

## Contributing Examples

When adding new examples:

1. **Keep It Simple**: Each example should demonstrate one or two features clearly
2. **Add Comments**: Explain what's happening in the code
3. **Include README**: For complex examples, add a README.md in a subdirectory
4. **Test It**: Ensure the example compiles and runs correctly
5. **Show Errors**: Consider adding a "Common Mistakes" section

See `EXAMPLE_UPDATE_OPPORTUNITIES.md` in the docs/ folder for planned example improvements.

---

## Running Examples in Godot

For Godot examples (move, bounce, etc.), you need to:

1. Open the `godot_test` project in Godot
2. Copy the .ferris script to `godot_test/scripts/`
3. Attach the script to a node in the scene
4. Run the scene in Godot

See individual example READMEs for specific setup instructions.

---

## Related Documentation

- [EXAMPLE_UPDATE_OPPORTUNITIES.md](../docs/EXAMPLE_UPDATE_OPPORTUNITIES.md) - Planned example improvements
- [EDGE_CASE_ERROR_HANDLING_PLAN.md](../docs/archive/v0.0.2/phases/EDGE_CASE_ERROR_HANDLING_PLAN.md) - Error handling system details
- [v0.1.0-ROADMAP.md](../docs/v0.1.0-ROADMAP.md) - Upcoming features that will need examples

---

**Last Updated**: October 2025  
**FerrisScript Version**: v0.0.2
