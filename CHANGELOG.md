# Changelog

All notable changes to FerrisScript will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added

- None

### Changed

- None

### Fixed

- None

---

## [0.0.3] - 2025-10-08

**Codename**: "Editor Experience Alpha" 💡🔍

This release transforms FerrisScript into a professional development environment with enhanced error diagnostics, full IDE support, and comprehensive development tooling. Focus on developer experience and editor integration.

### Added

#### Enhanced Error Diagnostics (Phases 1-3)

- **Structured Error Code System** (Phase 1, PR #27)
  - Error codes E001-E499 across 5 categories (Lexical, Syntax, Type, Semantic, Runtime)
  - `ERROR_CODES.md` comprehensive reference with examples and fixes
  - Documentation URLs in error messages (GitHub + Jekyll site)
  - All 63 error codes categorized and documented

- **"Did You Mean?" Suggestions** (Phase 2, PR #28)
  - Levenshtein distance-based typo detection for variables, functions, and types
  - Adaptive thresholds for short vs long identifiers
  - 20+ comprehensive suggestion tests
  - Keyword suggestions deferred to v0.0.4 (Phase 2B)

- **Error Documentation & Recovery** (Phase 3, PRs #32, #34, #35)
  - Enhanced ERROR_CODES.md with cross-references between related errors
  - Jekyll documentation site at https://dev-parkins.github.io/FerrisScript
  - Professional landing page with error code lookup and navigation
  - **Parser Error Recovery (Phase 3C)**: Panic-mode error recovery enabling parser to continue after syntax errors
  - Synchronization points: `;`, `}`, `fn`, `let`
  - Cascading error prevention
  - 23 new recovery-specific tests (13 unit + 10 integration)
  - Foundation for multi-error reporting (Phase 3D/3E deferred to v0.0.4)

#### VS Code Extension Features (Phases 4-5)

- **Code Completion** (Phase 4, PR #37)
  - Keyword completion (let, fn, if, else, while, return, mut, true, false)
  - Type completion (i32, f32, bool, String, Vector2, Node, void)
  - Built-in function completion (print with parameter hints)
  - Context-aware completion (statement-level vs expression keywords)
  - TypeScript extension infrastructure

- **Hover & Problem Panel** (Phase 5, PR #38)
  - Hover tooltips for keywords, types, and functions
  - Markdown-formatted hover content with syntax-highlighted examples
  - Problem panel integration (inline red squiggles)
  - Automatic compiler detection (workspace or PATH)
  - Real-time error diagnostics
  - File icons for `.ferris` files (Rust-inspired crab icon)
  - Improved marketplace presentation

#### Development Tooling & CI (Phase 6+7)

- **Development Scripts** (Phase 6, PR #39)
  - `scripts/lint.ps1` / `.sh` - Cargo clippy with strict warnings
  - `scripts/test.ps1` / `.sh` - Run all workspace tests
  - `scripts/bench.ps1` / `.sh` - Run benchmarks
  - `scripts/format.ps1` / `.sh` - Code formatting
  - `scripts/coverage.ps1` / `.sh` - Coverage reports
  - Pre-commit hooks (format, lint, tests)
  - `scripts/install-git-hooks.ps1` / `.sh` - Hook installation
  - `scripts/README.md` - Comprehensive script documentation

- **Benchmark CI** (Phase 7, PR #39)
  - `.github/workflows/benchmarks.yml` - Automated benchmarking
  - Runs on push to main/develop, manual dispatch for feature branches
  - Compiler benchmarks (lexer, parser, type_checker)
  - Runtime benchmarks
  - Baseline measurements documented
  - Results uploaded as artifacts

- **Project Presentation** (Phase 9 Quick Wins)
  - GitHub badges in README.md (build status, version, license, Rust, Godot, stars)

#### Infrastructure Improvements

- **CI/CD Consolidation** (PR #40)
  - Consolidated code scanning & coverage workflows
  - SonarQube quality scanning (coverage disabled)
  - Codecov reporting in code-scanning.yml
  - CodeQL evaluation documented
  - SHA-pinned actions for security (supply chain protection)

### Changed

- **Error Messages**: All errors now include error codes, source context, and helpful hints
- **VS Code Extension**: Upgraded from basic syntax highlighting to full IDE support
- **Development Workflow**: Streamlined with comprehensive scripts and pre-commit hooks

### Fixed

- **Critical Parser Bug** (Phase 3C, PR #35): Fixed infinite loop in error recovery that consumed all RAM when encountering unexpected top-level tokens. Parser now always advances past problematic tokens before synchronizing to recovery points.
- **CI Build Errors**: Fixed cross-compilation target installation for Linux/macOS builds
- **Documentation Links**: Fixed 11 broken markdown links across planning documents
- **Dependency Updates**: Updated criterion 0.5 → 0.7, godot 0.1 → 0.4, fixed API breaking changes

### Deferred

- **Phase 3D/3E**: Multi-error reporting and diagnostic collection (deferred to v0.0.4)
- **Phase 8**: Integration tests and cross-platform verification (deferred to v0.0.4)
- **Phase 9**: Test coverage badge, Rustdoc hosting, marketplace submission (deferred to v0.1.0)

---

## [0.0.2] - 2025-10-05

**Codename**: "Foundation & Polish" 🦀✨

This release focuses on establishing a solid foundation for contributors and users through comprehensive documentation, improved error messages, code quality enhancements, and basic editor support. No new language features were added (saving those for v0.1.0).

### Added

#### Community Infrastructure (PR #3, Phase 4)

- **Contributing Guidelines**
  - `CONTRIBUTING.md` - Complete contributor guide with development setup, PR workflow, code style guidelines, testing requirements, commit conventions, and first-time contributor resources
  - `CODE_OF_CONDUCT.md` - Contributor Covenant 2.1 code of conduct with enforcement guidelines
  - `.github/PULL_REQUEST_TEMPLATE.md` - PR checklist template linking to contributing guide
  - `.github/ISSUE_TEMPLATE/bug_report.md` - Structured bug report with environment details
  - `.github/ISSUE_TEMPLATE/feature_request.md` - Feature request template with use case section
  - `.github/ISSUE_TEMPLATE/documentation.md` - Documentation improvement request template
  - `.github/ISSUE_TEMPLATE/config.yml` - Template configuration with Discussions and guide links

- **User Documentation**
  - `docs/FAQ.md` - 31 frequently asked questions covering installation, language features, Godot integration, performance, and roadmap
  - `docs/TROUBLESHOOTING.md` - Platform-specific troubleshooting for Windows (MSVC), macOS (Xcode), Linux (libclang), build errors, Godot integration, and debugging techniques

- **Security Documentation**
  - `SECURITY.md` - Vulnerability reporting policy with GitHub Security Advisories, 48-hour response time, coordinated disclosure, and security best practices

- **Architecture Documentation**
  - `docs/ARCHITECTURE.md` - 917 lines of comprehensive system design covering compiler pipeline (lexer→parser→type checker), runtime execution, Godot integration (GDExtension + TLS), design decisions, extension points, performance characteristics

- **Enhanced Examples**
  - `examples/hello/README.md` - Hello world tutorial with 4 variations covering FerrisScript basics, lifecycle, and common gotchas
  - `examples/move/README.md` - Movement tutorial with 6 variations explaining framerate-independent movement, delta time, and property access
  - `examples/bounce/README.md` - Bouncing animation tutorial with 6 variations demonstrating global variables, state management, and boundary checks

- **Documentation Infrastructure**
  - `.github/workflows/docs-lint.yml` - Automated documentation linting CI with markdownlint and link checking
  - `.markdownlint.json` - Markdown linting rules (50+ configured)
  - `.markdown-link-check.json` - Link validation configuration
  - `package.json` - Documentation linting scripts (`docs:lint`, `docs:fix`)

#### Error Handling Improvements (PR #12, #13, Phases 2-3)

- **Enhanced Error Messages** (38 total: 6 lexer + 14 parser + 18 type checker)
  - All errors now include line and column numbers
  - Source context display with ±2 lines around error
  - Visual indicators pointing to exact error location
  - Helpful hints for common mistakes
  - "did you mean?" suggestions for typos
  - Example corrections showing proper syntax

- **Error Message Testing**
  - 17 new error context validation tests
  - 11 error context module tests
  - Error message quality verification in test suite

#### Testing & Quality (PR #7, #11, Code Quality Improvements)

- **Test Coverage Expansion**
  - 96 → 116 tests (+20 tests, +20.8% growth)
  - 65-70% → 70-75% line coverage (+5% improvement)
  - 50-55% → 55-60% branch coverage (+5% improvement)

- **Edge Case Testing** (15 new tests in 3 categories)
  - Empty script files: 4 tests
  - Comment-only scripts: 5 tests
  - Long identifier names: 6 tests (up to 10,000 chars)

- **Coverage Infrastructure**
  - `scripts/coverage.sh` - Linux coverage generation script
  - `scripts/coverage.ps1` - Windows coverage generation script
  - `.github/workflows/coverage.yml` - CI coverage reporting with cargo-tarpaulin
  - `docs/COVERAGE_SETUP_NOTES.md` - Platform-specific coverage tool setup
  - `docs/archive/v0.0.2/TEST_COVERAGE_ANALYSIS.md` - Detailed coverage analysis and gaps

- **Benchmarking**
  - `BENCHMARK_BASELINE.md` - Performance baseline metrics
  - Lexer benchmarks: 384 ns - 3.74 μs
  - Parser benchmarks: 600 ns - 7.94 μs
  - Type checker benchmarks: 851 ns - 3.58 μs
  - Runtime execution: 1.05 μs per call, 180 ns per loop iteration
  - Analysis: 16K+ function calls per frame at 60 FPS (excellent for game scripting)

#### API Documentation (PR #15, #16, Phases 4A-4B)

- **Rustdoc Comments** (395+ lines added)
  - Compiler crate: Complete rustdoc for all public APIs (lexer, parser, type checker, AST, errors)
  - Runtime crate: Complete rustdoc for Value, Env, execute, call_function
  - Performance metrics documented throughout
  - Code examples in doc comments
  - All doctests passing (0 warnings)

- **Documentation Files**
  - `docs/archive/v0.0.2/phases/PHASE_4A_COMPLETION_SUMMARY.md` - API documentation phase summary
  - `docs/archive/v0.0.2/phases/PHASE_4B_RUSTDOC_SUMMARY.md` - Rustdoc implementation summary

#### GitHub Project Setup (PR #17, Phase 5A)

- **GitHub Features**
  - `docs/GITHUB_PROJECT_MANAGEMENT.md` - CI/CD optimization, label system (20 labels), milestone planning, wiki decisions
  - `docs/GITHUB_INSIGHTS_DESCRIPTION.md` - Repository descriptions, topics, social preview guidance
  - `docs/GITHUB_BADGES_GUIDE.md` - Badge selection and setup guide
  - `docs/GITHUB_LABELS.md` - Complete label system with priorities, types, statuses, difficulties, components

- **Branch Protection**
  - `docs/BRANCH_PROTECTION.md` - Branch protection recommendations and rationale

#### VS Code Extension (PR #18, Phase 5B)

- **Syntax Highlighting Foundation**
  - `extensions/vscode/syntaxes/ferrisscript.tmLanguage.json` - TextMate grammar for `.ferris` files
  - Complete keyword highlighting (let, mut, fn, if, else, while, return, self, etc.)
  - Type highlighting (i32, f32, bool, String, Vector2, Node)
  - Operator highlighting (arithmetic, comparison, logical, assignment)
  - String and comment support

- **VS Code Extension Manifest**
  - `extensions/vscode/package.json` - Extension metadata
  - `extensions/vscode/README.md` - Extension documentation
  - `extensions/vscode/CHANGELOG.md` - Extension changelog
  - `extensions/vscode/language-configuration.json` - Auto-closing brackets, comments, folding

- **Code Snippets** (11 total)
  - Common patterns: function, if, if-else, while, let, let mut, return
  - Godot-specific: _ready(),_process(delta), print()

- **Maintenance Guide**
  - `docs/SYNTAX_HIGHLIGHTING_MAINTENANCE.md` - 350+ lines covering grammar updates, quarterly audits, testing procedures

#### Documentation Polish (PR #19, Phase 5C)

- **Testing Guide** (Archived to `docs/archive/v0.0.2/`)
  - 655-line comprehensive testing guide created for v0.0.2
  - Quick start commands
  - Test naming conventions and templates
  - Test coverage goals and generation methods
  - Error message testing framework
  - Test categories (core, edge cases, errors, integration, performance)
  - Testing workflows (before PR, during dev, after merge)
  - Troubleshooting common test issues
  - Future testing roadmap (v0.0.3-v0.1.0)
  - Current testing practices now in `CONTRIBUTING.md`

- **Godot Test Project Documentation**
  - `godot_test/README.md` - Enhanced with "Adding New Test Scripts" section
  - Step-by-step test script creation guide
  - Reusable test script template
  - Testing best practices (5 key principles)
  - Common test patterns (position, state, conditionals)
  - Updated version info (v0.0.2, October 5, 2025)

#### Version Planning & Workflow (PR #6, #8, #9)

- **Strategic Planning**
  - `docs/VERSION_PLANNING.md` - Version strategy overview and semantic versioning
  - `docs/planning/v0.0.2-roadmap.md` - v0.0.2 detailed roadmap
  - `docs/planning/v0.0.3-roadmap.md` - v0.0.3 (Editor Experience Alpha)
  - `docs/planning/v0.0.4-roadmap.md` - v0.0.4 (Godot API Expansion)
  - `docs/planning/v0.0.5-roadmap.md` - v0.0.5 (LSP Alpha - CRITICAL)
  - `docs/planning/v0.0.6-7-roadmap.md` - v0.0.6-7 (Language Features)
  - `docs/planning/v0.1.0-release-plan.md` - v0.1.0 major release plan

- **Documentation Organization**
  - `docs/v0.0.2/` - Version-specific documentation folder
  - `docs/DOCUMENTATION_INVENTORY.md` - Complete documentation audit
  - `docs/DOCUMENTATION_ORGANIZATION.md` - Documentation structure guide
  - `docs/SINGLE_SOURCE_OF_TRUTH.md` - Anti-duplication matrix
  - `docs/DOCUMENTATION_LINKING_GUIDELINES.md` - Cross-linking best practices

- **Workflow Documentation**
  - `docs/archive/v0.0.2/v0.0.2-DOCUMENTATION-WORKFLOW.md` - 40+ page phase-by-phase guide
  - `docs/archive/v0.0.2/v0.0.2-QUICK-START.md` - Quick start for v0.0.2 work
  - `docs/archive/v0.0.2/v0.0.2-CHECKLIST.md` - Complete task checklist
  - `.github/prompts/workstream-execution.prompt.md` - AI-assisted workstream execution template

- **Phase Completion Reports**
  - `docs/archive/v0.0.2/phases/PHASE_2_COMPLETION_REPORT.md` - Error handling phase 2 summary
  - `docs/archive/v0.0.2/phases/PHASE_3_COMPLETION_REPORT.md` - Error handling phase 3 summary
  - `docs/archive/v0.0.2/phases/PHASE_4_COMPLETION_REPORT.md` - Community docs phase summary
  - `docs/archive/v0.0.2/phases/PHASE_5A_GITHUB_SETUP_SUMMARY.md` - GitHub setup phase summary
  - `docs/archive/v0.0.2/phases/PHASE_5B_SYNTAX_HIGHLIGHTING_SUMMARY.md` - Syntax highlighting phase summary
  - `docs/archive/v0.0.2/phases/PHASE_5C_DOCUMENTATION_POLISH_SUMMARY.md` - Documentation polish phase summary

#### Tooling & Scripts

- **Development Scripts**
  - `scripts/README.md` - Script documentation and usage guide
  - `scripts/coverage.sh` - Linux/macOS coverage generation
  - `scripts/coverage.ps1` - Windows coverage generation (PowerShell)
  - Cross-platform script support (Bash + PowerShell)

### Changed

- **Issue Template Configuration**
  - `.github/ISSUE_TEMPLATE/config.yml` - Clarified Discussions categories (Q&A, Ideas, Show and Tell)

- **README Updates**
  - Added dedicated Examples section with links to detailed tutorials
  - Updated test count from 96 to 116
  - Added coverage metrics (70-75% line coverage)

- **Clippy Compliance**
  - Resolved `collapsible_match` warning in `runtime/src/lib.rs`
  - All code now passes `clippy` with `-D warnings` (deny warnings mode)
  - Zero clippy warnings across entire workspace

### Fixed

- **Installation Instructions**
  - Corrected `cd` command to match repository directory name (`FerrisScript` vs `ferrisscript`) for case-sensitive file systems (Linux/macOS)

- **README Corruption**
  - Removed 255 lines of corrupted duplicate source code appended after closing message

- **Documentation Linting**
  - Fixed 100% of markdown formatting issues across entire repository
  - Resolved dead links in documentation
  - Updated Rust grammar URLs in syntax highlighting docs
  - Fixed relative paths in extension documentation

- **SonarCloud Integration**
  - Added code quality analysis integration (PR #14)

### Performance

- **Benchmarks Established**
  - Baseline metrics for lexer, parser, type checker, runtime documented
  - Function call overhead: ~1 μs (excellent for game scripting)
  - 16K+ function calls per frame possible at 60 FPS

### Documentation

- **Total Documentation Added**: 10,000+ lines across 50+ files
- **Test Count**: 96 → 116 (+20 tests)
- **Coverage**: 65-70% → 70-75% line coverage
- **PRs Merged**: 17 PRs (PR #3-19)
- **Clippy Warnings**: Resolved all warnings

### Deferred to Future Versions

See `docs/archive/v0.0.2/V0.0.2_DEFERRAL_ANALYSIS.md` for complete deferral analysis:

- **v0.0.3**: Enhanced error diagnostics, development scripts, VS Code marketplace, CI optimizations (17 items)
- **v0.0.4**: GODOT_INTEGRATION.md, signal support docs, Godot screenshots (8 items)
- **v0.0.5+**: Type system refinements, performance optimizations, LSP foundation (27 items)

---

## Upgrade Notes

### From v0.0.1 to v0.0.2

**No Breaking Changes** - v0.0.2 is fully backward compatible with v0.0.1.

**New Features Available**:

- Improved error messages with source context
- VS Code syntax highlighting (install from `extensions/vscode/`)
- Comprehensive testing practices documented in `CONTRIBUTING.md`
- Enhanced example tutorials in `examples/*/README.md`

**For Contributors**:

- Read `CONTRIBUTING.md` before submitting PRs (includes testing best practices)
- Use `npm run docs:fix` before committing documentation changes
- Test coverage target: 70-75% (use `scripts/coverage.sh` or `scripts/coverage.ps1`)

**Documentation Updates**:

- All documentation now follows markdown linting rules
- Documentation organized by version in `docs/v0.0.2/`
- Strategic roadmap available in `docs/planning/`

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
