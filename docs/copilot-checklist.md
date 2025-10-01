# RustyScript 0.0.1 Development Checklist

**Project Goal**: Create a minimal proof-of-concept Rust-inspired scripting language for Godot 4.x

**Last Updated**: October 1, 2025

---

## 📋 Development Phases

### ✅ Phase 1: Project Setup (1-2 commits)
- [x] **1.1** Initialize git repository with `.gitignore`
  - Commit: `chore: initialize git repository with .gitignore` ✅
  - Add Rust/Cargo ignore patterns ✅
  - Add Godot ignore patterns ✅
  - Add OS-specific patterns (Windows, macOS, Linux) ✅

- [x] **1.2** Create workspace structure with all crates
  - Commit: `feat: scaffold workspace structure with compiler, runtime, and godot_bind crates` ✅
  - Create `crates/compiler`, `crates/runtime`, `crates/godot_bind` ✅
  - Add workspace `Cargo.toml` ✅
  - Add individual crate `Cargo.toml` files ✅
  - Add placeholder `lib.rs` for each crate ✅

- [x] **1.3** Fix Godot 4.x dependency (gdext instead of gdnative)
  - Commit: `fix: update to gdext for Godot 4.x compatibility` ✅
  - Research latest `gdext` version ✅
  - Update `godot_bind/Cargo.toml` with correct dependency ✅
  - Add GDExtension configuration files ✅

- [x] **1.4** Create example `.rscr` files
  - Commit: `docs: add example RustyScript files` ✅
  - Create `examples/hello.rscr` ✅
  - Create `examples/move.rscr` ✅
  - Create `examples/bounce.rscr` ✅

---

### ✅ Phase 2: Minimal Lexer (2-3 commits)

- [x] **2.1** Implement Token enum and basic structure
  - Commit: `feat(compiler): implement complete lexer with tokenization` ✅
  - Complete Token enum with all variants ✅
  - Add `#[derive(Debug, Clone, PartialEq)]` ✅
  - Add helper methods for token display ✅

- [x] **2.2** Implement lexer core with whitespace and identifiers
  - Commit: Combined into single comprehensive lexer implementation ✅
  - Character iterator with peek ✅
  - Keyword recognition (fn, let, mut, if, else, while, return, true, false) ✅
  - Identifier parsing ✅
  - Whitespace skipping ✅
  - Basic error reporting with line/column tracking ✅

- [x] **2.3** Add number and string literal parsing
  - Commit: Combined into single comprehensive lexer implementation ✅
  - Integer and float parsing ✅
  - String literal parsing with escape sequences (\n, \t, \r, \\, \") ✅
  - Boolean literals (true/false) ✅

- [x] **2.4** Add operators and delimiters
  - Commit: Combined into single comprehensive lexer implementation ✅
  - Single-char operators (+, -, *, /, etc.) ✅
  - Multi-char operators (==, !=, <=, >=, &&, ||) ✅
  - Compound assignment operators (+=, -=) ✅
  - Delimiters (parentheses, braces, semicolons, commas, dot, colon) ✅

- [x] **2.5** Add lexer unit tests
  - Commit: Combined into single comprehensive lexer implementation ✅
  - Test each token type ✅
  - Test `hello.rscr` tokenization ✅
  - Test `move.rscr` tokenization ✅
  - Test `bounce.rscr` tokenization ✅
  - Test error cases (unterminated strings, invalid escapes, unexpected chars) ✅
  - Test edge cases (empty input, only whitespace, comments) ✅

---

### 🌳 Phase 3: Basic Parser (3-4 commits)

- [ ] **3.1** Complete AST definitions
  - Commit: `feat(compiler): complete AST node definitions`
  - Finalize all AST node types
  - Add position/span information for error reporting
  - Add Display/Debug implementations

- [ ] **3.2** Implement function declaration parsing
  - Commit: `feat(compiler): implement function declaration parsing`
  - Parse `fn name(params) { body }`
  - Parse parameter lists with types
  - Parse function body as statement list

- [ ] **3.3** Implement statement parsing
  - Commit: `feat(compiler): implement statement parsing`
  - Let bindings with optional type annotations
  - Expression statements
  - If/else statements
  - While loops

- [ ] **3.4** Implement expression parsing with precedence
  - Commit: `feat(compiler): implement expression parsing with operator precedence`
  - Pratt parser or precedence climbing
  - Binary operators with correct precedence
  - Literals and variables
  - Function calls
  - Parenthesized expressions

- [ ] **3.5** Implement field access for `self.property.field`
  - Commit: `feat(compiler): add field access expression parsing`
  - Dot notation parsing
  - Chained field access (e.g., `self.position.x`)

- [ ] **3.6** Add parser unit tests
  - Commit: `test(compiler): add comprehensive parser unit tests`
  - Test function parsing
  - Test statement parsing
  - Test expression parsing
  - Test `hello.rscr` full parse
  - Test error recovery

---

### 🔍 Phase 4: Type Checker Stub (1-2 commits)

- [ ] **4.1** Implement basic type environment
  - Commit: `feat(compiler): implement basic type checking infrastructure`
  - Symbol table for variables and functions
  - Built-in type definitions (i32, f32, bool, String)
  - Godot type stubs (Vector2, Node, etc.)

- [ ] **4.2** Implement expression type checking
  - Commit: `feat(compiler): implement expression type checking`
  - Type inference for literals
  - Type checking for binary operations
  - Type checking for function calls
  - Basic error reporting for type mismatches

- [ ] **4.3** Add type checker unit tests
  - Commit: `test(compiler): add type checker unit tests`
  - Test valid programs pass
  - Test type mismatches are caught
  - Test undefined variable detection

---

### 🏃 Phase 5: Stub Runtime (2 commits)

- [ ] **5.1** Implement runtime environment and value representation
  - Commit: `feat(runtime): implement runtime value types and environment`
  - Value enum (Int, Float, Bool, String, etc.)
  - Environment struct with variable scopes
  - Scope push/pop for block scoping

- [ ] **5.2** Implement statement execution
  - Commit: `feat(runtime): implement statement execution`
  - Execute let bindings
  - Execute expression statements
  - Execute if/else conditionals
  - Execute while loops

- [ ] **5.3** Implement expression evaluation
  - Commit: `feat(runtime): implement expression evaluation`
  - Evaluate literals
  - Evaluate variables (lookup in environment)
  - Evaluate binary operations
  - Type coercion and runtime checks

- [ ] **5.4** Implement built-in function stubs
  - Commit: `feat(runtime): add built-in function stubs (print, etc.)`
  - `print()` function
  - Other utility functions as needed
  - Function registry system

- [ ] **5.5** Add runtime unit tests
  - Commit: `test(runtime): add runtime execution unit tests`
  - Test variable binding and lookup
  - Test expression evaluation
  - Test control flow
  - Test `hello.rscr` execution

---

### 🎮 Phase 6: Godot Integration (3-4 commits)

- [ ] **6.1** Set up gdext project structure
  - Commit: `feat(godot_bind): set up gdext project structure`
  - Create `.gdextension` configuration file
  - Set up proper build configuration
  - Add godot-rust project template

- [ ] **6.2** Create RustyScriptNode GDExtension class
  - Commit: `feat(godot_bind): implement RustyScriptNode class`
  - Define RustyScriptNode inheriting from Node
  - Register class with Godot
  - Add basic properties

- [ ] **6.3** Implement script loading and compilation
  - Commit: `feat(godot_bind): add script loading and compilation`
  - Load `.rscr` file from path
  - Compile to AST using compiler crate
  - Cache compiled scripts
  - Error handling and reporting to Godot console

- [ ] **6.4** Implement `_ready` callback integration
  - Commit: `feat(godot_bind): implement _ready callback integration`
  - Find `_ready` function in compiled script
  - Execute `_ready` when node enters tree
  - Pass Godot node context to runtime

- [ ] **6.5** Test in Godot project
  - Commit: `test(godot_bind): add test Godot project with RustyScript`
  - Create minimal Godot 4.x project
  - Add test scene with RustyScriptNode
  - Verify `hello.rscr` prints to console
  - Document build and test process

---

### 🔄 Phase 7: Process Loop (2-3 commits)

- [ ] **7.1** Implement `_process` callback integration
  - Commit: `feat(godot_bind): implement _process callback integration`
  - Find `_process` function in compiled script
  - Call on every frame
  - Pass delta parameter

- [ ] **7.2** Implement `self` binding mechanism
  - Commit: `feat(runtime): implement self binding for Godot node access`
  - Special `self` variable in runtime environment
  - Bridge to Godot node properties
  - Field access through Godot's property system

- [ ] **7.3** Implement property getter/setter bridge
  - Commit: `feat(godot_bind): implement property getter/setter bridge`
  - Get node properties from Godot (e.g., `position`)
  - Set node properties back to Godot
  - Type conversion between runtime values and Godot types

- [ ] **7.4** Test `move.rscr` in Godot
  - Commit: `test(godot_bind): verify move.rscr works in Godot`
  - Create test scene with moving node
  - Verify position updates each frame
  - Document any issues or limitations

---

### 🎯 Phase 8: Mutable State & Control Flow (2-3 commits)

- [ ] **8.1** Implement mutable variable tracking
  - Commit: `feat(runtime): implement mutable variable tracking`
  - Track mut vs immutable in environment
  - Enforce immutability at runtime
  - Error on illegal mutations

- [ ] **8.2** Implement persistent script state
  - Commit: `feat(godot_bind): implement persistent script state between frames`
  - Maintain runtime environment across `_process` calls
  - Initialize state in `_ready`
  - Preserve variable values between frames

- [ ] **8.3** Complete if/while implementation in runtime
  - Commit: `feat(runtime): complete control flow implementation`
  - Ensure if/else works correctly
  - Ensure while loops work correctly
  - Test with complex conditions

- [ ] **8.4** Test `bounce.rscr` in Godot
  - Commit: `test(godot_bind): verify bounce.rscr works in Godot`
  - Create test scene with bouncing node
  - Verify direction changes at boundaries
  - Verify mutable `dir` variable persists

---

## 🎉 Phase 9: Polish & Documentation (1-2 commits)

- [ ] **9.1** Update README with build instructions
  - Commit: `docs: add comprehensive build and usage instructions`
  - How to build the workspace
  - How to use in Godot projects
  - Known limitations
  - Future roadmap

- [ ] **9.2** Add CI/CD configuration (optional)
  - Commit: `ci: add GitHub Actions workflow`
  - Run tests on push
  - Build all targets
  - Check formatting and clippy

- [ ] **9.3** Create release notes for 0.0.1
  - Commit: `docs: add 0.0.1 release notes`
  - Summarize features
  - Document examples
  - List known issues

---

## 📝 Notes

### Key Technical Decisions to Make:
1. **Error Handling Strategy**: Return `Result` vs panic for different error types
2. **Memory Model**: How to handle Godot object lifetimes in Rust
3. **Type System Scope**: Which Godot types to support initially
4. **String Interning**: Use string interning for identifiers?
5. **Performance**: Interpreted vs bytecode compilation for 0.0.1

### Known Limitations for 0.0.1:
- No struct definitions
- No signals
- No inheritance/composition
- No generics
- Limited Godot type support
- No hot reload
- No debugging support
- No editor integration

### Future Enhancements (post-0.0.1):
- Language server protocol (LSP) for IDE support
- Hot reload
- More Godot types and APIs
- Struct definitions
- Enums and pattern matching
- Signal support
- Editor plugin for syntax highlighting
- Bytecode compilation for performance
- Debugging protocol integration

---

## 🔗 Quick Links

- [Godot 4.x Documentation](https://docs.godotengine.org/en/stable/)
- [gdext Documentation](https://godot-rust.github.io/docs/gdext/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Crafting Interpreters](https://craftinginterpreters.com/)

---

**Progress Tracking**: Update checkboxes as tasks are completed. Each major milestone should be committed with proper commit messages following conventional commits format.
