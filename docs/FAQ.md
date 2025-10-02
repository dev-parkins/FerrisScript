# Frequently Asked Questions (FAQ)

**Last Updated:** October 2, 2025  
**Version:** v0.0.2

Welcome to the FerrisScript FAQ! Find answers to common questions about installation, usage, and Godot integration.

> **Can't find your answer?** Check [TROUBLESHOOTING.md](TROUBLESHOOTING.md) or ask in [GitHub Discussions](https://github.com/dev-parkins/FerrisScript/discussions).

---

## Table of Contents

- [Installation & Setup](#installation--setup)
- [Language & Syntax](#language--syntax)
- [Godot Integration](#godot-integration)
- [Development & Contributing](#development--contributing)
- [Performance & Optimization](#performance--optimization)
- [Project Status & Roadmap](#project-status--roadmap)

---

## Installation & Setup

### What are the minimum requirements?

- **Rust 1.70+** ([Install Rust](https://www.rust-lang.org/tools/install))
- **Godot 4.2+** (for Godot integration) ([Download Godot](https://godotengine.org/download))
- **Git** (for cloning the repository)

See the [README Installation section](../README.md#installation) for detailed setup instructions.

### How long does the build take?

**First build:** 3-5 minutes on modern hardware due to dependency compilation  
**Subsequent builds:** 1-2 seconds if no code changes  
**Clean rebuild:** 3-5 minutes

**Tip:** Use `cargo build --release` for optimized builds (takes longer but produces faster binaries).

### Do I need Godot installed to build FerrisScript?

**No!** You can build and test the FerrisScript compiler and runtime without Godot:

```bash
cargo build --workspace
cargo test --workspace
```

You only need Godot if you want to:
- Use FerrisScript scripts in Godot projects
- Test Godot integration (GDExtension)
- Run the example Godot project in `godot_test/`

### I'm getting compile errors. What should I check?

Common issues:

1. **Rust version too old** - Run `rustc --version` and ensure it's 1.70+
   - Fix: `rustup update`

2. **Missing dependencies** (Windows) - Need Visual Studio Build Tools
   - See [TROUBLESHOOTING.md - Windows Issues](TROUBLESHOOTING.md#windows-issues)

3. **Stale build artifacts** - Sometimes cached builds cause issues
   - Fix: `cargo clean && cargo build`

4. **Network issues** - Can't download crates from crates.io
   - Check your internet connection
   - Try: `cargo build --offline` if you've built before

See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for platform-specific solutions.

### Can I use FerrisScript without Godot?

**Not in v0.0.1** - Currently FerrisScript is designed for Godot integration.

**Coming in v0.2.0** - Standalone mode will allow:
- Running `.ferris` scripts independently
- Command-line REPL
- Non-game applications

See [v0.1.0-ROADMAP.md](v0.1.0-ROADMAP.md) for planned features.

### What's the file extension? `.ferris` or `.rscr`?

**The correct extension is `.ferris`** ✅

All FerrisScript files use `.ferris`:
```
examples/hello.ferris
examples/bounce.ferris
examples/move.ferris
```

**Note:** Early documentation incorrectly referenced `.rscr` - this was corrected in v0.0.2.

---

## Language & Syntax

### What's the difference between FerrisScript and Rust?

| Feature | Rust | FerrisScript |
|---------|------|--------------|
| **Type system** | Full Rust ownership | Simplified ownership (v0.0.1) |
| **Compilation** | Native binary | Interpreted via runtime |
| **Target** | Systems programming | Game scripting |
| **Borrowing** | Full borrow checker | Planned for v0.1.0 |
| **Macros** | Full macro system | Not supported |
| **Standard library** | std, alloc, core | Godot-specific + subset |
| **Async/await** | Full async runtime | Not supported yet |

**TL;DR:** FerrisScript is inspired by Rust's syntax but simplified for game scripting. It's not a Rust compiler.

### Can I use existing Rust libraries (crates)?

**Not directly** - FerrisScript is not a Rust compiler and doesn't have access to cargo/crates.io.

**However:**
- The FerrisScript **runtime** is written in Rust and uses Rust crates internally
- You can extend FerrisScript by adding Rust code to the runtime (`crates/runtime/`)
- Future versions may support a plugin system for Rust interop

**Workaround for v0.0.1:** Modify the runtime source code to add functionality.

### How does FerrisScript integrate with GDScript?

FerrisScript and GDScript can coexist in the same Godot project:

- **FerrisScript** → Compiled to native code via GDExtension
- **GDScript** → Interpreted by Godot engine
- **Communication** → Both can call each other through Godot's scripting API

**Example:**
```ferris
// my_script.ferris
pub fn process_input(input: Input) -> Vector2 {
    // Performance-critical code in FerrisScript
}
```

```gdscript
# my_scene.gd
extends Node2D

func _ready():
    var ferris_script = FerrisScriptNode.new()
    var result = ferris_script.process_input(Input.get_mouse_position())
```

**Best practice:** Use FerrisScript for performance-critical logic, GDScript for game logic/UI.

### What Godot types does FerrisScript support?

**v0.0.1 Supported Types:**
- `Vector2` - 2D vectors
- `Node` - Base Godot node type
- Basic types: `i32`, `f32`, `bool`, `String`

**Coming in v0.1.0:**
- `Vector3` (3D vectors)
- `Color` (RGBA colors)
- `Transform` (3D transforms)
- `Resource` (Godot resources)
- More node types

See [v0.1.0-ROADMAP.md](v0.1.0-ROADMAP.md) for complete type roadmap.

### Does FerrisScript have a REPL?

**Not yet** - REPL (Read-Eval-Print Loop) is planned for v0.2.0.

**Current workflow:**
1. Write `.ferris` files
2. Build with `cargo build`
3. Run in Godot or via runtime

**Alternatives:**
- Use Godot's output console for debugging
- Run tests with `cargo test` for quick feedback

---

## Godot Integration

### How do I load FerrisScript in Godot?

See the [README "Using in Godot" section](../README.md#using-in-godot) for the 4-step process:

1. Build the GDExtension
2. Copy library to `godot_test/addons/ferrisscript/bin/`
3. Create `.gdextension` file
4. Import in Godot

**Full tutorial:** Coming in Phase 4 (Advanced Documentation).

### Godot doesn't recognize my `.ferris` files. Why?

**Common causes:**

1. **Missing .gdextension file** - Godot needs this to find FerrisScript
   - Location: `res://addons/ferrisscript/ferrisscript.gdextension`
   - See [README](../README.md#using-in-godot) for template

2. **Wrong library path** - `.gdextension` must point to correct binary
   - Windows: `ferrisscript_godot_bind.dll`
   - Linux: `libferrisscript_godot_bind.so`
   - macOS: `libferrisscript_godot_bind.dylib`

3. **Godot editor needs restart** - Changes to GDExtensions require restart

4. **Build not complete** - Ensure `cargo build --package ferrisscript_godot_bind` succeeded

See [TROUBLESHOOTING.md - Godot Integration](TROUBLESHOOTING.md#godot-integration) for detailed fixes.

### Can I mix FerrisScript and GDScript in the same project?

**Yes!** They work side-by-side:

```
my_game/
├── scripts/
│   ├── player.ferris         # Performance-critical player controller
│   ├── ui_manager.gd          # UI logic in GDScript
│   └── enemy_ai.ferris        # AI in FerrisScript
└── scenes/
    └── main.tscn
```

**Best practices:**
- Use FerrisScript for CPU-intensive tasks (physics, AI, large calculations)
- Use GDScript for prototyping, UI, and game logic
- Keep scripts organized by purpose, not language

### What's the performance overhead of FerrisScript?

**Current status (v0.0.1):**
- **Not optimized yet** - Focus is on correctness, not speed
- Early benchmarks show **2-5x faster than GDScript** for computation-heavy tasks
- Overhead from runtime interpretation (not JIT compiled)

**Optimization plans (v0.1.0+):**
- JIT compilation
- Better memory management
- Inline optimizations
- Target: **5-10x faster than GDScript**

See [v0.1.0-ROADMAP.md - Performance Roadmap](v0.1.0-ROADMAP.md) for details.

### Can I use FerrisScript for 3D games?

**Yes, but limited in v0.0.1:**
- `Vector2` support only (2D focus)
- `Vector3` coming in v0.1.0
- 3D node types coming in v0.1.0

**Current 3D workaround:**
- Write game logic in FerrisScript
- Use GDScript for 3D-specific code
- Wait for v0.1.0 (planned December 2025)

---

## Development & Contributing

### How do I contribute to FerrisScript?

See [CONTRIBUTING.md](../CONTRIBUTING.md) for complete guide:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests (`cargo test`)
5. Submit a Pull Request

**First-time contributors:** Look for issues labeled `good first issue`.

### How do I run the tests?

```bash
# Run all tests
cargo test --workspace

# Run tests for specific crate
cargo test --package rustyscript_compiler

# Run specific test
cargo test test_lexer_tokenizes_keywords

# Run with output
cargo test -- --nocapture
```

See [CONTRIBUTING.md - Testing Guidelines](../CONTRIBUTING.md#testing-guidelines) for more.

### How do I add a new language feature?

**Process:**

1. **Discuss first** - Open an issue describing the feature
2. **Update lexer** - Add tokens in `crates/compiler/src/lexer.rs`
3. **Update parser** - Add AST nodes in `crates/compiler/src/parser.rs`
4. **Update type checker** - Add type rules in `crates/compiler/src/type_checker.rs`
5. **Update runtime** - Add execution in `crates/runtime/src/lib.rs`
6. **Add tests** - Write tests for each component
7. **Update docs** - Document the feature

**Example:** See how `let mut` was implemented by reviewing git history.

### What's the code review process?

1. **Submit PR** - Use the [PR template](.github/PULL_REQUEST_TEMPLATE.md)
2. **CI checks** - All tests must pass, code must be formatted
3. **Maintainer review** - Typically within 2-3 days
4. **Address feedback** - Make requested changes
5. **Approval** - At least 1 maintainer approval required
6. **Merge** - Squash and merge into main

See [CONTRIBUTING.md - Pull Request Process](../CONTRIBUTING.md#pull-request-process).

---

## Performance & Optimization

### Is FerrisScript faster than GDScript?

**Short answer:** Yes, for computation-heavy tasks, but not optimized yet.

**Benchmarks (v0.0.1, preliminary):**
- Simple math loops: **2-3x faster**
- Complex algorithms: **3-5x faster**
- String operations: **1.5-2x faster**
- Godot API calls: **Similar** (bottleneck is Godot, not FerrisScript)

**Optimization status:**
- ❌ No JIT compilation
- ❌ No inlining
- ❌ No dead code elimination
- ✅ Static typing (compile-time checks)

**v0.1.0 goals:** 5-10x faster with optimizations.

### When should I use FerrisScript vs. GDScript?

**Use FerrisScript for:**
- Performance-critical loops
- Complex algorithms (pathfinding, physics)
- Large data processing
- Type safety (catch errors early)

**Use GDScript for:**
- Rapid prototyping
- UI and game logic
- Godot-specific features
- When performance doesn't matter

**Hybrid approach** (recommended):
- Prototype in GDScript
- Optimize bottlenecks with FerrisScript

### How do I profile FerrisScript code?

**v0.0.1:** No built-in profiler yet.

**Workarounds:**
- Use Godot's profiler for high-level metrics
- Add manual timing with `println!` in runtime
- Use Rust profilers (e.g., `cargo flamegraph`) on runtime

**Coming in v0.2.0:**
- Built-in profiler
- Hot reload
- Performance metrics

---

## Project Status & Roadmap

### What's the current status of FerrisScript?

- **Current version:** v0.0.1 (released October 2, 2025)
- **Status:** Early alpha, experimental
- **Production ready:** Not yet - expect breaking changes

**v0.0.1 Features:**
- Basic compiler (lexer, parser, type checker)
- Runtime execution
- Godot 4.x GDExtension support
- Variables, functions, basic types

**What's NOT ready:**
- Match expressions, enums, structs
- Full Rust ownership model
- Standard library
- Debugging tools

### What's the release schedule?

- **v0.0.2** - Documentation & Polish (October 15, 2025)
- **v0.1.0** - Language Features (December 15, 2025)
- **v0.2.0** - Tooling & Developer Experience (March 15, 2026)

See [v0.1.0-ROADMAP.md](v0.1.0-ROADMAP.md) for detailed roadmap.

### Can I use FerrisScript in production?

**Not recommended for v0.0.1:**
- Breaking changes expected
- Limited features
- Not performance-optimized
- Minimal documentation

**Wait for v0.2.0 (March 2026)** for stable production use.

**However:**
- ✅ Great for prototyping
- ✅ Learning Rust concepts
- ✅ Game jams (experimental)

### How can I stay updated?

- **GitHub Releases:** Watch the repository
- **Discussions:** Follow [GitHub Discussions](https://github.com/dev-parkins/FerrisScript/discussions)
- **Issues:** Subscribe to [feature requests](https://github.com/dev-parkins/FerrisScript/labels/enhancement)
- **Changelog:** Check [CHANGELOG.md](../CHANGELOG.md)

### Is there a Discord/community?

**Not yet** - Currently using GitHub Discussions for community interaction.

**Community channels (planned for v0.1.0):**
- Discord server
- Reddit community
- Twitter updates

**For now:** Use [GitHub Discussions](https://github.com/dev-parkins/FerrisScript/discussions) for:
- Questions (Q&A category)
- Feature ideas (Ideas category)
- Show your projects (Show and Tell)

---

## Still Have Questions?

- **Bugs/Issues:** [Open an issue](https://github.com/dev-parkins/FerrisScript/issues/new/choose)
- **Feature Requests:** [Discussions](https://github.com/dev-parkins/FerrisScript/discussions/categories/ideas)
- **General Help:** [Q&A Discussions](https://github.com/dev-parkins/FerrisScript/discussions/categories/q-a)
- **Troubleshooting:** See [TROUBLESHOOTING.md](TROUBLESHOOTING.md)
- **Contributing:** See [CONTRIBUTING.md](../CONTRIBUTING.md)

---

**Made with 🦀 and ❤️ for the Godot community**
