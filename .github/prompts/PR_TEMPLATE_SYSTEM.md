# PR Template System - Quick Reference

This document explains how FerrisScript's automated PR template system works.

---

## 🎯 Overview

FerrisScript uses **branch name prefixes** to automatically apply the appropriate PR template when you create a pull request. This ensures consistency and helps both humans and automation (like GitHub Copilot) create well-structured PRs.

---

## 📋 Branch Naming Convention

| Branch Prefix | PR Template | Use For |
|---------------|-------------|---------|
| `bugfix/*` or `fix/*` | [bug_fix.md](../.github/PULL_REQUEST_TEMPLATE/bug_fix.md) | Bug fixes, issue resolutions |
| `feature/*` or `feat/*` | [feature.md](../.github/PULL_REQUEST_TEMPLATE/feature.md) | New features, enhancements |
| `docs/*` or `doc/*` | [docs.md](../.github/PULL_REQUEST_TEMPLATE/docs.md) | Documentation updates |
| *(anything else)* | [docs.md](../.github/PULL_REQUEST_TEMPLATE/docs.md) | Default fallback |

---

## 🤖 How It Works

### Workflow: `.github/workflows/pr-template.yml`

```
1. You create a PR
   ↓
2. Workflow detects branch name
   ↓
3. Selects appropriate template file
   ↓
4. Applies template to PR body (if body is empty/minimal)
   ↓
5. Adds a comment explaining which template was applied
```

### Automation Behavior

**Only runs on:** PR opened (not on edits or synchronize)
**Skips if:** PR body already has >50 characters (respects manual work)
**Adds comment:** Lets you know which template was applied

---

## 📝 Template Features

### Bug Fix Template (`bug_fix.md`)

**Focus:**

- Root cause analysis
- Before/after comparison
- Regression testing
- Risk assessment

**Key Sections:**

- Bug description (what/why/how)
- Test coverage requirements
- Affected areas checklist
- Risk level assessment

### Feature Template (`feature.md`)

**Focus:**

- Feature motivation
- Usage examples
- Breaking changes
- Performance impact

**Key Sections:**

- Motivation and alternatives
- Example usage code
- Performance benchmarks
- Godot integration testing

### Documentation Template (`docs.md`)

**Focus:**

- Markdown linting
- Link validation
- Code example testing
- Version-specific docs

**Key Sections:**

- Type of doc change (checkboxes)
- Linting/link check results
- Code example verification
- Target audience

---

## 💡 Examples

### Creating a Bug Fix PR

```bash
# 1. Create branch with bugfix/ prefix
git checkout -b bugfix/parser-null-pointer

# 2. Make your changes and commit
git add .
git commit -m "fix(parser): handle null pointer in expression parsing"

# 3. Push to GitHub
git push origin bugfix/parser-null-pointer

# 4. Create PR on GitHub
# → Bug Fix template automatically applied ✅
```

### Creating a Feature PR

```bash
# 1. Create branch with feature/ prefix
git checkout -b feature/async-script-loading

# 2. Make your changes and commit
git add .
git commit -m "feat(runtime): add async script loading support"

# 3. Push to GitHub
git push origin feature/async-script-loading

# 4. Create PR on GitHub
# → Feature template automatically applied ✅
```

### Creating a Documentation PR

```bash
# 1. Create branch with docs/ prefix
git checkout -b docs/add-api-examples

# 2. Make your changes and commit
git add .
git commit -m "docs: add API usage examples to README"

# 3. Push to GitHub
git push origin docs/add-api-examples

# 4. Create PR on GitHub
# → Documentation template automatically applied ✅
```

---

## 🤖 GitHub Copilot Integration

### Why This Matters for Copilot

When GitHub Copilot (or the coding agent) creates PRs automatically:

1. **Branch naming is detected** → Correct template applied
2. **PR body is populated** → Copilot can fill in template sections
3. **Reviewers get context** → Consistent PR structure

### Copilot Best Practices

If you're using GitHub Copilot to create PRs:

```bash
# Copilot should create branches with proper prefixes:
- bugfix/issue-123-null-pointer
- feature/add-async-loading
- docs/update-contributing-guide

# Then when PR is created:
# → Automation applies appropriate template
# → Copilot fills in the template sections
# → Reviewers get well-structured PR
```

---

## 🔧 Manual Template Selection

You can also **manually choose** a template:

1. Go to GitHub PR creation page
2. Look for "Choose a template" dropdown (appears automatically)
3. Select: `bug_fix.md`, `feature.md`, or `docs.md`

This is useful if:

- You forgot to use the branch naming convention
- You want a different template than the auto-selected one
- You're creating a PR from the GitHub UI

---

## 🚨 Troubleshooting

### Template Not Applied?

**Check:**

1. Did you use the correct branch prefix? (`bugfix/`, `feature/`, `docs/`)
2. Was the PR body already filled out? (automation skips if >50 chars)
3. Did you create the PR via GitHub? (templates only work there)

### Wrong Template Applied?

**Solutions:**

1. Close PR and recreate with correct branch name
2. Manually select the correct template from dropdown
3. Copy/paste the correct template from `.github/PULL_REQUEST_TEMPLATE/`

### Automation Didn't Run?

**Check:**

1. Look for workflow run in "Actions" tab
2. Check if workflow file exists: `.github/workflows/pr-template.yml`
3. Verify you have Actions enabled on your fork

---

## 📚 Related Documentation

- [CONTRIBUTING.md](../CONTRIBUTING.md#branch-naming-convention) - Full contribution guide
- [DEVELOPMENT.md](../docs/DEVELOPMENT.md) - Development workflow
- [PR Templates](.github/PULL_REQUEST_TEMPLATE/) - Actual template files
- [Workflow File](.github/workflows/pr-template.yml) - Automation implementation

---

## 🎯 Quick Commands Reference

```bash
# Bug fix
git checkout -b bugfix/issue-description
git commit -m "fix(scope): description"
git push origin bugfix/issue-description

# Feature
git checkout -b feature/feature-name
git commit -m "feat(scope): description"
git push origin feature/feature-name

# Documentation
git checkout -b docs/doc-update
git commit -m "docs: description"
git push origin docs/doc-update
```

---

**Questions?** See [CONTRIBUTING.md](../CONTRIBUTING.md) or open an issue!
