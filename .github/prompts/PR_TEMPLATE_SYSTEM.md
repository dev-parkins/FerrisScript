# PR Template System - Reference Card

Quick reference for FerrisScript's automated PR template system.

---

## 📋 Branch Naming Convention

| Branch Prefix | PR Template | Use For | Commit Type |
|---------------|-------------|---------|-------------|
| `bugfix/*` or `fix/*` | [bug_fix.md](../PULL_REQUEST_TEMPLATE/bug_fix.md) | Bug fixes, issue resolutions | `fix(scope):` |
| `feature/*` or `feat/*` | [feature.md](../PULL_REQUEST_TEMPLATE/feature.md) | New features, enhancements | `feat(scope):` |
| `docs/*` or `doc/*` | [docs.md](../PULL_REQUEST_TEMPLATE/docs.md) | Documentation updates | `docs:` |
| *(anything else)* | [docs.md](../PULL_REQUEST_TEMPLATE/docs.md) | Default fallback | *(varies)* |

**Why**: Branch name prefix automatically applies appropriate PR template via GitHub Actions (`.github/workflows/pr-template.yml`)

---

## 🎯 Quick Commands

### Bug Fix Workflow

```bash
git checkout -b bugfix/issue-description
# Make changes
git commit -m "fix(parser): handle null pointer in expression parsing"
git push origin bugfix/issue-description
# Create PR → Bug Fix template applied automatically
```

### Feature Workflow

```bash
git checkout -b feature/feature-name
# Make changes
git commit -m "feat(runtime): add async script loading support"
git push origin feature/feature-name
# Create PR → Feature template applied automatically
```

### Documentation Workflow

```bash
git checkout -b docs/doc-update
# Make changes
git commit -m "docs: add API usage examples to README"
git push origin docs/doc-update
# Create PR → Documentation template applied automatically
```

---

## 📝 Commit Message Format

**Convention**: Conventional Commits

```
type(scope): description

Examples:
  feat(parser): add error recovery support
  fix(runtime): handle null pointer in expression evaluation
  docs: update LEARNINGS.md with Phase 3C insights
  refactor(lexer): simplify token matching logic
  test: add edge case tests for empty files
  chore: update dependencies
```

**Types**: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `perf`, `ci`

---

## 🤖 How Automation Works

1. You create a PR
2. GitHub Actions detects branch name prefix
3. Selects appropriate template file
4. Applies template to PR body (if empty/minimal)
5. Adds comment explaining which template was applied

**Automation Behavior**:

- Only runs on PR opened (not edits)
- Skips if PR body >50 characters
- Adds helpful comment

---

## 🔧 Manual Template Selection

If you need to manually choose a template:

1. Go to GitHub PR creation page
2. Click "Choose a template" dropdown
3. Select: `bug_fix.md`, `feature.md`, or `docs.md`

**Useful when**:

- Forgot branch naming convention
- Want different template
- Creating PR from GitHub UI

---

## � Related Documentation

- **Full details**: [CONTRIBUTING.md](../../CONTRIBUTING.md#branch-naming-convention)
- **Development workflow**: [docs/DEVELOPMENT.md](../../docs/DEVELOPMENT.md)
- **PR templates**: [.github/PULL_REQUEST_TEMPLATE/](../PULL_REQUEST_TEMPLATE/)
- **Automation**: [.github/workflows/pr-template.yml](../workflows/pr-template.yml)

---

**Last Updated**: October 7, 2025
