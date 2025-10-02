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
  - `docs/PHASE_TRACKING.md` - Action items for all phases
  - `docs/FUTURE_AUTOMATION.md` - Godot CLI automation planning
- **Community Documentation (Phase 2)**
  - `CONTRIBUTING.md` - Comprehensive contribution guide with development setup, PR workflow, code style, and first-time contributor resources
  - `CODE_OF_CONDUCT.md` - Contributor Covenant 2.1 code of conduct
  - `.github/ISSUE_TEMPLATE/bug_report.md` - Bug report template with environment details
  - `.github/ISSUE_TEMPLATE/feature_request.md` - Feature request template with use cases
  - `.github/ISSUE_TEMPLATE/documentation.md` - Documentation improvement template
  - `.github/ISSUE_TEMPLATE/config.yml` - Issue template configuration with links to Discussions and guides
  - `.github/PULL_REQUEST_TEMPLATE.md` - PR template with checklist and type indicators
- **FAQ and Troubleshooting (Phase 3)**
  - `docs/FAQ.md` - Comprehensive FAQ covering 31 questions across installation, language features, Godot integration, performance, and project roadmap
  - `docs/TROUBLESHOOTING.md` - Platform-specific troubleshooting for Windows (MSVC), macOS (Xcode), Linux (libclang), common build errors, Godot integration, and runtime issues
- **GitHub Project Management Documentation**
  - `docs/GITHUB_PROJECT_MANAGEMENT.md` - CI/CD optimization strategies, label system (20 labels), milestone planning, wiki decision matrix, and implementation roadmap
  - `docs/GITHUB_INSIGHTS_DESCRIPTION.md` - Repository descriptions, topics, badges, and social preview guidance for improved discoverability
- **Security and Architecture Documentation (Phase 4)**
  - `SECURITY.md` - Vulnerability reporting policy with GitHub Security Advisories integration, 48-hour response time, coordinated disclosure process, and security best practices
  - `docs/ARCHITECTURE.md` - Comprehensive system architecture documentation covering compiler pipeline (lexer→parser→type checker), runtime execution (tree-walking interpreter), Godot integration (GDExtension + thread-local storage), design decisions, extension points, and performance considerations
  - `.github/workflows/docs-lint.yml` - Automated documentation linting CI with markdownlint and link checking
  - `.markdownlint.json` - Markdown linting configuration with project-specific rules
  - `.markdown-link-check.json` - Link checking configuration with retry logic and localhost ignoring
- **Enhanced Example Documentation**
  - `examples/hello/README.md` - Comprehensive hello world tutorial covering FerrisScript basics, `_ready()` lifecycle, `print()` function, running examples, common gotchas, and 4 variations
  - `examples/move/README.md` - Movement tutorial explaining `_process(delta)`, framerate-independent movement, `self.position` access, delta time calculations, and 6 variations
  - `examples/bounce/README.md` - Bouncing animation tutorial demonstrating global variables, mutability, conditionals, state management, boundary checks, and 6 variations
  - `README.md` - Added dedicated Examples section with links to detailed example tutorials

### Changed
- `.github/ISSUE_TEMPLATE/config.yml` - Updated Discussions link description to clarify available categories (Q&A, Ideas, Show and Tell)

### Fixed
- Installation instructions: corrected `cd` command to match actual repository directory name (`FerrisScript` instead of `ferrisscript`) for case-sensitive file systems (Linux/macOS)
- README.md: removed 255 lines of corrupted duplicate source code that was appended after closing message

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
