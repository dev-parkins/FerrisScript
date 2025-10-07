# Phase 3C: Parser Error Recovery - Execution Plan

**Date**: January 6, 2025  
**Agent**: GitHub Copilot  
**Status**: In Progress  
**Branch**: `feature/v0.0.3-phase-3c-recovery`  
**Estimated Effort**: 6-8 hours

---

## 🎯 Mission

Implement parser error recovery so FerrisScript can continue parsing after syntax errors and report multiple errors in a single compilation pass. This eliminates the frustrating fix-compile-fix cycle and brings FerrisScript in line with modern compiler expectations.

---

## 📋 Q&A: Context Gathering

### Workstream Context

**Q1: What is the primary goal?**  
A: Implement panic-mode error recovery in the parser. When a syntax error occurs, synchronize to a safe point (statement boundary) and continue parsing to find additional errors.

**Q2: What version is this for?**  
A: v0.0.3 (patch release) - "Editor Experience Alpha"

**Q3: What type of release?**  
A: Patch release focused on error system improvements

**Q4: Why is this work important?**  
A: Matches modern compiler UX (Rust, TypeScript, Swift). Reduces debugging time by showing all errors at once. Foundation for LSP integration in Phase 4-5.

**Q5: What's the source of requirements?**  
A: `docs/planning/v0.0.3/PHASE_3_ERROR_DOCS_RECOVERY.md` - comprehensive planning document with acceptance criteria

### Prior Work Context

**Q6: Has similar work been done before?**  
A: Phases 1-2-3A-3B complete:
- Phase 1: Error code system (E001-E499)
- Phase 2: Error suggestions ("did you mean?")
- Phase 3A: Documentation URLs in errors
- Phase 3B: Jekyll site + cross-references

No prior error recovery work exists - this is new functionality.

**Q7: What are existing tests?**  
A: Parser has test module at line 761 in `parser.rs`. Tests cover valid syntax parsing. Need to add error recovery tests.

**Q8: What documentation exists?**  
A: Full planning doc, v0.0.3 roadmap, LEARNINGS.md from previous phases

**Q9: What patterns should I follow?**  
A: Existing parser uses `Result<T, String>` for error handling, test modules use `#[cfg(test)]`, tests use `tokenize()` + `parse()` pattern

**Q10: What should I NOT change?**  
A: Existing parser behavior for valid syntax. Current `parse()` API signature. Error formatting from Phase 1-2-3A.

### User Decisions (from Q&A)

**Scope Strategy**: ✅ **Option C** - Phase 3C only (parser recovery), separate PR  
**Synchronization Tokens**: ✅ `;`, `}`, `fn`, `let` (defer `if`, `while`, `return` for now)  
**Cascading Error Strategy**: ✅ Conservative - suppress errors while in panic mode  
**Test Coverage Target**: ✅ 80%+ of new code paths (following project standard)  
**Diagnostic Struct**: ✅ Defer to Phase 3D (keep 3C focused)  
**Parser API**: ✅ Keep `Result<Program, String>` (no breaking changes in 3C)  
**LEARNINGS.md**: ✅ Update at end with discoveries  
**Branch Name**: ✅ `feature/v0.0.3-phase-3c-recovery` (new branch)

---

## ✅ Acceptance Criteria

### 1. Parser Recovery Implementation

- [ ] **Add recovery fields to Parser struct**
  - [ ] `panic_mode: bool` - tracks if currently recovering from error
  - [ ] `errors: Vec<String>` - collect all errors during parsing
  - [ ] Initialize both fields in `Parser::new()`

- [ ] **Implement `synchronize()` method**
  - [ ] Skip tokens until reaching sync point
  - [ ] Sync points: `;`, `}`, `fn`, `let`
  - [ ] Clear panic mode when sync point found
  - [ ] Handle EOF correctly (don't infinite loop)

- [ ] **Add `record_error()` helper method**
  - [ ] Store error in `self.errors` vector
  - [ ] Set `panic_mode = true`
  - [ ] Suppress duplicate errors while in panic mode

### 2. Error Handling Modifications

- [ ] **Update `parse_program()` top-level parser**
  - [ ] On error: record error, synchronize, continue
  - [ ] Return first error if any (API compatibility)
  - [ ] Log all collected errors (for future multi-error reporting)

- [ ] **Update statement parsing methods**
  - [ ] `parse_statement()`: recover on error
  - [ ] `parse_let_statement()`: recover on error
  - [ ] `parse_if_statement()`: recover on error
  - [ ] `parse_while_statement()`: recover on error
  - [ ] `parse_return_statement()`: recover on error

- [ ] **Update expression parsing (conservative)**
  - [ ] Don't change expression parsing yet (complex, separate concern)
  - [ ] Statement-level recovery is sufficient for Phase 3C

### 3. Testing Requirements

- [ ] **Unit tests for recovery mechanism**
  - [ ] Test `synchronize()` finds semicolon
  - [ ] Test `synchronize()` finds closing brace
  - [ ] Test `synchronize()` finds `fn` keyword
  - [ ] Test `synchronize()` finds `let` keyword
  - [ ] Test `synchronize()` handles EOF gracefully

- [ ] **Integration tests for common errors**
  - [ ] Missing semicolon: `let x = 5 let y = 10;`
  - [ ] Unclosed brace: `fn test() { let x = 5`
  - [ ] Invalid token: `fn test() { @ }`
  - [ ] Multiple independent errors: combine above

- [ ] **Cascading error prevention tests**
  - [ ] Verify no false errors reported while in panic mode
  - [ ] Verify parser exits panic mode correctly
  - [ ] Test with deeply nested structures

- [ ] **Edge case tests**
  - [ ] Empty file (no errors)
  - [ ] File with only errors (all statements fail)
  - [ ] Valid code after error (recovery works)
  - [ ] Multiple errors in sequence

### 4. Quality Standards

- [ ] **All existing tests pass** (`cargo test --workspace`)
- [ ] **New tests cover recovery** (80%+ branch coverage)
- [ ] **Clippy clean** (`cargo clippy --workspace --all-targets --all-features -- -D warnings`)
- [ ] **Formatting verified** (`cargo fmt --all`)
- [ ] **Documentation linting** (`npm run docs:lint`)
- [ ] **No performance regression** (run parser benchmarks)

---

## 🏗️ Implementation Plan

### Phase 0: Setup ✅ (COMPLETE)

- [x] Review planning documentation
- [x] Ask clarifying questions
- [x] Get user decisions
- [x] Create execution plan
- [x] Create TODO list

### Phase 1: Add Recovery Infrastructure (1-2h)

**Tasks**:
1. Create new branch `feature/v0.0.3-phase-3c-recovery`
2. Add `panic_mode: bool` field to `Parser` struct
3. Add `errors: Vec<String>` field to `Parser` struct
4. Update `Parser::new()` to initialize new fields
5. Implement `synchronize()` method
6. Implement `record_error()` helper method

**Deliverables**:
- Modified `crates/compiler/src/parser.rs`
- Code compiles, existing tests still pass

**Validation**:
```bash
cargo build --workspace
cargo test --package ferrisscript_compiler
```

### Phase 2: Modify Error Handling (2-3h)

**Tasks**:
1. Update `parse_program()` to use recovery
2. Update `parse_statement()` to use recovery
3. Update individual statement parsing methods
4. Test that parser continues after errors
5. Verify first error still returned for API compatibility

**Deliverables**:
- Modified error handling in parser methods
- Parser collects multiple errors
- Existing API maintained

**Validation**:
```bash
cargo test --package ferrisscript_compiler
cargo clippy --package ferrisscript_compiler
```

### Phase 3: Write Recovery Tests (2-3h)

**Tasks**:
1. Write unit tests for `synchronize()`
2. Write integration tests for common errors
3. Write tests for cascading error prevention
4. Write edge case tests
5. Verify 80%+ coverage of new code

**Deliverables**:
- New test module or expanded existing tests
- All tests pass
- Edge cases covered

**Validation**:
```bash
cargo test --workspace
# Check specific test output
cargo test --package ferrisscript_compiler recovery -- --nocapture
```

### Phase 4: Quality Assurance (1h)

**Tasks**:
1. Run full test suite
2. Run clippy with strict mode
3. Run formatter
4. Run documentation linting
5. Review git diff for unintended changes
6. Run parser benchmarks (ensure no regression)

**Deliverables**:
- All quality checks pass
- No performance regression
- Clean git diff

**Validation**:
```bash
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
npm run docs:lint
cargo bench --package ferrisscript_compiler -- parser
```

### Phase 5: Documentation & PR (1h)

**Tasks**:
1. Update `LEARNINGS.md` with Phase 3C discoveries
2. Update phase tracking in `README.md`
3. Create summary document
4. Self-review all changes
5. Create PR with comprehensive description

**Deliverables**:
- Updated `docs/planning/v0.0.3/LEARNINGS.md`
- Updated `docs/planning/v0.0.3/README.md`
- New `PHASE_3C_SUMMARY.md`
- PR ready for review

**Validation**:
- All documentation links work
- PR description is comprehensive
- Summary document follows template

---

## 📦 Deliverables

### Code Changes
- **Modified**: `crates/compiler/src/parser.rs`
  - New fields: `panic_mode`, `errors`
  - New methods: `synchronize()`, `record_error()`
  - Modified: `parse_program()`, `parse_statement()`, statement parsing methods

### Tests
- **New tests in `parser.rs`**:
  - `test_synchronize_on_semicolon()`
  - `test_synchronize_on_brace()`
  - `test_synchronize_on_keyword()`
  - `test_recovery_missing_semicolon()`
  - `test_recovery_unclosed_brace()`
  - `test_recovery_multiple_errors()`
  - `test_no_cascading_errors()`
  - (8-12 new test functions)

### Documentation
- **Updated**: `docs/planning/v0.0.3/LEARNINGS.md`
- **Updated**: `docs/planning/v0.0.3/README.md`
- **Updated**: `docs/planning/v0.0.3/PHASE_3_ERROR_DOCS_RECOVERY.md`
- **New**: `docs/planning/v0.0.3/PHASE_3C_SUMMARY.md`

---

## 🔬 Technical Approach

### Recovery Algorithm (Panic-Mode)

```rust
impl<'a> Parser<'a> {
    /// Synchronize parser to next safe recovery point
    fn synchronize(&mut self) {
        self.panic_mode = true;
        
        while !self.is_at_end() {
            // Found semicolon - statement boundary
            if matches!(self.previous(), Token::Semicolon) {
                self.panic_mode = false;
                return;
            }
            
            // Found keyword or brace - likely safe point
            match self.current() {
                Token::Fn | Token::Let | Token::RightBrace => {
                    self.panic_mode = false;
                    return;
                }
                _ => { self.advance(); }
            }
        }
        
        // Reached EOF
        self.panic_mode = false;
    }
    
    /// Record error without immediately returning
    fn record_error(&mut self, error: String) {
        // Don't record cascading errors
        if !self.panic_mode {
            self.errors.push(error);
            self.panic_mode = true;
        }
    }
}
```

### Modified Error Handling Pattern

**Before** (immediate return):
```rust
fn parse_statement(&mut self) -> Result<Stmt, String> {
    match self.current() {
        Token::Let => self.parse_let_statement(),
        _ => Err("Expected statement")  // Stops parsing
    }
}
```

**After** (recover and continue):
```rust
fn parse_statement(&mut self) -> Option<Stmt> {
    match self.current() {
        Token::Let => {
            match self.parse_let_statement() {
                Ok(stmt) => Some(stmt),
                Err(e) => {
                    self.record_error(e);
                    self.synchronize();
                    None  // Continue parsing
                }
            }
        }
        _ => {
            self.record_error("Expected statement");
            self.synchronize();
            None
        }
    }
}
```

---

## 🚫 Out of Scope (Deferred)

### Deferred to Phase 3D
- Multi-error reporting with `Diagnostic` struct
- Batch vs stream reporting modes
- CLI flags for error reporting control
- Proper multi-error API (`Vec<Diagnostic>`)

### Deferred to Future Work
- Expression-level recovery (complex)
- Advanced recovery strategies (context-sensitive)
- Recovery confidence scoring
- Speculative parsing

### Explicitly NOT Changing
- Error formatting (from Phase 1-2-3A)
- Documentation URLs (from Phase 3A)
- Error suggestions (from Phase 2)
- Public `parse()` API signature

---

## 📊 Success Metrics

### Quantitative
- ✅ All existing tests pass (270+ tests)
- ✅ 8-12 new tests added
- ✅ 80%+ coverage of new recovery code
- ✅ Zero clippy warnings
- ✅ Zero formatting issues
- ✅ No performance regression (< 5% slowdown)

### Qualitative
- ✅ Parser continues after syntax errors
- ✅ Multiple errors collected correctly
- ✅ No cascading false positives
- ✅ Code is clear and maintainable
- ✅ Error recovery is predictable

---

## 🎯 Quality Gates

Before marking Phase 3C complete:

- [ ] All tests pass: `cargo test --workspace`
- [ ] Clippy strict: `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] Formatting: `cargo fmt --all -- --check`
- [ ] Documentation: `npm run docs:lint`
- [ ] Benchmarks: No regression > 5%
- [ ] Self-review: Clean git diff
- [ ] Documentation: LEARNINGS.md updated
- [ ] Summary: PHASE_3C_SUMMARY.md created

---

## 💡 Expected Learnings

Things I anticipate discovering during implementation:

1. **Best sync points**: Are `;`, `}`, `fn`, `let` sufficient? Or need more?
2. **Panic mode duration**: How long do we stay in panic mode typically?
3. **False positive rate**: How effective is suppression strategy?
4. **Edge cases**: What unexpected recovery scenarios arise?
5. **Performance impact**: Does error collection slow down parsing?

These will be documented in LEARNINGS.md and summary document.

---

## 🔗 Related Documents

- **Planning**: [PHASE_3_ERROR_DOCS_RECOVERY.md](./PHASE_3_ERROR_DOCS_RECOVERY.md)
- **Roadmap**: [v0.0.3-roadmap.md](./v0.0.3-roadmap.md)
- **Phase Tracking**: [README.md](./README.md)
- **Prior Learnings**: [LEARNINGS.md](./LEARNINGS.md)

---

**Status**: Ready to begin Phase 1 implementation 🚀  
**Next Action**: Create branch and add recovery infrastructure to Parser
