# FerrisScript

Statically-typed, Rust-inspired scripting language for Godot 4.x game development.

## Tech Stack

- **Rust 1.94+** (edition 2024) — pinned in Cargo.toml
- **Godot 4.2+** (GDExtension API via gdext 0.5.4)
- **Tree-walking interpreter** (not bytecode)
- **`.ferris` file extension** for all scripts
- **Workspace structure**: 4 crates (compiler, runtime, godot_bind, test_harness)

## Commands

```bash
# Build
cargo build --workspace
cargo build --release

# Test
cargo test --workspace
cargo test -p ferrisscript_compiler
cargo test test_name -- --exact

# Quality checks (MUST pass before PR)
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
npm run docs:lint  # For markdown files

# Coverage
./scripts/coverage.sh  # Linux/macOS
.\scripts\coverage.ps1  # Windows

# Integration tests (requires Godot)
ferris-test --all
```

## Architecture

```
crates/compiler/     → Lexer → Parser → Type Checker → AST (543 tests)
crates/runtime/      → Tree-walking interpreter, variable scoping, builtins (110 tests)
crates/godot_bind/   → GDExtension integration, node property access, lifecycle hooks (11 tests)
crates/test_harness/ → Integration test framework (38 tests)

Data flow: .ferris → Lexer → Tokens → Parser → AST → Type Checker → Runtime → Godot
```

**Key conventions:**

- Error codes follow `E0XX` pattern (see `docs/ERROR_CODES.md`)
- Unit tests co-located in each crate's `src/` (inline `#[cfg(test)]` modules)
- Integration tests in `godot_test/scripts/` as `.ferris` files
- All public functions must have doc comments

## Code Style

**Naming:**

- `snake_case` for functions, variables, modules
- `PascalCase` for types, traits, enums
- `SCREAMING_SNAKE_CASE` for constants

**Error handling:**

```rust
// CORRECT
pub fn check_type(expr: &Expression, expected: &Type) -> Result<(), TypeError> {
    if !types_match(expr, expected) {
        return Err(TypeError::Mismatch { ... });
    }
    Ok(())
}

// WRONG — never panic in the compiler
pub fn check_type(expr: &Expression, expected: &Type) {
    if !types_match(expr, expected) {
        panic!("Type mismatch!");  // ❌ Never do this
    }
}
```

**Testing:**

```rust
// CORRECT — descriptive test names
#[test]
fn test_type_mismatch_string_to_i32() {
    let source = "let x: i32 = \"hello\";";
    let result = compile(source);
    assert!(result.is_err());
}

// WRONG — vague test names
#[test]
fn test_types() {
    // ...
}
```

## Boundaries

**Always do:**

- Run `cargo test --workspace` before committing
- Update `CHANGELOG.md` for user-facing changes
- Add tests for new features (minimum: unit tests)
- Use conventional commits: `feat:`, `fix:`, `docs:`, `test:`, `refactor:`, `chore:`

**Ask first:**

- Breaking API changes
- Adding new dependencies
- Modifying GDExtension binding layer
- Changing error code allocations

**Never do:**

- Modify `target/` or `node_modules/`
- Hardcode Godot version numbers — use GDExtension API constants
- Add dependencies without updating `Cargo.toml` workspace.dependencies
- Convert sync patterns to async without discussion
- Commit without running quality checks

## Git Workflow

**Branch naming:**

- `bugfix/` or `fix/` → Bug fixes
- `feature/` or `feat/` → New features
- `docs/` or `doc/` → Documentation

**Commit format:**

```
type(scope): description

feat(parser): add support for array types
fix(runtime): handle division by zero correctly
docs: update installation instructions
```

**PR process:**

1. Create branch from `main`
2. Make changes with conventional commits
3. Run quality checks locally
4. Push and create PR (template auto-applied based on branch name)
5. Squash merge after approval

## Skills (Load on Demand)

Read the relevant skill before working in a specific area:

- **compiler-conventions**: Parser patterns, AST nodes, error codes → `docs/agent-skills/compiler-conventions.md`
- **runtime-patterns**: Interpreter scoping, builtins, evaluation → `docs/agent-skills/runtime-patterns.md`
- **godot-bindings**: GDExtension lifecycle, properties, signals → `docs/agent-skills/godot-bindings.md`
- **testing-guide**: Test harness, coverage, integration tests → `docs/agent-skills/testing-guide.md`

## Additional Context

- **Current version**: v0.0.5 (alpha)
- **Test coverage**: ~82% (target: 80%+)
- **Total tests**: 843+ across all layers
- **CI**: GitHub Actions with cross-platform builds
- **Documentation**: See `docs/ARCHITECTURE.md` for detailed design
