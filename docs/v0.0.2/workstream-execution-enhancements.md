# Workstream Execution Prompt - Recommended Enhancements

**Based on**: Phase 1 Edge Case Tests execution (Oct 3, 2025)  
**Context**: Successfully completed 15 tests in 3.5h (50% faster than estimated)  
**Key Learning**: Small refinements can prevent time waste and improve quality

---

## High-Priority Additions

### 1. Pre-Flight Checks Section (NEW - Insert after "Your Mission")

```markdown
## 🔍 Pre-Flight Checks

Before starting any work:

1. **Verify Current Branch**: `git status` - on correct branch?
2. **Check Recent History**: `git log --oneline -5` for context
3. **Build Baseline**: Run build command to ensure clean start
4. **Check for Manual Edits**: Context may show user edited files - ALWAYS read current contents
5. **Review Uncommitted Changes**: Any stashed work or WIP?
```

**Why**: Prevents assumptions about file state, ensures clean starting point.

---

### 2. Code Structure Discovery (ADD to "Context Discovery")

```markdown
### About Code Structure (Check FIRST!)

**Before writing any code**:

1. **Examine data structures**: What fields exist? (e.g., `Program.global_vars` not `.statements`)
2. **Read existing tests**: Find 1-2 similar tests, copy their patterns
3. **Note test location**: Inline `mod tests` or `tests/` directory?
4. **Check test helpers**: Existing utilities, assertion macros?
5. **Verify imports**: What modules/functions are available?

**Action Template**:
```rust
// Step 1: Read struct definition
// Step 2: Find existing test for reference
// Step 3: Copy pattern, adapt to new test
// Step 4: Verify field names match actual code
```

**Why**: Prevents wasting time with wrong field names, ensures tests follow project patterns.

---

### 3. Execution Strategy Options (ADD to "Planning Phase")

```markdown
## 🔄 Choose Execution Strategy

**Always propose ONE of these approaches and let user choose**:

### Option A: Full Sequential (Large PR)
- All phases in one branch → single large PR
- **Use when**: Tightly coupled work, stable requirements
- **Pros**: Complete solution, one review
- **Cons**: Long review time, high risk

### Option B: Phase-by-Phase (Medium PRs)
- 1-2 phases per PR → multiple medium PRs  
- **Use when**: Independent phases, want some feedback
- **Pros**: Moderate PR size, incremental progress
- **Cons**: Some context switching

### Option C: Incremental Validation (Small PRs) ✅ **RECOMMENDED**
- Smallest viable unit per PR (e.g., just tests for one category)
- **Use when**: Want to validate approach, uncertain requirements
- **Pros**: Fast feedback, low risk, easy review
- **Cons**: More PRs to manage

**Default to C unless user specifies otherwise.**
```

**Why**: Makes iteration strategy explicit, sets expectations for PR size and timing.

---

### 4. Quality Checks Timing (ENHANCE existing section)

```markdown
## ⚙️ When to Run Quality Checks

### During Development (Incremental)
- ✅ After each file: `cargo build` (does it compile?)
- ✅ After each test file: `cargo test --test <name>` (does THIS test pass?)
- ✅ After major change: Run affected tests

### Before Commit (Comprehensive)
- ✅ `cargo test --workspace` - all tests
- ✅ `cargo clippy --workspace --tests` - linting
- ✅ `cargo fmt --all` - formatting
- ✅ `npm run docs:lint` - markdown (if applicable)
- ✅ `git status` - review all changed files
- ✅ `git diff` - scan changes for unintended edits

### After Push (User Responsibilities)
⚠️ **Always mention to user**:
- "You may need to run lint:fix or project-specific checks"
- "Please verify link checking, cross-platform builds, etc."
- "CI will validate on other platforms (Linux, macOS)"

**Don't assume push = done!**
```

**Why**: You had to re-run lint:fix. I should have mentioned this explicitly.

---

### 5. Manual Edit Detection (NEW - Insert in "Common Pitfalls")

```markdown
### 9. Not Checking for Manual Edits

❌ **Bad**: Assume files match your last edit  
✅ **Good**: Check context for "user made manual edits", always read current contents

**Detection Pattern**:
- Context shows: "Made manual edits to file.rs"
- Action: ALWAYS `read_file` before making assumptions
- Ask: "I see you edited [files]. Should I review changes first?"

### 10. Wrong Data Structure Assumptions

❌ **Bad**: Write tests based on documentation/assumptions  
✅ **Good**: Read actual struct definition first, verify field names

**Example**: 
- Assumed: `Program.statements`
- Actual: `Program.global_vars` and `Program.functions`
- Solution: Read `ast.rs` before writing tests

### 11. Not Mentioning Post-Push Checks

❌ **Bad**: "All done! PR created. ✅"  
✅ **Good**: "PR created. Note: You may need to run lint:fix, link checking, or other project-specific validations."
```

**Why**: These were actual pitfalls I encountered - make them explicit warnings.

---

### 6. Summary Document Template (NEW - Add to "Deliverables")

```markdown
## 📄 Summary Document (Always Create)

**File**: `docs/[area]/[WORKSTREAM]_SUMMARY.md`

**Template**:
```markdown
# [Workstream Name] Summary

**Date**: October 3, 2025
**Time**: 3.5h actual vs. 7h estimated (50% faster)
**Branch**: feature/[name]
**PR**: #[number]
**Status**: ✅ Complete / ⏳ In Progress / ❌ Blocked

## Deliverables
- Concrete list of what was created/modified
- Test files with test counts
- Documentation files updated

## Test Results
```bash
cargo test --workspace
# All X tests passing
```

## Key Discoveries

1. **Technical**: [Data structure insights, API behaviors]
2. **Process**: [What worked well, what was inefficient]
3. **Limitations**: [Known issues, edge cases not covered]

## Time Analysis

| Phase | Estimated | Actual | Notes |
|-------|-----------|--------|-------|
| Phase 1 | 7h | 3.5h | Clear plan helped |

**Efficiency Factors**:

- What made work faster/slower than estimated
- Lessons for future estimates

## Recommendations

### For Current Release

- Merge this PR (reason)
- Document X finding

### For Future Work  

- Phase 2-N considerations
- Related improvements

### For Documentation

- Missing docs identified
- Unclear areas found

## Next Steps

1. **Immediate**: PR review, CI validation
2. **Short-term**: Address review feedback
3. **Long-term**: Remaining phases from plan

```

**Why**: Summary document was valuable for handoff - make it standard practice.

---

## Implementation Priority

1. **High Priority** (Most Impact):
   - Pre-Flight Checks (prevents assumptions)
   - Code Structure Discovery (prevents rework)
   - Manual Edit Detection (prevents overwrites)

2. **Medium Priority** (Improves Process):
   - Execution Strategy Options (sets expectations)
   - Quality Checks Timing (ensures completeness)

3. **Nice to Have** (Adds Polish):
   - Summary Document Template (improves handoff)

---

## Estimated Impact

**Time Savings**: 15-30 minutes per workstream
- 10 min: Not needing to fix wrong field names
- 5 min: Not needing to re-run forgotten checks  
- 10 min: Not needing to investigate manual edits

**Quality Improvements**:
- Fewer compilation errors
- Better test patterns
- More complete handoff documentation

**Process Improvements**:
- Clearer expectations
- Better communication
- Smoother iteration

---

## How to Apply

### Option A: Inline Additions
Add sections directly to existing prompt at marked locations

### Option B: Checklist Supplement  
Create separate `WORKSTREAM_CHECKLIST.md` that references main prompt

### Option C: Version 1.1
Create `workstream-execution-v1.1.prompt.md` with all enhancements

**Recommendation**: Option A (inline) for immediate benefit, promote to v1.1 after validation.

---

**End of Enhancement Recommendations**
