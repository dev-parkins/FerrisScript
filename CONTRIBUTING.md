# Contributing to FerrisScript

First off, thank you for considering contributing to FerrisScript! 🦀 It's people like you that make FerrisScript a great tool for bringing Rust-like syntax to Godot development.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [What Should I Know Before I Get Started?](#what-should-i-know-before-i-get-started)
- [How Can I Contribute?](#how-can-i-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Features](#suggesting-features)
  - [Contributing Documentation](#contributing-documentation)
  - [Contributing Code](#contributing-code)
- [Development Environment Setup](#development-environment-setup)
- [Pull Request Process](#pull-request-process)
- [Code Style Guidelines](#code-style-guidelines)
- [Testing Guidelines](#testing-guidelines)
- [First-Time Contributors](#first-time-contributors)
- [Community](#community)

## Code of Conduct

This project and everyone participating in it is governed by the [FerrisScript Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## What Should I Know Before I Get Started?

### About FerrisScript

FerrisScript is a Rust-inspired scripting language designed specifically for the Godot game engine. It brings Rust's powerful type system and ownership concepts to game scripting while maintaining ease of use. The project consists of:

- **Compiler** (`crates/compiler/`): Lexer, parser, AST, and type checker
- **Runtime** (`crates/runtime/`): Execution environment for FerrisScript code
- **Godot Binding** (`crates/godot_bind/`): Integration layer with Godot 4.x

### Version Status

- **v0.0.1**: Released October 2, 2025 - Initial compiler and runtime implementation
- **v0.0.2**: In progress - Documentation improvements and community standards

### File Extensions

FerrisScript uses the `.ferris` file extension for all script files (not `.rscr`).

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the [existing issues](https://github.com/dev-parkins/FerrisScript/issues) to avoid duplicates.

When creating a bug report, please include:

- **A clear and descriptive title**
- **Steps to reproduce** the issue
- **Expected behavior** vs **actual behavior**
- **Code samples** demonstrating the issue (if applicable)
- **Environment details**: OS, Rust version, Godot version
- **Error messages** or stack traces

Use the [bug report template](.github/ISSUE_TEMPLATE/bug_report.md) when creating your issue.

### Suggesting Features

Feature suggestions are welcome! Before suggesting a feature:

1. Check if it's already been suggested or implemented
2. Consider if it aligns with FerrisScript's goals (Rust-like syntax for Godot)
3. Think about how it would benefit the community

When suggesting a feature, please include:

- **A clear and descriptive title**
- **Detailed description** of the proposed feature
- **Use cases** showing when/why this would be helpful
- **Code examples** showing how the feature would be used
- **Alternative solutions** you've considered

Use the [feature request template](.github/ISSUE_TEMPLATE/feature_request.md) when creating your issue.

### Contributing Documentation

Documentation improvements are always appreciated! This includes:

- Fixing typos or unclear explanations
- Adding examples to existing documentation
- Writing tutorials or guides
- Improving code comments
- Translating documentation (future)

**Important**: Before contributing documentation:

1. Review the [Anti-Duplication Matrix](docs/SINGLE_SOURCE_OF_TRUTH.md) to ensure you're editing the primary location for content
2. Link to existing documentation rather than duplicating it
3. Follow the structure outlined in [Phase Tracking](docs/v0.0.2/PHASE_TRACKING.md)
4. **Run documentation linting locally** before pushing (see [Documentation Quality Checks](#documentation-quality-checks) below)

Use the [documentation template](.github/ISSUE_TEMPLATE/documentation.md) when creating documentation-related issues.

#### Documentation Quality Checks

**IMPORTANT**: Always run documentation linting locally before pushing to catch issues early!

We use automated tools to ensure documentation quality:

- **markdownlint**: Checks markdown formatting consistency
- **markdown-link-check**: Verifies all links work (internal and external)

**Quick Setup** (first time only):

```bash
npm install
```

**Before Every Documentation Commit**:

```bash
# Option 1: VS Code Task (Recommended)
# Press Ctrl+Shift+P, type "Run Task", select "Docs: Full Check"

# Option 2: npm script
npm run docs:check

# Option 3: PowerShell script
.\scripts\lint-docs.ps1

# To auto-fix formatting issues:
npm run docs:fix
# or
.\scripts\lint-docs.ps1 --fix
```

**What Gets Checked**:

- ✅ Heading hierarchy and style
- ✅ List formatting consistency
- ✅ Code block formatting
- ✅ Line length (soft limit: 120 chars)
- ✅ Trailing whitespace
- ✅ Internal links (broken references)
- ✅ External links (with retries)

**CI Integration**: These same checks run automatically on pull requests. Catching issues locally saves review time!

See [scripts/README.md](scripts/README.md) for detailed documentation linting guide.

### Contributing Code

Code contributions are welcome for:

- Bug fixes
- New features (after discussion in an issue)
- Performance improvements
- Test coverage improvements
- Refactoring for better maintainability

**Before starting work on code**, please:

1. **Open an issue** (or comment on an existing one) to discuss your approach
2. **Wait for feedback** from maintainers to ensure alignment
3. **Create a feature branch** from `main` for your work

## Development Environment Setup

### Prerequisites

Before you begin, ensure you have:

- **Rust 1.70+** (we use 1.90.0 in development)
- **Git** for version control
- **A text editor or IDE** (VS Code with rust-analyzer recommended)

For detailed installation instructions, see the [README.md Installation section](README.md#installation).

**Do not duplicate installation instructions here** - always link to the README.md as the single source of truth.

### Setting Up Your Development Environment

1. **Fork and clone the repository**:

   ```bash
   # Fork via GitHub UI, then:
   git clone https://github.com/YOUR_USERNAME/FerrisScript.git
   cd FerrisScript
   ```

2. **Add the upstream remote**:

   ```bash
   git remote add upstream https://github.com/dev-parkins/FerrisScript.git
   ```

3. **Build the project**:

   ```bash
   cargo build
   ```

4. **Run the tests** to verify your setup:

   ```bash
   cargo test
   ```

All 96 tests should pass. If they don't, please open an issue.

### Running Examples

To verify your environment setup, try running an example:

```bash
# Build the project
cargo build --release

# Run an example
cargo run --bin rustyscript_runtime examples/hello.ferris
```

You should see "Hello from FerrisScript!" printed to the console.

## Pull Request Process

We use a **feature branch workflow** with **squash and merge** strategy.

### Creating a Pull Request

1. **Create a feature branch** with a descriptive name:

   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/bug-description
   ```

2. **Make your changes** in small, logical commits:

   ```bash
   git add .
   git commit -m "feat: add new feature"
   # or
   git commit -m "fix: resolve issue with parser"
   ```

3. **Keep your branch up to date**:

   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

4. **Push your branch**:

   ```bash
   git push origin feature/your-feature-name
   ```

5. **Open a Pull Request** via GitHub:
   - Use a clear, descriptive title
   - Fill out the [PR template](.github/PULL_REQUEST_TEMPLATE.md)
   - Reference related issues (e.g., "Closes #42")
   - Describe what changed and why

### PR Requirements

Before your PR can be merged:

- ✅ All tests must pass (`cargo test`)
- ✅ Code must be formatted (`cargo fmt`)
- ✅ Code must pass linting (`cargo clippy`)
- ✅ Documentation must be updated (if applicable)
- ✅ CHANGELOG.md must be updated (see below)
- ✅ At least one maintainer approval

### Merge Strategy

- **Feature branches**: We use **squash and merge** to keep main branch history clean
- **Hotfix branches**: We use **merge commit** to preserve context
- **Branch deletion**: Branches are automatically deleted after merge (enable in your fork's settings)

### Draft Pull Requests

You can open a PR early as a **draft** to get feedback:

- Mark as draft in the PR creation UI
- Or add `[WIP]` to the title
- Add a "Notes to Reviewers" section explaining current status

### Updating CHANGELOG.md

For significant changes, update the `[Unreleased]` section in CHANGELOG.md:

```markdown
## [Unreleased]

### Added
- Your new feature description

### Fixed
- Your bug fix description

### Changed
- Your modification description
```

Follow the [Keep a Changelog](https://keepachangelog.com/) format.

## Code Style Guidelines

### Rust Style

We follow standard Rust conventions:

- **Formatting**: Use `cargo fmt` before committing
- **Linting**: Fix all `cargo clippy` warnings
- **Naming**:
  - `snake_case` for functions, variables, modules
  - `PascalCase` for types, traits, enums
  - `SCREAMING_SNAKE_CASE` for constants
- **Comments**:
  - Use `///` for public API documentation
  - Use `//` for inline comments
  - Explain "why", not "what"

### Code Organization

- Keep functions small and focused (one responsibility)
- Group related functionality into modules
- Use meaningful variable and function names
- Prefer explicit over implicit

### Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types**:

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Formatting, missing semi-colons, etc.
- `refactor`: Code restructuring without behavior change
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples**:

```
feat(parser): add support for match expressions
fix(lexer): handle escaped quotes in strings
docs: update installation instructions for Windows
```

## Testing Guidelines

### Writing Tests

- Every new feature should include tests
- Bug fixes should include regression tests
- Place tests in the same file using `#[cfg(test)]` modules
- Use descriptive test names: `test_parser_handles_nested_functions`

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p rustyscript_compiler

# Run a specific test
cargo test test_lexer_tokenizes_keywords

# Run tests with output
cargo test -- --nocapture
```

### Test Coverage

We aim for high test coverage, especially for:

- Parser and lexer (edge cases, error handling)
- Type checker (all type rules, error messages)
- Runtime (execution correctness, error recovery)

#### Running Code Coverage

FerrisScript uses two code coverage tools for different environments:

- **cargo-llvm-cov** - Preferred for local development (Windows, macOS, Linux)
- **cargo-tarpaulin** - Used in CI (Linux only, due to Windows file locking issues)

**Quick Start - Running Coverage Locally**:

```bash
# PowerShell (Windows)
.\scripts\coverage.ps1

# Bash (Linux/macOS)
./scripts/coverage.sh
```

The scripts will:

1. Automatically install `cargo-llvm-cov` if not present
2. Run coverage analysis across all workspace crates
3. Generate both HTML and LCOV reports in `target/coverage/`

**Viewing Coverage Reports**:

```powershell
# Windows - Open HTML report
Invoke-Item target/coverage/html/index.html

# Linux
xdg-open target/coverage/html/index.html

# macOS
open target/coverage/html/index.html
```

**Manual Coverage Commands**:

```bash
# Install prerequisites (first time only)
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov

# Run coverage with HTML output
cargo llvm-cov --workspace --html --output-dir target/coverage

# Run coverage with LCOV output (for external tools)
cargo llvm-cov --workspace --lcov --output-path target/coverage/lcov.info

# Run coverage for specific crate
cargo llvm-cov -p ferrisscript_compiler --html
```

**Understanding Coverage Results**:

- **Green lines**: Covered by tests
- **Red lines**: Not covered by tests
- **Yellow lines**: Partially covered (branches)
- **Coverage percentage**: Shown per file and overall

**Coverage Goals**:

- **Current baseline**: Established in test coverage analysis
- **Target for new code**: 80%+ line coverage
- **Critical paths**: Parser, type checker, runtime should have high coverage

**Why Two Tools?**

- **cargo-llvm-cov**: Cross-platform, native Rust tooling, no file locking issues on Windows
- **cargo-tarpaulin**: More mature in CI environments, used in GitHub Actions (Linux runners)

See [docs/COVERAGE_SETUP_NOTES.md](docs/COVERAGE_SETUP_NOTES.md) for technical details on the Windows file locking issue that led to this dual-tool approach.

**Before Submitting a PR**:

For significant code changes:

1. Run coverage locally: `.\scripts\coverage.ps1` or `./scripts/coverage.sh`
2. Check that your new code is covered by tests
3. Aim for 80%+ coverage on modified files
4. CI will also run coverage checks using tarpaulin

**Troubleshooting Coverage Issues**:

- **"cargo-llvm-cov not found"**: The script will auto-install it, or run `cargo install cargo-llvm-cov` manually
- **Windows file locking errors**: Close VS Code and rust-analyzer, or use the provided scripts which avoid these issues
- **Coverage seems low**: Ensure tests are actually executing your code paths; add `#[cfg(test)]` module tests
- **CI coverage differs from local**: CI uses tarpaulin (Linux), local uses llvm-cov; minor differences are normal

### Testing with Godot (Deferred)

Currently, Godot integration testing is deferred as it requires manual setup. See [FUTURE_AUTOMATION.md](docs/FUTURE_AUTOMATION.md) for plans to automate this in v0.0.3+.

## First-Time Contributors

New to open source? Welcome! Here's how to get started:

1. **Look for beginner-friendly issues** labeled:
   - `good first issue` - Ideal for newcomers
   - `documentation` - Documentation improvements
   - `help wanted` - We need contributors for these

2. **Start small**:
   - Fix a typo in documentation
   - Add an example to existing docs
   - Write a test for an existing feature

3. **Ask for help**:
   - Comment on the issue to let us know you're working on it
   - Ask questions if you're stuck
   - Request review from maintainers

4. **Learn the workflow**:
   - Fork the repository
   - Make your changes
   - Submit a pull request
   - Respond to feedback

Don't worry about making mistakes - we're here to help! Every contributor started where you are now.

### Resources for New Contributors

- [GitHub's Hello World Guide](https://guides.github.com/activities/hello-world/)
- [Syncing a Fork](https://docs.github.com/en/github/collaborating-with-issues-and-pull-requests/syncing-a-fork)
- [Creating a Pull Request](https://docs.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request)
- [Rust Programming Language Book](https://doc.rust-lang.org/book/)

## Community

### Getting Help

- **Issues**: [GitHub Issues](https://github.com/dev-parkins/FerrisScript/issues) for bugs and feature requests
- **Discussions**: *Coming soon* - for questions, ideas, and community chat
- **Project Documentation**: Check [docs/](docs/) for development workflows and guides

### Recognition

All contributors are recognized in:

- Git commit history
- GitHub contributors page
- Future release notes (for significant contributions)

Thank you for contributing to FerrisScript! 🦀❤️

---

Made with 🦀 and ❤️ for the Godot community
