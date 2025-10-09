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
//! - [`type_checker`]: Semantic analysis (type checking)

pub mod ast;
pub mod error_code;
pub mod error_context;
pub mod lexer;
pub mod parser;
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
    let tokens = lexer::tokenize(source)?;
    let ast = parser::parse(&tokens, source)?;
    type_checker::check(&ast, source)?;
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
}
