# FerrisScript 🦀 Development Guide

This guide covers development setup, workflows, and contribution guidelines for FerrisScript.

---

## 🚀 Quick Start

### Prerequisites

- **Rust 1.70+** (install via [rustup](https://rustup.rs/))
  - Currently tested on Rust 1.90.0 (stable)
  - Uses Rust 2021 edition
- **Godot 4.2+** (for testing GDExtension integration)
- **Git** for version control

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/dev-parkins/FerrisScript.git
cd FerrisScript

# Build all crates
cargo build --workspace

# Run all tests (96 tests)
cargo test --workspace

# Build optimized release
cargo build --workspace --release
```

### Quick Commands

```bash
# Build specific crate
cargo build -p ferrisscript_compiler
cargo build -p ferrisscript_runtime
cargo build -p ferrisscript_godot_bind

# Run tests with output
cargo test --workspace -- --show-output

# Check code formatting
cargo fmt --all -- --check

# Run clippy linter
cargo clippy --workspace -- -D warnings

# Generate documentation
cargo doc --workspace --open
```

---

## 📁 Project Structure

```
ferrisscript/
├── .github/workflows/       # CI/CD automation
├── crates/
│   ├── compiler/            # Lexer, parser, type checker (69 tests)
│   ├── runtime/             # AST interpreter (26 tests)
│   └── godot_bind/          # GDExtension integration (1 test)
├── docs/
│   ├── archive/             # Version-specific documentation
│   └── DEVELOPMENT.md       # This file
├── examples/                # 11 example .ferris scripts
├── godot_test/              # Test Godot project
├── ARCHITECTURE.md          # Technical design documentation
├── LICENSE                  # MIT License
├── README.md                # Main project documentation
└── RELEASE_NOTES.md         # v0.0.1 release information
```

### Crate Responsibilities

| Crate | Purpose | Lines of Code | Tests |
|-------|---------|---------------|-------|
| `ferrisscript_compiler` | Tokenization, parsing, type checking | ~1,500 | 69 |
| `ferrisscript_runtime` | AST interpretation, execution engine | ~1,200 | 26 |
| `ferrisscript_godot_bind` | Godot 4.x GDExtension integration | ~800 | 1 |

---

## 🔧 Development Workflow

### 1. Choose a Task

Check the roadmap in [ARCHITECTURE.md](../ARCHITECTURE.md) or create an issue for new features.

### 2. Create a Branch

```bash
git checkout -b feat/your-feature-name
# or
git checkout -b fix/bug-description
```

### 3. Make Changes

- Write code following Rust conventions
- Add tests for new functionality
- Update documentation if needed

### 4. Test Your Changes

```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p ferrisscript_compiler

# Run specific test
cargo test test_compile_hello

# Check formatting and linting
cargo fmt --all
cargo clippy --workspace
```

### 5. Commit with Conventional Commits

```bash
# Feature additions
git commit -m "feat(compiler): add support for array types"

# Bug fixes
git commit -m "fix(runtime): handle division by zero correctly"

# Documentation
git commit -m "docs: update README with new examples"

# Tests
git commit -m "test(parser): add tests for field access"

# Maintenance
git commit -m "chore: update dependencies to latest versions"
```

### 6. Push and Create PR

```bash
git push origin feat/your-feature-name
# Then create a pull request on GitHub
```

---

## 🧪 Testing

### Test Organization

- **Unit tests**: In each module (e.g., `lexer.rs`, `parser.rs`)
- **Integration tests**: In `tests/` directories
- **Example tests**: Validate `.ferris` files compile correctly

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p ferrisscript_compiler

# Specific module
cargo test --lib lexer::tests

# With output (see println! statements)
cargo test -- --show-output --nocapture

# Single test
cargo test test_compile_hello -- --exact
```

### Adding Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_feature() {
        let result = your_function();
        assert_eq!(result, expected_value);
    }
}
```

---

## 📝 Code Style

### Rust Conventions

- Use `rustfmt` for formatting: `cargo fmt --all`
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use descriptive variable names
- Add doc comments for public APIs

### Documentation

```rust
/// Compiles FerrisScript source code into an AST.
///
/// # Arguments
/// * `source` - The FerrisScript source code as a string
///
/// # Returns
/// * `Ok(Program)` - Successfully compiled AST
/// * `Err(String)` - Compilation error message
///
/// # Examples
/// ```
/// let source = "fn _ready() { print(\"Hello\"); }";
/// let ast = compile(source)?;
/// ```
pub fn compile(source: &str) -> Result<Program, String> {
    // ...
}
```

---

## �️ Rust Edition Explained

### What is a Rust Edition?

Rust uses an **edition system** (2015, 2018, 2021) to introduce backwards-incompatible changes without breaking existing code. Think of it like:
- **Python 2 vs Python 3** (but less painful)
- **C++11, C++14, C++17** standards

### FerrisScript Uses 2021 Edition

```toml
[package]
edition = "2021"  # In all Cargo.toml files
```

**Benefits of 2021 Edition**:
- **Disjoint capture in closures**: Closures only borrow fields they use
- **Panic message consistency**: Better error messages
- **IntoIterator for arrays**: Can iterate `[1, 2, 3]` directly
- **New preludes**: `TryFrom`, `TryInto` available automatically
- **Cargo resolver v2**: Better dependency resolution

**Backwards Compatibility**: Code compiled with edition 2021 works with libraries using 2015/2018. The edition only affects **how your code is compiled**, not the ABI.

### Should We Upgrade?

**Current**: 2021 edition (latest)  
**Recommendation**: ✅ Keep 2021 - it's the latest stable edition

---

## 🔄 Dependency Management

### Updating Dependencies

```bash
# Check for outdated dependencies
cargo outdated

# Update all dependencies to latest compatible versions
cargo update

# Update specific dependency
cargo update -p gdext

# Update to breaking versions (edit Cargo.toml first)
cargo build
```

### Adding Dependencies

```toml
# In crates/compiler/Cargo.toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

---

## 🐛 Debugging

### Debug Prints

```rust
// Use dbg! macro for debugging
let result = dbg!(some_calculation());

// Use eprintln! for errors
eprintln!("Error: {}", error_message);
```

### Godot Integration Debugging

```rust
// In godot_bind/src/lib.rs
godot_print!("Debug info: {}", value);
godot_warn!("Warning: {}", warning);
godot_error!("Error: {}", error);
```

### LLDB/GDB Debugging

```bash
# Build with debug symbols
cargo build

# Run with debugger
rust-lldb target/debug/your_binary
# or
rust-gdb target/debug/your_binary
```

---

## 📊 Performance Profiling

```bash
# Build with release optimizations
cargo build --release

# Profile with perf (Linux)
perf record target/release/your_binary
perf report

# Benchmark (if you add benches/)
cargo bench
```

---

## 🤝 Contributing

### Commit Message Convention

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style changes (formatting, no logic change)
- `refactor`: Code refactoring
- `test`: Adding/updating tests
- `chore`: Maintenance tasks
- `ci`: CI/CD changes

**Examples**:
```bash
feat(compiler): add array type support
fix(runtime): handle division by zero
docs: update ARCHITECTURE.md with new patterns
test(parser): add tests for function calls
chore: update gdext to 0.2.0
```

### Code Review Checklist

Before submitting a PR:
- [ ] All tests pass (`cargo test --workspace`)
- [ ] Code is formatted (`cargo fmt --all`)
- [ ] No clippy warnings (`cargo clippy --workspace`)
- [ ] Documentation updated if needed
- [ ] Commit messages follow convention
- [ ] PR description explains the change

---

## 📚 Resources

### FerrisScript Docs
- [ARCHITECTURE.md](../ARCHITECTURE.md) - Technical design and decisions
- [RELEASE_NOTES.md](../RELEASE_NOTES.md) - Release information
- [docs/archive/](archive/) - Version-specific development docs

### Rust Learning
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### Compiler/Interpreter Resources
- [Crafting Interpreters](https://craftinginterpreters.com/) - Excellent book
- [Pratt Parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
- [Rust Compiler Development Guide](https://rustc-dev-guide.rust-lang.org/)

### Godot + Rust
- [Godot 4.x Documentation](https://docs.godotengine.org/en/stable/)
- [gdext Book](https://godot-rust.github.io/book/)
- [GDExtension Documentation](https://docs.godotengine.org/en/stable/tutorials/scripting/gdextension/)

---

## 📄 License

FerrisScript is licensed under the [MIT License](../LICENSE)

MIT (to be added)

