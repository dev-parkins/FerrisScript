@AGENTS.md

## Claude-Specific Notes

### Working with FerrisScript

- **Always run quality checks before committing**: `cargo test --workspace && cargo clippy --workspace --all-targets --all-features -- -D warnings && cargo fmt --all -- --check`
- **The compiler is the most complex crate** — read `docs/ARCHITECTURE.md` sections on lexer/parser/type checker before modifying
- **FerrisScript is alpha (v0.0.5)** — breaking changes are acceptable with proper error messages and CHANGELOG entries
- **Integration tests require Godot** — use `ferris-test --all` for end-to-end validation, but unit tests (`cargo test`) are sufficient for most changes

### Common Pitfalls

- **Never panic in the compiler** — always return `Result<T, FerrisError>` with proper error codes
- **Test names must be descriptive** — `test_type_mismatch_string_to_i32` not `test_types`
- **Error codes are semantic** — check `docs/ERROR_CODES.md` before allocating new codes
- **GDExtension API version matters** — check `crates/godot_bind/Cargo.toml` for the current `api-4-X` feature flag

### When to Load Skills

Before working in a specific area, load the relevant skill from `docs/agent-skills/`:

- Modifying parser/type checker → `compiler-conventions.md`
- Modifying interpreter/runtime → `runtime-patterns.md`
- Modifying Godot integration → `godot-bindings.md`
- Adding tests → `testing-guide.md`

### Project Context

- **Solo maintainer** (dev-parkins) — be proactive with quality checks
- **843+ tests** across 4 crates — maintain or improve coverage
- **8-month dormancy** (Oct 2025 - Jul 2026) — recent work focused on gdext 0.5.4 upgrade and stabilization
- **Roadmap**: v0.0.6 (LSP foundation), v0.0.7 (arrays/for loops), v0.1.0 (metadata system)
