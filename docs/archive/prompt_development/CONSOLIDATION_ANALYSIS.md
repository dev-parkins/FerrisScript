# Prompts Folder Consolidation Analysis

**Date**: October 7, 2025  
**Context**: Evaluating .github/prompts/ folder for fragmentation and Copilot usability  
**Goal**: Simplify for Copilot while reducing maintenance burden

---

## 🎯 Key Insight: What Copilot Actually Uses

**Critical Understanding**: When you invoke `/prompt #file:workstream-execution.prompt.md`, Copilot loads **ONLY that file**.

- ✅ Copilot reads: `workstream-execution.prompt.md`
- ❌ Copilot does NOT read: `README.md`, `QUICK_REFERENCE.md`, `PR_TEMPLATE_SYSTEM.md`
- ❌ Copilot does NOT auto-discover other files in the directory

**The other files are for HUMANS reading on GitHub**, not for Copilot execution.

---

## 📊 Current State Analysis

### File 1: `workstream-execution.prompt.md` (1,046 lines)

**Purpose**: The actual prompt Copilot executes  
**Audience**: Copilot (primary), Humans (secondary - can read to understand)  
**Status**: ✅ **KEEP** - This is the core artifact

**Completeness Check**:

- ✅ Pre-flight checks
- ✅ Context gathering questions (5 categories, 25 questions)
- ✅ Execution methodology
- ✅ Quality checklist
- ✅ LEARNINGS.md template
- ✅ Deferral recommendations guidance
- ✅ Common pitfalls
- ⚠️ **MISSING**: Specific branch naming conventions (asks user to provide)
- ⚠️ **MISSING**: Specific commit message format examples (asks user to provide)

---

### File 2: `README.md` (280 lines)

**Purpose**: Directory overview and documentation  
**Audience**: Humans (GitHub repo visitors, contributors)  
**Used by Copilot**: ❌ No (not loaded unless explicitly attached)

**Content Analysis**:

- Overview of prompts directory
- When to use workstream prompts (✅ vs ❌ examples)
- Usage examples (how to invoke)
- Best practices (for users and agents)
- Expected flow diagrams

**Unique Value**:

- ✅ When/when not to use (helps users decide)
- ✅ Usage examples (onboarding for new contributors)
- ❌ Best practices (duplicates main prompt's "Common Pitfalls")
- ❌ Expected flow (duplicates main prompt's methodology)

**Recommendation**:

- **SIMPLIFY** to ~100 lines: Overview + when to use + usage examples only
- Remove duplicated best practices (refer to main prompt)
- Remove duplicated flow diagrams (refer to main prompt)

---

### File 3: `QUICK_REFERENCE.md` (230 lines)

**Purpose**: TL;DR for users unfamiliar with the system  
**Audience**: Humans (quick start guide)  
**Used by Copilot**: ❌ No (not loaded)

**Content Analysis**:

- Quick start (how to invoke) - duplicates README
- What happens next (phase overview) - duplicates main prompt
- Questions Copilot will ask - duplicates main prompt Section 2
- Common issues & solutions - duplicates main prompt "Common Pitfalls"
- Pro tips - some unique, some duplicate

**Unique Value**:

- ⚠️ Pro Tip: "Attach context files" - useful but should be in main prompt
- ⚠️ Pro Tip: "Highlight specific text" - useful but should be in main prompt
- ❌ Everything else duplicates main prompt or README

**Recommendation**:

- **REMOVE** - Fully redundant with main prompt
- Extract 2-3 "Pro Tips" and add to main prompt if not already there
- Users can read the main prompt directly (it's well-structured)

---

### File 4: `PR_TEMPLATE_SYSTEM.md` (308 lines)

**Purpose**: Reference documentation for PR creation process  
**Audience**: Humans (developers creating PRs), Copilot (if explicitly invoked)  
**Used by Copilot**: ⚠️ Only if user attaches it or it's added to main prompt

**Content Analysis**:

- Branch naming conventions: `bugfix/*`, `feature/*`, `docs/*`
- How PR templates auto-apply (GitHub automation)
- Examples for creating branches/PRs
- Troubleshooting template issues
- Quick commands reference

**Unique Value**:

- ✅ Specific branch naming conventions (missing from main prompt!)
- ✅ PR template automation explanation
- ⚠️ Examples and troubleshooting (useful for humans, not Copilot)

**Recommendation**:

- **EXTRACT** branch naming conventions → add to main prompt
- **KEEP** as reference documentation (useful for humans)
- **SIMPLIFY** to ~150 lines: Conventions + quick commands only
- Remove detailed examples (they're in CONTRIBUTING.md)

---

## 🎯 Recommended Changes

### Option A: Aggressive Consolidation (Simplest for Copilot)

**Changes**:

1. ✅ Keep `workstream-execution.prompt.md` (1,046 lines)
2. ➕ Add branch naming conventions to main prompt (~20 lines)
3. ➕ Add commit message format to main prompt (~20 lines)
4. 🗑️ Delete `QUICK_REFERENCE.md` (fully redundant)
5. ✂️ Simplify `README.md` to ~100 lines (overview + usage only)
6. ✂️ Simplify `PR_TEMPLATE_SYSTEM.md` to ~150 lines (conventions + quick commands)

**Result**:

- Copilot has everything in one file (1,086 lines - still manageable)
- Users have lightweight reference docs
- Reduced fragmentation (4 files → 3 files)
- Reduced duplication (~800 lines → ~250 lines of supplementary docs)

**Pros**:

- ✅ Single source of truth for Copilot
- ✅ Easier maintenance
- ✅ Less confusion

**Cons**:

- ⚠️ Main prompt slightly longer (1,046 → 1,086 lines)

---

### Option B: Minimal Changes (Keep Reference Docs)

**Changes**:

1. ✅ Keep `workstream-execution.prompt.md` (1,046 lines)
2. ➕ Add branch naming conventions to main prompt (~20 lines)
3. ➕ Add commit message format to main prompt (~20 lines)
4. ✅ Keep `README.md` as-is (directory documentation)
5. ✅ Keep `QUICK_REFERENCE.md` as-is (user convenience)
6. ✅ Keep `PR_TEMPLATE_SYSTEM.md` as-is (process reference)

**Result**:

- Copilot has everything in main prompt
- Users have full reference documentation
- Duplication remains

**Pros**:

- ✅ Least disruptive
- ✅ Users have quick references

**Cons**:

- ❌ Maintenance burden continues
- ❌ Fragmentation remains
- ❌ Duplication across files

---

### Option C: Single File Approach (Most Aggressive)

**Changes**:

1. ✅ Keep `workstream-execution.prompt.md` (1,046 lines)
2. ➕ Add branch naming conventions to main prompt (~20 lines)
3. ➕ Add commit message format to main prompt (~20 lines)
4. ➕ Add "How to Use This Prompt" section to main prompt (~50 lines)
5. 🗑️ Delete `README.md`, `QUICK_REFERENCE.md`, `PR_TEMPLATE_SYSTEM.md`
6. ➕ Add brief note in directory: "See workstream-execution.prompt.md for all documentation"

**Result**:

- Single file (1,136 lines - still reasonable)
- Zero fragmentation
- Zero duplication
- Main prompt becomes self-documenting

**Pros**:

- ✅ Absolute simplicity
- ✅ Zero maintenance burden for supplementary docs
- ✅ Copilot and humans read same file

**Cons**:

- ⚠️ Main prompt is both execution guide AND usage documentation
- ⚠️ No separate "quick start" for users (but prompt is well-structured)

---

## 💡 My Recommendation: **Option A** (Aggressive Consolidation)

### Why Option A?

1. **Copilot-First Design**: Everything Copilot needs in one file
2. **Reasonable Complexity**: 1,086 lines is still manageable
3. **Reduced Maintenance**: ~75% reduction in supplementary docs
4. **Clear Separation**: Main prompt = execution, slim docs = reference
5. **No Loss of Value**: All critical info preserved

### What Changes

#### Main Prompt: Add Branch & Commit Conventions

Add to "About Contribution Workflow" section (~line 197):

```markdown
### About Contribution Workflow

1. **What branch should I create?**
   
   **FerrisScript Convention**:
   - Bug fixes: `bugfix/issue-description` or `fix/issue-description`
   - Features: `feature/feature-name` or `feat/feature-name`
   - Documentation: `docs/doc-update` or `doc/doc-update`
   - Other: Use descriptive name (e.g., `refactor/parser-cleanup`)
   
   **Rationale**: Branch name determines PR template (auto-applied by GitHub Actions)

2. **What's the commit message format?**
   
   **FerrisScript Convention**: Conventional Commits
   - Format: `type(scope): description`
   - Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `perf`, `ci`
   - Example: `feat(parser): add error recovery support`
   - Example: `fix(runtime): handle null pointer in expression evaluation`
   - Example: `docs: update LEARNINGS.md with Phase 3C insights`
```

#### README.md: Simplify to ~100 lines

Keep:

- Purpose (what this directory is)
- When to use (✅ vs ❌ examples)
- Quick start (usage examples)

Remove:

- Best practices (refer to main prompt)
- Expected flow (refer to main prompt)
- Detailed examples (refer to main prompt)

New structure (~100 lines):

```markdown
# GitHub Copilot Workstream Prompts

## 📋 Purpose
[2 paragraphs]

## 🚀 Quick Start
[Simple invocation example]

## 🎯 When to Use
[✅ vs ❌ table]

## 📂 Files
- workstream-execution.prompt.md - Main prompt (read this!)
- PR_TEMPLATE_SYSTEM.md - Branch/PR conventions reference

## 📖 Full Documentation
See workstream-execution.prompt.md for complete methodology, questions, best practices, and examples.
```

#### PR_TEMPLATE_SYSTEM.md: Simplify to ~150 lines

Keep:

- Branch naming table
- Quick commands reference
- Link to CONTRIBUTING.md for details

Remove:

- Detailed examples (they're in CONTRIBUTING.md)
- Troubleshooting (GitHub docs cover this)
- Long explanations (keep it reference-style)

#### QUICK_REFERENCE.md: DELETE

Fully redundant. Users can read the main prompt (it's well-structured with clear sections).

---

## 📋 Implementation Plan

### Step 1: Update Main Prompt

- [ ] Add branch naming conventions to "About Contribution Workflow"
- [ ] Add commit message format to "About Contribution Workflow"

### Step 2: Simplify README.md

- [ ] Reduce to ~100 lines (purpose + when to use + quick start)
- [ ] Remove duplicated best practices
- [ ] Remove duplicated flow diagrams
- [ ] Add "see main prompt for details" references

### Step 3: Simplify PR_TEMPLATE_SYSTEM.md

- [ ] Reduce to ~150 lines (conventions + quick commands)
- [ ] Remove detailed examples
- [ ] Keep as reference card

### Step 4: Delete QUICK_REFERENCE.md

- [ ] Remove file
- [ ] Update any links pointing to it

### Step 5: Test

- [ ] Verify main prompt has all critical info
- [ ] Test Copilot invocation with updated prompt
- [ ] Verify README provides useful quick start
- [ ] Verify PR_TEMPLATE_SYSTEM is useful reference

---

## 🎓 Maintenance Going Forward

### Single Source of Truth Principle

- **Execution methodology**: Main prompt ONLY
- **Process conventions** (branch/commit): Main prompt + reference card
- **Usage examples**: Main prompt ONLY
- **Best practices**: Main prompt ONLY

### When to Update

- **Main Prompt**: After every workstream with learnings
- **Reference Docs**: Only when conventions change (rare)

### Future Additions

If you want to add new execution guidance:

1. ✅ Add to main prompt (Copilot sees it)
2. ❌ Don't add to README (Copilot doesn't see it)

If you want to add new process documentation:

1. ✅ Add to CONTRIBUTING.md (primary process docs)
2. ⚠️ Add to reference card only if frequently referenced

---

## 📊 Impact Summary

| Metric | Before | After (Option A) | Change |
|--------|--------|------------------|--------|
| Files | 4 | 3 | -25% |
| Total Lines | ~1,864 | ~1,336 | -28% |
| Main Prompt | 1,046 | 1,086 | +4% |
| Supplementary | 818 | 250 | -69% |
| Duplication | High | Low | ✅ |
| Maintenance | High | Low | ✅ |
| Copilot Clarity | Good | Excellent | ✅ |

---

## ✅ Conclusion

**Recommendation**: Implement **Option A** (Aggressive Consolidation)

**Why**:

- Copilot gets everything in one place
- Reduced maintenance burden (69% reduction in supplementary docs)
- Clearer for both Copilot and humans
- Minimal increase to main prompt size (+40 lines = 4%)

**Next Steps**:

1. Review this analysis
2. Approve Option A (or request alternative)
3. Implement changes
4. Test with Copilot invocation
5. Update PR #36 with these changes

---

**Ready to proceed?** 🚀
