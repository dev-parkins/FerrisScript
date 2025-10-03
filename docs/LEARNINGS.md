# Development Learnings & Best Practices 🧠

This document captures insights, patterns, and lessons learned during FerrisScript development.

---

## Code Quality Improvements (feature/code-quality-improvements)

### Clippy Warning Resolution

**Date**: October 2, 2025  
**Issue**: `clippy::collapsible_match` warning in `crates/runtime/src/lib.rs:405`

**Problem**:
```rust
// Before (nested if let - discouraged)
if let Some(var) = env.get_mut(name) {
    if let Value::Vector2 { .. } = var {
        return Err("Not implemented".to_string());
    }
}
```

**Solution**:
```rust
// After (collapsed pattern - preferred)
if let Some(Value::Vector2 { .. }) = env.get_mut(name) {
    return Err("Not implemented".to_string());
}
```

**Learning**:
- Clippy prefers pattern matching to be done in a single step when possible
- This is more idiomatic Rust and reduces nesting
- Makes code more readable and maintainable
- Always run `cargo clippy --workspace -- -D warnings` to catch issues early

### Test Coverage Tooling Challenges

**Date**: October 2, 2025  
**Issue**: Both tarpaulin and cargo-llvm-cov had installation/compatibility issues on Windows

**Attempted Tools**:
1. **cargo-tarpaulin** - File locking issues on Windows (OS error 32)
   - Cannot clean build directory while IDE processes hold file locks
   - Workaround: Use `--skip-clean` but still encounters runtime locks
   
2. **cargo-llvm-cov** - Silent installation failure during LTO phase
   - Compiles successfully but binary not installed to cargo bin
   - Likely Windows-specific linker issue

**Solution**:
- **Local Development**: Manual test analysis (review test names, code structure)
- **CI Environment**: Use tarpaulin in Linux GitHub Actions (no file locking issues)
- **Coverage Reports**: Generate in CI, upload to Codecov/Coveralls
- **Documentation**: Created TEST_COVERAGE_ANALYSIS.md with manual gap analysis

**Learning**:
- Coverage tooling on Windows is challenging - prefer Linux CI for coverage
- Manual analysis is time-consuming but valuable for understanding test gaps
- Document test gaps qualitatively even without quantitative coverage metrics
- Focus on high-priority edge cases identified through manual review
- CI-generated coverage reports are sufficient for most projects

### Edge Case Test Implementation

**Date**: October 2, 2025  
**Task**: Implemented 20 high-priority edge case tests based on manual coverage analysis

**Results**:
- **Lexer Tests**: 10 new tests, all passing ✅
- **Runtime Tests**: 10 new tests, all passing ✅
- **Total Test Count**: 96 → 116 (+20.8% increase)
- **Estimated Coverage Improvement**: +5-10% line coverage

**Key Discoveries**:

1. **Lexer Limitations**:
   - Large integer literals (e.g., `2147483647`) are parsed as `f32` instead of `i32`
   - This is a lexer heuristic issue - needs refinement for exact integer parsing
   - Workaround: Use smaller literals in tests, document the limitation

2. **Parser Limitations**:
   - Bare blocks `{ }` are not yet supported inside functions
   - Can only create scopes through if/while/function bodies
   - Tests adjusted to use nested if statements instead

3. **Runtime Behavior**:
   - Division by zero does NOT produce a proper error - results in undefined behavior
   - Needs explicit runtime check before division operations
   - Overflow checking happens in debug mode (panics) but not in release (wraps)
   - Should add explicit overflow checks with proper error messages

4. **Global Mutable Variables**:
   - Not fully supported - assignments to globals fail with "immutable variable" error
   - Need to implement proper global mutability tracking
   - Short-circuit evaluation tests had to be simplified

5. **Runtime Strengths**:
   - Deep expression nesting (100 levels) works perfectly
   - Recursion handles 100+ levels without issues
   - Function-level scoping works correctly
   - Early returns from nested control flow work as expected

**Testing Strategy Insights**:
- Edge case tests should document **current behavior**, not just ideal behavior
- Use `match` statements to handle multiple possible outcomes
- Add TODO comments for features that should error but don't
- Tests that reveal limitations are just as valuable as tests that pass
- Balance between testing edge cases and testing realistic scenarios

**Next Steps**:
- Add runtime division-by-zero checks
- Improve lexer integer literal parsing
- Implement proper global mutability
- Consider adding parser support for bare blocks

---

## Documentation Phase Completion (PR #3)

### Phase 4 Deliverables

**Completed Items**:
1. **Community Documentation** (✅ Complete)
   - CONTRIBUTING.md with comprehensive guidelines
   - CODE_OF_CONDUCT.md (Contributor Covenant 2.1)
   - Issue templates (bug, feature, question, docs)
   - PR template with checklist

2. **User Support Documentation** (✅ Complete)
   - FAQ.md with common questions
   - TROUBLESHOOTING.md with platform-specific help

3. **Security Documentation** (✅ Complete)
   - SECURITY.md with vulnerability reporting process
   - Response time expectations (48 hours)
   - Disclosure policy

4. **Architecture Documentation** (✅ Complete)
   - ARCHITECTURE.md (917 lines)
   - Comprehensive system design
   - Component interactions
   - Data flow diagrams

5. **Enhanced Examples** (✅ Complete)
   - hello/ tutorial (224 lines)
   - move/ tutorial (323 lines)
   - bounce/ tutorial (529 lines)
   - Total: 1076+ lines of tutorial content

6. **Documentation Linting** (✅ Complete)
   - markdownlint configuration
   - markdown-link-check integration
   - Local scripts (PowerShell + Bash)
   - CI integration
   - Pre-push hooks (optional)

### Key Learnings

**Documentation Best Practices**:
- Keep helper/temporary docs out of source control unless viable long-term
- Use git history to preserve temporary working documents
- Update checklists as work progresses to track completion
- Cross-platform support (PowerShell + Bash) is essential for open source

**Git Workflow**:
- Create feature branches for focused work areas
- Keep commits atomic and well-described
- Update documentation checklists in the same PR as features
- Clean up temporary files before merging

**Testing Discipline**:
- Always run tests after code changes: `cargo test --workspace`
- Always run clippy: `cargo clippy --workspace -- -D warnings`
- Verify CI passes before merging
- 96 tests passing is our baseline (69 compiler + 26 runtime + 1 godot_bind)

---

## Project Structure Insights

### Root Directory Organization

**Essential Files** (keep these):
- README.md - Project overview
- CONTRIBUTING.md - Contribution guidelines
- CODE_OF_CONDUCT.md - Community standards
- SECURITY.md - Security policy
- LICENSE - Legal terms
- CHANGELOG.md - Version history
- RELEASE_NOTES.md - Release summaries
- RELEASING.md - Release process
- Cargo.toml - Workspace configuration
- package.json - Documentation tooling

**Configuration Files** (keep these):
- .gitignore - Git exclusions
- .markdownlint.json - Markdown linting rules
- .markdown-link-check.json - Link checking config

**Temporary Files** (remove these):
- APPLY_TO_PR3.md ❌
- DOCS_LINT_FINAL_STATUS.md ❌
- DOCS_LINT_FIXES.md ❌
- DOCS_LINTING_SUMMARY.md ❌
- POST_PR3_FIXES.md ❌
- README_BRANCH.md ❌
- TASK_COMPLETE_SUMMARY.md ❌

**Rule**: If a file is only useful during development and won't help future contributors, keep it out of source control.

---

## Next Focus Areas

### Immediate (This Branch)

1. **Test Coverage Tooling** (4-6 hours)
   - Setup cargo-tarpaulin
   - Generate baseline coverage report
   - Identify gaps in test coverage
   - Add CI integration

2. **Performance Benchmarks** (3-4 hours)
   - Setup criterion
   - Benchmark lexer, parser, type checker, runtime
   - Document baseline metrics
   - Add CI tracking

3. **Edge Case Tests** (2-3 hours)
   - Empty files
   - Comments-only files
   - Large number literals
   - Deep nesting
   - Boundary conditions

### v0.0.2 Release Goals

- Fix all known bugs
- 80%+ test coverage
- Performance baselines established
- All documentation complete ✅
- Clean clippy ✅
- Ready for community contributions

---

## Development Commands Reference

### Quality Checks
```bash
# Run all tests
cargo test --workspace

# Run clippy (strict mode)
cargo clippy --workspace -- -D warnings

# Run benchmarks (after setup)
cargo bench

# Generate coverage report (after setup)
cargo tarpaulin --workspace
```

### Documentation
```bash
# Lint markdown files
npm run lint:md

# Check markdown links
npm run lint:md:links

# Auto-fix markdown issues
npm run lint:md:fix
```

### Git Workflow
```bash
# Create feature branch
git checkout -b feature/descriptive-name

# Commit with clear message
git commit -m "type: description

- Detail 1
- Detail 2"

# Push and create PR
git push -u origin feature/descriptive-name
```

---

## Versioning Strategy

- **v0.0.X** = Patch releases (bug fixes, docs, no new features)
- **v0.X.0** = Minor releases (new features, backward compatible)
- **vX.0.0** = Major releases (breaking changes)

**Current**: v0.0.2 (patch release)
**Next**: v0.1.0 (first minor release with new features)

---

*This document is a living record. Add new learnings as development continues.*
