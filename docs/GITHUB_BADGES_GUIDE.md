# GitHub Badges Guide

**Created:** October 2, 2025  
**Purpose:** Instructions for adding badges to README.md  
**Reference:** Phase 3 completion - GitHub insights documentation

---

## Shields.io Badge Syntax

All badges use [shields.io](https://shields.io/) for consistent styling.

### GitHub Stars Badge

**Badge Code:**
```markdown
![GitHub Stars](https://img.shields.io/github/stars/dev-parkins/FerrisScript?style=flat-square&logo=github)
```

**Result:**
- Shows current star count
- Auto-updates when stars change
- Flat square style matches modern GitHub design

### Full Recommended Badge Set

```markdown
<!-- Project Status -->
![Version](https://img.shields.io/badge/version-0.0.1-blue?style=flat-square)
![Status](https://img.shields.io/badge/status-alpha-orange?style=flat-square)

<!-- Build & Tests -->
![Build Status](https://img.shields.io/github/actions/workflow/status/dev-parkins/FerrisScript/ci.yml?branch=main&style=flat-square&logo=github-actions)
![Tests](https://img.shields.io/badge/tests-96%20passing-green?style=flat-square)

<!-- Community -->
![GitHub Stars](https://img.shields.io/github/stars/dev-parkins/FerrisScript?style=flat-square&logo=github)
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)
![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen?style=flat-square)

<!-- Requirements -->
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange?style=flat-square&logo=rust)
![Godot](https://img.shields.io/badge/godot-4.2%2B-blue?style=flat-square&logo=godot-engine)
```

---

## Badge Placement in README

**Recommended Location:** Directly after the title, before description.

```markdown
# FerrisScript 🦀

![Version](https://img.shields.io/badge/version-0.0.1-blue?style=flat-square)
![Status](https://img.shields.io/badge/status-alpha-orange?style=flat-square)
![GitHub Stars](https://img.shields.io/github/stars/dev-parkins/FerrisScript?style=flat-square&logo=github)
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange?style=flat-square&logo=rust)
![Godot](https://img.shields.io/badge/godot-4.2%2B-blue?style=flat-square&logo=godot-engine)

A Rust-inspired scripting language for Godot 4 via GDExtension...
```

---

## Dynamic Badges (Auto-Updating)

### Build Status (requires GitHub Actions)

**Prerequisites:**
- `.github/workflows/ci.yml` file exists
- Workflow has `name: CI` or similar

**Badge Code:**
```markdown
![Build Status](https://img.shields.io/github/actions/workflow/status/dev-parkins/FerrisScript/ci.yml?branch=main&style=flat-square&logo=github-actions)
```

### Stars (already dynamic)

```markdown
![GitHub Stars](https://img.shields.io/github/stars/dev-parkins/FerrisScript?style=flat-square&logo=github)
```

### Forks

```markdown
![GitHub Forks](https://img.shields.io/github/forks/dev-parkins/FerrisScript?style=flat-square&logo=github)
```

### Issues

```markdown
![GitHub Issues](https://img.shields.io/github/issues/dev-parkins/FerrisScript?style=flat-square&logo=github)
```

### Last Commit

```markdown
![Last Commit](https://img.shields.io/github/last-commit/dev-parkins/FerrisScript?style=flat-square&logo=github)
```

---

## Static Badges (Manual Update)

### Version (update on each release)

```markdown
![Version](https://img.shields.io/badge/version-0.0.1-blue?style=flat-square)
```

**Update to 0.0.2:**
```markdown
![Version](https://img.shields.io/badge/version-0.0.2-blue?style=flat-square)
```

### Status (update based on project stage)

```markdown
<!-- Alpha -->
![Status](https://img.shields.io/badge/status-alpha-orange?style=flat-square)

<!-- Beta (v0.2.0+) -->
![Status](https://img.shields.io/badge/status-beta-yellow?style=flat-square)

<!-- Stable (v1.0.0+) -->
![Status](https://img.shields.io/badge/status-stable-green?style=flat-square)
```

### Tests (update when test count changes significantly)

```markdown
![Tests](https://img.shields.io/badge/tests-96%20passing-green?style=flat-square)
```

**Update to 120 tests:**
```markdown
![Tests](https://img.shields.io/badge/tests-120%20passing-green?style=flat-square)
```

---

## Badge Colors

Follow shields.io color conventions:

| Status | Color | Hex | Use Case |
|--------|-------|-----|----------|
| Success | `green` | `#4c1` | Passing tests, stable |
| Info | `blue` | `#007ec6` | Version, license |
| Warning | `yellow` | `#dfb317` | Beta status |
| Important | `orange` | `#fe7d37` | Alpha status, Rust version |
| Critical | `red` | `#e05d44` | Failing tests, deprecated |

---

## Shields.io Badge Builder

**URL:** https://shields.io/

### Custom Badge Builder

1. Go to https://shields.io/
2. Click "Static Badge"
3. Fill in fields:
   - **Label:** Text on left (e.g., "version")
   - **Message:** Text on right (e.g., "0.0.1")
   - **Color:** Badge color (e.g., "blue")
4. Select style: `flat-square`
5. Add logo: `github`, `rust`, `godot-engine`
6. Copy generated markdown

### Example: Custom "PRs Welcome" Badge

```markdown
![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen?style=flat-square)
```

---

## Badge Maintenance Checklist

### On Each Release (v0.0.2, v0.1.0, etc.)

- [ ] Update version badge
- [ ] Update status badge (alpha → beta → stable)
- [ ] Update test count badge if significantly changed
- [ ] Verify build status badge is green
- [ ] Check dynamic badges still work (stars, forks)

### Monthly

- [ ] Verify all badge links work (no 404s)
- [ ] Check if shields.io syntax changed
- [ ] Update badge colors if conventions changed

---

## Advanced: Custom GitHub Actions Badge

**For v0.0.3+ when CI is implemented:**

### Step 1: Name your workflow

```yaml
# .github/workflows/ci.yml
name: CI  # This name appears in badge

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: cargo test
```

### Step 2: Add badge to README

```markdown
![CI](https://img.shields.io/github/actions/workflow/status/dev-parkins/FerrisScript/ci.yml?branch=main&style=flat-square&logo=github-actions&label=CI)
```

**Result:** Badge shows "passing" (green) or "failing" (red) based on latest CI run.

---

## Examples from Popular Projects

### Rust Project (serde)

```markdown
![Build Status](https://img.shields.io/github/actions/workflow/status/serde-rs/serde/ci.yml)
![crates.io](https://img.shields.io/crates/v/serde.svg)
![docs.rs](https://img.shields.io/docsrs/serde)
```

### Godot-Rust (gdext)

```markdown
![docs](https://img.shields.io/badge/docs-v0.1-blue)
![Build Status](https://img.shields.io/github/actions/workflow/status/godot-rust/gdext/full-ci.yml)
![License](https://img.shields.io/badge/license-MPL--2.0-blue)
```

### FerrisScript (Recommended)

```markdown
![Version](https://img.shields.io/badge/version-0.0.1-blue?style=flat-square)
![Status](https://img.shields.io/badge/status-alpha-orange?style=flat-square)
![GitHub Stars](https://img.shields.io/github/stars/dev-parkins/FerrisScript?style=flat-square&logo=github)
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange?style=flat-square&logo=rust)
![Godot](https://img.shields.io/badge/godot-4.2%2B-blue?style=flat-square&logo=godot-engine)
![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen?style=flat-square)
```

---

## Implementation Steps

### Immediate (Next 5 minutes)

1. Copy recommended badge set above
2. Open `README.md`
3. Paste badges directly after `# FerrisScript 🦀` title
4. Commit: `docs: add GitHub badges for version, status, stars, and requirements`
5. Push to main

### After CI Implementation (v0.0.3)

1. Add build status badge with real workflow status
2. Remove static "tests passing" badge, use dynamic CI badge
3. Update GITHUB_BADGES_GUIDE.md with actual CI workflow name

---

## Troubleshooting

### Badge not showing

- Check URL syntax (no typos in repo name)
- Verify repo is public (badges don't work on private repos without tokens)
- Check shields.io service status: https://status.shields.io/

### Stars badge shows 0

- Wait 5-10 minutes for shields.io cache to refresh
- Hard refresh browser: Ctrl+F5 (Windows) or Cmd+Shift+R (Mac)
- Check repo has public stars (not hidden)

### Build status badge shows "unknown"

- Verify `.github/workflows/ci.yml` file exists
- Verify workflow has run at least once
- Check workflow file name matches badge URL
- Verify branch name (main vs master)

---

## Resources

- **Shields.io:** https://shields.io/
- **GitHub Badges:** https://docs.github.com/en/actions/monitoring-and-troubleshooting-workflows/adding-a-workflow-status-badge
- **Badge Examples:** https://github.com/badges/awesome-badges
- **Color Reference:** https://shields.io/badges/static-badge (scroll to "Colors")

---

**Ready to implement:** Copy badge set to README.md and commit! 🚀
