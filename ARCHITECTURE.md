# FerrisScript 🦀 Architecture

**Version**: 0.0.1  
**Last Updated**: January 2025

## Overview

FerrisScript is a Rust-inspired scripting language designed for Godot 4.x. This document describes the architectural decisions, design patterns, and technical rationale behind the v0.0.1 implementation.

---

## Core Architecture

### Three-Crate Design

```
ferrisscript/
├── crates/compiler/      # Lexer, Parser, Type Checker
├── crates/runtime/       # AST Interpreter, Execution Engine
└── crates/godot_bind/    # GDExtension Integration
```

**Rationale**: Separation of concerns allows independent testing and future optimization (e.g., swapping interpreter for bytecode compiler).

---

## Compiler (`ferrisscript_compiler`)

### Lexer

**Implementation**: Single-pass character-by-character tokenization

**Key Decisions**:
- **Line comments (`//`)**: Added early for testing convenience
- **Error handling**: `Result<Vec<Token>, String>` with position tracking
- **Keyword recognition**: HashMap for O(1) lookup performance

**Learning**: Early span tracking made error reporting much better in later phases.

### Parser

**Implementation**: Recursive descent parser with Pratt parsing for expressions

**Key Decisions**:
- **Pratt parser for expressions**: Handles operator precedence elegantly, easy to extend
- **Compound assignment desugaring**: `+=` and `-=` expand to `x = x + ...` at parse time
- **Field access chains**: `self.position.x` represented as nested `FieldAccess` nodes
- **Global variables**: Parsed at program level for persistent state (critical for `bounce.ferris`)

**Learning**: Pratt parsing scales beautifully when adding new operators.

### Type Checker

**Implementation**: Two-pass type checking with scoped symbol tables

**Key Decisions**:
- **Type system scope**: `i32`, `f32`, `bool`, `String`, `Vector2`, `Node` (minimal for MVP)
- **Type coercion**: Implicit `i32 → f32` coercion for ergonomics
- **Scoped symbol tables**: Prevents variable shadowing bugs
- **Field access validation**: Type-checked against known struct definitions (e.g., `Vector2.x: f32`)

**Learning**: Scoped symbol tables prevented subtle bugs during testing.

---

## Runtime (`ferrisscript_runtime`)

### Execution Model

**Implementation**: Tree-walking AST interpreter

**Key Decisions**:
- **Value representation**: Simple `enum Value` for all runtime values
- **Environment**: `HashMap<String, VarInfo>` for variable storage
- **Control flow**: `FlowControl` enum pattern for returns in nested scopes
- **Mutability tracking**: `VarInfo { value, is_mutable }` separates concerns

**Performance**: Interpreted execution is sufficient for v0.0.1 scripting use cases.

**Learning**: Simple enum-based values work well for MVP, can optimize later if needed.

### Built-in Functions

**Implementation**: Function registry with `Fn(&[Value]) -> Result<Value, String>`

**Current Built-ins**:
- `print(...)`: Variable argument printing
- More to be added in future versions

**Extensibility**: Registry pattern allows easy addition of new built-ins.

---

## Godot Integration (`ferrisscript_godot_bind`)

### GDExtension Binding

**Framework**: `gdext` (official Rust binding for Godot 4.x)

**Key Decisions**:
- **Node type**: `FerrisScriptNode` extends `Node2D` (for `position` property)
- **Script loading**: Use Godot's `FileAccess` API for `res://` path handling
- **Compilation**: Compile `.ferris` files at `_ready()` time
- **Error reporting**: Use `godot_print!`, `godot_warn!`, `godot_error!` macros

### State Persistence

**Implementation**: Store `Env` (runtime environment) in `FerrisScriptNode`

**Rationale**: 
- Each node instance maintains its own variable state
- Variables persist across frames (critical for stateful scripts like `bounce.ferris`)
- Multiple nodes can run different scripts independently

**Learning**: Storing `Env` in the node works perfectly for frame-to-frame state.

### Property Bridge (Self Binding)

**Implementation**: Thread-local storage for property getter/setter callbacks

**Key Decisions**:
- **Thread-local storage**: Chosen over context passing for MVP simplicity
- **Property sync pattern**: Read node properties → execute script → write properties back
- **Supported properties**: `position: Vector2` (with `x`, `y` field access)

**Architecture**:
```rust
// Set up callbacks before execution
PROPERTY_GETTER.set(|| get_node_position());
PROPERTY_SETTER.set(|| set_node_position(new_value));

// Execute script (can access self.position)
runtime::execute_function("_process", &[delta], &mut env);

// Write back modified properties
write_properties_back_to_node();
```

**Rationale**: Thread-local storage avoids complex lifetime/borrowing issues in v0.0.1. Future versions may use context passing for better testability.

**Learning**: This pattern cleanly separates runtime from Godot binding concerns.

### Callbacks

**Supported Callbacks**:
- `_ready()`: Called when node enters scene tree
- `_process(delta: f32)`: Called every frame

**Future**: `_physics_process`, `_input`, custom signals

---

## Design Patterns

### Error Handling

**Pattern**: `Result<T, String>` throughout all crates

**Rationale**:
- Consistent error propagation with `?` operator
- String errors sufficient for MVP (no error codes needed)
- Span information included in error messages for helpful diagnostics

### Testing Strategy

**Approach**: Comprehensive unit tests at each layer

**Statistics** (v0.0.1):
- Compiler tests: 69 (lexer, parser, type checker)
- Runtime tests: 26 (execution, control flow, mutability)
- Godot bind tests: 1 (placeholder for manual testing)
- **Total**: 96 automated tests

**Learning**: Testing with real example files (`hello.ferris`, `move.ferris`, `bounce.ferris`) validated design decisions early.

### Manual Testing

**Process**: Godot 4.x integration validated manually for each phase

**Rationale**: Automated GDExtension testing is complex; manual validation caught issues automated tests missed (e.g., property synchronization timing).

---

## Key Technical Decisions

### 1. Error Handling Strategy
**Decision**: `Result<T, String>` with span-based error messages  
**Rationale**: Consistent, easy to propagate with `?`, sufficient for MVP  
**Trade-off**: No structured error types yet (can add later)

### 2. Type System Scope
**Decision**: `i32`, `f32`, `bool`, `String`, `Vector2`, `Node` only  
**Rationale**: Minimal types needed for Godot integration  
**Future**: Add more Godot types, arrays, custom structs

### 3. Type Coercion
**Decision**: Implicit `i32 → f32` coercion  
**Rationale**: Ergonomics (avoids explicit casts in common cases)  
**Trade-off**: Less strict than pure Rust, but acceptable for scripting

### 4. Operator Precedence
**Decision**: Pratt parser for expressions  
**Rationale**: Clean, extensible, handles precedence naturally  
**Learning**: Easy to add new operators as needed

### 5. Compound Assignment Desugaring
**Decision**: `+=` and `-=` desugar at parse time  
**Rationale**: Simplifies type checker and runtime (treat as regular assignment)  
**Trade-off**: No opportunity for optimization (acceptable for v0.0.1)

### 6. Field Access
**Decision**: Chained field access (`self.position.x`) as nested AST nodes  
**Rationale**: Properly parsed and type-checked  
**Learning**: Enables natural Godot property access patterns

### 7. Global Variables
**Decision**: Supported at program level  
**Rationale**: Needed for persistent state (e.g., `bounce.ferris`)  
**Trade-off**: Not idiomatic Rust, but common in game scripting

### 8. Memory Model
**Decision**: Simple ownership (all values cloned)  
**Rationale**: Avoids lifetime complexity in MVP  
**Future**: Consider reference counting for performance

### 9. String Interning
**Decision**: Not implemented in v0.0.1  
**Rationale**: Premature optimization  
**Future**: Add if profiling shows string allocation overhead

### 10. Performance (Interpreted vs Bytecode)
**Decision**: Tree-walking interpreter for v0.0.1  
**Rationale**: Simpler to implement, sufficient for scripting  
**Future**: Bytecode compilation for performance-critical scripts

### 11. Runtime Value Representation
**Decision**: Simple `enum Value { Int, Float, Bool, String, Vector2, Node }`  
**Rationale**: Easy to implement, covers all MVP types  
**Future**: Optimize memory layout if needed

---

## Known Limitations (v0.0.1)

### Language Features
- No struct definitions (only built-in types)
- No enums or pattern matching (only if/else)
- No generics
- No method calls (only free functions)
- No array/collection types
- No string interpolation

### Godot Integration
- Limited type support (only `Vector2`, `Node`)
- No signals
- No custom properties (only built-in `position`)
- No inheritance/composition
- No hot reload
- No debugging support
- No editor integration (syntax highlighting, autocomplete)

### Performance
- Interpreted execution (no bytecode)
- Value cloning (no reference counting)
- No optimization passes

---

## Future Enhancements (post-0.0.1)

### Language Features
- Struct definitions with methods
- Enums and pattern matching
- Array and dictionary types
- String interpolation
- Generics (long-term)

### Tooling
- Language Server Protocol (LSP) for IDE support
- Editor plugin for syntax highlighting
- Debugger integration
- REPL for interactive development

### Performance
- Bytecode compilation
- Constant folding and dead code elimination
- Reference counting instead of cloning
- String interning

### Godot Integration
- Full Godot API coverage (more types and methods)
- Signal support
- Custom properties
- Resource loading
- Scene instantiation
- Hot reload
- Editor integration (custom inspector, syntax highlighting)

---

## References

- **Pratt Parsing**: [Matklad's Blog Post](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
- **gdext Documentation**: [Godot-Rust Book](https://godot-rust.github.io/book/)
- **Rust Error Handling**: [The Rust Book](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

## Acknowledgments

- **Ferris 🦀**: The Rust mascot, our project's namesake
- **Godot Engine**: Powerful open-source game engine
- **gdext**: Excellent Rust bindings for Godot 4.x
- **Rust Community**: For creating an amazing language and ecosystem
