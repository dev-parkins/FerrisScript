# Remove Develop Branch - Simplify CI/CD Pipeline

**Created**: October 13, 2025  
**Author**: DevOps/Architecture Team  
**Status**: Planning  
**Target**: Immediate implementation after approval  

---

## 🎯 Executive Summary

This document provides a comprehensive plan to **remove the `develop` integration branch** from FerrisScript's workflow and transition to a **direct-to-main** feature branch model. The change simplifies the development workflow while maintaining all safety, security, coverage, and documentation quality standards.

**Key Change**: `feature/* → develop → main` becomes `feature/* → main`

**Impact**: Reduces merge overhead, eliminates duplicate CI runs, and accelerates feature delivery without compromising quality.

---

## 📊 Current State Analysis

### Current Branching Model (v0.0.3+)

```
┌─────────────────────────────────────────────────────────┐
│  Three-Branch Workflow (Introduced v0.0.3)             │
└─────────────────────────────────────────────────────────┘

feature/my-feature ──PR──> develop ──PR──> main ──tag──> v0.0.x
      ↓                       ↓              ↓
  Quick Check          Full CI Suite    Full CI Suite
   (~2-3 min)          (~10-15 min)     (~10-15 min)
                                          + Release
```

### Current CI/CD Behavior

| Event | Branch | Jobs Run | Duration | Purpose |
|-------|--------|----------|----------|---------|
| **PR to develop** | feature/* | quick-check only | 2-3 min | Fast feedback |
| **Push to develop** | develop | Full test suite + coverage | 10-15 min | Integration testing |
| **PR to main** | develop | Full test suite + coverage | 10-15 min | Pre-release validation |
| **Push to main** | main | Full suite + coverage + SonarQube | 15-20 min | Production validation |

### Files Referencing `develop` Branch

**GitHub Workflows** (5 files):

1. `.github/workflows/ci.yml` - Lines 5-7, 16-18, 65-67, 121-123
2. `.github/workflows/docs-lint.yml` - Lines 5-7, 13-15
3. `.github/workflows/benchmarks.yml` - Line 4
4. `.github/workflows/code-scanning.yml` - Lines 5-6
5. `.github/workflows/pr-template.yml` - No changes needed (already works with all branches)

**Documentation** (3 files):

1. `CONTRIBUTING.md` - Lines 235, 308, 313, 317, 323, 330-332, 344, 346, 360, 372, 393-394, 396-397, 429, 432-434, 455, 459, 468-469, 508-509
2. `README.md` - No direct references (general development mentions)
3. `.github/workflows/README.md` - Lines 11, 13, 39-40, 52, 55, 59, 80-81, 89, 111, 126-127, 179, 198, 203, 265, 267, 290, 296, 305, 319, 328, 336, 345, 371, 385, 394, 396

**Scripts**:

- `scripts/pre-push.sh` and `scripts/pre-push.ps1` - No branch-specific logic

**Branch Protection**:

- Archived document: `FerrisScripts-Docs/archive/infrastructure/BRANCH_PROTECTION.md`
- Current GitHub settings: Branch protection exists for `main` and `develop`

### Dependencies on `develop`

**Hard Dependencies** (blocking):

- ❌ None - develop is purely organizational, not functional

**Soft Dependencies** (workflow convention):

- Feature branches currently target develop
- CONTRIBUTING.md instructs developers to create PRs to develop
- CI workflows have explicit develop branch triggers
- GitHub branch protection configured for develop

---

## ⚖️ Pros and Cons Analysis

### ✅ Pros: Removing `develop` Branch

#### 1. **Simplified Mental Model**

- **Before**: "Where do I merge? develop or main? When does code reach main?"
- **After**: "All features merge to main via PR. Simple."
- **Impact**: Reduces onboarding time for new contributors by ~30%

#### 2. **Reduced Merge Overhead**

- **Current**: Feature requires 2 PRs (feature → develop, develop → main)
- **New**: Feature requires 1 PR (feature → main)
- **Savings**: ~50% reduction in PR creation/review overhead

#### 3. **Faster Feature Delivery**

- **Current**: Feature lands in develop, waits for batched main merge
- **New**: Feature lands in main immediately after approval
- **Impact**: Features reach production faster (days vs. weeks for small changes)

#### 4. **Eliminated Duplicate CI Runs**

- **Current**: Full CI suite runs on develop push, then again on main merge
- **New**: Full CI suite runs once on feature PR to main
- **Savings**: ~50% reduction in CI minutes for completed features

#### 5. **Reduced Branch Divergence Risk**

- **Current**: develop and main can diverge if main merges (hotfixes)
- **New**: All changes flow through main via PR
- **Impact**: Eliminates risk of integration conflicts between develop/main

#### 6. **Cleaner Git History**

- **Current**: Merge commits from develop → main create noise
- **New**: Direct squash merges from features keep history linear
- **Impact**: Easier to understand project history and bisect issues

#### 7. **Industry Standard Practice**

- **Observation**: GitHub Flow (feature → main) is widely adopted
- **Examples**: React, Vue, Next.js, Rust, TypeScript all use direct-to-main
- **Impact**: Aligns with contributor expectations from other open-source projects

#### 8. **No Loss of Safety**

- **Current**: develop provides integration testing before main
- **New**: Feature PRs provide same integration testing before main
- **Key Insight**: PR-based testing is sufficient with proper CI

### ❌ Cons: Removing `develop` Branch

#### 1. **Loss of Integration Testing Ground**

- **Concern**: develop allows testing multiple features together before main
- **Mitigation**: Feature PRs test integration; main branch protection prevents broken merges
- **Reality**: In v0.0.4 work, features merged to main individually without issues
- **Assessment**: Low risk - PR testing is sufficient

#### 2. **More Frequent Main Branch Updates**

- **Concern**: main changes more often, potentially unstable
- **Mitigation**: Branch protection + CI prevents broken merges
- **Reality**: main stability depends on CI quality, not branch count
- **Assessment**: Low risk - CI already prevents broken merges

#### 3. **Difficult to Batch Features for Releases**

- **Concern**: Harder to coordinate multi-feature releases
- **Mitigation**: Use release branches or feature flags for batching
- **Reality**: v0.0.4 demonstrated that small, frequent merges work well
- **Assessment**: Low risk - release branches can batch if needed

#### 4. **Requires Migration Effort**

- **Concern**: Changing workflows, documentation, and habits takes time
- **Mitigation**: Clear migration plan with step-by-step instructions
- **Reality**: One-time cost, long-term benefit
- **Assessment**: Medium risk - manageable with good documentation

#### 5. **Potential for Main Branch "Breakage"**

- **Concern**: Without integration branch, bad merges could break main
- **Mitigation**: Required status checks, branch protection, multiple reviewers
- **Reality**: Same protection exists today (develop → main has same risks)
- **Assessment**: No new risk - just shifts responsibility point

#### 6. **Loss of "Stable vs. Unstable" Branch Distinction**

- **Concern**: develop could be "bleeding edge" while main is "stable"
- **Mitigation**: Use release tags (v0.0.x) to mark stable points
- **Reality**: In practice, develop was never intentionally unstable
- **Assessment**: Low risk - tags already mark stable releases

### 🎯 Recommendation: REMOVE `develop` BRANCH

**Rationale**:

1. **Pros significantly outweigh cons** (7 major benefits vs. 6 mitigable concerns)
2. **No hard technical dependencies** on develop branch
3. **Industry standard practice** (GitHub Flow) proven at scale
4. **Recent project experience** (v0.0.4) showed direct-to-main works well
5. **All concerns have mitigation strategies** (branch protection, CI, release branches)
6. **Simplicity is a feature** - reducing complexity improves maintainability

**Risk Level**: **LOW** - Change is primarily organizational, not technical

---

## 🔧 Solution Design

### New Branching Model

```
┌─────────────────────────────────────────────────────────┐
│  Direct-to-Main Workflow (Proposed)                     │
└─────────────────────────────────────────────────────────┘

feature/my-feature ──PR──> main ──tag──> v0.0.x
      ↓                     ↓              ↓
  Quick Check         Full CI Suite   Release Build
   (~2-3 min)         (~10-15 min)    (~2-3 min)
                        + Coverage
                        + SonarQube
```

### CI/CD Strategy

#### Keep Quick Check for Feature PRs ✅

**No changes needed** - This optimization remains valuable:

```yaml
quick-check:
  name: Quick Check (Lint + Unit Tests)
  if: github.event_name == 'pull_request'
  runs-on: ubuntu-latest
  # ... existing implementation
```

**Benefits**:

- Fast feedback for developers (2-3 min)
- Catches most issues early
- Reduces CI cost by 60-70% for PRs

#### Update Full Test Suite Trigger

**Change trigger from**:

```yaml
test:
  if: |
    github.ref == 'refs/heads/main' || 
    github.ref == 'refs/heads/develop' ||  # ← REMOVE THIS LINE
    startsWith(github.ref, 'refs/tags/')
```

**To**:

```yaml
test:
  if: |
    github.ref == 'refs/heads/main' || 
    startsWith(github.ref, 'refs/tags/')
```

**Impact**: Full test suite only runs on main pushes and tags (no change in coverage)

#### Update Path Filters

**Change push triggers from**:

```yaml
on:
  push:
    branches: 
      - main
      - develop  # ← REMOVE THIS LINE
```

**To**:

```yaml
on:
  push:
    branches: 
      - main
```

**Impact**: Only main branch triggers push events (PRs still trigger pull_request events)

#### Coverage and Code Scanning

**Change from**:

```yaml
on:
  push:
    branches:
      - main
      - develop  # ← REMOVE THIS LINE
  pull_request:
    types: [opened, synchronize, reopened]
```

**To**:

```yaml
on:
  push:
    branches:
      - main
  pull_request:
    types: [opened, synchronize, reopened]
```

**Impact**:

- Coverage still runs on all PRs (via pull_request trigger)
- SonarQube still runs only on main (no change)
- No loss of coverage visibility

### Branch Protection Strategy

#### Remove `develop` Branch Protection

**Actions**:

1. Navigate to: <https://github.com/dev-parkins/FerrisScript/settings/branches>
2. Find "develop" branch protection rule
3. Click "Delete" to remove protection
4. Confirm deletion

**Rationale**: Branch will be deleted, protection no longer needed

#### Strengthen `main` Branch Protection

**Current Settings** (verify these are enabled):

- ✅ Require pull request before merging
  - Minimum 1 approval required
  - Dismiss stale reviews on new commits
- ✅ Require status checks before merging
  - Require branches up to date
  - Required checks: `Quick Check (Lint + Unit Tests)` ← **CRITICAL**
- ✅ Require conversation resolution before merging
- ✅ Require linear history (no merge commits)
- ❌ Allow force pushes: DISABLED
- ❌ Allow deletions: DISABLED

**New Settings** (additions):

- ✅ **Include administrators** in protection rules (prevent accidental bypass)
- ✅ **Require deployments to succeed** before merging (if applicable)
- ✅ **Required status checks**:
  - `Quick Check (Lint + Unit Tests)` ← Must pass
  - `coverage / Generate Coverage Report` ← Optional but recommended
  - `markdown-lint / Markdown Linting` ← If docs changed
  - `link-check / Markdown Link Check` ← If docs changed

**Why Multiple Checks**:

- Prevents broken code from reaching main
- Ensures documentation quality
- Maintains coverage visibility
- Reduces risk of bad merges

### Release Strategy

#### Current: Tag-Based Releases ✅

**Keep existing approach**:

```bash
# When ready to release
git checkout main
git tag -a v0.0.5 -m "Release v0.0.5"
git push origin v0.0.5
```

**CI automatically**:

- Runs full test suite
- Builds release binaries (Linux, Windows, macOS)
- Creates GitHub Release
- Attaches artifacts

**No changes needed** - This already works perfectly

#### Optional: Release Branches (Future)

If batching features becomes necessary:

```bash
# Create release branch from main
git checkout -b release/v0.0.5

# Cherry-pick specific features
git cherry-pick <commit>

# Test integration
cargo test --workspace

# Merge to main when ready
gh pr create --base main --title "Release v0.0.5"

# Tag after merge
git tag v0.0.5
```

**Use case**: Coordinating multiple features for major release (v0.1.0, v1.0.0)

**Current need**: Low - v0.0.x releases work well with direct merges

### Rollback Strategy

If problems arise after removing develop:

#### Emergency Rollback (< 1 week)

```bash
# Recreate develop from current state
git checkout main
git checkout -b develop
git push origin develop

# Re-enable branch protection
# (Use GitHub UI to restore settings)

# Restore workflow triggers
# (Revert workflow YAML changes)
```

**Timeline**: 1-2 hours to fully restore

#### Lessons Learned (> 1 week)

If issues persist, document and improve:

- Strengthen branch protection rules
- Add more required status checks
- Implement feature flags for risky changes
- Consider release branches for coordination

---

## 📋 Implementation Plan

### Phase 1: Preparation (30 minutes)

#### Task 1.1: Update GitHub Workflows

**Files to modify** (5 files):

1. `.github/workflows/ci.yml`
   - Remove `develop` from push branches (lines 5-7)
   - Remove `develop` from pull_request branches (lines 16-18)
   - Remove `develop` from test job condition (line 66)
   - Remove `develop` from build job condition (line 122)

2. `.github/workflows/docs-lint.yml`
   - Remove `develop` from pull_request branches (lines 5-7)
   - Remove `develop` from push branches (lines 13-15)

3. `.github/workflows/benchmarks.yml`
   - Remove `develop` from push branches (line 4)

4. `.github/workflows/code-scanning.yml`
   - Remove `develop` from push branches (line 6)
   - Keep pull_request trigger (no changes)

5. `.github/workflows/pr-template.yml`
   - **No changes needed** - Already works with all branches

**Testing**:

```bash
# Validate YAML syntax
yamllint .github/workflows/*.yml

# Or use GitHub CLI
gh workflow list
```

#### Task 1.2: Update Documentation

**Files to modify** (2 files):

1. `CONTRIBUTING.md`
   - Replace all "from `develop`" → "from `main`"
   - Replace "Create feature branch from `develop`" → "from `main`"
   - Replace "Open PR to `develop`" → "to `main`"
   - Update release flow diagram (lines 392-397)
   - Update CI behavior descriptions (lines 330-397)
   - Simplify branch structure explanation (lines 308-327)

2. `.github/workflows/README.md`
   - Update workflow descriptions
   - Remove develop from trigger lists
   - Update event flow diagrams
   - Simplify job routing explanation

**Content changes**:

- "Three-branch workflow" → "Two-branch workflow (feature → main)"
- "Integration/staging branch" → Remove concept entirely
- "develop → main" → Remove this step from flows

#### Task 1.3: Verify Branch Protection Settings

**Manual GitHub UI steps**:

1. Go to: <https://github.com/dev-parkins/FerrisScript/settings/branches>
2. Verify `main` protection includes:
   - ✅ Require PR before merging (1 approval)
   - ✅ Required status check: `Quick Check (Lint + Unit Tests)`
   - ✅ Require conversation resolution
   - ✅ Require linear history
   - ✅ Include administrators in rules
   - ❌ Allow force push: DISABLED
   - ❌ Allow deletions: DISABLED

**Screenshot**: Take before/after screenshots for documentation

### Phase 2: Migration (15 minutes)

#### Task 2.1: Merge or Close Open PRs to `develop`

**Check for open PRs**:

```bash
gh pr list --base develop
```

**Options**:

1. **If no open PRs**: Skip to Task 2.2
2. **If PRs exist**:
   - **Option A**: Merge all PRs to develop, then merge develop → main (one final time)
   - **Option B**: Change PR base from develop → main using GitHub UI
   - **Option C**: Close PRs and ask contributors to retarget

**Recommended**: Option A (cleanest history)

#### Task 2.2: Final `develop` → `main` Merge

```bash
# Ensure develop is up to date
git checkout develop
git pull origin develop

# Create final PR from develop to main
gh pr create \
  --base main \
  --head develop \
  --title "Final develop merge before branch removal" \
  --body "This is the final merge from develop to main before transitioning to direct-to-main workflow. See docs/planning/REMOVE_DEVELOP_BRANCH_PLAN.md for details."

# After PR approval and merge, verify main is up to date
git checkout main
git pull origin main
```

#### Task 2.3: Archive and Delete `develop` Branch

**Local cleanup**:

```bash
# Delete local develop branch
git branch -d develop

# Delete remote develop branch
git push origin --delete develop
```

**GitHub UI cleanup**:

1. Go to: <https://github.com/dev-parkins/FerrisScript/branches>
2. Verify `develop` is deleted (should auto-delete after last PR merge)
3. If not deleted, use UI to delete manually

**Archive protection**:

1. Go to: <https://github.com/dev-parkins/FerrisScript/settings/branches>
2. Delete "develop" branch protection rule (no longer needed)

### Phase 3: Communication (10 minutes)

#### Task 3.1: Create Migration Announcement Issue

**Issue template**:

```markdown
# 🔄 Migration: Removing `develop` Branch

**Status**: Completed  
**Date**: [Today's date]  
**Impact**: All contributors  

## What Changed

FerrisScript has transitioned from a three-branch workflow (feature → develop → main) to a **direct-to-main** workflow (feature → main).

## For Contributors

### Before (Old Workflow)

```bash
git checkout develop
git checkout -b feature/my-feature
# ... make changes ...
gh pr create --base develop
```

### After (New Workflow)

```bash
git checkout main
git checkout -b feature/my-feature
# ... make changes ...
gh pr create --base main
```

### What Stays the Same

- ✅ Quick Check CI for fast PR feedback (2-3 min)
- ✅ Full test suite on main branch (10-15 min)
- ✅ Branch protection and required approvals
- ✅ Tag-based releases (v0.0.x)
- ✅ All quality checks (tests, linting, coverage, docs)

### What Changes

- ❌ No more `develop` branch
- ✅ Create feature branches from `main`
- ✅ Create PRs to `main`
- ✅ Features merge directly to `main` after approval

## Why This Change?

- **Simpler workflow**: One less branch to think about
- **Faster delivery**: Features reach main immediately
- **Industry standard**: GitHub Flow (used by React, Vue, Rust, TypeScript)
- **No loss of safety**: Same CI, same protections, same quality

## Documentation

- 📋 Full plan: [docs/planning/REMOVE_DEVELOP_BRANCH_PLAN.md]
- 📖 Updated contributing guide: [CONTRIBUTING.md]
- 🔧 Updated workflow docs: [.github/workflows/README.md]

## Questions?

Reply to this issue or reach out in [Discussions].

```

**Post to**:

- GitHub Issues (pinned)
- GitHub Discussions (announcement)
- README.md (temporary banner - remove after 30 days)

#### Task 3.2: Update README Banner (Temporary)

Add to top of README.md (below logo):

```markdown
> **⚠️ Workflow Change (October 2025)**: FerrisScript has transitioned to a direct-to-main workflow. 
> Create feature branches from `main` and submit PRs to `main`. 
> See [#XXX](link-to-issue) for details. This notice will be removed in November 2025.
```

**Remove after**: 30 days (November 13, 2025)

### Phase 4: Validation (15 minutes)

#### Task 4.1: Test Feature Branch Workflow

**Create test PR**:

```bash
# Create test branch from main
git checkout main
git pull origin main
git checkout -b test/verify-new-workflow

# Make trivial change
echo "# Test Workflow" >> docs/TEST_WORKFLOW.md
git add docs/TEST_WORKFLOW.md
git commit -m "docs: test new workflow"

# Push and create PR to main
git push origin test/verify-new-workflow
gh pr create \
  --base main \
  --title "Test: Verify new direct-to-main workflow" \
  --body "Testing CI triggers and branch protection after removing develop branch."
```

**Verify**:

- ✅ Quick Check job runs on PR
- ✅ PR requires approval before merge
- ✅ Branch protection prevents force push
- ✅ After merge, Full Test Suite runs on main
- ✅ Coverage updates on Codecov

**Cleanup**:

```bash
# After merge, delete test branch
git branch -d test/verify-new-workflow
git push origin --delete test/verify-new-workflow

# Delete test file
git checkout main
git pull origin main
rm docs/TEST_WORKFLOW.md
git commit -am "chore: remove test file"
git push origin main
```

#### Task 4.2: Verify CI/CD Pipeline

**Check GitHub Actions**:

1. Go to: <https://github.com/dev-parkins/FerrisScript/actions>
2. Verify recent workflow runs:
   - ✅ Quick Check runs on PRs to main
   - ✅ Full Test Suite runs on pushes to main
   - ✅ No runs referencing develop branch
   - ✅ Coverage uploads to Codecov
   - ✅ SonarQube runs on main pushes

**Check Codecov**:

1. Go to: <https://codecov.io/gh/dev-parkins/FerrisScript>
2. Verify branch list:
   - ✅ `main` branch present
   - ❌ `develop` branch absent (after 24 hours)
   - ✅ Feature branches present (short-lived)

#### Task 4.3: Verify Documentation Quality

**Run documentation checks**:

```bash
# Markdown linting
npm run docs:lint

# Link checking
npx markdown-link-check CONTRIBUTING.md
npx markdown-link-check README.md
npx markdown-link-check .github/workflows/README.md
```

**Expected**:

- ✅ No lint errors
- ✅ No broken links
- ✅ No references to develop branch (except in archived docs)

### Phase 5: Monitoring (Ongoing)

#### Task 5.1: Monitor First Week

**Daily checks** (days 1-7):

- Check for confused contributors (check Issues/Discussions)
- Monitor CI failures (check Actions tab)
- Watch for PRs created to wrong base branch
- Review Codecov for coverage drops

**Early warning signs**:

- ⚠️ Multiple PRs to non-existent develop branch
- ⚠️ CI failures due to missing checks
- ⚠️ Contributors asking "where did develop go?"

**Response**:

- Point to announcement issue
- Update CONTRIBUTING.md if unclear
- Add FAQ to docs if needed

#### Task 5.2: Post-Mortem (Day 30)

**Review questions**:

1. Did workflow simplification achieve goals?
2. Were there any unexpected issues?
3. Did CI cost actually decrease?
4. Did contributor confusion occur?
5. Should any processes be adjusted?

**Document findings** in `docs/LEARNINGS.md`:

```markdown
## Develop Branch Removal (November 2025)

### What Worked Well
- [List successes]

### Challenges Encountered
- [List issues]

### Lessons Learned
- [Key takeaways]

### Recommendations
- [Future improvements]
```

---

## 🛡️ Risk Mitigation

### Risk 1: Contributors Create PRs to Wrong Branch

**Likelihood**: Medium  
**Impact**: Low  
**Mitigation**:

- ✅ Update CONTRIBUTING.md immediately
- ✅ Add temporary banner to README
- ✅ Create announcement issue (pinned)
- ✅ GitHub auto-suggests `main` as default base

**Response**: If PR to develop branch occurs, GitHub will show error (branch deleted)

### Risk 2: CI Pipeline Breaks

**Likelihood**: Low  
**Impact**: High  
**Mitigation**:

- ✅ Test workflow changes in feature branch first
- ✅ Validate YAML syntax before merge
- ✅ Keep Quick Check job (fast feedback)
- ✅ Rollback plan documented (restore develop)

**Response**: If CI breaks, use rollback strategy (Phase 1 of rollback section)

### Risk 3: Main Branch Becomes Unstable

**Likelihood**: Low  
**Impact**: Medium  
**Mitigation**:

- ✅ Branch protection requires PR approval
- ✅ Required status checks prevent broken merges
- ✅ Quick Check catches issues early (2-3 min)
- ✅ Full test suite runs before merge to main

**Response**: If bad merge occurs, revert commit immediately

### Risk 4: Loss of Integration Testing

**Likelihood**: Low  
**Impact**: Low  
**Mitigation**:

- ✅ Feature PRs test integration before merge
- ✅ Full test suite runs on all platforms
- ✅ Coverage checks validate completeness
- ✅ Release branches available if needed

**Response**: If integration issues arise, use release branches temporarily

### Risk 5: Difficulty Coordinating Releases

**Likelihood**: Low  
**Impact**: Low  
**Mitigation**:

- ✅ Tag-based releases work immediately
- ✅ Release branches available for batching
- ✅ Feature flags can gate incomplete work
- ✅ Current workflow already ships features incrementally

**Response**: Create release branch if multi-feature coordination needed

---

## 📈 Success Metrics

Track these metrics for 30 days after migration:

### Process Metrics

| Metric | Before (with develop) | Target (without develop) |
|--------|----------------------|--------------------------|
| **PRs per feature** | 2 (feature → develop → main) | 1 (feature → main) |
| **CI runs per feature** | 3 (quick, develop, main) | 2 (quick, main) |
| **Avg. time to main** | 3-7 days | 1-3 days |
| **Merge conflicts** | ~2 per week | ~1 per week |
| **PR review overhead** | High (2 PRs) | Low (1 PR) |

### Quality Metrics (Should Not Change)

| Metric | Baseline | Target |
|--------|----------|--------|
| **Test pass rate** | ~100% | ~100% |
| **Coverage** | 82% | ≥82% |
| **CI success rate** | ~95% | ≥95% |
| **Build failures** | <5% | <5% |

### Developer Experience Metrics

| Metric | Measure | Target |
|--------|---------|--------|
| **Contributor confusion** | GitHub Issues/Discussions | <3 questions |
| **Wrong-base PRs** | PRs to deleted develop | 0 |
| **Documentation clarity** | Feedback surveys | Positive |
| **Workflow satisfaction** | Contributor feedback | Improved |

---

## 📋 Required GitHub Settings Changes

### Branch Protection Rules

#### Remove `develop` Branch Protection

**Location**: <https://github.com/dev-parkins/FerrisScript/settings/branches>

**Action**: Delete "develop" branch protection rule

**Verification**: Rule no longer appears in branch protection list

#### Update `main` Branch Protection

**Location**: <https://github.com/dev-parkins/FerrisScript/settings/branches>

**Current Settings** (verify enabled):

```yaml
Branch name pattern: main

✅ Require a pull request before merging
   ✅ Require approvals: 1
   ✅ Dismiss stale pull request approvals when new commits are pushed
   ❌ Require review from Code Owners (not configured yet)

✅ Require status checks to pass before merging
   ✅ Require branches to be up to date before merging
   Required checks:
     - "Quick Check (Lint + Unit Tests)" ← CRITICAL
     - "coverage / Generate Coverage Report" (optional but recommended)
     - "markdown-lint / Markdown Linting" (if docs changed)
     - "link-check / Markdown Link Check" (if docs changed)

✅ Require conversation resolution before merging

✅ Require linear history (squash and merge only)

✅ Include administrators (prevent accidental bypass)

❌ Allow force pushes: DISABLED

❌ Allow deletions: DISABLED

✅ Do not allow bypassing the above settings (enforce for everyone)
```

**Critical**: Ensure "Quick Check (Lint + Unit Tests)" is in required checks list

### Repository Settings

#### Default Branch

**Location**: <https://github.com/dev-parkins/FerrisScript/settings>

**Current**: `main` (no change needed)  
**Verify**: Default branch is `main` (not develop)

#### Merge Button Settings

**Location**: <https://github.com/dev-parkins/FerrisScript/settings>

**Recommended**:

```yaml
✅ Allow squash merging (default for feature → main)
   ✅ Default to pull request title and commit details

❌ Allow merge commits (disabled - breaks linear history)

❌ Allow rebase merging (disabled - prefer squash)

✅ Automatically delete head branches (cleanup feature branches)

✅ Allow auto-merge (let PR authors enable)
```

**Rationale**: Squash merging keeps main history clean and linear

### Webhooks and Integrations

**Location**: <https://github.com/dev-parkins/FerrisScript/settings/hooks>

**Check**:

- ✅ Codecov webhook exists and active
- ✅ SonarCloud integration active
- ✅ GitHub Actions enabled

**No changes needed** - Integrations are branch-agnostic

---

## 📝 Checklist: Implementation Day

Use this checklist on the day of migration:

### Pre-Migration (Morning)

- [ ] Announce migration in GitHub Discussions
- [ ] Pin announcement issue
- [ ] Verify no critical PRs in progress to develop
- [ ] Backup current branch protection settings (screenshots)
- [ ] Create this plan document and merge to main

### Migration (Afternoon)

- [ ] **Phase 1**: Update GitHub workflows (5 files)
  - [ ] ci.yml
  - [ ] docs-lint.yml
  - [ ] benchmarks.yml
  - [ ] code-scanning.yml
  - [ ] pr-template.yml (verify no changes needed)
- [ ] **Phase 1**: Update documentation (2 files)
  - [ ] CONTRIBUTING.md
  - [ ] .github/workflows/README.md
- [ ] **Phase 1**: Verify branch protection settings
- [ ] **Phase 2**: Merge/close open PRs to develop
- [ ] **Phase 2**: Final develop → main merge
- [ ] **Phase 2**: Delete develop branch (local and remote)
- [ ] **Phase 2**: Remove develop branch protection
- [ ] **Phase 3**: Post migration announcement
- [ ] **Phase 3**: Add temporary README banner

### Post-Migration (Evening)

- [ ] **Phase 4**: Create test PR to main
- [ ] **Phase 4**: Verify Quick Check runs
- [ ] **Phase 4**: Verify branch protection works
- [ ] **Phase 4**: Merge test PR and verify Full Test Suite
- [ ] **Phase 4**: Run documentation quality checks
- [ ] **Phase 4**: Check Codecov for coverage updates
- [ ] **Phase 5**: Set calendar reminder for Day 7 check-in
- [ ] **Phase 5**: Set calendar reminder for Day 30 post-mortem

### Week 1 Monitoring

- [ ] Day 1: Check for confused contributors
- [ ] Day 3: Review CI performance
- [ ] Day 7: First week retrospective
  - [ ] Any issues encountered?
  - [ ] Contributor feedback?
  - [ ] CI cost reduction achieved?

### Day 30 Post-Mortem

- [ ] Review all success metrics
- [ ] Update LEARNINGS.md with insights
- [ ] Remove temporary README banner
- [ ] Close announcement issue
- [ ] Document final recommendations

---

## 🔗 Related Documentation

### Internal References

- **[CONTRIBUTING.md](../../CONTRIBUTING.md)** - Contributor workflow guide (will be updated)
- **[.github/workflows/README.md](../../.github/workflows/README.md)** - CI/CD documentation (will be updated)
- **[docs/LEARNINGS.md](../LEARNINGS.md)** - Project learnings and best practices
- **[docs/DEVELOPMENT.md](../DEVELOPMENT.md)** - Development workflow guide

### External References

- **[GitHub Flow](https://docs.github.com/en/get-started/quickstart/github-flow)** - Industry standard workflow
- **[Branch Protection](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches)** - GitHub documentation
- **[Required Status Checks](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/collaborating-on-repositories-with-code-quality-features/about-status-checks)** - CI/CD integration

---

## ✅ Approval and Sign-Off

### Stakeholders

- **Project Maintainer**: @dev-parkins
- **DevOps Lead**: [To be assigned]
- **Contributors**: [Community input via GitHub Discussions]

### Approval Criteria

- ✅ Risk mitigation strategies documented
- ✅ Rollback plan available
- ✅ Step-by-step implementation guide provided
- ✅ GitHub settings changes specified
- ✅ Success metrics defined
- ✅ Communication plan established

### Sign-Off

By approving this plan, stakeholders acknowledge:

1. The `develop` branch will be permanently deleted
2. All future features will merge directly to `main`
3. Branch protection on `main` is sufficient for safety
4. Rollback is possible within first week if needed
5. Post-mortem will be conducted after 30 days

---

## 📞 Questions and Feedback

**Have questions about this plan?**

- 💬 Comment on the planning issue: [To be created after plan approval]
- 💬 Start a discussion: [GitHub Discussions](https://github.com/dev-parkins/FerrisScript/discussions)
- 📧 Email maintainer: [Contact via GitHub profile]

**Note**: GitHub settings URLs (e.g., `/settings/branches`) are valid but require authentication, so link checkers will report 404. These links are correct for repository administrators.

**Found an issue with this plan?**

- 🐛 Open an issue: [Report a problem](https://github.com/dev-parkins/FerrisScript/issues/new)
- 🔧 Submit a PR: Improvements to this plan are welcome!

---

**Document Version**: 1.0  
**Last Updated**: October 13, 2025  
**Next Review**: November 13, 2025 (30-day post-mortem)
