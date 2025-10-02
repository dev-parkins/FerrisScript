# Changelog

All notable changes to FerrisScript will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- Comprehensive version planning documentation
  - `docs/VERSION_PLANNING.md` - Version strategy overview
  - `docs/v0.0.2-CHECKLIST.md` - Patch release planning
  - `docs/v0.1.0-ROADMAP.md` - Feature roadmap
  - `docs/DOCUMENTATION_INVENTORY.md` - Documentation audit
- v0.0.2 documentation workflow with phase-by-phase guide
  - `docs/v0.0.2-DOCUMENTATION-WORKFLOW.md` - Complete workflow
  - `docs/v0.0.2-QUICK-START.md` - Quick start guide
  - `docs/VALIDATION_REPORT.md` - Installation validation results
  - `docs/SINGLE_SOURCE_OF_TRUTH.md` - Anti-duplication matrix

### Fixed
- Installation instructions: corrected `cd` command to match actual repository directory name (`FerrisScript` instead of `ferrisscript`) for case-sensitive file systems (Linux/macOS)

---

## [0.0.1] - 2025-10-02

**Codename**: "Ferris Awakens" 🦀

### Added

#### Language Features
- Variables with `let` (immutable) and `let mut` (mutable)
- Basic types: `i32`, `f32`, `bool`, `String`, `Vector2`, `Node`
- Automatic type coercion (`i32` → `f32`)
- Arithmetic operators: `+`, `-`, `*`, `/`, `%`
- Comparison operators: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical operators: `&&`, `||`, `!`
- Compound assignment: `+=`, `-=`
- Control flow: `if`/`else` conditionals
- Loops: `while` loops
- Functions with parameters and return types
- Line comments (`//`)
- Chained field access (e.g., `self.position.x`)

#### Godot Integration
- Full Godot 4.x integration via GDExtension
- `FerrisScriptNode` custom node type (extends `Node2D`)
- Script loading from `.ferris` files
- Lifecycle callbacks: `_ready()` and `_process(delta: f32)`
- Self binding for node property access (`self.position`)
- Built-in `print()` function
- Compilation and runtime error reporting to Godot console

#### Developer Tools
- 96 automated tests (69 compiler, 26 runtime, 1 godot_bind)
- 11 example scripts demonstrating all features
- Complete documentation suite
  - README.md with quick start and examples
  - ARCHITECTURE.md with technical design
  - DEVELOPMENT.md with contributor guide
  - RELEASING.md with release process
- Cargo workspace with 3 crates:
  - `ferrisscript_compiler` - Lexer, parser, type checker
  - `ferrisscript_runtime` - Runtime execution engine
  - `ferrisscript_godot_bind` - GDExtension bindings
- Cross-platform support (Linux, Windows, macOS)
- GitHub Actions CI/CD workflow
- MIT License

#### Branding & Assets
- Official FerrisScript logo (crab with scroll)
- Brand guidelines and color palette
- GitHub social preview image

### Documentation
- Project README with features, examples, and installation
- Architecture documentation explaining design decisions
- Development guide for contributors
- Release process documentation
- 11 example `.ferris` scripts
- Godot integration test project with README

### Fixed
- All clippy warnings resolved
- Error messages improved for better debugging
- GitHub Actions upgraded to latest versions

### Performance
- Interpreted execution (baseline for future optimizations)
- All tests complete in < 3 seconds

---

## Release Links

- [0.0.1](https://github.com/dev-parkins/FerrisScript/releases/tag/v0.0.1) - 2025-10-02

---

## Legend

Types of changes:
- **Added** - New features
- **Changed** - Changes to existing functionality
- **Deprecated** - Soon-to-be removed features
- **Removed** - Removed features
- **Fixed** - Bug fixes
- **Security** - Vulnerability fixes
