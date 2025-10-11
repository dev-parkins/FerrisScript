# v0.0.3 Deferred Items Tracking

**Version**: 0.0.3 → Future Versions  
**Date**: October 8, 2025  
**Status**: Complete Inventory

---

## 📋 Purpose

This document tracks all items deferred from v0.0.3 to future versions, ensuring nothing is lost and proper roadmap integration.

---

## 🔄 Deferred to v0.0.4 (Godot API Expansion)

### Phase 2B: Keyword Suggestions

**Original Plan**: Suggest keywords when users mistype (`fnn` → `fn`)  
**Why Deferred**: Requires lexer changes for context-aware keyword detection  
**Complexity**: Medium (3-4 days)  
**Tracking**: Added to v0.0.4-roadmap.md under "Developer Experience Improvements"

**Rationale**: Phase 2 (variable/function/type suggestions) provides 90% of value. Keyword typos are less common and require cross-component changes (lexer + type checker coordination).

**Implementation Notes**:

- Lexer needs to track "almost keywords" (tokens that are close to keywords)
- Context-aware: `fnn` at statement start → suggest `fn`, but `fnn` in expression → variable typo
- Low priority: Most keyword typos caught by parser (unexpected token errors)

---

### Phase 3D: Multi-Error Reporting (Batch/Stream Modes)

**Original Plan**: Report all errors in one pass, not just first error  
**Why Deferred**: Phase 3C (parser recovery) provides foundation, 3D is enhancement  
**Complexity**: Medium (4-5 days)  
**Tracking**: Added to v0.0.4-roadmap.md under "Error Diagnostics Enhancements"

**Current State**: Parser collects multiple errors internally, but `compile()` API returns only first error

**What's Needed**:

- [ ] **Batch Mode**: Return all errors at once (`Result<Program, Vec<CompilerError>>`)
- [ ] **Stream Mode**: Callback-based error reporting (for IDEs)
- [ ] **CLI Flag**: `--all-errors` to enable batch mode
- [ ] **API Changes**: New `compile_all_errors()` function
- [ ] **Error Ordering**: Sort by line/column for user-friendly presentation

**Benefits**:

- Fix multiple issues per compile cycle (faster iteration)
- Better IDE integration (show all diagnostics)
- Matches behavior of Rust/TypeScript compilers

**Dependencies**: Phase 3C complete ✅ (error recovery foundation exists)

---

### Phase 3E: Diagnostic Collection Infrastructure

**Original Plan**: Standardized diagnostic system for all compiler stages  
**Why Deferred**: Phase 3D prerequisite, architectural refactoring  
**Complexity**: High (5-7 days)  
**Tracking**: Added to v0.0.4-roadmap.md under "Error Diagnostics Enhancements"

**Vision**: Unified `Diagnostic` struct used by lexer, parser, type checker, runtime

```rust
struct Diagnostic {
    severity: Severity,     // Error, Warning, Info, Hint
    code: ErrorCode,        // E001-E499
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

**Benefits**:

- Warnings support (unused variables, etc.)
- Multi-level diagnostics (errors + warnings + hints)
- Better LSP integration (diagnostic protocol mapping)
- Consistent error format across all stages

**Dependencies**: Phase 3D (multi-error API design)

---

### Phase 8: Integration Tests & Cross-Platform Verification

**Original Plan**: Full compiler → runtime → Godot pipeline tests  
**Why Deferred**: More valuable with expanded Godot API surface (v0.0.4)  
**Complexity**: Medium (5-7 days)  
**Tracking**: Added to v0.0.4-roadmap.md under "Testing Infrastructure"

**Scope**:

- [ ] Compiler → Runtime integration tests (full pipeline)
- [ ] Godot integration tests (script loading, execution in Godot)
- [ ] Cross-platform CI (Linux, Windows, macOS)
- [ ] Platform-specific badges
- [ ] Performance regression tests (benchmark tracking)

**Rationale**: Integration tests are most valuable when testing against comprehensive Godot API. Current v0.0.3 focus is editor experience, not API expansion. v0.0.4 will add signals, callbacks, node queries - better testing targets.

**Current Coverage**:

- ✅ Unit tests (270+ across lexer, parser, type checker, runtime)
- ✅ Integration tests (error messages, suggestions, recovery)
- ❌ Godot integration tests (manual only)
- ❌ Cross-platform automation

**Implementation Priority**: After v0.0.4 Godot API expansion (signals, callbacks)

---

## 🔮 Deferred to v0.1.0 (Release Preparation)

### Phase 9: Test Coverage Badge

**Original Plan**: Codecov/Coveralls badge showing test coverage %  
**Why Deferred**: Requires coverage service setup, not critical for alpha  
**Complexity**: Low (1-2 days)  
**Tracking**: Added to v0.1.0-ROADMAP.md under "Release Preparation"

**What's Needed**:

- Choose coverage service (Codecov vs Coveralls vs Codacy)
- Add coverage generation to CI (`cargo tarpaulin` or `cargo llvm-cov`)
- Upload coverage reports to service
- Add badge to README.md

**Current State**: Coverage script exists (`scripts/coverage.ps1/.sh`), but not automated in CI

---

### Phase 9: Rustdoc Hosting

**Original Plan**: Host API documentation on docs.rs or GitHub Pages  
**Why Deferred**: Release-level documentation, not needed for v0.0.3 alpha  
**Complexity**: Low (2-3 days)  
**Tracking**: Added to v0.1.0-ROADMAP.md under "Documentation"

**Options**:

1. **docs.rs** (automatic for published crates on crates.io)
2. **GitHub Pages** (custom deployment)
3. **Netlify/Vercel** (custom domain support)

**What's Needed**:

- Comprehensive rustdoc comments (crate, module, public API)
- Doc examples with tests (`/// # Examples`)
- CI job to build and deploy docs
- Link from README.md and documentation site

**Current State**: Moderate rustdoc coverage, but not comprehensive or published

---

### Phase 9: VS Code Marketplace Submission

**Original Plan**: Publish extension to VS Code Marketplace  
**Why Deferred**: Final polish and release task, not needed for development  
**Complexity**: Medium (3-4 days for polish + submission)  
**Tracking**: Added to v0.1.0-ROADMAP.md under "Release Preparation"

**What's Needed**:

- [ ] Publisher account setup (Microsoft Azure DevOps)
- [ ] Extension polish (icon, README, screenshots, video)
- [ ] Marketplace metadata (categories, keywords, pricing)
- [ ] Legal review (license, privacy policy if needed)
- [ ] `vsce publish` workflow
- [ ] Marketplace listing URL

**Current State**: Extension works locally, full features in v0.0.3 (completion, hover, diagnostics)

**Blockers**: Wait for v0.1.0 stability, gather user feedback from local installation first

---

### Phase 9: Edge Case Tests

**Original Plan**: Comprehensive edge case coverage (large numbers, long lines, pathological inputs)  
**Why Deferred**: Ongoing quality work, not release blocker  
**Complexity**: Medium (ongoing effort)  
**Tracking**: Added to v0.1.0-ROADMAP.md under "Quality Improvements"

**Scope**:

- [ ] Large number literals (i32::MAX, overflow detection)
- [ ] Very long identifiers (1000+ chars)
- [ ] Deep nesting (100+ levels of if/while)
- [ ] Very long lines (10,000+ chars)
- [ ] Unicode edge cases (emoji, RTL text, zero-width chars)
- [ ] Pathological comment patterns
- [ ] Huge files (100,000+ lines)

**Current State**: Basic edge cases covered, but not exhaustive

**Implementation Strategy**: Add edge case tests incrementally as issues are discovered

---

### Phase 9: Code Organization Improvements

**Original Plan**: Refactor for maintainability and extensibility  
**Why Deferred**: Ongoing refactoring, not release blocker  
**Complexity**: High (ongoing effort)  
**Tracking**: Added to v0.1.0-ROADMAP.md under "Technical Debt"

**Improvements Identified**:

- [ ] Extract error formatting into separate module
- [ ] Unify diagnostic infrastructure (Phase 3E dependency)
- [ ] Improve parser modularity (split into submodules)
- [ ] Type checker state management (reduce mutable state)
- [ ] Runtime error handling (structured error types)

**Approach**: Refactor incrementally during feature development, not as separate phase

---

## 🔭 Deferred to v0.0.5+ (LSP & Advanced Features)

### LSP Implementation

**Deferred From**: Phase 4 & 5 (simple completion/hover implemented, full LSP postponed)  
**Target**: v0.0.5  
**Complexity**: Very High (10-15 days)  
**Tracking**: Added to v0.0.5-roadmap.md

**Features Requiring LSP**:

- Scope-aware completion (local variables, parameters)
- Type inference for hover tooltips
- Go-to-definition
- Find references
- Rename symbol
- Code actions (quick fixes)
- Workspace symbols

**Current Workaround**: Static completion lists and simple hover (sufficient for v0.0.3)

---

### VS Code Extension Automated Testing

**Deferred From**: Phase 4 (manual testing used instead)  
**Target**: v0.0.5 (with LSP implementation)  
**Complexity**: High (5-7 days)  
**Tracking**: Added to v0.0.5-roadmap.md

**What's Needed**:

- `@vscode/test-electron` setup
- Mock VS Code API
- Integration tests for completion/hover/diagnostics
- CI integration (headless VS Code testing)

**Rationale**: Manual testing sufficient for simple providers. Automated testing valuable when LSP adds complexity.

---

### Custom Domain Setup (ferrisscript.dev)

**Deferred From**: Phase 3B (Jekyll site)  
**Target**: Post-v0.1.0 (optional polish)  
**Complexity**: Low (1 day)  
**Status**: Domain acquired, GitHub Pages sufficient for now

**What's Needed**:

- DNS configuration (A records to GitHub Pages IPs)
- GitHub Pages custom domain setup
- SSL/TLS certificate (automatic via GitHub)
- Update all documentation links

**Current State**: https://dev-parkins.github.io/FerrisScript works perfectly

**Decision**: Defer until post-v0.1.0, no user impact

---

## 📊 Deferred Items Summary

| Item | Target Version | Complexity | Priority | Tracking |
|------|---------------|------------|----------|----------|
| Phase 2B: Keyword Suggestions | v0.0.4 | Medium | Low | v0.0.4-roadmap.md |
| Phase 3D: Multi-Error Reporting | v0.0.4 | Medium | High | v0.0.4-roadmap.md |
| Phase 3E: Diagnostic Collection | v0.0.4 | High | Medium | v0.0.4-roadmap.md |
| Phase 8: Integration Tests | v0.0.4 | Medium | High | v0.0.4-roadmap.md |
| Phase 9: Test Coverage Badge | v0.1.0 | Low | Low | v0.1.0-ROADMAP.md |
| Phase 9: Rustdoc Hosting | v0.1.0 | Low | Medium | v0.1.0-ROADMAP.md |
| Phase 9: Marketplace Submission | v0.1.0 | Medium | Medium | v0.1.0-ROADMAP.md |
| Phase 9: Edge Case Tests | v0.1.0 | Medium | Low | v0.1.0-ROADMAP.md |
| Phase 9: Code Organization | v0.1.0 | High | Low | v0.1.0-ROADMAP.md |
| LSP Implementation | v0.0.5 | Very High | Critical | v0.0.5-roadmap.md |
| VS Code Automated Testing | v0.0.5 | High | Medium | v0.0.5-roadmap.md |
| Custom Domain Setup | Post-v0.1.0 | Low | Low | Future |

**Total Deferred Items**: 12  
**v0.0.4 Targets**: 4 items  
**v0.1.0 Targets**: 5 items  
**v0.0.5+ Targets**: 2 items  
**Future**: 1 item

---

## ✅ Verification Checklist

- [x] All deferred items documented with rationale
- [x] Target versions assigned based on strategic fit
- [x] Complexity estimates provided
- [x] Dependencies identified
- [x] Tracking location specified (roadmap files)
- [x] Benefits and current state documented
- [x] Implementation notes provided where applicable

---

## 📝 Notes

**Strategic Deferrals Philosophy**:

- **Quality over Speed**: Don't rush features that need more thought
- **User Value Focus**: Prioritize features with immediate user impact
- **Architectural Soundness**: Defer features requiring major refactoring
- **Incremental Delivery**: Ship working features early, enhance later

**Decision Process**:

1. **Is it a blocker?** → No → Consider deferring
2. **Is foundation ready?** → No → Defer until prerequisites complete
3. **Is user value immediate?** → No → Defer to appropriate version
4. **Does it require major refactoring?** → Yes → Defer or simplify scope

**Tracking Integrity**:

- All deferred items added to target version roadmaps
- Roadmap updates committed with v0.0.3 release
- Cross-references between documents maintained
- Nothing lost in deferral process

---

**Date Created**: October 8, 2025  
**Last Updated**: October 8, 2025  
**Author**: Release preparation workstream
