//! Edge case tests for empty script files
//!
//! Tests the compiler's behavior when handling empty input at different stages:
//! - Lexer: Must handle empty string without panicking
//! - Parser: Must return valid empty AST
//! - Full pipeline: Must compile successfully

use ferrisscript_compiler::{compile, lexer, parser};

#[test]
fn test_empty_file_lexer() {
    // Test that lexer handles completely empty input
    let input = "";
    let result = lexer::tokenize(input);

    // Should succeed with no tokens (or just EOF if lexer adds one)
    assert!(
        result.is_ok(),
        "Lexer should handle empty input without error"
    );
    let tokens = result.unwrap();
    // Empty or contains only EOF token
    assert!(
        tokens.is_empty() || tokens.len() == 1,
        "Empty input should produce no tokens or only EOF"
    );
}

#[test]
fn test_empty_file_parser() {
    // Test that parser handles empty token stream
    let input = "";
    let tokens = lexer::tokenize(input).expect("Lexer should succeed on empty input");
    let result = parser::parse(&tokens, input);

    // Should succeed with empty program
    assert!(
        result.is_ok(),
        "Parser should handle empty input without error"
    );
    let ast = result.unwrap();
    assert_eq!(
        ast.global_vars.len(),
        0,
        "Empty input should have no global vars"
    );
    assert_eq!(
        ast.functions.len(),
        0,
        "Empty input should have no functions"
    );
}

#[test]
fn test_empty_file_full_pipeline() {
    // Test complete compilation pipeline with empty script
    let input = "";
    let result = compile(input);

    // Should succeed - empty script is valid
    assert!(result.is_ok(), "Empty script should compile successfully");
    let ast = result.unwrap();
    assert_eq!(
        ast.global_vars.len(),
        0,
        "Empty script should have no global vars"
    );
    assert_eq!(
        ast.functions.len(),
        0,
        "Empty script should have no functions"
    );
}

#[test]
fn test_empty_file_whitespace_only() {
    // Test that whitespace-only files are treated as empty
    let test_cases = vec![" ", "\n", "\t", "   ", "\n\n\n", " \t \n ", "\r\n"];

    for input in test_cases {
        let result = compile(input);
        assert!(
            result.is_ok(),
            "Whitespace-only input '{}' should compile successfully",
            input.escape_debug()
        );
        let ast = result.unwrap();
        assert_eq!(
            ast.global_vars.len(),
            0,
            "Whitespace-only input should have no global vars"
        );
        assert_eq!(
            ast.functions.len(),
            0,
            "Whitespace-only input should have no functions"
        );
    }
}
