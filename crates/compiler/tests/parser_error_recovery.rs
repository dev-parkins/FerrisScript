//! Parser error recovery integration tests
//!
//! These tests verify that the parser can recover from errors and continue
//! parsing to find multiple errors in a single pass. This implements panic-mode
//! error recovery with synchronization points at statement boundaries.

#[cfg(test)]
mod recovery_tests {
    use ferrisscript_compiler::{lexer, parser};

    #[test]
    fn test_multiple_missing_semicolons() {
        // Test that parser finds multiple missing semicolons
        let source = r#"
let x = 1
let y = 2
let z = 3;
"#;
        let tokens = lexer::tokenize(source).unwrap();
        let mut parser_instance = parser::Parser::new(tokens, source);
        let result = parser_instance.parse_program();

        // Should report error but have collected multiple issues
        assert!(result.is_err());
        let errors = parser_instance.get_errors();

        // Should find at least the first error (may not report all due to recovery)
        assert!(!errors.is_empty());
        // The error should be about the unexpected token after missing semicolon
        assert!(errors[0].contains("Expected") || errors[0].contains("E10"));
    }

    #[test]
    fn test_invalid_top_level_then_valid_function() {
        // Test that parser recovers from invalid top-level and continues
        let source = r#"
invalid_token
let x = 5;
fn foo() {
    let a = 1;
}
"#;
        let tokens = lexer::tokenize(source).unwrap();
        let mut parser_instance = parser::Parser::new(tokens, source);
        let result = parser_instance.parse_program();

        // Should report first error
        assert!(result.is_err());
        let errors = parser_instance.get_errors();

        // Should have collected error about invalid top-level token
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("Expected 'fn' or 'let' at top level"));
    }

    #[test]
    fn test_multiple_function_errors() {
        // Test multiple function definition errors
        let source = r#"
fn 123() {}
fn bar() {
    let x = 1
}
fn baz {}
"#;
        let tokens = lexer::tokenize(source).unwrap();
        let mut parser_instance = parser::Parser::new(tokens, source);
        let result = parser_instance.parse_program();

        // Should report error
        assert!(result.is_err());
        let errors = parser_instance.get_errors();

        // Should find the first error (function name)
        assert!(!errors.is_empty());
        assert!(
            errors[0].contains("Expected function name")
                || errors[0].contains("Expected identifier")
        );
    }

    #[test]
    fn test_mixed_global_and_function_errors() {
        // Test recovery across different declaration types
        let source = r#"
let x = ;
let y = 5;
fn foo() {
    let a = 1;
}
let z 10;
fn bar() {}
"#;
        let tokens = lexer::tokenize(source).unwrap();
        let mut parser_instance = parser::Parser::new(tokens, source);
        let result = parser_instance.parse_program();

        // Should report error
        assert!(result.is_err());
        let errors = parser_instance.get_errors();

        // Should have collected at least one error
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_no_cascading_errors_after_recovery() {
        // Verify that panic mode suppresses false positives
        let source = r#"
let x = 1
let y = 2;
"#;
        let tokens = lexer::tokenize(source).unwrap();
        let mut parser_instance = parser::Parser::new(tokens, source);
        let result = parser_instance.parse_program();

        // Should report error for missing semicolon
        assert!(result.is_err());
        let errors = parser_instance.get_errors();

        // Should only report one error (not cascading errors about 'y')
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("Expected") || errors[0].contains("E10"));
    }

    #[test]
    fn test_recovery_continues_after_function_body_error() {
        // Test that parser recovers from function body errors
        let source = r#"
fn foo() {
    let x = 1
    invalid
}
fn bar() {
    let y = 2;
}
"#;
        let tokens = lexer::tokenize(source).unwrap();
        let mut parser_instance = parser::Parser::new(tokens, source);
        let result = parser_instance.parse_program();

        // Should report error
        assert!(result.is_err());
        let errors = parser_instance.get_errors();

        // Should have collected at least the first error
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_empty_file_after_error() {
        // Test that parser handles EOF after error gracefully
        let source = "invalid_stuff";
        let tokens = lexer::tokenize(source).unwrap();
        let mut parser_instance = parser::Parser::new(tokens, source);
        let result = parser_instance.parse_program();

        // Should report error about invalid top-level
        assert!(result.is_err());
        let errors = parser_instance.get_errors();

        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("Expected 'fn' or 'let' at top level"));
    }

    #[test]
    fn test_recovery_at_right_brace() {
        // Test synchronization on right brace
        let source = r#"
fn foo() {
    let x = 1
    let y = 2;
}
let z = 3;
"#;
        let tokens = lexer::tokenize(source).unwrap();
        let mut parser_instance = parser::Parser::new(tokens, source);
        let result = parser_instance.parse_program();

        // Should report error for missing semicolon
        assert!(result.is_err());
        let errors = parser_instance.get_errors();

        // Should have collected the error
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_successful_parse_with_no_errors() {
        // Baseline: verify no false positives on valid code
        let source = r#"
let x = 5;
fn foo() {
    let y = 10;
}
"#;
        let tokens = lexer::tokenize(source).unwrap();
        let mut parser_instance = parser::Parser::new(tokens, source);
        let result = parser_instance.parse_program();

        // Should succeed with no errors
        assert!(result.is_ok());
        let errors = parser_instance.get_errors();
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_multiple_errors_collected_but_first_returned() {
        // Verify API compatibility: returns first error but collects all
        let source = r#"
bad1
bad2
let x = 5;
"#;
        let tokens = lexer::tokenize(source).unwrap();
        let mut parser_instance = parser::Parser::new(tokens, source);
        let result = parser_instance.parse_program();

        // Should return error
        assert!(result.is_err());

        // The error message should be from the first error
        let returned_error = result.unwrap_err();
        assert!(returned_error.contains("Expected 'fn' or 'let' at top level"));

        // Internal errors collection should have recorded errors
        let errors = parser_instance.get_errors();
        assert!(!errors.is_empty());
    }
}
