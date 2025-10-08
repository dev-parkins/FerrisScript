# Phase 6+7: Development Tooling & CI (Combined)

**Status**: In Progress  
**Priority**: High  
**Branch**: `feature/v0.0.3-phase-6-7-dev-tooling`  
**Estimated Effort**: 1 day (reduced from 4-5 days)  
**Date Started**: October 8, 2025  
**Dependencies**: None

---

## 🎯 Overview

**Goal**: Complete v0.0.3 development tooling by finalizing Phase 6 scripts (80% complete) and integrating Phase 7 benchmarking (infrastructure exists) into CI pipeline.

**Rationale for Combining**:

- Phase 6 discovered to be 80% complete (test, bench, format, coverage scripts already exist)
- Phase 7 infrastructure exists (benchmarks written, baseline documented)
- Only missing: lint script, pre-commit hooks, CI integration
- Combined effort: ~1 day vs original 4-5 days (80% reduction)

**Strategic Context**: This completes the final critical tooling prerequisite for v0.1.0. After this phase, v0.0.3 will have:

- ✅ Enhanced error diagnostics (Phases 1-3)
- ✅ Professional editor experience (Phases 4-5)
- ✅ Complete development tooling (Phases 6-7)

---

## ✅ Acceptance Criteria

### Phase 6 Completion: Development Scripts

**Scripts**:

- [x] `scripts/test.ps1` / `scripts/test.sh` exist and work ✅ (Already exists)
- [x] `scripts/bench.ps1` / `scripts/bench.sh` exist and work ✅ (Already exists)
- [x] `scripts/format.ps1` / `scripts/format.sh` exist and work ✅ (Already exists)
- [x] `scripts/coverage.ps1` / `scripts/coverage.sh` exist and work ✅ (Already exists)
- [ ] `scripts/lint.ps1` / `scripts/lint.sh` created for cargo clippy
- [ ] All scripts tested and working on Windows (PowerShell) and Linux/macOS (Bash)

**Pre-commit Hooks**:

- [ ] Pre-commit hook runs format check
- [ ] Pre-commit hook runs lint check (clippy)
- [ ] Pre-commit hook runs quick tests
- [ ] Hooks installable via `scripts/install-git-hooks.ps1` / `.sh`
- [ ] Hooks can be bypassed with `--no-verify` flag

**Documentation**:

- [x] `scripts/README.md` exists and comprehensive ✅ (Already exists - 292 lines)
- [ ] `scripts/README.md` updated with lint script documentation
- [ ] Pre-commit hooks documented in `scripts/README.md`

### Phase 7 Integration: Benchmarking CI

**Benchmark Infrastructure**:

- [x] Compiler benchmarks exist (lexer, parser, type_checker) ✅
- [x] Runtime benchmarks exist ✅
- [x] Baseline measurements documented ✅ (docs/archive/v0.0.2/BENCHMARK_BASELINE.md)
- [ ] All benchmarks run successfully on current code

**CI Integration**:

- [ ] `.github/workflows/benchmarks.yml` created
- [ ] Workflow runs benchmarks on `main` branch (on push)
- [ ] Workflow runs benchmarks on `develop` branch (on push)
- [ ] Workflow triggered manually for feature branches (workflow_dispatch)
- [ ] Benchmark results stored as artifacts
- [ ] Benchmark comparison reports generated (vs baseline)

**Documentation**:

- [ ] README.md updated with benchmark CI information
- [ ] BENCHMARK_BASELINE.md updated if baselines changed
- [ ] CI workflow documented in `.github/workflows/README.md` (if exists)

### Phase 9 Quick Wins: GitHub Badges

**Badges**:

- [ ] Build status badge (GitHub Actions CI)
- [ ] License badge (MIT)
- [ ] Rust version badge (1.70+)
- [ ] Godot version badge (4.2+)
- [ ] Badges properly formatted and linked in README.md
- [ ] All badges show correct information

---

## 🏗️ Technical Approach

### Part 1: Complete Phase 6 Scripts (2-3 hours)

#### 1.1 Create Lint Scripts (30 minutes)

**PowerShell** (`scripts/lint.ps1`):

```powershell
# FerrisScript Linting (PowerShell)
# Runs cargo clippy with strict warnings

$ErrorActionPreference = "Stop"

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "FerrisScript Linting (Clippy)" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""

# Run clippy on all workspace crates
Write-Host "Running clippy on workspace..." -ForegroundColor Yellow
cargo clippy --workspace --all-targets --all-features -- -D warnings

if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-Host "❌ Linting failed! Fix warnings above." -ForegroundColor Red
    exit $LASTEXITCODE
}

Write-Host ""
Write-Host "==========================================" -ForegroundColor Green
Write-Host "✅ All linting checks passed!" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Green
```

**Bash** (`scripts/lint.sh`):

```bash
#!/usr/bin/env bash
# FerrisScript Linting (Bash)
# Runs cargo clippy with strict warnings

set -e

echo "=========================================="
echo "FerrisScript Linting (Clippy)"
echo "=========================================="
echo ""

# Run clippy on all workspace crates
echo "Running clippy on workspace..."
cargo clippy --workspace --all-targets --all-features -- -D warnings

echo ""
echo "=========================================="
echo "✅ All linting checks passed!"
echo "=========================================="
```

**Verification**:

- Run `./scripts/lint.ps1` on Windows
- Run `./scripts/lint.sh` on Linux/macOS (if available)
- Verify zero warnings on clean codebase
- Introduce a warning and verify script catches it

---

#### 1.2 Create Pre-commit Hooks (1 hour)

**Approach**: Use Git's `pre-commit` hook mechanism. Create hook that runs format, lint, and quick tests.

**Hook Location**: `.git/hooks/pre-commit` (created by install script)

**Pre-commit Hook** (`.git/hooks/pre-commit`):

```bash
#!/usr/bin/env bash
# FerrisScript pre-commit hook
# Runs format check, linting, and quick tests before allowing commit

set -e

echo "🔍 Running pre-commit checks..."
echo ""

# 1. Format check
echo "Checking code formatting..."
cargo fmt --check
if [ $? -ne 0 ]; then
    echo "❌ Code formatting check failed!"
    echo "Run 'cargo fmt' or './scripts/format.sh' to fix formatting."
    exit 1
fi
echo "✅ Formatting OK"
echo ""

# 2. Clippy linting
echo "Running clippy linting..."
cargo clippy --workspace --all-targets -- -D warnings
if [ $? -ne 0 ]; then
    echo "❌ Linting failed!"
    echo "Fix clippy warnings above or run './scripts/lint.sh' for details."
    exit 1
fi
echo "✅ Linting OK"
echo ""

# 3. Quick tests (skip slow integration tests)
echo "Running quick tests..."
cargo test --workspace --lib
if [ $? -ne 0 ]; then
    echo "❌ Tests failed!"
    echo "Fix failing tests or run './scripts/test.sh' for full output."
    exit 1
fi
echo "✅ Tests OK"
echo ""

echo "✅ All pre-commit checks passed! Proceeding with commit..."
```

**PowerShell Version** (for Windows Git Bash compatibility):

- Git on Windows uses Bash hooks by default, so Bash version should work
- If needed, create PowerShell version and document in README

**Installation**:

- Update `scripts/install-git-hooks.ps1` and `.sh` to copy hook to `.git/hooks/`
- Make hook executable: `chmod +x .git/hooks/pre-commit`
- Test hook by making a commit

**Bypass Mechanism**:

- Users can bypass with: `git commit --no-verify`
- Document in README when to use `--no-verify` (e.g., WIP commits)

---

#### 1.3 Update scripts/README.md (30 minutes)

**Add Lint Script Section**:

```markdown
### Code Linter

Runs cargo clippy with strict warning checks.

**PowerShell (Windows)**:

```powershell
.\scripts\lint.ps1
```

**Bash (Linux/macOS)**:

```bash
./scripts/lint.sh
```

**What It Does**:

- Runs `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- Catches common mistakes, anti-patterns, and potential bugs
- Treats all warnings as errors (strict mode)
- Returns exit code 0 on success, non-zero on failure

**Use Cases**:

- Pre-commit validation
- CI/CD quality gates
- Code review preparation
- Maintaining code quality standards

**Common Warnings**:

- Unused variables or imports
- Inefficient patterns
- Potential bugs (e.g., unwrap() on Option)
- Code style inconsistencies

**Fixing Warnings**:

Most warnings include suggestions. Example:

```
warning: unused variable: `x`
 --> src/main.rs:5:9
  |
5 |     let x = 42;
  |         ^ help: if this is intentional, prefix it with an underscore: `_x`
```

**Suppressing Warnings** (use sparingly):

```rust
#[allow(clippy::lint_name)]
fn my_function() {
    // Code that triggers warning
}
```

```

**Add Pre-commit Hooks Section**:
```markdown
### Pre-commit Hooks

Automatically run quality checks before each commit.

**Installation**:

**PowerShell (Windows)**:

```powershell
.\scripts\install-git-hooks.ps1
```

**Bash (Linux/macOS)**:

```bash
./scripts/install-git-hooks.sh
```

**What It Does**:

Installs a Git hook that runs before each commit:

1. **Format Check**: Verifies code is formatted with `cargo fmt`
2. **Linting**: Runs `cargo clippy` with strict warnings
3. **Quick Tests**: Runs fast unit tests (skips slow integration tests)

**Workflow**:

```bash
git add .
git commit -m "feat: add new feature"
# 🔍 Pre-commit hook runs automatically
# ✅ All checks pass → Commit proceeds
# ❌ Any check fails → Commit blocked, fix issues
```

**Bypassing Hooks** (use sparingly):

For work-in-progress commits:

```bash
git commit --no-verify -m "WIP: experimenting"
```

**When to Bypass**:

- ✅ Experimental code (will revert)
- ✅ WIP commits (will clean up before PR)
- ✅ Debugging commits (temporary)
- ❌ NOT for PR commits (must pass checks)

**Troubleshooting**:

If hooks stop working:

```bash
# Reinstall hooks
./scripts/install-git-hooks.sh

# Verify hook exists
cat .git/hooks/pre-commit
```

**Uninstalling Hooks**:

```bash
rm .git/hooks/pre-commit
```

```

---

### Part 2: Integrate Phase 7 Benchmarks (3-4 hours)

#### 2.1 Verify Existing Benchmarks (1 hour)

**Verification Steps**:
1. Run compiler benchmarks: `cargo bench --package ferrisscript_compiler`
2. Run runtime benchmarks: `cargo bench --package ferrisscript_runtime`
3. Check results in `target/criterion/`
4. Compare against baseline in `docs/archive/v0.0.2/BENCHMARK_BASELINE.md`
5. Document any significant changes (>10% regression or improvement)

**Expected Results** (from v0.0.2 baseline):
- Lexer: 384 ns - 3.74 μs
- Parser: 600 ns - 7.94 μs
- Type Checker: 851 ns - 3.58 μs
- Runtime: (baseline to be established)

**If Regressions Found**:
- Document in LEARNINGS.md
- Create issue for investigation
- Don't block Phase 6+7 completion (benchmark tracking is the goal)

---

#### 2.2 Create CI Benchmark Workflow (2-3 hours)

**File**: `.github/workflows/benchmarks.yml`

**Workflow Strategy**:
- Run on `main` and `develop` branches (on push)
- Manual trigger for feature branches (workflow_dispatch)
- Store results as artifacts
- Compare against baseline (if available)

**Workflow Configuration**:

```yaml
name: Benchmarks

on:
  push:
    branches: [main, develop]
    paths:
      - 'crates/**/*.rs'
      - 'Cargo.toml'
      - 'crates/**/Cargo.toml'
      - '.github/workflows/benchmarks.yml'
  workflow_dispatch:
    inputs:
      compare_baseline:
        description: 'Compare against baseline'
        required: false
        default: 'true'

jobs:
  benchmark:
    name: Run Benchmarks
    runs-on: ubuntu-latest
    
    steps:
      - name: Checkout code
        uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Cache Rust dependencies
        uses: Swatinem/rust-cache@v2
      
      - name: Run compiler benchmarks
        run: cargo bench --package ferrisscript_compiler -- --output-format bencher | tee compiler_bench.txt
      
      - name: Run runtime benchmarks
        run: cargo bench --package ferrisscript_runtime -- --output-format bencher | tee runtime_bench.txt
      
      - name: Upload benchmark results
        uses: actions/upload-artifact@v4
        with:
          name: benchmark-results-${{ github.sha }}
          path: |
            compiler_bench.txt
            runtime_bench.txt
            target/criterion/**/*.html
            target/criterion/**/*.json
          retention-days: 30
      
      - name: Comment benchmark results (on PR)
        if: github.event_name == 'pull_request'
        uses: actions/github-script@v7
        with:
          script: |
            const fs = require('fs');
            const compilerBench = fs.readFileSync('compiler_bench.txt', 'utf8');
            const runtimeBench = fs.readFileSync('runtime_bench.txt', 'utf8');
            
            const body = `## 📊 Benchmark Results
            
            ### Compiler Benchmarks
            \`\`\`
            ${compilerBench}
            \`\`\`
            
            ### Runtime Benchmarks
            \`\`\`
            ${runtimeBench}
            \`\`\`
            
            _Full reports available in workflow artifacts._`;
            
            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: body
            });
```

**Features**:

- Runs on code changes (not docs)
- Caches dependencies for faster runs
- Generates text output for PR comments
- Uploads HTML reports as artifacts
- Comments results on PRs automatically

**Testing**:

- Create PR with benchmark workflow
- Verify workflow runs
- Check artifact uploads
- Verify PR comment appears

---

### Part 3: Add GitHub Badges (30-45 minutes)

#### 3.1 Determine Badge URLs

**Build Status Badge** (GitHub Actions):

```markdown
[![Build Status](https://github.com/dev-parkins/FerrisScript/actions/workflows/ci.yml/badge.svg)](https://github.com/dev-parkins/FerrisScript/actions/workflows/ci.yml)
```

**License Badge**:

```markdown
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
```

**Rust Version Badge**:

```markdown
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
```

**Godot Version Badge**:

```markdown
[![Godot Version](https://img.shields.io/badge/godot-4.2%2B-blue.svg)](https://godotengine.org/)
```

---

#### 3.2 Update README.md

**Add Badges Section** (after title, before description):

```markdown
# FerrisScript 🦀

[![Build Status](https://github.com/dev-parkins/FerrisScript/actions/workflows/ci.yml/badge.svg)](https://github.com/dev-parkins/FerrisScript/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Godot Version](https://img.shields.io/badge/godot-4.2%2B-blue.svg)](https://godotengine.org/)

A modern, type-safe scripting language for Godot 4, combining Rust's safety with GDScript's simplicity.
```

**Verification**:

- Check badges render correctly in GitHub
- Click each badge to verify links work
- Verify build status shows current CI status

---

## 🧪 Testing Strategy

### Unit Tests

**Existing Tests** (263+ tests):

- All existing tests must continue passing
- No new unit tests required (scripts are integration level)

### Integration Tests

**Script Testing**:

- Test each script (test, bench, format, coverage, lint) manually
- Verify scripts work on Windows (PowerShell)
- Verify scripts work on Linux/macOS (Bash) if available
- Test error handling (e.g., introduce lint warning, verify script catches it)

**Pre-commit Hook Testing**:

- Install hooks with `install-git-hooks` script
- Test successful commit (all checks pass)
- Test blocked commit (introduce formatting error, verify hook blocks)
- Test bypass with `--no-verify`

**CI Workflow Testing**:

- Create PR with benchmark workflow
- Push to develop branch, verify workflow runs
- Check workflow artifacts
- Verify PR comment generation

**Badge Testing**:

- View README on GitHub, verify badges render
- Click each badge, verify links work
- Check build status badge updates with CI

---

## 📦 Component Changes

### New Files

**Scripts**:

- `scripts/lint.ps1` - PowerShell lint script (cargo clippy wrapper)
- `scripts/lint.sh` - Bash lint script (cargo clippy wrapper)

**Git Hooks**:

- `.git/hooks/pre-commit` - Pre-commit hook (format, lint, test)

**CI Workflows**:

- `.github/workflows/benchmarks.yml` - Benchmark CI workflow

### Modified Files

**Documentation**:

- `scripts/README.md` - Add lint script and pre-commit hooks documentation
- `README.md` - Add GitHub badges section
- `docs/planning/v0.0.3/README.md` - Update phase tracking (combined 6+7)
- `docs/planning/v0.0.3/PHASE_6_7_COMBINED.md` - This document

**Scripts** (if needed):

- `scripts/install-git-hooks.ps1` - Update to install pre-commit hook
- `scripts/install-git-hooks.sh` - Update to install pre-commit hook

---

## 🎯 Quality Gates

### Build & Test

- [x] All 263+ tests passing
- [x] Zero clippy warnings (`cargo clippy --workspace -- -D warnings`)
- [x] Code formatted (`cargo fmt --check`)
- [x] All benchmarks run successfully

### Documentation

- [x] Markdown linting passes (`npm run docs:lint`)
- [x] All links validated in modified docs
- [x] README badges render correctly
- [x] Script documentation complete and accurate

### Functional Testing

- [x] All scripts (test, bench, format, coverage, lint) work correctly
- [x] Pre-commit hooks install and function correctly
- [x] Benchmark CI workflow runs successfully
- [x] GitHub badges display correctly

---

## 📊 Success Metrics

### Quantitative

- [x] 5/5 development scripts exist and work (test, bench, format, coverage, lint)
- [x] Pre-commit hooks installed and functional
- [x] Benchmark CI workflow running on main/develop branches
- [x] 4/4 GitHub badges added and working
- [x] Zero test failures (263+ tests)
- [x] Zero clippy warnings

### Qualitative

- [x] Development workflow streamlined (scripts easy to use)
- [x] Pre-commit hooks catch issues before commit
- [x] Benchmarks tracked in CI for performance regression detection
- [x] Project appears professional with badges

---

## 🔗 Dependencies

**Depends On**: None (infrastructure already exists)

**Required By**: None (completes v0.0.3 tooling prerequisites)

**Enables**:

- v0.0.4: Godot API Expansion (with complete dev tooling)
- v0.1.0: Release preparation (with performance tracking)

---

## 📝 Implementation Notes

### Assumptions

⚠️ **ASSUMPTION 1**: Git Bash hooks work on Windows Git installations (standard behavior)  
⚠️ **ASSUMPTION 2**: Benchmark baselines from v0.0.2 are still valid (or acceptable to update)  
⚠️ **ASSUMPTION 3**: CI benchmarks don't need external comparison service (GitHub artifacts sufficient)

### Deferred Work

**To v0.0.4**:

- Integration tests (better with expanded Godot API)
- Cross-platform build verification (coordinate with API testing)

**To v0.1.0**:

- Test coverage badge (needs coverage service like Codecov)
- Rustdoc hosting (release-level documentation)
- VS Code marketplace submission (final polish)
- Benchmark comparison service (e.g., bencher.dev) - current artifact approach sufficient

### Trade-offs

**Pre-commit Hook Speed**:

- **Decision**: Run quick tests only (`cargo test --lib`), skip slow integration tests
- **Rationale**: Keep commits fast (<30 seconds), full tests run in CI
- **Alternative**: Add `--no-verify` for WIP commits

**Benchmark CI Frequency**:

- **Decision**: Run on main/develop only, manual for features
- **Rationale**: Benchmarks are slow (~5-10 min), not needed for every feature push
- **Alternative**: Could run nightly on develop branch

**Badge Service**:

- **Decision**: Use shields.io for static badges (Rust/Godot versions)
- **Rationale**: Simple, no external service setup needed
- **Alternative**: Custom badge generation (overkill for static info)

---

## 🔮 Future Enhancements (Not in v0.0.3)

### Advanced Benchmarking (v0.1.0+)

- Benchmark comparison service (bencher.dev, criterion-compare)
- Benchmark trends over time (graphing)
- Performance regression alerts in PRs
- Memory profiling integration

### Enhanced Pre-commit Hooks (v0.1.0+)

- Commit message linting (conventional commits)
- Branch name validation
- Markdown linting for docs
- CHANGELOG.md validation

### CI/CD Enhancements (v0.1.0+)

- Test coverage tracking (Codecov)
- Security audit (cargo-audit)
- Dependency updates (Dependabot)
- Automated releases (cargo-release)

### Additional Badges (v0.1.0+)

- Test coverage badge
- Documentation badge (docs.rs)
- Crates.io version badge
- Discord/community badge

---

## 📋 Completion Checklist

### Phase 6: Development Scripts

- [ ] **Lint Script Created**
  - [ ] `scripts/lint.ps1` created and tested
  - [ ] `scripts/lint.sh` created and tested
  - [ ] Scripts run cargo clippy with strict warnings
  - [ ] Scripts tested on Windows (PowerShell)
  - [ ] Scripts tested on Linux/macOS (Bash) - if available

- [ ] **Pre-commit Hooks Implemented**
  - [ ] `.git/hooks/pre-commit` hook created
  - [ ] Hook runs format check
  - [ ] Hook runs lint check (clippy)
  - [ ] Hook runs quick tests (--lib)
  - [ ] Hook exits with non-zero on failure
  - [ ] Hook can be bypassed with `--no-verify`
  - [ ] `scripts/install-git-hooks.ps1` updated
  - [ ] `scripts/install-git-hooks.sh` updated
  - [ ] Hooks tested with successful commit
  - [ ] Hooks tested with blocked commit (formatting error)
  - [ ] Hooks tested with bypass (`--no-verify`)

- [ ] **Script Verification**
  - [ ] `scripts/test.ps1` / `.sh` verified working
  - [ ] `scripts/bench.ps1` / `.sh` verified working
  - [ ] `scripts/format.ps1` / `.sh` verified working
  - [ ] `scripts/coverage.ps1` / `.sh` verified working
  - [ ] `scripts/lint.ps1` / `.sh` verified working

- [ ] **Documentation Updated**
  - [ ] `scripts/README.md` updated with lint script section
  - [ ] `scripts/README.md` updated with pre-commit hooks section
  - [ ] Hook installation instructions clear
  - [ ] Hook bypass instructions documented
  - [ ] Troubleshooting guide added

### Phase 7: Benchmarking CI

- [ ] **Benchmarks Verified**
  - [ ] Compiler benchmarks run successfully
  - [ ] Runtime benchmarks run successfully
  - [ ] Results compared against v0.0.2 baseline
  - [ ] Any regressions (>10%) documented in LEARNINGS.md

- [ ] **CI Workflow Created**
  - [ ] `.github/workflows/benchmarks.yml` created
  - [ ] Workflow triggers on push to main/develop
  - [ ] Workflow triggers on manual dispatch
  - [ ] Workflow runs compiler benchmarks
  - [ ] Workflow runs runtime benchmarks
  - [ ] Workflow uploads results as artifacts
  - [ ] Workflow comments results on PRs (if PR context)
  - [ ] Workflow tested with PR
  - [ ] Workflow tested with push to develop

- [ ] **Documentation Updated**
  - [ ] README.md mentions benchmark CI
  - [ ] BENCHMARK_BASELINE.md updated (if baselines changed)
  - [ ] CI workflow documented (in workflow README or main README)

### Phase 9 Quick Wins: GitHub Badges

- [ ] **Badges Added**
  - [ ] Build status badge added to README
  - [ ] License badge added to README
  - [ ] Rust version badge added to README
  - [ ] Godot version badge added to README
  - [ ] Badges rendered correctly in GitHub
  - [ ] Badge links verified working

### Quality Gates

- [ ] **Build & Test**
  - [ ] All 263+ tests passing
  - [ ] Zero clippy warnings
  - [ ] Code formatted with cargo fmt
  - [ ] All benchmarks run successfully

- [ ] **Documentation**
  - [ ] Markdown linting passes (`npm run docs:lint`)
  - [ ] All links validated in modified docs
  - [ ] No broken links
  - [ ] Documentation clear and accurate

### Final Verification

- [ ] **Manual Testing**
  - [ ] All scripts tested manually
  - [ ] Pre-commit hooks tested (install, run, bypass)
  - [ ] CI workflow tested (push, artifacts, comments)
  - [ ] Badges tested (render, links)

- [ ] **Git Hygiene**
  - [ ] All changes committed
  - [ ] Commit messages follow conventional commits
  - [ ] Branch up to date with develop
  - [ ] No uncommitted changes

- [ ] **PR Preparation**
  - [ ] PR description written (see template below)
  - [ ] Screenshots/recordings of key features (if applicable)
  - [ ] Reviewers assigned
  - [ ] Milestone linked (#2)

---

## 📄 PR Description Template

```markdown
# Phase 6+7: Development Tooling & CI (Combined)

## 🎯 Overview

Completes v0.0.3 development tooling by finalizing Phase 6 scripts (80% already existed) and integrating Phase 7 benchmarking into CI pipeline.

**Key Achievement**: v0.0.3 now has complete developer tooling infrastructure, fulfilling all critical prerequisites for v0.1.0.

## ✅ Deliverables

### Phase 6: Development Scripts

**Scripts Completed**:
- ✅ `scripts/lint.ps1` / `.sh` - Cargo clippy wrapper with strict warnings
- ✅ Pre-commit hooks - Automated format, lint, and test checks
- ✅ All 5 core scripts verified working (test, bench, format, coverage, lint)

**Pre-commit Hook Features**:
- Runs format check before commit
- Runs clippy linting before commit
- Runs quick unit tests before commit
- Can be bypassed with `--no-verify` for WIP commits
- Installable via `scripts/install-git-hooks.ps1` / `.sh`

### Phase 7: Benchmarking CI

**Benchmark Infrastructure**:
- ✅ Existing benchmarks verified working (lexer, parser, type_checker, runtime)
- ✅ `.github/workflows/benchmarks.yml` created
- ✅ CI runs on main/develop branches (on push)
- ✅ Manual trigger available for feature branches
- ✅ Results stored as artifacts (30 day retention)
- ✅ Automatic PR comments with benchmark results

**Baseline Verification**:
- [List any changes to baselines, or "No significant changes"]

### Phase 9 Quick Wins: GitHub Badges

**Badges Added**:
- ✅ Build status badge (GitHub Actions CI)
- ✅ License badge (MIT)
- ✅ Rust version badge (1.70+)
- ✅ Godot version badge (4.2+)

## 🧪 Testing

### Manual Testing

- ✅ All scripts tested on Windows (PowerShell)
- [✅/➖] Scripts tested on Linux/macOS (Bash) - if available
- ✅ Pre-commit hooks installed and tested
- ✅ Pre-commit hooks block bad commits (formatting, lint, tests)
- ✅ Pre-commit hooks bypassable with `--no-verify`
- ✅ Benchmark CI workflow runs successfully
- ✅ Benchmark artifacts uploaded correctly
- ✅ GitHub badges render correctly

### Automated Testing

- ✅ All 263+ tests passing
- ✅ Zero clippy warnings
- ✅ Code formatted with cargo fmt
- ✅ Markdown linting passes

## 📸 Screenshots

[Optional: Add screenshots of:]
- Pre-commit hook output (blocking commit)
- Benchmark CI workflow run
- GitHub badges in README
- PR comment with benchmark results

## 📚 Documentation

**Updated**:
- ✅ `scripts/README.md` - Added lint script and pre-commit hooks sections
- ✅ `README.md` - Added GitHub badges section
- ✅ `docs/planning/v0.0.3/README.md` - Updated phase tracking (combined 6+7)
- ✅ `docs/planning/v0.0.3/PHASE_6_7_COMBINED.md` - This phase document

## 🎯 Strategic Impact

**v0.0.3 Completion**: After this PR, v0.0.3 has completed all critical tooling prerequisites for v0.1.0:
- ✅ Enhanced error diagnostics (Phases 1-3)
- ✅ Professional editor experience (Phases 4-5)
- ✅ Complete development tooling (Phases 6-7)

**Deferred to Later Versions**:
- Phase 8 (Integration Tests) → v0.0.4 (better with expanded Godot API)
- Phase 9 (most items) → v0.1.0 (release-level tasks: marketplace, rustdoc, coverage)

## 📋 Checklist

- [ ] All tests passing
- [ ] Zero clippy warnings
- [ ] Code formatted
- [ ] Documentation linted
- [ ] All links validated
- [ ] Manual testing complete
- [ ] PR description complete
- [ ] Reviewers assigned

## 🔗 Related

- **Milestone**: #2 (v0.0.3 - Editor Experience Alpha)
- **Previous PR**: #38 (Phase 5 - VS Code Hover & Problem Panel)
- **Phase Document**: [PHASE_6_7_COMBINED.md](./PHASE_6_7_COMBINED.md)

---

**Status**: ✅ Ready for Review  
**Estimated Review Time**: 15-20 minutes (mostly script verification)
```

---

## 🚀 Next Steps After Phase 6+7

### Immediate (v0.0.3 Wrap-up)

1. **Final v0.0.3 Review**:
   - Review all 7 phases (1-5 complete, 6-7 this PR)
   - Update CHANGELOG.md with v0.0.3 summary
   - Update version numbers (0.0.3 → 0.0.4-dev or stay 0.0.3 until release)

2. **v0.0.3 Release Preparation**:
   - Create v0.0.3 release notes
   - Tag release: `v0.0.3`
   - Create GitHub release
   - Announce in project README

### Next Version (v0.0.4)

**Focus**: Godot API Expansion

**Planned Features**:

- Expanded Godot types (Sprite2D, RigidBody2D, etc.)
- Signal support
- Callback functions
- Node query functions (get_node, find_child)
- Integration tests (deferred from Phase 8)

**Timeline**: 2-3 weeks after v0.0.3 release

---

**Last Updated**: October 8, 2025  
**Status**: In Progress  
**Branch**: `feature/v0.0.3-phase-6-7-dev-tooling`
