# Centralized Version Management & Branching Strategy Research

**Date**: October 8, 2025  
**Author**: GitHub Copilot (Research Task)  
**Status**: Feasibility Analysis  
**Version**: Draft 1.0

---

## 🎯 Executive Summary

This document provides comprehensive research and recommendations for simplifying FerrisScript's release management, branching strategy, and version tracking. The goal is to reduce manual overhead, prevent version desynchronization, and streamline the development-to-release workflow.

### Key Findings

**Current Pain Points**:

- ✅ Version numbers scattered across **7+ locations** (manual sync required)
- ✅ Long-lived `develop` branch accumulates **24+ commits** per release
- ✅ Manual version bumping in multiple file formats (TOML, JSON, YAML, Markdown)
- ✅ No automated version propagation across cargo/npm/docs ecosystems
- ✅ Git history on `develop` must be "reset" after each release

**Recommended Approach** (Hybrid Solution):

1. **Version Management**: Centralized `.version` file + automated sync scripts (NOT .env)
2. **Branching Strategy**: GitHub Flow (feature → main, no develop) + release branches
3. **Automation**: Pre-commit hooks + CI validation for version consistency
4. **Tag Strategy**: Semantic versioning with component-specific pre-release tags

**Expected Benefits**:

- ⏱️ **Time Savings**: ~15-20 minutes per release (no manual version syncing)
- 🔒 **Consistency**: Automated validation prevents version mismatches
- 📦 **Flexibility**: Independent versioning for cargo, npm, and docs when needed
- 🚀 **Simplified Workflow**: Eliminate long-lived integration branch complexity

---

## 📋 Table of Contents

1. [Current State Analysis](#1-current-state-analysis)
2. [Version Management Research](#2-version-management-research)
3. [Branching Strategy Research](#3-branching-strategy-research)
4. [Impact Analysis](#4-impact-analysis)
5. [Proposed Architecture](#5-proposed-architecture)
6. [Migration Strategy](#6-migration-strategy)
7. [Implementation Roadmap](#7-implementation-roadmap)
8. [Recommendations](#8-recommendations)

---

## 1. Current State Analysis

### 1.1 Version Tracking Locations

**Identified Files**:

| File | Current Version | Format | Purpose |
|------|----------------|--------|---------|
| `Cargo.toml` (workspace) | 0.0.3 | TOML | Rust workspace version (all crates) |
| `extensions/vscode/package.json` | 0.0.3 | JSON | VS Code extension |
| `package.json` (root) | 0.0.3 | JSON | Documentation tooling |
| `package-lock.json` (root) | **0.0.2** ⚠️ | JSON | NPM lock file (DESYNC) |
| `README.md` | 0.0.3 | Markdown | Badges, installation instructions |
| `docs/_config.yml` | Implicit | YAML | Jekyll site (references v0.0.3 folders) |
| `CHANGELOG.md` | 0.0.3 | Markdown | Version history |
| `docs/planning/v0.0.X/` | Directory names | Filesystem | Version-specific documentation |

**Pain Points**:

1. **Manual Synchronization**: Each release requires editing 5+ files individually
2. **Format Diversity**: TOML, JSON, YAML, Markdown, directory names
3. **Desynchronization Risk**: `package-lock.json` still shows 0.0.2 (missed in v0.0.3 bump)
4. **No Validation**: No automated check that versions match across files
5. **Documentation Overhead**: Version numbers embedded in many doc files

### 1.2 Branching Strategy

**Current Workflow** (Git Flow-style):

```
feature/X ──PR──> develop ──PR──> main
                   ↑                ↓
                   │          (tagged v0.0.X)
                   └──────────────┘
                  (reset after release)
```

**Current Metrics**:

- `develop` is **24 commits** ahead of `main` (as of v0.0.3 development)
- v0.0.3 involved **39 commits** total on `develop` during development
- `main` only updated on release (receives squashed PR from `develop`)
- CI runs different test suites based on branch (quick-check on PR, full suite on develop/main)

**Documented in**:

- `docs/planning/v0.0.3/v0.0.3-roadmap.md` (lines 416-490)
- `.github/workflows/ci.yml` (conditional jobs based on branch)
- `CONTRIBUTING.md` (feature branch naming, PR process)

**Pain Points**:

1. **Long-Lived Integration Branch**: `develop` accumulates many commits between releases
2. **History Management**: No clear strategy for "resetting" develop to match main
3. **CI Duplication**: Need to maintain branch-specific CI logic
4. **Contributor Confusion**: Two target branches (when to use develop vs main?)
5. **Release Bottleneck**: All features must wait for develop → main PR

### 1.3 Release Process

**Current Process** (from `RELEASING.md`):

1. Manual version bump in `Cargo.toml`, `package.json`, `extensions/vscode/package.json`
2. Update `CHANGELOG.md` with new version section
3. Update `README.md` badges and references
4. Commit version bump to `develop`
5. PR `develop` → `main` (comprehensive release PR)
6. Merge to `main`
7. Create Git tag `vX.Y.Z` on `main`
8. ⚠️ **Manual step**: Reset `develop` to match `main` (not documented)

**Time Estimate**: ~30-45 minutes per release (mostly manual verification)

**Error-Prone Steps**:

- Forgetting to update `package-lock.json` (happened in v0.0.3)
- Missing version references in documentation
- Inconsistent version tags across files

---

## 2. Version Management Research

### 2.1 Industry Approaches

#### Option A: `.env` File for Version Tracking

**Concept**: Store all version numbers in a root `.env` file, reference via environment variables.

**Example `.env`**:

```env
# FerrisScript Version Configuration
VERSION_MAJOR=0
VERSION_MINOR=0
VERSION_PATCH=3
VERSION_FULL=0.0.3

# Component Versions (independent if needed)
CARGO_VERSION=0.0.3
VSCODE_EXT_VERSION=0.0.3
DOCS_VERSION=0.0.3

# Release Tags
VERSION_TAG=alpha
RELEASE_DATE=2025-10-08

# Branch Configuration
MAIN_BRANCH=main
INTEGRATION_BRANCH=develop
```

**Pros**:

- ✅ Single source of truth for version numbers
- ✅ Easy to read/parse in scripts (shell, Node.js, Rust)
- ✅ Supports environment-specific overrides
- ✅ Human-readable format

**Cons**:

- ❌ `.env` not natively supported by Cargo or npm
- ❌ Requires build-time variable substitution (complexity)
- ❌ Git merge conflicts on single-line changes
- ❌ Not a Rust ecosystem standard

**Verdict**: ⚠️ **Not Recommended** for Rust-centric projects. `.env` is a Node.js convention, not widely adopted in Rust tooling.

---

#### Option B: Centralized `.version` File + Sync Scripts

**Concept**: Store version in a simple text file, use scripts to propagate to all target files.

**Example `.version`**:

```
0.0.3
```

**Or structured**:

```toml
# .version (TOML format for flexibility)
[version]
major = 0
minor = 0
patch = 3
tag = "alpha"

[components]
cargo = "0.0.3"
vscode = "0.0.3"
docs = "0.0.3"

[metadata]
release_date = "2025-10-08"
```

**Sync Script**: `scripts/sync-versions.ps1` / `.sh`

```powershell
# sync-versions.ps1
$VERSION = Get-Content .version -Raw
$VERSION = $VERSION.Trim()

# Update Cargo.toml
(Get-Content Cargo.toml) -replace 'version = "[^"]+"', "version = `"$VERSION`"" | Set-Content Cargo.toml

# Update package.json files
$pkgJson = Get-Content package.json | ConvertFrom-Json
$pkgJson.version = $VERSION
$pkgJson | ConvertTo-Json -Depth 10 | Set-Content package.json

# Update VSCode extension
$vscodePkg = Get-Content extensions/vscode/package.json | ConvertFrom-Json
$vscodePkg.version = $VERSION
$vscodePkg | ConvertTo-Json -Depth 10 | Set-Content extensions/vscode/package.json

Write-Output "✅ Synced version $VERSION to all files"
```

**Pros**:

- ✅ Simple, Rust-friendly approach
- ✅ Scriptable across platforms (PowerShell + Bash)
- ✅ Can be validated in CI (fail if out of sync)
- ✅ Supports independent component versioning

**Cons**:

- ⚠️ Requires discipline to run sync script before committing
- ⚠️ Manual intervention for version bumps (no full automation)
- ⚠️ Potential for desync if script not run

**Verdict**: ✅ **Recommended**. Simple, maintainable, fits Rust ecosystem.

---

#### Option C: Cargo Workspace Version + npm Scripts

**Concept**: Use `Cargo.toml` as source of truth, extract version via `cargo metadata`, sync to npm.

**Example `package.json` script**:

```json
{
  "scripts": {
    "sync-version": "node scripts/sync-version.js"
  }
}
```

**`scripts/sync-version.js`**:

```javascript
const { execSync } = require('child_process');
const fs = require('fs');

// Extract version from Cargo.toml
const cargoMetadata = JSON.parse(
  execSync('cargo metadata --format-version 1 --no-deps').toString()
);
const cargoVersion = cargoMetadata.workspace_members[0].split(' ')[1];

// Update package.json
const pkg = require('../package.json');
pkg.version = cargoVersion;
fs.writeFileSync('package.json', JSON.stringify(pkg, null, 2));

// Update VS Code extension
const vscode = require('../extensions/vscode/package.json');
vscode.version = cargoVersion;
fs.writeFileSync('extensions/vscode/package.json', JSON.stringify(vscode, null, 2));

console.log(`✅ Synced version ${cargoVersion} from Cargo.toml`);
```

**Pros**:

- ✅ Cargo.toml remains authoritative (Rust-first)
- ✅ Leverages existing `cargo metadata` API
- ✅ No new file formats introduced
- ✅ npm scripts provide cross-platform automation

**Cons**:

- ⚠️ Requires Node.js even for Rust-only operations
- ⚠️ Complex dependency (cargo → node → json updates)
- ⚠️ Doesn't handle documentation version references

**Verdict**: ⚙️ **Viable Alternative**. Good for Rust-first projects, but adds Node.js dependency.

---

#### Option D: Automated Version Bumping Tools

**Tools**:

1. **cargo-release** ([GitHub](https://github.com/crate-ci/cargo-release))
   - Automates version bumping, Git tagging, publishing to crates.io
   - Supports workspace version management
   - Integrates with conventional commits

2. **semantic-release** ([GitHub](https://github.com/semantic-release/semantic-release))
   - Fully automated versioning based on commit messages
   - Supports npm, but requires plugins for Cargo
   - Generates CHANGELOG.md automatically

3. **release-plz** ([GitHub](https://github.com/MarcoIeni/release-plz))
   - GitHub Action for Rust projects
   - Automated PR creation for version bumps
   - Supports workspace crates

**Pros**:

- ✅ Fully automated (no manual version editing)
- ✅ Based on conventional commits (feat, fix, BREAKING CHANGE)
- ✅ Generates changelogs automatically
- ✅ Industry-standard approach

**Cons**:

- ❌ Steep learning curve for configuration
- ❌ Requires strict commit message discipline
- ❌ May not handle multi-ecosystem projects (Cargo + npm + docs)
- ❌ Less control over versioning strategy

**Verdict**: 🔮 **Future Enhancement**. Overkill for v0.0.X releases, but valuable for v1.0+.

---

### 2.2 Recommended Version Management Solution

**Hybrid Approach: Centralized `.version` + Sync Scripts + CI Validation**

**Architecture**:

```
.version (TOML)
    ↓
scripts/sync-versions.{ps1,sh}
    ↓
├── Cargo.toml (workspace.package.version)
├── package.json (root, documentation tooling)
├── extensions/vscode/package.json (VS Code extension)
└── [Future] docs/_config.yml (Jekyll site metadata)
    ↓
.github/workflows/version-check.yml (CI validation)
```

**Workflow**:

1. Developer updates `.version` file (single edit)
2. Runs `./scripts/sync-versions.sh` (or pre-commit hook auto-runs)
3. Script propagates version to all target files
4. CI validates all versions match on PR (fails if desync detected)
5. Git tag created with same version on release

**Benefits**:

- ✅ Single source of truth (`.version`)
- ✅ Automated propagation (sync script)
- ✅ CI-enforced consistency (version-check workflow)
- ✅ Supports independent component versioning (if needed)
- ✅ Pre-commit hook can auto-sync (optional)

---

## 3. Branching Strategy Research

### 3.1 Workflow Comparison

#### Current: Git Flow (Modified)

**Structure**:

```
main        ────●────────────────●─────> (production)
                 ↑                ↑
develop     ─●───┴──●──●──●──●───┴──●──> (integration)
              ↑      ↑  ↑  ↑  ↑
features      └──●   └──└──└──┘
```

**Characteristics**:

- Two long-lived branches (`main`, `develop`)
- Features merge to `develop` for integration testing
- `develop` merges to `main` for releases
- `develop` reset to `main` after each release (manual)

**Pros**:

- ✅ Integration testing on `develop` before production
- ✅ `main` always represents production-ready state
- ✅ Clear separation between development and release

**Cons**:

- ❌ Long-lived integration branch accumulates commits (24+ for v0.0.3)
- ❌ Manual "reset" of `develop` required after release (error-prone)
- ❌ Contributor confusion (which branch to target?)
- ❌ CI complexity (branch-specific logic)
- ❌ Merge conflicts between `develop` and `main` if not synced

---

#### Alternative A: GitHub Flow

**Structure**:

```
main    ──●────●──●────●────●──────> (production + development)
           ↑    ↑  ↑    ↑    ↑
features   └──● └──└──● └──● └──●
```

**Characteristics**:

- Single long-lived branch (`main`)
- Features branch directly from `main`
- Features merge directly to `main` (after review + CI)
- Deployments triggered by merges to `main`
- Tags used for releases

**Pros**:

- ✅ Simplest workflow (1 long-lived branch)
- ✅ No branch synchronization issues
- ✅ Clear contributor workflow (always target `main`)
- ✅ Fast feedback loop (no integration branch delay)
- ✅ Industry standard for open-source projects

**Cons**:

- ⚠️ `main` contains unreleased features (not production-only)
- ⚠️ Requires strong CI/CD to ensure `main` is always releasable
- ⚠️ No "staging" branch for multi-feature integration testing
- ⚠️ Rollbacks more complex (revert commits vs. branch switch)

**Best For**: Projects with strong CI, frequent releases, trunk-based development

---

#### Alternative B: GitHub Flow + Release Branches

**Structure**:

```
main        ──●──●──●────●──●──────●──> (development)
               ↑  ↑  ↑    ↑  ↑      ↑
features       └──└──└──● └──└────● └──●
                         ↓         ↓
release/v0.0.3           ●──●──────● (hotfixes only)
                                   ↓
                                  tag v0.0.3
```

**Characteristics**:

- `main` is active development branch
- Release branches (`release/vX.Y.Z`) created from `main` when ready
- Release branches only receive hotfixes (no new features)
- Tags created from release branches
- Release branches can be long-lived or deleted after release

**Pros**:

- ✅ Clear separation between development and release stabilization
- ✅ Hotfixes can be applied without blocking new development
- ✅ Multiple releases can be maintained simultaneously (v0.0.3, v0.0.4)
- ✅ `main` always moving forward (no resets needed)

**Cons**:

- ⚠️ Requires discipline to only backport hotfixes (not features)
- ⚠️ Slightly more complex than pure GitHub Flow
- ⚠️ Release branches must be merged back to `main` (hotfix propagation)

**Best For**: Projects with version support requirements, alpha/beta releases

---

#### Alternative C: Trunk-Based Development

**Structure**:

```
main (trunk)  ──●─●─●─●─●─●─●─●─●─●─●─●─●─●─●─●──> (always releasable)
                 ↑ ↑ ↑ ↑ ↑ ↑ ↑ ↑ ↑ ↑ ↑ ↑ ↑ ↑ ↑ ↑
short-lived      └─└─└─└─└─└─└─└─└─└─└─└─└─└─└─┘
branches         (< 1 day, small PRs)
```

**Characteristics**:

- Single `main` branch (trunk)
- Short-lived feature branches (1-2 days max)
- Very small PRs (< 400 lines)
- Feature flags for incomplete features
- Continuous deployment

**Pros**:

- ✅ Maximum velocity (no integration delays)
- ✅ Minimal merge conflicts (small, frequent merges)
- ✅ Forces small, incremental changes
- ✅ Industry best practice for high-velocity teams

**Cons**:

- ❌ Requires feature flags for incomplete work
- ❌ Demands strong testing infrastructure (unit + integration + e2e)
- ❌ Not suitable for alpha releases (always shipping to production)
- ❌ Difficult for solo/small teams

**Best For**: Large teams, SaaS products, mature CI/CD infrastructure

---

### 3.2 Recommended Branching Strategy

**Recommendation: GitHub Flow + Release Branches** (Hybrid Approach)

**Rationale**:

1. **Simplicity**: Eliminates long-lived `develop` branch complexity
2. **Flexibility**: Release branches allow stabilization without blocking development
3. **Version Support**: Can maintain multiple releases (v0.0.3, v0.0.4) simultaneously
4. **Clear History**: `main` shows linear development progress
5. **Contributor-Friendly**: Simple rule: "Always branch from and target `main`"

**Proposed Workflow**:

```
Step 1: Feature Development
  main ──●──────> (latest development)
          ↓
  feature/X ──●──●──● (PR back to main)

Step 2: Release Preparation
  main ──●──●──●──────> (continue development)
          ↑        ↓
          PR    release/v0.0.3 ──●──● (bugfixes only)
                                    ↓
                                  tag v0.0.3

Step 3: Hotfix (if needed)
  release/v0.0.3 ──●──● (hotfix)
                    ↓   ↓
                    ↓  tag v0.0.3.1
                    ↓
  main ──●──●──●──●─┘ (cherry-pick hotfix)
```

**Key Rules**:

1. **All features target `main`** (no develop branch)
2. **Release branches created from `main`** when version is feature-complete
3. **Release branches only accept bugfixes** (no new features)
4. **Tags created from release branches** (e.g., `v0.0.3` on `release/v0.0.3`)
5. **Hotfixes merged back to `main`** (cherry-pick or merge)
6. **Release branches can be deleted** after next version released (optional)

**CI Strategy**:

- **PRs to `main`**: Run quick-check (lint + unit tests)
- **Merges to `main`**: Run full suite (cross-platform, coverage, integration tests)
- **Release branches**: Run full suite on every push
- **Tags**: Trigger release workflows (GitHub Releases, cargo publish, VS Code marketplace)

---

### 3.3 Handling v0.0.3 History (Transition Plan)

**Problem**: `develop` is 24 commits ahead of `main`, contains all v0.0.3 work.

**Option A: Squash Merge develop → main (Recommended)**

```bash
# 1. Create release PR from develop to main
git checkout main
git pull origin main
git merge --squash develop
git commit -m "Release v0.0.3: Editor Experience Alpha

- Enhanced error diagnostics with error codes
- VS Code IntelliSense features
- Development scripts and CI improvements

See CHANGELOG.md for full details."

# 2. Push to main
git push origin main

# 3. Tag the release
git tag v0.0.3
git push origin v0.0.3

# 4. Delete develop branch (START FRESH)
git branch -d develop
git push origin --delete develop
```

**Pros**:

- ✅ Clean release commit on `main` (single commit for v0.0.3)
- ✅ No complex rebase or history rewriting
- ✅ Clear changelog entry
- ✅ Eliminates develop branch permanently

**Cons**:

- ⚠️ Loses granular commit history on `main` (stored on feature branches)

**Option B: Rebase develop onto main**

```bash
# Preserve individual commit history (not recommended)
git checkout develop
git rebase main
git checkout main
git merge --ff-only develop
```

**Pros**:

- ✅ Preserves all 24 commits on `main`

**Cons**:

- ❌ Messy history (24 commits for single release)
- ❌ Hard to identify "what's in v0.0.3" (no single release commit)

**Recommendation**: **Option A (Squash Merge)**. Clean history, clear releases, industry standard.

---

## 4. Impact Analysis

### 4.1 Files Requiring Changes

**Version Management Changes**:

| Category | Files | Change Type | Effort |
|----------|-------|-------------|--------|
| **New Files** | `.version`, `scripts/sync-versions.{ps1,sh}`, `.github/workflows/version-check.yml` | Create | Medium |
| **Modified Files** | `Cargo.toml`, `package.json`, `extensions/vscode/package.json` | Auto-updated by script | Low |
| **Documentation** | `RELEASING.md`, `CONTRIBUTING.md`, `README.md` (installation instructions) | Update workflow | Low |
| **CI Workflows** | `.github/workflows/ci.yml`, `code-scanning.yml`, `benchmarks.yml` | Remove `develop` branch triggers | Low |
| **Prompts** | `.github/prompts/workstream-execution.prompt.md` | Remove `develop` references | Low |
| **Docs** | `docs/planning/v0.0.3/v0.0.3-roadmap.md`, `docs/LEARNINGS.md` | Update branching strategy docs | Low |

**Total Estimated Files**: ~20-30 files

---

**Branching Strategy Changes**:

| Area | Change | Effort |
|------|--------|--------|
| **GitHub Repository Settings** | Remove branch protection from `develop`, delete `develop` branch | Low |
| **CI/CD Workflows** | Update branch triggers (`develop` → `main`), remove branch-specific logic | Medium |
| **Documentation** | Update contributor guides, roadmap references, PR templates | Medium |
| **Scripts** | Update pre-push hooks (remove develop checks) | Low |
| **Prompts** | Update `.github/prompts/workstream-execution.prompt.md` branching instructions | Low |

**Total Effort**: ~3-5 hours for complete migration

---

### 4.2 Breaking Changes

**For Contributors**:

- ✅ **Workflow Change**: Feature branches now target `main` (not `develop`)
- ✅ **Branch Deletion**: `develop` branch will be permanently deleted
- ✅ **Version Bumps**: Must run `sync-versions` script before committing version changes

**For CI/CD**:

- ✅ **Workflow Triggers**: All workflows updated to run on `main` (not `develop`)
- ✅ **Release Process**: New release branch creation step

**For Maintainers**:

- ✅ **Release Workflow**: Create release branch before tagging
- ✅ **Version Management**: Use `.version` file as source of truth

**Backward Compatibility**:

- ✅ **Git History**: Preserved (squash merge retains all work)
- ✅ **Tags**: No changes to existing `v0.0.1`, `v0.0.2` tags
- ✅ **Dependencies**: No impact on external dependencies

**Risk Mitigation**:

- ✅ Create `docs/MIGRATION_GUIDE.md` for contributors
- ✅ Update all documentation before branch deletion
- ✅ Announce change in GitHub Discussion + README
- ✅ Test new workflow on feature branch before full rollout

---

### 4.3 GitHub Repository Configuration

**Required Changes**:

1. **Branch Protection Rules**:
   - Remove protection from `develop` (before deletion)
   - Update `main` protection rules:
     - Require pull request reviews: ✅ (keep)
     - Require status checks: ✅ (keep, update check names)
     - Require conversation resolution: ✅ (keep)

2. **Default Branch**:
   - Already `main` (no change needed)

3. **Branch Deletion**:
   - Delete `develop` after v0.0.3 release merged to `main`
   - Archive develop history in documentation (optional)

4. **Workflows to Update**:
   - `.github/workflows/ci.yml` (remove develop triggers)
   - `.github/workflows/code-scanning.yml` (remove develop triggers)
   - `.github/workflows/benchmarks.yml` (update branches list)
   - `.github/workflows/docs-lint.yml` (remove develop triggers)
   - Add `.github/workflows/version-check.yml` (new validation workflow)

---

## 5. Proposed Architecture

### 5.1 Centralized Version File

**Location**: `/.version` (root directory)

**Format** (TOML for flexibility):

```toml
# FerrisScript Version Configuration
# This is the SINGLE SOURCE OF TRUTH for version numbers
# Run `./scripts/sync-versions.sh` after editing to propagate changes

[version]
# Semantic version components
major = 0
minor = 0
patch = 3

# Combined version string (derived from above)
full = "0.0.3"

# Pre-release tag (optional: alpha, beta, rc.1, etc.)
tag = "alpha"

[components]
# Component-specific versions (if they diverge from main version)
cargo = "0.0.3"         # Rust crates
vscode = "0.0.3"        # VS Code extension
docs = "0.0.3"          # Documentation site

[metadata]
# Release metadata (for automation)
release_date = "2025-10-08"
codename = "Editor Experience Alpha"

[branches]
# Branch configuration (for automation scripts)
main = "main"
release_prefix = "release/"
```

**Alternative (Simple Text File)**:

```
0.0.3
```

**Recommendation**: Start with **simple text file** (just version number), evolve to TOML if component versioning needed.

---

### 5.2 Sync Script Architecture

**`scripts/sync-versions.ps1`** (PowerShell):

```powershell
#!/usr/bin/env pwsh
# FerrisScript Version Synchronization Script
# Propagates version from .version to all target files

param(
    [switch]$DryRun,
    [switch]$Validate
)

$VERSION_FILE = ".version"
$VERSION = (Get-Content $VERSION_FILE -Raw).Trim()

Write-Output "📦 Syncing version: $VERSION"

# Target files to update
$targets = @(
    @{
        Path = "Cargo.toml"
        Pattern = 'version = "[^"]+"'
        Replacement = "version = `"$VERSION`""
        Line = 10  # workspace.package.version line
    },
    @{
        Path = "package.json"
        Pattern = '"version": "[^"]+"'
        Replacement = "`"version`": `"$VERSION`""
    },
    @{
        Path = "extensions/vscode/package.json"
        Pattern = '"version": "[^"]+"'
        Replacement = "`"version`": `"$VERSION`""
    }
)

# Dry run mode: show what would change
if ($DryRun) {
    Write-Output "🔍 DRY RUN - No files will be modified"
    foreach ($target in $targets) {
        $content = Get-Content $target.Path -Raw
        if ($content -match $target.Pattern) {
            Write-Output "  ✓ $($target.Path): Would update to $VERSION"
        } else {
            Write-Output "  ⚠ $($target.Path): Pattern not found!"
        }
    }
    exit 0
}

# Validate mode: check if all versions match
if ($Validate) {
    $allMatch = $true
    foreach ($target in $targets) {
        $content = Get-Content $target.Path -Raw
        if ($content -match $target.Pattern) {
            $currentVersion = [regex]::Match($content, $target.Pattern).Value
            if ($currentVersion -notmatch $VERSION) {
                Write-Output "  ❌ $($target.Path): Version mismatch (expected $VERSION)"
                $allMatch = $false
            } else {
                Write-Output "  ✅ $($target.Path): Version matches"
            }
        } else {
            Write-Output "  ❌ $($target.Path): Pattern not found!"
            $allMatch = $false
        }
    }
    if (-not $allMatch) {
        exit 1
    }
    Write-Output "✅ All versions synchronized"
    exit 0
}

# Apply changes
foreach ($target in $targets) {
    $content = Get-Content $target.Path -Raw
    $newContent = $content -replace $target.Pattern, $target.Replacement
    Set-Content -Path $target.Path -Value $newContent -NoNewline
    Write-Output "  ✓ Updated $($target.Path)"
}

# Run npm install to update package-lock.json
if (Test-Path "package-lock.json") {
    Write-Output "📝 Updating package-lock.json..."
    npm install --package-lock-only 2>&1 | Out-Null
    Write-Output "  ✓ Updated package-lock.json"
}

Write-Output "✅ Version synchronized to $VERSION"
```

**`scripts/sync-versions.sh`** (Bash):

```bash
#!/usr/bin/env bash
# FerrisScript Version Synchronization Script

set -euo pipefail

VERSION_FILE=".version"
VERSION=$(cat "$VERSION_FILE" | tr -d '[:space:]')

echo "📦 Syncing version: $VERSION"

# Update Cargo.toml
sed -i.bak "s/^version = \".*\"$/version = \"$VERSION\"/" Cargo.toml && rm Cargo.toml.bak
echo "  ✓ Updated Cargo.toml"

# Update package.json files using jq (requires jq)
if command -v jq &> /dev/null; then
    jq ".version = \"$VERSION\"" package.json > package.json.tmp && mv package.json.tmp package.json
    echo "  ✓ Updated package.json"
    
    jq ".version = \"$VERSION\"" extensions/vscode/package.json > extensions/vscode/package.json.tmp && \
        mv extensions/vscode/package.json.tmp extensions/vscode/package.json
    echo "  ✓ Updated extensions/vscode/package.json"
else
    echo "  ⚠ jq not found, using sed (less reliable)"
    sed -i.bak "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/" package.json && rm package.json.bak
    sed -i.bak "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/" extensions/vscode/package.json && rm extensions/vscode/package.json.bak
fi

# Update package-lock.json
if [ -f "package-lock.json" ]; then
    echo "📝 Updating package-lock.json..."
    npm install --package-lock-only > /dev/null 2>&1
    echo "  ✓ Updated package-lock.json"
fi

echo "✅ Version synchronized to $VERSION"
```

---

### 5.3 CI Version Validation Workflow

**`.github/workflows/version-check.yml`**:

```yaml
name: Version Consistency Check

on:
  pull_request:
    paths:
      - '.version'
      - 'Cargo.toml'
      - 'package.json'
      - 'extensions/vscode/package.json'
  push:
    branches:
      - main
    paths:
      - '.version'
      - 'Cargo.toml'
      - 'package.json'
      - 'extensions/vscode/package.json'

jobs:
  validate-versions:
    name: Validate Version Synchronization
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Check version consistency
        run: |
          VERSION=$(cat .version | tr -d '[:space:]')
          echo "📦 Expected version: $VERSION"
          
          # Check Cargo.toml
          CARGO_VERSION=$(grep -E '^version = "[^"]+"' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
          if [ "$CARGO_VERSION" != "$VERSION" ]; then
            echo "❌ Cargo.toml version mismatch: $CARGO_VERSION != $VERSION"
            exit 1
          fi
          echo "✅ Cargo.toml matches"
          
          # Check package.json
          PKG_VERSION=$(jq -r '.version' package.json)
          if [ "$PKG_VERSION" != "$VERSION" ]; then
            echo "❌ package.json version mismatch: $PKG_VERSION != $VERSION"
            exit 1
          fi
          echo "✅ package.json matches"
          
          # Check VS Code extension
          VSCODE_VERSION=$(jq -r '.version' extensions/vscode/package.json)
          if [ "$VSCODE_VERSION" != "$VERSION" ]; then
            echo "❌ extensions/vscode/package.json version mismatch: $VSCODE_VERSION != $VERSION"
            exit 1
          fi
          echo "✅ extensions/vscode/package.json matches"
          
          echo "✅ All versions synchronized"
```

---

### 5.4 Pre-Commit Hook (Optional)

**`.git/hooks/pre-commit`** (installed via `scripts/install-git-hooks.ps1`):

```bash
#!/bin/bash
# Pre-commit hook: Auto-sync versions if .version changed

if git diff --cached --name-only | grep -q "^\.version$"; then
    echo "📦 Detected .version change, syncing to target files..."
    ./scripts/sync-versions.sh || exit 1
    git add Cargo.toml package.json extensions/vscode/package.json package-lock.json
    echo "✅ Version sync complete, files staged"
fi
```

**Benefits**:

- ✅ Automatic synchronization (no manual step)
- ✅ Prevents committing desynchronized versions
- ✅ Transparent to developer (happens on commit)

**Cons**:

- ⚠️ Requires `scripts/install-git-hooks.ps1` to be run during setup
- ⚠️ May surprise developers (silent file modification)

**Recommendation**: Make pre-commit hook **optional** (not enforced), document in `CONTRIBUTING.md`.

---

### 5.5 Tag Strategy

**Current**: Tags created manually on `main` after release (`v0.0.1`, `v0.0.2`).

**Proposed**: Tags created on release branches (or `main` after release branch merge).

**Semantic Versioning Tags**:

- **Release**: `v0.0.3` (production-ready)
- **Pre-release**: `v0.0.3-alpha.1`, `v0.0.3-beta.2`, `v0.0.3-rc.1`
- **Component-specific** (future): `vscode-v0.0.3`, `cargo-v0.0.3`

**Tag Creation Workflow**:

```bash
# After release branch stabilization
git checkout release/v0.0.3
git tag v0.0.3
git push origin v0.0.3

# Trigger GitHub Release creation (automated via CI)
```

**GitHub Releases**:

- Automatically created from tags (CI workflow)
- Include CHANGELOG.md excerpt for version
- Attach build artifacts (VS Code .vsix, cargo binaries)

---

## 6. Migration Strategy

### 6.1 Phase 1: Version Management (No Branching Changes)

**Goal**: Implement centralized version file without changing branching strategy.

**Steps**:

1. ✅ **Create `.version` file**: Add `0.0.3` as content
2. ✅ **Create sync scripts**: `scripts/sync-versions.{ps1,sh}` (with validation mode)
3. ✅ **Update documentation**: Add section to `RELEASING.md` explaining new process
4. ✅ **Add CI validation**: `.github/workflows/version-check.yml`
5. ✅ **Test on feature branch**: Create `feature/centralized-version-management`, test workflow
6. ✅ **Update `.gitignore`**: Ensure `.version` is tracked (not ignored)
7. ✅ **PR to develop**: Merge feature branch, validate CI passes
8. ✅ **Document in CHANGELOG**: Note process improvement in v0.0.4 release notes

**Estimated Effort**: 2-3 hours

**Validation**:

- ✅ Run `./scripts/sync-versions.ps1 -Validate` (should pass)
- ✅ Edit `.version` → run sync script → verify all files updated
- ✅ Create PR with version change → CI should validate consistency

**Rollback Plan**: Revert `.version` file + scripts, continue manual version bumps.

---

### 6.2 Phase 2: Branching Strategy Migration

**Goal**: Transition from Git Flow (develop + main) to GitHub Flow + Release Branches.

**Prerequisites**:

- ✅ Phase 1 complete (centralized version management working)
- ✅ v0.0.3 released to `main` (develop branch ready to delete)

**Steps**:

1. ✅ **Squash merge develop → main**:

   ```bash
   git checkout main
   git merge --squash develop
   git commit -m "Release v0.0.3: Editor Experience Alpha"
   git push origin main
   ```

2. ✅ **Tag v0.0.3**:

   ```bash
   git tag v0.0.3
   git push origin v0.0.3
   ```

3. ✅ **Update all workflows**: Remove `develop` from branch triggers

   Files to update:
   - `.github/workflows/ci.yml`
   - `.github/workflows/code-scanning.yml`
   - `.github/workflows/benchmarks.yml`
   - `.github/workflows/docs-lint.yml`

   Change:

   ```yaml
   # Before
   on:
     push:
       branches: [main, develop]
   
   # After
   on:
     push:
       branches: [main]
       tags: ['v*']  # Also trigger on version tags
   ```

4. ✅ **Update documentation**:

   Files to update:
   - `CONTRIBUTING.md` (branching strategy section)
   - `RELEASING.md` (release process)
   - `.github/prompts/workstream-execution.prompt.md`
   - `docs/planning/v0.0.3/v0.0.3-roadmap.md` (archive section)
   - `docs/LEARNINGS.md` (add migration notes)

5. ✅ **Create migration guide**: `docs/MIGRATION_GUIDE.md` for contributors

6. ✅ **Announce change**: GitHub Discussion, README notice

7. ✅ **Delete develop branch**:

   ```bash
   git push origin --delete develop
   git branch -d develop
   ```

8. ✅ **Update GitHub settings**: Remove branch protection rules for `develop`

9. ✅ **Test new workflow**: Create test feature branch, PR to `main`, verify CI

**Estimated Effort**: 3-4 hours

**Validation**:

- ✅ All CI workflows run on `main` (not `develop`)
- ✅ Feature branches can PR directly to `main`
- ✅ No broken links in documentation
- ✅ Contributors can follow new workflow from `CONTRIBUTING.md`

**Rollback Plan**: Recreate `develop` from `main`, revert workflow changes.

---

### 6.3 Phase 3: Release Branch Workflow (First Use)

**Goal**: Practice new release branch workflow for v0.0.4.

**Steps** (for v0.0.4 release):

1. ✅ **Feature development**: All features merge to `main` as usual
2. ✅ **Create release branch**: When v0.0.4 feature-complete

   ```bash
   git checkout main
   git pull origin main
   git checkout -b release/v0.0.4
   git push origin release/v0.0.4
   ```

3. ✅ **Stabilization**: Only bugfixes merge to `release/v0.0.4`
4. ✅ **Version bump**: Update `.version` to `0.0.4`, run sync script
5. ✅ **Update CHANGELOG**: Finalize v0.0.4 changelog entry
6. ✅ **Tag release**:

   ```bash
   git checkout release/v0.0.4
   git tag v0.0.4
   git push origin v0.0.4
   ```

7. ✅ **GitHub Release**: CI creates GitHub Release from tag
8. ✅ **Merge hotfixes back to main** (if any):

   ```bash
   git checkout main
   git cherry-pick <hotfix-commit>
   ```

9. ✅ **Delete release branch** (optional, after v0.0.5 released):

   ```bash
   git push origin --delete release/v0.0.4
   ```

**Estimated Effort**: Ongoing (part of normal release process)

**Validation**:

- ✅ Release branch only contains bugfixes (no new features)
- ✅ Tag created from release branch (not `main`)
- ✅ Hotfixes propagated to `main`

---

## 7. Implementation Roadmap

### Milestone 1: Centralized Version Management (v0.0.4)

**Timeline**: Post-v0.0.3 release (immediate)

**Deliverables**:

1. `.version` file (root directory)
2. `scripts/sync-versions.{ps1,sh}` (with --validate flag)
3. `.github/workflows/version-check.yml` (CI validation)
4. Updated `RELEASING.md` (new version bump process)
5. Updated `scripts/README.md` (document sync-versions script)
6. Optional: Pre-commit hook for auto-sync

**Success Criteria**:

- ✅ CI fails if versions are desynchronized
- ✅ Version bumps take < 1 minute (update `.version`, run script, commit)
- ✅ No version mismatches in v0.0.4 release

**Estimated Time**: 2-3 hours

---

### Milestone 2: Branching Strategy Migration (Post-v0.0.3)

**Timeline**: After v0.0.3 merged to `main` and tagged

**Deliverables**:

1. `develop` branch deleted
2. All workflows updated (remove `develop` triggers)
3. Documentation updated (CONTRIBUTING.md, RELEASING.md, prompts)
4. `docs/MIGRATION_GUIDE.md` created
5. GitHub Discussion announcement
6. README notice (temporary, for 1-2 releases)

**Success Criteria**:

- ✅ All CI workflows run on `main` only
- ✅ Feature branches target `main` (not `develop`)
- ✅ Contributors can follow new workflow from docs
- ✅ No broken CI pipelines

**Estimated Time**: 3-4 hours

---

### Milestone 3: Release Branch Workflow (v0.0.4 Release)

**Timeline**: When v0.0.4 is feature-complete (future)

**Deliverables**:

1. `release/v0.0.4` branch created from `main`
2. v0.0.4 tag created from `release/v0.0.4`
3. GitHub Release automated from tag
4. Hotfixes (if any) cherry-picked to `main`
5. Release branch maintained until v0.0.5 or deleted

**Success Criteria**:

- ✅ Release branch only contains stabilization commits
- ✅ Tag created from release branch
- ✅ Main continues development without blocking

**Estimated Time**: Part of normal release (no additional overhead)

---

### Milestone 4: Automation Enhancements (v0.1.0+)

**Timeline**: Future (after v0.0.X stabilizes)

**Deliverables**:

1. `cargo-release` integration (automated Cargo version bumps)
2. `semantic-release` integration (automated CHANGELOG generation)
3. GitHub Actions for automated release PR creation
4. Component-specific versioning (if cargo/vscode/docs diverge)

**Success Criteria**:

- ✅ Version bumps fully automated (based on conventional commits)
- ✅ CHANGELOG.md generated from commit messages
- ✅ GitHub Releases created automatically from tags

**Estimated Time**: 1-2 days (research + implementation)

---

## 8. Recommendations

### 8.1 Immediate Actions (v0.0.4 Cycle)

**Priority 1: Centralized Version Management**

- ✅ **Implement `.version` + sync scripts** (Milestone 1)
- ✅ **Add CI validation** (prevent future desync)
- ✅ **Update release process docs**

**Rationale**: Low risk, high value, solves immediate pain point (version desync).

---

**Priority 2: Branching Strategy Migration**

- ✅ **Finalize v0.0.3 release to main** (squash merge from develop)
- ✅ **Delete develop branch** (clean break)
- ✅ **Update all documentation** (contributor guides, workflows)

**Rationale**: Eliminate long-term maintenance burden, simplify contributor workflow.

---

### 8.2 Medium-Term Enhancements (v0.0.5-0.1.0)

**Priority 3: Release Branch Workflow**

- ✅ **Practice release branch creation for v0.0.4**
- ✅ **Document lessons learned** (update RELEASING.md)
- ✅ **Evaluate if release branches add value** (may not need them for v0.0.X)

**Rationale**: Optional enhancement, test before committing long-term.

---

**Priority 4: Automation Tools**

- ⏸️ **Defer to v0.1.0**: `cargo-release`, `semantic-release`
- ⏸️ **Focus on features first**: v0.0.X is alpha, prioritize language features over tooling
- ✅ **Revisit after v1.0**: Automated versioning makes sense for stable releases

**Rationale**: Avoid over-engineering during alpha phase.

---

### 8.3 Decision Matrix

| Approach | Complexity | Value | Risk | Recommendation |
|----------|------------|-------|------|----------------|
| **Centralized .version file** | Low | High | Low | ✅ **Implement Now** (v0.0.4) |
| **Sync scripts** | Low | High | Low | ✅ **Implement Now** (v0.0.4) |
| **CI version validation** | Low | High | Low | ✅ **Implement Now** (v0.0.4) |
| **GitHub Flow (no develop)** | Low | High | Medium | ✅ **Implement Post-v0.0.3** |
| **Release branches** | Medium | Medium | Low | ⚙️ **Test in v0.0.4**, evaluate |
| **cargo-release automation** | High | Medium | Medium | ⏸️ **Defer to v0.1.0+** |
| **semantic-release** | High | Medium | High | ⏸️ **Defer to v1.0+** |

---

### 8.4 Final Recommendations

**Recommended Approach** (Balanced):

1. ✅ **Implement centralized version management** (`.version` + scripts) in v0.0.4 cycle
2. ✅ **Migrate to GitHub Flow** (delete `develop` branch) after v0.0.3 release
3. ⚙️ **Experiment with release branches** in v0.0.4 (evaluate if needed)
4. ⏸️ **Defer automation tools** to post-v1.0 (focus on language features first)

**Key Principles**:

- 🎯 **Simplicity First**: Solve version desync with minimal tooling
- 📏 **Incremental Migration**: Phase 1 (versioning) → Phase 2 (branching) → Phase 3 (automation)
- 🛡️ **Risk Mitigation**: Test new workflows on feature branches before full rollout
- 📚 **Documentation**: Update contributor guides before deleting `develop`

---

## 9. Appendices

### Appendix A: Example Migration Checklist

**Post-v0.0.3 Release Checklist**:

- [ ] Squash merge `develop` → `main`
- [ ] Tag `v0.0.3` on `main`
- [ ] Create `.version` file with `0.0.3`
- [ ] Create `scripts/sync-versions.{ps1,sh}`
- [ ] Add `.github/workflows/version-check.yml`
- [ ] Update `RELEASING.md` (new version bump process)
- [ ] Update `CONTRIBUTING.md` (remove develop references)
- [ ] Update `.github/prompts/workstream-execution.prompt.md`
- [ ] Remove `develop` from all workflow triggers
- [ ] Create `docs/MIGRATION_GUIDE.md`
- [ ] Announce change in GitHub Discussion
- [ ] Delete `develop` branch (locally + remotely)
- [ ] Update GitHub repository settings (remove develop protection)
- [ ] Test new workflow (create test feature branch → PR to main)

---

### Appendix B: Version Sync Script Usage

**Basic Usage**:

```bash
# Update .version file
echo "0.0.4" > .version

# Sync to all files (PowerShell)
./scripts/sync-versions.ps1

# Sync to all files (Bash)
./scripts/sync-versions.sh

# Validate consistency (CI mode)
./scripts/sync-versions.ps1 -Validate

# Dry run (preview changes)
./scripts/sync-versions.ps1 -DryRun
```

**Integration with Release Process**:

```bash
# Step 1: Bump version
echo "0.0.4" > .version

# Step 2: Sync files
./scripts/sync-versions.sh

# Step 3: Update CHANGELOG
vim CHANGELOG.md

# Step 4: Commit
git add .version Cargo.toml package.json extensions/vscode/package.json CHANGELOG.md
git commit -m "chore: Bump version to 0.0.4"

# Step 5: Create release branch
git checkout -b release/v0.0.4
git push origin release/v0.0.4

# Step 6: Tag release
git tag v0.0.4
git push origin v0.0.4
```

---

### Appendix C: Alternative Workflows Considered

**Not Recommended**:

1. ❌ **Keep develop + add release branches**: Too many long-lived branches
2. ❌ **Trunk-based development**: Requires feature flags, too complex for alpha
3. ❌ **Monorepo with separate versioning**: Cargo/npm/docs at different versions (confusing)
4. ❌ **Automated commit-based versioning**: Requires strict conventional commits (too rigid for alpha)

---

## 10. Next Steps

### For User Review

**Questions for Decision**:

1. ✅ **Version Management**: Approve `.version` + sync scripts approach?
2. ✅ **Branching Strategy**: Approve GitHub Flow (delete `develop` after v0.0.3)?
3. ⚙️ **Release Branches**: Test in v0.0.4 or skip for now?
4. ⏸️ **Automation Tools**: Defer to v0.1.0+ or investigate now?

**Approval Needed**:

- [ ] Conceptual approval of centralized version management
- [ ] Approval to delete `develop` branch after v0.0.3 release
- [ ] Approval to update contributor documentation

---

### Implementation Timeline

**If Approved**:

- **Week 1** (v0.0.4 Cycle): Implement centralized version management
- **Week 2** (Post-v0.0.3 Release): Migrate to GitHub Flow, delete `develop`
- **Week 3+** (Ongoing): Practice new workflow, iterate based on feedback

**Estimated Total Effort**: 1-2 days (spread across 2-3 weeks)

---

## 11. Conclusion

This research demonstrates that **centralized version management + GitHub Flow** provides the optimal balance of simplicity, flexibility, and automation for FerrisScript's current stage (v0.0.X alpha releases).

**Key Takeaways**:

1. ✅ **Version desync is solvable** with simple `.version` file + sync scripts
2. ✅ **Branching strategy can be simplified** by eliminating long-lived `develop` branch
3. ✅ **No need for complex automation** at alpha stage (defer to v1.0+)
4. ✅ **Low-risk migration** with phased rollout and rollback plans

**Recommended Action**: **Proceed with Phase 1 (Version Management)** immediately, evaluate branching migration after v0.0.3 release.

---

**Document Status**: Ready for review and decision  
**Next Revision**: After user feedback and approval  
**Contact**: Open GitHub Discussion for questions/feedback
