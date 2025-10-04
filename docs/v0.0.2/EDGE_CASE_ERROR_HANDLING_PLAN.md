# Edge Case & Error Handling Implementation Plan

**Workstream**: FerrisScript v0.0.2 Edge Case Testing & Error Handling Improvements  
**Date Started**: October 3, 2025  
**Version**: v0.0.2 (Patch Release)  
**Agent**: Claude 4.5  
**Feature Branch**: `feature/edge-case-error-handling`

---

## 📋 Executive Summary

This document outlines the implementation plan for completing the edge case testing and error handling improvements identified in the v0.0.2 checklist. Work builds upon the initial edge case testing completed in the `feature/code-quality-improvements` branch (20 tests added, documented in LEARNINGS.md).

**Scope**: Complete remaining high-priority edge cases and implement comprehensive error handling improvements.

---

## ✅ Prior Work Validation

### Completed (feature/code-quality-improvements)

**Edge Case Tests Added** (20 tests, 96 → 116):

- ✅ Large integer literals (documented limitation: parsed as f32)
- ✅ Deep expression nesting (100 levels tested)
- ✅ Deep recursion (100+ levels tested)
- ✅ Variable shadowing
- ✅ Early returns from nested control flow
- ✅ Short-circuit evaluation (simplified due to global mutability limitation)

**Documented in**:

- `docs/v0.0.2/LEARNINGS.md` (lines 70-127)
- `docs/v0.0.2/TEST_COVERAGE_ANALYSIS.md` (high-priority gaps identified)

### Remaining Gaps (This Workstream)

**From v0.0.2-CHECKLIST.md (lines 47-58)**:

**Edge Cases**:

- ❌ Empty script files
- ❌ Scripts with only comments
- ❌ Very long variable names (1000+ characters)

**Error Handling**:

- ❌ Better error messages (include line numbers, context)
- ❌ Colorized error output in Godot console
- ❌ Error recovery (continue parsing after errors)
- ❌ Validate error messages in tests

---

## 🎯 Acceptance Criteria

### Edge Case Tests

#### 1. Empty Script Files

**AC-1.1**: Lexer must handle empty input (0 bytes) without panicking  
**AC-1.2**: Parser must return a valid empty AST or appropriate error  
**AC-1.3**: Runtime must execute empty scripts without errors  
**AC-1.4**: Test must verify behavior at all pipeline stages (lexer → parser → runtime)

#### 2. Scripts with Only Comments

**AC-2.1**: Lexer must tokenize comment-only files correctly  
**AC-2.2**: Parser must handle files with only comments (no code)  
**AC-2.3**: Runtime must execute comment-only scripts without errors  
**AC-2.4**: Test variations: single comment, multiple comments, mixed line/block comments (if supported)

#### 3. Very Long Variable Names

**AC-3.1**: Lexer must handle identifiers with 1000+ characters  
**AC-3.2**: Parser must accept long identifiers in all contexts (declarations, references, function names)  
**AC-3.3**: Runtime must correctly resolve long variable names  
**AC-3.4**: Test must verify performance doesn't degrade significantly  
**AC-3.5**: Document any practical limits or recommendations

### Error Handling Improvements

#### 4. Better Error Messages (Line Numbers & Context)

**AC-4.1**: All lexer errors must include line number and column  
**AC-4.2**: All parser errors must include line number, column, and surrounding context (±2 lines)  
**AC-4.3**: All type checker errors must include location and type information  
**AC-4.4**: All runtime errors must include line number and call stack (where applicable)  
**AC-4.5**: Error messages must be clear, actionable, and user-friendly

#### 5. Colorized Error Output

**AC-5.1**: Errors must use ANSI color codes for terminal output  
**AC-5.2**: Godot console integration must support colorized output  
**AC-5.3**: Color scheme: Red for errors, Yellow for warnings, Cyan for hints  
**AC-5.4**: Colorization must be optional (environment variable or config flag)  
**AC-5.5**: Plain text fallback for non-TTY environments

#### 6. Error Recovery (Continue Parsing After Errors)

**AC-6.1**: Parser must attempt to recover from syntax errors and continue parsing  
**AC-6.2**: Multiple errors in a single file must be reported in one pass  
**AC-6.3**: Recovery strategies must be documented (e.g., skip to next statement, insert missing tokens)  
**AC-6.4**: Recovered AST must be marked as "incomplete" to prevent runtime execution  
**AC-6.5**: Maximum error count threshold to prevent infinite loops (suggest 10 errors)

#### 7. Validate Error Messages in Tests

**AC-7.1**: All error tests must use `assert_eq!` or similar to validate exact error messages  
**AC-7.2**: Error message format must be consistent across all compiler stages  
**AC-7.3**: Tests must verify line numbers and context are correct  
**AC-7.4**: Regression tests must be added for any error message changes  
**AC-7.5**: Documentation must include examples of all error message formats

---

## 📂 File Structure & Locations

### Test Files (New)

```
crates/compiler/src/
  tests/
    edge_cases_empty.rs           # AC-1 (Empty files)
    edge_cases_comments.rs         # AC-2 (Comment-only files)
    edge_cases_long_identifiers.rs # AC-3 (Long variable names)
    error_messages.rs              # AC-4 (Line numbers & context)
    error_recovery.rs              # AC-6 (Parser error recovery)
```

### Implementation Files (Modified)

```
crates/compiler/src/
  lexer.rs                # Add position tracking, improve error messages
  parser.rs               # Add error recovery, improve error messages
  type_checker.rs         # Improve error messages with context
  
crates/runtime/src/
  lib.rs                  # Add stack trace to runtime errors
  
crates/godot_bind/src/
  lib.rs                  # Add colorized console output for Godot
```

### Documentation Files (Updated)

```
docs/v0.0.2/
  EDGE_CASE_ERROR_HANDLING_SUMMARY.md  # NEW: Workstream summary
  TEST_COVERAGE_ANALYSIS.md            # Update with new tests
  LEARNINGS.md                         # Add new discoveries
  v0.0.2-CHECKLIST.md                  # Mark items complete
  
CONTRIBUTING.md                        # Update error message conventions
```

---

## 🔧 Implementation Tasks

### Phase 1: Edge Case Tests (Estimated: 1 day)

#### Task 1.1: Empty Script Files Test

**Priority**: High  
**Estimated Time**: 2 hours  
**Files**:

- `crates/compiler/src/tests/edge_cases_empty.rs` (NEW)

**Implementation**:

```rust
#[test]
fn test_empty_file_lexer() {
    let input = "";
    let tokens = lex(input);
    assert_eq!(tokens.len(), 0); // Or EOF token only
}

#[test]
fn test_empty_file_parser() {
    let input = "";
    let ast = parse(input);
    assert!(ast.is_ok());
    assert_eq!(ast.unwrap().statements.len(), 0);
}

#[test]
fn test_empty_file_runtime() {
    let input = "";
    let result = execute(input);
    assert!(result.is_ok());
}
```

**Validation**: All 3 tests must pass; no panics or unwrap failures.

#### Task 1.2: Comment-Only Files Test

**Priority**: High  
**Estimated Time**: 2 hours  
**Files**:

- `crates/compiler/src/tests/edge_cases_comments.rs` (NEW)

**Implementation**:

```rust
#[test]
fn test_single_line_comment_only() {
    let input = "// Just a comment";
    let ast = parse(input);
    assert!(ast.is_ok());
    assert_eq!(ast.unwrap().statements.len(), 0);
}

#[test]
fn test_multiple_comments_only() {
    let input = "// Comment 1\n// Comment 2\n// Comment 3";
    let ast = parse(input);
    assert!(ast.is_ok());
}

#[test]
fn test_comments_with_whitespace() {
    let input = "\n\n  // Comment\n\n";
    let ast = parse(input);
    assert!(ast.is_ok());
}
```

**Validation**: All tests must pass; verify lexer skips comments correctly.

#### Task 1.3: Long Variable Names Test

**Priority**: High  
**Estimated Time**: 3 hours  
**Files**:

- `crates/compiler/src/tests/edge_cases_long_identifiers.rs` (NEW)

**Implementation**:

```rust
#[test]
fn test_long_variable_name_1000_chars() {
    let long_name = "a".repeat(1000);
    let input = format!("let {} = 42;", long_name);
    let ast = parse(&input);
    assert!(ast.is_ok());
}

#[test]
fn test_long_function_name() {
    let long_name = "function_".to_string() + &"x".repeat(990);
    let input = format!("fn {}() {{ return 1; }}", long_name);
    let result = compile_and_execute(&input);
    assert!(result.is_ok());
}

#[test]
fn test_long_variable_resolution() {
    let long_name = "my_variable_".to_string() + &"z".repeat(988);
    let input = format!("let {} = 10;\nlet result = {} + 5;", long_name, long_name);
    let result = execute(&input);
    assert_eq!(result.unwrap(), 15);
}
```

**Validation**: All tests pass; document any performance concerns.

---

### Phase 2: Error Message Improvements ✅ **COMPLETE** (PR #12)

**Status**: ✅ Completed October 3, 2025  
**Actual Time**: 3 hours (70% faster than 10h estimate)  
**Deliverables**: All 31 compiler errors now include line/column information

#### Task 2.1: Add Position Tracking to Lexer

**Priority**: High  
**Estimated Time**: 4 hours  
**Files**:

- `crates/compiler/src/lexer.rs` (MODIFY)

**Implementation**:

- Add `line: usize` and `column: usize` to `Token` struct
- Update lexer to track current line and column during tokenization
- Include position in all error messages

**Validation**: Verify all lexer errors include line/column.

#### Task 2.2: Improve Parser Error Messages

**Priority**: High  
**Estimated Time**: 6 hours  
**Files**:

- `crates/compiler/src/parser.rs` (MODIFY)
- `crates/compiler/src/tests/error_messages.rs` (NEW)

**Implementation**:

- Extract context (±2 lines) from source for error messages
- Format errors with visual indicators (e.g., `^` pointer)
- Add tests to validate error message format

**Example Error Format**:

```
Error: Unexpected token '}' at line 5, column 12

  3 | fn add(a: i32, b: i32) -> i32 {
  4 |     let result = a + b
  5 | }
    |            ^ Expected ';' before '}'
```

**Validation**: Tests must verify exact error format.

#### Task 2.3: Add Type Checker Error Context

**Priority**: Medium  
**Estimated Time**: 3 hours  
**Files**:

- `crates/compiler/src/type_checker.rs` (MODIFY)

**Implementation**:

- Include type information in error messages
- Show expected vs. actual types
- Add source context

**Validation**: Type mismatch errors must show both types clearly.

#### Task 2.4: Add Runtime Stack Traces

**Priority**: Medium  
**Estimated Time**: 4 hours  
**Files**:

- `crates/runtime/src/lib.rs` (MODIFY)

**Implementation**:

- Track call stack during execution
- Include stack trace in runtime error messages
- Show line numbers for each frame

**Validation**: Runtime errors must include call stack.

---

### Phase 3: Source Context Display ✅ **COMPLETE** (PR #13)

**Status**: ✅ Completed October 4, 2025  
**Actual Time**: ~4 hours  
**Deliverables**: All 38 compiler errors (6 lexer + 14 parser + 18 type checker) now display:
- ±2 lines of source context around error location
- Visual pointer (^) indicating exact error column
- Helpful hints explaining what's expected
- 17 comprehensive integration tests validating error display

**Test Coverage**: 182 tests passing (+71 since baseline)

---

### Phase 4: Colorized Output (Estimated: 1 day) - DEFERRED

#### Task 3.1: Add ANSI Color Support

**Priority**: Medium  
**Estimated Time**: 3 hours  
**Files**:

- `crates/compiler/src/error.rs` (NEW or MODIFY)

**Implementation**:

- Use `colored` or `termcolor` crate for ANSI colors
- Implement color scheme: Red (errors), Yellow (warnings), Cyan (hints)
- Add environment variable check: `NO_COLOR` or `FERRIS_COLOR=off`

**Validation**: Errors must display in color on TTY.

#### Task 3.2: Godot Console Integration

**Priority**: Medium  
**Estimated Time**: 4 hours  
**Files**:

- `crates/godot_bind/src/lib.rs` (MODIFY)

**Implementation**:

- Use Godot's `push_error` with BBCode for colorization
- Map ANSI colors to Godot BBCode tags
- Test in Godot editor console

**Validation**: Errors must display colorized in Godot console.

---

### Phase 4: Error Recovery (Estimated: 2 days)

#### Task 4.1: Implement Parser Error Recovery

**Priority**: High  
**Estimated Time**: 8 hours  
**Files**:

- `crates/compiler/src/parser.rs` (MODIFY)
- `crates/compiler/src/tests/error_recovery.rs` (NEW)

**Implementation**:

- Use panic-mode recovery: skip tokens until synchronization point
- Synchronization points: `;`, `}`, `fn`, `let`
- Collect all errors in a `Vec<ParseError>`
- Return partial AST marked as incomplete

**Validation**: Multiple errors must be reported in one pass.

#### Task 4.2: Add Error Recovery Tests

**Priority**: High  
**Estimated Time**: 4 hours  
**Files**:

- `crates/compiler/src/tests/error_recovery.rs` (NEW)

**Implementation**:

```rust
#[test]
fn test_multiple_syntax_errors() {
    let input = r#"
        fn test() {
            let x = 10  // Missing semicolon
            let y = 20  // Missing semicolon
            return x + y;
        }
    "#;
    let result = parse(input);
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 2); // Both errors reported
}
```

**Validation**: Test must collect all errors.

---

### Phase 5: Error Message Validation (Estimated: 1 day)

#### Task 5.1: Add Error Message Tests

**Priority**: High  
**Estimated Time**: 4 hours  
**Files**:

- `crates/compiler/src/tests/error_messages.rs` (MODIFY)

**Implementation**:

- Test exact error message text
- Verify line/column numbers
- Validate error context formatting

**Validation**: All error tests must use `assert_eq!` for messages.

#### Task 5.2: Document Error Message Formats

**Priority**: Medium  
**Estimated Time**: 3 hours  
**Files**:

- `CONTRIBUTING.md` (MODIFY)
- `docs/ERROR_MESSAGE_GUIDE.md` (NEW)

**Implementation**:

- Document standard error message format
- Provide examples for each error type
- Add guidelines for writing new error messages

**Validation**: Documentation review.

---

## 📊 Progress Tracking

### Status Table

| Phase | Task | Status | Estimated | Actual | Notes |
|-------|------|--------|-----------|--------|-------|
| 1 | Empty Files Test | ✅ Complete | 2h | 1h | 4 tests passing |
| 1 | Comment-Only Test | ✅ Complete | 2h | 1h | 5 tests passing |
| 1 | Long Identifiers Test | ✅ Complete | 3h | 1.5h | 6 tests passing |
| 2 | Lexer Position Tracking | ⬜ Not Started | 4h | - | |
| 2 | Parser Error Messages | ⬜ Not Started | 6h | - | |
| 2 | Type Checker Context | ⬜ Not Started | 3h | - | |
| 2 | Runtime Stack Traces | ⬜ Not Started | 4h | - | |
| 3 | ANSI Color Support | ⬜ Not Started | 3h | - | |
| 3 | Godot Console Integration | ⬜ Not Started | 4h | - | |
| 4 | Parser Error Recovery | ⬜ Not Started | 8h | - | |
| 4 | Error Recovery Tests | ⬜ Not Started | 4h | - | |
| 5 | Error Message Tests | ⬜ Not Started | 4h | - | |
| 5 | Documentation | ⬜ Not Started | 3h | - | |

**Total Estimated Time**: 50 hours (~6-7 working days)

**Legend**: ⬜ Not Started | 🟡 In Progress | ✅ Complete | ❌ Blocked

---

## 🔍 Contribution & Development Compliance

### CONTRIBUTING.md Rules

- ✅ Use feature branch: `feature/edge-case-error-handling`
- ✅ Write tests for all new functionality
- ✅ Run `cargo test --workspace` before committing
- ✅ Run `cargo clippy --workspace -- -D warnings` before committing
- ✅ Run `npm run docs:fix` for markdown files
- ✅ Write clear commit messages: `type(scope): description`
- ✅ Update CHANGELOG.md with changes
- ✅ Update documentation for user-facing changes

### DEVELOPMENT.md Rules

- ✅ Test coverage: Aim for 80%+ line coverage
- ✅ Benchmarks: Run benchmarks if performance-sensitive
- ✅ Documentation: Add rustdoc comments for public APIs
- ✅ Error handling: All errors must be handled gracefully
- ✅ Code style: Follow Rust conventions and clippy suggestions

### Copilot-Specific Rules

- ✅ Use chat TODO lists for visibility
- ✅ Document all decisions and trade-offs
- ✅ Reference related issues and PRs
- ✅ Update related documentation atomically
- ✅ Validate markdown linting before committing

---

## 🧪 Testing Strategy

### Test Categories

1. **Unit Tests**: Test individual functions (lexer, parser, type checker)
2. **Integration Tests**: Test full pipeline (source → runtime)
3. **Error Tests**: Validate error messages and recovery
4. **Regression Tests**: Ensure fixes don't break existing functionality

### Test Validation Checklist

- [ ] All new tests pass
- [ ] All existing tests still pass
- [ ] Test coverage increased (target: +5-10%)
- [ ] Error messages validated with `assert_eq!`
- [ ] Performance tested (no significant degradation)
- [ ] Cross-platform testing (Windows, Linux, macOS)

---

## 📝 Documentation Updates Required

### Files to Update

1. **docs/v0.0.2/EDGE_CASE_ERROR_HANDLING_SUMMARY.md** (NEW)
   - Workstream summary
   - Learnings and discoveries
   - Recommendations for future work

2. **docs/v0.0.2/TEST_COVERAGE_ANALYSIS.md** (UPDATE)
   - Mark completed gaps as ✅
   - Update test counts
   - Update coverage estimates

3. **docs/v0.0.2/LEARNINGS.md** (UPDATE)
   - Add new discoveries from error handling work
   - Document any limitations or trade-offs

4. **docs/v0.0.2/v0.0.2-CHECKLIST.md** (UPDATE)
   - Mark completed items as ✅
   - Update progress tracking

5. **CONTRIBUTING.md** (UPDATE)
   - Add error message conventions
   - Document testing requirements for error handling

6. **CHANGELOG.md** (UPDATE)
   - Add entry for v0.0.2 changes
   - List all new tests and features

---

## 🎯 Success Metrics

### Quantitative

- ✅ All 13 new tests passing (3 edge cases + 10 error handling)
- ✅ Test coverage increased by 5-10%
- ✅ All error messages include line numbers
- ✅ Error recovery works for 2+ errors per file
- ✅ All markdown files pass linting

### Qualitative

- ✅ Error messages are clear and actionable
- ✅ Developers can quickly identify and fix errors
- ✅ Godot console output is easy to read
- ✅ Documentation is comprehensive and accurate
- ✅ Code follows FerrisScript conventions

---

## 🚧 Known Limitations & Future Work

### Documented Limitations (from LEARNINGS.md)

1. **Large integer literals**: Parsed as f32 instead of i32 (lexer heuristic issue)
2. **Global mutable variables**: Not fully supported
3. **Bare blocks**: Not yet supported in functions
4. **Division by zero**: No proper error (needs runtime check)

### Recommendations for v0.0.3

- [ ] Fix large integer literal parsing
- [ ] Implement global mutability tracking
- [ ] Add bare block support
- [ ] Add division-by-zero runtime checks
- [ ] Consider property-based testing with `proptest`

---

## 📞 Questions & Blockers

### Pre-Implementation Questions

1. **Color scheme**: Confirm Red/Yellow/Cyan is appropriate
2. **Error limit**: Is 10 errors per file reasonable?
3. **Recovery strategy**: Should we use panic-mode or other techniques?
4. **Breaking changes**: Any concerns about error message format changes?

### Potential Blockers

- None identified at planning stage

---

## 🔗 Related Documents

- [v0.0.2 Checklist](./v0.0.2-CHECKLIST.md) - High-level checklist
- [Test Coverage Analysis](./TEST_COVERAGE_ANALYSIS.md) - Coverage gaps
- [Learnings](./LEARNINGS.md) - Previous edge case work
- [Contributing Guide](../../CONTRIBUTING.md) - Contribution rules
- [Development Guide](../../docs/DEVELOPMENT.md) - Development workflow

---

**Last Updated**: October 3, 2025  
**Status**: 🟡 Planning Complete - Ready for Implementation  
**Next Step**: Create feature branch and begin Phase 1
