# Phase 1: Error Code System

**Status**: Not Started  
**Priority**: Critical  
**Branch**: `feature/v0.0.3-error-codes`  
**Estimated Effort**: 3-4 days  
**Dependencies**: None

---

## 🎯 Overview

Implement a structured error code system (E001-E499) to provide clear, categorized, and actionable error messages. This establishes the foundation for error suggestions, documentation links, and future LSP integration.

**Strategic Value**: Professional error reporting significantly improves developer experience and debugging efficiency. Error codes enable precise error tracking and documentation.

---

## ✅ Acceptance Criteria

### 1. Error Code Infrastructure

- [ ] **Error enum with codes**: Define `ErrorCode` enum with variants E001-E499
- [ ] **Category organization**: Group codes into lexical, syntax, type, semantic, and runtime categories
- [ ] **Error formatting**: Update error display to include error codes (e.g., `Error[E201]: Undefined variable`)
- [ ] **Backward compatibility**: Ensure existing error messages still work during transition

**Validation**:

```rust
// Example test
#[test]
fn test_error_code_display() {
    let err = CompilerError::new(ErrorCode::E201, "Undefined variable 'x'");
    assert_eq!(err.code(), "E201");
    assert!(err.to_string().contains("Error[E201]"));
}
```

### 2. Lexical Error Codes (E001-E099)

- [ ] **E001**: Invalid character
- [ ] **E002**: Unterminated string literal
- [ ] **E003**: Invalid escape sequence
- [ ] **E004**: Invalid number format
- [ ] **E005**: Invalid identifier
- [ ] **E006**: Unexpected end of file (lexer level)

**Validation**: Each error code must have:

- At least 2 unit tests (positive and edge case)
- Example in error code reference docs
- Clear error message text

### 3. Syntax Error Codes (E100-E199)

- [ ] **E100**: Expected token (e.g., expected `;`, found `}`)
- [ ] **E101**: Unexpected token
- [ ] **E102**: Missing closing delimiter (parenthesis, bracket, brace)
- [ ] **E103**: Invalid expression
- [ ] **E104**: Invalid statement
- [ ] **E105**: Invalid function declaration
- [ ] **E106**: Invalid type annotation
- [ ] **E107**: Invalid pattern
- [ ] **E108**: Unexpected end of file (parser level)

**Validation**: Parser tests must verify error codes for common syntax errors.

### 4. Type Error Codes (E200-E299)

- [ ] **E200**: Type mismatch
- [ ] **E201**: Undefined variable
- [ ] **E202**: Undefined function
- [ ] **E203**: Undefined type
- [ ] **E204**: Wrong number of arguments
- [ ] **E205**: Incorrect argument type
- [ ] **E206**: Return type mismatch
- [ ] **E207**: Cannot assign to immutable variable
- [ ] **E208**: Duplicate definition
- [ ] **E209**: Invalid field access
- [ ] **E210**: Invalid method call

**Validation**: Type checker tests must verify error codes for type violations.

### 5. Semantic Error Codes (E300-E399)

- [ ] **E300**: Unreachable code
- [ ] **E301**: Unused variable (warning)
- [ ] **E302**: Unused function (warning)
- [ ] **E303**: Dead code (warning)
- [ ] **E304**: Invalid break/continue (not in loop)
- [ ] **E305**: Invalid return (not in function)

**Validation**: Semantic analyzer tests must verify error codes for semantic violations.

### 6. Runtime Error Codes (E400-E499)

- [ ] **E400**: Division by zero
- [ ] **E401**: Index out of bounds
- [ ] **E402**: Null pointer access
- [ ] **E403**: Stack overflow
- [ ] **E404**: Memory exhaustion
- [ ] **E405**: Godot API error

**Validation**: Runtime tests must verify error codes are propagated correctly.

### 7. Error Code Reference Documentation

- [ ] **Create docs/ERROR_CODES.md**: Reference table of all error codes
- [ ] **Category sections**: Organize by E001-E099, E100-E199, etc.
- [ ] **For each error**: Include:
  - Error code and name
  - Description
  - Common causes
  - Example code that triggers error
  - How to fix
  - Related error codes

**Validation**: Documentation must be linked from README.md and pass markdown linting.

### 8. Test Coverage

- [ ] **Unit tests**: At least 2 tests per error code (80+ tests total)
- [ ] **Integration tests**: Test error codes through full compiler pipeline
- [ ] **Coverage**: 80%+ coverage for error reporting code
- [ ] **Error code exhaustiveness**: Test that all error codes are documented

**Validation**: Run `cargo test --workspace` and `cargo tarpaulin` to verify coverage.

---

## 🏗️ Technical Approach

### Error Code Enum

```rust
// crates/compiler/src/error.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    // Lexical Errors (E001-E099)
    E001, // Invalid character
    E002, // Unterminated string literal
    E003, // Invalid escape sequence
    E004, // Invalid number format
    E005, // Invalid identifier
    E006, // Unexpected EOF (lexer)
    
    // Syntax Errors (E100-E199)
    E100, // Expected token
    E101, // Unexpected token
    E102, // Missing closing delimiter
    E103, // Invalid expression
    E104, // Invalid statement
    E105, // Invalid function declaration
    E106, // Invalid type annotation
    E107, // Invalid pattern
    E108, // Unexpected EOF (parser)
    
    // Type Errors (E200-E299)
    E200, // Type mismatch
    E201, // Undefined variable
    E202, // Undefined function
    E203, // Undefined type
    E204, // Wrong number of arguments
    E205, // Incorrect argument type
    E206, // Return type mismatch
    E207, // Cannot assign to immutable
    E208, // Duplicate definition
    E209, // Invalid field access
    E210, // Invalid method call
    
    // Semantic Errors (E300-E399)
    E300, // Unreachable code
    E301, // Unused variable
    E302, // Unused function
    E303, // Dead code
    E304, // Invalid break/continue
    E305, // Invalid return
    
    // Runtime Errors (E400-E499)
    E400, // Division by zero
    E401, // Index out of bounds
    E402, // Null pointer access
    E403, // Stack overflow
    E404, // Memory exhaustion
    E405, // Godot API error
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::E001 => "E001",
            Self::E002 => "E002",
            // ... etc
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            Self::E001 => "Invalid character",
            Self::E002 => "Unterminated string literal",
            // ... etc
        }
    }
    
    pub fn category(&self) -> ErrorCategory {
        match self {
            Self::E001..=Self::E099 => ErrorCategory::Lexical,
            Self::E100..=Self::E199 => ErrorCategory::Syntax,
            Self::E200..=Self::E299 => ErrorCategory::Type,
            Self::E300..=Self::E399 => ErrorCategory::Semantic,
            Self::E400..=Self::E499 => ErrorCategory::Runtime,
        }
    }
}
```

### Error Display Update

```rust
// Before
Error: Undefined variable 'velocty'
  --> move.ferris:5:10

// After
Error[E201]: Undefined variable
  --> move.ferris:5:10
   |
 5 |     self.velocty.x += 50.0;
   |          ^^^^^^^ not found in this scope
```

### Migration Strategy

1. **Phase 1a**: Define ErrorCode enum and display logic
2. **Phase 1b**: Update lexer errors (E001-E099)
3. **Phase 1c**: Update parser errors (E100-E199)
4. **Phase 1d**: Update type checker errors (E200-E299)
5. **Phase 1e**: Update semantic errors (E300-E399)
6. **Phase 1f**: Update runtime errors (E400-E499)
7. **Phase 1g**: Write documentation

---

## 🧪 Testing Strategy

### Unit Tests

```rust
#[test]
fn test_lexer_error_codes() {
    // E002: Unterminated string
    let input = r#"let x = "hello"#;
    let errors = lexer::lex(input);
    assert_eq!(errors[0].code(), ErrorCode::E002);
}

#[test]
fn test_parser_error_codes() {
    // E100: Expected semicolon
    let input = "let x = 5";
    let errors = parser::parse(input);
    assert_eq!(errors[0].code(), ErrorCode::E100);
}

#[test]
fn test_type_error_codes() {
    // E201: Undefined variable
    let input = "fn main() { print(x); }";
    let errors = type_checker::check(input);
    assert_eq!(errors[0].code(), ErrorCode::E201);
}
```

### Integration Tests

```rust
#[test]
fn test_error_codes_end_to_end() {
    let test_cases = vec![
        (r#"let x = "unterminated"#, ErrorCode::E002),
        ("let x = 5", ErrorCode::E100),
        ("fn main() { print(x); }", ErrorCode::E201),
    ];
    
    for (input, expected_code) in test_cases {
        let result = compile(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), expected_code);
    }
}
```

---

## 📦 Component Changes

### Modified Files

1. **crates/compiler/src/error.rs**
   - Add ErrorCode enum
   - Update error display logic
   - Add error code utility functions

2. **crates/compiler/src/lexer.rs**
   - Update lexer errors to use E001-E099

3. **crates/compiler/src/parser.rs**
   - Update parser errors to use E100-E199

4. **crates/compiler/src/type_checker.rs**
   - Update type checker errors to use E200-E299

5. **crates/compiler/src/semantic.rs** (may need to create)
   - Add semantic analyzer for E300-E399

6. **crates/runtime/src/lib.rs**
   - Update runtime errors to use E400-E499

7. **docs/ERROR_CODES.md** (new)
   - Comprehensive error code reference

8. **README.md**
   - Add link to ERROR_CODES.md

### New Files

- **docs/ERROR_CODES.md**: Error code reference documentation
- **crates/compiler/src/semantic.rs** (if needed): Semantic analysis

---

## 🎯 Quality Gates

### Pre-Merge Checklist

- [ ] All acceptance criteria met
- [ ] All unit tests pass (`cargo test --workspace`)
- [ ] All integration tests pass
- [ ] Code coverage ≥ 80% for modified files (`cargo tarpaulin`)
- [ ] No clippy warnings (`cargo clippy --workspace --all-targets -- -D warnings`)
- [ ] Code formatted (`cargo fmt --check --all`)
- [ ] Documentation complete (ERROR_CODES.md)
- [ ] Documentation linting passes (`npm run docs:lint`)
- [ ] Link checking passes (`npm run docs:links`)
- [ ] PR description includes example error output (before/after)
- [ ] LEARNINGS.md updated with insights

### CI Requirements

- [ ] Quick-check job passes (2-3 minutes)
- [ ] Full test suite passes (10-15 minutes)
- [ ] No regression in existing tests
- [ ] Error code coverage test passes

---

## 📊 Success Metrics

- **Error Code Coverage**: 100% of compiler errors have error codes
- **Test Coverage**: 80%+ for error reporting code
- **Documentation**: All error codes documented with examples
- **User Experience**: Clear, professional error messages with codes
- **Foundation**: Ready for Phase 2 (error suggestions)

---

## 🔗 Dependencies

**Depends On**: None (foundational phase)

**Required By**:

- Phase 2: Error Suggestions (needs error code infrastructure)
- Phase 3: Error Documentation (needs error codes to link)
- Phase 5: VS Code Problem Panel (needs structured errors)

---

## 📝 Notes

- **Backward Compatibility**: Maintain existing error message structure during migration
- **Reserved Codes**: Reserve E050-E099, E150-E199, E250-E299, E350-E399, E450-E499 for future expansion
- **Warning vs Error**: E301-E303 are warnings (code still compiles)
- **Runtime Errors**: E400-E499 may be handled by runtime, not compiler
- **LSP Preparation**: Error code structure aligns with LSP diagnostic codes for future integration

---

## 🔮 Future Enhancements (Not in v0.0.3)

- Error code quick fixes (LSP v0.0.5)
- Error code telemetry (track most common errors)
- Localization support for error messages
- Machine-readable error output (JSON)
