# Workstream Execution - Quick Reference

**TL;DR**: Invoke `/prompt #file:workstream-execution.prompt.md` + context → Copilot asks questions → Creates execution plan → Works systematically

---

## 🚀 Quick Start

```
/prompt #file:workstream-execution.prompt.md

Context:
- #file:[checklist or requirements file]
- Highlighted: [paste relevant text]
- Priority: [High/Medium/Low]
```

---

## 📋 What Happens Next

### 1. Context Gathering (5-10 min)

- Copilot reads your context
- Asks 5-15 clarifying questions
- You answer the questions
- Copilot confirms understanding

### 2. Execution Planning (10-15 min)

- Creates execution plan document
- Defines acceptance criteria  
- Breaks work into phases
- Creates TODO list
- Gets your approval

### 3. Implementation (varies)

- Works through phases one at a time
- Updates TODO list for visibility
- Runs quality checks after each phase
- Documents decisions as they're made

### 4. Validation & Documentation (30-60 min)

- Runs full test suite
- Runs all linting
- Verifies all acceptance criteria met
- Updates related documentation
- Creates summary with learnings

---

## ❓ Questions Copilot Will Ask

### About the Work (5 questions)

1. What's the primary goal?
2. What version is this for?
3. What type of release? (patch/minor/major)
4. Why is this important?
5. Source of requirements?

### About Prior Work (5 questions)

1. Has similar work been done?
2. Are there existing tests?
3. What documentation exists?
4. What patterns to follow?
5. What NOT to change?

### About Constraints (5 questions)

1. What changes are allowed?
2. What changes NOT allowed?
3. Performance requirements?
4. Platform considerations?
5. Timeline?

### About Quality (5 questions)

1. What tests must pass?
2. What linting must pass?
3. Test coverage target?
4. Documentation requirements?
5. Code review process?

### About Workflow (5 questions)

1. Branch naming?
2. Commit message format?
3. Where do files go?
4. What docs need updating?
5. How to track progress?

**Note**: Copilot won't ask ALL questions - only what's unclear from context.

---

## ✅ What You Get

### Execution Plan Document

```markdown
# [Workstream] Execution Plan

## Q&A: Context Gathering
[All questions and your answers recorded]

## Decisions Made
[Design decisions and trade-offs]

## Acceptance Criteria
[Specific, testable requirements]

## Execution Phases
[Phases with tasks and status]

## Deliverables
[Files to create/modify]

## Learnings & Discoveries
[What we learned]
```

### Quality Validation

- All tests pass
- All linting passes
- All acceptance criteria met
- Documentation updated
- Summary document created

---

## 📊 Progress Visibility

### TODO List Format

```markdown
- [x] Phase 0: Planning ✅
  - Created execution plan with Q&A
- [-] Phase 1: Implementation 🔄  
  - Working on task 2 of 5...
- [ ] Phase 2: Validation ⏸️
- [ ] Phase 3: Documentation ⏸️
```

**Legend**:

- `[x]` = Complete
- `[-]` = In Progress (current phase)
- `[ ]` = Not Started
- `[!]` = Blocked

---

## 🎯 Acceptance Criteria Examples

### Good (Specific, Measurable)

✅ "All errors must include line number, column, and ±2 lines of context"  
✅ "Empty file tests must pass without panicking (3 tests)"  
✅ "Test coverage increases by 5-10% (measured with cargo tarpaulin)"  
✅ "All edge case tests complete in < 100ms each"

### Bad (Vague, Unmeasurable)

❌ "Improve error messages" (how? by how much?)  
❌ "Add some tests" (which ones? how many?)  
❌ "Better documentation" (what makes it better?)  
❌ "Fix performance" (what's the target?)

---

## 🚨 Common Issues & Solutions

### Issue: Copilot Doesn't Ask Enough Questions

**Solution**: Provide less context upfront, or say "Please ask clarifying questions about [specific area]"

### Issue: Copilot Asks Too Many Questions

**Solution**: Provide more context upfront (attach more files, be more specific)

### Issue: Work Doesn't Fit Template

**Solution**: The prompt is generic! Just provide your specific context - Copilot adapts

### Issue: Unclear Acceptance Criteria

**Solution**: In Q&A phase, explicitly ask "What makes this complete?" and be specific

### Issue: Copilot Skips Quality Checks

**Solution**: Remind it to run the quality checklist before marking phases complete

---

## 💡 Pro Tips

### Tip 1: Attach Context Files

```
/prompt #file:workstream-execution.prompt.md

Context:
- #file:docs/v0.0.2/v0.0.2-CHECKLIST.md
- #file:docs/v0.0.2/LEARNINGS.md (see "Edge Case Tests" section)
- #file:docs/v0.0.2/TEST_COVERAGE_ANALYSIS.md (see gaps)
```

More context = fewer questions needed.

### Tip 2: Highlight Specific Text

Copy relevant checklist items or requirements and paste them:

```
Highlighted from checklist:
"- [ ] Test edge cases: empty files, comments-only, long variable names
 - [ ] Better error messages with line numbers and context
 - [ ] Colorized error output for Godot console"
```

### Tip 3: Specify Priority

Help Copilot prioritize:

```
Priority: High (blocking v0.0.2 release)
Timeline: Complete by end of week
```

### Tip 4: Reference Prior Work

```
Related work:
- PR #42 added 20 edge case tests (different ones)
- See docs/v0.0.2/LEARNINGS.md for what's already done
```

### Tip 5: Stay Engaged

- Answer questions promptly
- Review execution plan before implementation
- Check TODO updates periodically
- Provide feedback on approach

---

## 📁 File Locations

### Primary Prompt

- `.github/prompts/workstream-execution.prompt.md` - The generic prompt

### Documentation

- `.github/prompts/README.md` - Full documentation
- `.github/prompts/QUICK_REFERENCE.md` - This file

### Examples

- `.github/prompts/edge-case-error-handling-v0.0.2.md` - Legacy example (v0.0.2 specific)

---

## 🔗 Quick Links

- [Full README](./ README.md) - Complete documentation
- [Main Prompt](./workstream-execution.prompt.md) - The prompt file
- [CONTRIBUTING.md](../../CONTRIBUTING.md) - Project contribution rules
- [DEVELOPMENT.md](../../docs/DEVELOPMENT.md) - Development workflow

---

## ⚡ One-Liner Examples

### Edge Case Testing

```
/prompt #file:workstream-execution.prompt.md
Work: Edge case tests for empty files, comments-only, long identifiers (v0.0.2)
```

### Error Handling

```
/prompt #file:workstream-execution.prompt.md  
Work: Better error messages with line numbers + colorized Godot output (v0.0.2)
```

### Refactoring

```
/prompt #file:workstream-execution.prompt.md
Work: Refactor parser for error recovery (no breaking changes, maintain perf)
```

### Documentation

```
/prompt #file:workstream-execution.prompt.md
Work: Reorganize v0.0.2 docs by category (dev guide, architecture, learnings)
```

---

**Last Updated**: October 3, 2025  
**Keep This Handy**: Pin this file for quick reference when starting new workstreams!
