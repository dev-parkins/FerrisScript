# FerrisScript Master Roadmap

**Single Source of Truth for Version Planning**  
**Last Updated**: July 21, 2026  
**Current Version**: v0.0.5 in progress (Stabilization & Engine Modernization)

**Note (2026-07-21)**: The version originally planned as "v0.0.5: LSP
Foundation and Safety Fix" has been renumbered to **v0.0.6** and every
version after it shifted up by one (old v0.0.6 → v0.0.7, old v0.0.7 →
v0.0.8, etc.). After 8 months of dormancy, v0.0.5 was re-scoped to
stabilization (landing two stalled PRs, housekeeping) and engine
modernization (gdext 0.4→0.5.4, Godot 4.7) instead — see the new `v0.0.5`
section below. The LSP/test-framework plan itself is unchanged in content,
just renumbered; its planning docs live in `docs/planning/v0.0.6/` (moved
from `docs/planning/v0.0.5/`).

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

## 📍 Current State (v0.0.5 in progress, built on v0.0.4)

### What Works Today ✅

- ✅ Core compiler (lexer, parser, type checker)
- ✅ Godot GDExtension integration
- ✅ Signal system (declaration, registration, emit) - **Phase 1 COMPLETE**
- ✅ Lifecycle callbacks (_ready, _process, _physics_process, _input, _enter_tree, _exit_tree) - **Phase 2 COMPLETE**
- ✅ Node queries (get_node, has_node, find_child, get_parent) - **Phase 3 COMPLETE**
- ✅ Test harness infrastructure (headless testing with metadata protocol) - **Phase 3 COMPLETE**
- ✅ Godot types (Vector2, Color, Rect2, Transform2D) - **Phase 4 COMPLETE**
- ✅ Struct literal syntax (Vector2 { x: 10.0, y: 20.0 }) - **Phase 4.5 COMPLETE**
- ✅ Property exports (@export annotation, Inspector integration) - **Phase 5 Sub-Phases 1-3 COMPLETE**
- ✅ Basic types (i32, f32, String, bool, Node, Variant, InputEvent)
- ✅ Functions and control flow (if/else, while, return)
- ✅ Error reporting with context (65+ error codes, E801-E816 for exports)
- ✅ 843 tests passing (543 compiler + 110 runtime + 38 harness + 15 integration + 137 other)

### In Progress 🔄

- 🔄 v0.0.5 "Stabilization & Engine Modernization" — see version details below

### What's Missing ❌

- ❌ LSP / editor support (coming in v0.0.6)
- ❌ Arrays and for loops (coming in v0.0.7)
- ❌ Advanced Godot types (Vector3, Basis, etc. - coming in v0.0.8)
- ❌ Metadata/manifest system (coming in v0.1.0)
- ❌ Extended type system (i64, f64, u8, i16, u16 - coming in v0.2.0)
- ❌ Arithmetic safety (checked/saturating methods - coming in v0.3.0)
- ❌ Documentation site (coming in v0.4.0)
- ❌ Godot editor integration (coming in v0.5.0)

---

## 🗓️ Version Roadmap

| Version | Status | Focus | Timeline | Premium Requests |
|---------|--------|-------|----------|------------------|
| **v0.0.4** | ✅ Complete | Godot API + Inspector integration | Completed Oct 10, 2025 | 6-8 used |
| **v0.0.5** | 🔄 In Progress | Stabilization & Engine Modernization (gdext 0.5.4, Godot 4.7) | ~1 week | 4-6 |
| **v0.0.6** | 📋 Next | LSP + Compiler Prerequisites + Test Framework 🛡️ | 6-7 weeks | 20-25 ⬆️ |
| **v0.0.7** | 📋 Planned | Language features (arrays/for) | 2-3 weeks | 8-12 |
| **v0.0.8** | 📋 Planned | Godot API + node safety 🛡️ | 2-3 weeks | 9-12 ⬆️ |
| **v0.1.0** | 🎯 Milestone | LSP UX + Type Safety + Metadata | 3-4 weeks | 10-15 ⬆️ |
| **v0.2.0** | 🚀 Planned | Extended types + LSP Navigation + Validation | 4-5 weeks | 12-16 ⬆️ |
| **v0.3.0** | 🚀 Planned | Arithmetic safety + LSP Advanced + Type Safety Advanced | 5-7 weeks | 14-20 ⬆️ |
| **v0.4.0** | 🚀 Planned | Documentation site + tooling | 3-4 weeks | 10-15 |
| **v0.5.0** | 🚀 Planned | Godot editor integration + hot reload | 5-7 weeks | 16-21 |
| **v0.6.0+** | 🌱 Community | Ecosystem & advanced features | 12+ weeks | 50+ |

**Total to v0.1.0**: ~11-15 weeks, ~40-57 premium requests ⬆️ (increased due to LSP + type safety + manifest)

**Total to v0.4.0**: ~26-34 weeks, ~76-108 premium requests ⬆️ (includes LSP full stack + type system expansion)

**Total to v0.5.0**: ~31-41 weeks, ~92-129 premium requests ⬆️ (includes Godot editor integration)

**⚠️ Note**: Timelines increased to reflect realistic LSP development effort (doubled estimates based on Phase 5 velocity)

**⚠️ Note**: v0.3.0+ features require community validation before committing. Timeline assumes user demand exists.

---

## 📦 Version Details

### v0.0.4: Godot API Expansion (✅ Complete)

**Goal**: Comprehensive Godot integration for 2D game development

**Phase 1** ✅ Complete (PR #46):

- Signal system (declaration, registration, emit)
- 29 new tests
- Editor connection support
- Signal parameters and validation

**Phase 2** ✅ Complete (PR #47-48):

- [x] `_ready()`, `_process(delta: f32)` callbacks
- [x] `_physics_process(delta: f32)` callback
- [x] `_input(event: InputEvent)`, `_enter_tree()`, `_exit_tree()` callbacks
- [x] InputEvent type implementation
- [x] 11 new tests

**Phase 3** ✅ Complete (PR #51):

- [x] Node query functions (get_node, has_node, find_child, get_parent)
- [x] 22 new tests (17 initial + 5 edge cases)
- [x] 12 new error codes (E601-E613)
- [x] Test harness infrastructure (38 test_harness tests, 3,500+ lines)
- [x] Test coverage analysis (31% scenario coverage, 64 scenarios tracked)
- [x] Node invalidation research (safety roadmap for v0.0.6/v0.0.8)
- [x] 4 example scripts with metadata protocol

**Phase 4** ✅ Complete (Commit 6b51076, 00e47b0):

- [x] Additional Godot types (Color, Rect2, Transform2D)
- [x] Field access support (r/g/b/a, position/size, position/rotation/scale)
- [x] 31 type-specific tests (8 Color + 10 Rect2 + 12 Transform2D + 1 Vector2)
- [x] 10 error codes defined (E701-E710)
- [x] Runtime field get/set operations
- [x] Godot binding conversions

**Phase 4.5** ✅ Complete (Commit 7624f4f, 00e47b0):

- [x] Struct literal syntax parsing (`Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 }`)
- [x] Type checker validation for struct literals
- [x] Runtime evaluation of struct literals
- [x] 39 robustness tests (27 compiler + 12 runtime)
- [x] 5 integration examples demonstrating real-world usage
- [x] Checkpoint methodology documentation (50% faster than Phase 4)

**Phase 5** ✅ Complete (Sub-Phases 1-3, October 10, 2025):

- [x] @export annotation parsing (Sub-Phase 1: 8 checkpoints, 4 hours)
- [x] Property types and hints validation (Sub-Phase 2: 8 checkpoints, 2 hours)
- [x] Inspector integration and runtime sync (Sub-Phase 3: 8 checkpoints, ~6 hours)
- [x] 51+ export tests (34 parser + 61 type checker + 10 runtime + 15 integration)
- [x] Error codes E801-E816 for comprehensive validation
- [x] Property hints: range, file, enum with Godot-compliant formatting
- [x] Bidirectional Inspector ↔ Runtime synchronization
- [x] Hot-reload support with property persistence
- **Effort**: 12 hours actual (vs 21-29 hour estimate, 58% efficiency gain)
- **Documentation**: Complete execution plan, 3 sub-phase completion reports, 8 bundle summaries

**Deliverables Remaining**:

- None (v0.0.4 feature-complete)

**What Was Delivered**:

- ✅ All Godot types for 2D development (Color, Rect2, Transform2D)
- ✅ Struct literal syntax for type construction
- ✅ 39 robustness tests + 5 integration examples
- ✅ Comprehensive documentation and checkpoint methodology

**Timeline**: Phase 4-4.5 complete (October 10, 2025), Phase 5 complete (October 10, 2025)  
**Actual Premium Requests Used**: Phase 4: 3-4, Phase 5: ~3-4 (12 hours actual vs 21-29 estimate)  
**Documentation Polish Remaining**: 30 minutes (README + property patterns doc)

---

### v0.0.5: Stabilization & Engine Modernization (🔄 In Progress)

**Goal**: Return the project to a healthy, current baseline after 8 months of
dormancy (last commit Oct 2025 → resumed July 2026) before continuing feature
work — land the two stalled PRs, pay down housekeeping debt, and bring the
Godot binding stack up to date.

**Why this exists / why it's not LSP work**: The project's `v0.0.5` slot was
originally slated for LSP Foundation work (see `v0.0.6` below, its
renumbered home). On return, two finished-but-unmerged PRs were sitting on
`origin` and the `godot` (gdext) dependency was a full major version behind
current, with the property/Inspector binding code being the most
version-fragile part of the codebase. Rather than resume a 6-7 week LSP
effort on top of a stale foundation, this cycle stabilizes first.

**Phase 0 — Environment & baseline**:

- [x] Installed Godot 4.7.stable locally (first time this project has run
      on a non-Windows dev machine)
- [x] Verified `ferris-test` harness works via `GODOT_EXE`/`GODOT_PROJECT`
      env overrides (no `ferris-test.toml` edit needed)

**Phase 1 — Land stalled PRs**:

- [x] Merged PR #59 (source span tracking — `crates/compiler/src/span.rs`,
      `Position`/`Span` across AST/parser/type checker; LSP foundation Phase
      0.1, now filed under `v0.0.6` below)
- [x] Merged PR #60 (Inspector `clear_on_error()` fix — stale exported
      properties no longer persist after a compile error)
- [x] Fixed a GitHub repo ruleset that required a "Full Test Suite" status
      check which never runs on `pull_request` events, permanently blocking
      merges on a solo-maintainer repo; now requires only "Quick Check" +
      "Generate Coverage Report", 0 approvals, squash-only

**Phase 2 — Housekeeping** (PR #61, #62):

- [x] Fixed `.gdextension` library paths for Linux/macOS (missing the `lib`
      prefix cargo adds to cdylibs — the extension had never loaded on
      non-Windows platforms)
- [x] Fixed the test harness deleting its own source `.ferris` scripts when
      the scripts directory under test was the project's live scripts
      directory
- [x] Fixed clippy lints introduced by newer stable Rust
- [x] Restored `docs/archive/` (97 files) and `docs/planning/technical/` (9
      files), unintentionally deleted across the PR #53 revert and #54
      squash-merge; fixed the resulting dangling links
- [x] Added v0.0.3/v0.0.4 sections to `RELEASE_NOTES.md` (was stale at
      v0.0.2)
- [x] Untracked harness-generated `.tscn` scenes; committed `Cargo.lock`

**Phase 3 — gdext 0.4.5 → 0.5.4, Godot 4.7 (PR #63)**:

- [x] `godot` crate 0.4 → 0.5 (`api-4-3` → `api-4-7`), Rust Edition 2021 →
      2024, MSRV declared 1.94
- [x] Migrated `crates/godot_bind/src/lib.rs`'s Inspector integration:
      `PropertyInfo`/`PropertyHintInfo` module move, `export_info_functions`
      → `export_fns` rename, `class_id: ClassId` → `class_name: StringName`,
      and the `get_property_list`/`get_property`/`set_property` virtuals
      renamed to `on_get_property_list`/`on_get`/`on_set`
- [x] `godot_test/ferrisscript.gdextension` `compatibility_minimum` 4.1 →
      4.2; `godot_test/project.godot` engine features "4.5" → "4.7"
- [x] Verified: full test suite green (883 tests), release build succeeds,
      extension loads cleanly under Godot 4.7.stable headless, `ferris-test`
      corpus pass rate unchanged (19/33 — remaining failures are pre-existing
      test-script drift against unimplemented language features, unrelated
      to this upgrade)

**Phase 4 — Inspector fix verification**:

- [x] Verified PR #60's fix headlessly (no interactive editor in this
      environment) via a scripted GDScript test
      (`godot_test/scripts/inspector_refresh_test.gd`) driving a
      `FerrisScriptNode` through compile-error/fix/missing-file scenarios —
      5/5 checks pass under Godot 4.7 / gdext 0.5.4

**Phase 5 — Release** (this section + roadmap renumbering):

- [x] Renumbered `v0.0.5` (LSP Foundation) → `v0.0.6` and cascaded every
      later version up by one; moved `docs/planning/v0.0.5/` →
      `docs/planning/v0.0.6/`
- [ ] Version bumps (workspace + all crates → 0.0.5), CHANGELOG/RELEASE_NOTES
      finalization, tag `v0.0.5`

**Deliverables Remaining**: version bump finalization and release tag (see
Phase 5 above).

**What Was Delivered**: a current, healthy baseline — no code changes beyond
dependency/API migration and bug fixes already merged in prior versions;
zero new language features (that resumes in `v0.0.7`, see below).

---

### v0.0.6: LSP Foundation + Safety Fix (HIGHEST PRIORITY)

**Goal**: Real-time diagnostics and node safety foundation

**Why This Matters**:

- 🔥 **Editor Support**: Makes type safety visible while coding (red squiggles)
- 🔥 **Differentiation**: Positions FerrisScript vs GDScript on developer experience
- 🔥 **Adoption**: Attracts Rust developers who expect great tooling
- 🛡️ **Safety**: Prevents crashes from freed nodes

**Philosophy**: LSP transforms type safety from "annoying batch errors" to "helpful real-time guidance"

**Phases** (Updated with Option A decisions):

0. **Compiler Prerequisites** (3 PRs, 2-3 weeks) 🆕 BLOCKING
   - **Phase 0.1**: Source spans in AST (1 week)
     - Add `Span` and `Position` structs to all AST nodes
     - Update parser to track spans from tokens
     - Required for LSP error reporting
   - **Phase 0.2**: Symbol table extraction (1 week)
     - Extract symbol table from type checker
     - Public API for LSP (go-to-definition, autocomplete)
     - Scope chain tracking
   - **Phase 0.3**: Incremental compilation (1 week)
     - AST caching with source hash invalidation
     - Dependency graph for transitive invalidation
     - Target: 5-10x speedup for LSP responsiveness

1. **Test Framework Foundation** (3-4 PRs, 1 week)
   - Create `crates/test_harness` with metadata parsing
   - Test discovery from `/examples` directory
   - Rust test integration with filtering
   - Cross-platform compatibility (Windows/Linux/macOS)

2. **LSP Server Foundation** (2-3 PRs, 1 week)
   - Create `ferrisscript_lsp` crate with tower-lsp
   - Basic LSP protocol (initialize, shutdown, capabilities)
   - Document manager with incremental compiler integration
   - Text document synchronization (incremental updates)

3. **LSP Test Integration** (2-3 PRs, 1 week)
   - Custom LSP protocol (`ferrisscript/documentTests`, `ferrisscript/runTest`)
   - Test discovery API in LSP
   - Test result cache for status tracking
   - VS Code CodeLens provider ("Run Test" buttons)

4. **Real-Time Diagnostics** (2-3 PRs, 1 week)
   - Integrate compiler with LSP (lexer → parser → type checker)
   - Incremental compilation on each keystroke (using Phase 0.3 cache)
   - Publish diagnostics (errors, warnings) to client
   - Error recovery for partial syntax
   - Performance target: <100ms for typical edits (cache hit)

5. **Godot Test Runner** (2-3 PRs, 1 week)
   - Implement `test_runner.gd` with assertion validation
   - Timeout mechanism and signal monitoring
   - JSON output for CI integration
   - Cross-platform filesystem access

6. **VS Code Extension** (2-3 PRs, 1 week)
   - Extension scaffolding (package.json, activation)
   - LSP client integration (vscode-languageclient)
   - Enhanced syntax highlighting (TextMate grammar reuse)
   - Marketplace publishing (extension ID, icon, README)

**Deliverables**:

- ✅ Compiler with source spans, symbol table, and incremental compilation
- ✅ Consolidated test framework (single source of truth in `/examples`)
- ✅ Working LSP server with real-time error checking
- ✅ LSP test integration (CodeLens "Run Test" buttons)
- ✅ VS Code extension published to marketplace
- ✅ Red squiggles for type errors (instant feedback)
- ✅ Godot headless test runner with assertion validation
- ✅ CI integration with JSON test results
- ✅ Documentation and installation guide

**What's NOT Included** (deferred to v0.1.0):

- ❌ Hover tooltips (needs Phase 0.2 symbol table, but UI deferred)
- ❌ Autocomplete (needs Phase 0.2 symbol table, but UI deferred)
- ❌ Go-to-definition (needs Phase 0.1 spans, but UI deferred)
- ❌ Find references (needs Phase 0.2 symbol table, but UI deferred)
- ❌ Node invalidation Phase 1 (moved to v0.0.8)

**Timeline**: 6-7 weeks (was 3-4 weeks, then 4-5 weeks)  
**Estimated Premium Requests**: 20-25 (was 11-16, then 13-18)

**Timeline Breakdown**:

- Weeks 1-3: Compiler prerequisites (spans, symbol table, incremental compilation)
- Week 4: Test framework foundation
- Week 5: LSP test integration
- Week 6: Godot test runner
- Week 7: CI integration and migration

**Dependencies**: v0.0.4 complete

**Note**: Focus on **diagnostics only** (Minimum Viable LSP). Additional features come in v0.1.0.

**See**: `docs/planning/v0.0.6/LSP_ARCHITECTURE_SUPPORT.md` for detailed architecture

#### v0.0.6 Consistency Checklist 🆕

**Purpose**: Ensure all v0.0.6 planning documents align with master roadmap decisions.

**Critical Decisions Made** (Option A for all):

1. ✅ Ship v0.0.6 with compiler prerequisites (spans + symbol table) - adds 1 week
2. ✅ Full LSP test integration in v0.0.6 - no deferral
3. ✅ Add incremental compilation to v0.0.6 - adds 2-3 weeks

**Updated Timeline**: 6-7 weeks (was 3-4 weeks, then 4-5 weeks)

**Scope Consistency Checklist**:

- [x] **CONSOLIDATED_TEST_FRAMEWORK_IMPLEMENTATION.md**:
  - [x] Timeline updated to 6-7 weeks
  - [x] Phase 0 added (Compiler Prerequisites: Spans, Symbol Table, Incremental Compilation)
  - [x] Phase 2.5 includes test discovery API and result cache
  - [x] Success criteria includes incremental compilation metrics
  - [x] Dependencies section lists Phase 0 as blocking
  - [x] Risk mitigation covers compiler refactoring and cache bugs

- [x] **LSP_ARCHITECTURE_SUPPORT.md**: ✅ UPDATED
  - [x] Verify incremental compilation architecture matches Phase 0.3
  - [x] Confirm symbol table API matches Phase 0.2
  - [x] Ensure span tracking matches Phase 0.1
  - [x] Timeline reflects 6-7 weeks total (not just LSP-specific time)
  - [x] Decision log documents Option A choices
  - [x] Roadmap section updated with v0.0.6/v0.1.0 split

- [ ] **ROADMAP_MASTER.md**: (This document)
  - [x] v0.0.6 timeline updated to 6-7 weeks
  - [x] Premium request estimate updated (13-18 → likely 20-25 with added scope)
  - [ ] Phases 0-3 clearly documented
  - [ ] Dependencies call out compiler prerequisites

**Verification Steps**:

1. **Before starting Phase 0**:
   - [ ] All three documents agree on 6-7 week timeline
   - [ ] Phase 0 tasks match across documents
   - [ ] Compiler refactoring risks acknowledged

2. **End of Week 3** (Decision Point):
   - [ ] Phase 0 complete or on track
   - [ ] Re-assess timeline if falling behind
   - [ ] Consider de-scoping incremental compilation if critical

3. **Before starting Phase 2.5** (Week 5):
   - [ ] Compiler prerequisites (Phase 0) verified complete
   - [ ] Symbol table API stable and tested
   - [ ] Incremental compilation benchmarked

**De-Scoping Plan** (if timeline slips):

Priority 1 (Keep):

- Source spans in AST
- Symbol table extraction
- Basic LSP diagnostics
- Test framework foundation

Priority 2 (Defer to v0.0.7 if needed):

- Incremental compilation (fallback: always recompile)
- LSP test integration (ship test framework standalone)

Priority 3 (Defer to v0.1.0):

- Advanced caching strategies
- Dependency graph optimizations

---

### v0.0.7: Language Features

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

**Dependencies**: v0.0.6 Phase 2 (compiler integration)

**Parallelization**: Can start during v0.0.6 phases 3-5

---

### v0.0.8: Godot API Expansion + Node Safety

**Goal**: Comprehensive type coverage for 2D game development + robust node references

**Features**:

1. **Node Invalidation Phase 2** (1 PR) 🛡️ NEW
   - ObjectID-based weak references
   - Migrate from string-based tracking
   - Automatic handle cleanup
   - **Effort**: 3-4 hours
   - **Priority**: MEDIUM (robustness improvement)
   - **Rationale**: Fits thematically with Godot API work

2. **Core Math Types** (3-4 PRs)
   - Vector2 enhancements
   - Vector3
   - Color
   - Rect2
   - Transform2D

3. **Node Query Functions** (3-4 PRs)
   - ✅ `get_node(path: String) -> Node` (v0.0.4)
   - ✅ `has_node(path: String) -> bool` (v0.0.4)
   - ✅ `find_child(name: String) -> Node` (v0.0.4)
   - ✅ `get_parent() -> Node` (v0.0.4)
   - Enhanced with ObjectID safety (Phase 2)
   - `get_children() -> [Node]` (NEW)

4. **Resource Types** (2-3 PRs)
   - Resource base type
   - Texture2D
   - AudioStream
   - PackedScene

**Deliverables**:

- Node invalidation Phase 2 complete
- All Godot types implemented
- Rust-side wrappers
- FerrisScript bindings
- Examples demonstrating usage
- Performance validation

**Timeline**: 2-3 weeks  
**Estimated Premium Requests**: 9-12 ⬆️ (increased by 1 PR for node safety)

**Dependencies**: Arrays (v0.0.7) for `get_children()`, Phase 1 (v0.0.6) for validity checking

---

### v0.1.0: LSP UX + Type Safety + Metadata (MILESTONE)

**Goal**: Production-ready v0.1.0 with great developer experience

**⚠️ CRITICAL**: This version includes:

1. LSP UX features that make type safety **feel good**
2. Type safety enhancements that catch more errors at compile-time
3. Metadata system that enables Godot editor integration (v0.5.0)

**Features**:

1. **Type Safety Enhancements** (1 PR, 2-3 hours) 🆕
   - **Property Hint Type Validation** (E820-E823 error codes):
     - Range hints (`@range`) only valid for `i32`/`f32` types
     - Enum hints (`@enum`) only valid for `String` type
     - File hints (`@file`) only valid for `String` type
     - Multiline hints (`@multiline`) only valid for `String` type
   - Compile-time error messages with clear explanations
   - 10-15 new validation tests
   - Error code documentation updated
   - **Impact**: Catches property hint mismatches before Inspector shows them

2. **LSP UX Features** (2-3 PRs, 2-3 weeks) 🆕
   - **Hover Tooltips**: Show type information, documentation, and examples
   - **Basic Keyword Completion**: Complete `fn`, `let`, `if`, `while`, `return`, etc.
   - **Document Symbols**: Outline view (Ctrl+Shift+O) shows functions and globals
   - **Platform Testing**: Verify on Windows, Linux, macOS
   - **Impact**: Makes type safety visible and interactive while coding

3. **Metadata System** (4-6 PRs)
   - Design manifest schema (`ferris_manifest.json`)
   - Generate JSON manifest (signals, properties, methods)
   - Implement `FerrisMetadataRegistry` in godot_bind
   - CLI tooling (`ferris build`, `ferris lint`)
   - Export for Godot editor
   - Documentation generation

4. **Demo Game Development** (1 PR)
   - Choose demo type (Pong recommended, Breakout optional)
   - Implement complete game in FerrisScript
   - Use all major features (arrays, for loops, match, signals, exports, node queries)
   - Polish and playability
   - Development process documentation
   - Video walkthrough

5. **Complete Documentation** (2-3 PRs)
   - Language reference (comprehensive syntax/semantics)
   - Godot integration guide (complete API reference)
   - Tutorial series (getting started → building complete game)
   - API documentation (generated from code)
   - Migration guide (GDScript → FerrisScript)

6. **Example Projects** (1-2 PRs)
   - Expand existing examples (hello, move, bounce with arrays)
   - New examples: state_machine (match), bullet_hell (arrays), game_loop (complete structure)
   - Complete games: pong, breakout (if time)
   - Each includes: code, README, tutorial, screenshots/GIFs

7. **Performance Validation** (1 PR)
   - Run comprehensive benchmarks
   - Compare with GDScript baseline
   - Optimize hot paths if needed
   - Document performance characteristics
   - Target: **Within 2x of GDScript**
   - Profile real-world game scenarios
   - Memory usage analysis

8. **Quality Assurance** (1 PR)
   - All tests passing (target: 200+ tests)
   - Test coverage ≥ 80%
   - Zero clippy warnings
   - Zero markdown linting errors
   - All examples working
   - Demo game fully playable
   - LSP tested on multiple platforms
   - Cross-platform builds verified (Linux/Windows/macOS)
   - Integration testing with Godot 4.2+
   - Stress testing (large files, many nodes)

**Deliverables**:

- Property hint type validation working
- LSP with hover, completion, and symbols
- JSON metadata working
- Manifest schema documented
- Metadata registry in GDExtension
- CLI tooling operational
- Demo game(s) playable
- Complete documentation
- All docs up to date
- Multiple example projects
- Performance validated
- Quality assured
- v0.1.0 release 🎉

**Timeline**: 3-4 weeks (was 2-3 weeks)  
**Estimated Premium Requests**: 10-15 (was 6-9)

**Dependencies**: All v0.0.6-7 complete

**Blocks**: 🚨 All Godot editor integration (v0.5.0)

**See**:

- `docs/planning/v0.1.0-release-plan.md` for execution plan
- `docs/TYPE_SAFETY_ROADMAP_ANALYSIS.md` for type safety details
- `docs/planning/v0.0.6/LSP_ARCHITECTURE_SUPPORT.md` for LSP architecture

---

## 🔀 Parallelization Strategy

### Timeline Overview

| Week | Primary Track | Secondary Track | Deliverable |
|------|---------------|-----------------|-------------|
| 1-2 | v0.0.4 Phase 2 | - | Lifecycle callbacks ✅ |
| 3-6 | v0.0.6 Phases 1-2 | - | LSP foundation + diagnostics |
| 7-9 | v0.0.6 Phases 3-4 | v0.0.7 arrays start | Autocomplete + navigation |
| 10-11 | v0.0.6 Phase 5 | v0.0.7 for/match | VS Code extension published |
| 12-14 | v0.0.8 Godot API | v0.0.7 string interp | API expansion complete |
| 15-16 | v0.1.0 polish | - | v0.1.0 release 🎉 |

**Key Insight**: Different crates can be developed simultaneously (LSP + language features)

---

## 🎯 Strategic Priorities

### 1. Editor Experience First

**Decision**: LSP in v0.0.6 (not v0.2.0 as originally planned)

**Rationale**:

- Developers need great tooling to be productive
- Editor support is adoption-critical
- Differentiates FerrisScript from GDScript
- Attracts Rust developers to Godot

### 2. Smaller, Faster Releases

**Decision**: Split v0.1.0 into v0.0.6-7 + v0.1.0 polish

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

### v0.2.0: Extended Types + LSP Navigation + Script Validation

**Goal**: Complete type system, navigation features, and runtime validation

**Why This Matters**:

- **Extended Types**: Enable large values, high-precision math, memory optimization
- **LSP Navigation**: Jump to definitions, find references, workspace-wide search
- **Script Validation**: Catch interface mismatches at load-time

**Features**:

1. **64-bit Numeric Types** (2-3 PRs)
   - `i64` (signed 64-bit integer)
   - `f64` (double-precision float)
   - Use cases: timestamps, entity IDs, large coordinates, high-precision physics
   - Godot compatibility (safe widening from i32→i64, f32→f64)

2. **Smaller Integer Types** (2-3 PRs)
   - `i16` (16-bit signed integer)
   - `u8` (8-bit unsigned integer)
   - `u16` (16-bit unsigned integer)
   - Use cases: color channels (u8), tile coordinates (i16), compact state (u8)
   - Overflow considerations (panic in debug, wrap in release)

3. **Explicit Type Casting** (2-3 PRs)
   - Cast syntax: `value as TargetType`
   - Widening casts (always safe): `i32` → `i64`, `f32` → `f64`
   - Narrowing casts (lossy): `i64` → `i32` (truncates), `f64` → `f32` (precision loss)
   - Cross-family casts: `i32` → `f32` (may lose precision)
   - Runtime checks for debug builds

4. **LSP Navigation Features** (2-3 PRs, 2-3 weeks) 🆕
   - **Go-to-Definition**: Ctrl+Click or F12 to jump to function/variable definition
   - **Find References**: Shift+F12 to show all usages of a symbol
   - **Symbol Index**: Workspace-wide symbol search (Ctrl+T)
   - **Cross-File Navigation**: Navigate between script files
   - **Impact**: Makes large codebases navigable and refactorable

5. **Script Signature Validation** (1 PR, 5-7 hours) 🆕
   - Define `ScriptSignature` interface (properties, functions, signals)
   - Validate scripts have required properties at load-time
   - Validate scripts have required functions at load-time
   - Better error messages for interface mismatches
   - **Use Cases**: Plugin systems, child node contracts, team API validation
   - **Impact**: Catches interface breaks before runtime

**Deliverables**:

- All new types implemented in lexer, parser, type checker, runtime
- Explicit casting with overflow/truncation handling
- LSP navigation working (go-to-def, find refs, workspace symbols)
- Script signature validation at load-time
- Comprehensive tests (~50-60 new tests)
- Documentation and examples
- Godot bindings updated

**Timeline**: 4-5 weeks (was 2-3 weeks)  
**Estimated Premium Requests**: 12-16 (was 6-9)

**Dependencies**: v0.1.0 complete (LSP foundation, type system)

**See**:

- `docs/planning/v0.2.0-roadmap.md` for type system details
- `docs/TYPE_SAFETY_ROADMAP_ANALYSIS.md` for validation details
- `docs/planning/v0.0.6/LSP_ARCHITECTURE_SUPPORT.md` for LSP navigation

---

### v0.3.0: Arithmetic Safety + LSP Advanced + Type Safety Advanced

**Goal**: Complete safety features and advanced LSP capabilities

**Why This Matters**:

- **Arithmetic Safety**: Prevent overflow bugs in game logic (health, scores, timers)
- **LSP Advanced**: Enable safe refactoring and better code intelligence
- **Type Safety Advanced**: Optional features for maximum safety (community-driven)

**Features**:

1. **Checked Methods** (2-3 PRs)
   - Returns `Option<T>`, `None` on overflow
   - Methods: `checked_add()`, `checked_sub()`, `checked_mul()`, `checked_div()`, `checked_neg()`, `checked_pow()`
   - Supports all integer types (i32, i64, i16, u8, u16, etc.)

2. **Saturating Methods** (2-3 PRs)
   - Clamps to `MIN`/`MAX` instead of wrapping
   - Methods: `saturating_add()`, `saturating_sub()`, `saturating_mul()`, `saturating_pow()`
   - Perfect for game health/mana, score systems, physics

3. **Wrapping Methods** (1-2 PRs)
   - Explicitly documents wrapping behavior
   - Methods: `wrapping_add()`, `wrapping_sub()`, `wrapping_mul()`, etc.
   - Same behavior as default `+` in release builds

4. **Overflowing Methods** (1-2 PRs)
   - Returns `(value, overflowed: bool)` tuple
   - Methods: `overflowing_add()`, etc.
   - Allows custom overflow handling

5. **LSP Advanced Features** (3-4 PRs, 3-4 weeks) 🆕
   - **Rename Symbol**: Safe refactoring (F2 to rename across all files)
   - **Parameter Hints**: Signature help (Ctrl+Shift+Space shows parameter names/types)
   - **Context-Aware Completions**: Type-filtered autocomplete (only show valid properties)
   - **Semantic Highlighting**: Color code based on symbol types
   - **Impact**: Transforms LSP from "helpful" to "essential"

6. **Type Safety Advanced** (2-3 PRs, TBD - community-driven) 🆕⚠️
   - **Type-Safe Node Queries** *(if Godot types available)*:
     - `get_node::<Player>("Player")` returns typed node
     - Compile-time validation of node types
     - **Requires**: Godot type registry (significant effort)
   - **Strict Mode** *(if community wants it)*:
     - `@strict` attribute for maximum type safety
     - Require explicit types everywhere (no inference)
     - Require explicit casts (no implicit conversions)
   - **Pre-Compilation** *(if performance needed)*:
     - Compile `.ferris` → `.fsc` at build time
     - Faster load times, no runtime compilation
   - **⚠️ Note**: These features require **community validation** before implementation

**Deliverables**:

- All arithmetic safety methods implemented
- `Option<T>` type added to type system
- LSP rename and advanced completions working
- Type safety advanced features (if validated)
- Comprehensive tests (~70-90 new tests)
- Documentation with game-specific examples
- Performance benchmarks

**Timeline**: 5-7 weeks (was 2-3 weeks)  
**Estimated Premium Requests**: 14-20 (was 6-10)

**Dependencies**: v0.2.0 (extended type system, LSP navigation)

**See**:

- `docs/planning/v0.3.0-roadmap.md` for arithmetic safety details
- `docs/planning/v0.0.6/LSP_ARCHITECTURE_SUPPORT.md` for LSP advanced features
- `docs/TYPE_SAFETY_ROADMAP_ANALYSIS.md` for type safety advanced (community validation required)

---

### v0.4.0: Documentation Site & Developer Tooling

**Goal**: Professional documentation infrastructure and enhanced compiler intelligence

**Why This Matters**:

- Central hub for learning and API reference
- Cross-platform syntax highlighting consistency
- Better developer experience through warnings and linting
- Community growth through accessible documentation

**Features**:

1. **Official Documentation Website** (3-5 PRs) 🔥 CRITICAL
   - Tech stack: VitePress (recommended) or Docusaurus
   - Site structure: guide, reference, examples, blog, playground (stretch)
   - Features: search (Algolia), dark/light theme, mobile-responsive, versioned docs
   - Hosting: GitHub Pages or similar

2. **Shiki Syntax Highlighting** (1 PR)
   - Reuses VS Code TextMate grammar
   - Perfect consistency between editor and docs
   - Multiple theme support

3. **GitHub Linguist Support** (1 PR)
   - Submit `.ferris` language definition
   - Enables GitHub syntax highlighting
   - Repository language statistics

4. **Compiler Warnings System** (3-4 PRs)
   - Warning infrastructure separate from errors
   - Lint warnings: unused variables, unreachable code, style issues
   - Performance warnings: expensive operations, allocations in hot paths
   - Configurable warning levels

5. **API Documentation Generator** (2-3 PRs)
   - Extract doc comments from FerrisScript code
   - Generate HTML/Markdown documentation
   - Integration with documentation site

**Deliverables**:

- Live documentation website (ferrisscript.org or similar)
- GitHub syntax highlighting enabled
- Comprehensive warning system
- Auto-generated API docs
- Examples gallery with live code

**Timeline**: 3-4 weeks  
**Estimated Premium Requests**: 10-15

**Dependencies**: v0.1.0 (stable language), v0.2.0 (complete type system)

**See**: `planning/v0.4.0-roadmap.md` for detailed specifications

---

### v0.5.0: Godot Editor Integration

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

**Note**: Previously labeled v0.2.0, renumbered to v0.5.0 due to type system expansions

**⭐ New Features** from research analysis:

- Hot reload enables sub-second iteration (major productivity win)
- Profiler integration provides performance transparency
- Doc generation improves discoverability

### v0.6.0+: Advanced Integration (Conditional) ⏸️

**Goal**: Scene validation, parallel processing, and advanced features

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

5. **Package Ecosystem**
   - Integration with crates.io
   - FerrisScript package registry
   - Module versioning and compatibility

6. **WASM Modding Sandbox**
   - Safe user-authored scripts
   - Sandboxed compilation
   - Plugin API for modders

**Estimated**: Beyond solo dev capacity (50+ premium requests, 12+ months)

**Risk**: Very High (new subsystems, engine limitations, requires team)

**Prerequisites**:

- User feedback from v0.5.0
- Community validation of priorities
- Established community with multiple contributors
- Prototype risky features first

---

## 🔮 Long-Term Vision (Phase 2.0+)

See `planning/VISION.md` for aspirational multi-year goals including:

- Multi-engine support (Bevy, Fyrox, etc.)
- Cross-engine runtime abstraction
- Advanced determinism features
- Visual debugger integration

**Status**: Aspirational only - not committed to roadmap

---

## 📚 Related Documentation

### Decision & Analysis Documents (Historical Context)

- **planning/ROADMAP_CONSOLIDATION_ANALYSIS.md** - Strategic analysis (Oct 2025)
- **planning/LSP_VERSION_RECONCILIATION.md** - LSP priority decision (v0.0.6)
- **planning/NODE_INVALIDATION_PRIORITY_RECONCILIATION.md** - Node safety phasing
- **planning/EDITOR_INTEGRATION_IMPACT.md** - Editor integration scope analysis (v0.5.0)

### Version-Specific Implementation Plans (Detailed Reference)

- **planning/v0.0.6-roadmap.md** - Detailed 5-phase LSP implementation
- **planning/v0.0.7-7-roadmap.md** - Arrays, for loops, match + Node Invalidation Phase 2
- **planning/v0.1.0-ROADMAP.md** - Feature specifications & requirements
- **planning/v0.1.0-release-plan.md** - Release execution (demo game, docs, QA) ✅ Merged into v0.1.0 above
- **planning/v0.2.0-roadmap.md** - Extended type system implementation details ✅ Merged into v0.2.0 above
- **planning/v0.3.0-roadmap.md** - Arithmetic safety implementation details ✅ Merged into v0.3.0 above
- **planning/v0.4.0-roadmap.md** - Documentation site implementation details ✅ Merged into v0.4.0 above

### Technical Documentation

- **ARCHITECTURE.md** - System design and structure
- **DEVELOPMENT.md** - Development workflow and guidelines
- **planning/technical/EDITOR_INTEGRATION_PLAN.md** - Editor integration blueprint (v0.5.0)
- **planning/technical/TYPE_PROMOTION_RESEARCH.md** - Type system research

### Archived Documentation (Completed Versions)

- **archive/v0.0.4/planning/PHASE_1_2_TRANSITION_SUMMARY.md** - Phase 1→2 transition
- **archive/v0.0.4/planning/PHASE_2_PREP.md** - Lifecycle callbacks plan
- **archive/v0.0.4/planning/PHASE_5_EXECUTION_PLAN_FEEDBACK.md** - Phase 5 feedback ⏳ To be archived

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
4. ⏳ Begin v0.0.6 LSP research

### Next 2-4 Weeks

1. ⏳ Complete v0.0.4 Phase 2 (all lifecycle callbacks)
2. ⏳ Ship v0.0.4 release
3. ⏳ Start v0.0.6 LSP implementation
4. ⏳ Archive superseded planning docs

### Next 2-3 Months

1. ⏳ Complete v0.0.6 (LSP alpha)
2. ⏳ Complete v0.0.7 (language features)
3. ⏳ Complete v0.0.8 (Godot API)
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
