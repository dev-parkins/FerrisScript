# FerrisScript v0.0.4 Roadmap 🦀

**Version**: 0.0.4 (Patch Release)  
**Focus**: Godot API Expansion  
**Timeline**: 3-4 weeks  
**Prerequisites**: v0.0.3 (solid editor + error reporting)

---

## 🎯 Overview

**Strategic Goal**: Expand Godot integration to enable real 2D game development without adding new language features.

**Key Priorities**:

1. Signal support for event-driven programming
2. Additional lifecycle callbacks
3. Node query functions for scene tree interaction
4. Additional Godot types for 2D development

**Alignment with v0.1.0 Strategy**: Major step in reprioritized roadmap by providing comprehensive Godot API coverage before LSP. Enables developers to build real interactive games with current language features.

---

## 📦 High Priority Deliverables

### 1. Signal Support 🔥

**Status**: Not Started  
**Priority**: Critical (Core Godot Feature)

**Rationale**: Essential for event-driven programming in Godot. High priority in reprioritized v0.1.0 roadmap.

**Scope**:

- [ ] Define signals in FerrisScript:

  ```rust
  signal health_changed(old: i32, new: i32);
  signal player_died;
  ```

- [ ] Emit signals from FerrisScript:

  ```rust
  emit_signal("health_changed", old_health, health);
  emit_signal("player_died");
  ```

- [ ] Connect signals from Godot editor
- [ ] Connect signals from FerrisScript code
- [ ] Signal with parameters (multiple types)
- [ ] Signal without parameters
- [ ] Disconnect signals

**Example Usage**:

```rust
signal health_changed(old: i32, new: i32);
signal player_died;

let mut health: i32 = 100;

fn take_damage(amount: i32) {
    let old_health: i32 = health;
    health -= amount;
    emit_signal("health_changed", old_health, health);
    
    if health <= 0 {
        emit_signal("player_died");
    }
}

fn _ready() {
    // Signals can be connected from Godot editor or code
}
```

**Implementation Details**:

- Add `signal` keyword to lexer
- Add signal declaration to parser and AST
- Store signals in environment/runtime
- Integrate with Godot's signal system via GDExtension
- Handle signal parameters and type checking

**Estimated Effort**: 5-7 days

**Components Affected**: Lexer, parser, type checker, runtime, Godot binding

---

### 2. Additional Callbacks

**Status**: Not Started  
**Priority**: High

**Rationale**: Enables input handling and physics, essential for interactive games.

**Scope**:

- [ ] `_input(event: InputEvent)` - Handle user input:

  ```rust
  fn _input(event: InputEvent) {
      if event.is_action_pressed("jump") {
          velocity.y = -300.0;
      }
      if event.is_action_pressed("shoot") {
          spawn_bullet();
      }
  }
  ```

- [ ] `_physics_process(delta: f32)` - Fixed timestep updates:

  ```rust
  fn _physics_process(delta: f32) {
      // Physics calculations here
      self.position.x += velocity.x * delta;
      self.position.y += velocity.y * delta;
      
      // Apply gravity
      velocity.y += 980.0 * delta;
  }
  ```

- [ ] `_enter_tree()` - Node enters scene tree:

  ```rust
  fn _enter_tree() {
      print("Node entered the scene tree");
  }
  ```

- [ ] `_exit_tree()` - Node exits scene tree:

  ```rust
  fn _exit_tree() {
      print("Node exited the scene tree");
      // Cleanup resources
  }
  ```

- [ ] `_draw()` - Custom 2D drawing (optional):

  ```rust
  fn _draw() {
      // Custom drawing code
      draw_circle(position, 10.0, Color::RED);
  }
  ```

**Implementation Details**:

- Add InputEvent type to type system
- Register new callbacks in Godot binding
- Hook into Godot's callback system
- Test each callback thoroughly

**Estimated Effort**: 3-4 days

**Components Affected**: Type system, runtime, Godot binding

---

### 3. Node Query Functions

**Status**: Not Started  
**Priority**: High

**Rationale**: Enables scene tree interaction, essential for accessing other nodes.

**Scope**:

- [ ] `get_node(path: String) -> Node`:

  ```rust
  fn _ready() {
      let player: Node = get_node("../Player");
      let hud: Node = get_node("/root/HUD");
  }
  ```

- [ ] `get_parent() -> Node`:

  ```rust
  fn _ready() {
      let parent: Node = get_parent();
      print("Parent node found");
  }
  ```

- [ ] `get_children() -> [Node]`:

  ```rust
  fn _ready() {
      let children: [Node] = get_children();
      // Note: Requires arrays (may defer to v0.0.6)
      // For now, could return count or implement basic iteration
  }
  ```

- [ ] `has_node(path: String) -> bool`:

  ```rust
  fn _ready() {
      if has_node("Enemy") {
          print("Enemy found!");
      }
  }
  ```

- [ ] `find_child(name: String) -> Node`:

  ```rust
  fn _ready() {
      let health_bar: Node = find_child("HealthBar");
  }
  ```

**Note on Arrays**: `get_children()` returns an array, which requires array support. Options:

1. Defer `get_children()` to v0.0.6 when arrays are implemented
2. Return a special NodeList type with iterator methods
3. Implement basic array support just for this use case

**Recommendation**: Option 1 (defer) to keep v0.0.4 focused and avoid partial array implementation.

**Estimated Effort**: 2-3 days (excluding get_children)

**Components Affected**: Runtime built-ins, Godot binding

---

## 📊 Medium Priority Deliverables

### Additional Godot Types

**Status**: Not Started  
**Priority**: Medium

**Scope**:

- [ ] `Color` - RGBA colors:

  ```rust
  let red: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
  self.modulate = red;
  ```

- [ ] `Rect2` - 2D rectangles:

  ```rust
  let bounds: Rect2 = Rect2 {
      position: Vector2 { x: 0.0, y: 0.0 },
      size: Vector2 { x: 100.0, y: 100.0 }
  };
  ```

- [ ] `Transform2D` - 2D transformations:

  ```rust
  let transform: Transform2D = Transform2D::identity();
  self.transform = transform;
  ```

**Estimated Effort**: 3-4 days

**Components Affected**: Type system, Godot binding

---

### Custom Property Exports

**Status**: Not Started  
**Priority**: Medium

**Rationale**: Expose script variables to Godot Inspector for easy tweaking.

**Scope**:

- [ ] `@export` annotation:

  ```rust
  @export
  let speed: f32 = 100.0;
  
  @export(range: 0.0, 1.0)
  let volume: f32 = 0.5;
  ```

- [ ] Property types: int, float, string, bool
- [ ] Property hints: range, file, enum
- [ ] Read from Inspector
- [ ] Update when changed in Inspector

**Estimated Effort**: 4-5 days

**Components Affected**: Parser, Godot binding

---

## 📝 Additional Tasks from v0.0.2 Deferral

The following items were deferred from v0.0.2 and align with v0.0.4's Godot integration expansion:

### Documentation

**Priority**: Medium

- [ ] **GODOT_INTEGRATION.md creation** - Comprehensive guide covering:
  - Detailed GDExtension setup
  - FerrisScriptNode usage
  - Property exposure
  - Signal handling (v0.0.4 feature)
  - Best practices
  - Common patterns and examples

- [ ] **Godot UI screenshots** - Add screenshots/GIFs showing:
  - FerrisScript node in Godot editor
  - Properties in Inspector
  - Signal connections
  - Error messages in Godot console
  - Example games running

**Rationale**: Much more comprehensive after signal support and additional callbacks. Visual documentation improves onboarding.

**Estimated Effort**: 2-3 days

---

### Testing & Quality Assurance

**Priority**: High

- [ ] **Godot integration end-to-end tests** - Test complete workflows:
  - Script compilation in Godot context
  - Signal emission and connection
  - Callback invocation from Godot
  - Node query functions in scene tree
  - Property exports and Inspector updates
  - Cross-platform Godot builds

- [ ] **GDScript performance comparison** - Benchmark FerrisScript vs GDScript:
  - Simple operations (math, loops, conditions)
  - Godot API calls (get_node, emit_signal)
  - Physics processing overhead
  - Memory usage comparison
  - Compilation time
  - Runtime execution time
  - Document results and analysis

**Rationale**: Need complete Godot integration (signals, callbacks, node queries) to create meaningful tests and comparisons. Performance comparison demonstrates value proposition.

**Estimated Effort**: 3-4 days total

---

**Note**: These deferred items enhance the core v0.0.4 deliverables. Complete signal support, callbacks, and node queries first, then add comprehensive documentation and testing. GODOT_INTEGRATION.md and performance comparisons are valuable for community adoption.

---

## 🎯 Success Metrics

### Quantitative Goals

- [ ] Signals working with parameters
- [ ] All 5 new callbacks implemented and tested
- [ ] 4 node query functions working (defer get_children)
- [ ] 3 new Godot types supported
- [ ] 20-30 new tests added
- [ ] All existing tests passing

### Qualitative Goals

- [ ] Can build simple interactive games (with input)
- [ ] Event-driven programming feels natural
- [ ] Scene tree interaction is intuitive
- [ ] Physics processing works smoothly

---

## 🚫 Out of Scope

The following are explicitly **NOT** included in v0.0.4:

- ❌ Array types (deferred to v0.0.6)
- ❌ For loops (deferred to v0.0.6)
- ❌ Match expressions (deferred to v0.0.6)
- ❌ LSP implementation (deferred to v0.0.5)
- ❌ Resource loading (load scenes, textures)
- ❌ Timer nodes
- ❌ Animation support
- ❌ 3D support
- ❌ Advanced physics (RigidBody2D, etc.)

---

## 📋 Task Breakdown

### Week 1

- Day 1-3: Signal definition and emission
- Day 4-5: Signal connection from Godot
- Day 6-7: Signal parameter handling and testing

### Week 2

- Day 1-2: `_input` callback
- Day 3-4: `_physics_process` callback
- Day 5: `_enter_tree` and `_exit_tree` callbacks
- Day 6-7: Node query functions

### Week 3

- Day 1-2: Additional Godot types (Color, Rect2)
- Day 3-4: Transform2D type
- Day 5-7: Custom property exports

### Week 4 (Buffer/Polish)

- Day 1-2: Integration testing
- Day 3-4: Bug fixes and edge cases
- Day 5: Documentation and examples

---

## 🔗 Dependencies for Next Version

**Enables v0.0.5**:

- ✅ Comprehensive Godot API → Better examples for LSP testing
- ✅ Signals and callbacks → More realistic use cases for LSP
- ✅ Node queries → Complete scripting capabilities for demos

**Critical Path**:

v0.0.3 (Editor Alpha) → v0.0.4 (Godot API) → v0.0.5 (LSP Alpha)

---

## 📝 Notes

### Example Game Enabled by v0.0.4

With v0.0.4, developers can build a simple platformer:

```rust
signal health_changed(new_health: i32);

let mut health: i32 = 3;
let mut velocity: Vector2 = Vector2 { x: 0.0, y: 0.0 };

fn _ready() {
    emit_signal("health_changed", health);
}

fn _input(event: InputEvent) {
    if event.is_action_pressed("jump") {
        velocity.y = -300.0;
    }
}

fn _physics_process(delta: f32) {
    // Apply gravity
    velocity.y += 980.0 * delta;
    
    // Update position
    self.position.x += velocity.x * delta;
    self.position.y += velocity.y * delta;
    
    // Check for ground collision
    if self.position.y > 400.0 {
        self.position.y = 400.0;
        velocity.y = 0.0;
    }
}

fn take_damage() {
    health -= 1;
    emit_signal("health_changed", health);
    
    if health <= 0 {
        // Game over
        let game_over: Node = get_node("/root/GameOver");
        // Show game over screen
    }
}
```

### Release Checklist

- [ ] All tests passing
- [ ] Signal system tested extensively
- [ ] All callbacks working in Godot
- [ ] Node query functions verified
- [ ] Example game created
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version numbers bumped
- [ ] Tag created: v0.0.4
- [ ] GitHub release created

### Communication

- [ ] Announce Godot API expansion
- [ ] Share example game (platformer or similar)
- [ ] Demo signal usage
- [ ] Promote input handling capabilities
- [ ] Request community feedback on API

---

## � Deferred from v0.0.3 - Error Diagnostics & Testing

The following items were deferred from v0.0.3 development and align with v0.0.4's focus on developer experience and quality.

### Phase 2B: Keyword Suggestions 💡

**Status**: Deferred from v0.0.3  
**Priority**: Low  
**Estimated Effort**: 3-4 days

**What It Is**: Suggest keywords when users mistype (e.g., `fnn` → `fn`, `lett` → `let`)

**Why Deferred**: Requires lexer changes for context-aware keyword detection. Phase 2A (variable/function/type suggestions) provides 90% of value. Keyword typos are less common.

**Implementation Notes**:

- Lexer needs to track "almost keywords" (tokens close to keywords)
- Context-aware: `fnn` at statement start → suggest `fn`, but `fnn` in expression → variable typo
- Most keyword typos already caught by parser (unexpected token errors)

**Scope**:

- [ ] Add keyword suggestion infrastructure to lexer
- [ ] Context detection (statement vs expression position)
- [ ] Integration with existing suggestion system
- [ ] Tests for common keyword typos

**Acceptance Criteria**:

```rust
// Example: Typo in keyword
let code = "fnn main() {}";
let result = compile(code);
assert!(result.unwrap_err().contains("did you mean 'fn'?"));

// Example: Context-aware
let code = "let fnn = 5;";  // Variable name, not keyword typo
let result = compile(code);
assert!(result.is_ok());  // Should not suggest 'fn'
```

**References**: v0.0.3 DEFERRED_ITEMS_TRACKING.md, LEARNINGS.md Phase 2

---

### Phase 3D: Multi-Error Reporting 🔥

**Status**: Deferred from v0.0.3  
**Priority**: High  
**Estimated Effort**: 4-5 days

**What It Is**: Report all errors in one pass, not just first error. Enables batch and stream error reporting modes.

**Why Deferred**: Phase 3C (parser recovery) provides foundation. Multi-error reporting is an enhancement that requires API design and CLI changes.

**Current State**: Parser collects multiple errors internally via error recovery, but `compile()` API returns only first error.

**Implementation Notes**:

- **Batch Mode**: Return all errors at once (`Result<Program, Vec<CompilerError>>`)
- **Stream Mode**: Callback-based error reporting (for IDEs)
- **CLI Flag**: `--all-errors` to enable batch mode
- **API Changes**: New `compile_all_errors()` function
- **Error Ordering**: Sort by line/column for user-friendly presentation

**Scope**:

- [ ] Design multi-error API (`CompilerError` struct with span, code, message)
- [ ] Implement `compile_all_errors()` function
- [ ] Add CLI flag `--all-errors`
- [ ] Add callback-based stream mode for IDE integration
- [ ] Sort errors by line/column
- [ ] Update CLI error display (show all errors, not just first)
- [ ] Tests for multiple error scenarios

**Benefits**:

- Fix multiple issues per compile cycle (faster iteration)
- Better IDE integration (show all diagnostics at once)
- Matches behavior of Rust/TypeScript compilers

**Acceptance Criteria**:

```rust
// Example: Multiple syntax errors
let code = r#"
fn broken() {
    let x = 
    let y = 
}
"#;
let errors = compile_all_errors(code);
assert_eq!(errors.len(), 2);  // Both missing values caught
assert!(errors[0].code == ErrorCode::E108);
assert!(errors[1].code == ErrorCode::E108);
```

**Dependencies**: Phase 3C (parser recovery) ✅ Complete

**References**: v0.0.3 DEFERRED_ITEMS_TRACKING.md, PHASE_3_ERROR_DOCS_RECOVERY.md

---

### Phase 3E: Diagnostic Collection Infrastructure 🏗️

**Status**: Deferred from v0.0.3  
**Priority**: Medium  
**Estimated Effort**: 5-7 days

**What It Is**: Standardized diagnostic system for all compiler stages (lexer, parser, type checker, runtime). Enables warnings, info messages, and hints beyond just errors.

**Why Deferred**: Phase 3D prerequisite (multi-error API design). Architectural refactoring that touches all compiler stages.

**Vision**: Unified `Diagnostic` struct used by all compiler stages

```rust
struct Diagnostic {
    severity: Severity,     // Error, Warning, Info, Hint
    code: ErrorCode,        // E001-E499 (errors), W001+ (warnings)
    message: String,
    span: Span,             // Source location
    suggestions: Vec<String>,
    related: Vec<Diagnostic>, // Related diagnostics
}

struct DiagnosticCollector {
    diagnostics: Vec<Diagnostic>,
    max_errors: usize,      // Stop after N errors
}
```

**Scope**:

- [ ] Define `Diagnostic` struct with severity levels
- [ ] Create `DiagnosticCollector` for accumulating diagnostics
- [ ] Refactor lexer to use diagnostic system
- [ ] Refactor parser to use diagnostic system
- [ ] Refactor type checker to use diagnostic system
- [ ] Add warning codes (W001-W099: unused variables, dead code, etc.)
- [ ] CLI flag: `--max-errors N` (stop after N errors)
- [ ] Tests for diagnostic collection across stages

**Benefits**:

- Warnings support (unused variables, dead code, deprecated features)
- Multi-level diagnostics (errors + warnings + hints)
- Better LSP integration (diagnostic protocol mapping)
- Consistent error format across all stages

**Implementation Strategy**:

1. Create diagnostic infrastructure (Diagnostic, DiagnosticCollector)
2. Migrate lexer to diagnostics (smallest stage, lowest risk)
3. Migrate parser to diagnostics
4. Migrate type checker to diagnostics
5. Add warning codes and detection

**Dependencies**: Phase 3D (multi-error API design)

**References**: v0.0.3 DEFERRED_ITEMS_TRACKING.md

---

### Phase 8: Integration Tests & Cross-Platform Verification 🧪

**Status**: Deferred from v0.0.3  
**Priority**: High  
**Estimated Effort**: 5-7 days

**What It Is**: Full compiler → runtime → Godot pipeline tests, cross-platform CI, platform badges.

**Why Deferred**: Integration tests more valuable with expanded Godot API surface (signals, callbacks, node queries). Current v0.0.3 focused on editor experience, not API expansion.

**Current Coverage**:

- ✅ Unit tests (270+ across lexer, parser, type checker, runtime)
- ✅ Integration tests (error messages, suggestions, recovery)
- ❌ Godot integration tests (manual only)
- ❌ Cross-platform automation

**Scope**:

- [ ] **Compiler → Runtime Integration Tests**
  - Test full compilation pipeline (source → AST → execution)
  - Verify error propagation through pipeline
  - Test runtime behavior matches type checker expectations

- [ ] **Godot Integration Tests** (v0.0.4 perfect timing!)
  - Script loading and compilation in Godot context
  - Signal emission and connection (v0.0.4 feature)
  - Callback invocation from Godot (v0.0.4 feature)
  - Node query functions in scene tree (v0.0.4 feature)
  - Property exports and Inspector updates
  - Cross-platform Godot builds

- [ ] **Cross-Platform CI**
  - Automated Linux builds
  - Automated Windows builds (already works)
  - Automated macOS builds
  - Platform-specific badges in README.md

- [ ] **Performance Regression Tests**
  - Benchmark tracking over time
  - Alert on >10% performance regressions
  - Historical performance graphs

**Implementation Priority**: After v0.0.4 Godot API expansion (signals, callbacks, node queries)

**Why v0.0.4 is Perfect Timing**:

- Signal tests: Verify signal emission and connection
- Callback tests: Verify `_input`, `_physics_process` invocation
- Node query tests: Verify `get_node`, `get_parent` in scene tree
- Much more API surface to test than v0.0.3 had

**Acceptance Criteria**:

```rust
// Example: Godot integration test
#[test]
fn test_signal_emission_in_godot() {
    let script = r#"
        signal test_signal(value: i32);
        
        fn _ready() {
            emit_signal("test_signal", 42);
        }
    "#;
    
    let node = load_script_in_godot(script);
    let signal_emitted = connect_signal_spy(&node, "test_signal");
    node.call("_ready");
    
    assert!(signal_emitted.was_called());
    assert_eq!(signal_emitted.get_arg(0), 42);
}
```

**References**: v0.0.3 README.md Phase 8, DEFERRED_ITEMS_TRACKING.md

---

## �🔮 Additional Opportunities from v0.0.3 (Workstream Prompt Improvements)

### High Priority (v0.0.4)

**1. Prompt Testing & Validation Framework** (1-2 days)

**Rationale**: Now that workstream prompt improvements are in place (Phase 3C learnings), we need to validate effectiveness.

**Scope**:

- Track time estimates vs. actual completion time
- Measure date accuracy (correct vs. defaulted incorrectly)
- Monitor LEARNINGS.md update rate
- Analyze TODO list discipline adherence
- Generate effectiveness report

**Benefits**: Data-driven prompt refinement, identify persistent issues

**Implementation**: Parse PR descriptions and LEARNINGS.md, generate static report

**Blocker**: No, but valuable for continuous improvement

---

**2. Link Checking Automation** (1-2 days)

**Rationale**: CI already validates links, but local convenience would improve developer experience.

**Scope**:

- Create VS Code task: "Docs: Check All Links"
- Optional: Git hook for pre-commit link validation
- Documentation in scripts/README.md

**Benefits**: Catch broken links before commit, reduce CI failures

**Current State**: Manual `npx markdown-link-check` works well, CI comprehensive

**Blocker**: No, CI sufficient

---

### Medium Priority (v0.0.5 - Deferred to LSP Focus)

**3. Automated Pre-Flight Check Script** (1 day)

**Scope**: `scripts/pre-flight.sh` and `.ps1` to automate manual checks

**Rationale**: Nice quality-of-life improvement, reduces repetitive work

**Blocker**: No, manual works fine

---

**4. LEARNINGS.md Template Generator** (4-6 hours)

**Scope**: `scripts/generate-learnings-entry.ps1` to create phase entry skeleton

**Rationale**: After LEARNINGS.md template stabilizes, automation reduces friction

**Blocker**: No, manual template copying works

---

**Last Updated**: October 7, 2025  
**Status**: 🟡 Planning  
**Previous Version**: v0.0.3 (Editor Experience Alpha)  
**Next Version**: v0.0.5 (LSP Alpha)
