//! Edge case tests for scripts containing only comments
//!
//! Tests the compiler's behavior when handling files with only comments:
//! - Single line comments
//! - Multiple line comments
//! - Comments mixed with whitespace
//! - Comments at different positions

use ferrisscript_compiler::compile;

#[test]
fn test_single_line_comment_only() {
    // Test file with only one line comment
    let input = "// Just a comment";
    let result = compile(input);

    assert!(result.is_ok(), "Single comment should compile successfully");
    let ast = result.unwrap();
    assert_eq!(
        ast.global_vars.len(),
        0,
        "Comment-only file should have no global vars"
    );
    assert_eq!(
        ast.functions.len(),
        0,
        "Comment-only file should have no functions"
    );
}

#[test]
fn test_multiple_line_comments() {
    // Test file with multiple line comments
    let input = r#"// Comment 1
// Comment 2
// Comment 3
// Comment 4"#;

    let result = compile(input);
    assert!(
        result.is_ok(),
        "Multiple comments should compile successfully"
    );
    let ast = result.unwrap();
    assert_eq!(
        ast.global_vars.len(),
        0,
        "Comment-only file should have no global vars"
    );
    assert_eq!(
        ast.functions.len(),
        0,
        "Comment-only file should have no functions"
    );
}

#[test]
fn test_comments_with_whitespace() {
    // Test comments mixed with various whitespace
    let test_cases = vec![
        "\n\n  // Comment\n\n",
        "  // Comment 1\n\n  // Comment 2  \n",
        "\t// Tab-indented comment",
        "// Comment\n\n\n\n",
        "   \n   // Spaces before\n   ",
    ];

    for input in test_cases {
        let result = compile(input);
        assert!(
            result.is_ok(),
            "Comments with whitespace should compile: {:?}",
            input
        );
        let ast = result.unwrap();
        assert_eq!(ast.global_vars.len(), 0, "Should have no global vars");
        assert_eq!(ast.functions.len(), 0, "Should have no functions");
    }
}

#[test]
fn test_comment_at_end_of_empty_lines() {
    // Test comments after newlines
    let input = "\n\n\n// Comment at end";
    let result = compile(input);

    assert!(result.is_ok(), "Comment after newlines should compile");
    let ast = result.unwrap();
    assert_eq!(ast.global_vars.len(), 0, "Should have no global vars");
    assert_eq!(ast.functions.len(), 0, "Should have no functions");
}

#[test]
fn test_comment_with_special_characters() {
    // Test comments containing special characters
    let test_cases = vec![
        "// Comment with symbols: !@#$%^&*()",
        "// Comment with brackets: []{}()",
        "// Comment with quotes: \"hello\" 'world'",
        "// Comment with numbers: 123456789",
        "// Comment with unicode: 🦀 Rust",
    ];

    for input in test_cases {
        let result = compile(input);
        assert!(
            result.is_ok(),
            "Comment with special chars should compile: {:?}",
            input
        );
        let ast = result.unwrap();
        assert_eq!(ast.global_vars.len(), 0, "Should have no global vars");
        assert_eq!(ast.functions.len(), 0, "Should have no functions");
    }
}
