# FerrisScript 🦀 Release Notes

## v0.0.1 - Initial Release (January 2025)

**Status**: ✅ Complete  
**Tag**: `v0.0.1`  
**Codename**: "Ferris Awakens"

### 🎉 Highlights

FerrisScript v0.0.1 is the initial proof-of-concept release of a Rust-inspired scripting language for Godot 4.x. Named after Ferris 🦀 (the Rust mascot), this release demonstrates core language features and functional Godot integration.

### ✨ Features

#### Language Support
- ✅ **Variables**: `let` (immutable) and `let mut` (mutable)
- ✅ **Types**: `i32`, `f32`, `bool`, `String`, `Vector2`, `Node`
- ✅ **Type Coercion**: Automatic `i32 → f32` conversion
- ✅ **Operators**: Arithmetic (`+`, `-`, `*`, `/`, `%`), comparison, logical
- ✅ **Compound Assignment**: `+=`, `-=`
- ✅ **Control Flow**: `if`/`else`, `while` loops
- ✅ **Functions**: Function definitions with parameters and return values
- ✅ **Comments**: Line comments (`//`)
- ✅ **Field Access**: Chained access (e.g., `self.position.x`)

#### Godot Integration
- ✅ **GDExtension**: Full Godot 4.x integration via `gdext`
- ✅ **FerrisScriptNode**: Custom node type (extends `Node2D`)
- ✅ **Script Loading**: Load `.ferris` files from `res://` paths
- ✅ **Callbacks**: `_ready()` and `_process(delta: f32)`
- ✅ **Self Binding**: Access node properties via `self.position`
- ✅ **Built-ins**: `print()` function for console output
- ✅ **Error Reporting**: Compilation and runtime errors to Godot console

#### Developer Experience
- ✅ **96 Automated Tests**: Comprehensive test coverage
- ✅ **Example Scripts**: 11 examples demonstrating all features
- ✅ **Documentation**: Complete README, architecture docs, testing guides
- ✅ **Build System**: Cargo workspace with 3 crates
- ✅ **Cross-Platform**: Windows, Linux, macOS support

### 📦 Installation

#### Prerequisites
- Rust 1.70+ with Cargo
- Godot 4.2+

#### Build from Source

```bash
# Clone repository
git clone https://github.com/dev-parkins/FerrisScript.git
cd FerrisScript

# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Build for release
cargo build --workspace --release
```

#### Godot Integration

1. Copy `ferrisscript.gdextension` to your Godot project
2. Ensure the extension points to the correct DLL/SO/DYLIB path:
   - Windows: `target/debug/ferrisscript_godot_bind.dll`
   - Linux: `target/debug/libferrisscript_godot_bind.so`
   - macOS: `target/debug/libferrisscript_godot_bind.dylib`
3. Place `.ferris` scripts in your project (e.g., `res://scripts/`)
4. Add `FerrisScriptNode` to your scene
5. Set the `script_path` property to your script

See [README.md](README.md) for detailed instructions.

### 📝 Example Scripts

#### Hello World (`hello.ferris`)
```rust
fn _ready() {
    print("Hello, Godot! FerrisScript is working!");
}
```

#### Movement (`move.ferris`)
```rust
fn _process(delta: f32) {
    self.position.x += 50.0 * delta;
}
```

#### Bouncing (`bounce.ferris`)
```rust
let mut velocity: f32 = 100.0;

fn _process(delta: f32) {
    self.position.y += velocity * delta;
    
    if self.position.y > 400.0 {
        velocity = -100.0;
    } else if self.position.y < 100.0 {
        velocity = 100.0;
    }
}
```

### 🧪 Testing

```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p ferrisscript_compiler
cargo test -p ferrisscript_runtime

# Run with output
cargo test --workspace -- --show-output
```

**Test Results**: 96/96 passing (100% success rate)
- Compiler: 69 tests
- Runtime: 26 tests
- Godot Bind: 1 test

### 📊 Project Statistics

- **Total Commits**: 22 (across 9 development phases)
- **Lines of Code**: ~3,500 (Rust)
- **Example Scripts**: 11 (`.ferris` files)
- **Documentation**: 5 main docs + 4 archived phase guides
- **Build Time**: ~2-3 seconds (debug), ~30 seconds (release)

### 🔍 Known Limitations

#### Language Features
- ❌ No struct definitions (only built-in types)
- ❌ No enums or pattern matching
- ❌ No generics
- ❌ No method calls (only free functions)
- ❌ No array/collection types
- ❌ No string interpolation

#### Godot Integration
- ❌ Limited Godot type support (only `Vector2`, `Node`)
- ❌ No signals
- ❌ No custom properties (only `position`)
- ❌ No hot reload
- ❌ No debugging support
- ❌ No editor integration

#### Performance
- ⚠️ Interpreted execution (no bytecode)
- ⚠️ Value cloning (no reference counting)

See [ARCHITECTURE.md](ARCHITECTURE.md) for full technical details.

### 🐛 Known Issues

- None reported as of release

### 🛣️ Roadmap (v0.1.0+)

#### Language Features
- [ ] Struct definitions with methods
- [ ] Enums and match expressions
- [ ] Array and dictionary types
- [ ] String interpolation
- [ ] For loops

#### Godot Integration
- [ ] More Godot types (Color, Rect2, Transform2D, etc.)
- [ ] Signal support
- [ ] Custom properties
- [ ] More callbacks (input, physics)
- [ ] Hot reload

#### Tooling
- [ ] Language Server Protocol (LSP)
- [ ] Syntax highlighting plugin
- [ ] Debugger integration
- [ ] REPL

#### Performance
- [ ] Bytecode compilation
- [ ] Constant folding
- [ ] Reference counting

### 🙏 Acknowledgments

- **Ferris 🦀**: The Rust mascot and our project's namesake
- **Godot Engine**: For creating an amazing open-source game engine
- **gdext**: For excellent Rust bindings to Godot 4.x
- **Rust Community**: For building such a wonderful language and ecosystem

### 📄 License

FerrisScript is licensed under the [MIT License](LICENSE).

### 🤝 Contributing

Contributions are welcome! Please see [README.md](README.md) for contribution guidelines.

### 📧 Contact

- **Repository**: [github.com/dev-parkins/FerrisScript](https://github.com/dev-parkins/FerrisScript)
- **Issues**: [GitHub Issues](https://github.com/dev-parkins/FerrisScript/issues)
- **Discussions**: [GitHub Discussions](https://github.com/dev-parkins/FerrisScript/discussions)

---

## Release Checklist

- [x] All 96 tests passing
- [x] Documentation complete (README, ARCHITECTURE, godot_test/README)
- [x] Example scripts working (`hello.ferris`, `move.ferris`, `bounce.ferris`)
- [x] Manual Godot validation passed
- [x] License added (MIT)
- [x] Project rebranded to FerrisScript
- [x] File extensions updated (`.rscr` → `.ferris`)
- [x] GitHub repository created (dev-parkins/FerrisScript)
- [x] CI/CD workflow configured
- [ ] Code pushed to GitHub
- [ ] Release tagged (v0.0.1)
- [ ] Release published on GitHub

---

**Released by**: FerrisScript Contributors  
**Release Date**: January 2025  
**Build**: v0.0.1-stable
