# GitHub Actions Workflows

This directory contains automated workflows for continuous integration, deployment, and maintenance of the FerrisScript project.

## 🆕 What's New in v0.0.3

The v0.0.3 infrastructure update introduces significant CI optimizations:

- ⚡ **Quick Check Job**: Fast 2-3 minute feedback for PRs (60-70% faster)
- 📝 **Path Filters**: Skip CI entirely for docs-only changes (95% savings)
- 🔄 **Three-Branch Workflow**: main → develop → feature/* with appropriate CI
- 🚀 **70% Overall CI Cost Reduction** across all PR types
- 🎯 **Smart Job Routing**: PRs get quick checks, main/develop get full validation

See [Performance Metrics](#performance-metrics) for detailed savings analysis.

## Table of Contents

- [What's New in v0.0.3](#-whats-new-in-v003)
- [Available Workflows](#available-workflows)
  - [CI/CD Pipeline](#cicd-pipeline)
  - [Documentation Linting](#documentation-linting)
  - [PR Template Automation](#pr-template-automation)
- [Workflow Visualization](#workflow-visualization)
- [Job Details](#job-details)
- [Event Flow Examples](#event-flow-examples)
- [Performance Metrics](#performance-metrics)

---

## Available Workflows

### CI/CD Pipeline

**File:** [`ci.yml`](ci.yml)  
**Name:** CI/CD  
**Triggers:**

- Pull requests to `main` or `develop`
- Pushes to `main` or `develop` branches
- Tags matching `v*` pattern
- **Path Filters (pushes only):** Ignores `docs/**`, `**.md`, `LICENSE`, `.gitignore`

**Jobs:**

1. **quick-check** - Pull Request only (NEW in v0.0.3)
   - Single-OS testing (Ubuntu only)
   - Fast feedback: format, clippy, unit tests only
   - Runs on all PRs (~2-3 minutes)
   - **Optimization:** 60-70% faster than full suite

2. **test** (renamed to "Full Test Suite") - Main/Develop/Tags only
   - Multi-OS testing (Ubuntu, Windows, macOS)
   - Complete test suite including all tests
   - Only runs on main/develop branches and tags (~10-15 minutes)
   - **Does NOT run on PRs** (quick-check runs instead)

3. **build** - Conditional execution
   - Only on push to `main`, `develop`, or tags `v*`
   - Builds release binaries for all platforms
   - Uploads artifacts (~15 minutes)

4. **release** - Tag-only execution
   - Only on tags matching `v*`
   - Creates GitHub Release
   - Attaches platform-specific binaries (~2 minutes)

5. **coverage** - Moved to code-scanning.yml (as of Oct 8, 2025)
   - See [Code Scanning & Coverage](#code-scanning--coverage) workflow
   - Consolidated with other security/quality scanning tools

---

### Code Scanning & Coverage

**File:** [`code-scanning.yml`](code-scanning.yml)  
**Name:** Code Scanning & Coverage  
**Triggers:**

- Pull requests to `main` or `develop`
- Pushes to `main` or `develop` branches

**Jobs:**

1. **sonarqube** - Quality analysis
   - Static code analysis (quality, security hotspots)
   - Code smells and technical debt tracking
   - **Note:** Coverage reporting disabled (handled by Codecov job)
   - Runs on all PRs and pushes (~5-8 minutes)

2. **codecov** - Code coverage (main/develop only)
   - Generates coverage with `cargo-tarpaulin`
   - Uploads to Codecov for tracking
   - Only runs on push to main/develop (~3-5 minutes)
   - **Consolidated:** Moved from ci.yml (Oct 8, 2025)

**Rationale for Consolidation:**

- Groups all security/quality scanning tools in one workflow
- Separates build/test (ci.yml) from analysis (code-scanning.yml)
- Easier to add future tools (e.g., CodeQL) without cluttering ci.yml
- SonarQube no longer reports coverage (avoids redundancy with Codecov)

See [`docs/infrastructure/README.md`](../../docs/infrastructure/README.md) for detailed infrastructure documentation.

---

### Documentation Linting

**File:** [`docs-lint.yml`](docs-lint.yml)  
**Name:** Documentation Linting  
**Triggers:**

- Pull requests to `main` or `develop` with markdown changes
- Pushes to `main` or `develop` with markdown changes

**Jobs:**

1. **markdown-lint** - Style and formatting
   - Uses markdownlint-cli
   - Config: `.markdownlint.json`

2. **link-check** - Validates URLs (UPDATED in v0.0.3)
   - Runs markdown-link-check directly (not via action)
   - **Excludes:** `docs/archive/**` (archived documentation)
   - Config: `.markdown-link-check.json`
   - Prevents false failures from outdated archive links

---

### PR Template Automation

**File:** [`pr-template.yml`](pr-template.yml)  
**Name:** Auto Apply PR Template  
**Triggers:**

- Pull request opened events only

**Jobs:**

1. **apply-template** - Branch-based template selection
   - Detects branch naming pattern
   - Applies appropriate PR template
   - Posts explanation comment

**Branch Patterns:**

| Branch Pattern | Template Applied |
|----------------|------------------|
| `bugfix/*` or `fix/*` | `bug_fix.md` |
| `feature/*` or `feat/*` | `feature.md` |
| `docs/*` or `doc/*` | `docs.md` |
| Other | `docs.md` (default) |

---

## Workflow Visualization

### CI/CD Pipeline Flow

```mermaid
flowchart TD
    Start([Event Triggered]) --> CheckEvent{Event Type?}
    
    CheckEvent -->|Pull Request to main| PR[Pull Request Flow]
    CheckEvent -->|Push to main| MainPush[Main Push Flow]
    CheckEvent -->|Push to develop| DevPush[Develop Push Flow]
    CheckEvent -->|Push tag v*| TagPush[Release Tag Flow]
    
    %% Pull Request Flow
    PR --> TestPR[✅ Test Job<br/>3 OS: Ubuntu, Windows, macOS]
    TestPR --> BuildCheck1{Build Condition:<br/>Push to main OR tag?}
    BuildCheck1 -->|No - PR| BuildSkipPR[❌ Build Job SKIPPED<br/>Not on PR]
    BuildSkipPR --> CovCheck1{Coverage Condition:<br/>Push to main?}
    CovCheck1 -->|No - PR| CovSkipPR[❌ Coverage Job SKIPPED<br/>Not on PR]
    CovSkipPR --> PREnd([PR Complete<br/>~15 minutes])
    
    %% Main Push Flow
    MainPush --> TestMain[✅ Test Job<br/>3 OS: Ubuntu, Windows, macOS]
    TestMain --> BuildCheck2{Build Condition:<br/>Push to main OR tag?}
    BuildCheck2 -->|Yes - main| BuildMain[✅ Build Job<br/>3 OS: Ubuntu, Windows, macOS]
    BuildMain --> ReleaseCheck1{Release Condition:<br/>Tag v*?}
    ReleaseCheck1 -->|No - main push| ReleaseSkipMain[❌ Release Job SKIPPED<br/>No tag]
    ReleaseSkipMain --> CovCheck2{Coverage Condition:<br/>Push to main?}
    CovCheck2 -->|Yes - main| CovMain[✅ Coverage Job<br/>Ubuntu only]
    CovMain --> MainEnd([Main Push Complete<br/>~33 minutes])
    
    %% Develop Push Flow
    DevPush --> TestDev[✅ Test Job<br/>3 OS: Ubuntu, Windows, macOS]
    TestDev --> BuildCheck3{Build Condition:<br/>Push to main OR tag?}
    BuildCheck3 -->|No - develop| BuildSkipDev[❌ Build Job SKIPPED<br/>Not main/tag]
    BuildSkipDev --> CovCheck3{Coverage Condition:<br/>Push to main?}
    CovCheck3 -->|No - develop| CovSkipDev[❌ Coverage Job SKIPPED<br/>Not main]
    CovSkipDev --> DevEnd([Develop Push Complete<br/>~15 minutes])
    
    %% Release Tag Flow
    TagPush --> TestTag[✅ Test Job<br/>3 OS: Ubuntu, Windows, macOS]
    TestTag --> BuildCheck4{Build Condition:<br/>Push to main OR tag?}
    BuildCheck4 -->|Yes - tag| BuildTag[✅ Build Job<br/>3 OS: Ubuntu, Windows, macOS]
    BuildTag --> ReleaseCheck2{Release Condition:<br/>Tag v*?}
    ReleaseCheck2 -->|Yes - tag| ReleaseTag[✅ Release Job<br/>Creates GitHub Release<br/>Uploads artifacts]
    ReleaseTag --> CovCheck4{Coverage Condition:<br/>Push to main?}
    CovCheck4 -->|Yes - tag on main| CovTag[✅ Coverage Job<br/>Ubuntu only]
    CovTag --> TagEnd([Release Complete<br/>~35 minutes])
    
    style TestPR fill:#228B22,color:#FFFFFF
    style TestMain fill:#228B22,color:#FFFFFF
    style TestDev fill:#228B22,color:#FFFFFF
    style TestTag fill:#228B22,color:#FFFFFF
    
    style BuildMain fill:#228B22,color:#FFFFFF
    style BuildTag fill:#228B22,color:#FFFFFF
    style BuildSkipPR fill:#DC143C,color:#FFFFFF
    style BuildSkipDev fill:#DC143C,color:#FFFFFF
    
    style ReleaseTag fill:#228B22,color:#FFFFFF
    style ReleaseSkipMain fill:#DC143C,color:#FFFFFF
    
    style CovMain fill:#228B22,color:#FFFFFF
    style CovTag fill:#228B22,color:#FFFFFF
    style CovSkipPR fill:#DC143C,color:#FFFFFF
    style CovSkipDev fill:#DC143C,color:#FFFFFF
    
    style PREnd fill:#1E90FF,color:#FFFFFF
    style MainEnd fill:#1E90FF,color:#FFFFFF
    style DevEnd fill:#1E90FF,color:#FFFFFF
    style TagEnd fill:#1E90FF,color:#FFFFFF
```

**Legend:**

- 🟢 **Green** (Forest Green) = Job runs
- 🔴 **Red** (Crimson) = Job skipped
- 🔵 **Blue** (Dodger Blue) = Completion state

---

## Job Details

### Quick Check Job (ci.yml) - NEW in v0.0.3

**Condition:** Pull requests only

```yaml
name: Quick Check (Lint + Unit Tests)
if: github.event_name == 'pull_request'
runs-on: ubuntu-latest
```

**When it runs:**

- ✅ Pull requests to main or develop
- ❌ Push to main (SKIPPED - uses Full Test Suite)
- ❌ Push to develop (SKIPPED - uses Full Test Suite)
- ❌ Push tag v* (SKIPPED - uses Full Test Suite)

**Steps:**

- Checkout code
- Install Rust toolchain (stable)
- Cache cargo dependencies
- Check formatting with `cargo fmt --all -- --check`
- Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- Run `cargo test --workspace --lib --bins --verbose` (unit tests only)

**Optimization Benefits:**

- ⚡ **2-3 minutes** (vs 10-15 for full suite)
- 🖥️ **Single OS** (Ubuntu only, no Windows/macOS)
- 📦 **Unit tests only** (no integration tests)
- 💰 **60-70% CI time savings** on feature branches

---

### Full Test Suite Job (ci.yml)

**Condition:** Push to main/develop/tags only

```yaml
name: Full Test Suite
if: |
  github.ref == 'refs/heads/main' || 
  github.ref == 'refs/heads/develop' ||
  startsWith(github.ref, 'refs/tags/')
runs-on: [ubuntu-latest, windows-latest, macos-latest]
```

**When it runs:**

- ❌ Pull requests (SKIPPED - uses Quick Check instead)
- ✅ Push to main
- ✅ Push to develop
- ✅ Push tag v*

**Steps:**

- Checkout code
- Install Rust toolchain (stable)
- Cache cargo dependencies
- Run `cargo test --workspace --verbose` (all tests)
- Run `cargo clippy --workspace --all-targets -- -D warnings`
- Check formatting with `cargo fmt --all -- --check`

**Why separate from quick-check?**

- Integration testing before main/develop merge
- Cross-platform validation (Linux, Windows, macOS)
- Full test coverage including integration tests
- Ensures production-ready code quality

---

### Build Job (ci.yml)

**Condition:** Push to main/develop OR tag v*

```yaml
name: Build Release
needs: test
if: |
  github.event_name == 'push' && 
  (github.ref == 'refs/heads/main' || 
   github.ref == 'refs/heads/develop' || 
   startsWith(github.ref, 'refs/tags/v'))
runs-on: [ubuntu-latest, windows-latest, macos-latest]
```

**When it runs:**

- ❌ Pull requests (SKIPPED)
- ✅ Push to main
- ✅ Push to develop (NEW in v0.0.3)
- ✅ Push tag v*

**Artifacts:**

- `ferrisscript-linux-x86_64.so` (Ubuntu)
- `ferrisscript-windows-x86_64.dll` (Windows)
- `ferrisscript-macos-x86_64.dylib` (macOS)

---

### Release Job (ci.yml)

**Condition:** Tag v* only

```yaml
name: Create Release
needs: build
if: startsWith(github.ref, 'refs/tags/v')
runs-on: ubuntu-latest
```

**When it runs:**

- ❌ Pull requests (SKIPPED)
- ❌ Push to main (SKIPPED)
- ❌ Push to develop (SKIPPED)
- ✅ Push tag v*

**Actions:**

- Downloads all build artifacts
- Creates GitHub Release with tag name
- Attaches platform binaries
- Includes `RELEASE_NOTES.md` as body

---

### Coverage Job (MOVED to code-scanning.yml)

> **Note**: As of October 8, 2025, code coverage has been moved to `code-scanning.yml` for consolidation with other security/quality scanning tools (SonarQube, future CodeQL).

See [Code Scanning & Coverage](#code-scanning--coverage) workflow for current implementation.

---

## Event Flow Examples

### Example 1: Opening a Feature Branch Pull Request

```text
1. Developer creates PR #42 (feature/new-feature → develop)
   ↓
2. CI Triggers: pull_request event
   ↓
3. Quick Check Job: ✅ Runs (Ubuntu ONLY)
   ├─ cargo fmt --all -- --check
   ├─ cargo clippy --workspace --all-targets --all-features
   └─ cargo test --workspace --lib --bins (unit tests)
   ↓
4. Full Test Suite: ❌ Skipped (only for main/develop/tags)
   ↓
5. Build Job: ❌ Skipped (only for push events)
   ↓
6. Coverage Job: ❌ Skipped (only for push to main/develop)
   ↓
7. Docs Lint: ✅ Runs if markdown files changed
   ├─ markdownlint
   └─ markdown-link-check (excludes archive/)
   ↓
8. PR Template: ✅ Applied based on branch name
   ↓
9. Result: PR validated in ~2-3 minutes ⚡
```

### Example 2: Documentation-Only Pull Request

```text
1. Developer creates PR #43 (docs/update-readme → develop)
   - Changes: README.md, docs/FAQ.md only
   ↓
2. CI Triggers: pull_request event
   ↓
3. Quick Check Job: ❌ Skipped (path filter: **.md)
   ↓
4. Full Test Suite: ❌ Skipped (only for main/develop/tags)
   ↓
5. Build Job: ❌ Skipped (only for push events)
   ↓
6. Coverage Job: ❌ Skipped (only for push to main/develop)
   ↓
7. Docs Lint: ✅ Runs
   ├─ markdownlint
   └─ markdown-link-check
   ↓
8. PR Template: ✅ Applied (docs template)
   ↓
9. Result: PR validated in ~1 minute 📝
   (95% savings vs code changes!)
```

### Example 3: Merging PR to Develop

```text
1. PR #42 merged to develop branch
   ↓
2. CI Triggers: push event (develop)
   ↓
3. Quick Check Job: ❌ Skipped (only for PRs)
   ↓
4. Full Test Suite: ✅ Runs (Ubuntu, Windows, macOS)
   ├─ cargo test --workspace --verbose (all tests)
   ├─ cargo clippy --workspace --all-targets
   └─ cargo fmt --all -- --check
   ↓
5. Build Job: ✅ Runs (Ubuntu, Windows, macOS)
   ├─ Builds release binaries for all platforms
   └─ Uploads artifacts
   ↓
6. Coverage Job: ✅ Runs (Ubuntu)
   ├─ cargo tarpaulin
   └─ Upload to Codecov
   ↓
7. Release Job: ❌ Skipped (no tag)
   ↓
8. Docs Lint: ✅ Runs if markdown files changed
   ↓
9. Result: Develop branch fully validated in ~28-33 minutes
```

### Example 4: Merging Develop to Main

```text
1. PR from develop → main merged
   ↓
2. CI Triggers: push event (main)
   ↓
3. Quick Check Job: ❌ Skipped (only for PRs)
   ↓
4. Full Test Suite: ✅ Runs (Ubuntu, Windows, macOS)
   ├─ All tests across all platforms
   └─ Ensures production readiness
   ↓
5. Build Job: ✅ Runs (Ubuntu, Windows, macOS)
   ├─ Builds release binaries
   └─ Uploads artifacts
   ↓
6. Coverage Job: ✅ Runs (Ubuntu)
   ├─ cargo tarpaulin
   └─ Upload to Codecov
   ↓
7. Release Job: ❌ Skipped (no tag yet)
   ↓
8. Result: Main branch fully validated in ~28-33 minutes
```

### Example 5: Creating a Release

```text
1. Developer pushes tag v0.1.0 on main branch
   ↓
2. CI Triggers: push event (tag v*)
   ↓
3. Quick Check Job: ❌ Skipped (only for PRs)
   ↓
4. Full Test Suite: ✅ Runs (Ubuntu, Windows, macOS)
   ├─ Final validation of release
   └─ All tests across all platforms
   ↓
5. Build Job: ✅ Runs (Ubuntu, Windows, macOS)
   ├─ Builds release binaries for all platforms
   └─ Uploads artifacts
   ↓
6. Release Job: ✅ Runs (Ubuntu)
   ├─ Downloads all artifacts
   ├─ Creates GitHub Release v0.1.0
   ├─ Attaches binaries + gdextension
   └─ Includes RELEASE_NOTES.md
   ↓
7. Coverage Job: ✅ Runs (Ubuntu)
   ├─ cargo tarpaulin
   └─ Upload to Codecov
   ↓
8. Result: Release v0.1.0 published in ~30-35 minutes 🎉
```

---

## Performance Metrics

### Timing Breakdown (v0.0.3 Optimized)

| Event | Quick Check | Full Test | Build | Release | Coverage | Total Time |
|-------|-------------|-----------|-------|---------|----------|------------|
| **PR to main/develop** | ~2-3m (1 OS) | ❌ Skip | ❌ Skip | ❌ Skip | ❌ Skip | **~2-3 min** ✨ |
| **Push to main** | ❌ Skip | ~10-15m (3 OS) | ~15m (3 OS) | ❌ Skip | ~3m | **~28-33 min** |
| **Push to develop** | ❌ Skip | ~10-15m (3 OS) | ~15m (3 OS) | ❌ Skip | ~3m | **~28-33 min** |
| **Push tag v*** | ❌ Skip | ~10-15m (3 OS) | ~15m (3 OS) | ~2m | ~3m | **~30-35 min** |
| **Docs-only PR** | ❌ Skipped (path filter) | ❌ Skip | ❌ Skip | ❌ Skip | ❌ Skip | **~0 min** 🎉 |

### Cost Savings Analysis

#### Before v0.0.3 Optimization

- Every PR: Full Test (15m × 3 OS) = ~15 minutes
- Every push to main: Test + Build + Coverage = ~33 minutes
- **Total for feature PR → merge: ~48 minutes**

#### After v0.0.3 Optimization

- Feature PR: Quick Check (2-3m × 1 OS) = ~2-3 minutes
- Push to main/develop: Full Test + Build + Coverage = ~28-33 minutes
- **Total for feature PR → merge: ~30-36 minutes**
- **Savings: 12-18 minutes per feature (25-37% reduction)** 🚀

#### Docs-Only Changes

- Before: Full Test Suite = ~15 minutes
- After: CI skipped via path filters = ~0 minutes
- **Savings: 95-100% on documentation PRs** 📝

#### Optimization Strategy (v0.0.3)

1. **Quick Check Job** (NEW) - Fast PR feedback
   - Rationale: Feature branches need fast iteration
   - Single OS (Ubuntu), unit tests only
   - Savings: ~12-13 minutes per PR (80% faster)

2. **Path Filters** (NEW) - Skip docs-only changes
   - Rationale: Markdown changes don't need Rust CI
   - Ignores: `docs/**`, `**.md`, `LICENSE`, `.gitignore`
   - Savings: 95-100% on docs PRs

3. **Build Job** - Skip on PRs, add develop
   - Rationale: PRs don't need release artifacts
   - Now builds on develop for integration testing
   - Savings: ~15 minutes per PR

4. **Coverage Job** - Run on main AND develop
   - Rationale: Monitor coverage on integration branch
   - No change to PR cost (still skipped)

5. **Prevent Duplicate Runs** (NEW)
   - Removed `feature/**` from push triggers
   - Prevents running twice when PR exists
   - Saves duplicate CI runs

#### Cumulative Savings

- **Per feature PR:** 60-70% faster (2-3 min vs 15 min)
- **Per docs PR:** 95-100% savings (CI skipped entirely)
- **Overall CI cost reduction:** ~70% across all PR types

---

## Configuration Files

### Workflow Configs

- `.markdownlint.json` - Markdown style rules
- `.markdown-link-check.json` - Link checking configuration
- `tarpaulin.toml` - Code coverage settings

### Related Documentation

- [CONTRIBUTING.md](../../CONTRIBUTING.md) - Branch naming conventions
- [docs/DEVELOPMENT.md](../../docs/DEVELOPMENT.md) - Development workflow
- [.github/prompts/PR_TEMPLATE_SYSTEM.md](../prompts/PR_TEMPLATE_SYSTEM.md) - PR template system guide

---

## Troubleshooting

### Workflow Not Triggering

**Issue:** Workflow doesn't run on push/PR

**Check:**

1. Branch matches trigger pattern (main, develop, or PR to those branches)
2. File paths match (CI skips docs/**,**.md, LICENSE, .gitignore on push)
3. Workflow file has no YAML syntax errors
4. Feature branches should NOT trigger push events (PRs only)

### Job Skipped Unexpectedly

**Issue:** Expected job doesn't run

**Check:**

1. Review job `if` conditions in `ci.yml`
2. Verify event type matches condition:
   - `quick-check`: Only on `pull_request` events
   - `test` (Full Test Suite): Only on push to main/develop/tags
3. Check previous job success (for `needs` dependencies)
4. Verify branch name in condition (main vs develop)

### Quick Check Not Running on PR

**Issue:** PR doesn't trigger quick-check job

**Check:**

1. PR targets main OR develop branch
2. Not a docs-only change (those skip quick-check)
3. Workflow file is valid YAML
4. Check Actions tab for any errors

### Full Test Suite Running on PR

**Issue:** PR triggers full test suite instead of quick-check

**Check:**

1. Verify CI workflow has correct conditions
2. Should be: `if: github.event_name == 'pull_request'` for quick-check
3. Should be: `if: github.ref == 'refs/heads/main' || ...` for full test
4. May indicate workflow file issue

### CI Running Twice on Same Commit

**Issue:** Same commit triggers CI multiple times

**Cause:** This should NOT happen in v0.0.3+

**Check:**

1. Verify `feature/**` is NOT in `push.branches` trigger
2. Should only trigger on PR, not on push to feature branches
3. If still occurring, check for multiple open PRs

### PR Template Not Applied

**Issue:** Template not auto-applied on PR creation

**Check:**

1. Branch name matches pattern (`bugfix/*`, `feature/*`, `docs/*`)
2. PR was just opened (not edited/synced)
3. Template file exists in `.github/PULL_REQUEST_TEMPLATE/`
4. PR targets main or develop (not other branches)

### Build Artifacts Missing

**Issue:** Release job can't find artifacts

**Check:**

1. Build job completed successfully
2. Artifact names match expected pattern
3. Download artifacts step has correct names
4. Check if push was to main/develop/tag (build only runs on those)

### Path Filter Not Working

**Issue:** CI runs on docs-only changes

**Check:**

1. Verify `paths-ignore` in `ci.yml` includes `docs/**` and `**.md`
2. This only applies to push events, NOT pull_request events
3. PRs will run quick-check regardless (but it's fast)
4. Docs-lint will still run for markdown changes

---

## Maintenance Notes

### Adding New Jobs

1. Add job definition to appropriate workflow file
2. Update this README with job details
3. Update Mermaid diagram if affects CI/CD flow
4. Test with a draft PR

### Modifying Conditions

1. Edit `if` conditions in workflow file
2. Update job details section in this README
3. Regenerate timing estimates
4. Update Mermaid diagram flow

### Changing Triggers

1. Modify `on` section in workflow file
2. Update "Available Workflows" section
3. Test with appropriate event type

---

## See Also

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust CI/CD Best Practices](https://doc.rust-lang.org/cargo/guide/continuous-integration.html)
- [Codecov Integration](https://docs.codecov.com/docs/quick-start)
- [Mermaid Diagram Syntax](https://mermaid.js.org/syntax/flowchart.html)
