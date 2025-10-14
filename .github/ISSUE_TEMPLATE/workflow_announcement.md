---
name: 📢 Workflow Change Announcement
about: FerrisScript has simplified its branching workflow
title: "[ANNOUNCEMENT] Simplified Workflow - Direct to Main Branch"
labels: documentation, announcement
assignees: ''
pinned: true
---

# 🎉 Workflow Simplification - We've Removed the `develop` Branch!

**Effective Date:** October 13, 2025  
**Impact:** All contributors

---

## 📋 What Changed?

FerrisScript has **simplified its development workflow** from a three-branch model to a direct-to-main model (GitHub Flow).

### Before (Three-Branch Workflow)
```
feature/my-feature → develop (PR #1)
develop → main (PR #2)
```

### After (Direct-to-Main Workflow)
```
feature/my-feature → main (PR)
```

---

## ✨ Why This Change?

After completing v0.0.4, we realized:

1. **✅ Faster feedback** - Reduced PR process from 2 steps to 1
2. **✅ Lower friction** - Contributors create one PR instead of two
3. **✅ Simpler mental model** - Aligned with industry standard (GitHub Flow)
4. **✅ Better resource usage** - Eliminated duplicate CI runs
5. **✅ Same quality** - Branch protection + required reviews maintain standards

See [docs/planning/REMOVE_DEVELOP_BRANCH_PLAN.md](https://github.com/dev-parkins/FerrisScript/blob/main/docs/planning/REMOVE_DEVELOP_BRANCH_PLAN.md) for the complete rationale.

---

## 🚀 What You Need to Do

### For New Contributors
**Nothing!** Just follow the updated [CONTRIBUTING.md](https://github.com/dev-parkins/FerrisScript/blob/main/CONTRIBUTING.md) guide.

### For Existing Contributors with Feature Branches

If you have an **existing feature branch** based on `develop`:

```bash
# Option 1: Rebase onto main (recommended)
git checkout your-feature-branch
git fetch origin
git rebase origin/main
git push --force-with-lease

# Option 2: Start fresh from main
git checkout main
git pull origin main
git checkout -b your-feature-branch-v2
# Cherry-pick or recreate your changes
```

### Updating Your Fork

```bash
# Delete local develop branch
git branch -d develop

# Delete remote develop branch from your fork
git push origin --delete develop

# Update main
git checkout main
git pull upstream main
git push origin main
```

---

## 📖 Updated Workflow

1. **Create feature branch from `main`**:
   ```bash
   git checkout main
   git pull origin main
   git checkout -b feature/your-feature
   ```

2. **Make changes and test locally**:
   ```bash
   cargo test --workspace
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   ```

3. **Create PR to `main`**:
   ```bash
   git push -u origin feature/your-feature
   gh pr create --base main --title "feat: your feature"
   ```

4. **After PR approval**: Squash and merge to `main`

---

## 🔧 CI Behavior (Unchanged)

The **Quick Check optimization** remains in place:

- **Feature branch PRs**: ⚡ Quick Check (2-3 min) - Fast feedback
- **Main branch**: 🔄 Full Test Suite (10-15 min) - Production validation

---

## 📚 Updated Documentation

All documentation has been updated:
- ✅ [CONTRIBUTING.md](https://github.com/dev-parkins/FerrisScript/blob/main/CONTRIBUTING.md) - Updated workflow instructions
- ✅ GitHub workflows - Removed `develop` triggers
- ✅ Branch protection - Only `main` is protected now

---

## 🤔 Questions?

- **"Will this affect code quality?"** - No! Branch protection + required reviews are unchanged.
- **"What about integration testing?"** - Still happens on `main` before release tags.
- **"Can I still use draft PRs?"** - Yes! Draft PRs are great for early feedback.
- **"What if I have merge conflicts?"** - Rebase on `main` instead of `develop`.

**Have other questions?** Comment below! 👇

---

## 📊 Success Metrics (First Week)

We're tracking:
- Average PR time (expect 20-30% reduction)
- CI resource usage (expect 15-20% reduction)
- Contributor feedback

---

**Thank you for adapting to this change!** This simplification helps us focus on building great features. 🦀❤️

---

**Related:**
- Full Plan: [docs/planning/REMOVE_DEVELOP_BRANCH_PLAN.md](https://github.com/dev-parkins/FerrisScript/blob/main/docs/planning/REMOVE_DEVELOP_BRANCH_PLAN.md)
- Contributing Guide: [CONTRIBUTING.md](https://github.com/dev-parkins/FerrisScript/blob/main/CONTRIBUTING.md)
