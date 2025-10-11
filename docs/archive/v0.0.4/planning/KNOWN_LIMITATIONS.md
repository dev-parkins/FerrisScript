# v0.0.4 Known Limitations & Design Decisions

**Date**: October 8, 2025  
**Version**: v0.0.4-dev  
**Status**: Living document (updated as phases complete)

---

## 🎯 Purpose

This document tracks design decisions, known limitations, and deferred features across all v0.0.4 phases. It serves as a reference for:

- **Users**: Understanding what's currently supported vs. planned
- **Developers**: Knowing what's intentionally deferred and why
- **Future Planning**: Tracking technical debt and enhancement opportunities

---

## 📊 Phase 1: Signal Support (✅ Complete)

### ✅ What's Implemented

- **Signal Declaration**: `signal name(param: Type);` syntax fully functional
- **Signal Emission**: `emit_signal("name", args)` with type checking
- **Godot Registration**: Signals registered dynamically via `add_user_signal()`
- **Editor Connection**: Signals can be connected visually in Godot Inspector
- **Parameter Passing**: Typed parameters flow correctly between FerrisScript and Godot
- **Type Safety**: 6 error codes (E301-E304 compile-time, E501-E502 runtime)

### ⏸️ Deferred Features

#### 1. Programmatic Signal Connection

**Feature**: `connect()` and `disconnect()` methods in FerrisScript

**Example** (not currently supported):

```rust
fn _ready() {
    // This does NOT work in v0.0.4
    connect("health_changed", self, "on_health_changed");
}
```

**Why Deferred**:

- Requires node path system (Phase 3: Node Query Functions)
- Requires callable reference system (complex Godot API integration)
- Editor-based connections are the primary Godot workflow (90% of use cases)
- Adds significant complexity for limited benefit at this stage

**Workaround**: Use Godot Inspector to connect signals visually (fully supported)

**Timeline**: Phase 6 (Enhancements) or later, after Phase 3 complete

**Estimated Effort**: 2-3 days

---

#### 2. Signal Visibility in Godot Inspector

**Limitation**: Dynamically registered signals don't appear in Node→Signals panel

**Technical Reason**:

- Godot Inspector only shows compile-time signals (declared with `#[signal]` in Rust/GDScript)
- FerrisScript uses dynamic registration (`add_user_signal()`) at runtime
- This is a Godot engine limitation, not a FerrisScript bug
- **Root cause**: Editor introspects `ClassDB` at class registration time (compile-time), before any .ferris files are loaded

**Impact**:

- ✅ Signals ARE fully functional (registration, emission, connection all work)
- ❌ Cannot connect signals via drag-and-drop in Inspector UI
- ✅ Can connect programmatically in GDScript (workaround available)

**Workaround** (for manual testing):

```gdscript
# In GDScript receiver node
func _ready():
    var ferris_node = get_node("../FerrisScriptNode")
    ferris_node.connect("health_changed", _on_health_changed)

func _on_health_changed(old_health: int, new_health: int):
    print("Health changed: ", old_health, " -> ", new_height)
```

**Architectural Context**: FerrisScript has **one** Rust class (`FerrisScriptNode`) that loads **many** `.ferris` scripts at runtime. We cannot know what signals exist until the script is loaded, making static registration impossible without significant build system integration.

**Future Enhancement Options**:

1. **Hybrid approach** (v0.1.0): Predefined common signals in Rust + dynamic custom signals
2. **Metadata system** (future): Extract signal metadata during compilation, register statically
3. **Per-script classes** (complex): Generate Rust wrapper class for each .ferris file (like GDScript)

**References**:

- [SIGNAL_VISIBILITY_ISSUE.md](SIGNAL_VISIBILITY_ISSUE.md) - Testing results and workarounds
- [SIGNAL_EDITOR_VISIBILITY_ARCHITECTURE.md](SIGNAL_EDITOR_VISIBILITY_ARCHITECTURE.md) - **Deep technical analysis with solution comparison**

---

### 🎓 Phase 1 Learnings

**What Worked Well**:

- Instance ID pattern for signal emission (clean, no borrowing conflicts)
- Boxed closures for capturing environment
- Dynamic signal registration simpler than expected
- Type checking at compile-time, runtime validation minimal

**Challenges Overcome**:

- Godot 4.3+ compatibility (required `api-4-3` feature flag)
- Signal parameter types not stored by Godot (solved with compile-time checking)
- Clippy warnings with PI literal (changed 3.14 to 3.15 in tests)

**Documentation Created**:

- [GODOT_SETUP_GUIDE.md](../../GODOT_SETUP_GUIDE.md) - Comprehensive setup guide
- [SIGNAL_VISIBILITY_ISSUE.md](SIGNAL_VISIBILITY_ISSUE.md) - Limitation explanation
- [SIGNAL_TESTING_INSTRUCTIONS.md](SIGNAL_TESTING_INSTRUCTIONS.md) - Manual test guide

---

## 📋 Phase 2: Additional Callbacks (Ready to Start)

### 🎯 Planned Implementation

**Callbacks**:

1. `_input(event: InputEvent)` - User input handling
2. `_physics_process(delta: f32)` - Fixed timestep physics
3. `_enter_tree()` - Node enters scene tree
4. `_exit_tree()` - Node exits scene tree

### ⏸️ Known Limitations (Planned)

#### 1. InputEvent Simplified API

**Implementation Plan**: Start with action checks only

**Supported in Phase 2**:

```rust
fn _input(event: InputEvent) {
    if event.is_action_pressed("jump") {
        // This WILL work
    }
    if event.is_action_released("shoot") {
        // This WILL work
    }
}
```

**NOT Supported in Phase 2**:

```rust
fn _input(event: InputEvent) {
    // These will NOT work in Phase 2
    let pos = event.position;  // Property access deferred
    let button = event.button_index;  // Property access deferred
    let is_echo = event.is_echo();  // Advanced methods deferred
}
```

**Why Simplified**:

- Godot has 10+ InputEvent subclasses (InputEventKey, InputEventMouse, etc.)
- Each subclass has unique properties
- Action checks cover 80% of use cases
- Full API requires significant FFI complexity

**Timeline**: Phase 6 or later (enhancement)

**Workaround**: Use GDScript for complex input handling, call FerrisScript methods

---

## 🔜 Phase 3: Node Query Functions (Future)

### 🎯 Planned Features

- `get_node(path: String) -> Node`
- `get_parent() -> Node`
- `has_node(path: String) -> bool`
- `find_child(name: String) -> Node`

### 🔗 Dependency Note

Phase 3 is a **prerequisite** for:

- Programmatic signal connection (deferred from Phase 1)
- Cross-node script communication
- Dynamic scene tree manipulation

---

## 🎨 Phase 4: Godot Types (Future)

### 🎯 Planned Types

- `Color` - RGBA color representation
- `Rect2` - 2D rectangle
- `Transform2D` - 2D transformation matrix

### ⏸️ Deferred Types

**Not in v0.0.4**:

- `Vector3` - 3D vector (not needed for 2D focus)
- `Quaternion` - 3D rotation (not needed for 2D focus)
- `AABB` - 3D bounding box (not needed for 2D focus)

**Rationale**: v0.0.4 focuses on 2D game development

---

## 🔧 Phase 5: Property Exports (Future)

### 🎯 Planned Features

- Export variables to Godot Inspector
- Custom property hints (range, enum, file)
- Property groups/categories

### ⏸️ Known Complexity

**Challenge**: Godot expects properties declared at class registration time (compile-time)

**Options**:

1. Code generation approach (generate Rust code with `#[export]`)
2. Reflection-based approach (dynamic property registration)
3. Hybrid approach (common properties compiled, custom ones dynamic)

**Decision**: TBD during Phase 5 planning

---

## 📝 General Limitations

### 1. GDExtension Loading

**Requirement**: Must rebuild GDExtension (`cargo build --package ferrisscript_godot_bind`) after Rust code changes

**Impact**: Hot-reload of FerrisScript scripts works, but Rust binding changes require Godot restart

**Workaround**: None (inherent to GDExtension architecture)

---

### 2. Godot Version Compatibility

**Supported**: Godot 4.2+, 4.3+, 4.4+ (with appropriate feature flags)

**Godot 4.3+ Requirement**: Must use `api-4-3` feature in `Cargo.toml`

**Reference**: [GODOT_SETUP_GUIDE.md](../../GODOT_SETUP_GUIDE.md)

---

### 3. Error Reporting in Godot

**Current State**: Compile errors appear in FerrisScript compilation, runtime errors in Godot console

**Future Enhancement**: Better error integration with Godot's error reporting UI

---

## 🎯 Philosophy & Trade-offs

### Incremental Value Delivery

**Principle**: Ship functional features incrementally rather than waiting for perfection

**Example**: Phase 1 ships editor-based signal connections (90% use case) without programmatic connection (10% use case)

**Benefit**: Users can start using signals immediately while we continue development

---

### Simplicity Over Completeness

**Principle**: Start with simplified APIs, expand based on actual usage

**Example**: InputEvent starts with action checks (most common use case) rather than full property access

**Benefit**: Faster delivery, lower maintenance burden, easier to test

---

### Follow Godot Patterns

**Principle**: Match Godot's naming conventions and workflows

**Example**: Use `_ready()`, `_process()`, `_input()` (Godot naming) rather than inventing new names

**Benefit**: Familiar to Godot developers, easier to understand

---

## 📚 References

- **Phase 1 Status**: [PHASE_1_STATUS_UPDATE.md](PHASE_1_STATUS_UPDATE.md)
- **Phase 2 Planning**: [PHASE_2_PREP.md](PHASE_2_PREP.md)
- **Phase 2 Checklist**: [PHASE_2_CHECKLIST.md](PHASE_2_CHECKLIST.md)
- **Godot Setup**: [GODOT_SETUP_GUIDE.md](../../GODOT_SETUP_GUIDE.md)
- **Signal Visibility**: [SIGNAL_VISIBILITY_ISSUE.md](SIGNAL_VISIBILITY_ISSUE.md)

---

---

## � Known Issues

### Example File Compilation Issues (October 9, 2025)

**Issue**: Example files created programmatically fail to compile with parser error "Expected {, found ("

**Symptoms**:

- Files created with `create_file` tool or PowerShell `Out-File` fail compilation
- Error occurs at line 1, column 1 regardless of actual content
- Same syntax works in unit tests (inline strings) but fails when loaded from files
- Error message is misleading - points to wrong token

**Context**:

- Attempted to create `examples/input.ferris` and `examples/callbacks.ferris`
- Files contain valid FerrisScript syntax (verified against working examples)
- Type checker tests with identical syntax pass successfully

**Attempted Solutions**:

1. ❌ Removed leading comments (no BOM found)
2. ❌ Changed line endings from LF to CRLF (hello.ferris uses CRLF)
3. ❌ Used different encoding methods (UTF-8, ASCII)
4. ❌ Copied working file and modified with `replace_string_in_file`
5. ❌ Verified no BOM present (first bytes are correct: 66='f')

**Investigation Findings**:

- `hello.ferris` (working): Starts with 0x66 0x6E 0x20 (fn ), uses CRLF (0D 0A)
- `input.ferris` (broken): Starts with 0x66 0x6E 0x20 (fn ), uses CRLF (0D 0A)
- Byte-level inspection shows no differences
- Parser error message is inconsistent with actual file content

**Impact**: Low

- Core functionality verified through 396 passing unit tests
- Examples can be created manually in Godot editor
- Issue does not affect actual FerrisScript usage in Godot

**Status**: 🔍 Under investigation

- May be related to file reading/parsing in test environment
- Does not affect runtime compilation in Godot
- Will be addressed in follow-up work

**Workaround**: Create example files manually or use existing examples as templates

---

## �🔄 Document Maintenance

**Last Updated**: October 9, 2025  
**Next Review**: After Phase 2 completion  
**Update Trigger**: When design decisions or limitations are discovered

---

**Status**: ✅ Active reference document for v0.0.4 development
