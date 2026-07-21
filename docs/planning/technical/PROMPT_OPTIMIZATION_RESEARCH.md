# Prompt Optimization Research: Request Efficiency over Token Efficiency

**Date**: October 7, 2025  
**Context**: GitHub Copilot premium requests optimization  
**Goal**: Achieve 1 request per feature (plan + execute + finalize in one go)

---

## 🎯 Problem Statement

### Current Constraint Model

**User pays per premium Copilot request, NOT per token.**

This fundamentally reframes optimization priorities:

| Metric | Old Model (Token-Limited) | New Model (Request-Limited) |
|--------|---------------------------|----------------------------|
| **Long prompts** | 🔴 Expensive (consumes tokens) | 🟢 Free (same cost as short) |
| **Multiple roundtrips** | 🟢 Acceptable (resets context) | 🔴 Expensive (each = 1 request) |
| **Re-prompts/clarifications** | 🟢 Normal workflow | 🔴 Burns premium allotment |
| **Front-loaded context** | ⚠️ Token waste | ✅ Critical investment |

### Current Cost Analysis

Typical feature with current workstream-execution prompt:

| Step | Actor | Description | Premium Request |
|------|-------|-------------|-----------------|
| 1️⃣ | User | `/prompt #file:workstream-execution.prompt.md` + context | 1 |
| 2️⃣ | Copilot | Proposes plan + asks clarifications | ✅ 1 |
| 3️⃣ | User | Answers questions + "start execution" | 1 |
| 4️⃣ | Copilot | Executes full plan | ✅ 1 |
| 5️⃣ | User | Reviews output | 0 (free) |

**Typical feature cost**: 2-4 premium requests

**May climb to 3-4 if**:

- Copilot misinterprets roadmap
- Asks redundant follow-ups
- Needs separate doc/test generation pass
- Requires bug fixes due to validation failures

---

## 🎯 Optimization Goal

> **Achieve 1 premium request per feature**, where Copilot can plan + execute + finalize inline.

**Success Pattern**:

```
User: /prompt #file:workstream-execution.prompt.md + context
      Feature: [description]

Copilot: [brief plan] + [execution] + [docs] + [tests] + ✅ Complete
```

**Cost**: 1 premium request total

---

## 🔧 Optimization Strategies

### Strategy 1: Clarification Minimization ⭐⭐⭐ (HIGH IMPACT)

**Problem**: Copilot pauses execution to ask questions, requiring another request to proceed.

**Current Behavior**:

```
Request 1: Copilot asks 15 clarifying questions
Request 2: User answers → Copilot proceeds
```

**Optimized Behavior**:

```
Request 1: Copilot makes reasonable assumptions (noted inline) → proceeds to completion
```

**Implementation**:

```markdown
## 🧠 Ambiguity Resolution Strategy

**Default Approach: Self-Resolve with Documented Assumptions**

When encountering ambiguity during context gathering:

1. **Assess Criticality**: Could this ambiguity cause breaking changes or critical errors?
   - **High Risk** (e.g., API contract changes, data loss): Stop and ask
   - **Low-Medium Risk** (e.g., test organization, doc structure): Make reasonable assumption

2. **Make Inference**: Use project patterns, existing code, and roadmap context to infer intent

3. **Document Assumption**: Note inline with format:
   ```

   ⚠️ ASSUMPTION: [What was assumed] based on [reasoning]

   ```

4. **Proceed**: Continue execution without waiting for clarification

**Examples of Self-Resolvable Ambiguities**:
- Test file naming conventions → Follow existing patterns
- Documentation structure → Mirror related docs
- Code organization → Match similar features
- Variable naming → Use project style guide patterns
- Error message wording → Keep consistent with existing errors

**Examples Requiring Clarification**:
- Breaking API changes → Confirm with user
- Performance trade-offs → Get user priority
- Security implications → Explicit approval needed
- Version target unclear → Ask explicitly
```

**Impact**: Converts 2-request cycles (ask → answer → execute) into 1-request (assume → execute → note).

**Estimated Savings**: 1 request per feature (50% reduction)

---

### Strategy 2: Plan + Execute Fusion ⭐⭐⭐ (HIGH IMPACT)

**Problem**: Current prompt separates planning from execution, requiring user approval between phases.

**Current Behavior**:

```
Request 1: Create detailed execution plan → Wait for approval
Request 2: Execute plan phases
```

**Optimized Behavior**:

```
Request 1: Brief inline plan (≤5 bullets) → Immediate execution → Complete
```

**Implementation**:

```markdown
## 🔄 Execution Strategy Enhancement

**Default Mode: Fused Plan + Execute** ✅

When requirements are clear from attached context:

1. **Generate Brief Plan** (≤5 bullets):
   ```

## Execution Summary

   1. [Phase 1 goal]
   2. [Phase 2 goal]
   3. [Phase 3 goal]

   ```

2. **Proceed Immediately to Execution**: No approval required

3. **Document as You Go**: Record decisions inline during implementation

**Fallback: Explicit Planning Mode**

Only use separate planning phase if:
- User explicitly requests it: "Create execution plan first"
- Ambiguity is genuinely high-risk (breaking changes, major refactoring)
- Work scope is unclear even after context analysis

**Mode Indicators**:
- User says "plan this" → Planning-only mode
- User says "implement [feature]" → Fused mode (default)
- No explicit instruction + clear requirements → Fused mode (default)
```

**Impact**: Merges 2 requests (plan approval + execution) into 1 (fused execution).

**Estimated Savings**: 1 request per feature (50% reduction)

---

### Strategy 3: Deterministic Completion Behavior ⭐⭐ (MEDIUM-HIGH IMPACT)

**Problem**: Some executions stop mid-way with incomplete output or "waiting for approval" pauses.

**Current Behavior**:

```
Request 1: Partial implementation → "Is this what you wanted?"
Request 2: User confirms → Continue
```

**Optimized Behavior**:

```
Request 1: Complete implementation + validation + ✅ marker
```

**Implementation**:

```markdown
## ✅ Definition of Done

A workstream execution is **COMPLETE** when ALL of the following are true:

### Code Deliverables
- ✅ All code files created/modified as planned
- ✅ All code compiles successfully (`cargo build`)
- ✅ All tests pass (`cargo test --workspace`)
- ✅ All linting passes (`cargo clippy -- -D warnings`)
- ✅ Code formatting applied (`cargo fmt --all`)

### Documentation Deliverables
- ✅ All documentation created/updated
- ✅ Markdown linting fixed (`npm run docs:fix`)
- ✅ Markdown linting passes (`npm run docs:lint`)
- ✅ All links validated (`npx markdown-link-check` on changed files)
- ✅ LEARNINGS.md updated with insights

### Validation Deliverables
- ✅ All acceptance criteria verified
- ✅ Self-review completed
- ✅ PR ready for human review

### Output Requirements
- ✅ Hierarchical output structure followed (see below)
- ✅ All assumptions documented
- ✅ Completion marker present: **✅ Workstream Execution Complete**

**DO NOT** end execution with:
- ❌ "Does this look good?"
- ❌ "Should I continue?"
- ❌ Incomplete implementation
- ❌ Untested code
- ❌ Unvalidated output

**ALWAYS** end execution with:
- ✅ Complete, validated implementation
- ✅ Clear completion marker
- ✅ Summary of what was delivered
```

**Impact**: Prevents partial executions requiring follow-up requests.

**Estimated Savings**: 0.5-1 request per feature (25-50% reduction in multi-phase work)

---

### Strategy 4: Self-Correction + Validation ⭐⭐ (MEDIUM IMPACT)

**Problem**: Failed builds or validation errors force re-runs to fix issues.

**Current Behavior**:

```
Request 1: Generate code → Push → CI fails on syntax error
Request 2: Fix syntax error → Push again
```

**Optimized Behavior**:

```
Request 1: Generate code → Self-validate → Fix issues → Push clean code
```

**Implementation**:

```markdown
## 🔍 Internal Validation Loop

**Before outputting final implementation, run this validation sequence:**

### Phase 1: Syntax Validation
```bash
# Does it compile?
cargo build --workspace
```

- **If fails**: Fix syntax errors automatically, retry
- **If passes**: Proceed to Phase 2

### Phase 2: Test Validation

```bash
# Do tests pass?
cargo test --workspace
```

- **If fails**: Analyze failure, fix logic errors, retry
- **If passes**: Proceed to Phase 3

### Phase 3: Quality Validation

```bash
# Does linting pass?
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check

# Does doc linting pass?
npm run docs:lint
```

- **If fails**: Auto-fix with `cargo fmt` and `npm run docs:fix`, verify clean
- **If passes**: Proceed to output

### Phase 4: Link Validation (if docs modified)

```bash
# For each modified markdown file
npx markdown-link-check <file.md>

# Also check key navigation files
npx markdown-link-check README.md
npx markdown-link-check docs/LEARNINGS.md
```

- **If fails**: Fix broken links, retry
- **If passes**: Ready for output

**Only after ALL validations pass**: Output final implementation

**Validation Failure Limit**: If 3 consecutive attempts fail, output current state with:

```
⚠️ VALIDATION ISSUE: [Description of failure]
Attempted fixes: [What was tried]
Recommendation: [Manual intervention needed]
```

```

**Impact**: Fewer re-prompts due to validation failures.

**Estimated Savings**: 0.5-1 request per feature (especially for complex implementations)

---

### Strategy 5: Hierarchical Output Structure ⭐ (LOW-MEDIUM IMPACT)

**Problem**: Large, interleaved outputs cause confusion about what was delivered.

**Current Behavior**:
```

[Code snippet] [explanation] [another code snippet] [doc update]
User: "Did you update the README?" → Request 2 to clarify

```

**Optimized Behavior**:
```

Clear sections: Summary → Implementation → Docs → Tests → Complete
User: [Can see everything at a glance]

```

**Implementation**:

```markdown
## 📋 Required Output Structure

All workstream executions MUST follow this structure:

### 1. Executive Summary (Top of Output)
```markdown
## 🎯 Workstream Summary

**Goal**: [One-line description]
**Context**: [Where this fits in roadmap]
**Approach**: [Key strategy/decisions]
**Assumptions Made**: [List any assumptions with ⚠️ markers]
```

### 2. Implementation Section

```markdown
## 💻 Implementation

### Files Created
- `path/to/file1.rs` - [Brief description]
- `path/to/file2.rs` - [Brief description]

### Files Modified
- `path/to/file3.rs` - [What changed]
- `Cargo.toml` - [Dependencies added]

### Key Changes
- [Major change 1]
- [Major change 2]
```

### 3. Documentation Section

```markdown
## 📚 Documentation Updates

### Created
- `docs/planning/technical/NEW_DOC.md` - [Purpose]

### Updated
- `README.md` - [Section updated]
- `docs/LEARNINGS.md` - [Insights added]

### Link Validation Results
✅ All links verified in 5 files:
- README.md (23 links checked, 0 broken)
- docs/LEARNINGS.md (12 links checked, 0 broken)
- [etc.]
```

### 4. Test Section

```markdown
## 🧪 Testing Results

### Tests Added
- `tests/integration/feature_test.rs` - [Coverage]
- 12 new test cases covering edge cases

### Test Results
```

cargo test --workspace
✅ 234 tests passed (0 failed)

```

### Coverage Impact
- Previous: 78.5%
- Current: 79.2%
- Delta: +0.7%
```

### 5. Validation Section

```markdown
## ✅ Validation Results

### Build Status
```

cargo build --workspace
✅ Compilation successful (0 warnings)

```

### Linting Status
```

cargo clippy -- -D warnings
✅ All linting passed (0 warnings)

npm run docs:lint
✅ Markdown linting passed (0 errors)

```

### Acceptance Criteria
- [x] Criterion 1: [Evidence]
- [x] Criterion 2: [Evidence]
- [x] Criterion 3: [Evidence]
```

### 6. Next Steps Section

```markdown
## 🔮 Recommendations & Deferred Work

### High Priority (v0.0.4)
1. [Deferred item] - [Rationale]

### Medium Priority (v0.0.5)
2. [Future enhancement] - [Rationale]

### Assumptions to Validate
- ⚠️ ASSUMPTION: [List any assumptions user should verify]
```

### 7. Completion Marker (Required)

```markdown
## ✅ Workstream Execution Complete

**Deliverables**: [N] code files, [M] doc files, [K] tests
**Status**: Ready for PR and human review
**Next Action**: User reviews changes and creates PR
```

**This marker MUST be present at end of output.**

```

**Impact**: Easier verification without re-query, clearer communication.

**Estimated Savings**: 0.25 requests per feature (reduces clarification questions)

---

### Strategy 6: Execution Mode Toggle ⭐ (LOW IMPACT)

**Problem**: User sometimes wants quick planning without burning premium request on execution.

**Implementation**:

```markdown
## 🎛️ Execution Modes (Optional User Control)

Modes can be specified in user prompt: `/prompt #file:workstream-execution.prompt.md mode=[mode]`

### Mode: `full` (Default)
- Plan + Execute + Document + Test in one pass
- **Use when**: Ready to implement feature
- **Cost**: 1 premium request
- **Output**: Complete implementation

### Mode: `plan`
- Only create execution plan with phases
- **Use when**: Exploring approach, not ready to execute
- **Cost**: 1 premium request (but faster)
- **Output**: Detailed plan document only

### Mode: `execute`
- Assume plan already exists, produce code/docs only
- **Use when**: Plan approved, ready for implementation
- **Cost**: 1 premium request
- **Output**: Implementation without re-planning

**Default**: If no mode specified, assume `mode=full`

**Examples**:
```

/prompt #file:workstream-execution.prompt.md mode=plan
Feature: Add error recovery to parser

/prompt #file:workstream-execution.prompt.md mode=full
Feature: Add error recovery to parser (proceed with implementation)

```
```

**Impact**: Gives user control over request granularity, but doesn't reduce requests unless user explicitly chooses planning-only mode.

**Estimated Savings**: 0 requests (flexibility feature, not optimization)

---

### Strategy 7: Error Recovery Directive ⭐⭐ (MEDIUM IMPACT)

**Problem**: Copilot sometimes stops execution due to confusion or uncertainty.

**Current Behavior**:

```
Copilot encounters edge case → "I'm not sure how to handle this" → Stops
User: "Use approach X" → Request 2
```

**Optimized Behavior**:

```
Copilot encounters edge case → Makes best-effort decision → Notes assumption → Continues
```

**Implementation**:

```markdown
## 🚀 Forward Progress Mandate

**Core Principle**: Always make forward progress unless ambiguity is genuinely critical.

### When Encountering Uncertainty

**DO**:
1. Make a reasonable inference based on:
   - Existing code patterns
   - Project conventions
   - Common best practices
   - Similar features in codebase
2. Document the assumption inline:
   ```

   ⚠️ ASSUMPTION: Using pattern X (mirroring similar feature Y)

   ```
3. Continue execution with chosen approach
4. Note alternative approaches in "Recommendations" section

**DON'T**:
- ❌ Stop and ask "How should I handle this?"
- ❌ Output incomplete work waiting for guidance
- ❌ Leave placeholder comments like `// TODO: Implement this`

### Exception: Critical Ambiguities

Stop ONLY if:
- Breaking API changes with unclear contract
- Security implications requiring explicit approval
- Data loss/corruption risk
- Explicit conflict in requirements

For everything else: **proceed with best effort**.

### Example Scenarios

| Scenario | Current Behavior | Optimized Behavior |
|----------|------------------|-------------------|
| Unsure about test file location | "Where should I put tests?" | Check existing tests → mirror pattern → proceed |
| Variable naming ambiguous | "What should I name this?" | Follow Rust conventions → proceed |
| Edge case handling unclear | "How to handle X?" | Implement defensive approach → note assumption |
| Doc section ordering unclear | "Where does this go?" | Match similar docs → proceed |
```

**Impact**: Prevents execution halts, encourages completion in single request.

**Estimated Savings**: 0.5 requests per feature (for complex features with edge cases)

---

### Strategy 8: Context Pre-Loading ⭐⭐ (MEDIUM-HIGH IMPACT)

**Problem**: User must repeatedly provide same context across features.

**Current Behavior**:

```
Each feature: User attaches same roadmap, conventions, checklist templates
```

**Optimized Behavior**:

```
Prompt file contains all static context → User only provides feature-specific details
```

**Implementation** (Already partially implemented, enhance further):

```markdown
## 📚 Pre-Loaded Project Context

**These are baked into this prompt - you don't need to ask for them:**

### Project Identity
- **Name**: FerrisScript
- **Language**: Rust (for compiler/runtime), FerrisScript (language itself)
- **Domain**: Godot game engine scripting language
- **Version**: v0.0.3 (current), targeting v0.1.0 (stable release)

### Repository Structure
```

FerrisScript/
├── crates/
│   ├── compiler/    # Lexer, parser, type checker
│   ├── runtime/     # Runtime execution engine
│   └── godot_bind/  # Godot GDExtension bindings
├── docs/
│   ├── planning/    # Roadmaps, research, execution plans
│   ├── archive/     # Version-specific historical docs
│   └── *.md         # Architecture, development guides
├── examples/        # .ferris example programs
├── godot_test/      # Godot integration test project
└── scripts/         # Automation scripts (coverage, linting, git hooks)

```

### Code Conventions (Rust)
- **Style**: Standard Rust formatting (`rustfmt`)
- **Linting**: Clippy in strict mode (`-D warnings`)
- **Testing**: Inline `mod tests` blocks, integration tests in `tests/`
- **Naming**: Snake_case for functions/vars, PascalCase for types/structs
- **Error Handling**: Result types, descriptive error messages
- **Documentation**: Rustdoc comments for public APIs

### Documentation Conventions
- **Format**: Markdown (CommonMark)
- **Linting**: markdownlint with `npm run docs:fix` before commit
- **Links**: Follow DOCUMENTATION_LINKING_GUIDELINES.md
  - Long-standing docs (README, CONTRIBUTING) avoid version-specific refs
  - Use generalized/evergreen content
  - Always validate links with `npx markdown-link-check`
- **Dates**: ALWAYS use current date from context (e.g., October 2025, NOT January)

### Branch Naming (Auto-selects PR template)
- Bug fixes: `bugfix/` or `fix/`
- Features: `feature/` or `feat/`
- Documentation: `docs/` or `doc/`
- Refactoring: `refactor/`
- Other: Descriptive name

### Commit Message Format (Conventional Commits)
- Format: `type(scope): description`
- Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `perf`, `ci`
- Examples:
  - `feat(parser): add error recovery support`
  - `fix(runtime): handle null pointer in expression evaluation`
  - `docs: update LEARNINGS.md with Phase 3C insights`

### Quality Standards
- **Build**: Must compile (`cargo build --workspace`)
- **Tests**: Must pass (`cargo test --workspace`)
- **Linting**: Clippy warnings = errors (`-D warnings`)
- **Docs**: Markdown linting must pass (`npm run docs:lint`)
- **Links**: All links must be valid (checked before commit)
- **Coverage**: Maintain or improve coverage (tracked in `tarpaulin.toml`)

### Test Commands
```bash
# Build
cargo build --workspace

# Test
cargo test --workspace

# Lint
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all -- --check

# Doc linting
npm run docs:fix  # Auto-fix issues
npm run docs:lint # Verify clean

# Link checking
npx markdown-link-check <file.md>
```

### CI/CD Pipeline

- GitHub Actions runs on: `push`, `pull_request`
- Checks: Build, test, clippy, rustfmt, doc linting
- Code coverage: Generated with tarpaulin, uploaded to Codecov
- PR required: At least one approval, all checks passing

### Version Planning

- **v0.0.3** (current): Error handling, basic runtime
- **v0.0.4** (next): Enhanced diagnostics, more examples
- **v0.1.0** (stable): Feature-complete, production-ready

**You have all this context - don't ask about it again!**

```

**Impact**: User provides only feature-specific context, not project basics.

**Estimated Savings**: 0.25-0.5 requests per feature (fewer clarifying questions)

---

## 📊 Expected Impact Summary

| Strategy | Impact | Savings (Requests) | Priority |
|----------|--------|-------------------|----------|
| 1. Clarification Minimization | ⭐⭐⭐ High | 1.0 per feature | Must-Have |
| 2. Plan + Execute Fusion | ⭐⭐⭐ High | 1.0 per feature | Must-Have |
| 3. Deterministic Completion | ⭐⭐ Medium-High | 0.5-1.0 per feature | Should-Have |
| 4. Self-Correction + Validation | ⭐⭐ Medium | 0.5-1.0 per feature | Should-Have |
| 5. Hierarchical Output | ⭐ Low-Medium | 0.25 per feature | Nice-to-Have |
| 6. Execution Mode Toggle | ⭐ Low | 0 (flexibility) | Nice-to-Have |
| 7. Error Recovery | ⭐⭐ Medium | 0.5 per feature | Should-Have |
| 8. Context Pre-Loading | ⭐⭐ Medium-High | 0.25-0.5 per feature | Should-Have |

**Current Average**: 2-4 requests per feature  
**Optimized Average**: 1-1.5 requests per feature  
**Potential Savings**: 50-75% reduction in premium requests

---

## 🎯 Implementation Recommendations

### Phase 1: High-Impact (Implement First)
1. **Clarification Minimization** - Add "Ambiguity Resolution Strategy" section
2. **Plan + Execute Fusion** - Make fused execution the default mode
3. **Deterministic Completion** - Add explicit "Definition of Done" checklist

**Expected Outcome**: Most features complete in 1 request

### Phase 2: Risk Mitigation (Implement Second)
4. **Self-Correction + Validation** - Add internal validation loop
5. **Error Recovery** - Add "Forward Progress Mandate"
6. **Context Pre-Loading** - Expand embedded project context

**Expected Outcome**: Complex features complete in 1 request (currently need 2-3)

### Phase 3: Polish (Implement Third)
7. **Hierarchical Output** - Standardize output structure
8. **Execution Mode Toggle** - Add optional planning-only mode

**Expected Outcome**: Better UX, clearer communication

---

## 🚀 Next Steps

1. **Review this research** with project stakeholders
2. **Update workstream-execution.prompt.md** with Phase 1 changes
3. **Test on 2-3 features** to validate savings
4. **Iterate based on real-world usage**
5. **Implement Phase 2** once Phase 1 proven
6. **Track metrics**: Requests per feature before/after

---

## 📝 Success Metrics

### Quantitative
- **Requests per feature**: Target 1.0 (currently 2-4)
- **Follow-up requests**: Target 0 (currently 1-3)
- **Validation failures**: Target 0 (currently 0.5-1 per feature)

### Qualitative
- User reports faster feature delivery
- Fewer "can you clarify?" back-and-forths
- More complete first outputs
- Higher confidence in agent execution

---

## 🔗 Related Documents

- [Workstream Execution Prompt](/.github/prompts/workstream-execution.prompt.md) - Current prompt file
- [Phase 3C Completion Report](/docs/PHASE_3_COMPLETION_REPORT.md) - Learnings from recent work
- [Version Planning](/docs/VERSION_PLANNING.md) - Roadmap context

---

**Remember**: In this economy, *maximize yield per prompt submit*. Every press of Enter should complete a full feature cycle! 🚀
