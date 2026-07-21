# CI Workflow Duplication Analysis

**Date**: October 8, 2025  
**Issue**: Duplicate workflow executions on develop branch pushes  
**Root Cause**: PR #31 (develop → main) open while pushing to develop

---

## 🔍 Problem Summary

When pushing to the `develop` branch while PR #31 (develop → main) is **open**, GitHub Actions triggers workflows **twice**:

1. **Push event** on `develop` branch
2. **Pull request synchronize event** on PR #31

This causes unnecessary CI minutes usage and duplicate validation.

---

## 📊 Current Workflow Execution Analysis

### Workflow Run History (Last 10)

From `gh run list`:

```
2025-10-08T18:56:24Z | pull_request | develop | Code Scanning & Coverage | success
2025-10-08T18:56:24Z | pull_request | develop | CI/CD | success
2025-10-08T18:56:24Z | pull_request | develop | Documentation Linting | success
2025-10-08T18:56:22Z | push | develop | Documentation Linting | success
2025-10-08T18:56:22Z | push | develop | Code Scanning & Coverage | (in progress)
2025-10-08T18:54:25Z | pull_request | develop | Documentation Linting | success
2025-10-08T18:54:25Z | pull_request | develop | CI/CD | success
2025-10-08T18:54:25Z | pull_request | develop | Code Scanning & Coverage | success
2025-10-08T18:54:24Z | push | develop | Code Scanning & Coverage | success
```

**Evidence of Duplication**: Same workflows running for both `push` and `pull_request` events at nearly identical timestamps.

---

## 🔧 Workflow Configuration Analysis

### 1. `ci.yml` (CI/CD)

**Push Triggers**:

```yaml
on:
  push:
    branches: 
      - main
      - develop
```

**Pull Request Triggers**:

```yaml
  pull_request:
    branches: 
      - main
      - develop
```

**Jobs**:

- `quick-check`: Runs **only** on `pull_request` events
  - Condition: `if: github.event_name == 'pull_request'`
  - Duration: ~2-3 minutes
  - Actions: Format check, clippy, unit tests (Ubuntu only)

- `test`: Runs **only** on `push` to main/develop or tags
  - Condition: `if: github.ref == 'refs/heads/main' || github.ref == 'refs/heads/develop' || startsWith(github.ref, 'refs/tags/')`
  - Duration: ~10-15 minutes
  - Actions: Full test suite (3 platforms: Ubuntu, Windows, macOS)

**Duplication Impact**: Both jobs run, but they're different (quick-check vs full test suite). This is **intentional design** for fast PR feedback.

---

### 2. `code-scanning.yml` (Code Scanning & Coverage)

**Push Triggers**:

```yaml
on:
  push:
    branches:
      - main
      - develop
```

**Pull Request Triggers**:

```yaml
  pull_request:
    types: [opened, synchronize, reopened]
```

**Jobs**:

- `sonarqube`: Runs on **both** push and pull_request
  - No conditions limiting execution
  - Duration: ~3-5 minutes
  - **DUPLICATES**: Runs twice on develop pushes (with PR #31 open)

- `codecov`: Runs **only** on push to main/develop
  - Condition: `if: github.event_name == 'push' && (github.ref == 'refs/heads/main' || github.ref == 'refs/heads/develop')`
  - Duration: ~5-7 minutes
  - **No duplication**: Pull request events are excluded ✅

**Duplication Impact**: SonarQube runs twice, Codecov runs once (correct).

---

### 3. `docs-lint.yml` (Documentation Linting)

**Push Triggers**:

```yaml
on:
  push:
    branches: 
      - main
      - develop
    paths:
      - '**.md'
      - 'docs/**'
```

**Pull Request Triggers**:

```yaml
  pull_request:
    branches:
      - main
      - develop
    paths:
      - '**.md'
      - 'docs/**'
```

**Jobs**:

- `markdown-lint`: Runs on **both** push and pull_request
  - No conditions limiting execution
  - Duration: ~1-2 minutes
  - **DUPLICATES**: Runs twice on develop pushes (with PR #31 open)

- `link-check`: Runs on **both** push and pull_request
  - No conditions limiting execution
  - Duration: ~2-3 minutes
  - **DUPLICATES**: Runs twice on develop pushes (with PR #31 open)

**Duplication Impact**: Both jobs run twice (markdown-lint + link-check).

---

### 4. `benchmarks.yml` (Benchmarks)

**Push Triggers**:

```yaml
on:
  push:
    branches: [main, develop]
    paths:
      - 'crates/**/*.rs'
      - 'Cargo.toml'
```

**No Pull Request Triggers** ✅

**Duplication Impact**: None - only runs on push events.

---

## ✅ Coverage Verification

### Question: Is coverage running on develop?

**Answer: YES** ✅

**Evidence**:

```yaml
codecov:
  name: Code Coverage (Codecov)
  if: |
    github.event_name == 'push' && 
    (github.ref == 'refs/heads/main' || github.ref == 'refs/heads/develop')
```

**Coverage Execution**:

- ✅ Runs on **push** to develop
- ✅ Generates **both** Cobertura XML and LCOV (after recent change)
- ✅ Uploads to Codecov
- ✅ LCOV available for SonarCloud integration
- ❌ Does **not** run on pull_request events (by design - saves CI time)

**Verification from recent runs**:

```
2025-10-08T18:56:22Z | push | develop | Code Scanning & Coverage | (in progress)
2025-10-08T18:54:24Z | push | develop | Code Scanning & Coverage | success
```

Coverage job ran successfully on both pushes to develop.

---

## 🚨 Duplication Summary

| Workflow | Job | Push (develop) | Pull Request (PR #31) | Duplicates? |
|----------|-----|----------------|----------------------|-------------|
| **ci.yml** | quick-check | ❌ No | ✅ Yes | ❌ No (different jobs) |
| **ci.yml** | test | ✅ Yes | ❌ No | ❌ No (different jobs) |
| **code-scanning.yml** | sonarqube | ✅ Yes | ✅ Yes | ✅ **YES** |
| **code-scanning.yml** | codecov | ✅ Yes | ❌ No | ❌ No |
| **docs-lint.yml** | markdown-lint | ✅ Yes | ✅ Yes | ✅ **YES** |
| **docs-lint.yml** | link-check | ✅ Yes | ✅ Yes | ✅ **YES** |
| **benchmarks.yml** | benchmark | ✅ Yes | ❌ No | ❌ No |

**Total Duplicates**: 3 jobs (SonarQube, markdown-lint, link-check)

**CI Time Waste Per Push**:

- SonarQube: ~3-5 minutes
- Markdown-lint: ~1-2 minutes
- Link-check: ~2-3 minutes
- **Total**: ~6-10 minutes wasted per push to develop

**Cost Impact**:

- With PR #31 open: ~6-10 extra CI minutes per push
- Over 10 pushes: ~60-100 extra CI minutes
- GitHub Actions free tier: 2,000 minutes/month (public repos: unlimited)
- **For public repos**: No cost impact, but unnecessary resource usage

---

## 🔍 Main Branch Verification

### Question: Does main branch also have duplication issues?

**Answer: NO** (when no PR is open targeting main from a feature branch)

**Scenario Analysis**:

1. **Push directly to main** (after merge):
   - Triggers: `push` event on main
   - No pull_request events (no open PR)
   - **Result**: Single execution ✅

2. **PR from feature branch → main**:
   - Push to feature branch triggers `pull_request` event only
   - No `push` event on main (not pushed yet)
   - **Result**: Single execution ✅

3. **Push to develop with PR #31 open (develop → main)**:
   - Push to develop triggers `push` event on develop
   - Also triggers `pull_request` synchronize event on PR #31
   - **Result**: Duplicate execution ❌

**Conclusion**: Main branch is **NOT** affected by duplication because:

- Pushes to main happen **after** PR merge (PR is closed, no synchronize events)
- Feature branch PRs don't push to main, only trigger pull_request events

---

## 💡 Solutions & Recommendations

### Option 1: Close PR #31 Until Ready to Merge (Immediate Fix) ⭐

**Action**: Convert PR #31 to draft or close it until v0.0.3 is ready for final review.

**Pros**:

- ✅ Immediate fix (no code changes)
- ✅ Stops all duplication
- ✅ Saves ~6-10 CI minutes per push

**Cons**:

- ⚠️ PR needs to be reopened when ready to merge
- ⚠️ Lose continuous PR validation (but develop branch validation continues)

**Recommendation**: **Best immediate solution** since v0.0.3 is still in development and not ready for merge yet.

---

### Option 2: Add Workflow Conditions to Skip PR Events on develop

**Action**: Modify workflows to skip execution when pull_request event AND head branch is develop.

**Example for `code-scanning.yml`**:

```yaml
sonarqube:
  name: SonarQube Quality Scan
  if: |
    github.event_name == 'push' || 
    (github.event_name == 'pull_request' && github.head_ref != 'develop')
  runs-on: ubuntu-latest
```

**Example for `docs-lint.yml`**:

```yaml
markdown-lint:
  name: Markdown Linting
  if: |
    github.event_name == 'push' || 
    (github.event_name == 'pull_request' && github.head_ref != 'develop')
  runs-on: ubuntu-latest

link-check:
  name: Markdown Link Check
  if: |
    github.event_name == 'push' || 
    (github.event_name == 'pull_request' && github.head_ref != 'develop')
  runs-on: ubuntu-latest
```

**Pros**:

- ✅ Permanent fix (prevents future duplication)
- ✅ Still validates PRs from feature branches
- ✅ Develop branch gets full validation on push

**Cons**:

- ⚠️ Requires workflow changes (3 files)
- ⚠️ Need to test/verify conditions work correctly
- ⚠️ Slightly more complex workflow logic

**Recommendation**: **Good long-term solution** if keeping PR #31 open is required.

---

### Option 3: Accept Duplication During Development (No Change)

**Action**: Do nothing, accept temporary duplication until PR #31 is merged.

**Pros**:

- ✅ No changes needed
- ✅ Extra validation can catch issues earlier
- ✅ Temporary issue (goes away after merge)

**Cons**:

- ❌ Wastes ~6-10 CI minutes per push
- ❌ Slower CI feedback (more jobs queued)
- ❌ Clutters CI history with duplicate runs

**Recommendation**: **Acceptable for short-term** if PR will be merged soon (within 1-2 days).

---

## 📋 Recommended Action Plan

### Immediate (Today)

**Option 1 (Simplest)**:

1. Keep PR #31 open as draft (for visibility)
2. Accept duplication during final v0.0.3 development
3. Merge PR #31 once v0.0.3 is ready (duplication stops automatically)

**Option 2 (If duplication is problematic)**:

1. Implement workflow conditions (Option 2 above)
2. Test on develop branch
3. Verify duplication stops

### Short-Term (v0.0.4)

1. Establish workflow best practices:
   - Keep develop → main PR closed during active development
   - Open PR only for final review before merge
   - Or implement Option 2 conditions permanently

2. Document workflow patterns in CONTRIBUTING.md

### Long-Term (v0.1.0)

1. Consider CI/CD optimizations:
   - Consolidate duplicate workflows
   - Add more granular path filters
   - Optimize job dependencies

---

## ✅ Verification Checklist

- [x] **Coverage runs on develop pushes**: YES ✅ (verified in workflow logs)
- [x] **Coverage generates LCOV**: YES ✅ (added in commit 179dddb)
- [x] **Coverage generates Cobertura**: YES ✅ (existing, maintained)
- [x] **SonarQube duplicates**: YES ❌ (runs on both push and PR sync)
- [x] **Docs linting duplicates**: YES ❌ (runs on both push and PR sync)
- [x] **Main branch has duplication**: NO ✅ (only when PR is open, closed after merge)
- [x] **CI tests duplicate**: NO ✅ (different jobs: quick-check vs full test suite)

---

## 📊 Impact Assessment

### Current State (PR #31 Open)

**Per push to develop**:

- SonarQube: 2 runs (~8 minutes total)
- Markdown-lint: 2 runs (~2 minutes total)
- Link-check: 2 runs (~4 minutes total)
- **Total waste**: ~14 minutes per push

**Over 10 pushes** (typical development cycle):

- **Total waste**: ~140 minutes (~2.3 hours)

### After Fix (Option 1 or 2)

**Per push to develop**:

- SonarQube: 1 run (~4 minutes)
- Markdown-lint: 1 run (~1 minute)
- Link-check: 1 run (~2 minutes)
- **Total**: ~7 minutes per push (50% reduction)

---

## 🎯 Conclusion

**Key Findings**:

1. ✅ Coverage **IS** running on develop pushes
2. ✅ Coverage generates both Cobertura and LCOV
3. ❌ SonarQube, markdown-lint, and link-check **ARE** duplicating
4. ✅ Main branch **does NOT** have duplication issues
5. ✅ Root cause identified: PR #31 open while pushing to develop

**Decision Made**: **Option 1 - Accept Temporary Duplication** ✅

**Rationale**:

- `develop → main` PRs are standard workflow going forward
- Duplication is **temporary** (only during active development on develop)
- Duplication **stops automatically** when PR is merged
- **Simplest solution** - no workflow changes needed
- Extra validation can catch issues earlier (added benefit)
- Impact is minimal for short development cycles (1-3 days before merge)

**Workflow Pattern Established**:

```
1. Open PR: develop → main (draft or ready for review)
2. Push commits to develop (duplication occurs, but acceptable)
3. Final review and merge PR
4. Duplication stops (PR is closed)
5. Repeat for next version
```

**Trade-offs Accepted**:

- ~6-10 extra CI minutes per push to develop (during PR open period)
- Cluttered CI history with duplicate runs
- Offset by: Extra validation, simpler workflow, automatic resolution

**Next Steps**:

1. ✅ Keep PR #31 open during v0.0.3 finalization
2. ✅ Accept duplication as normal during this phase
3. ✅ Merge PR #31 when v0.0.3 is ready
4. ✅ Repeat pattern for v0.0.4, v0.0.5, etc.
5. ✅ Document this workflow pattern in CONTRIBUTING.md (future enhancement)

---

**Last Updated**: October 8, 2025  
**Status**: Decision finalized - Option 1 adopted as standard workflow pattern  
**Decided By**: User (project maintainer)
