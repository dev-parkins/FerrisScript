# Test Coverage Strategy

**Project**: FerrisScript  
**Status**: v0.0.3 "Editor Experience Alpha" Post-Release  
**Last Updated**: October 8, 2025

---

## 📊 Coverage Tools Overview

FerrisScript uses **two separate coverage tools** due to language-specific limitations:

| Tool | Languages | Purpose | Dashboard |
|------|-----------|---------|-----------|
| **Codecov** | Rust | Primary coverage tracking | [codecov.io/gh/dev-parkins/FerrisScript](https://codecov.io/gh/dev-parkins/FerrisScript) |
| **SonarCloud** | TypeScript, Markdown | Code quality + coverage (future) | [sonarcloud.io/project/overview?id=dev-parkins_FerrisScript](https://sonarcloud.io/project/overview?id=dev-parkins_FerrisScript) |

---

## 🦀 Rust Code Coverage

### Current Status

- **Tool**: Codecov (via cargo-tarpaulin)
- **Current Coverage**: 64.54% (as of commit 4d48d67)
- **Target**:
  - v0.0.4: 70-75%
  - v0.1.0: 80%+
- **Format**: Cobertura XML + LCOV
- **CI Workflow**: `.github/workflows/code-scanning.yml` → `coverage` job

### How It Works

```yaml
# CI generates coverage on every push to develop/main
- name: Generate Coverage Report
  run: |
    cargo tarpaulin --verbose --all-features --workspace \
      --timeout 300 \
      --out Xml --out Lcov \
      --output-dir coverage/
```

**Outputs**:

- `coverage/cobertura.xml` → Uploaded to Codecov
- `coverage/lcov.info` → Uploaded to Codecov (alternate format)

**Artifact**: `coverage-reports` (retained 7 days)

### Why Not SonarCloud for Rust?

**SonarCloud does NOT support Rust as a language.**

**Supported Languages** (as of 2025):

- ✅ Java, C#, JavaScript/TypeScript, Python, Go
- ✅ PHP, Kotlin, Ruby, Scala, Swift, Objective-C
- ❌ **Rust** (not supported)

**Implications**:

- SonarCloud cannot analyze Rust code for quality issues
- SonarCloud cannot parse Rust coverage reports (LCOV or otherwise)
- Property `sonar.rust.lcov.reportPaths` **does not exist**
- Coverage artifact download removed from SonarQube job (unnecessary)

**See**: `docs/planning/technical/SONARCLOUD_RUST_LIMITATION_ANALYSIS.md`

---

## 📜 TypeScript Code Coverage (Future)

### Current Status

- **Tool**: SonarCloud (when coverage generated)
- **Current Coverage**: Not yet implemented
- **Target**: 70%+ (when VSCode extension has tests)
- **Format**: LCOV (if/when implemented)

### Future Implementation

```yaml
# Example: When VSCode extension has tests
- name: Test VSCode Extension
  working-directory: extensions/vscode
  run: |
    npm test -- --coverage
    # Generates: extensions/vscode/coverage/lcov.info
```

**sonar-project.properties** (future):

```properties
sonar.typescript.lcov.reportPaths=extensions/vscode/coverage/lcov.info
```

**Status**: Not yet implemented (VSCode extension in early development)

---

## 📋 Coverage Targets by Version

### v0.0.3 (Current: "Editor Experience Alpha")

**Rust Coverage**:

- ✅ Current: **64.54%**
- Target: 65%+ (MET ✅)
- Focus: Core compiler functionality

**TypeScript Coverage**:

- ❌ Not implemented yet
- Target: N/A (extension too early)

### v0.0.4 (Planned: "Enhanced Developer Experience")

**Rust Coverage**:

- Current: 64.54%
- Target: **70-75%**
- Focus: Improved testing for error handling, edge cases

**TypeScript Coverage**:

- Target: **Initial tests** (if extension development progresses)
- Focus: Basic functionality tests

### v0.1.0 (Planned: "Production Ready")

**Rust Coverage**:

- Target: **80%+**
- Focus: Comprehensive testing, all critical paths covered

**TypeScript Coverage**:

- Target: **70%+**
- Focus: Full extension functionality tested

---

## 🚀 CI/CD Integration

### Workflow: `.github/workflows/code-scanning.yml`

**Jobs**:

1. **`coverage`**: Generate Rust coverage (push events only)
   - Runs: cargo-tarpaulin
   - Outputs: Cobertura XML + LCOV
   - Uploads to: Codecov
   - Artifact: coverage-reports (7 days)

2. **`sonarqube`**: Quality scan (push to main/develop)
   - Runs: SonarCloud analysis
   - Analyzes: TypeScript, Markdown, YAML (NOT Rust)
   - Note: Does NOT download coverage artifact (Rust unsupported)

3. **`sonarqube-pr`**: Lightweight scan (pull requests)
   - Runs: SonarCloud PR analysis
   - No coverage required

### Coverage Upload Flow

```
┌─────────────────────────────────────────────┐
│  Push to develop/main                       │
└─────────────────┬───────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────┐
│  Job: coverage                              │
│  ├─ cargo tarpaulin                         │
│  ├─ Generate cobertura.xml + lcov.info      │
│  ├─ Upload to Codecov (Rust)                │
│  └─ Upload artifact: coverage-reports       │
└─────────────────┬───────────────────────────┘
                  │
                  │ (parallel, no dependency)
                  │
                  ▼
┌─────────────────────────────────────────────┐
│  Job: sonarqube                             │
│  ├─ SonarCloud scan                         │
│  ├─ Analyze: TypeScript, Markdown           │
│  └─ Does NOT use coverage artifact          │
└─────────────────────────────────────────────┘
```

**Why No Job Dependency?**

- SonarCloud cannot use Rust coverage
- Parallel execution saves ~4 minutes
- No need to wait for coverage completion

---

## 📈 How to View Coverage

### Rust Coverage (Codecov)

1. **Dashboard**: https://codecov.io/gh/dev-parkins/FerrisScript
2. **PR Comments**: Codecov bot comments on every PR with diff coverage
3. **Files View**: Browse coverage by file in Codecov UI
4. **Trends**: Track coverage over time

**Example**:

```
Codecov Report
Merging #32 (abc1234) into develop (def5678) will increase coverage by 0.12%.
The diff coverage is 75.00%.

@@            Coverage Diff             @@
##           develop    #32      +/-   ##
==========================================
+ Coverage    64.54%  64.66%   +0.12%     
==========================================
  Files           42      42              
  Lines         3521    3548     +27     
==========================================
+ Hits          2273    2295     +22     
- Misses        1248    1253      +5     
```

### TypeScript Quality (SonarCloud)

1. **Dashboard**: https://sonarcloud.io/project/overview?id=dev-parkins_FerrisScript
2. **Quality Gate**: Shows code smells, bugs, security issues (NOT Rust coverage)
3. **PR Decoration**: SonarCloud comments on PRs (quality metrics only)

**Current Metrics** (focus areas):

- Code Smells: Maintainability issues
- Bugs: Potential runtime errors
- Security Hotspots: Security review needed
- Duplication: Repeated code blocks

**Note**: Coverage shown as "0.0%" or "N/A" (expected for Rust project)

---

## 🎯 Coverage Goals

### Short-Term (v0.0.4)

**Rust**:

- [ ] Increase coverage to 70%+
- [ ] Add tests for error handling paths
- [ ] Add tests for edge cases in type checker
- [ ] Add tests for Godot binding layer

**TypeScript**:

- [ ] Add basic tests for VSCode extension (if time permits)
- [ ] Set up Jest or Vitest
- [ ] Initial 50%+ coverage on core functions

### Long-Term (v0.1.0)

**Rust**:

- [ ] Achieve 80%+ coverage
- [ ] 100% coverage on critical paths (lexer, parser, type checker)
- [ ] Add property-based tests (proptest)
- [ ] Add fuzzing tests

**TypeScript**:

- [ ] 70%+ coverage on VSCode extension
- [ ] Integration tests with mock LSP server
- [ ] E2E tests with VS Code test runner

---

## 🛠️ Local Coverage Commands

### Generate Rust Coverage Locally

```powershell
# Install tarpaulin (if not installed)
cargo install cargo-tarpaulin

# Generate coverage report (HTML + terminal)
cargo tarpaulin --verbose --all-features --workspace `
  --timeout 300 `
  --out Html --out Lcov `
  --output-dir coverage/

# View HTML report
Start-Process coverage/index.html  # Windows
```

### Generate TypeScript Coverage Locally (Future)

```powershell
# When VSCode extension has tests
cd extensions/vscode
npm test -- --coverage

# View HTML report
Start-Process coverage/lcov-report/index.html  # Windows
```

---

## 📝 Best Practices

### Writing Tests for Coverage

**DO**:

- ✅ Focus on critical paths first (parser, type checker)
- ✅ Test error handling explicitly
- ✅ Test edge cases (empty input, max values, etc.)
- ✅ Use property-based tests for complex logic
- ✅ Aim for meaningful tests, not just coverage numbers

**DON'T**:

- ❌ Write tests just to increase coverage percentage
- ❌ Skip error paths ("this can never happen")
- ❌ Test implementation details
- ❌ Ignore failing tests to keep coverage high

### Coverage vs. Quality

**Coverage is NOT quality!**

- 100% coverage ≠ Bug-free code
- Focus on **meaningful tests** that verify behavior
- Use coverage to **find untested code**, not as a goal
- Combine with code review, static analysis, fuzzing

**Good Test Indicators**:

- Tests fail when code is broken
- Tests document expected behavior
- Tests are readable and maintainable
- Tests run quickly (< 1 second per test)

---

## 🔗 Related Documentation

- **SonarCloud Rust Limitation**: `docs/planning/technical/SONARCLOUD_RUST_LIMITATION_ANALYSIS.md`
- **Coverage Setup Notes**: `docs/COVERAGE_SETUP_NOTES.md`
- **Test Coverage Analysis**: `docs/TEST_COVERAGE_ANALYSIS.md`
- **CI Workflow**: `.github/workflows/code-scanning.yml`
- **SonarCloud Config**: `sonar-project.properties`

---

## 💡 FAQ

### Q: Why is SonarCloud showing 0% coverage?

**A**: SonarCloud does NOT support Rust language. The 0% is expected and not a bug. Use Codecov for Rust coverage.

### Q: Can we use one tool for both Rust and TypeScript?

**A**: Not effectively. Codecov excels at Rust coverage visualization. SonarCloud excels at code quality analysis for supported languages. Using both provides the best developer experience.

### Q: Why not just use SonarCloud's Generic Test Coverage format?

**A**: Even with Generic format, SonarCloud cannot analyze Rust code quality (smells, bugs, security). Coverage percentage alone without quality analysis provides limited value. Codecov's Rust-specific features (file browsing, diff coverage, trends) are superior.

### Q: Will we add Rust support to SonarCloud in the future?

**A**: This depends on SonarSource (SonarCloud vendor) adding Rust language support. As of 2025, there are no official plans. We'll continue using Codecov for Rust, which is the industry-standard tool for Rust projects.

### Q: How do I get coverage in my local environment?

**A**: See "Local Coverage Commands" section above. Use `cargo tarpaulin` for Rust, generates HTML reports you can view in browser.

---

**Last Updated**: October 8, 2025  
**Next Review**: v0.0.4 milestone kick-off  
**Owner**: Development Team
