# FerrisScript v0.0.4 - Godot API Expansion

**Version**: 0.0.4 (Patch Release)  
**Milestone**: TBD (GitHub milestone to be created)  
**Timeline**: 3-4 weeks (Quality-focused, no strict deadline)  
**Strategy**: Phased implementation, small focused PRs  
**Branch Pattern**: `feature/v0.0.4-<feature-name>` → `develop` → `main`

---

## 🎯 Overview

**Strategic Goal**: Expand Godot integration to enable real 2D game development without adding new language features.

**Key Focus Areas**:

1. **Signal Support** - Event-driven programming foundation
2. **Additional Callbacks** - Input handling and physics processing
3. **Node Query Functions** - Scene tree interaction
4. **Godot Types** - Color, Rect2, Transform2D support
5. **Property Exports** - Inspector integration

**Alignment with v0.1.0 Strategy**: Major step in reprioritized roadmap by providing comprehensive Godot API coverage before LSP. Enables developers to build real interactive games with current language features.

---

## � Key Documentation

- **[Known Limitations & Design Decisions](KNOWN_LIMITATIONS.md)** - What's supported, what's deferred, and why
- **[Phase 2 Implementation Checklist](PHASE_2_CHECKLIST.md)** - Detailed task list for Phase 2
- **[Godot Setup Guide](../../GODOT_SETUP_GUIDE.md)** - Installation and troubleshooting
- **[Signal Visibility Issue](SIGNAL_VISIBILITY_ISSUE.md)** - Why signals don't appear in Inspector
- **[Signal Testing Instructions](SIGNAL_TESTING_INSTRUCTIONS.md)** - Manual testing guide

---

## �📊 Phase Tracker

### Phase 1: Signal Support ✅

**Status**: ✅ **COMPLETE & MERGED** ([PR #46](https://github.com/dev-parkins/FerrisScript/pull/46))  
**Priority**: Critical (Core Godot Feature)  
**Branch**: `feature/v0.0.4-signals` → `develop` (merged October 8, 2025)  
**Document**: [PHASE_1_SIGNALS.md](PHASE_1_SIGNALS.md), [PHASE_1_STATUS_UPDATE.md](PHASE_1_STATUS_UPDATE.md)  
**Actual Effort**: 3-4 days (under 5-7 day estimate)

**Key Deliverables**:

- [x] Signal definition in FerrisScript (`signal health_changed(old: i32, new: i32);`)
- [x] Signal emission (`emit_signal("health_changed", old, new);`)
- [x] Signal connection from Godot editor
- [ ] Signal connection from FerrisScript code (⏸️ Deferred - see status doc)
- [x] Signal with parameters (multiple types)
- [x] Signal without parameters
- [ ] Signal disconnect support (⏸️ Deferred - depends on programmatic connection)
- [x] Comprehensive tests (29 tests added, 382 total passing)

**Implementation Highlights**:

- ✅ Full signal lifecycle (declaration → emission → Godot integration)
- ✅ Type checking with 6 error codes (E301-E304, E501-E502)
- ✅ Editor-based connections fully functional
- ✅ Comprehensive documentation and examples
- ⏸️ Programmatic connection deferred (non-blocking for Phase 2)

**Dependencies**: None (clean start for v0.0.4)  
**Enables**: Phase 2 (callbacks may use signals for events)

---

### Phase 2: Additional Callbacks ✅

**Status**: ✅ **COMPLETE** (October 9, 2025)  
**Priority**: High  
**Branch**: `feature/v0.0.4-phase1-prep` (continued from Phase 1)  
**Document**: [PHASE_2_CHECKLIST.md](PHASE_2_CHECKLIST.md)  
**Target PR**: TBD (Signal architecture research included)  
**Actual Effort**: 1 day (callbacks + tests)

**Key Deliverables**:

- [x] `_input(event: InputEvent)` - User input handling
- [x] `_physics_process(delta: f32)` - Fixed timestep updates
- [x] `_enter_tree()` - Node enters scene tree
- [x] `_exit_tree()` - Node exits scene tree
- [x] InputEvent type implementation
- [x] Callback integration tests (11 new tests, 396 total passing)
- [ ] Example scripts demonstrating usage (⚠️ Deferred - compilation investigation needed)

**Implementation Highlights**:

- ✅ All 4 lifecycle callbacks implemented and validated
- ✅ InputEvent type with action checks (is_action_pressed/released)
- ✅ E305 error code for lifecycle validation
- ✅ 11 new tests (7 type checker + 4 runtime)
- ✅ 4 clean commits with passing pre-commit hooks
- ⚠️ Examples deferred due to file compilation issue (core functionality verified via tests)

**Additional Work**:

- ✅ **Signal Editor Visibility Research** - Deep architectural analysis of why signals don't appear in Godot's Node→Signals panel
- ✅ **Production-Ready Implementation Pattern** - Complete `FerrisMetadataRegistry` helper for future v0.1.0+ implementation
- ✅ **Roadmap Validation** - Confirmed by Godot GDExtension experts that current direction is correct

**Dependencies**: Phase 1 complete ✅  
**Enables**: Phase 3 (node queries can use callbacks for testing)

---

### Phase 3: Node Query Functions

**Status**: Not Started  
**Priority**: High  
**Branch**: `feature/v0.0.4-node-queries`  
**Document**: *(To be created: PHASE_3_NODE_QUERIES.md)*  
**Target PR**: TBD

**Key Deliverables**:

- [ ] `get_node(path: String) -> Node` - Retrieve node by path
- [ ] `get_parent() -> Node` - Get parent node
- [ ] `has_node(path: String) -> bool` - Check node existence
- [ ] `find_child(name: String) -> Node` - Find child by name
- [ ] Error handling for invalid paths
- [ ] Integration with Godot node system
- [ ] Comprehensive path tests (absolute, relative, invalid)

**Note**: `get_children()` deferred to v0.0.6 (requires array support)

**Dependencies**: Phase 2 (callbacks may use node queries)  
**Estimated Effort**: 2-3 days

---

### Phase 4: Additional Godot Types

**Status**: Not Started  
**Priority**: Medium  
**Branch**: `feature/v0.0.4-godot-types`  
**Document**: *(To be created: PHASE_4_GODOT_TYPES.md)*  
**Target PR**: TBD

**Key Deliverables**:

- [ ] `Color` type - RGBA colors with field access
- [ ] `Rect2` type - 2D rectangles (position, size)
- [ ] `Transform2D` type - 2D transformations
- [ ] Type integration with type checker
- [ ] Field access support for all types
- [ ] Godot binding implementation
- [ ] Type-specific tests (30+ cases)

**Dependencies**: Phase 3 (types may be used in node operations)  
**Estimated Effort**: 3-4 days

---

### Phase 5: Custom Property Exports

**Status**: Not Started  
**Priority**: Medium  
**Branch**: `feature/v0.0.4-property-exports`  
**Document**: *(To be created: PHASE_5_PROPERTY_EXPORTS.md)*  
**Target PR**: TBD

**Key Deliverables**:

- [ ] `@export` annotation parsing
- [ ] Property types: int, float, string, bool
- [ ] Property hints: range, file, enum
- [ ] Inspector integration
- [ ] Property change detection
- [ ] Export validation
- [ ] Inspector update tests

**Dependencies**: Phases 1-4 (exports may include signals and types)  
**Estimated Effort**: 4-5 days

---

## 📚 Deferred Items from v0.0.3

### Error Diagnostics Enhancements (Tracked in ROADMAP.md)

**Moved from v0.0.3**:

- Phase 2B: Keyword Suggestions (3-4 days)
- Phase 3D: Multi-Error Reporting (4-5 days)
- Phase 3E: Diagnostic Collection Infrastructure (5-7 days)

**Status**: Documented in v0.0.4 ROADMAP.md under "Additional Tasks from v0.0.2 Deferral"  
**Decision**: May implement between Godot API phases if time permits, or defer to v0.0.5

---

### Integration Tests & Quality (Tracked in ROADMAP.md)

**Moved from v0.0.3**:

- Godot integration end-to-end tests
- GDScript performance comparison benchmarks
- Cross-platform CI verification

**Status**: Documented in v0.0.4 ROADMAP.md  
**Priority**: High (better suited for v0.0.4 with expanded API surface)  
**Decision**: Implement after Phase 5, before release

---

### Documentation (Tracked in ROADMAP.md)

**Moved from v0.0.2**:

- GODOT_INTEGRATION.md comprehensive guide
- Godot UI screenshots and GIFs
- Usage examples and patterns

**Status**: Documented in v0.0.4 ROADMAP.md  
**Priority**: Medium  
**Decision**: Create after core Godot API features complete (signals, callbacks, node queries)

---

## 🔄 Workflow

1. **Branch**: Create `feature/v0.0.4-<feature-name>` from `develop`
2. **Implement**: Follow acceptance criteria in phase document
3. **Test**: Meet test coverage targets (75%+ for new code)
4. **Lint**: Pass strict clippy, formatting, documentation checks
5. **PR**: Open PR to `develop` with phase checklist
6. **Review**: Address feedback, ensure quality gates pass
7. **Merge**: Merge to `develop` after approval
8. **Periodic Integration**: Merge `develop` to `main` after major milestones

---

## 📈 Success Metrics

### Quantitative Goals

- [ ] Signals working with parameters (define, emit, connect)
- [ ] All 5 new callbacks implemented and tested
- [ ] 4 node query functions working (defer get_children)
- [ ] 3 new Godot types supported (Color, Rect2, Transform2D)
- [ ] Property exports working in Inspector
- [ ] 30-50 new tests added (comprehensive coverage)
- [ ] All existing tests passing (zero regressions)
- [ ] Test coverage: 70-75% overall (up from 64.54% in v0.0.3)

### Qualitative Goals

- [ ] Can build simple interactive games (input-driven)
- [ ] Event-driven programming feels natural
- [ ] Scene tree interaction is intuitive
- [ ] Physics processing works smoothly
- [ ] Inspector integration is user-friendly

---

## 🚀 Release Criteria

### Code Quality

- [ ] All planned features implemented
- [ ] All tests passing (cargo test --workspace)
- [ ] Zero clippy warnings (strict mode: -D warnings)
- [ ] Code formatted (cargo fmt --all)
- [ ] Benchmarks run and documented

### Documentation

- [ ] All phase documents created
- [ ] Learnings captured in v0.0.4/LEARNINGS.md
- [ ] README updated with new API features
- [ ] CHANGELOG.md updated with v0.0.4 entry
- [ ] All markdown linting passing
- [ ] All links validated

### Integration

- [ ] Godot integration tests passing
- [ ] Example games work (platformer/shooter/puzzle)
- [ ] Cross-platform verified (Windows/Linux at minimum)

---

## 📁 Phase Documents

Each phase will have a detailed document with:

- Acceptance criteria (specific, measurable)
- Technical approach
- Component changes (lexer, parser, type checker, runtime, Godot binding)
- Test coverage requirements
- Quality gates (clippy, formatting, documentation)
- Dependencies on other phases
- Estimated effort

Documents will be created as phases begin, following v0.0.3 pattern.

---

## 🎯 Example Game After v0.0.4

With v0.0.4 complete, developers can build games like this:

```rust
signal health_changed(new_health: i32);
signal player_died;

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
    
    // Move player
    let motion: Vector2 = velocity * delta;
    // ... collision handling ...
}

fn take_damage() {
    health -= 1;
    emit_signal("health_changed", health);
    
    if health <= 0 {
        emit_signal("player_died");
    }
}
```

**Demonstrates**:

- ✅ Signals (health_changed, player_died)
- ✅ Input handling (_input callback)
- ✅ Physics processing (_physics_process)
- ✅ Vector2 math operations
- ✅ Event-driven game logic

---

## 📚 Related Documents

### v0.0.4 Planning

- [v0.0.4 ROADMAP](./ROADMAP.md) - Comprehensive feature roadmap
- [Known Limitations](./KNOWN_LIMITATIONS.md) - What's supported, deferred, and why
- [Phase 2 Checklist](./PHASE_2_CHECKLIST.md) - Implementation task list

### v0.0.4 Phase 1 (Complete)

- [Phase 1 Status](./PHASE_1_STATUS_UPDATE.md) - Completion report
- [Phase 1 Signals](./PHASE_1_SIGNALS.md) - Original plan
- [Signal Visibility Issue](./SIGNAL_VISIBILITY_ISSUE.md) - Known limitation
- [Signal Testing Guide](./SIGNAL_TESTING_INSTRUCTIONS.md) - Manual testing

### v0.0.4 Phase 2 (Ready)

- [Phase 2 Preparation](./PHASE_2_PREP.md) - Technical approach
- [Phase 2 Checklist](./PHASE_2_CHECKLIST.md) - Task-by-task breakdown

### Cross-Version

- [v0.0.3 Archive](../../archive/v0.0.3/README.md) - Previous version reference
- [v0.1.0 Roadmap](../v0.1.0-release-plan.md) - Future plans
- [Architecture](../../ARCHITECTURE.md) - System architecture
- [Development](../../DEVELOPMENT.md) - Development setup
- [Learnings](../../LEARNINGS.md) - Cross-version insights

---

## 📝 Status Summary (October 9, 2025)

### ✅ Completed

- **Phase 1: Signal Support** - Merged to develop ([PR #46](https://github.com/dev-parkins/FerrisScript/pull/46))
  - Signal declaration, emission, Godot integration
  - 29 new tests, 382 total passing
  - Known limitations documented

- **Phase 2: Additional Callbacks** - Implementation complete (October 9, 2025)
  - All 4 callbacks implemented (_input, _physics_process, _enter_tree, _exit_tree)
  - InputEvent type with action checks
  - 11 new tests, 396 total passing
  - Signal editor visibility research completed
  - Actual effort: 1 day (under 3-4 day estimate)

### � Ready to Start

- **Phase 3: Node Query Functions** - Ready after Phase 2 PR merge
  - get_node(), get_parent(), has_node(), find_child()
  - Estimated 3-5 days
  - Depends on Phase 2 complete ✅

### 🔜 Upcoming

- **Phase 4**: Godot Types (Color, Rect2, Transform2D)
- **Phase 5**: Property Exports

### 📊 Quality Metrics

- **Tests**: 396 passing (231 compiler + 68 runtime + 97 integration)
- **Build Status**: Clean (0 errors, 0 warnings)
- **Godot Compatibility**: 4.2+, 4.3+ (with api-4-3 feature), 4.5 (validated)
- **Documentation**: Comprehensive (setup, limitations, testing guides, architectural analysis)

---

## 📝 Notes

- **Quality over Speed**: No strict timeline. Focus on comprehensive Godot API coverage and solid testing.
- **Deferred Items Tracked**: All limitations documented in [KNOWN_LIMITATIONS.md](./KNOWN_LIMITATIONS.md) with rationale.
- **Feature Grouping**: Each phase targets specific Godot functionality for focused PRs.
- **Incremental Value**: Ship functional features early, iterate based on usage.
- **Test-Driven**: Write tests before/during implementation, not after.
- **Integration Focus**: v0.0.4 is perfect timing for comprehensive Godot integration tests (more API surface than v0.0.3).
- **Strategic Position**: This release enables real game development and sets foundation for v0.0.5 LSP work.
- **Milestone Tracking**: GitHub milestone to be created for v0.0.4 PR tracking

---

**Last Updated**: October 8, 2025  
**Status**: Initialized, ready to begin Phase 1
