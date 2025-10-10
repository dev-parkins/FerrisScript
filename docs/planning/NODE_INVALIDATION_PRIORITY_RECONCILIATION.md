# Node Invalidation Priority Reconciliation

**Date**: October 9, 2025  
**Status**: ✅ Complete  
**Decision**: Phased integration across v0.0.5, v0.0.7, and post-v0.1.0

---

## 📊 Executive Summary

During v0.0.4 Phase 3 development, we identified a node safety issue: NodeHandle uses string-based tracking without validity checking, which can lead to crashes when nodes are freed in Godot. Comprehensive research (663 lines) produced a 3-phase implementation plan.

However, v0.0.5 was already committed to LSP Alpha (the HIGHEST priority feature for adoption). This document reconciles the competing priorities.

---

## ⚖️ Priority Conflict

### Node Invalidation Research Recommendation

| Phase | Version | Effort | Priority | Scope |
|-------|---------|--------|----------|-------|
| Phase 1 | v0.0.5 | 1-2 hours | HIGH | Basic validity checking |
| Phase 2 | v0.0.6 | 3-4 hours | MEDIUM | ObjectID migration |
| Phase 3 | v0.1.0+ | 8-12 hours | LOW | Full weak reference API |

### Existing Roadmap Commitments

| Version | Focus | Timeline | Premium Requests | Status |
|---------|-------|----------|------------------|--------|
| v0.0.5 | LSP Alpha | 3-4 weeks | 11-16 | HIGHEST PRIORITY |
| v0.0.6 | Arrays/Loops | 2-3 weeks | 8-12 | Language features |
| v0.0.7 | Godot API | 2-3 weeks | 8-11 | Math types, resources |

**Conflict**: Phase 1 is HIGH priority safety (1-2 hours) vs LSP is CRITICAL for adoption (3-4 weeks, 11-16 PRs)

---

## ✅ Resolution: Phased Integration

### Phase 1: v0.0.5 Week 1 (Before LSP)

**Decision**: Add Phase 1 to v0.0.5 early, before LSP work starts

**Rationale**:

- ✅ Only 1-2 hours - minimal impact on timeline
- ✅ Safety issue should be addressed quickly
- ✅ Can be done during LSP planning/setup week
- ✅ Doesn't block or interfere with LSP development
- ✅ Improves error messages immediately

**Updated v0.0.5**:

- Timeline: 3-4 weeks (unchanged)
- Premium Requests: 12-17 ⬆️ (increased by 1)
- Phases: 0 (Node Safety) + 1-5 (LSP)

### Phase 2: v0.0.7 (With Godot API)

**Decision**: Move Phase 2 from v0.0.6 to v0.0.7

**Rationale**:

- ✅ v0.0.7 is "Godot API Expansion" - node improvements fit thematically
- ✅ v0.0.6 arrays/loops are more critical for language usability
- ✅ Not urgent - Phase 1 addresses immediate safety concern
- ✅ 3-4 hours is manageable addition to v0.0.7
- ✅ Gives time for `has_node()` safety pattern to be established
- ✅ Keeps v0.0.6 focused on one thing (language features)

**Updated v0.0.7**:

- Timeline: 2-3 weeks (unchanged)
- Premium Requests: 9-12 ⬆️ (increased by 1)
- Phases: Node Safety Phase 2 + Math Types + Resources

### Phase 3: Post-v0.1.0 (Deferred)

**Decision**: Defer Phase 3 to v0.2.0 or later

**Rationale**:

- ✅ Requires type system features that don't exist yet (Option, method calls, references)
- ✅ Lower priority - Phase 1+2 address core safety/robustness issues
- ✅ Can wait for community feedback on API design
- ✅ Proper type system support needed first

---

## 📈 Impact Analysis

### Timeline Impact

| Metric | Before | After | Change |
|--------|--------|-------|--------|
| v0.0.5 Premium Requests | 11-16 | 12-17 | +1 |
| v0.0.7 Premium Requests | 8-11 | 9-12 | +1 |
| Total to v0.1.0 | ~37-54 | ~39-56 | +2 |
| Timeline to v0.1.0 | ~10-15 weeks | ~10-15 weeks | Unchanged |

**Key Insight**: +2 premium requests total, but timeline unchanged due to minimal effort per phase (1-2 hours and 3-4 hours respectively).

### Feature Grouping

| Version | Theme | Node Work | Rationale |
|---------|-------|-----------|-----------|
| v0.0.5 | Editor Support | Phase 1 (Week 1) | Safety fix during LSP planning |
| v0.0.6 | Language Features | None | Keep focused on arrays/loops |
| v0.0.7 | Godot API | Phase 2 | Thematic fit with API work |
| v0.1.0+ | Polish/Ecosystem | Phase 3 (deferred) | Requires type system |

---

## 🎯 Strategic Priorities

This reconciliation maintains the strategic priorities established in ROADMAP_MASTER.md:

### 1. Editor Experience First ✅

- LSP remains primary focus of v0.0.5
- Phase 1 doesn't interfere (Week 1, 1-2 hours)
- Editor support is still adoption-critical

### 2. Safety Matters ✅

- Phase 1 addresses immediate crash risk
- Phase 2 improves robustness
- Properly sequenced based on urgency

### 3. Smaller, Faster Releases ✅

- Each phase is minimal effort (1-2 hours, 3-4 hours)
- Doesn't disrupt existing release cadence
- Fits naturally into existing version themes

### 4. Thematic Grouping ✅

- v0.0.5: Editor + Safety = Developer Experience
- v0.0.6: Language Features = Core Usability
- v0.0.7: Godot API + Node Improvements = Engine Integration

---

## 📝 Documentation Updates

The following documents have been updated to reflect this reconciliation:

### Primary Roadmap Documents

1. **`docs/planning/ROADMAP_MASTER.md`**
   - Updated v0.0.5 title: "LSP Alpha + Safety Fix"
   - Added Phase 0 (Node Invalidation Phase 1) to v0.0.5
   - Updated v0.0.7 title: "Godot API Expansion + Node Safety"
   - Added Phase 2 (Node Invalidation Phase 2) to v0.0.7
   - Updated premium request counts
   - Updated total effort estimates

2. **`docs/planning/v0.0.5-roadmap.md`**
   - Updated focus: "LSP Alpha + Node Safety Fix"
   - Added Section 0: Node Invalidation Phase 1
   - Detailed implementation plan (1-2 hours)
   - Added timing: Week 1, before LSP work starts
   - Added examples and tests

3. **`docs/planning/v0.0.6-7-roadmap.md`**
   - Added note about Phase 2 inclusion in v0.0.7
   - Added Section 1: Node Invalidation Phase 2
   - Detailed implementation plan (3-4 hours)
   - Renumbered subsequent sections (Match → 2, String Interpolation → 3)
   - Added dependency on Phase 1

4. **`docs/planning/v0.0.4/NODE_INVALIDATION_RESEARCH.md`**
   - Added comprehensive "Priority Reconciliation and Roadmap Integration" section
   - Documented conflict analysis
   - Explained resolution rationale
   - Updated status and next steps
   - Listed all documentation updates

### Status Summary

| Document | Status | Changes |
|----------|--------|---------|
| ROADMAP_MASTER.md | ✅ Updated | Phase 1 in v0.0.5, Phase 2 in v0.0.7 |
| v0.0.5-roadmap.md | ✅ Updated | Added Phase 1 section |
| v0.0.6-7-roadmap.md | ✅ Updated | Added Phase 2 section |
| NODE_INVALIDATION_RESEARCH.md | ✅ Updated | Added reconciliation section |
| This document | ✅ Created | Summary of all changes |

---

## 🔍 Key Decisions

### Why Phase 1 in v0.0.5 Week 1?

1. **Safety First**: Crashes from freed nodes are HIGH priority
2. **Minimal Impact**: Only 1-2 hours - doesn't disrupt LSP work
3. **Good Timing**: Can be done during LSP planning week
4. **Quick Win**: Improves error messages immediately

### Why Phase 2 in v0.0.7 (not v0.0.6)?

1. **Thematic Fit**: v0.0.7 is "Godot API" - node improvements belong there
2. **Not Urgent**: Phase 1 addresses immediate safety, Phase 2 is robustness
3. **Focus v0.0.6**: Keep arrays/loops release focused on language features
4. **Has_Node Pattern**: Give time for safe patterns to be established

### Why Phase 3 Deferred?

1. **Dependencies**: Requires Option type, method calls, reference types
2. **Priority**: Phase 1+2 solve the core problems
3. **Community Input**: Need feedback on API design before full implementation
4. **Type System**: Proper language support needed first

---

## 📋 Next Steps

### Immediate (v0.0.4 Phase 3)

- ✅ Research complete
- ✅ Roadmap reconciliation complete
- ✅ Documentation updated
- 🔄 Ready to merge Phase 3 PR

### v0.0.5 Week 1 (Phase 1)

- [ ] Implement `is_instance_valid()` binding
- [ ] Add validity checking to NodeHandle
- [ ] Update node query functions
- [ ] Add tests for freed node scenarios
- [ ] Update examples with safe patterns
- [ ] Document limitation and usage

### v0.0.7 (Phase 2)

- [ ] Design ObjectID-based NodeHandle
- [ ] Implement ObjectID capture
- [ ] Migrate node query functions
- [ ] Add automatic cleanup
- [ ] Performance testing
- [ ] Update documentation

### Post-v0.1.0 (Phase 3)

- [ ] Wait for type system features (Option, methods, references)
- [ ] Gather community feedback on API design
- [ ] Design full weak reference API
- [ ] Implement when dependencies ready

---

## 🎉 Conclusion

This reconciliation successfully balances:

- ✅ **Safety**: Addresses node invalidation in phases
- ✅ **Adoption**: Keeps LSP as v0.0.5 primary focus
- ✅ **Pragmatism**: Minimal effort per phase (1-2h, 3-4h)
- ✅ **Thematic Grouping**: Each phase fits its version's theme
- ✅ **Timeline**: No disruption to existing roadmap cadence

**Result**: Node safety work integrated without compromising LSP priority or timeline.

---

**Author**: GitHub Copilot  
**Date**: October 9, 2025  
**Related Documents**:

- `docs/planning/ROADMAP_MASTER.md` (master roadmap)
- `docs/planning/v0.0.5-roadmap.md` (Phase 1 details)
- `docs/planning/v0.0.6-7-roadmap.md` (Phase 2 details)
- `docs/planning/v0.0.4/NODE_INVALIDATION_RESEARCH.md` (research findings)
