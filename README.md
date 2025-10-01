# RustyScript 0.0.1 – Complete Copilot-Friendly Scaffold

RustyScript is a **statically typed, Rust-inspired scripting language** for Godot 4.x.  
This repository contains a minimal prototype workspace for compiler, runtime, and Godot bindings.

---

## Workspace Layout

```
rustyscript/
├── Cargo.toml # workspace root
├── crates/
│ ├── compiler/
│ │ ├── Cargo.toml
│ │ └── src/
│ │ ├── lib.rs
│ │ ├── lexer.rs
│ │ ├── parser.rs
│ │ ├── type_checker.rs
│ │ └── ast.rs
│ ├── runtime/
│ │ ├── Cargo.toml
│ │ └── src/
│ │ └── lib.rs
│ └── godot_bind/
│ ├── Cargo.toml
│ └── src/
│ └── lib.rs
└── examples/
├── hello.rscr
├── move.rscr
└── bounce.rscr
```

---

## Root `Cargo.toml` (workspace)

```toml
[workspace]
members = [
    "crates/compiler",
    "crates/runtime",
    "crates/godot_bind",
]
````

---

## Crate: `compiler`

### Cargo.toml

```toml
[package]
name = "rustyscript_compiler"
version = "0.0.1"
edition = "2021"

[dependencies]
```

### src/lib.rs

```rust
pub mod lexer;
pub mod parser;
pub mod type_checker;
pub mod ast;

/// Compile RustyScript source to AST
pub fn compile(source: &str) -> Result<ast::Program, String> {
    let tokens = lexer::tokenize(source)?;
    let ast = parser::parse(&tokens)?;
    type_checker::check(&ast)?;
    Ok(ast)
}
```

### src/lexer.rs

```rust
#[derive(Debug, Clone)]
pub enum Token {
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
name = "rustyscript_runtime"
version = "0.0.1"
edition = "2021"

[dependencies]
rustyscript_compiler = { path = "../compiler" }
```

### src/lib.rs

```rust
use rustyscript_compiler::ast;

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
name = "rustyscript_godot_bind"
version = "0.0.1"
edition = "2021"

[dependencies]
gdnative = "0.10" # adjust to latest Godot 4.x Rust binding
rustyscript_runtime = { path = "../runtime" }
```

### src/lib.rs

```rust
use gdnative::prelude::*;
use rustyscript_runtime::Env;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct RustyScriptNode {
    env: Env,
}

#[methods]
impl RustyScriptNode {
    fn new(_owner: &Node) -> Self {
        RustyScriptNode {
            env: Env {},
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        godot_print!("RustyScript _ready hook");
    }

    #[export]
    fn _process(&mut self, _owner: &Node, delta: f64) {
        // placeholder: call runtime execution here
    }
}
```

---

## Examples

### `examples/hello.rscr`

```rust
fn _ready() {
    print("Hello from RustyScript!");
}
```

### `examples/move.rscr`

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
