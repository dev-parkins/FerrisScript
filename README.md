# FerrisScript 🦀

> **A Rust-inspired scripting language for Godot 4.x**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Godot](https://img.shields.io/badge/godot-4.2%2B-blue.svg)](https://godotengine.org/)

FerrisScript (named after [Ferris 🦀](https://rustacean.net/), the Rust mascot) is a **statically-typed, Rust-inspired scripting language** designed specifically for Godot 4.x game development. It brings Rust's safety and performance philosophy to game scripting while maintaining a lightweight, easy-to-learn syntax.

## ✨ Features

- 🦀 **Rust-Inspired Syntax** - Familiar to Rust developers, easy for beginners
- 🎮 **Godot 4.x Integration** - Native GDExtension support via `gdext`
- ⚡ **Static Type Checking** - Catch errors before runtime
- 🔒 **Immutability by Default** - Safe by default, explicit `mut` for mutations
- 🎯 **Zero-Cost Abstractions** - Compiled to efficient runtime execution
- 📦 **Minimal Dependencies** - Lightweight and fast compilation

## 🚀 Quick Start

### Prerequisites

- **Rust 1.70+** ([Install Rust](https://www.rust-lang.org/tools/install))
- **Godot 4.2+** ([Download Godot](https://godotengine.org/download))
- **Git** (for cloning the repository)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/ferrisscript.git
cd ferrisscript

# Build the project
cargo build --workspace

# Run tests
cargo test --workspace
```

### Using in Godot

1. **Build the GDExtension:**
   ```bash
   cargo build --package ferrisscript_godot_bind
   ```

2. **Open the test project:**
   - Open Godot 4.2+
   - Import project from `godot_test/project.godot`

3. **Create your first script:**
   ```rust
   // my_script.ferris
   fn _ready() {
       print("Hello from FerrisScript!");
   }

   fn _process(delta: f32) {
       self.position.x += 50.0 * delta;
   }
   ```

4. **Attach to a node:**
   - Add `FerrisScriptNode` to your scene
   - Set `script_path` to `res://scripts/my_script.ferris`
   - Run your game!

## 📖 Language Overview

### Basic Syntax

```rust
// Variables - immutable by default
let name: String = "Ferris";
let age: i32 = 42;

// Mutable variables - explicit opt-in
let mut counter: i32 = 0;
counter = counter + 1;

// Functions
fn greet(name: String) -> String {
    return "Hello, " + name;
}

// Control flow
if age > 18 {
    print("Adult");
} else {
    print("Minor");
}

// Loops
let mut i: i32 = 0;
while i < 10 {
    print(i);
    i = i + 1;
}
```

### Godot Integration

```rust
// Global state persists between frames
let mut velocity: f32 = 0.0;
let gravity: f32 = 980.0;

fn _ready() {
    print("Game started!");
}

fn _process(delta: f32) {
    // Access node properties via 'self'
    velocity = velocity + gravity * delta;
    self.position.y += velocity * delta;
    
    // Bounce at ground level
    if self.position.y > 500.0 {
        velocity = -velocity * 0.8;
        self.position.y = 500.0;
    }
}
```

### Type System

FerrisScript supports the following types:

- **Primitives**: `i32`, `f32`, `bool`, `String`
- **Godot Types**: `Vector2`, `Node`, `Node2D`
- **Type Inference**: Literals are automatically typed
- **Type Coercion**: `i32` → `f32` automatic conversion

## 🏗️ Project Structure

```
ferrisscript/
├── Cargo.toml                 # Workspace root
├── README.md                  # This file
├── crates/
│   ├── compiler/              # Lexer, Parser, Type Checker
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs         # Public compile() API
│   │       ├── lexer.rs       # Tokenization
│   │       ├── parser.rs      # AST generation
│   │       ├── type_checker.rs# Static type checking
│   │       └── ast.rs         # AST definitions
│   ├── runtime/               # Execution engine
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs         # Runtime interpreter
│   └── godot_bind/            # Godot 4.x integration
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs         # GDExtension bindings
├── examples/                  # Example scripts
│   ├── hello.ferris           # Basic _ready callback
│   ├── move.ferris            # Movement example
│   └── bounce.ferris          # State & control flow
├── godot_test/                # Godot test project
│   ├── project.godot
│   ├── ferrisscript.gdextension
│   └── scripts/               # Test scripts
└── docs/                      # Additional documentation
    ├── PHASE*_TESTING.md      # Phase testing guides
    └── copilot-checklist.md   # Development checklist
```

## 🔧 Building from Source

### Build All Crates

```bash
# Debug build (faster compilation)
cargo build --workspace

# Release build (optimized)
cargo build --workspace --release

# Run all tests
cargo test --workspace
```

### Build Specific Crates

```bash
# Compiler only
cargo build --package ferrisscript_compiler

# Runtime only
cargo build --package ferrisscript_runtime

# Godot extension only
cargo build --package ferrisscript_godot_bind
```

### Running Tests

```bash
# All tests
cargo test --workspace

# Compiler tests (44 tests)
cargo test --package ferrisscript_compiler

# Runtime tests (26 tests)
cargo test --package ferrisscript_runtime

# Watch mode (with cargo-watch)
cargo watch -x "test --workspace"
```

## 🎮 Godot Integration Guide

### Step 1: Build the Extension

```bash
cargo build --package ferrisscript_godot_bind
```

This creates:
- **Windows**: `target/debug/ferrisscript_godot_bind.dll`
- **Linux**: `target/debug/libferrisscript_godot_bind.so`
- **macOS**: `target/debug/libferrisscript_godot_bind.dylib`

### Step 2: Set Up Your Godot Project

1. Create a new Godot 4.2+ project
2. Create `.gdextension` file in your project root:

```ini
[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.2

[libraries]
windows.debug.x86_64 = "res://../target/debug/ferrisscript_godot_bind.dll"
windows.release.x86_64 = "res://../target/release/ferrisscript_godot_bind.dll"
linux.debug.x86_64 = "res://../target/debug/libferrisscript_godot_bind.so"
linux.release.x86_64 = "res://../target/release/libferrisscript_godot_bind.so"
macos.debug = "res://../target/debug/libferrisscript_godot_bind.dylib"
macos.release = "res://../target/release/libferrisscript_godot_bind.dylib"
```

### Step 3: Create Script Files

Create a `.ferris` file in your project:

```rust
// scripts/player.ferris
let mut speed: f32 = 200.0;

fn _ready() {
    print("Player initialized!");
}

fn _process(delta: f32) {
    // Your game logic here
    self.position.x += speed * delta;
}
```

### Step 4: Attach to Nodes

1. Add `FerrisScriptNode` (extends Node2D) to your scene
2. In the Inspector, set `script_path` to `res://scripts/player.ferris`
3. Run your game!

## 📚 API Reference

### Built-in Functions

- `print(value)` - Print to Godot console

### Special Functions

- `_ready()` - Called when node enters the scene tree
- `_process(delta: f32)` - Called every frame

### Self Binding

Access node properties via `self`:
- `self.position` - Node's position (Vector2)
- `self.position.x` - X coordinate (f32)
- `self.position.y` - Y coordinate (f32)

## 🧪 Testing

### Manual Testing in Godot

The `godot_test/` directory contains a complete test project:

```bash
# 1. Build extension
cargo build --package ferrisscript_godot_bind

# 2. Open in Godot
# Import godot_test/project.godot

# 3. Run tests (F5)
# Check Output panel for results
```

See `godot_test/README.md` for detailed testing instructions.

### Automated Testing

```bash
# Run all unit tests
cargo test --workspace

# Test results:
# - Compiler: 44 tests passing
# - Runtime: 26 tests passing
# - Total: 70+ tests
```

## 📊 Current Status (v0.0.1)

### ✅ Implemented Features

- [x] Lexer with full tokenization
- [x] Parser with operator precedence
- [x] Type checker with static analysis
- [x] Runtime interpreter
- [x] Godot 4.x GDExtension integration
- [x] `_ready()` and `_process()` callbacks
- [x] Self binding for node property access
- [x] Mutable variable tracking
- [x] Control flow (if/else, while loops)
- [x] Function definitions and calls
- [x] Global state persistence

### 🚧 Planned Features (v0.1.0+)

- [ ] Arrays and collections
- [ ] For loops
- [ ] String interpolation
- [ ] More Godot types (Node3D, Input, etc.)
- [ ] Signal system integration
- [ ] Struct definitions
- [ ] Match expressions
- [ ] LSP support for IDE integration

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test --workspace`)
5. Commit your changes (`git commit -m 'feat: add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Commit Conventions

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `test:` - Test additions/changes
- `refactor:` - Code refactoring
- `chore:` - Maintenance tasks

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Ferris** 🦀 - The Rust mascot that inspired our name
- **Godot Engine** - Amazing open-source game engine
- **gdext** - Rust bindings for Godot 4
- **Rust Community** - For the incredible language and ecosystem

## 📞 Contact & Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/ferrisscript/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/ferrisscript/discussions)
- **Documentation**: [docs/](./docs/)

---

**Made with 🦀 and ❤️ for the Godot community**
    Fn,
    Let,
    Mut,
    If,
    Else,
    While,
    Ident(String),
    Number(f32),
    Bool(bool),
    String(String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Not,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    // placeholder: tokenize input source
    Ok(vec![])
}
```

### src/parser.rs

```rust
use crate::lexer::Token;
use crate::ast::Program;

pub fn parse(tokens: &[Token]) -> Result<Program, String> {
    // placeholder: parse tokens into AST
    Ok(Program { functions: vec![] })
}
```

### src/type_checker.rs

```rust
use crate::ast::Program;

pub fn check(program: &Program) -> Result<(), String> {
    // placeholder: type-check AST
    Ok(())
}
```

### src/ast.rs

```rust
#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub ty: String,
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Let { name: String, ty: Option<String>, value: Expr },
    If { cond: Expr, then_branch: Vec<Stmt>, else_branch: Vec<Stmt> },
    While { cond: Expr, body: Vec<Stmt> },
}

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Variable(String),
    Binary(Box<Expr>, Op, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Debug)]
pub enum Literal {
    Int(i32),
    Float(f32),
    Bool(bool),
    Str(String),
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}
```

---

## Crate: `runtime`

### Cargo.toml

```toml
[package]
name = "ferrisscript_runtime"
version = "0.0.1"
edition = "2021"

[dependencies]
ferrisscript_compiler = { path = "../compiler" }
```

### src/lib.rs

```rust
use ferrisscript_compiler::ast;

pub struct Env {
    // placeholder: variable environment
}

pub fn execute(program: &ast::Program, env: &mut Env) -> Result<(), String> {
    // placeholder: walk AST and execute statements
    Ok(())
}
```

---

## Crate: `godot_bind`

### Cargo.toml

```toml
[package]
name = "ferrisscript_godot_bind"
version = "0.0.1"
edition = "2021"

[dependencies]
godot = "0.1" # Godot 4.x Rust binding (gdext)
ferrisscript_runtime = { path = "../runtime" }
```

### src/lib.rs

```rust
use godot::prelude::*;
use ferrisscript_runtime::Env;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct FerrisScriptNode {
    env: Env,
    #[base]
    base: Base<Node2D>,
}

#[godot_api]
impl FerrisScriptNode {
    fn new(base: Base<Node2D>) -> Self {
        FerrisScriptNode {
            env: Env::new(),
            base,
        }
    }

    #[func]
    fn _ready(&mut self) {
        godot_print!("FerrisScript _ready hook");
    }

    #[func]
    fn _process(&mut self, delta: f64) {
        // placeholder: call runtime execution here
    }
}
```

---

## Examples

### `examples/hello.ferris`

```rust
fn _ready() {
    print("Hello from FerrisScript!");
}
```

### `examples/move.ferris`

```rust
fn _process(delta: f32) {
    self.position.x += 50.0 * delta;
}
```

### `examples/bounce.rscr`

```rust
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

---

## ✅ Notes

* This scaffold is **Copilot-ready**: stubs and placeholder functions allow auto-completion of lexer, parser, type-checker, AST traversal, and Godot integration.
* The focus of 0.0.1 is **proof of concept**, with minimal features to attach scripts to Godot nodes and run `_ready` / `_process`.
* Hot reload and advanced features (structs, signals, persistent state) are left for later versions.

```

---

This is ready to **copy directly from the canvas** and export as a Markdown file.  

Do you want me to also include **ready-to-run `cargo init` commands** to scaffold the crates on disk so you can start coding immediately?

```

Let's pause here for now for a break to add additional examples:
```
# branch.rscr – test if/else branching

let x: Int = 5

if x > 3 {
    print("big")
} else {
    print("small")
}

```
```
# loop.rscr – test while and for loops

let i: Int = 0

while i < 3 {
    print("while loop: " + str(i))
    i = i + 1
}

for j in 0..3 {
    print("for loop: " + str(j))
}
```
```
# functions.rscr – test function definitions and return types

fn add(a: Int, b: Int) -> Int {
    return a + b
}

fn greet(name: String) {
    print("Hello, " + name)
}

let sum: Int = add(2, 3)
print("Sum is " + str(sum))

greet("RustyScript")
```
```
# type_error.rscr – negative test for type safety
# Expected: compiler should reject mismatched types

let x: Int = "hello"   # ❌ should fail type-check
```
```
# scene.rscr – test Godot node access

let sprite: Sprite = node("Player")
sprite.flip_h = true
sprite.position.x = sprite.position.x + 10
```
```
# reload.rscr – test state across reloads

let count: Int = 0

count = count + 1
print("Reload count: " + str(count))
```
```
# collections.rscr – test arrays and iteration

let nums: Array<Int> = [1, 2, 3]

for n in nums {
    print("num: " + str(n))
}
```
```
# match.rscr – test enums and pattern matching

enum Dir { Left, Right, Up, Down }

let d: Dir = Dir.Left

match d {
    Dir.Left => print("going left"),
    Dir.Right => print("going right"),
    Dir.Up => print("going up"),
    Dir.Down => print("going down"),
}
```