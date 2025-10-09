# FerrisScript Master Roadmap

**Single Source of Truth for Version Planning**  
**Last Updated**: October 9, 2025  
**Current Version**: v0.0.4 (Phase 1 complete, Phase 2 in progress)

---

## 🎯 Mission

Build a statically-typed, Rust-inspired scripting language for Godot with **compile-time safety** and **first-class editor support**.

**Vision**: "Rust semantics, Godot rhythm" - Predictable performance, zero runtime surprises, excellent developer experience.

---

## 🎮 Positioning & Use Cases

### What FerrisScript Is

> **"Rust-powered, statically compiled, Godot-native scripting for deterministic, high-performance gameplay systems"**

**Key Differentiators vs GDScript**:

1. **Compile-Time Safety**: Catch errors before running Godot
2. **Deterministic Execution**: Perfect for lockstep multiplayer and replays
3. **Predictable Performance**: Zero-cost abstractions, no GC pauses
4. **Systems Integration**: Access Rust crates and native libraries
5. **CI-Friendly**: Compile and test without launching the editor

### Target Use Cases

| Use Case | Why FerrisScript | Example Games |
|----------|------------------|---------------|
| **Simulation-Heavy** | 1000+ agents, ECS-style updates | Factorio, RimWorld |
| **Lockstep Multiplayer** | Deterministic execution required | Age of Empires, StarCraft II |
| **AI/Behavior Systems** | Complex state machines, typed behaviors | Oxygen Not Included, Dwarf Fortress |
| **Performance-Critical** | Physics, pathfinding, procedural generation | Noita, Enter the Gungeon |
| **Testing & CI** | Compile-time validation, no editor | Any project needing reliability |

### What FerrisScript Is NOT

- ❌ Replacement for GDScript (use both together)
- ❌ Low-level engine modification (use C++ for that)
- ❌ Visual scripting alternative (use Godot's visual scripts)
- ❌ Beginner-friendly first language (GDScript is better for learning)

**Best Practice**: Use FerrisScript for performance-critical systems, GDScript for rapid prototyping and scene glue.

---

## 📍 Current State (v0.0.4)

### What Works Today ✅

- ✅ Core compiler (lexer, parser, type checker)
- ✅ Godot GDExtension integration
- ✅ Signal system (declaration, registration, emit)
- ✅ Basic types (i32, f32, String, bool, Node, Variant)
- ✅ Functions and control flow (if/else, while, return)
- ✅ Error reporting with context
- ✅ 379 compiler tests passing

### In Progress 🔄

- 🔄 **v0.0.4 Phase 2**: Lifecycle callbacks (`_ready`, `_process`, `_physics_process`, `_input`)
- 🔄 Roadmap consolidation and strategic planning

### What's Missing ❌

- ❌ LSP / editor support (coming in v0.0.5)
- ❌ Arrays and for loops (coming in v0.0.6)
- ❌ Advanced Godot types (Vector2/3, Color, etc. - coming in v0.0.7)
- ❌ Metadata/reflection (coming in v0.1.0)

---

## 🗓️ Version Roadmap

| Version | Status | Focus | Timeline | Premium Requests |
|---------|--------|-------|----------|------------------|
| **v0.0.4** | 🔄 Current | Runtime stability + lifecycle | 2-3 weeks | 4-6 remaining |
| **v0.0.5** | 📋 Next | LSP Alpha (CRITICAL) | 3-4 weeks | 11-16 |
| **v0.0.6** | 📋 Planned | Language features (arrays/for) | 2-3 weeks | 8-12 |
| **v0.0.7** | 📋 Planned | Godot API expansion | 2-3 weeks | 8-11 |
| **v0.1.0** | 🎯 Milestone | Metadata + polish | 2-3 weeks | 6-9 |
| **v0.2.0** | � Planned | Editor plugins + hot reload | 5-7 weeks ⬆️ | 16-21 ⬆️ |
| **v0.3.0** | ⏸️ Conditional | Scene contracts + parallelism | 8-12 weeks | 25-36 |
| **v0.4.0+** | 🌱 Community | Ecosystem & modding | 12+ weeks | 50+ |

**Total to v0.1.0**: ~10-15 weeks, ~37-54 premium requests ⬆️ (increased due to manifest complexity)

**Total to v0.2.0**: ~19-28 weeks, ~59-81 premium requests ⬆️ (hot reload + profiling added)

**Total to v0.3.0**: ~27-40 weeks, ~84-117 premium requests (if pursuing conditional features)

**⚠️ Note**: v0.3.0+ features require community validation before committing. Timeline assumes user demand exists.

---

## 📦 Version Details

### v0.0.4: Runtime Stability (CURRENT)

**Goal**: Solid foundation for Godot integration

**Phase 1** ✅ Complete:
- Signal system (declaration, registration, emit)
- 379 tests passing
- Edge case testing comprehensive

**Phase 2** 🔄 In Progress:
- [ ] `_ready()` callback
- [ ] `_process(delta: f32)` callback
- [ ] `_physics_process(delta: f32)` callback
- [ ] `_input(event: InputEvent)` callback

**Deliverables**:
- All lifecycle callbacks working
- Integration tests for each
- Examples demonstrating usage
- Documentation updated

**Timeline**: 1-2 weeks remaining  
**Estimated Premium Requests**: 4-6

---

### v0.0.5: LSP Alpha (HIGHEST PRIORITY)

**Goal**: First-class editor support with real-time diagnostics

**Why This Matters**: 
- 🔥 Editor support is adoption-critical
- 🔥 Differentiates FerrisScript from GDScript
- 🔥 Attracts Rust developers to Godot
- 🔥 Enables productivity with basic language features

**Phases**:

1. **LSP Server Foundation** (2-3 PRs)
   - Create `ferrisscript_lsp` crate
   - Basic LSP protocol handler (tower-lsp)
   - Text document synchronization

2. **Syntax Checking** (2-3 PRs)
   - Integrate compiler (lexer + parser + type checker)
   - Real-time diagnostics
   - Publish errors to client

3. **Autocompletion** (3-4 PRs)
   - Keyword completion
   - Type completion
   - Built-in function completion
   - User symbol completion

4. **Navigation** (2-3 PRs)
   - Go to definition
   - Hover documentation
   - Symbol outline

5. **VS Code Extension** (2-3 PRs)
   - Extension scaffolding
   - LSP client integration
   - Syntax highlighting (TextMate)
   - Marketplace publishing

**Deliverables**:
- Working LSP server
- VS Code extension published
- Real-time error checking
- Autocomplete and navigation
- Documentation and examples

**Timeline**: 3-4 weeks  
**Estimated Premium Requests**: 11-16

**Dependencies**: v0.0.4 Phase 2 complete

---

### v0.0.6: Language Features

**Goal**: Arrays, loops, and pattern matching

**Features**:

1. **Array Type** (3-4 PRs)
   - Array type syntax: `[T]`
   - Array literals: `[1, 2, 3]`
   - Array indexing: `arr[0]`
   - Array methods: `len()`, `push()`, `pop()`, etc.

2. **For Loops** (2-3 PRs)
   - For-in syntax: `for item in array { }`
   - Range syntax: `for i in 0..10 { }`
   - Break/continue

3. **Match Expressions** (2-3 PRs)
   - Pattern matching
   - Exhaustiveness checking
   - Match as expression

4. **String Interpolation** (1-2 PRs)
   - Interpolation syntax: `"Hello {name}"`
   - Expression support

**Deliverables**:
- All language features implemented
- Comprehensive tests
- Examples for each feature
- Documentation updated

**Timeline**: 2-3 weeks  
**Estimated Premium Requests**: 8-12

**Dependencies**: v0.0.5 Phase 2 (compiler integration)

**Parallelization**: Can start during v0.0.5 phases 3-5

---

### v0.0.7: Godot API Expansion

**Goal**: Comprehensive type coverage for 2D game development

**Features**:

1. **Core Math Types** (3-4 PRs)
   - Vector2 enhancements
   - Vector3
   - Color
   - Rect2
   - Transform2D

2. **Node Query Functions** (3-4 PRs)
   - `get_node(path: String) -> Node`
   - `has_node(path: String) -> bool`
   - `find_child(name: String) -> Node`
   - `get_parent() -> Node`
   - `get_children() -> [Node]`

3. **Resource Types** (2-3 PRs)
   - Resource base type
   - Texture2D
   - AudioStream
   - PackedScene

**Deliverables**:
- All Godot types implemented
- Rust-side wrappers
- FerrisScript bindings
- Examples demonstrating usage
- Performance validation

**Timeline**: 2-3 weeks  
**Estimated Premium Requests**: 8-11

**Dependencies**: Arrays (v0.0.6) for `get_children()`

---

### v0.1.0: Metadata & Polish (MILESTONE)

**Goal**: Cohesive v0.1.0 release with metadata system

**⚠️ CRITICAL**: This version includes manifest generation that **enables all Godot editor integration in v0.2.0**. Design must be solid. See `planning/technical/EDITOR_INTEGRATION_PLAN.md` for details.

**Features**:

1. **Metadata System** (4-6 PRs) ⬆️ Increased scope
   - Design manifest schema (`ferris_manifest.json`)
   - Generate JSON manifest (signals, properties, methods)
   - Implement `FerrisMetadataRegistry` in godot_bind
   - CLI tooling (`ferris build`, `ferris lint`)
   - Export for Godot editor
   - Documentation generation

2. **Release Polish** (2-3 PRs)
   - Comprehensive testing
   - Performance profiling
   - Documentation updates
   - Example projects
   - Release notes

**Deliverables**:
- JSON metadata working
- Manifest schema documented
- Metadata registry in GDExtension
- CLI tooling operational
- All docs up to date
- Example projects
- v0.1.0 release

**Timeline**: 2-3 weeks ⬆️ (was 1-2)  
**Estimated Premium Requests**: 6-9 ⬆️ (was 5-7)

**Dependencies**: All v0.0.5-7 complete

**Blocks**: 🚨 All Godot editor integration (v0.2.0)

---

## 🔀 Parallelization Strategy

### Timeline Overview

| Week | Primary Track | Secondary Track | Deliverable |
|------|---------------|-----------------|-------------|
| 1-2 | v0.0.4 Phase 2 | - | Lifecycle callbacks ✅ |
| 3-6 | v0.0.5 Phases 1-2 | - | LSP foundation + diagnostics |
| 7-9 | v0.0.5 Phases 3-4 | v0.0.6 arrays start | Autocomplete + navigation |
| 10-11 | v0.0.5 Phase 5 | v0.0.6 for/match | VS Code extension published |
| 12-14 | v0.0.7 Godot API | v0.0.6 string interp | API expansion complete |
| 15-16 | v0.1.0 polish | - | v0.1.0 release 🎉 |

**Key Insight**: Different crates can be developed simultaneously (LSP + language features)

---

## 🎯 Strategic Priorities

### 1. Editor Experience First

**Decision**: LSP in v0.0.5 (not v0.2.0 as originally planned)

**Rationale**:
- Developers need great tooling to be productive
- Editor support is adoption-critical
- Differentiates FerrisScript from GDScript
- Attracts Rust developers to Godot

### 2. Smaller, Faster Releases

**Decision**: Split v0.1.0 into v0.0.5-7 + v0.1.0 polish

**Rationale**:
- Shorter feedback cycles
- Lower risk per release
- Faster time to market
- More opportunities to course-correct

### 3. Solo Dev Optimization

**Decision**: Default to small incremental PRs (workstream Option C)

**Rationale**:
- Easier to review alone
- Clear phase boundaries
- Lower risk per PR
- Better for context switching

### 4. Maximize Premium Requests

**Decision**: Structure work into self-contained execution packages

**Rationale**:
- Each package completes in 1-2 premium requests
- Clear acceptance criteria
- Minimal dependencies
- Testable independently

---

## � Future Versions (v0.2.0+)

### v0.2.0: Godot Editor Integration

**Goal**: In-editor tooling and native Godot experience

**Scope** (based on `planning/technical/EDITOR_INTEGRATION_PLAN.md` + research insights):

1. **FerrisProjectPlugin** (4-6 PRs)
   - Build/rebuild/test panel
   - Console output
   - Manifest viewer
   - File watchers

2. **FerrisInspectorPlugin** (4-6 PRs)
   - Typed property display
   - Signal connection UI
   - Method listing
   - Source links

3. **Hot Reload System** ⭐ (2-3 PRs)
   - Incremental compilation
   - State preservation where possible
   - File system change detection
   - Sub-second iteration times

4. **Performance Profiler Integration** ⭐ (1-2 PRs)
   - Compiler-injected timing hooks
   - FerrisScript functions in Godot profiler
   - Per-function timing display

5. **Documentation Generation** ⭐ (1-2 PRs)
   - Extract doc comments from compiler
   - Generate HTML/Markdown docs
   - Inline tooltips in editor

6. **Enhanced LSP** (2-4 PRs)
   - Workspace symbols
   - Rename refactoring
   - Enhanced hover docs
   - Manifest integration

**Dependencies**: 🚨 v0.1.0 manifest system MUST be complete

**New Skillset**: Godot EditorPlugin development (GDScript or GDExtension)

**Estimated**: 16-21 premium requests ⬆️ (was 12-16), 5-7 weeks ⬆️ (was 4-6)

**Risk**: High (new territory) + Medium (hot reload complexity)

**⭐ New Features** from research analysis:
- Hot reload enables sub-second iteration (major productivity win)
- Profiler integration provides performance transparency
- Doc generation improves discoverability

### v0.3.0: Advanced Integration (Conditional) ⏸️

**Goal**: Compile-time scene validation and parallel processing

**⚠️ CRITICAL**: These features require **community validation** before committing. Defer unless proven user demand.

**Scope** (all conditional):

1. **Scene Contracts** (6-8 PRs)
   - Compile-time node path validation
   - Scene dependency graph
   - Missing node errors at build time
   - **Requires**: Scene parser subsystem (high complexity)

2. **Asset Validation** (3-4 PRs)
   - Compile-time resource existence checks
   - Type validation for assets
   - Missing texture/sound warnings
   - **Requires**: Asset manifest system

3. **Parallel Processing** (8-12 PRs)
   - Safe threading primitives
   - `par_iter_mut()` equivalent for collections
   - Job system integration
   - **Requires**: Careful Godot threading model integration

4. **Determinism Toolkit** (8-12 PRs)
   - `#[deterministic]` function attribute
   - Fixed-point math option
   - Cross-platform validation
   - Frame-by-frame replay support
   - **Requires**: Deep runtime instrumentation

**Estimated**: 25-36 premium requests, 8-12 weeks

**Risk**: Very High (new subsystems, engine limitations)

**Prerequisites**:
- User feedback from v0.2.0
- Community validation of priorities
- Prototype risky features first

### v0.4.0+: Ecosystem & Community-Led Features 🌱

**Goal**: Community-driven ecosystem beyond solo dev capability

**⚠️ STATUS**: **Aspirational** - Requires community contributors

**Scope** (community-led):

1. **Package Ecosystem**
   - Integration with crates.io
   - FerrisScript package registry
   - Module versioning and compatibility

2. **WASM Modding Sandbox**
   - Safe user-authored scripts
   - Sandboxed compilation
   - Plugin API for modders

3. **Advanced Tooling**
   - Visual graph editor
   - Blueprint-style programming
   - Code analyzers marketplace

4. **Multi-Project Module Sharing**
   - Reusable FerrisScript libraries
   - Cross-project dependencies
   - ABI stability guarantees

**Estimated**: Beyond solo dev capacity (50+ premium requests, 12+ months)

**Risk**: Very High (requires team, long-term commitment)

**Prerequisites**:
- Established community
- Multiple contributors
- Proven adoption and demand

---

## 🔮 Long-Term Vision (Phase 2.0+)

See `planning/VISION.md` for aspirational multi-year goals including:
- Multi-engine support (Bevy, Fyrox, etc.)
- Cross-engine runtime abstraction
- Advanced determinism features
- Visual debugger integration

**Status**: Aspirational only - not committed to roadmap

---

## �📚 Related Documentation

### Planning Documents

- **planning/ROADMAP_CONSOLIDATION_ANALYSIS.md** - Comprehensive strategic analysis
- **planning/LSP_VERSION_RECONCILIATION.md** - LSP priority decision rationale
- **planning/technical/EDITOR_INTEGRATION_PLAN.md** - 🆕 Editor integration blueprint
- **planning/v0.0.5-roadmap.md** - Detailed LSP implementation plan
- **planning/v0.0.6-7-roadmap.md** - Language features specification
- **planning/v0.1.0-ROADMAP.md** - Major milestone details

### Technical Documentation

- **ARCHITECTURE.md** - System design and structure
- **DEVELOPMENT.md** - Development workflow and guidelines
- **planning/technical/TYPE_PROMOTION_RESEARCH.md** - Type system research

### Current Version

- **v0.0.4/PHASE_1_2_TRANSITION_SUMMARY.md** - Current state
- **v0.0.4/PHASE_2_PREP.md** - Lifecycle callbacks plan

---

## ✅ Success Criteria

### Per Version

**Code Quality**:
- ✅ All tests passing
- ✅ Zero compiler warnings
- ✅ Code coverage >80%
- ✅ Performance within 10% of baseline

**Documentation**:
- ✅ API docs updated
- ✅ Examples working
- ✅ Release notes written

**Adoption** (v0.1.0+):
- 📊 VS Code extension downloads
- 📊 GitHub stars/forks
- 📊 Community feedback
- 📊 Example projects built by community

### Overall Roadmap

**Timeline**:
- 🎯 Target: v0.1.0 in 10-15 weeks
- 🚀 Stretch: v0.1.0 in 8-10 weeks (aggressive parallelization)

**Premium Requests**:
- 🎯 Target: 35-50 requests total
- 🚀 Stretch: 30-40 requests (optimized workstreams)

**Feature Completeness**:
- ✅ All planned features implemented
- ✅ No critical bugs
- ✅ Documentation comprehensive
- ✅ Performance validated

---

## 🚀 Next Actions

### This Week
1. ✅ Complete roadmap consolidation analysis
2. ✅ Review and approve strategic direction
3. 🔄 Execute v0.0.4 Phase 2 (`_ready()` callback first)
4. ⏳ Begin v0.0.5 LSP research

### Next 2-4 Weeks
1. ⏳ Complete v0.0.4 Phase 2 (all lifecycle callbacks)
2. ⏳ Ship v0.0.4 release
3. ⏳ Start v0.0.5 LSP implementation
4. ⏳ Archive superseded planning docs

### Next 2-3 Months
1. ⏳ Complete v0.0.5 (LSP alpha)
2. ⏳ Complete v0.0.6 (language features)
3. ⏳ Complete v0.0.7 (Godot API)
4. ⏳ Ship v0.1.0 milestone 🎉

---

## 📝 Maintenance

**Update Frequency**: After each version release

**Review Triggers**:
- Major feature complete
- Strategic priority change
- Community feedback requiring pivot
- Performance issues requiring rework

**Owner**: Project Lead (solo dev)

**Last Review**: October 9, 2025 (initial consolidation)

---

**Status**: Active - Single Source of Truth  
**Version**: 1.0  
**Last Updated**: October 9, 2025
