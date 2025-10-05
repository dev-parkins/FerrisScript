# FerrisScript 🦀 Release Notes

> **Note**: This file contains the release notes for the current/latest release.  
> For historical release notes, see `docs/archive/vX.Y.Z/RELEASE_NOTES_vX.Y.Z.md`  
> For a complete changelog across all versions, see [CHANGELOG.md](CHANGELOG.md)

---

## v0.0.2 - Foundation & Polish (October 5, 2025)

**Status**: ✅ Complete  
**Tag**: `v0.0.2`  
**Codename**: "Community Foundation"

### 🎉 Highlights

FerrisScript v0.0.2 focuses on building a solid foundation for community contribution and development. This release significantly improves error messages, adds comprehensive documentation, implements VS Code editor support, and expands test coverage—all while maintaining 100% backward compatibility with v0.0.1.

### 🌟 What's New for Users

#### Better Error Messages 🔍

- **38 Enhanced Error Messages** with actionable context and hints
- **"Did You Mean?" Suggestions** for typos in variable/function names
- **Code Snippets** showing exactly where errors occur
- **Multiple Related Locations** for complex errors (e.g., "variable declared here, used here")
- **Helpful Hints** like "use `let mut` if you want to modify this variable"

**Example**:

```
error: Cannot assign to immutable variable 'x'
  ┌─ examples/assign.ferris:3:1
  │
2 │     let x = 10;
  │         - variable declared as immutable here
3 │     x = 20;
  │     ^^^^^^ cannot assign to immutable variable
  │
  = help: use `let mut x = 10` if you want to modify this variable
```

#### VS Code Syntax Highlighting ✨

- **FerrisScript VS Code Extension** with syntax highlighting for `.ferris` files
- **11 Code Snippets** for common patterns (`fn`, `let`, `if`, `while`, etc.)
- **Installation**: Available in workspace (see `/crates/vscode-ferrisscript/`)

#### Comprehensive Testing Guide 📚

- **New TESTING.md** (655 lines) covering all 116 tests
- **Test Organization** by module (parser, type checker, runtime, integration)
- **Coverage Information** (70-75% overall, 80-85% core logic)
- **Running Tests** documentation for contributors

### 🛠️ What's New for Contributors

#### Community Infrastructure

- **CONTRIBUTING.md**: Complete contribution guide with workflow, PR guidelines, code style
- **CODE_OF_CONDUCT.md**: Community standards and enforcement process
- **Issue Templates**: Bug reports, feature requests, documentation improvements
- **FAQ.md**: Common questions about language design, Godot integration, troubleshooting
- **SECURITY.md**: Vulnerability reporting process

#### Enhanced Documentation

- **10,000+ Lines** of new documentation across project
- **API Documentation**: 395+ lines of rustdoc covering compiler and runtime APIs
- **Architecture Guide**: Complete system design documentation
- **Troubleshooting Guide**: Common issues and solutions
- **Version Planning**: Strategic roadmaps for v0.0.3–v0.1.0

#### GitHub Setup

- **Custom Labels**: Organized by type, status, priority, area
- **GitHub Badges**: Build status, test coverage, license, version
- **Branch Protection**: Recommendations for main branch security

#### Improved Testing & Quality

- **96 → 116 Tests** (+20.8% growth)
- **Test Coverage**: 65-70% → 70-75% (+5%)
- **Edge Case Coverage**: Enhanced handling of error conditions
- **Benchmarks**: Performance baselines for compiler and runtime
- **CI Integration**: Automated testing on every PR

### 📦 Dependencies & Compatibility

- **Rust**: 1.70+ required (unchanged)
- **Godot**: 4.2+ supported (unchanged)
- **gdext**: 0.1.x (Godot Rust bindings, unchanged)
- **No Breaking Changes**: 100% compatible with v0.0.1 scripts

### 🔄 Upgrade Guide

#### For Script Authors

1. **No changes required** - all v0.0.1 scripts work in v0.0.2
2. **Optional**: Install VS Code extension for better editing experience
3. **Optional**: Review new FAQ.md if you had questions about language behavior

#### For Contributors

1. **Read CONTRIBUTING.md** before submitting PRs (includes testing best practices)
2. **Review API docs** for compiler/runtime internals
3. **Use issue templates** when reporting bugs or requesting features

#### Installing VS Code Extension

```bash
# From project root
cd crates/vscode-ferrisscript
code --install-extension vscode-ferrisscript-0.0.1.vsix
```

### 📊 Metrics

- **PRs Merged**: 17 (#3-19)
- **Commits**: 150+ commits
- **Documentation**: 10,000+ new lines
- **Tests**: 116 total (20.8% increase)
- **Coverage**: 70-75% overall
- **Files**: 60+ new documentation and infrastructure files

### 🔗 Resources

- **Full Changelog**: [CHANGELOG.md v0.0.2](CHANGELOG.md#002---2025-10-05)
- **Deferral Analysis**: See `docs/archive/v0.0.2/V0.0.2_DEFERRAL_ANALYSIS.md` for incomplete items moved to future versions
- **Testing Guide**: See `CONTRIBUTING.md` for testing practices and `cargo test` usage
- **API Docs**: Run `cargo doc --open` for generated rustdoc
- **Examples**: `/examples` and `/godot_test/scripts` directories

### 🚀 What's Next?

**v0.0.3 "Editor Experience Alpha"** will focus on:

- Enhanced VS Code language server with autocomplete and diagnostics
- Improved development workflow with `develop` branch
- CI optimization for faster feedback
- Additional error message improvements

See [v0.0.3-roadmap.md](docs/planning/v0.0.3-roadmap.md) for detailed plans.

### 🙏 Acknowledgments

Thanks to the Rust and Godot communities for inspiration and tooling support. Special thanks to contributors of documentation improvements and testing enhancements.

---

## v0.0.1 - Initial Release (October 2, 2025)

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

See [ARCHITECTURE.md](docs/ARCHITECTURE.md) for full technical details.

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
- [x] Code pushed to GitHub ✅
- [x] Release tagged (v0.0.1) ✅
- [x] Release published on GitHub ✅

---

**Released by**: FerrisScript Contributors  
**Release Date**: October 2, 2025  
**Build**: v0.0.1-stable
