# Error Message Improvements (Phase 2) - Execution Plan

**Workstream**: FerrisScript v0.0.2 Error Handling - Phase 2  
**Date**: October 3, 2025  
**Agent**: GitHub Copilot  
**Status**: Planning → In Progress  
**Branch**: `feature/error-messages-phase2`  
**Target PR**: #12 (TBD)

---

## 📋 Executive Summary

This workstream implements Phase 2 of the error handling improvements from `EDGE_CASE_ERROR_HANDLING_PLAN.md`. Phase 1 (edge case tests) is complete with 15 tests merged in PR #11. Phase 2 focuses on ensuring all compiler errors include line numbers and column information.

**Scope**: Line/column position tracking only (no source context display, no colorization, no error recovery - those are Phase 3-5)

---

## Q&A: Context Gathering

### Workstream Context

**Q1: What is the primary goal?**
A: Ensure ALL compiler errors (lexer, parser, type checker) include line number and column information for better debugging.

**Q2: What version is this for?**
A: v0.0.2 (Patch Release)

**Q3: What type of release?**
A: Patch release - bug fixes and improvements, no breaking changes to API (error message text changes are acceptable for pre-1.0)

**Q4: Why is this work important?**
A: Current error messages are inconsistent - some include positions, some don't. Users need precise error locations to debug scripts efficiently.

**Q5: What's the source of requirements?**
A: v0.0.2-CHECKLIST.md (lines 54-58) and EDGE_CASE_ERROR_HANDLING_PLAN.md

### Prior Work

**Q1: Has similar work been done before?**
A: Yes! Position tracking infrastructure already exists:

- `Span` struct in `ast.rs` with line/column fields ✅
- Lexer tracks `self.line` and `self.column` ✅
- Some errors already include "at line X, column Y" ✅

**Q2: Are there existing tests?**
A: 111 tests total (96 original + 15 from Phase 1). No specific error message validation tests yet.

**Q3: What documentation exists?**
A: EDGE_CASE_ERROR_HANDLING_PLAN.md (657 lines), TEST_COVERAGE_ANALYSIS.md, LEARNINGS.md

**Q4: What patterns should I follow?**
A: Existing error format in parser.rs: `"Expected {}, found {} at line {}, column {}"`

**Q5: What should I NOT change?**
A: Don't modify AST structure, don't change token types, don't alter compilation behavior (only improve error messages)

### Constraints

**Q1: What changes are allowed?**
A: Error message text improvements, adding position info, creating error message tests

**Q2: What changes are NOT allowed?**
A: No new features, no API changes, no breaking changes to compilation behavior

**Q3: Are there performance requirements?**
A: Position tracking must not significantly impact compilation speed (current lexer already tracks positions)

**Q4: Are there platform considerations?**
A: Must work on Windows, Linux, macOS (CI tests all three)

**Q5: What's the timeline?**
A: High priority for v0.0.2, estimated 6-8 hours (vs. 17 hours in original plan)

### Quality Standards

**Q1: What tests must pass?**
A: `cargo test --workspace` (all 111 tests)

**Q2: What linting must pass?**
A: `cargo clippy --workspace -- -D warnings`, `cargo fmt --check`, `npm run docs:lint`

**Q3: What's the test coverage target?**
A: Maintain or improve current coverage (~70-75%)

**Q4: What's the documentation requirement?**
A: Update TEST_COVERAGE_ANALYSIS.md, v0.0.2-CHECKLIST.md, create summary document

**Q5: What's the code review process?**
A: Self-review git diff, create PR with detailed description

### Contribution Workflow

**Q1: What branch should I create?**
A: `feature/error-messages-phase2` (feature/* convention from CONTRIBUTING.md)

**Q2: What's the commit message format?**
A: Conventional commits: `fix(compiler): improve error messages with consistent line/column info`

**Q3: Where should files go?**
A: Tests in `crates/compiler/tests/`, docs in `docs/v0.0.2/`

**Q4: What documents need updating?**
A: TEST_COVERAGE_ANALYSIS.md, v0.0.2-CHECKLIST.md, EDGE_CASE_ERROR_HANDLING_PLAN.md

**Q5: How should I track progress?**
A: TODO list in Copilot (already created)

---

## 🎯 Acceptance Criteria

Based on EDGE_CASE_ERROR_HANDLING_PLAN.md (AC-4):

### AC-4.1: Lexer Errors

✅ All lexer errors must include line number and column

### AC-4.2: Parser Errors  

✅ All parser errors must include line number and column (defer ±2 lines context to Phase 3)

### AC-4.3: Type Checker Errors

✅ All type checker errors must include location and type information

### AC-4.4: Runtime Errors

⏸️ DEFERRED to Phase 3 (requires call stack tracking - more complex)

### AC-4.5: Error Quality

✅ Error messages must be clear, actionable, and user-friendly

### Additional Criteria (This Phase)

✅ Create error message validation tests  
✅ Document error message format in code comments  
✅ All existing tests continue to pass  
✅ No performance degradation

---

## 📊 Execution Strategy Decision

**DECISION: Option C - Incremental Validation (Phase 2 Only)**

**Rationale:**

- Phase 1 complete (15 tests merged) ✅
- Phase 2 is self-contained (line/column improvements only)
- Estimated 6-8 hours (manageable single PR)
- Phases 3-5 are more complex (colorization, recovery) - defer to separate PRs
- Fast feedback loop - validate approach before larger work

**Out of Scope (Phases 3-5):**

- ❌ Source context display (±2 lines)
- ❌ Visual indicators (^ pointer)
- ❌ Colorized output
- ❌ Error recovery (multiple errors per pass)
- ❌ Runtime stack traces

---

## 🔧 Execution Phases

### Phase 0: Planning & Context Gathering ✅

- [x] Read attached documentation
- [x] Discovered existing infrastructure (Span, line/column tracking)
- [x] Created execution plan document
- [x] Defined acceptance criteria
- [x] Created TODO list

### Phase 2.1: Audit Current Error Messages 🔄

**Estimated**: 1 hour

- [ ] Scan `lexer.rs` for all `Err()` returns
- [ ] Scan `parser.rs` for all `Err()` returns  
- [ ] Scan `type_checker.rs` for all `Err()` returns
- [ ] Document which errors have position info, which don't
- [ ] Create audit report in this document

### Phase 2.2: Improve Lexer Errors

**Estimated**: 1.5 hours

- [ ] Review all lexer error generation points
- [ ] Add line/column to any missing error messages
- [ ] Verify format consistency
- [ ] Create test file: `error_messages_lexer.rs`
- [ ] Run tests: `cargo test --test error_messages_lexer`

### Phase 2.3: Improve Parser Errors

**Estimated**: 2 hours

- [ ] Review all parser error generation points
- [ ] Add line/column to any missing error messages
- [ ] Standardize error format
- [ ] Create test file: `error_messages_parser.rs`
- [ ] Run tests: `cargo test --test error_messages_parser`

### Phase 2.4: Improve Type Checker Errors

**Estimated**: 1.5 hours

- [ ] Review all type checker error generation points
- [ ] Add location and type info to errors
- [ ] Ensure expected vs. actual types are clear
- [ ] Create test file: `error_messages_type_checker.rs`
- [ ] Run tests: `cargo test --test error_messages_type_checker`

### Phase 2.5: Quality Validation & Documentation

**Estimated**: 2 hours

- [ ] Run full test suite: `cargo test --workspace`
- [ ] Run clippy: `cargo clippy --workspace -- -D warnings`
- [ ] Run formatter: `cargo fmt --all`
- [ ] Run docs lint: `npm run docs:lint`
- [ ] Update TEST_COVERAGE_ANALYSIS.md
- [ ] Update v0.0.2-CHECKLIST.md
- [ ] Update EDGE_CASE_ERROR_HANDLING_PLAN.md
- [ ] Create ERROR_MESSAGES_PHASE2_SUMMARY.md
- [ ] Self-review git diff
- [ ] Create PR

---

## 📦 Deliverables

### Code Changes

**Files Modified:**

- `crates/compiler/src/lexer.rs` - Improve error messages
- `crates/compiler/src/parser.rs` - Improve error messages
- `crates/compiler/src/type_checker.rs` - Improve error messages

**Files Created:**

- `crates/compiler/tests/error_messages_lexer.rs` - Lexer error tests
- `crates/compiler/tests/error_messages_parser.rs` - Parser error tests
- `crates/compiler/tests/error_messages_type_checker.rs` - Type checker tests

### Documentation

**Files Updated:**

- `docs/v0.0.2/TEST_COVERAGE_ANALYSIS.md` - Add error message tests
- `docs/v0.0.2/v0.0.2-CHECKLIST.md` - Mark phase 2 complete
- `docs/v0.0.2/EDGE_CASE_ERROR_HANDLING_PLAN.md` - Update status table

**Files Created:**

- `docs/v0.0.2/ERROR_MESSAGES_PHASE2_EXECUTION_PLAN.md` - This document
- `docs/v0.0.2/ERROR_MESSAGES_PHASE2_SUMMARY.md` - Completion summary

---

## 📝 Implementation Notes

### Error Message Format Standard

**Lexer Errors:**

```
Invalid number '123.456.789' at line 5, column 10
```

**Parser Errors:**

```
Expected ';', found '}' at line 8, column 5
```

**Type Checker Errors:**

```
Type mismatch at line 12, column 15: expected i32, found f32
```

### Testing Approach

**Error Message Tests:**

- Use `assert!(result.is_err())` to verify error occurs
- Use `assert!(error_msg.contains("line"))` to verify position included
- Use `assert!(error_msg.contains("column"))` to verify column included
- For critical errors, use exact string matching with `assert_eq!`

### Trade-offs & Decisions

**Decision 1: Defer Source Context Display**

- **Rationale**: Requires passing source string through compilation pipeline (more complex)
- **Alternative**: Could implement now, but increases scope significantly
- **Chosen**: Defer to Phase 3 for cleaner, focused PR

**Decision 2: Pattern Matching vs. Exact Matching**

- **Rationale**: Error message text may evolve, exact matching is brittle
- **Alternative**: Use exact matching for stability
- **Chosen**: Pattern matching for position info, exact for critical errors

**Decision 3: Runtime Errors Deferred**

- **Rationale**: Requires call stack tracking infrastructure (not present)
- **Alternative**: Implement minimal stack tracking now
- **Chosen**: Defer to Phase 3 - runtime errors less critical than compile errors

---

## ⏱️ Time Estimates

| Phase | Task | Estimated | Actual | Notes |
|-------|------|-----------|--------|-------|
| 0 | Planning | 1h | 0.5h | Pre-existing plan helped |
| 2.1 | Audit | 1h | - | |
| 2.2 | Lexer | 1.5h | - | |
| 2.3 | Parser | 2h | - | |
| 2.4 | Type Checker | 1.5h | - | |
| 2.5 | Validation | 2h | - | |
| **Total** | **Phase 2** | **9h** | **TBD** | |

**Note**: Original plan estimated 17h for all Phase 2 tasks. Scoping down (no context display, no stack traces) reduces to ~8-9h.

---

## 🔗 Related Documents

- [EDGE_CASE_ERROR_HANDLING_PLAN.md](./EDGE_CASE_ERROR_HANDLING_PLAN.md) - Master plan
- [v0.0.2-CHECKLIST.md](./v0.0.2-CHECKLIST.md) - Release checklist
- [EDGE_CASE_TESTS_PHASE1_SUMMARY.md](./EDGE_CASE_TESTS_PHASE1_SUMMARY.md) - Phase 1 completion
- [TEST_COVERAGE_ANALYSIS.md](./TEST_COVERAGE_ANALYSIS.md) - Coverage tracking

---

## 📊 Audit Results (Phase 2.1 Complete)

### Lexer Errors Analysis

**Total Error Points**: 7  
**With Position Info**: 7 ✅ (100%)

| Error Type | Has Line/Column | Example |
|------------|----------------|---------|
| Invalid number | ✅ Yes | `Invalid number '123.456' at line 5, column 10` |
| Unterminated string | ✅ Yes | `Unterminated string at line 3, column 15` |
| Invalid escape sequence | ✅ Yes | `Invalid escape sequence '\x' at line 4, column 20` |
| Unexpected '&' | ✅ Yes | `Unexpected character '&' at line 2, column 5. Did you mean '&&'?` |
| Unexpected '|' | ✅ Yes | `Unexpected character '|' at line 6, column 12. Did you mean '||'?` |
| Unexpected character | ✅ Yes | `Unexpected character '@' at line 1, column 3` |

**Conclusion**: ✅ **Lexer errors are COMPLETE** - all already include line/column information!

### Parser Errors Analysis

**Total Error Points**: 14  
**With Position Info**: 3 ✅ (21%)  
**Missing Position Info**: 11 ❌ (79%)

| Error Type | Has Line/Column | Location |
|------------|----------------|----------|
| `expect()` method | ✅ Yes | line 45-51 |
| Top-level syntax error | ✅ Yes | line 69-75 |
| Unexpected token in expression | ✅ Yes | line 463-468 |
| Expected identifier after 'let' | ❌ No | line 266-270 |
| Expected type | ❌ No | line 106, 148, 277 |
| Expected function name | ❌ No | line 131 |
| Expected parameter name | ❌ No | line 141 |
| Expected parameter type | ❌ No | line 148 |
| Expected return type | ❌ No | line 172 |
| Expected '>' after '-' | ❌ No | line 175 |
| Expected field name after '.' | ❌ No | line 371 |
| Not a binary operator | ❌ No | line 499 |

**Conclusion**: ⚠️ **Parser needs improvement** - 11/14 errors missing position info

### Type Checker Errors Analysis

**Total Error Points**: 10  
**With Position Info**: 10 ✅ (100%)

| Error Type | Has Position Info | Format |
|------------|------------------|---------|
| Cannot infer type (global var) | ✅ Yes | `Cannot infer type for global variable 'x' at line 5, column 1` |
| Type mismatch (global var) | ✅ Yes | `Type mismatch in global variable 'x': expected i32, found f32 at line 5, column 1` |
| Cannot infer type (local var) | ✅ Yes | `Cannot infer type for variable 'x' at line 10, column 5` |
| Type mismatch (let binding) | ✅ Yes | `Type mismatch in let binding 'x': expected i32, found f32 at line 10, column 5` |
| Type mismatch (assignment) | ✅ Yes | `Type mismatch in assignment: expected i32, found f32 at line 12, column 5` |
| If condition must be bool | ✅ Yes | `If condition must be bool, found i32 at line 15, column 3` |
| While condition must be bool | ✅ Yes | `While condition must be bool, found i32 at line 20, column 3` |
| Undefined variable | ✅ Yes | `Undefined variable 'foo' at line 25, column 10` |
| Binary operation type error | ✅ Yes | `Binary operation + requires numeric types, found bool and i32 at line 30, column 15` |
| Undefined function | ✅ Yes | `Undefined function 'bar' at line 35, column 5` |

**Conclusion**: ✅ **Type checker errors are COMPLETE** - all already include position and type info!

### Summary

| Component | Complete | Needs Work | Priority |
|-----------|----------|------------|----------|
| Lexer | ✅ 7/7 (100%) | None | ✅ Done |
| Parser | ⚠️ 3/14 (21%) | 11 errors | 🔴 High |
| Type Checker | ✅ 10/10 (100%) | None | ✅ Done |

**Total Progress**: 20/31 errors (65%) already have position info  
**Work Required**: Fix 11 parser errors (35% of total)

**Revised Estimate**: 2-3 hours instead of 8 hours (lexer and type checker already done!)

---

**Status**: ✅ Phase 2 Complete - All Error Messages Include Position Info  
**Result**: 31/31 errors (100%) now have line/column information + 22 validation tests passing
