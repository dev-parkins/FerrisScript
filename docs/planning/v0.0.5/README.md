# FerrisScript v0.0.5: LSP Foundation + Test Framework

**Timeline**: 6-7 weeks  
**Status**: Planning Complete  
**Start Date**: TBD  
**Target Completion**: TBD

---

## 📋 Table of Contents

- [Problem Statement](#-problem-statement)
- [Solution Overview](#-solution-overview)
- [Key Deliverables](#-key-deliverables)
- [Acceptance Criteria](#-acceptance-criteria)
- [Phase Breakdown](#-phase-breakdown)
- [Work Delegation Strategy](#-work-delegation-strategy)
- [Dependencies & Critical Path](#-dependencies--critical-path)
- [Risk Management](#️-risk-management)
- [Success Metrics](#-success-metrics)

---

## 🎯 Problem Statement

### Current Pain Points

1. **No Real-Time Feedback**: Developers must compile manually to see type errors
   - Write code → Save → Run `cargo build` → Read terminal output → Fix
   - Feedback loop takes 30+ seconds per iteration
   - Type safety feels like a burden, not a benefit

2. **Fragmented Test Infrastructure**: Tests duplicated between `/examples` and `/godot_test/scripts`
   - Manual test registration required
   - No automated test discovery
   - Platform-specific failures (Windows symlinks)
   - CI doesn't catch all issues

3. **Slow Compilation for LSP**: Full recompilation on every keystroke
   - 150ms+ per edit is too slow for good editor experience
   - No caching or incremental compilation
   - LSP would feel sluggish and unresponsive

4. **Missing LSP Foundation**: Compiler lacks infrastructure for editor integration
   - No source span tracking (can't map errors to exact locations)
   - No symbol table (can't implement go-to-definition)
   - No incremental compilation (can't respond fast enough)

### Impact on Adoption

- **Developer Experience**: Static types without LSP = "annoying compile errors"
- **Competitive Position**: GDScript has better IDE support despite dynamic typing
- **Testing Friction**: Hard to add new tests, easy to break existing ones
- **Iteration Speed**: Slow feedback loops discourage experimentation

---

## 💡 Solution Overview

### What We're Building

**v0.0.5 transforms FerrisScript from "compile-time safety" to "real-time developer experience"**

#### Three Pillars

1. **Compiler Prerequisites** (Weeks 1-3)
   - Source spans: Map every AST node to exact source location
   - Symbol table: Track all variables, functions, types across files
   - Incremental compilation: Cache ASTs, recompile only changed regions
   - **Goal**: Foundation for fast, accurate LSP

2. **Test Framework Consolidation** (Weeks 4-7)
   - Single source of truth: All tests in `/examples` with metadata
   - Automated discovery: No manual registration
   - LSP integration: Run tests from editor with "Run Test" button
   - CI automation: JSON output for automated parsing
   - **Goal**: Zero-friction testing for contributors

3. **LSP Test Integration** (Week 5)
   - Custom LSP protocol: `ferrisscript/documentTests`, `ferrisscript/runTest`
   - CodeLens provider: Visual "Run Test" buttons in editor
   - Real-time status: ✅/❌/⏱️ indicators
   - Test result cache: Fast status updates
   - **Goal**: Validate LSP infrastructure with low-risk scope

### Why This Order?

1. **Compiler first**: LSP needs spans, symbol table, incremental compilation
2. **Test framework second**: Provides structured test cases for LSP validation
3. **LSP test integration third**: Proves LSP works end-to-end before full diagnostics
4. **General LSP later** (v0.1.0): Expand to all code once foundation is solid

---

## 🎁 Key Deliverables

### Phase 0: Compiler Prerequisites (Weeks 1-3)

| Deliverable | Description | Impact |
|-------------|-------------|--------|
| **Source Spans** | Every AST node tracks `(file, line, column, offset)` | LSP can show red squiggles at exact locations |
| **Symbol Table** | Public API: `lookup(name) -> Symbol`, `all_symbols() -> Vec<Symbol>` | LSP can implement go-to-definition, autocomplete |
| **Incremental Compiler** | `compile_file(uri, source)` with AST caching | <100ms compilation on typical edits |
| **Dependency Graph** | Tracks which files import which, transitive invalidation | Cache invalidation works correctly |

### Phase 1: Test Framework (Week 4)

| Deliverable | Description | Impact |
|-------------|-------------|--------|
| **Test Harness Crate** | `crates/test_harness` with metadata parser | Discovers tests from `/examples` automatically |
| **Rust Integration** | `tests/ferris_integration_tests.rs` | `cargo test` runs all FerrisScript tests |
| **Test Filtering** | `FERRIS_TEST_FILTER=bounce cargo test` | Run specific tests during development |
| **Cross-Platform** | Works on Windows/Linux/macOS | No more symlink failures |

### Phase 2-2.5: LSP Foundation + Test Integration (Week 5)

| Deliverable | Description | Impact |
|-------------|-------------|--------|
| **LSP Server** | `crates/lsp` with tower-lsp | Foundation for all LSP features |
| **Document Manager** | Tracks open files, incremental updates | Handles `textDocument/didChange` efficiently |
| **Custom Protocol** | `ferrisscript/documentTests`, `ferrisscript/runTest` | Editor can discover and run tests |
| **CodeLens Provider** | VS Code extension shows "Run Test" buttons | Visual test execution from editor |

### Phase 3-4: Godot Test Runner + Diagnostics (Week 6)

| Deliverable | Description | Impact |
|-------------|-------------|--------|
| **Test Runner GDScript** | `godot_test/test_runner.gd` | Headless Godot test execution |
| **Assertion Validation** | `get_variable()`, `get_emitted_signals()` | Tests actually validate behavior |
| **Timeout Mechanism** | Configurable timeouts (default 10s) | Prevents hanging tests |
| **Real-Time Diagnostics** | LSP publishes errors as you type | Red squiggles for type errors |

### Phase 5: CI Integration (Week 7)

| Deliverable | Description | Impact |
|-------------|-------------|--------|
| **GitHub Actions** | Updated workflow with new test harness | Automated testing on every PR |
| **JSON Output** | `test_runner.gd --json` mode | CI can parse results |
| **Test Migration** | All tests moved to `/examples` | Single source of truth |

---

## ✅ Acceptance Criteria

### Must Have (Blocking Release)

- [ ] **All AST nodes have source spans**
  - Parser tracks spans from tokens
  - All tests updated with span assertions
  - Error messages show exact locations

- [ ] **Symbol table built during type checking**
  - `SymbolTable` API: `lookup()`, `all_symbols()`, `symbols_in_scope()`
  - Integration tests verify symbol resolution
  - Public API documented

- [ ] **Incremental compilation working**
  - Cache hit rate >80% for typical edits
  - <100ms latency for cache hits
  - 5-10x speedup vs full recompilation

- [ ] **Test framework consolidated**
  - Zero `.ferris` files in `/godot_test/scripts`
  - All tests in `/examples` with metadata
  - `cargo test` discovers and runs all tests

- [ ] **LSP test integration working**
  - CodeLens shows "Run Test" button
  - Clicking runs test and shows result (✅/❌)
  - Test status persists in cache

- [ ] **Cross-platform compatibility**
  - Tests pass on Windows, Linux, macOS
  - No platform-specific workarounds
  - CI validates all platforms

### Should Have (Nice to Have)

- [ ] **Performance optimizations**
  - Cache eviction policy (LRU)
  - Disk cache for persistence
  - Metrics dashboard

- [ ] **Enhanced test metadata**
  - Regex patterns for assertions
  - Multiple timeout values
  - Test categories/tags

- [ ] **Better error messages**
  - Span-based error highlighting
  - Suggested fixes
  - Error code links to docs

### Won't Have (Deferred to v0.1.0)

- ❌ **Hover tooltips** (needs Phase 0.2, but UI deferred)
- ❌ **Autocomplete** (needs Phase 0.2, but UI deferred)
- ❌ **Go-to-definition** (needs Phase 0.1, but UI deferred)
- ❌ **Find references** (needs Phase 0.2, but UI deferred)

---

## 📦 Phase Breakdown

### Phase 0: Compiler Prerequisites (Weeks 1-3) 🔴 CRITICAL PATH

**Why First**: LSP fundamentally requires these features. Cannot proceed without them.

#### Week 1: Source Spans + Inspector Fix

**Goal**: Every AST node knows where it came from in the source code + fix Inspector property refresh bug

**Tasks**:

1. Define `Span` and `Position` structs (`crates/compiler/src/span.rs`)
2. Add `span` field to all AST nodes (`crates/compiler/src/ast.rs`)
3. Update parser to track spans from tokens
4. Update all tests with span assertions (543 compiler tests)
5. **🆕 Fix Inspector property refresh on compilation errors** (Quick win, 1-2 hours)

**Acceptance Criteria**:

- [x] `Span::new(start, end)` and `Span::merge(other)` work
- [x] Every `Expr`, `Stmt`, `Type` has a `span()` method
- [x] Parser creates spans from token positions
- [x] Error messages include `Span` information
- [x] All 568 compiler tests pass with spans (31 new span unit tests + 5 integration tests)
- [ ] **🆕 Inspector clears properties on compilation failure**
- [ ] **🆕 Switching scripts after type error updates Inspector correctly**

**Complexity**: 🔴 HIGH (touches entire AST, all tests)  
**Delegation**: ❌ **User-Interactive** (requires careful review of each AST node)

**Quick Win**: Inspector fix can run in parallel as background agent task  
**📄 Inspector Fix Details**: See [INSPECTOR_PROPERTY_FIX.md](INSPECTOR_PROPERTY_FIX.md)

**✅ Completed (PR #TBD)**: Tasks 1-4 complete. Created `span.rs` module with Position/Span structs, updated AST/parser/type_checker, all tests passing.

**⚠️ Implementation Notes**:
- **Byte offset tracking**: Currently using placeholder `0` values. Lexer doesn't track byte positions yet. Deferred to future enhancement (can calculate from line/column if needed, but adds overhead).
- **Point spans**: Most spans are zero-length (start == end) because `span_from()` helper exists but isn't called yet. Multi-token span tracking deferred to Week 2 parser enhancements.
- **Backward compatibility**: Re-exported `Span` from `ast` module to avoid breaking runtime crate. All existing `ast::Span` references still work.

---

#### Week 2: Symbol Table

**Goal**: Extract symbol information for LSP (variables, functions, types)

**Tasks**:

1. Define `SymbolTable`, `Symbol`, `Scope` structs (`crates/compiler/src/symbol_table.rs`)
2. Refactor `TypeChecker` to build `SymbolTable` during type checking
3. Update `compile()` to return `(Type, SymbolTable)`
4. Add integration tests for symbol resolution

**Acceptance Criteria**:

- [ ] `SymbolTable::lookup(name)` returns `Option<&Symbol>`
- [ ] `SymbolTable::all_symbols()` returns all symbols
- [ ] `SymbolTable::symbols_in_scope(id)` walks scope chain
- [ ] `TypeChecker` inserts symbols for variables, functions, parameters
- [ ] Integration tests verify symbol table correctness

**Complexity**: 🟡 MEDIUM (refactoring existing type checker)  
**Delegation**: ⚠️ **User-Guided** (needs review of type checker logic)

---

#### Week 3: Incremental Compilation

**Goal**: Cache ASTs and recompile only changed regions

**Tasks**:

1. Implement `IncrementalCompiler` with AST caching (`crates/compiler/src/incremental.rs`)
2. Add source hash-based cache invalidation
3. Create `DependencyGraph` for transitive invalidation
4. Performance benchmarks (cache hit rate, latency)

**Acceptance Criteria**:

- [ ] `IncrementalCompiler::compile_file(uri, source)` uses cache
- [ ] Cache hit returns result in <50ms (target: <100ms acceptable)
- [ ] Cache miss does full compilation and caches result
- [ ] `DependencyGraph` invalidates dependent files
- [ ] Benchmarks show 5-10x speedup for cache hits
- [ ] Cache hit rate >80% for typical editing sessions

**Complexity**: 🟡 MEDIUM (new module, integration with compiler)  
**Delegation**: ✅ **Background Agent** (well-defined spec, testable in isolation)

---

### Phase 1: Test Framework Foundation (Week 4)

**Why Now**: Provides test infrastructure for validating LSP integration

#### Tasks

1. **Create Test Harness Crate** (2 days)
   - `crates/test_harness/Cargo.toml`
   - Metadata parser: `parse_test_metadata(source) -> TestMetadata`
   - Test discovery: `discover_tests(examples_dir) -> Vec<TestFile>`
   - Error handling with `thiserror`

2. **Rust Test Integration** (2 days)
   - `tests/ferris_integration_tests.rs`
   - Test filtering: `FERRIS_TEST_FILTER=bounce cargo test`
   - Parallel execution with `rayon`
   - Negative testing support

3. **Cross-Platform Validation** (1 day)
   - Test on Windows (absolute paths, no symlinks)
   - Test on Linux (standard paths)
   - Test on macOS (if available)

**Acceptance Criteria**:

- [ ] `cargo test` discovers all `.ferris` files in `/examples`
- [ ] Metadata parsing handles all fields: `TEST:`, `CATEGORY:`, `EXPECT:`, `ASSERT:`, `TIMEOUT:`
- [ ] Test filtering works: `FERRIS_TEST_FILTER=bounce cargo test` runs only bounce test
- [ ] Cross-platform: Tests pass on Windows, Linux, macOS
- [ ] All tests have clear error messages for malformed metadata

**Complexity**: 🟢 LOW (new crate, clear spec)  
**Delegation**: ✅ **Background Agent** (unambiguous requirements, well-tested)

---

### Phase 2: LSP Server Foundation (Week 5a - Days 1-3)

**Why Now**: Needs Phase 0 (compiler prerequisites) complete

#### Tasks

1. **Create LSP Crate** (1 day)
   - `crates/lsp/Cargo.toml` with `tower-lsp` dependency
   - Basic server struct: `FerrisScriptServer`
   - Initialize/shutdown handlers

2. **Document Manager** (1 day)
   - Track open documents: `HashMap<Url, Document>`
   - Apply incremental changes: `did_change` handler
   - Integrate with `IncrementalCompiler` from Phase 0.3

3. **Text Synchronization** (1 day)
   - Handle `textDocument/didOpen`
   - Handle `textDocument/didChange` (incremental)
   - Handle `textDocument/didClose`

**Acceptance Criteria**:

- [ ] LSP server starts and responds to `initialize` request
- [ ] Document manager tracks open files
- [ ] Incremental changes apply correctly (no corruption)
- [ ] `IncrementalCompiler` recompiles on each change
- [ ] Server handles multiple documents simultaneously

**Complexity**: 🟡 MEDIUM (new LSP infrastructure)  
**Delegation**: ⚠️ **User-Guided** (LSP protocol requires careful implementation)

---

### Phase 2.5: LSP Test Integration (Week 5b - Days 4-5)

**Why Now**: Validates LSP works before full diagnostics (lower risk)

#### Tasks

1. **Custom LSP Protocol** (1 day)
   - Define `ferrisscript/documentTests` request/response
   - Define `ferrisscript/runTest` request/response
   - Implement handlers in LSP server

2. **Test Discovery API** (1 day)
   - Use `test_harness` crate from Phase 1
   - Parse test metadata from open documents
   - Return test ranges (line numbers)

3. **VS Code Extension** (1 day)
   - `testProvider.ts` with CodeLens provider
   - "Run Test" command
   - Test result UI (✅/❌/⏱️)

**Acceptance Criteria**:

- [ ] Opening `.ferris` file shows "Run Test" button above test
- [ ] Clicking button runs test via LSP
- [ ] Test result appears in editor (✅ pass, ❌ fail)
- [ ] Test status persists in cache (doesn't re-run on every edit)
- [ ] Multiple tests in same file all work

**Complexity**: 🟡 MEDIUM (custom LSP protocol, VS Code extension)  
**Delegation**: ⚠️ **User-Guided** (integration between LSP server and VS Code)

---

### Phase 3: Real-Time Diagnostics (Week 6a - Days 1-3)

**Why Now**: Validates incremental compilation works with LSP

#### Tasks

1. **Diagnostic Publishing** (1 day)
   - Convert compile errors to LSP `Diagnostic` format
   - Use `Span` from Phase 0.1 for accurate ranges
   - Publish on every `textDocument/didChange`

2. **Error Recovery** (1 day)
   - Parser continues on errors (don't stop at first error)
   - Type checker handles incomplete ASTs
   - Show partial diagnostics even with syntax errors

3. **Performance Validation** (1 day)
   - Benchmark latency: target <100ms for typical edits
   - Verify cache hit rate >80%
   - Profile slow paths

**Acceptance Criteria**:

- [ ] Red squiggles appear as you type (not just on save)
- [ ] Error messages accurate to exact location (uses spans)
- [ ] Latency <100ms for cache hits (typical single-line edit)
- [ ] Multiple errors shown simultaneously
- [ ] Parser doesn't crash on incomplete code

**Complexity**: 🟡 MEDIUM (error recovery is tricky)  
**Delegation**: ⚠️ **User-Guided** (performance tuning requires iteration)

---

### Phase 4: Godot Test Runner (Week 6b - Days 4-5)

**Why Now**: Can run in parallel with Phase 3 (different codebase)

#### Tasks

1. **Test Runner Script** (1 day)
   - `godot_test/test_runner.gd`
   - Discover tests via `FileAccess.open()` (no symlinks)
   - Load and run FerrisScript via `FerrisScriptRunner`
   - JSON output mode

2. **Assertion Validation** (1 day)
   - Add `get_variable(name)` to `FerrisScriptRunner`
   - Add `get_emitted_signals()` to `FerrisScriptRunner`
   - Parse `ASSERT:` metadata and validate
   - Timeout mechanism (default 10s)

**Acceptance Criteria**:

- [ ] `godot --headless --script test_runner.gd` runs all tests
- [ ] Tests timeout after 10s (configurable via `TIMEOUT:` metadata)
- [ ] Assertions validate actual runtime behavior (not just "compiles")
- [ ] JSON output parses correctly in CI
- [ ] Works on Windows (no symlink issues)

**Complexity**: 🟢 LOW (GDScript, clear spec)  
**Delegation**: ✅ **Background Agent** (well-defined, testable in Godot)

---

### Phase 5: CI Integration & Migration (Week 7)

**Why Now**: Final integration, migration, polish

#### Tasks

1. **GitHub Actions Workflow** (1 day)
   - Update `.github/workflows/test.yml`
   - Run `cargo test` (Phase 1 harness)
   - Run `godot --headless` (Phase 4 runner)
   - Parse JSON results

2. **Test Migration** (2 days)
   - Move all `.ferris` files from `/godot_test/scripts` to `/examples`
   - Add metadata to each test
   - Update documentation

3. **Cleanup** (1 day)
   - Remove old test infrastructure
   - Update README
   - Create migration guide for contributors

4. **Final Validation** (1 day)
   - Run full test suite on CI
   - Verify all platforms pass
   - Check performance metrics

**Acceptance Criteria**:

- [ ] CI runs on every PR and passes
- [ ] No `.ferris` files remain in `/godot_test/scripts`
- [ ] All tests have metadata headers
- [ ] Documentation updated (README, CONTRIBUTING)
- [ ] Migration guide written

**Complexity**: 🟢 LOW (mostly migration, documentation)  
**Delegation**: ✅ **Background Agent** (bulk file operations, well-defined)

---

## 🤖 Work Delegation Strategy

### Background Agent Tasks (Can Run Unattended)

These tasks have:

- ✅ Clear, unambiguous requirements
- ✅ Well-defined acceptance criteria
- ✅ No complex design decisions
- ✅ Easy to verify automatically (tests)

| Phase | Task | Rationale |
|-------|------|-----------|
| **0.3** | Incremental Compilation | Well-specified caching logic, testable in isolation |
| **1** | Test Harness Crate | New crate, clear spec, comprehensive tests |
| **4** | Godot Test Runner | GDScript implementation, clear inputs/outputs |
| **5** | Test Migration | Bulk file operations, metadata addition |
| **5** | CI Integration | GitHub Actions YAML, JSON parsing |

**Estimated Background Work**: ~2-3 weeks of the 6-7 week timeline

---

### User-Guided Tasks (Need Iteration & Feedback)

These tasks have:

- ⚠️ Complex design decisions
- ⚠️ Integration with existing code
- ⚠️ Performance tuning required
- ⚠️ Multiple valid approaches

| Phase | Task | Why User-Guided |
|-------|------|-----------------|
| **0.1** | Source Spans in AST | Touches 543 tests, every AST node, needs careful review |
| **0.2** | Symbol Table Extraction | Refactors type checker, complex scope handling |
| **2** | LSP Server Foundation | LSP protocol requires careful implementation |
| **2.5** | LSP Test Integration | Custom protocol, VS Code extension integration |
| **3** | Real-Time Diagnostics | Error recovery is tricky, performance tuning |

**Estimated User-Guided Work**: ~3-4 weeks of the 6-7 week timeline

---

### Parallelization Opportunities

| Primary Track | Secondary Track (Background Agent) | Week |
|---------------|-----------------------------------|------|
| Phase 0.1: Source Spans | - | 1 |
| Phase 0.2: Symbol Table | - | 2 |
| Phase 0.3: Incremental Compilation ⚠️ | Can be background if spec clear | 3 |
| Phase 1: Test Harness (user review) | Phase 1: Tests/docs (agent writes) | 4 |
| Phase 2: LSP Server (user guided) | Phase 4: Godot Runner (agent writes) | 5 |
| Phase 2.5 + 3: LSP Test Integration | - | 6 |
| Phase 5: Final validation | Phase 5: Migration (agent executes) | 7 |

**Key Insight**: Weeks 4-6 have the most parallelization potential. User works on LSP while agent handles Godot test runner.

---

## 🔗 Dependencies & Critical Path

### Critical Path (Must Complete in Order)

```
Phase 0.1 (Spans) 
    ↓
Phase 0.2 (Symbol Table) 
    ↓
Phase 0.3 (Incremental Compilation)
    ↓
Phase 2 (LSP Server Foundation)
    ↓
Phase 2.5 (LSP Test Integration)
    ↓
Phase 3 (Real-Time Diagnostics)
```

**Timeline**: 6 weeks (Weeks 1-6)

**Blocker**: Cannot start Phase 2 until Phase 0 complete (needs spans, symbol table, incremental compiler)

---

### Parallel Paths (Can Run Simultaneously)

```
Phase 1 (Test Harness) ─┐
                        ├─→ Phase 2.5 (LSP Test Integration)
Phase 4 (Godot Runner) ─┘

Phase 5 (CI Integration) ← All above complete
```

**Optimization**:

- Phase 1 (Week 4) can run while finishing Phase 0 if Phase 0 is ahead of schedule
- Phase 4 (Week 6) can run in parallel with Phase 3 (different codebase)
- Phase 5 (Week 7) waits for everything

---

## ⚠️ Risk Management

### High-Risk Items (Require Mitigation)

#### Risk 1: Compiler Refactoring Breaks Tests 🔴 HIGH

**Probability**: Very High  
**Impact**: All 543 compiler tests may fail after adding spans

**Mitigation**:

- Add spans incrementally (one AST node type at a time)
- Use default/dummy spans for unmodified nodes during transition
- Run full test suite after each AST change
- Create migration script to bulk-update test assertions
- **Timeline Buffer**: 1 week contingency built into Phase 0

**Decision Point**: End of Week 1 - if span migration is slower than expected, consider temporary dummy spans

---

#### Risk 2: Incremental Compilation Cache Bugs 🔴 HIGH

**Probability**: Medium  
**Impact**: LSP shows stale errors, incorrect completions

**Mitigation**:

- Extensive unit tests for cache invalidation (test each scenario)
- Add cache validation mode (compare cached vs fresh compilation)
- Implement "force full recompilation" command in LSP
- Monitor cache hit rate in production
- **Fallback**: Disable caching if bugs persist (graceful degradation)

**Decision Point**: End of Week 3 - verify cache correctness before proceeding to LSP

---

#### Risk 3: Timeline Overrun (7 weeks → 9+ weeks) 🟡 MEDIUM

**Probability**: Medium  
**Impact**: Delayed v0.0.5 release

**Mitigation**:

- Phase 0 has highest risk (3 weeks of compiler changes)
- Weekly check-ins to assess timeline
- De-scope features if falling behind (see Priority Plan below)
- **Decision Point**: End of Week 3 (reassess after Phase 0)

**De-Scoping Priority**:

**Priority 1 (Must Keep)**:

- Source spans in AST
- Symbol table extraction
- Basic LSP diagnostics
- Test framework foundation

**Priority 2 (Defer to v0.0.6 if behind)**:

- Incremental compilation (fallback: always recompile)
- LSP test integration (ship test framework standalone)

**Priority 3 (Defer to v0.1.0)**:

- Advanced caching strategies
- Dependency graph optimizations

---

### Medium-Risk Items (Monitor Closely)

#### Risk 4: LSP Protocol Complexity 🟡 MEDIUM

**Mitigation**: Start with simple test-specific LSP (Phase 2.5) before full diagnostics

#### Risk 5: Cross-Platform Test Failures 🟡 MEDIUM

**Mitigation**: Test on all platforms early (Phase 1, Week 4)

#### Risk 6: Performance Regression 🟡 MEDIUM

**Mitigation**: Benchmark cache hit rate, establish performance baselines

---

## 📊 Success Metrics

### Performance Metrics

| Metric | Target | Acceptable | Measurement |
|--------|--------|------------|-------------|
| **Cache Hit Rate** | >95% | >80% | Track in `IncrementalCompiler::metrics()` |
| **Cache Hit Latency** | <50ms | <100ms | Benchmark on typical edits |
| **Cache Miss Latency** | <200ms | <500ms | Full recompilation time |
| **Speedup vs Full Recompile** | 10x | 5x | Compare cache hit vs miss |
| **LSP Response Time** | <100ms | <200ms | `textDocument/didChange` → diagnostics published |

---

### Quality Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Test Coverage** | >80% | Lines covered by `cargo test` |
| **Cross-Platform** | 100% pass | Tests pass on Windows, Linux, macOS |
| **Zero Regressions** | 0 | All 843 existing tests still pass |
| **Documentation** | 100% | All public APIs documented |

---

### Adoption Metrics (Post-Release)

| Metric | Target | Measurement |
|--------|--------|-------------|
| **LSP Usage** | >50% of contributors | VS Code extension installs |
| **Test Additions** | +10% tests in 2 weeks | Track new `.ferris` files in `/examples` |
| **Issue Reports** | <5 critical bugs | GitHub issues labeled `v0.0.5` |
| **Feedback Sentiment** | >80% positive | Survey or Discord reactions |

---

## 📚 Related Documents

- **[CONSOLIDATED_TEST_FRAMEWORK_IMPLEMENTATION.md](./CONSOLIDATED_TEST_FRAMEWORK_IMPLEMENTATION.md)**: Detailed implementation guide with code examples
- **[LSP_ARCHITECTURE_SUPPORT.md](./LSP_ARCHITECTURE_SUPPORT.md)**: LSP architecture, rationale, and incremental compilation details
- **[ROADMAP_MASTER.md](../ROADMAP_MASTER.md)**: High-level version roadmap and consistency checklist

---

## 🚀 Getting Started

### For Contributors

1. **Read this README** to understand the big picture
2. **Review Phase 0** (Compiler Prerequisites) - this is the critical path
3. **Check [CONSOLIDATED_TEST_FRAMEWORK_IMPLEMENTATION.md](./CONSOLIDATED_TEST_FRAMEWORK_IMPLEMENTATION.md)** for detailed implementation
4. **Start with Phase 0.1** (Source Spans) - Week 1

### For Project Leads

1. **Review delegation strategy** (Background Agent vs User-Guided)
2. **Set up weekly check-ins** (especially end of Week 3 decision point)
3. **Monitor risk items** (compiler refactoring, cache bugs, timeline)
4. **Prepare de-scoping plan** if timeline slips

### For Background Agents

**Week 3-4**: Incremental Compilation (Phase 0.3) + Test Harness (Phase 1)  
**Week 5-6**: Godot Test Runner (Phase 4)  
**Week 7**: Test Migration + CI Integration (Phase 5)

---

## 🔗 Related Documents

### Planning Documents

- **[CONSOLIDATED_TEST_FRAMEWORK_IMPLEMENTATION.md](CONSOLIDATED_TEST_FRAMEWORK_IMPLEMENTATION.md)**: Detailed technical implementation guide with code examples
- **[LSP_ARCHITECTURE_SUPPORT.md](LSP_ARCHITECTURE_SUPPORT.md)**: LSP architecture, incremental compilation design, decision log
- **[ROADMAP_MASTER.md](../ROADMAP_MASTER.md)**: High-level version roadmap and consistency checklist

### Task Documents

- **[INSPECTOR_PROPERTY_FIX.md](INSPECTOR_PROPERTY_FIX.md)**: 🆕 Quick win bug fix (Phase 0.1.5) - Inspector property refresh on compilation error

### Reference Documents

- **[TROUBLESHOOTING.md](../../TROUBLESHOOTING.md)**: Known issues and workarounds
- **[DEVELOPMENT.md](../../DEVELOPMENT.md)**: Development workflow and contribution guide
- **[ARCHITECTURE.md](../../ARCHITECTURE.md)**: System architecture overview

---

## ✅ Current Status

- [x] **Planning Complete** (October 13, 2025)
  - [x] Option A decisions documented
  - [x] All three documents aligned (CONSOLIDATED, LSP, ROADMAP)
  - [x] README.md created with phase breakdown
  - [x] Delegation strategy defined
  - [x] Inspector fix task document created (INSPECTOR_PROPERTY_FIX.md)

- [ ] **Phase 0.1: Source Spans** (Week 1)
  - [ ] Define `Span` and `Position` structs
  - [ ] Add spans to all AST nodes
  - [ ] Update parser to track spans
  - [ ] Update all 543 compiler tests

- [ ] **Phase 0.2: Symbol Table** (Week 2)
- [ ] **Phase 0.3: Incremental Compilation** (Week 3)
- [ ] **Phase 1: Test Framework** (Week 4)
- [ ] **Phase 2-2.5: LSP Foundation + Test Integration** (Week 5)
- [ ] **Phase 3-4: Diagnostics + Godot Runner** (Week 6)
- [ ] **Phase 5: CI Integration & Migration** (Week 7)

---

**Last Updated**: October 13, 2025  
**Next Review**: End of Week 3 (Decision Point)
