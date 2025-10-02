# FerrisScript Architecture

This document provides a comprehensive overview of FerrisScript's architecture, design decisions, and implementation details. It's intended for contributors who want to understand how the language works internally and where to make changes.

## Table of Contents

1. [System Overview](#system-overview)
2. [Project Structure](#project-structure)
3. [Compiler Pipeline](#compiler-pipeline)
4. [Runtime Execution](#runtime-execution)
5. [Godot Integration](#godot-integration)
6. [Design Decisions](#design-decisions)
7. [Extension Points](#extension-points)
8. [Performance Considerations](#performance-considerations)

---

## System Overview

FerrisScript is a scripting language designed for use with the Godot game engine. It provides a Rust-like syntax with strong type checking, compiles to an abstract syntax tree (AST), and executes via a tree-walking interpreter.

### High-Level Architecture

```
┌─────────────────┐
│  .ferris files  │  User writes game scripts
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│     Lexer       │  Source code → Tokens
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│     Parser      │  Tokens → Abstract Syntax Tree (AST)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Type Checker   │  Validates types and semantics
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│    Runtime      │  Tree-walking interpreter executes AST
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Godot Bindings  │  GDExtension connects to Godot nodes
└─────────────────┘
```

### Key Components

- **Compiler** (`crates/compiler`): Lexer, parser, type checker, AST definitions
- **Runtime** (`crates/runtime`): Tree-walking interpreter, variable scoping, builtin functions
- **Godot Bindings** (`crates/godot_bind`): GDExtension integration, node property access, lifecycle hooks

---

## Project Structure

The project is organized as a Rust workspace with three main crates:

```
FerrisScript/
├── crates/
│   ├── compiler/          # Compilation pipeline
│   │   ├── src/
│   │   │   ├── lexer.rs       # Tokenization
│   │   │   ├── parser.rs      # Recursive descent parser
│   │   │   ├── type_checker.rs # Type checking and validation
│   │   │   ├── ast.rs         # AST node definitions
│   │   │   └── lib.rs         # Public API (compile function)
│   │   └── Cargo.toml
│   │
│   ├── runtime/           # Interpreter
│   │   ├── src/
│   │   │   └── lib.rs         # Environment, value types, execution
│   │   └── Cargo.toml
│   │
│   └── godot_bind/        # Godot GDExtension
│       ├── src/
│       │   └── lib.rs         # FerrisScriptNode, Godot callbacks
│       └── Cargo.toml
│
├── examples/              # Example .ferris scripts
│   ├── hello.ferris
│   ├── move.ferris
│   └── bounce.ferris
│
├── docs/                  # Documentation
├── Cargo.toml            # Workspace configuration
└── README.md
```

### Crate Dependencies

```
godot_bind
    ├── depends on: ferrisscript_compiler
    ├── depends on: ferrisscript_runtime
    └── depends on: godot (GDExtension bindings)

runtime
    └── depends on: ferrisscript_compiler (AST types)

compiler
    └── (no internal dependencies)
```

---

## Compiler Pipeline

The compiler transforms FerrisScript source code into a validated AST through three stages:

### 1. Lexer (Tokenization)

**File**: `crates/compiler/src/lexer.rs`

The lexer scans the source code character-by-character and produces a stream of tokens.

#### Token Types

- **Keywords**: `fn`, `let`, `mut`, `if`, `else`, `while`, `return`, `true`, `false`
- **Literals**: Identifiers (`foo`), Numbers (`42`, `3.14`), Strings (`"hello"`)
- **Delimiters**: `(`, `)`, `{`, `}`, `,`, `;`, `.`, `:`
- **Operators**: `+`, `-`, `*`, `/`, `=`, `==`, `!=`, `<`, `<=`, `>`, `>=`, `&&`, `||`, `!`, `+=`, `-=`

#### Example

```rust
// Input:
fn hello() {
    print("Hello, world!");
}

// Output (tokens):
[Fn, Ident("hello"), LParen, RParen, LBrace,
 Ident("print"), LParen, StringLit("Hello, world!"), RParen, Semicolon,
 RBrace, Eof]
```

#### Implementation Details

- **Multi-character operators**: `==`, `!=`, `<=`, `>=`, `&&`, `||`, `+=`, `-=` are recognized using lookahead
- **String literals**: Support escape sequences (`\n`, `\t`, `\"`, `\\`)
- **Comments**: Single-line (`//`) and block (`/* */`) comments are skipped
- **Error handling**: Reports line and column numbers for unexpected characters

### 2. Parser (Syntax Analysis)

**File**: `crates/compiler/src/parser.rs`

The parser uses **recursive descent** to build an AST from the token stream.

#### Grammar (Simplified)

```
Program     → (GlobalVar | Function)*
GlobalVar   → 'let' 'mut'? Ident (':' Type)? '=' Expr ';'
Function    → 'fn' Ident '(' Params? ')' ('->' Type)? Block
Params      → Param (',' Param)*
Param       → Ident ':' Type

Stmt        → LetStmt | AssignStmt | ReturnStmt | WhileStmt | IfStmt | ExprStmt
LetStmt     → 'let' 'mut'? Ident (':' Type)? '=' Expr ';'
AssignStmt  → Expr ('+=' | '-=' | '=') Expr ';'
ReturnStmt  → 'return' Expr? ';'
WhileStmt   → 'while' Expr Block
IfStmt      → 'if' Expr Block ('else' (IfStmt | Block))?
ExprStmt    → Expr ';'

Expr        → LogicalOr
LogicalOr   → LogicalAnd ('||' LogicalAnd)*
LogicalAnd  → Equality ('&&' Equality)*
Equality    → Comparison (('==' | '!=') Comparison)*
Comparison  → Term (('<' | '<=' | '>' | '>=') Term)*
Term        → Factor (('+' | '-') Factor)*
Factor      → Unary (('*' | '/') Unary)*
Unary       → ('!' | '-') Unary | Call
Call        → Primary ('(' Args? ')' | '.' Ident)*
Primary     → Number | String | 'true' | 'false' | 'self' | Ident | '(' Expr ')'
```

#### AST Nodes

**File**: `crates/compiler/src/ast.rs`

Key AST node types:

- **Program**: Contains global variables and functions
- **Function**: Name, parameters, return type, body (statements)
- **Stmt**: Let, Assign, Return, While, If, Expression statements
- **Expr**: Binary, Unary, Call, Property access, Literals, Variables
- **Span**: Source location (line, column) for error reporting

Example AST structure:

```rust
Program {
    global_vars: [],
    functions: [
        Function {
            name: "hello",
            params: [],
            return_type: None,
            body: [
                ExprStmt(
                    Call {
                        callee: Ident("print"),
                        args: [StringLit("Hello, world!")]
                    }
                )
            ]
        }
    ]
}
```

#### Error Recovery

The parser **does not** attempt error recovery. On the first parse error, it returns immediately with an error message including:
- Expected token vs. actual token
- Line and column number

This is sufficient for game scripting where scripts are small and errors are fixed immediately.

### 3. Type Checker (Semantic Analysis)

**File**: `crates/compiler/src/type_checker.rs`

The type checker validates the AST before execution.

#### Checks Performed

1. **Function existence**: All function calls reference defined functions or builtins
2. **Parameter count**: Function calls have the correct number of arguments
3. **Return type consistency**: Return statements match function return types
4. **Variable existence**: All variable references are declared
5. **Mutability**: Variables are only reassigned if declared `mut`
6. **Self usage**: `self` is only used inside functions (implies node context)
7. **Type compatibility**: Basic type checking for assignments and operators

#### Type System

FerrisScript has a **gradual type system**:
- Type annotations are **optional** (e.g., `let x = 5` or `let x: int = 5`)
- Type inference is **limited** (mainly for literals)
- Runtime type coercion (e.g., `int` to `float` for arithmetic)

Supported types:
- **Primitives**: `int`, `float`, `bool`, `string`
- **Godot types**: `Vector2`
- **Special**: `nil` (unit type)

Example type checking:

```rust
// Valid
fn add(a: int, b: int) -> int {
    return a + b;
}

// Error: wrong parameter count
add(1, 2, 3);  // Type checker error: Expected 2 arguments, found 3

// Error: return type mismatch
fn get_name() -> string {
    return 42;  // Type checker error: Expected string, got int
}
```

---

## Runtime Execution

**File**: `crates/runtime/src/lib.rs`

The runtime is a **tree-walking interpreter** that directly executes the AST.

### Value Types

```rust
pub enum Value {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Vector2 { x: f32, y: f32 },
    Nil,
    SelfObject,  // Represents the Godot node (self)
}
```

### Environment (Scope Management)

The `Env` struct manages:
- **Variable scopes**: Stack of hashmaps for lexical scoping
- **Functions**: Global function registry (name → AST `Function`)
- **Builtin functions**: Native Rust functions (e.g., `print`, `sqrt`)
- **Property callbacks**: Get/set Godot node properties (`self.position`)

#### Scope Lifecycle

```rust
// Example: while loop with local variable

let global = 10;  // Scope 0 (global)

fn process() {
    let x = 5;  // Scope 1 (function)
    
    while x > 0 {
        let temp = x * 2;  // Scope 2 (while block)
        x -= 1;
    }
    // Scope 2 popped, temp no longer accessible
}
// Scope 1 popped, x no longer accessible
```

### Statement Execution

The `execute` function evaluates statements:

1. **Let**: Declares a variable in the current scope
2. **Assign**: Updates an existing variable (checks mutability)
3. **Return**: Sets return value and exits function
4. **While**: Loops while condition is true (pushes new scope per iteration)
5. **If/Else**: Conditionally executes blocks
6. **Expression**: Evaluates expression and discards result

### Expression Evaluation

The `eval_expr` function evaluates expressions recursively:

- **Literals**: Return immediate values
- **Variables**: Look up in scope stack (innermost to outermost)
- **Binary ops**: Evaluate left and right, apply operator
  - Arithmetic: `+`, `-`, `*`, `/` (with int/float coercion)
  - Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
  - Logical: `&&`, `||` (short-circuit evaluation)
- **Unary ops**: `!` (logical not), `-` (negation)
- **Function calls**: Look up function, push new scope, execute body
- **Property access**: `self.property` → callback to Godot

### Builtin Functions

Registered in `Env::new()`:

- **print**: Outputs to console (or Godot console in GDExtension)
- **sqrt**: Square root
- **sin/cos**: Trigonometric functions
- **abs**: Absolute value
- **Vector2**: Constructor for `Vector2 { x, y }`

Example builtin function signature:

```rust
fn builtin_print(args: &[Value]) -> Result<Value, String> {
    // Print all arguments separated by spaces
    // Return Value::Nil
}
```

---

## Godot Integration

**File**: `crates/godot_bind/src/lib.rs`

FerrisScript integrates with Godot via **GDExtension** (Godot 4's native extension system).

### GDExtension Architecture

```
Godot Engine
    │
    ├── Loads .gdextension file (metadata)
    │
    ├── Loads native .dll/.so/.dylib (Rust compiled)
    │
    └── Registers GDExtension classes
            │
            └── FerrisScriptNode (Node2D subclass)
```

### FerrisScriptNode Class

```rust
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct FerrisScriptNode {
    base: Base<Node2D>,
    
    #[export(file = "*.ferris")]
    script_path: GString,  // Path to .ferris file (e.g., "res://scripts/hello.ferris")
    
    env: Option<Env>,      // Runtime environment
    program: Option<ast::Program>,  // Compiled AST
    script_loaded: bool,
}
```

### Lifecycle Hooks

#### `_ready()`

Called when node enters the scene tree:

1. Load `.ferris` file from `script_path`
2. Compile to AST (`compile(source)`)
3. Initialize runtime environment
4. Register Godot-specific property callbacks
5. Call `_ready()` function in script (if defined)

#### `_process(delta: f64)`

Called every frame:

1. Set thread-local node properties (position, velocity, etc.)
2. Call `_process(delta)` function in script (if defined)
3. Retrieve updated properties from thread-local storage
4. Apply changes to Godot node

### Property Binding

**Challenge**: FerrisScript runtime needs to access/modify Godot node properties.

**Solution**: **Thread-local storage** + **callbacks**

```rust
thread_local! {
    static NODE_POSITION: RefCell<Option<Vector2>> = const { RefCell::new(None) };
}

// Before calling script function:
NODE_POSITION.with(|pos| *pos.borrow_mut() = Some(node.get_position()));

// Inside script:
self.position.x += 10.0;  // Modifies thread-local storage

// After script function returns:
let new_pos = NODE_POSITION.with(|pos| pos.borrow().unwrap());
node.set_position(new_pos);
```

**Why thread-local?**
- Godot nodes are not `Send + Sync` (cannot cross thread boundaries)
- Thread-local storage avoids lifetime issues with `&mut self`

### Supported Properties

Currently supported `self` properties:

- `self.position` (Vector2): Node position in 2D space

**To add more properties**, see [Extension Points](#extension-points).

---

## Design Decisions

### Why a Tree-Walking Interpreter?

**Alternatives considered:**
1. **Bytecode VM**: Compile AST → bytecode → execute
2. **JIT compilation**: Compile to machine code at runtime
3. **Tree-walking**: Directly execute AST

**Decision: Tree-walking**

**Reasons:**
- **Simplicity**: Minimal code, easier to debug and extend
- **Development speed**: Get language features working quickly
- **Small scripts**: Game scripts are typically <500 lines, so performance isn't critical
- **Flexibility**: Easy to add new AST nodes and operators

**Trade-offs:**
- **Performance**: Slower than bytecode VM (not a concern for game scripts)
- **Memory**: AST is kept in memory (acceptable for small scripts)

**Future**: Could add bytecode VM or JIT in v1.0 if performance becomes an issue.

### Why GDExtension (Not GDScript Integration)?

**Alternatives considered:**
1. **Transpile to GDScript**: Compile `.ferris` → `.gd` files
2. **GDExtension**: Native Rust extension
3. **Standalone VM**: External process communicating via IPC

**Decision: GDExtension**

**Reasons:**
- **Performance**: Native code runs faster than GDScript
- **Type safety**: Rust's type system prevents many bugs
- **Ecosystem**: Leverage Rust crates (e.g., for future language features)
- **Direct access**: Can access Godot's C++ API directly

**Trade-offs:**
- **Compilation required**: Users must compile the extension (provides pre-built binaries)
- **Platform-specific**: Separate builds for Windows/Linux/macOS
- **Complexity**: More complex than pure GDScript

### Why No Garbage Collection?

**Decision: Static scoping + no heap allocation**

FerrisScript currently has **no dynamic memory allocation** in scripts:
- All values are stack-allocated or in the `Env` hashmaps
- Scopes are popped when blocks end (automatic cleanup)
- No closures or first-class functions (would require heap allocation)

**Future**: If we add closures, we'd need:
- Reference counting (like Rust's `Rc`)
- Tracing GC (like Python)
- Ownership system (like Rust)

Currently not needed for game scripting use cases.

### Why Rust-Like Syntax?

**Alternatives considered:**
1. **Python-like**: Indentation-based, dynamic typing
2. **C-like**: Curly braces, semicolons
3. **Lua-like**: Minimal syntax, `end` keywords

**Decision: Rust-like**

**Reasons:**
- **Familiarity**: Rust syntax is popular and well-documented
- **Type annotations**: Optional types (`let x: int = 5`) fit game scripting
- **Clear intent**: `mut` keyword makes mutability explicit
- **Consistency**: Matches Godot's GDScript `var` vs. `const` distinction

**Trade-offs:**
- **Learning curve**: Users must learn Rust syntax (mitigated by examples)
- **Verbosity**: More keywords than Python or Lua

---

## Extension Points

This section explains how to extend FerrisScript with new features.

### Adding a New Operator

**Example: Add `%` (modulo) operator**

1. **Add token** (`lexer.rs`):
   ```rust
   pub enum Token {
       // ... existing tokens ...
       Percent,  // %
   }
   
   // In tokenize():
   '%' => tokens.push(Token::Percent),
   ```

2. **Add AST node** (`ast.rs`):
   ```rust
   pub enum BinaryOp {
       // ... existing ops ...
       Modulo,
   }
   ```

3. **Add parsing** (`parser.rs`):
   ```rust
   fn parse_factor(&mut self) -> Result<Expr, String> {
       // ... existing code ...
       while matches!(self.current(), Token::Star | Token::Slash | Token::Percent) {
           let op = match self.advance() {
               Token::Star => BinaryOp::Multiply,
               Token::Slash => BinaryOp::Divide,
               Token::Percent => BinaryOp::Modulo,
               _ => unreachable!(),
           };
           // ... rest of parsing ...
       }
   }
   ```

4. **Add evaluation** (`runtime/lib.rs`):
   ```rust
   BinaryOp::Modulo => {
       let left_int = left.to_int().ok_or("Modulo requires int")?;
       let right_int = right.to_int().ok_or("Modulo requires int")?;
       Value::Int(left_int % right_int)
   }
   ```

5. **Add type checking** (`type_checker.rs`):
   ```rust
   BinaryOp::Modulo => {
       check_int_operands(left, right)?;
       Ok(Type::Int)
   }
   ```

6. **Add tests** (`compiler/lib.rs`, `runtime/lib.rs`):
   ```rust
   #[test]
   fn test_modulo() {
       let source = "fn main() { return 10 % 3; }";
       let program = compile(source).unwrap();
       let result = call_function(&mut env, "main", &[]).unwrap();
       assert_eq!(result, Value::Int(1));
   }
   ```

### Adding a New Builtin Function

**Example: Add `floor(x: float) -> int` function**

1. **Implement the function** (`runtime/lib.rs`):
   ```rust
   fn builtin_floor(args: &[Value]) -> Result<Value, String> {
       if args.len() != 1 {
           return Err("floor expects 1 argument".to_string());
       }
       let val = args[0].to_float()
           .ok_or("floor expects a number")?;
       Ok(Value::Int(val.floor() as i32))
   }
   ```

2. **Register in Env** (`runtime/lib.rs`, in `Env::new()`):
   ```rust
   env.builtin_fns.insert("floor".to_string(), builtin_floor);
   ```

3. **Add tests**:
   ```rust
   #[test]
   fn test_floor() {
       let source = "fn main() { return floor(3.7); }";
       let program = compile(source).unwrap();
       let result = call_function(&mut env, "main", &[]).unwrap();
       assert_eq!(result, Value::Int(3));
   }
   ```

### Adding a New Type

**Example: Add `Color { r, g, b, a }` type**

1. **Add to Value enum** (`runtime/lib.rs`):
   ```rust
   pub enum Value {
       // ... existing variants ...
       Color { r: f32, g: f32, b: f32, a: f32 },
   }
   ```

2. **Add type name** (`type_checker.rs`):
   ```rust
   pub enum Type {
       // ... existing types ...
       Color,
   }
   ```

3. **Add constructor builtin**:
   ```rust
   fn builtin_color(args: &[Value]) -> Result<Value, String> {
       if args.len() != 4 {
           return Err("Color expects 4 arguments".to_string());
       }
       let r = args[0].to_float().ok_or("Color expects numbers")?;
       let g = args[1].to_float().ok_or("Color expects numbers")?;
       let b = args[2].to_float().ok_or("Color expects numbers")?;
       let a = args[3].to_float().ok_or("Color expects numbers")?;
       Ok(Value::Color { r, g, b, a })
   }
   ```

4. **Add Godot conversion** (`godot_bind/lib.rs`):
   ```rust
   // In property getter/setter:
   "modulate" => {
       if let Value::Color { r, g, b, a } = value {
           let godot_color = godot::prelude::Color::from_rgba(r, g, b, a);
           // Set on Godot node
       }
   }
   ```

### Adding a New Godot Property

**Example: Add `self.rotation` (float) property**

1. **Add thread-local storage** (`godot_bind/lib.rs`):
   ```rust
   thread_local! {
       static NODE_ROTATION: RefCell<Option<f32>> = const { RefCell::new(None) };
   }
   ```

2. **Add to property getter**:
   ```rust
   fn get_node_property_tls(property_name: &str) -> Result<Value, String> {
       match property_name {
           // ... existing properties ...
           "rotation" => {
               NODE_ROTATION.with(|rot| {
                   rot.borrow().map(|r| Value::Float(r))
                       .ok_or_else(|| "Node rotation not available".to_string())
               })
           }
           _ => Err(format!("Property '{}' not supported", property_name)),
       }
   }
   ```

3. **Add to property setter**:
   ```rust
   fn set_node_property_tls(property_name: &str, value: Value) -> Result<(), String> {
       match property_name {
           // ... existing properties ...
           "rotation" => {
               if let Value::Float(r) = value {
                   NODE_ROTATION.with(|rot| *rot.borrow_mut() = Some(r));
                   Ok(())
               } else {
                   Err(format!("Expected float for rotation, got {:?}", value))
               }
           }
           _ => Err(format!("Property '{}' not supported", property_name)),
       }
   }
   ```

4. **Update `_process` hook** (`godot_bind/lib.rs`):
   ```rust
   // Before calling script function:
   NODE_ROTATION.with(|rot| *rot.borrow_mut() = Some(self.base().get_rotation()));
   
   // After script function:
   if let Some(new_rot) = NODE_ROTATION.with(|rot| *rot.borrow()) {
       self.base_mut().set_rotation(new_rot);
   }
   ```

---

## Performance Considerations

### Current Performance Characteristics

**Strengths:**
- Zero-cost FFI between Rust and Godot (GDExtension)
- No dynamic memory allocation in scripts (stack-only)
- Short-circuit evaluation for logical operators

**Bottlenecks:**
- Tree-walking: Each AST node traversal has function call overhead
- HashMap lookups: Variable resolution walks scope stack
- String cloning: Variable names and string literals are cloned

### Optimization Opportunities

1. **Bytecode VM**:
   - Compile AST → bytecode (array of instructions)
   - Faster than tree-walking (no recursive calls)
   - Smaller memory footprint

2. **Register allocation**:
   - Assign variables to fixed slots in an array
   - Avoid HashMap lookups for local variables

3. **Inline caching**:
   - Cache property access paths (`self.position.x`)
   - Avoid repeated HashMap lookups

4. **JIT compilation**:
   - Compile hot functions to native code
   - Use LLVM or Cranelift as backend

5. **Parallel execution**:
   - Run multiple scripts concurrently (one per thread)
   - Requires thread-safe Value type (Arc instead of Rc)

### Benchmarking

Currently no benchmarks. To add:

1. Create `benches/` directory
2. Use `criterion.rs` for micro-benchmarks
3. Compare against GDScript for equivalent scripts

**Suggested benchmarks:**
- Fibonacci (recursion performance)
- Matrix multiply (loops and arithmetic)
- Pathfinding (nested data structures)

---

## Testing Strategy

### Unit Tests

Each crate has unit tests:

- **Compiler**: Test lexer, parser, type checker independently
- **Runtime**: Test expression evaluation, statement execution
- **Godot bind**: Test property access, lifecycle hooks (harder, requires Godot)

**Example test** (`compiler/src/lib.rs`):
```rust
#[test]
fn test_compile_hello() {
    let source = std::fs::read_to_string("examples/hello.ferris").unwrap();
    assert!(compile(&source).is_ok());
}
```

### Integration Tests

Located in `examples/`:
- `hello.ferris`: Basic function call
- `move.ferris`: Variable mutation, arithmetic
- `bounce.ferris`: Conditionals, property access

Each example has a corresponding test in `compiler/lib.rs` that compiles the script.

### Manual Testing

To test with Godot:

1. Build GDExtension: `cargo build --release`
2. Copy `.dll`/`.so`/`.dylib` to Godot project
3. Create scene with `FerrisScriptNode`
4. Set `script_path` to `.ferris` file
5. Run scene and observe behavior

---

## Common Development Tasks

### Adding a New Language Feature

1. **Define syntax**: Write example `.ferris` code
2. **Add token(s)**: Update `lexer.rs`
3. **Add AST node(s)**: Update `ast.rs`
4. **Add parsing**: Update `parser.rs`
5. **Add type checking**: Update `type_checker.rs`
6. **Add execution**: Update `runtime/lib.rs`
7. **Add tests**: Unit tests + example script
8. **Update docs**: Update README, LANGUAGE_REFERENCE.md

### Debugging a Script

1. **Check compilation**: Run `cargo test` to compile example
2. **Add print statements**: Liberally use `print()` in script
3. **Check Godot console**: Errors appear in Godot's output panel
4. **Use Rust debugger**: Attach to Godot process, set breakpoints in `runtime/lib.rs`

### Profiling Performance

1. **Use `perf` (Linux)**: `perf record -g` + `perf report`
2. **Use Instruments (macOS)**: Time Profiler
3. **Use Visual Studio Profiler (Windows)**
4. **Rust profiling**: `cargo flamegraph` (requires `cargo-flamegraph`)

---

## Future Directions

### Short-term (v0.1)

- [ ] Add more builtin functions (min, max, clamp, lerp)
- [ ] Support more Godot types (Color, Rect2, Transform2D)
- [ ] Add arrays and dictionaries
- [ ] Improve error messages (show source code context)

### Medium-term (v0.5)

- [ ] Bytecode VM (performance)
- [ ] First-class functions / lambdas
- [ ] Structs and methods
- [ ] Pattern matching

### Long-term (v1.0+)

- [ ] JIT compilation (LLVM backend)
- [ ] Async/await (coroutines for Godot animations)
- [ ] Hot reloading (edit scripts while game is running)
- [ ] Language server protocol (IDE support)

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to contribute to FerrisScript. When making architectural changes:

1. **Discuss in an issue first**: Avoid wasted effort on rejected designs
2. **Update this document**: Keep ARCHITECTURE.md in sync with code
3. **Add tests**: All new features need tests
4. **Run benchmarks**: Ensure no performance regressions (when benchmarks exist)

---

## Questions?

For questions about the architecture:
- **GitHub Discussions**: https://github.com/dev-parkins/FerrisScript/discussions
- **Issue tracker**: https://github.com/dev-parkins/FerrisScript/issues

For questions about Godot integration, see:
- [Godot GDExtension docs](https://docs.godotengine.org/en/stable/tutorials/scripting/gdextension/index.html)
- [godot-rust book](https://godot-rust.github.io/)
