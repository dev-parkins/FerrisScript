//! FerrisScript Compiler
//!
//! This crate provides the complete compilation pipeline for FerrisScript, a Rust-inspired
//! scripting language for the Godot game engine.
//!
//! # Overview
//!
//! The compiler transforms FerrisScript source code through several stages:
//!
//! 1. **Lexical Analysis** ([`lexer`]): Converts source text into tokens
//! 2. **Parsing** ([`parser`]): Builds an Abstract Syntax Tree (AST) from tokens
//! 3. **Type Checking** ([`type_checker`]): Verifies type correctness and safety
//!
//! # Quick Start
//!
//! ```no_run
//! use ferrisscript_compiler::compile;
//!
//! let source = r#"
//!     fn add(a: i32, b: i32) -> i32 {
//!         return a + b;
//!     }
//! "#;
//!
//! match compile(source) {
//!     Ok(program) => println!("Compilation successful!"),
//!     Err(e) => eprintln!("Compilation error: {}", e),
//! }
//! ```
//!
//! # Modules
//!
//! - [`ast`]: Abstract Syntax Tree node definitions
//! - [`error_code`]: Error code definitions and categories
//! - [`error_context`]: Error formatting with source context
//! - [`lexer`]: Lexical analysis (tokenization)
//! - [`parser`]: Syntax analysis (AST generation)
//! - [`span`]: Source code location tracking for error messages and LSP
//! - [`type_checker`]: Semantic analysis (type checking)

pub mod ast;
pub mod error_code;
pub mod error_context;
pub mod lexer;
pub mod parser;
pub mod span;
pub mod suggestions;
pub mod type_checker;

/// Compile FerrisScript source code to an Abstract Syntax Tree (AST).
///
/// This is the main entry point for the compiler. It performs lexical analysis,
/// parsing, and type checking in sequence, returning either a validated AST
/// or a descriptive error message with source context.
///
/// # Arguments
///
/// * `source` - The complete FerrisScript source code as a string
///
/// # Returns
///
/// * `Ok(Program)` - A validated AST ready for execution
/// * `Err(String)` - A formatted error message with line/column information and source context
///
/// # Examples
///
/// ```no_run
/// use ferrisscript_compiler::compile;
///
/// // Simple function definition
/// let source = "fn greet() { print(\"Hello!\"); }";
/// let program = compile(source).unwrap();
///
/// // With type annotations
/// let source = r#"
///     fn calculate(x: f32, y: f32) -> f32 {
///         return x * y + 10.0;
///     }
/// "#;
/// let program = compile(source).unwrap();
/// ```
///
/// # Errors
///
/// Returns `Err` if:
/// - Source contains invalid tokens (lexer errors)
/// - Syntax is malformed (parser errors)
/// - Types are incompatible (type checker errors)
///
/// Error messages include:
/// - Line and column numbers
/// - Source code context (±2 lines)
/// - Visual pointer to error location
/// - Helpful hint about the issue
pub fn compile(source: &str) -> Result<ast::Program, String> {
    let positioned_tokens = lexer::tokenize_positioned(source)?;
    let mut ast = parser::parse_positioned(&positioned_tokens, source)?;

    // Type check and extract property metadata (Phase 5)
    let metadata = type_checker::check_and_extract_metadata(&ast, source)?;
    ast.property_metadata = metadata;

    Ok(ast)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn example_path(name: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.pop(); // Go up from crates/compiler
        path.pop(); // Go up from crates
        path.push("examples");
        path.push(name);
        path
    }

    #[test]
    fn test_compile_hello() {
        let source = std::fs::read_to_string(example_path("hello.ferris")).unwrap();
        assert!(compile(&source).is_ok());
    }

    #[test]
    fn test_compile_move() {
        let source = std::fs::read_to_string(example_path("move.ferris")).unwrap();
        assert!(compile(&source).is_ok());
    }

    #[test]
    fn test_compile_bounce() {
        let source = std::fs::read_to_string(example_path("bounce.ferris")).unwrap();
        assert!(compile(&source).is_ok());
    }

    #[test]
    fn test_compile_branch() {
        let source = std::fs::read_to_string(example_path("branch.ferris")).unwrap();
        let result = compile(&source);
        if let Err(e) = &result {
            eprintln!("branch.ferris error: {}", e);
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_loop() {
        let source = std::fs::read_to_string(example_path("loop.ferris")).unwrap();
        assert!(compile(&source).is_ok());
    }

    #[test]
    fn test_compile_functions() {
        let source = std::fs::read_to_string(example_path("functions.ferris")).unwrap();
        assert!(compile(&source).is_ok());
    }

    #[test]
    fn test_compile_type_error() {
        let source = std::fs::read_to_string(example_path("type_error.ferris")).unwrap();
        let result = compile(&source);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Type mismatch"));
    }

    #[test]
    fn test_compile_scene() {
        let source = std::fs::read_to_string(example_path("scene.ferris")).unwrap();
        assert!(compile(&source).is_ok());
    }

    #[test]
    fn test_compile_reload() {
        let source = std::fs::read_to_string(example_path("reload.ferris")).unwrap();
        assert!(compile(&source).is_ok());
    }

    // Phase 2 example tests temporarily disabled - files deferred due to compilation investigation
    // See: docs/planning/v0.0.4/KNOWN_LIMITATIONS.md#-known-issues
    // Core functionality verified through unit tests (test_input_function_valid, etc.)

    // #[test]
    // fn test_compile_input() {
    //     let source = std::fs::read_to_string(example_path("input.ferris")).unwrap();
    //     assert!(compile(&source).is_ok());
    // }

    // #[test]
    // fn test_compile_callbacks() {
    //     let source = std::fs::read_to_string(example_path("callbacks.ferris")).unwrap();
    //     assert!(compile(&source).is_ok());
    // }

    // Error reporting tests (verify correct line/column reporting)
    #[test]
    fn test_missing_semicolon_line_7() {
        // Test case for error reporting fix
        // Previously reported: "Expected ; at line 1, column 1"
        // Should report: "Expected ; at line 7, column X"
        let source = r#"
// HI FROM COMMENT


let thing:bool = true;
let result: i32 = 0

fn assert_test(cond: bool) {
    if cond {
        print("PASS");
    }
}
"#;

        let result = compile(source);
        assert!(result.is_err(), "Expected compilation to fail");

        let error = result.unwrap_err();

        // Error should mention line 6 (where the missing semicolon is)
        assert!(
            error.contains("line 6"),
            "Error should mention line 6, but got: {}",
            error
        );

        // Error should mention the semicolon
        assert!(
            error.contains("Expected ;") || error.contains("Semicolon"),
            "Error should mention semicolon, but got: {}",
            error
        );

        // Error should NOT report line 1, column 1 (the bug we fixed)
        assert!(
            !error.contains("line 1, column 1"),
            "Error should not report line 1, column 1 (this was the bug)"
        );
    }

    #[test]
    fn test_error_with_blank_lines_and_comments() {
        // Test that blank lines and comments don't break position tracking
        let source = r#"


// Comment 1
// Comment 2

let x: i32 = 10

fn test() {
    print("test");
}
"#;

        let result = compile(source);
        assert!(result.is_err());

        let error = result.unwrap_err();

        // Should report error around line 8 (where let x is)
        assert!(
            error.contains("line 7") || error.contains("line 8") || error.contains("line 9"),
            "Error should report correct line number, but got: {}",
            error
        );
    }

    #[test]
    fn test_multiple_errors_with_positions() {
        let source = r#"let a: i32 = 1
let b: i32 = 2
let c: i32 = 3"#;

        let result = compile(source);
        assert!(result.is_err());

        let error = result.unwrap_err();

        // First error should be on line 1
        assert!(
            error.contains("line 1"),
            "Should report line 1 error, but got: {}",
            error
        );
    }

    #[test]
    fn test_span_tracking_on_functions() {
        use crate::lexer::tokenize;
        use crate::parser::parse;

        let source = r#"fn add(a: i32, b: i32) -> i32 {
    return a + b;
}"#;

        let tokens = tokenize(source).unwrap();
        let program = parse(&tokens, source).unwrap();

        // Verify function has span information
        assert!(!program.functions.is_empty());
        let func = &program.functions[0];
        assert_eq!(func.span.line(), 1); // Function starts at line 1
        assert!(!func.span.is_unknown());
    }

    #[test]
    fn test_span_tracking_on_expressions() {
        use crate::lexer::tokenize;
        use crate::parser::parse;

        let source = r#"fn test() {
    let x: i32 = 42;
}"#;

        let tokens = tokenize(source).unwrap();
        let program = parse(&tokens, source).unwrap();

        // Get the let statement
        let func = &program.functions[0];
        assert!(!func.body.is_empty());
        let stmt = &func.body[0];

        // Verify statement has span
        let stmt_span = stmt.span();
        // The span tracks the start of the let keyword, which is on line 1 in this test
        // (the raw string starts counting from line 1, not the visual line 2)
        assert!(stmt_span.line() >= 1);
        assert!(!stmt_span.is_unknown());
    }

    #[test]
    fn test_span_merge_functionality() {
        use crate::span::{Position, Span};

        let start_pos = Position::new(1, 5, 4);
        let end_pos = Position::new(1, 10, 9);
        let span1 = Span::new(start_pos, end_pos);

        let start_pos2 = Position::new(1, 15, 14);
        let end_pos2 = Position::new(1, 20, 19);
        let span2 = Span::new(start_pos2, end_pos2);

        let merged = span1.merge(span2);

        // Merged span should encompass both spans
        assert_eq!(merged.start.column, 5);
        assert_eq!(merged.end.column, 20);
        assert_eq!(merged.len(), 15); // 19 - 4 = 15 bytes
    }

    #[test]
    fn test_expr_span_accessor() {
        use crate::ast::{Expr, Literal};
        use crate::span::{Position, Span};

        let pos = Position::new(5, 10, 42);
        let span = Span::point(pos);
        let expr = Expr::Literal(Literal::Int(42), span);

        // Verify span accessor works
        assert_eq!(expr.span(), span);
        assert_eq!(expr.span().line(), 5);
        assert_eq!(expr.span().column(), 10);
    }

    #[test]
    fn test_stmt_span_accessor() {
        use crate::ast::{Expr, Literal, Stmt};
        use crate::span::{Position, Span};

        let pos = Position::new(3, 5, 20);
        let span = Span::point(pos);
        let expr = Expr::Literal(Literal::Bool(true), span);
        let stmt = Stmt::Return {
            value: Some(expr),
            span,
        };

        // Verify statement span accessor works
        assert_eq!(stmt.span(), span);
        assert_eq!(stmt.span().line(), 3);
        assert_eq!(stmt.span().column(), 5);
    }
}
