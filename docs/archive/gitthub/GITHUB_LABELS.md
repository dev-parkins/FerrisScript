# GitHub Labels Configuration for FerrisScript

**Created**: October 4, 2025  
**For**: v0.0.2 Release - Phase 5A  
**Total Labels**: 20

---

## Label System Overview

This document defines the 20-label system for organizing issues and pull requests in the FerrisScript repository.

---

## 1. Priority Labels (4)

These indicate the urgency and importance of issues/PRs.

| Label | Color | Description |
|-------|-------|-------------|
| `P0-Critical` | `#d73a4a` (red) | Critical bugs or blockers requiring immediate attention |
| `P1-High` | `#ff6600` (orange) | High priority tasks that should be addressed soon |
| `P2-Medium` | `#fbca04` (yellow) | Medium priority tasks for regular workflow |
| `P3-Low` | `#0e8a16` (green) | Low priority tasks or nice-to-have improvements |

**Usage**: Every issue should have exactly one priority label.

---

## 2. Type Labels (6)

These categorize the nature of the issue/PR.

| Label | Color | Description |
|-------|-------|-------------|
| `bug` | `#d73a4a` (red) | Something isn't working correctly |
| `feature` | `#a2eeef` (light blue) | New feature or functionality request |
| `documentation` | `#0075ca` (blue) | Documentation improvements or additions |
| `enhancement` | `#84b6eb` (sky blue) | Improvement to existing functionality |
| `question` | `#d876e3` (purple) | Questions or clarifications needed |
| `discussion` | `#cc317c` (pink) | General discussion topics |

**Usage**: Every issue should have at least one type label.

---

## 3. Status Labels (4)

These track the current state of an issue/PR.

| Label | Color | Description |
|-------|-------|-------------|
| `needs-triage` | `#e4e669` (light yellow) | New issue awaiting initial review and prioritization |
| `in-progress` | `#fbca04` (yellow) | Work is actively being done on this issue |
| `blocked` | `#b60205` (dark red) | Blocked by external dependencies or decisions |
| `wontfix` | `#ffffff` (white/gray) | Issue will not be addressed (with explanation) |

**Usage**: Optional, used to track workflow state.

---

## 4. Difficulty Labels (3)

These help contributors find appropriate tasks for their skill level.

| Label | Color | Description |
|-------|-------|-------------|
| `good-first-issue` | `#7057ff` (purple) | Good for newcomers to the project |
| `intermediate` | `#008672` (teal) | Requires moderate knowledge of codebase |
| `advanced` | `#5319e7` (dark purple) | Requires deep understanding of architecture |

**Usage**: Optional, primarily for community contributions.

---

## 5. Component Labels (5)

These identify which part of the codebase is affected.

| Label | Color | Description |
|-------|-------|-------------|
| `compiler` | `#1d76db` (blue) | Related to compiler crate (lexer, parser, type checker) |
| `runtime` | `#0e8a16` (green) | Related to runtime crate (execution environment) |
| `godot-bind` | `#fbca04` (yellow) | Related to Godot GDExtension bindings |
| `docs` | `#0075ca` (blue) | Related to documentation (not code) |
| `ci` | `#ededed` (gray) | Related to CI/CD, GitHub Actions, workflows |

**Usage**: Issues/PRs may have multiple component labels.

---

## Label Usage Guidelines

### For Issue Triage

1. **New Issue Arrives**:
   - Automatically gets `needs-triage` label
   - Maintainer reviews and adds:
     - One priority label (P0-P3)
     - One or more type labels (bug, feature, etc.)
     - Relevant component labels
   - Remove `needs-triage` once categorized

2. **Work Begins**:
   - Add `in-progress` label
   - Assign to person working on it
   - Link to PR when created

3. **Work Blocked**:
   - Add `blocked` label
   - Add comment explaining what's blocking
   - Update when unblocked

4. **Work Completed**:
   - PR merged → Issue auto-closes
   - Labels preserved for search/analytics

### For Pull Requests

- Add same labels as related issue(s)
- Add component labels for files changed
- Priority reflects urgency of merge

### For Community Contributions

- Mark beginner-friendly issues with `good-first-issue`
- Add detailed descriptions and acceptance criteria
- Link to CONTRIBUTING.md for guidance

---

## Implementation Instructions

### Option 1: Manual Creation (GitHub Web UI)

1. Navigate to: `https://github.com/dev-parkins/FerrisScript/labels`
2. Click "New label" for each label
3. Enter name, description, and color code
4. Click "Create label"
5. Repeat for all 20 labels

### Option 2: GitHub CLI (Faster)

```bash
# Priority Labels
gh label create "P0-Critical" --description "Critical bugs or blockers requiring immediate attention" --color "d73a4a"
gh label create "P1-High" --description "High priority tasks that should be addressed soon" --color "ff6600"
gh label create "P2-Medium" --description "Medium priority tasks for regular workflow" --color "fbca04"
gh label create "P3-Low" --description "Low priority tasks or nice-to-have improvements" --color "0e8a16"

# Type Labels
gh label create "bug" --description "Something isn't working correctly" --color "d73a4a"
gh label create "feature" --description "New feature or functionality request" --color "a2eeef"
gh label create "documentation" --description "Documentation improvements or additions" --color "0075ca"
gh label create "enhancement" --description "Improvement to existing functionality" --color "84b6eb"
gh label create "question" --description "Questions or clarifications needed" --color "d876e3"
gh label create "discussion" --description "General discussion topics" --color "cc317c"

# Status Labels
gh label create "needs-triage" --description "New issue awaiting initial review and prioritization" --color "e4e669"
gh label create "in-progress" --description "Work is actively being done on this issue" --color "fbca04"
gh label create "blocked" --description "Blocked by external dependencies or decisions" --color "b60205"
gh label create "wontfix" --description "Issue will not be addressed (with explanation)" --color "ffffff"

# Difficulty Labels
gh label create "good-first-issue" --description "Good for newcomers to the project" --color "7057ff"
gh label create "intermediate" --description "Requires moderate knowledge of codebase" --color "008672"
gh label create "advanced" --description "Requires deep understanding of architecture" --color "5319e7"

# Component Labels
gh label create "compiler" --description "Related to compiler crate (lexer, parser, type checker)" --color "1d76db"
gh label create "runtime" --description "Related to runtime crate (execution environment)" --color "0e8a16"
gh label create "godot-bind" --description "Related to Godot GDExtension bindings" --color "fbca04"
gh label create "docs" --description "Related to documentation (not code)" --color "0075ca"
gh label create "ci" --description "Related to CI/CD, GitHub Actions, workflows" --color "ededed"
```

### Option 3: GitHub API (Automation)

See `scripts/create-labels.sh` (if created) for API-based creation.

---

## Label Examples in Practice

### Example 1: Bug Report

**Issue**: "Parser crashes on empty function body"

**Labels**:

- `P1-High` (serious bug, but not blocking)
- `bug` (it's broken)
- `compiler` (parser is in compiler crate)

### Example 2: Feature Request

**Issue**: "Add support for arrays"

**Labels**:

- `P2-Medium` (important but not urgent)
- `feature` (new functionality)
- `compiler` (requires parser, type checker changes)
- `runtime` (requires runtime support)

### Example 3: Documentation Improvement

**Issue**: "README needs comparison with GDScript"

**Labels**:

- `P2-Medium` (nice to have)
- `documentation` (docs change)
- `docs` (documentation component)
- `good-first-issue` (easy for newcomers)

### Example 4: Community Discussion

**Issue**: "Should we support operator overloading in v0.2.0?"

**Labels**:

- `P3-Low` (future planning)
- `discussion` (seeking feedback)
- `compiler` (would affect compiler)

---

## Analytics & Search

### Useful Queries

- **All critical bugs**: `is:open label:P0-Critical label:bug`
- **Good first issues**: `is:open label:good-first-issue`
- **Compiler work**: `is:open label:compiler`
- **In progress**: `is:open label:in-progress`
- **Blocked items**: `is:open label:blocked`

### Reporting

- Track by priority: Count issues per priority label
- Track by component: See which parts need attention
- Track completion: Closed issues by label over time

---

## Maintenance

### Quarterly Review

- Evaluate label usage effectiveness
- Add new labels if needed
- Deprecate unused labels
- Update descriptions for clarity

### As Project Grows

- May need sub-component labels (e.g., `compiler-lexer`, `compiler-parser`)
- May add release labels (e.g., `v0.0.2`, `v0.1.0`)
- May add platform labels (e.g., `windows`, `linux`, `macos`)

---

## References

- **CONTRIBUTING.md**: How to use labels when contributing
- **GITHUB_PROJECT_MANAGEMENT.md**: Overall project management strategy
- **GitHub Label Best Practices**: https://docs.github.com/en/issues/using-labels-and-milestones-to-track-work/managing-labels

---

**Status**: Ready for Implementation  
**Next Step**: Create labels via GitHub CLI or web interface
