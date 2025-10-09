# LSP Version Reconciliation

**Issue**: LSP feature appears in multiple version plans with conflicting priorities

**Date**: October 9, 2025  
**Decision Required**: Official version target for LSP implementation

---

## 📋 Current Conflict

### Version Appearances

1. **v0.0.5-roadmap.md** (710 lines)
   - Full detailed implementation plan
   - Marked as **CRITICAL** priority
   - 5 phases: Foundation → Diagnostics → Autocomplete → Navigation → Extension
   - Timeline: 4-5 weeks

2. **v0.1.0-ROADMAP.md** (1737 lines)
   - Strategic priority update section
   - Marked as **HIGHEST PRIORITY**
   - "Editor integration should be prioritized over language features"
   - Rationale: "Accelerates adoption through first-class editor support"

3. **Roadmap_Planning.md** (266 lines)
   - Version roadmap table places LSP in **v0.2.0** (Phase 7: "Tooling & LSP")
   - Conflicts with both above documents

### Priority Markers

- 🔥 **CRITICAL** (v0.0.5-roadmap.md)
- 🔥 **HIGHEST PRIORITY** (v0.1.0-ROADMAP.md)
- ⏰ **Phase 7 / v0.2.0** (Roadmap_Planning.md)

---

## 🎯 Recommended Resolution

### Official Decision: **v0.0.5 (Immediately After v0.0.4)**

**Rationale**:

1. **Adoption Critical**: Editor support is table stakes for developer productivity
2. **Differentiation**: Sets FerrisScript apart from GDScript (no LSP support)
3. **Attracts Rust Devs**: Rust developers expect excellent tooling
4. **Enables Productivity**: Developers can be productive with basic language features if editor is great
5. **Strategic Alignment**: v0.1.0-ROADMAP.md explicitly states "editor integration should be prioritized"

### Implementation Plan

**v0.0.5**: LSP Alpha (as detailed in v0.0.5-roadmap.md)

- Phase 1: LSP Server Foundation
- Phase 2: Syntax Checking
- Phase 3: Autocompletion
- Phase 4: Navigation (Go to Definition, Hover)
- Phase 5: VS Code Extension

**v0.0.6**: Language Features (arrays, for loops) - Can parallelize with v0.0.5 phases 3-5

**v0.0.7**: Godot API Expansion

**v0.1.0**: Metadata + Polish (includes LSP beta/polish)

**v0.2.0**: Advanced LSP features (workspace symbols, rename refactoring, etc.)

### Document Updates Required

1. **Update Roadmap_Planning.md**:
   - Change "Phase 7 / v0.2.0: Tooling & LSP" to "Phase 3 / v0.0.5: LSP Alpha"
   - Change "Phase 7 / v0.2.0" to "Advanced LSP Features (rename, workspace symbols)"

2. **Create ROADMAP_MASTER.md**:
   - Single source of truth
   - Clear version sequence
   - LSP in v0.0.5

3. **Archive contradictory docs** after content merged

---

## ✅ Action Items

- [x] Analyze conflict
- [x] Make recommendation
- [ ] Get approval from project lead (you)
- [ ] Update Roadmap_Planning.md
- [ ] Create ROADMAP_MASTER.md
- [ ] Archive superseded docs

---

## 📝 Decision Log

**Date**: October 9, 2025  
**Decision**: LSP moves to v0.0.5 (from v0.2.0 in original plan)  
**Reason**: Editor experience is adoption-critical  
**Impact**: Delays v0.0.6 language features by 3-4 weeks, but accelerates overall adoption  
**Risk**: Low (LSP is independent of language features)  
**Mitigation**: Parallelize language features (v0.0.6) with LSP polish (v0.0.5 phases 3-5)

**Approved by**: [Pending]  
**Status**: Recommendation
