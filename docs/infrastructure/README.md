# CI/CD Infrastructure Overview

**Last Updated**: October 8, 2025  
**Maintainer**: FerrisScript Team

---

## 🎯 Purpose

This document provides an overview of the FerrisScript CI/CD infrastructure, including build pipelines, code scanning, coverage reporting, and quality gates.

---

## 📊 Workflow Architecture

### Primary Workflows

#### 1. CI/CD Pipeline (`.github/workflows/ci.yml`)

**Purpose**: Build, test, and release automation  
**Triggers**: Push to main/develop, PRs, version tags

**Jobs**:

- **Quick Check** (PRs only): Fast feedback (2-3 min)
  - Format check (`cargo fmt`)
  - Linting (`cargo clippy`)
  - Unit tests (`cargo test --lib`)
  
- **Full Test Suite** (main/develop/tags): Comprehensive validation
  - Cross-platform testing (Ubuntu, Windows, macOS)
  - All tests (`cargo test --workspace`)
  - Linting and formatting checks
  
- **Build Release** (main/develop/tags): Release artifacts
  - Multi-platform builds (Linux, Windows, macOS)
  - Godot GDExtension binaries
  - Upload build artifacts
  
- **Release** (version tags only): GitHub releases
  - Create release from tag
  - Attach platform binaries
  - Include release notes

**Runtime**: 5-15 minutes (depending on job)

---

#### 2. Code Scanning & Coverage (`.github/workflows/code-scanning.yml`)

**Purpose**: Security scanning, quality analysis, and coverage reporting  
**Triggers**: Push to main/develop, PRs

**Jobs**:

- **SonarQube Quality Scan**: Static code analysis
  - Code quality metrics (maintainability, reliability, security)
  - Code smells and technical debt tracking
  - Security hotspots and vulnerabilities
  - **Note**: Coverage reporting disabled (handled by Codecov)
  
- **Codecov Coverage** (main/develop only): Test coverage reporting
  - Generate coverage with `cargo-tarpaulin`
  - Upload to Codecov for tracking and visualization
  - Diff coverage on PRs
  - Historical trends

**Runtime**: 8-12 minutes

---

#### 3. Documentation Linting (`.github/workflows/docs-lint.yml`)

**Purpose**: Markdown quality and link validation  
**Triggers**: PRs with `.md` file changes, pushes to main

**Jobs**:

- Markdown linting (`markdownlint`)
- Link validation
- Documentation standards enforcement

**Runtime**: 1-2 minutes

---

#### 4. Benchmark Tracking (`.github/workflows/benchmarks.yml`)

**Purpose**: Performance regression detection  
**Triggers**: Push to main/develop, manual dispatch

**Jobs**:

- Compiler benchmarks (`cargo bench --package ferrisscript_compiler`)
- Runtime benchmarks (`cargo bench --package ferrisscript_runtime`)
- Artifact uploads for comparison

**Runtime**: 5-10 minutes

---

## 🔐 Secrets Configuration

### Required Secrets

| Secret | Purpose | Used In |
|--------|---------|---------|
| `CODECOV_TOKEN` | Codecov upload authentication | `code-scanning.yml` |
| `SONAR_TOKEN` | SonarQube authentication | `code-scanning.yml` |
| `GITHUB_TOKEN` | GitHub API access (auto-provided) | All workflows |

### Setup Instructions

1. **Codecov Token**:
   - Go to [codecov.io](https://codecov.io/)
   - Add FerrisScript repository
   - Copy upload token
   - Add as `CODECOV_TOKEN` in GitHub repo secrets

2. **SonarQube Token**:
   - Go to SonarQube/SonarCloud dashboard
   - Generate project token
   - Add as `SONAR_TOKEN` in GitHub repo secrets

---

## 📈 Coverage Reporting Strategy

### Dual-Tool Approach

**Local Development**: `cargo-llvm-cov`

- Cross-platform (Windows, macOS, Linux)
- No file locking issues
- Fast execution
- HTML reports for detailed analysis

**CI/CD**: `cargo-tarpaulin`

- Linux-only (GitHub Actions runners)
- XML output for Codecov
- Standardized for all CI pipelines

### Coverage Targets

- **Unit Tests**: 80%+ coverage
- **Integration Tests**: Focus on critical paths
- **Overall Project**: 75%+ coverage

See [`COVERAGE_SETUP_NOTES.md`](./COVERAGE_SETUP_NOTES.md) for detailed technical background.

---

## 🛡️ Quality Gates

### Pre-Merge Requirements

All PRs must pass:

- ✅ Format check (`cargo fmt --check`)
- ✅ Linting (`cargo clippy` with zero warnings)
- ✅ All tests pass (`cargo test --workspace`)
- ✅ Documentation linting (`markdownlint`)
- ✅ SonarQube quality scan (no blockers)

### Branch Protection Rules

**Main Branch**:

- Require PR approval
- Require status checks to pass
- No direct pushes
- Enforce linear history

**Develop Branch**:

- Require status checks to pass
- Allow squash merges
- Require up-to-date branches

---

## 🔍 Code Scanning Tools

### Current Tools

#### SonarQube (Quality Analysis)

**Capabilities**:

- Code quality metrics (maintainability index)
- Security vulnerability detection
- Code smell identification
- Technical debt tracking
- Duplicate code detection

**Limitations**:

- No Rust-specific rules (generic static analysis)
- Coverage reporting disabled (redundant with Codecov)

#### Codecov (Coverage Reporting)

**Capabilities**:

- Line and branch coverage tracking
- Historical trends and graphs
- PR diff coverage
- Coverage badges
- Team notifications

**Limitations**:

- No static analysis (coverage only)
- Requires Codecov cloud account

### Future Tools (Under Evaluation)

See [`CODEQL_EVALUATION.md`](./CODEQL_EVALUATION.md) for CodeQL integration analysis.

---

## 🚀 Workflow Optimization

### Caching Strategy

All workflows use GitHub Actions caching:

- **Cargo Registry**: `~/.cargo/registry` (dependency metadata)
- **Cargo Index**: `~/.cargo/git` (crates.io index)
- **Build Artifacts**: `target/` (compiled objects)

**Cache Keys**: Based on `Cargo.lock` hash for invalidation on dependency changes.

### Parallelization

- **Quick Check**: Single job, fast feedback
- **Full Tests**: Matrix strategy (3 platforms × 1 Rust version = 3 parallel jobs)
- **Build Release**: Matrix strategy (3 platforms in parallel)

---

## 📝 Maintenance

### Updating Workflows

1. Edit workflow files in `.github/workflows/`
2. Test changes in feature branches
3. Review workflow logs in GitHub Actions tab
4. Update this documentation if behavior changes

### Updating Dependencies

GitHub Actions dependencies (actions):

- Use pinned SHA hashes for security (e.g., `@11bd71901bbe5b...`)
- Update via Dependabot or manual review
- Verify compatibility before merging

Rust toolchain:

- Use `stable` toolchain (auto-updates)
- Pin specific version only if compatibility issues arise

### Monitoring

- **GitHub Actions Tab**: View workflow runs and logs
- **Codecov Dashboard**: Track coverage trends
- **SonarQube Dashboard**: Monitor code quality
- **Dependabot Alerts**: Security vulnerability notifications

---

## 🔗 Related Documentation

- [Coverage Setup Notes](./COVERAGE_SETUP_NOTES.md) - Technical details on coverage tools
- [CodeQL Evaluation](./CODEQL_EVALUATION.md) - CodeQL integration analysis
- [Development Guide](../DEVELOPMENT.md) - Local development workflow
- [GitHub Actions README](../../.github/workflows/README.md) - Workflow descriptions

---

## 📞 Support

For infrastructure issues:

1. Check [Troubleshooting Guide](../TROUBLESHOOTING.md)
2. Review workflow logs in GitHub Actions
3. Open GitHub issue with `infrastructure` label
4. Contact maintainers for secrets/permissions issues
