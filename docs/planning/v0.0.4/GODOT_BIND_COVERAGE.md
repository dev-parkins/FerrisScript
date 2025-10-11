# Godot Bind Coverage Limitation

**Status**: Documentation  
**Date**: October 9, 2025  
**Context**: Phase 2 Coverage Improvement

## Overview

The `godot_bind` crate has 0% unit test coverage according to Codecov. This is **by design**, not a testing gap.

## Why Unit Tests Are Not Practical

### Technical Requirements

The `godot_bind` crate provides the GDExtension integration layer that:

1. Registers the `FerrisScriptNode` class with Godot's ClassDB
2. Implements Godot lifecycle callbacks (`_ready`, `_process`, `_input`, etc.)
3. Loads and executes `.ferris` scripts through the runtime
4. Bridges between Godot's type system and FerrisScript's runtime

**Key Constraint**: All of these operations require a running Godot editor or game engine.

### What Would Be Required

To unit test `godot_bind`, we would need:

```rust
// This is NOT possible in cargo test
#[test]
fn test_ferrisscript_node_ready() {
    // ❌ Error: No Godot engine running
    let node = FerrisScriptNode::new();
    node.ready();  // Crashes - needs Godot runtime
}
```

Requirements for genuine testing:

- A **running Godot 4.x instance**
- The **GDExtension library loaded**
- A **valid Godot scene tree**
- **File system access** to `.ferris` scripts
- **Godot API bindings** fully initialized

### Alternative Testing Strategy

| Test Type | Location | Coverage | Purpose |
|-----------|----------|----------|---------|
| **Unit Tests** | `compiler/` & `runtime/` | 396 tests | Core logic validation |
| **Integration Tests** | Manual Godot testing | Phase 3+ | End-to-end validation |
| **Example Projects** | `godot_test/` folder | Manual | Real-world usage |

## Integration Testing Approach

### Current Setup

```
godot_test/
├── project.godot
├── ferrisscript.gdextension
├── test_scene.tscn
└── scripts/
    ├── hello.ferris
    ├── move_test.ferris
    └── bounce_test.ferris
```

### Testing Process

1. **Build the Extension**:

   ```bash
   cargo build --package ferrisscript_godot_bind
   ```

2. **Open Godot Project**:

   ```bash
   cd godot_test
   godot --editor
   ```

3. **Manual Testing**:
   - Load `test_scene.tscn`
   - Attach FerrisScript Node
   - Set `script_path` property
   - Run scene and verify behavior

### Tested Features

✅ **Manually Validated**:

- Node registration with Godot ClassDB
- Script loading from `.ferris` files
- `_ready()` callback execution
- `_process(delta)` callback execution
- `_input(event)` callback execution
- `_physics_process(delta)` callback execution
- `_enter_tree()` callback execution
- `_exit_tree()` callback execution
- Signal emission from scripts
- Error reporting to Godot console

## Coverage Exclusion Rationale

### codecov.yml Configuration

```yaml
coverage:
  status:
    project:
      default:
        target: 80%
        paths:
          - "crates/compiler/"
          - "crates/runtime/"
        # Intentionally excludes crates/godot_bind/
```

**Justification**:

1. **Core Logic Separation**: Compiler and runtime are thoroughly unit-tested (396 tests)
2. **Integration Boundary**: godot_bind is a thin integration layer with minimal logic
3. **Testing ROI**: Setting up automated Godot testing infrastructure has low return on investment for this project phase
4. **Manual Validation**: Integration testing catches issues that unit tests would miss anyway

### What godot_bind Does NOT Contain

❌ **NOT in godot_bind** (these ARE unit-tested):

- FerrisScript parsing logic → `compiler`
- Type checking logic → `compiler`
- Runtime execution logic → `runtime`
- Signal registration logic → `runtime`
- Value coercion logic → `runtime`

✅ **Only in godot_bind** (requires Godot):

- GDExtension class registration boilerplate
- Godot → Rust callback forwarding
- File loading through Godot's `FileAccess` API
- Property exposure to Godot Inspector

## Future Automation Options

### Option 1: Godot Headless Testing (v0.2.0+)

```rust
// Requires godot-rust test harness (future work)
#[gdtest]
fn test_script_loading() {
    let node = FerrisScriptNode::new();
    node.set_script_path("res://test.ferris".into());
    node.ready();
    assert!(node.script_loaded());
}
```

**Pros**: Automated, runs in CI  
**Cons**: Complex setup, requires `gdtest` framework

### Option 2: Mock Godot Bindings (Low Priority)

```rust
// Create mock GDExtension types for testing
struct MockNode { /* ... */ }
```

**Pros**: Runs in `cargo test`  
**Cons**: Tests mocks, not real integration, high maintenance

### Option 3: Property-Based Testing (v0.3.0+)

Generate random FerrisScript programs and verify they load/execute without crashing.

**Pros**: Catches edge cases  
**Cons**: Doesn't test Godot-specific behavior

## Recommendation

**Current Approach (v0.0.4)**: ✅ **Acceptable**

- Focus unit testing on `compiler/` and `runtime/` (80%+ coverage)
- Manual integration testing for `godot_bind`
- Document tested features in integration test logs
- Defer automated GDExtension testing until Phase 5 or v0.2.0

## Coverage Report Interpretation

When viewing Codecov reports:

| File | Expected Coverage | Reason |
|------|------------------|--------|
| `crates/compiler/` | **85%+** | Fully unit-testable |
| `crates/runtime/` | **80%+** | Fully unit-testable |
| `crates/godot_bind/` | **0-10%** | Integration-only code |

**Total Project Coverage**: Should be calculated excluding `godot_bind`:

```
Coverage = (compiler_lines + runtime_lines) / (total_lines - godot_bind_lines)
```

## References

- **Phase 2 Checklist**: `PHASE_2_CHECKLIST.md` - Manual testing verification
- **Example Tests**: `godot_test/scripts/` - Real-world integration scenarios
- **CI Configuration**: `.github/workflows/ci.yml` - Automated unit tests only

## Related Issues

- Future: [#TBD] Set up automated GDExtension testing with godot-rust
- Future: [#TBD] Create headless Godot test runner for CI
- Current: Manual integration testing process documented in DEVELOPMENT.md

---

**Conclusion**: The 0% coverage on `godot_bind` is **not a quality concern**. The critical logic is tested in `compiler` and `runtime` crates. Integration behavior is validated manually through the `godot_test` project.
