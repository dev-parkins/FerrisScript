# Phase 3: Error Documentation & Recovery

**Status**: Not Started  
**Priority**: High  
**Branch**: `feature/v0.0.3-error-docs`  
**Estimated Effort**: 2-3 days  
**Dependencies**: Phase 1 (Error Codes) ✅, Phase 2 (Suggestions) ✅  
**Date Started**: TBD  
**Date Completed**: TBD  
**PR**: TBD

---

## 🎯 Overview

Link compiler errors to documentation and implement resilient error recovery so the parser can continue after recoverable syntax issues.

This phase focuses on **developer experience** — improving clarity, discoverability, and continuity of the compilation process without blocking progress toward the upcoming LSP & VS Code integration (Phases 4–5).

**Strategic Value**: Professional error documentation and multi-error reporting matches expectations from modern compilers (Rust, TypeScript, Swift). Reduces debugging friction by showing all errors at once rather than forcing fix-compile-fix cycles.

---

## ✅ Acceptance Criteria

### 1. Error Documentation Links

- [ ] **Add URL generation** to ErrorCode enum
  - [ ] Method: `get_docs_url() -> String`
  - [ ] Format: Hybrid approach (configurable via env var)
    - Default: GitHub URLs (works immediately)
    - Future: `FERRIS_DOCS_BASE` env var for custom site
  - [ ] Example: `https://github.com/dev-parkins/FerrisScript/blob/main/docs/ERROR_CODES.md#e001`

- [ ] **Integrate URLs into error messages**
  - [ ] Modify `format_error_with_code()` to append URL
  - [ ] Format: `note: see <URL> for more information`
  - [ ] Test all error types include URLs

- [ ] **Enhance ERROR_CODES.md**
  - [ ] Verify all entries have: Description, Causes, Example, Fix
  - [ ] Add "See also" cross-references for related errors
  - [ ] Ensure markdown anchors work (`#e001`, `#e100`, etc.)

**Example Enhancement**:

**Before**:
```
Error[E201]: Undefined variable
Undefined variable 'velocty' at line 5, column 10
  --> move.ferris:5:10
   |
 5 |     self.velocty.x += 50.0;
   |          ^^^^^^^ not found in this scope
   |
help: did you mean 'velocity'?
```

**After**:
```
Error[E201]: Undefined variable
Undefined variable 'velocty' at line 5, column 10
  --> move.ferris:5:10
   |
 5 |     self.velocty.x += 50.0;
   |          ^^^^^^^ not found in this scope
   |
help: did you mean 'velocity'?
   = note: see https://github.com/dev-parkins/FerrisScript/blob/main/docs/ERROR_CODES.md#e201 for more information
```

*Note: URL will automatically switch to `docs.ferrisscript.dev` when site launches (via env var)*

### 2. Parser Error Recovery

- [ ] **Implement panic-mode recovery**
  - [ ] Add synchronization tokens: `;`, `}`, `fn`, `let`
  - [ ] Method: `recover_to_sync_point()`
  - [ ] Continue parsing after recoverable errors

- [ ] **Add diagnostic collection**
  - [ ] Add `Vec<Diagnostic>` to Parser struct
  - [ ] Store errors instead of returning immediately
  - [ ] Return all diagnostics at end

- [ ] **Improve diagnostic messages**
  - [ ] Add "expected X, found Y" context
  - [ ] Track expected tokens at error points
  - [ ] Prevent cascading errors from single issue

**Example Recovery**:

**Before** (stops at first error):
```rust
let x = 5  // Missing semicolon
let y = 10;
let z = unknown_var;  // Never reported
```

Shows only:
```
Error[E100]: Unexpected token
Expected ';', found 'let'
```

**After** (continues and reports all):
```
Error[E100]: Unexpected token
Expected ';' after expression at line 1
Error[E201]: Undefined variable 'unknown_var' at line 3
```

### 3. Multi-Error Reporting

- [ ] **Add diagnostic data structure**
  - [ ] Struct: `Diagnostic { code, message, span, level }`
  - [ ] Levels: Error, Warning, Note
  - [ ] Store all diagnostics during compilation

- [ ] **Implement reporting modes**
  - [ ] Batch mode: collect all, report at end (default)
  - [ ] Stream mode: report immediately (for LSP)
  - [ ] CLI flag: `--report-mode=batch|stream`

- [ ] **Format multi-error output**
  - [ ] Group by file/module
  - [ ] Sort by line number
  - [ ] Add summary: "Found X errors, Y warnings"

### 4. Testing & Validation

- [ ] **Unit tests for error recovery**
  - [ ] Test recovery after missing semicolon
  - [ ] Test recovery after unclosed brace
  - [ ] Test recovery after invalid token
  - [ ] Verify no cascading false positives

- [ ] **Integration tests for multi-error**
  - [ ] Test file with 3+ independent errors
  - [ ] Verify all errors reported
  - [ ] Test batch vs stream modes

- [ ] **Documentation URL tests**
  - [ ] Verify URLs appear in all error types
  - [ ] Test URL format correctness
  - [ ] Verify markdown anchors resolve

---

## 🏗️ Technical Approach

### Documentation URL Generation

**Location**: `crates/compiler/src/error_code.rs`

```rust
impl ErrorCode {
    /// Get documentation URL for this error code
    /// 
    /// By default, links to GitHub repository. Set FERRIS_DOCS_BASE environment
    /// variable to use a custom documentation site (e.g., when docs.ferrisscript.dev launches).
    /// 
    /// # Examples
    /// 
    /// Default (GitHub):
    /// ```
    /// let url = ErrorCode::E201.get_docs_url();
    /// // "https://github.com/dev-parkins/FerrisScript/blob/main/docs/ERROR_CODES.md#e201"
    /// ```
    /// 
    /// With custom docs site:
    /// ```bash
    /// export FERRIS_DOCS_BASE=https://docs.ferrisscript.dev
    /// ```
    /// ```
    /// let url = ErrorCode::E201.get_docs_url();
    /// // "https://docs.ferrisscript.dev/errors/E201"
    /// ```
    pub fn get_docs_url(&self) -> String {
        let code = self.code_string(); // "E201"
        let anchor = code.to_lowercase(); // "e201"
        
        // Check for custom docs base URL
        if let Ok(base) = std::env::var("FERRIS_DOCS_BASE") {
            format!("{}/errors/{}", base.trim_end_matches('/'), code)
        } else {
            // Default: GitHub main branch (works immediately, no infrastructure needed)
            format!(
                "https://github.com/dev-parkins/FerrisScript/blob/main/docs/ERROR_CODES.md#{}",
                anchor
            )
        }
    }
}
```

**Rationale for Hybrid Approach**:
- ✅ **Works NOW**: GitHub URLs are immediately functional, clickable, and searchable
- ✅ **No Infrastructure**: No need to build/host documentation site for v0.0.3
- ✅ **Future-Proof**: Easy migration when `docs.ferrisscript.dev` launches (just set env var)
- ✅ **Zero Breaking Changes**: Old binaries continue working, new ones use new site automatically
- ✅ **Flexible**: Can test different doc sites or use self-hosted docs

**Deferred Items**:
- Documentation website (`docs.ferrisscript.dev`) → **Phase 9 or v0.0.4**
  - Use mdBook, Docusaurus, or similar
  - Host on GitHub Pages, Netlify, or Vercel
  - Set up CI to deploy on docs changes
  - Add search, navigation, examples
- When ready: Set `FERRIS_DOCS_BASE=https://docs.ferrisscript.dev` in releases/CI

**Integration**: Modify `format_error_with_code()` in `error_context.rs`:

```rust
pub fn format_error_with_code(
    code: ErrorCode,
    message: &str,
    source: &str,
    line: usize,
    col: usize,
) -> String {
    let mut output = /* existing formatting */;
    
    // Add documentation link
    output.push_str(&format!(
        "\n   = note: see {} for more information",
        code.get_docs_url()
    ));
    
    output
}
```

### Parser Error Recovery

**Location**: `crates/compiler/src/parser.rs`

**Add to Parser struct**:

```rust
struct Parser<'a> {
    tokens: Vec<Token>,
    source: &'a str,
    position: usize,
    current_line: usize,
    current_column: usize,
    diagnostics: Vec<Diagnostic>,  // NEW: collect errors
    panic_mode: bool,               // NEW: track recovery state
}
```

**Add recovery method**:

```rust
impl<'a> Parser<'a> {
    /// Synchronize parser to next safe point after error
    fn synchronize(&mut self) {
        self.panic_mode = true;
        
        while !self.is_at_end() {
            // Stop at statement boundaries
            if self.previous().token_type == TokenType::Semicolon {
                self.panic_mode = false;
                return;
            }
            
            // Stop at block boundaries or keywords
            match self.peek().token_type {
                TokenType::Fn | TokenType::Let | 
                TokenType::If | TokenType::While |
                TokenType::Return | TokenType::RightBrace => {
                    self.panic_mode = false;
                    return;
                }
                _ => self.advance(),
            }
        }
    }
    
    /// Record diagnostic instead of returning error
    fn add_diagnostic(&mut self, code: ErrorCode, message: String) {
        let diagnostic = Diagnostic {
            code,
            message,
            line: self.current_line,
            column: self.current_column,
            level: DiagnosticLevel::Error,
        };
        self.diagnostics.push(diagnostic);
    }
}
```

**Modify parse methods** to continue after errors:

```rust
fn parse_statement(&mut self) -> Option<Statement> {
    let result = match self.peek().token_type {
        TokenType::Let => self.parse_let_statement(),
        // ... other cases
        _ => {
            self.add_diagnostic(
                ErrorCode::E104,
                format!("Expected statement, found {:?}", self.peek())
            );
            self.synchronize();
            return None;
        }
    };
    
    match result {
        Ok(stmt) => Some(stmt),
        Err(e) => {
            self.add_diagnostic(/* extract code from error */, e);
            self.synchronize();
            None
        }
    }
}
```

### Multi-Error Reporting

**New file**: `crates/compiler/src/diagnostic.rs`

```rust
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub code: ErrorCode,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub level: DiagnosticLevel,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
}

pub struct DiagnosticBatch {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticBatch {
    pub fn new() -> Self {
        Self { diagnostics: Vec::new() }
    }
    
    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }
    
    pub fn report_batch(&self) -> String {
        let mut output = String::new();
        
        // Sort by line number
        let mut sorted = self.diagnostics.clone();
        sorted.sort_by_key(|d| d.line);
        
        // Format each diagnostic
        for diag in sorted {
            output.push_str(&format_diagnostic(&diag));
            output.push('\n');
        }
        
        // Add summary
        let error_count = self.diagnostics.iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        output.push_str(&format!(
            "\nFound {} error(s)\n",
            error_count
        ));
        
        output
    }
}
```

### CLI Integration

**Modify**: `crates/compiler/src/lib.rs` or main CLI

```rust
pub fn compile_with_options(source: &str, report_mode: ReportMode) -> Result<Program, String> {
    let tokens = tokenize(source)?;
    let mut parser = Parser::new(tokens, source);
    let program = parser.parse();
    
    match report_mode {
        ReportMode::Batch => {
            if !parser.diagnostics.is_empty() {
                return Err(format_batch_diagnostics(&parser.diagnostics));
            }
        }
        ReportMode::Stream => {
            for diag in &parser.diagnostics {
                eprintln!("{}", format_diagnostic(diag));
            }
            if parser.has_errors() {
                return Err("Compilation failed".to_string());
            }
        }
    }
    
    Ok(program)
}
```

---

## 🔬 Implementation Phases

### Phase 3A: Documentation URLs (4-6 hours)

1. Add `get_docs_url()` method to ErrorCode
2. Modify `format_error_with_code()` to include URLs
3. Test URL generation for all error codes
4. Run full test suite to ensure URLs appear

### Phase 3B: Enhance ERROR_CODES.md (2-3 hours)

1. Review all entries for completeness
2. Add "See also" cross-references
3. Verify markdown anchors work
4. Add missing examples where needed

### Phase 3C: Parser Recovery (6-8 hours)

1. Add `diagnostics` and `panic_mode` to Parser struct
2. Implement `synchronize()` method
3. Add `add_diagnostic()` helper
4. Modify parse methods to use recovery
5. Test with multiple error scenarios

### Phase 3D: Multi-Error Reporting (4-6 hours)

1. Create `diagnostic.rs` module
2. Implement `Diagnostic` and `DiagnosticBatch`
3. Add batch vs stream formatting
4. Integrate with parser
5. Add CLI flag for report mode

### Phase 3E: Testing & Polish (3-4 hours)

1. Write unit tests for recovery
2. Write integration tests for multi-error
3. Verify URL tests
4. Run full clippy and fmt
5. Update LEARNINGS.md

**Total Estimated Time**: 19-27 hours (2.5-3.5 days)

---

## 📊 Success Metrics

### Quantitative Goals

- [ ] 100% of error messages include documentation URLs
- [ ] Parser recovers from 90%+ of common syntax errors
- [ ] Multi-error reporting works for files with 5+ errors
- [ ] No false cascading errors (<5% false positive rate)
- [ ] Test coverage: 80%+ on new recovery code

### Qualitative Goals

- [ ] Error messages include helpful documentation links
- [ ] Parser reports all independent errors in one pass
- [ ] Recovery doesn't produce confusing cascading errors
- [ ] Documentation is easy to navigate and search
- [ ] Batch mode works well for CLI, stream mode ready for LSP

---

## 🚫 Out of Scope (Deferred)

### Documentation Website (Deferred to Phase 9 or v0.0.4)

**Why Deferred**:
- ERROR_CODES.md is sufficient for v0.0.3
- Full website (mdBook/Docusaurus) requires hosting setup
- LSP integration is higher priority
- Can build website from existing markdown later

**Future Work**:
- Set up mdBook or Docusaurus
- Add search functionality
- Host on GitHub Pages or Netlify
- Add interactive examples

### Advanced Error Recovery

**Why Deferred**:
- Basic synchronization is sufficient for v0.0.3
- Advanced strategies (speculative parsing, etc.) are complex
- Can improve incrementally in future versions

**Future Work**:
- Context-sensitive recovery strategies
- Better expected token suggestions
- Recovery confidence scoring

### JSON Diagnostic Export

**Why Deferred**:
- LSP will handle structured diagnostics
- CLI users don't need JSON format yet
- Can add later if tools ecosystem develops

---

## 📝 Dependencies

**Requires from Previous Phases**:
- ✅ Phase 1: Error code system (E001-E499)
- ✅ Phase 2: Error suggestions (for testing integration)
- ✅ ERROR_CODES.md exists with comprehensive content

**Enables for Future Phases**:
- Phase 4-5: VS Code extension can link to documentation
- Phase 5: LSP can use streaming diagnostic mode
- Phase 6+: Better developer experience for all tooling

---

## 🔗 Related Documents

- [Phase 1: Error Code System](./PHASE_1_ERROR_CODES.md) - Prerequisite
- [Phase 2: Error Suggestions](./PHASE_2_ERROR_SUGGESTIONS.md) - Prerequisite
- [v0.0.3 Roadmap](./v0.0.3-roadmap.md) - Context
- [LEARNINGS.md](./LEARNINGS.md) - Will update with discoveries

---

## ✅ Quality Gates

Before marking Phase 3 complete:

- [ ] All tests pass: `cargo test --workspace`
- [ ] Clippy passes (strict): `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] Code formatted: `cargo fmt --all`
- [ ] Documentation updated: LEARNINGS.md, README.md phase tracker
- [ ] PR created with detailed description
- [ ] Example error messages with URLs validated
- [ ] Parser recovery tested with multiple error files
- [ ] Multi-error reporting verified in batch and stream modes

---

**Last Updated**: October 6, 2025  
**Status**: 📝 Planning Complete, Ready for Implementation
