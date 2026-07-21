# FerrisScript Workstream Execution Guide

**Agent-agnostic workflow for multi-phase development tasks**

---

## Overview

This guide provides a structured approach for AI coding agents to execute complex, multi-phase work in the FerrisScript codebase. It's designed to work with any AI coding assistant (Claude Code, OpenCode, GitHub Copilot, Cursor, etc.).

**Core principles:**

- **Context first**: Understand the codebase before making changes
- **Incremental progress**: Complete smallest testable unit per PR
- **Quality gates**: Run all checks before declaring work complete
- **Document decisions**: Record assumptions and learnings

---

## Pre-Flight Checks

**Before starting any work:**

1. ✅ **Verify current branch**: `git status`
2. ✅ **Check for manual edits**: If context mentions "user made manual edits", read those files first
3. ✅ **Build baseline**: `cargo build --workspace`
4. ✅ **Review recent history**: `git log --oneline -5`
5. ✅ **Load relevant skill**: Read the appropriate skill from `docs/agent-skills/` based on which crate you're working in

---

## Execution Modes

### Mode: Full (Default)

**Use when:** Requirements are clear, ready to implement completely

**Behavior:**

1. Generate brief plan (≤5 bullets)
2. Implement all code changes
3. Write/update tests
4. Update documentation
5. Run all validations
6. Output completion summary

### Mode: Plan

**Use when:** Exploring approaches, want to review plan before implementation

**Behavior:**

1. Analyze requirements thoroughly
2. Create detailed execution plan
3. Define phases, tasks, acceptance criteria
4. Estimate effort
5. Stop after planning (no code generation)

### Mode: Execute

**Use when:** Plan already reviewed, proceed directly to implementation

**Behavior:**

1. Skip planning phase
2. Proceed directly to implementation
3. Complete all code, tests, docs
4. Run all validations

---

## Execution Strategy

**Default: Small Increments (Option C)**

- Complete smallest testable unit per PR
- Fast feedback, easy review, low risk
- Use when: Most cases (default choice)

**Alternative: Phase-by-Phase (Option B)**

- Complete 1-2 related phases per PR
- Use when: Phases are tightly coupled

**Alternative: Full Sequential (Option A)**

- All phases in one PR
- Use when: User explicitly requests it, work is indivisible

**Decision process:**

1. Default to Option C (small increments)
2. State your choice and proceed
3. Only ask if work seems indivisible

---

## Ambiguity Resolution

### Self-Resolve (Low-Medium Risk)

**Proceed with reasonable assumption when:**

- Test file naming conventions → Follow existing patterns
- Documentation structure → Mirror related docs
- Code organization → Match similar features
- Variable naming → Use project style guide
- Error message wording → Keep consistent with existing messages

**Document assumption inline:**

```
⚠️ ASSUMPTION: [What was assumed] based on [reasoning]
```

### Stop and Ask (High Risk)

**Ask user when:**

- Breaking API changes
- Performance trade-offs (speed vs memory vs maintainability)
- Security implications
- Version target unclear
- Major architectural decisions
- External dependencies
- Behavior changes

---

## Definition of Done

**Work is COMPLETE when ALL of the following are true:**

### Code Deliverables

- ✅ All code files created/modified as planned
- ✅ All code compiles: `cargo build --workspace`
- ✅ All tests pass: `cargo test --workspace`
- ✅ All linting passes: `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- ✅ Code formatting applied: `cargo fmt --all`

### Documentation Deliverables

- ✅ All documentation created/updated
- ✅ Markdown linting passes: `npm run docs:lint`
- ✅ All links validated (if applicable)
- ✅ CHANGELOG.md updated (for user-facing changes)

### Validation Deliverables

- ✅ All acceptance criteria verified
- ✅ Self-review completed
- ✅ No compilation warnings or errors
- ✅ No test failures
- ✅ No linting violations
- ✅ PR-ready state

### Output Requirements

- ✅ All assumptions documented with `⚠️ ASSUMPTION:` markers
- ✅ Hierarchical output structure (see below)
- ✅ Final completion marker: **"✅ Workstream Execution Complete"**

---

## Required Output Structure

### 1. Executive Summary

```markdown
## 🎯 Workstream Summary

**Goal**: [One-line description]
**Context**: [Where this fits in roadmap]
**Approach**: [Key strategy/decisions]
**Assumptions Made**: [List or "None"]
```

### 2. Implementation Section

```markdown
## 💻 Implementation

### Files Created
- `path/to/file.rs` - [Description]

### Files Modified
- `path/to/file.rs` - [What changed and why]

### Key Changes
1. [Major change with rationale]
```

### 3. Documentation Section

```markdown
## 📚 Documentation Updates

### Created
- `docs/planning/NEW_DOC.md` - [Purpose]

### Updated
- `README.md` - [Which section]
```

### 4. Testing Section

```markdown
## 🧪 Testing Results

### Tests Added
- `tests/integration/feature_test.rs` - [Coverage]

### Test Execution
cargo test --workspace
✅ 234 tests passed (0 failed)
```

### 5. Validation Section

```markdown
## ✅ Validation Results

### Build Status
cargo build --workspace
✅ Compilation successful

### Linting Status
cargo clippy --workspace --all-targets --all-features -- -D warnings
✅ All linting passed

cargo fmt --all -- --check
✅ Code formatting verified
```

### 6. Completion Marker

```markdown
## ✅ Workstream Execution Complete

**Deliverables**: [N] code files, [M] doc files, [K] tests
**All Validations**: ✅ Build | ✅ Tests | ✅ Linting
**Status**: Ready for PR creation and human review
```

---

## Code Structure Discovery

**Before writing code, discover structure:**

1. **Find similar code**: Search for existing implementations
2. **Verify data structures**: Read struct/type definitions (never assume field names)
3. **Note organization**: How are files named? What's the pattern?
4. **Check imports**: What modules are available?

**Only after structure discovery, proceed to implementation.**

---

## Quality Checks

**Run these after every major change:**

```bash
# Build
cargo build --workspace

# Test
cargo test --workspace

# Lint
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Format
cargo fmt --all

# Doc lint (if modifying markdown)
npm run docs:lint

# Coverage (optional, for significant changes)
./scripts/coverage.sh
```

---

## Common Pitfalls

### 1. Starting Without Enough Context

❌ **Bad**: "I'll just start implementing"  
✅ **Good**: Load relevant skill, read related code first

### 2. Unclear Acceptance Criteria

❌ **Bad**: "Improve error messages"  
✅ **Good**: "All errors must include line number, column, and ±2 lines of context"

### 3. Breaking Existing Functionality

❌ **Bad**: Change code without running tests  
✅ **Good**: Run tests after every change

### 4. Forgetting Documentation

❌ **Bad**: Only update code  
✅ **Good**: Update docs in same commit

### 5. Wrong Data Structure Assumptions

❌ **Bad**: Write tests based on documentation  
✅ **Good**: Read actual struct definition first

### 6. Not Recording Learnings

❌ **Bad**: Complete work, forget what was learned  
✅ **Good**: Document discoveries in LEARNINGS.md or PR description

---

## Proven Patterns

### Checkpoint Methodology

Break large features into 8 structured checkpoints:

1. Lexer changes
2. Parser changes
3. AST changes
4. Type checker changes
5. Runtime changes
6. Godot binding changes
7. Tests
8. Documentation

**Each checkpoint**: implement → test → commit

### MVP + Robustness Split

1. **MVP phase**: Core functionality, happy path
2. **Robustness phase**: Edge cases, error handling, tests

### Error Code Semantic Grouping

- `E001-E099`: Lexer errors
- `E100-E199`: Parser errors
- `E200-E299`: Type checker errors
- `E300-E399`: Signal errors
- `E500-E599`: Runtime errors
- `E600-E699`: Node query errors
- `E700-E799`: Godot type errors
- `E800-E899`: Export/Inspector errors

### Integration Examples as Tests

Create example `.ferris` files demonstrating practical patterns. These serve as both documentation and integration tests.

---

## Success Metrics

### Quantitative

- ✅ All acceptance criteria met (100%)
- ✅ All tests pass (0 failures)
- ✅ All linting passes (0 warnings)
- ✅ Test coverage target met

### Qualitative

- ✅ Code is clear and idiomatic
- ✅ Documentation is comprehensive
- ✅ Error messages are helpful
- ✅ Changes follow project patterns
- ✅ Work is ready for PR/review

---

## Getting Started

When starting a workstream:

1. **Acknowledge context**: List attached files and requirements
2. **Load relevant skill**: Read appropriate `docs/agent-skills/*.md`
3. **Ask clarifying questions**: Only for high-risk ambiguities
4. **Confirm understanding**: Brief summary before starting
5. **Execute**: Follow the workflow above
6. **Validate**: Run all quality checks
7. **Document**: Update docs and record learnings
8. **Complete**: Output structured summary with ✅ marker

---

## Related Resources

- **AGENTS.md**: Project-wide agent instructions
- **CLAUDE.md**: Claude Code-specific notes
- **docs/agent-skills/**: Detailed skill files for each crate
- **CONTRIBUTING.md**: Human contribution guidelines
- **docs/DEVELOPMENT.md**: Development workflow

---

**Last Updated**: July 2026  
**Version**: 2.0 (agent-agnostic rewrite)
