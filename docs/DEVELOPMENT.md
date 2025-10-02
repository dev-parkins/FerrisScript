# FerrisScript Development Guide

## Quick Start

### Prerequisites
- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Git

### Building the Project

```powershell
# Clone the repository (when available on GitHub)
git clone https://github.com/yourusername/FerrisScript.git
cd FerrisScript

# Build all crates (except godot_bind which requires Godot setup)
cargo build -p FerrisScript_compiler -p FerrisScript_runtime

# Run tests
cargo test -p FerrisScript_compiler -p FerrisScript_runtime
```

### Project Structure

```
FerrisScript/
├── Cargo.toml              # Workspace configuration
├── docs/
│   └── copilot-checklist.md # Development roadmap
├── crates/
│   ├── compiler/           # Lexer, parser, type checker
│   ├── runtime/            # Execution engine
│   └── godot_bind/         # Godot 4.x integration (gdext)
└── examples/               # Example .ferris scripts
    ├── hello.ferris          # Basic print example
    ├── move.ferris           # Movement example
    └── bounce.ferris         # Stateful bouncing example
```

## Development Status

### ✅ Completed (Phase 1)
- [x] Git repository with proper .gitignore
- [x] Workspace structure with compiler, runtime, godot_bind crates
- [x] Godot 4.x compatibility (gdext instead of gdnative)
- [x] Example .ferris files

### 🔄 Next Steps (See docs/copilot-checklist.md)
- [ ] Phase 2: Implement minimal lexer
- [ ] Phase 3: Implement basic parser
- [ ] Phase 4: Type checker stub
- [ ] Phase 5: Runtime execution
- [ ] Phase 6: Godot integration
- [ ] Phase 7: Process loop & mutable state

## Commit Convention

This project uses [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Adding/updating tests
- `chore:` - Maintenance tasks

## Contributing

1. Check `docs/copilot-checklist.md` for current tasks
2. Create feature branch: `git checkout -b feat/your-feature`
3. Make changes with proper commit messages
4. Run tests: `cargo test`
5. Submit pull request

## Resources

- [Godot 4.x Documentation](https://docs.godotengine.org/en/stable/)
- [gdext (Godot Rust)](https://godot-rust.github.io/docs/gdext/)
- [Crafting Interpreters Book](https://craftinginterpreters.com/)

## License

MIT (to be added)

