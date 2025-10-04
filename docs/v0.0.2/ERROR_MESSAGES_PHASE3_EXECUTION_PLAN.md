# Error Message Improvements (Phase 3) - Execution Plan

**Workstream**: FerrisScript v0.0.2 Error Handling - Phase 3  
**Date**: October 3, 2025  
**Agent**: GitHub Copilot  
**Status**: Planning → In Progress  
**Branch**: `feature/error-messages-phase3`  
**Target PR**: #13 (TBD)  
**Dependencies**: PR #12 (Phase 2 - line/column tracking)

---

## 📋 Executive Summary

This workstream implements Phase 3 of the error handling improvements from `EDGE_CASE_ERROR_HANDLING_PLAN.md`. Phase 2 (line/column tracking) added position information to all 31 compiler errors. Phase 3 focuses on adding **source context display** (±2 lines around the error) with **visual indicators** (e.g., `^` pointers) to make errors more actionable.

**Scope**: Source context display only (no colorization, no error recovery - those are Phase 4-5)

**Key Challenge**: Must pass source string through compilation pipeline to enable context extraction at error time.

---

## Q&A: Context Gathering

### Workstream Context

**Q1: What is the primary goal?**
A: Add source code context (±2 lines) to all compiler error messages with visual indicators showing exactly where the error occurred.

**Q2: What version is this for?**
A: v0.0.2 (Patch Release)

**Q3: What type of release?**
A: Patch release - error message improvements are acceptable for pre-1.0, no breaking API changes

**Q4: Why is this work important?**
A: Currently errors only show "at line X, column Y" but users can't see what code caused the error without switching to their editor. Context display makes errors instantly actionable.

**Q5: What's the source of requirements?**
A: EDGE_CASE_ERROR_HANDLING_PLAN.md (Task 2.2: "Extract context ±2 lines from source for error messages")

### Prior Work

**Q1: Has similar work been done before?**
A: Phase 2 (PR #12) added line/column tracking to all 31 errors. Infrastructure exists:

- ✅ Lexer tracks `self.line` and `self.column`
- ✅ Parser has `self.current_line` and `self.current_column`
- ✅ All errors include position: "at line X, column Y"
- ❌ Source strings NOT currently passed through pipeline (need to add)

**Q2: Are there existing tests?**
A: 153 tests total (111 original + 15 Phase 1 + 22 Phase 2 + 5 others). Phase 2 has error message validation tests.

**Q3: What documentation exists?**
A:

- EDGE_CASE_ERROR_HANDLING_PLAN.md - Original multi-phase plan
- ERROR_MESSAGES_PHASE2_EXECUTION_PLAN.md - Line/column tracking (completed)
- ERROR_MESSAGES_PHASE2_SUMMARY.md - Phase 2 completion summary

**Q4: What patterns should I follow?**
A: Example error format from plan (EDGE_CASE_ERROR_HANDLING_PLAN.md lines 303-313):

```
Error: Unexpected token '}' at line 5, column 12

  3 | fn add(a: i32, b: i32) -> i32 {
  4 |     let result = a + b
  5 | }
    |            ^ Expected ';' before '}'
```

**Q5: What should I NOT change?**
A: Don't modify AST structure, token types, or compilation behavior. Only enhance error message display.

### Constraints

**Q1: What changes are allowed?**
A:

- Add source context to error messages
- Modify function signatures to accept source string
- Create helper functions for context extraction
- Update tests to verify context display

**Q2: What changes are NOT allowed?**
A:

- No new features beyond error display
- No breaking changes to public API (internal changes OK)
- No colorization (that's Phase 4)
- No error recovery (that's Phase 5)

**Q3: Are there performance requirements?**
A: Context extraction should be minimal overhead (only happens on errors, not happy path)

**Q4: Are there platform considerations?**
A: Must work on Windows, Linux, macOS (line ending handling for \n vs \r\n)

**Q5: What's the timeline?**
A: Medium priority for v0.0.2, estimated 6-8 hours (per original plan: Task 2.2 = 6h)

### Quality Standards

**Q1: What tests must pass?**
A: `cargo test --workspace` (all 153 tests + new context tests)

**Q2: What linting must pass?**
A: `cargo clippy --workspace -- -D warnings`, `cargo fmt --check`, `npm run docs:lint`

**Q3: What's the test coverage target?**
A: Maintain or improve current coverage (~70-75%)

**Q4: What's the documentation requirement?**
A: Update TEST_COVERAGE_ANALYSIS.md, v0.0.2-CHECKLIST.md, EDGE_CASE_ERROR_HANDLING_PLAN.md, create summary

**Q5: What's the code review process?**
A: Self-review git diff, run all quality checks, create PR with detailed description

### Contribution Workflow

**Q1: What branch should I create?**
A: ✅ Created: `feature/error-messages-phase3` (branched from `feature/error-messages-phase2`)

**Q2: What's the commit message format?**
A: Conventional commits: `feat(compiler): add source context display to error messages`

**Q3: Where should files go?**
A: Modify existing compiler files, add tests to `crates/compiler/tests/error_context.rs`

**Q4: What documents need updating?**
A: TEST_COVERAGE_ANALYSIS.md, v0.0.2-CHECKLIST.md, EDGE_CASE_ERROR_HANDLING_PLAN.md, create SUMMARY.md

**Q5: How should I track progress?**
A: Using GitHub Copilot TODO list (already created with 8 phases)

---

## 🎯 Acceptance Criteria

### AC-1: Source Context Display

**AC-1.1**: All lexer errors must display ±2 lines of source context  
**AC-1.2**: All parser errors must display ±2 lines of source context  
**AC-1.3**: All type checker errors must display ±2 lines of source context  
**AC-1.4**: Context must handle edge cases (error on line 1, error on last line, files with <3 lines)  
**AC-1.5**: Line numbers must be displayed in context (e.g., "  3 | fn add() {")

### AC-2: Visual Indicators

**AC-2.1**: Errors must include a pointer line (e.g., "    |     ^ Expected ';'")  
**AC-2.2**: Pointer must align correctly with column position  
**AC-2.3**: Pointer line must include helpful error message text  
**AC-2.4**: Multi-column errors should show span with multiple carets if needed (stretch goal)

### AC-3: API Design

**AC-3.1**: Compilation functions must accept source string parameter  
**AC-3.2**: Source string should be stored/passed efficiently (Arc<str> or &str with lifetime)  
**AC-3.3**: API changes should be backward-compatible where possible  
**AC-3.4**: Helper functions should be reusable across lexer/parser/type_checker

### AC-4: Testing

**AC-4.1**: New tests must verify context appears in error messages  
**AC-4.2**: Tests must verify pointer alignment is correct  
**AC-4.3**: Tests must cover edge cases (line 1, last line, short files)  
**AC-4.4**: All existing 153 tests must still pass  
**AC-4.5**: Test coverage must not decrease

### AC-5: Quality

**AC-5.1**: All tests pass: `cargo test --workspace`  
**AC-5.2**: Clippy clean: `cargo clippy --workspace -- -D warnings`  
**AC-5.3**: Formatted: `cargo fmt --check`  
**AC-5.4**: Documentation lint passes: `npm run docs:lint`  
**AC-5.5**: No performance regression (context extraction only on error path)

---

## 📂 Execution Phases

### Phase 0: Planning & Context Gathering ✅ → 🔄

**Estimated**: 1 hour  
**Status**: In Progress

- [x] Create execution plan document
- [x] Ask clarifying questions (answered above)
- [x] Define acceptance criteria
- [x] Create TODO list for visibility
- [x] Review Phase 2 work to understand current state
- [ ] Complete planning phase and begin audit

---

### Phase 3.1: Audit Current Error Messages

**Estimated**: 1 hour  
**Status**: ✅ Complete

**Tasks**:

- [x] Review all error generation in lexer.rs (6 errors)
- [x] Review all error generation in parser.rs (14 errors)
- [x] Review all error generation in type_checker.rs (18 errors)
- [x] Document current error message format patterns
- [x] Identify function signatures that need modification (to accept source)
- [x] Note any existing helper functions for error formatting

**Audit Results**:

#### Lexer Errors (6 total)

All errors already include `line` and `column`. Format: `"[Message] at line {}, column {}"`

1. Line 196: Unterminated string
2. Line 229: Invalid escape sequence
3. Line 235: Unterminated string after escape
4. Line 358: Unexpected character '&'
5. Line 371: Unexpected character '|'
6. Line 411: Unexpected character

**Current API**: `pub fn tokenize(input: &str) -> Result<Vec<Token>, String>`  
**Source Available**: ✅ Yes - `input` is the source string

#### Parser Errors (14 total)

All errors include `line` and `column`. Format: `"[Message] at line {}, column {}"`

All 14 errors found in parser.rs (updated in Phase 2):

- Lines 45, 69, 95, 109, 141, 158, 172, 203, 212, 306, 320, 421, 519, 555

**Current API**: `pub fn parse(tokens: &[Token]) -> Result<Program, String>`  
**Source Available**: ❌ No - only receives tokens, needs source parameter added

#### Type Checker Errors (18 total)

All errors include span position. Format: `"[Message] at [Span]"` or `"[Message] at line {}, column {}"`

18 calls to `self.error()` found at lines:

- 129, 140, 215, 223, 243, 259, 283, 319, 340, 361, 374, 391, 401, 414, 427, 440, 451, 465

**Current API**: `pub fn check(program: &Program) -> Result<(), String>`  
**Source Available**: ❌ No - only receives AST, needs source parameter added

#### Function Signature Changes Needed

1. **Parser**: Add source parameter
   - `parse(tokens: &[Token], source: &str)`
   - Pass to `Parser::new()`

2. **Type Checker**: Add source parameter
   - `check(program: &Program, source: &str)`
   - Pass to `TypeChecker` struct

3. **Compile**: Already has source, pass through
   - `compile(source: &str)` → pass to `parse()` and `check()`

#### No Existing Helper Functions

❌ No error formatting helpers exist yet - will create in Phase 3.2

---

### Phase 3.2: Design Source Context API

**Estimated**: 1.5 hours  
**Status**: ✅ Complete

**Tasks**:

- [x] Design function signature changes (lexer, parser, type_checker)
- [x] Choose source string storage strategy (&str - simplest, source already available)
- [x] Create helper function: `extract_source_context(source: &str, line: usize) -> String`
- [x] Create helper function: `format_error_pointer(column, line_num_width, hint) -> String`
- [x] Create comprehensive helper: `format_error_with_context()`
- [x] Write complete module with documentation and tests (11 tests, all passing!)

**Deliverables**:

- ✅ `crates/compiler/src/error_context.rs` created (269 lines)
- ✅ 11 comprehensive tests covering all edge cases
- ✅ Module added to lib.rs and compiling

**Design Decisions**:

- ✅ Use `&str` references (source already available in lexer, will pass through for parser/type_checker)
- ✅ Helper module keeps error formatting code DRY
- ✅ Edge cases handled: line 1, last line, files with <3 lines, empty files
- ✅ Line number alignment for files with 99+ lines
- ✅ Pointer alignment accounts for line number width

**Function Signatures Needed**:

1. Parser: Add source to `Parser::new()` (will store as field)
2. Type Checker: Add source parameter to `check()` and `TypeChecker::new()`
3. Compile: Pass source to `parse()` and `check()`

---

### Phase 3.3: Implement Lexer Context Display

**Estimated**: 1 hour  
**Status**: Not Started

**Tasks**:

- [ ] Update `Lexer::new()` to store source reference
- [ ] Update all 7 lexer errors to call `extract_source_context()`
- [ ] Format errors with context + pointer line
- [ ] Test manually with a simple lexer error
- [ ] Run: `cargo test --package ferrisscript_compiler --lib lexer`

**Files Modified**:

- `crates/compiler/src/lexer.rs`

**Example Before**:

```
Invalid number '123.456.789' at line 5, column 10
```

**Example After**:

```
Invalid number '123.456.789' at line 5, column 10

  4 | let x = 100;
  5 | let y = 123.456.789;
    |          ^ Invalid number format
  6 | return y;
```

---

### Phase 3.4: Implement Parser Context Display

**Estimated**: 2 hours  
**Status**: Not Started

**Tasks**:

- [ ] Update `Parser::new()` to store source reference
- [ ] Update all 14 parser errors to use context helper
- [ ] Ensure pointer alignment is correct for column positions
- [ ] Test with multiple parser error scenarios
- [ ] Run: `cargo test --package ferrisscript_compiler --lib parser`

**Files Modified**:

- `crates/compiler/src/parser.rs`

**Example**:

```
Expected ';', found '}' at line 5, column 12

  3 | fn add(a: i32, b: i32) -> i32 {
  4 |     let result = a + b
  5 | }
    |            ^ Expected ';' before '}'
```

---

### Phase 3.5: Implement Type Checker Context

**Estimated**: 1 hour  
**Status**: Not Started

**Tasks**:

- [ ] Update `type_check()` to accept source parameter
- [ ] Update all 10 type checker errors with context
- [ ] Test type mismatch errors show context
- [ ] Run: `cargo test --package ferrisscript_compiler --lib type_checker`

**Files Modified**:

- `crates/compiler/src/type_checker.rs`

**Example**:

```
Type mismatch at line 12, column 15: expected i32, found f32

  11 | let x: i32 = 10;
  12 | let y: i32 = 3.14;
     |              ^^^^ Cannot assign f32 to i32
  13 | return x + y;
```

---

### Phase 3.6: Create Validation Tests

**Estimated**: 1.5 hours  
**Status**: Not Started

**Tasks**:

- [ ] Create `crates/compiler/tests/error_context.rs`
- [ ] Test lexer errors include context (3 tests)
- [ ] Test parser errors include context (5 tests)
- [ ] Test type checker errors include context (3 tests)
- [ ] Test edge cases: line 1, last line, single-line files (4 tests)
- [ ] Test pointer alignment (2 tests)
- [ ] Run: `cargo test --test error_context`

**Files Created**:

- `crates/compiler/tests/error_context.rs` (~200 lines)

**Test Structure**:

```rust
#[test]
fn test_lexer_error_shows_context() {
    let source = "let x = 10;\nlet y = 123.456.789;\nreturn y;";
    let result = lexer::tokenize(source);
    let error = result.unwrap_err();
    
    assert!(error.contains("  2 | let y = 123.456.789;"));
    assert!(error.contains("    |          ^"));
}
```

---

### Phase 3.7: Quality Validation & Documentation

**Estimated**: 1 hour  
**Status**: Not Started

**Tasks**:

- [ ] Run full test suite: `cargo test --workspace`
- [ ] Verify all 153+ tests pass
- [ ] Run clippy: `cargo clippy --workspace -- -D warnings`
- [ ] Run formatter: `cargo fmt --all`
- [ ] Run docs lint: `npm run docs:lint`
- [ ] Update TEST_COVERAGE_ANALYSIS.md (add ~15 new tests)
- [ ] Update v0.0.2-CHECKLIST.md (mark context display complete)
- [ ] Update EDGE_CASE_ERROR_HANDLING_PLAN.md (Phase 3 status)
- [ ] Create ERROR_MESSAGES_PHASE3_SUMMARY.md
- [ ] Self-review git diff
- [ ] Commit and push to `feature/error-messages-phase3`
- [ ] Create PR #13 targeting `main`

**Files Updated**:

- `docs/v0.0.2/TEST_COVERAGE_ANALYSIS.md`
- `docs/v0.0.2/v0.0.2-CHECKLIST.md`
- `docs/v0.0.2/EDGE_CASE_ERROR_HANDLING_PLAN.md`

**Files Created**:

- `docs/v0.0.2/ERROR_MESSAGES_PHASE3_SUMMARY.md`

---

## 📦 Deliverables

### Code Changes

**Files Created**:

- `crates/compiler/src/error_context.rs` - Helper functions for context extraction
- `crates/compiler/tests/error_context.rs` - Validation tests (~15 tests)

**Files Modified**:

- `crates/compiler/src/lib.rs` - Update public API signatures
- `crates/compiler/src/lexer.rs` - Add context to 7 errors
- `crates/compiler/src/parser.rs` - Add context to 14 errors
- `crates/compiler/src/type_checker.rs` - Add context to 10 errors

### Documentation

**Files Updated**:

- `docs/v0.0.2/TEST_COVERAGE_ANALYSIS.md` - Add ~15 new context tests
- `docs/v0.0.2/v0.0.2-CHECKLIST.md` - Mark Phase 3 complete
- `docs/v0.0.2/EDGE_CASE_ERROR_HANDLING_PLAN.md` - Update Phase 3 status

**Files Created**:

- `docs/v0.0.2/ERROR_MESSAGES_PHASE3_EXECUTION_PLAN.md` - This document
- `docs/v0.0.2/ERROR_MESSAGES_PHASE3_SUMMARY.md` - Completion summary

---

## 📝 Implementation Notes

### Source Context Helper Design

**Location**: `crates/compiler/src/error_context.rs`

**Key Functions**:

```rust
/// Extract ±2 lines of context around an error location
pub fn extract_source_context(source: &str, error_line: usize) -> Vec<String> {
    // Returns lines with line numbers (e.g., "  3 | fn add() {")
    // Handles edge cases: line 1, last line, files with <3 lines
}

/// Generate pointer line with column alignment
pub fn format_error_pointer(column: usize, message: &str) -> String {
    // Returns: "    |     ^ Expected ';'"
    // Column is 1-based, needs adjustment for 0-based string indexing
}

/// Complete error formatting with context + pointer
pub fn format_error_with_context(
    base_message: &str,
    source: &str,
    line: usize,
    column: usize,
    hint: &str
) -> String {
    // Combines base message + context + pointer + hint
}
```

**Edge Cases**:

- File with 1 line: Show just that line
- File with 2 lines: Show both lines
- Error on line 1: Show lines 1-3
- Error on last line: Show last 3 lines
- Windows line endings (\r\n): Use `lines()` iterator (handles automatically)

### API Changes

**Current (Phase 2)**:

```rust
pub fn compile(input: &str) -> Result<Program, String>
pub fn parse(tokens: Vec<Token>) -> Result<Program, String>
pub fn tokenize(input: &str) -> Result<Vec<Token>, String>
```

**Proposed (Phase 3)**:

```rust
// No public API changes needed! Internal functions pass source through:
// - tokenize() already has source
// - parse() needs source added to Parser::new()
// - type_check() needs source parameter added
```

**Strategy**: Keep public API unchanged, thread source internally.

### Error Message Format Standard

**Template**:

```
[Error Type] [Details] at line [LINE], column [COLUMN]

  [LINE-2] | [source code]
  [LINE-1] | [source code]
  [LINE  ] | [source code]
    |      [spaces]^ [Hint message]
  [LINE+1] | [source code]
  [LINE+2] | [source code]
```

**Line Number Formatting**:

- Right-align line numbers for files >99 lines
- Use consistent spacing: "  3 |" vs " 42 |" vs "142 |"

**Pointer Alignment**:

- Column is 1-based (first character = column 1)
- Pointer needs: (line number width) + (space) + (|) + (space) + (column-1 spaces) + (^)
- Example: Line 5, column 12 → "    |            ^"

### Trade-offs & Decisions

#### Decision 1: Don't Span Multiple Lines (Yet)

- **Why**: Multi-line error spans (e.g., unclosed string across 10 lines) are complex
- **Trade-off**: Users see pointer at error start, not full span
- **Future**: Phase 4 or 5 could add span ranges

#### Decision 2: Use &str Lifetimes (Not Arc<str>)

- **Why**: Source strings are short-lived (single compilation), no need for reference counting
- **Trade-off**: Slightly more complex lifetime annotations
- **Benefit**: Zero allocation overhead, better performance

#### Decision 3: Extract ±2 Lines (Not ±5)

- **Why**: 5 lines context = 11 total lines = cluttered terminal
- **Trade-off**: Less context for complex errors
- **Benefit**: Errors fit on screen, easier to read

#### Decision 4: No Colorization (Yet)

- **Why**: That's Phase 4 - keep changes focused
- **Trade-off**: Errors less visually distinct
- **Benefit**: Simpler implementation, easier review

---

## 🚀 Getting Started

Phase 0 is complete! Moving to Phase 3.1 (Audit) next.

**Current Status**:

- ✅ Branch created: `feature/error-messages-phase3`
- ✅ Execution plan created
- ✅ TODO list created
- ✅ Acceptance criteria defined
- 🔄 Ready to begin audit phase

**Next Action**: Audit all error messages in lexer/parser/type_checker to understand current state.
