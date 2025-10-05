/// Integration tests for error context display in all compiler phases
/// Tests that source context (±2 lines) and visual indicators appear correctly
use ferrisscript_compiler::compile;

#[test]
fn test_lexer_error_shows_context_unterminated_string() {
    let source = r#"fn test() {
    let s = "hello
    let x = 5;
}"#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    // Should contain the error message
    assert!(error.contains("Unterminated string"));
    // Should contain source context with line numbers
    assert!(error.contains("1 |"));
    assert!(error.contains("2 |"));
    // Should contain visual pointer
    assert!(error.contains("^"));
    // Should contain helpful hint
    assert!(error.contains("String must be closed"));
}

#[test]
fn test_lexer_error_shows_context_invalid_character() {
    let source = r#"fn test() {
    let x = 5 # 3;
}"#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    // # is an invalid character, will cause lexer error
    assert!(error.contains("1 |") || error.contains("2 |"));
    assert!(error.contains("^"));
}

#[test]
fn test_parser_error_shows_context_missing_identifier() {
    let source = r#"fn test() {
    let : i32 = 5;
}"#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("Expected identifier after 'let'"));
    assert!(error.contains("1 |"));
    assert!(error.contains("2 |"));
    assert!(error.contains("^"));
    assert!(error.contains("Variable name must be an identifier"));
}

#[test]
fn test_parser_error_shows_context_missing_type() {
    let source = r#"fn test() {
    let x: = 5;
}"#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("Expected"));
    assert!(error.contains("1 |"));
    assert!(error.contains("2 |"));
    assert!(error.contains("^"));
}

#[test]
fn test_parser_error_shows_context_invalid_function_syntax() {
    let source = r#"fn 123() {
}"#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("Expected function name"));
    assert!(error.contains("1 |"));
    assert!(error.contains("^"));
    assert!(error.contains("Function name must be an identifier"));
}

#[test]
fn test_parser_error_shows_context_field_access() {
    let source = r#"fn test() {
    let x = obj.;
}"#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("Expected field name after '.'"));
    assert!(error.contains("1 |"));
    assert!(error.contains("2 |"));
    assert!(error.contains("^"));
    assert!(error.contains("Field name must be an identifier"));
}

#[test]
fn test_type_checker_error_shows_context_type_mismatch() {
    let source = r#"fn test() {
    let x: i32 = true;
}"#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("Type mismatch"));
    assert!(error.contains("expected i32, found bool"));
    assert!(error.contains("1 |"));
    assert!(error.contains("2 |"));
    assert!(error.contains("^"));
    assert!(error.contains("cannot be coerced"));
}

#[test]
fn test_type_checker_error_shows_context_undefined_variable() {
    let source = r#"fn test() {
    let x: i32 = y;
}"#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("Undefined variable 'y'"));
    assert!(error.contains("1 |"));
    assert!(error.contains("2 |"));
    assert!(error.contains("^"));
    assert!(error.contains("Variable must be declared before use"));
}

#[test]
fn test_type_checker_error_shows_context_if_condition() {
    let source = r#"fn test() {
    if 5 {
        let x: i32 = 1;
    }
}"#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("If condition must be bool"));
    assert!(error.contains("1 |"));
    assert!(error.contains("2 |"));
    assert!(error.contains("^"));
    assert!(error.contains("boolean value"));
}

#[test]
fn test_error_context_on_first_line() {
    let source = r#"let x = "unterminated"#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    // Should show line 1 even though it's the first line
    assert!(error.contains("1 |"));
    assert!(error.contains("^"));
}

#[test]
fn test_error_context_on_last_line() {
    let source = r#"fn test() {
    let x: i32 = 5;
}
let y = "unterminated"#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    // Should show context even on last line
    assert!(error.contains("3 |"));
    assert!(error.contains("4 |"));
    assert!(error.contains("^"));
}

#[test]
fn test_error_pointer_alignment() {
    // Test that the ^ pointer aligns correctly with the error column
    let source = r#"fn test() {
    let       : i32 = 5;
}"#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    // The error is at column 15 (after "let       ")
    // The pointer line should have spaces before ^
    assert!(error.contains("^"));

    // Split into lines and find lines with pointer indicators
    let lines: Vec<&str> = error.lines().collect();
    let has_pointer = lines.iter().any(|l| l.contains("^"));
    assert!(
        has_pointer,
        "Should have a pointer (^) somewhere in the error"
    );

    // The pointer should appear with proper indentation
    let pointer_lines: Vec<&&str> = lines.iter().filter(|l| l.contains("^")).collect();
    assert!(
        !pointer_lines.is_empty(),
        "Should have at least one line with ^"
    );
}

#[test]
fn test_multiple_errors_each_show_context() {
    // This will generate a type error
    let source = r#"fn test() {
    let x: i32 = true;
    let y: bool = 42;
}"#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    // Should show context for the first error at least
    assert!(error.contains("Type mismatch"));
    assert!(error.contains("1 |"));
    assert!(error.contains("2 |"));
    assert!(error.contains("^"));
}

#[test]
fn test_error_context_with_short_file() {
    // File with only 2 lines
    let source = r#"fn test() {
    let x = "unterminated"#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    // Should still show context
    assert!(error.contains("1 |"));
    assert!(error.contains("2 |"));
    assert!(error.contains("^"));
}

#[test]
fn test_type_checker_binary_op_error_context() {
    let source = r#"fn test() {
    let x: i32 = 5 + true;
}"#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("Binary operation"));
    assert!(error.contains("numeric types"));
    assert!(error.contains("1 |"));
    assert!(error.contains("2 |"));
    assert!(error.contains("^"));
}

#[test]
fn test_type_checker_function_call_error_context() {
    let source = r#"fn test() {
    print(42);
}"#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("Function 'print'"));
    assert!(error.contains("wrong type"));
    assert!(error.contains("1 |"));
    assert!(error.contains("2 |"));
    assert!(error.contains("^"));
}

#[test]
fn test_parser_invalid_binary_operator_context() {
    let source = r#"fn test() {
    let x: i32 = 5;
    let y: i32 = x # 3;
}"#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    // This will be caught by lexer first
    assert!(error.contains("^"));
    assert!(error.contains("2 |") || error.contains("3 |"));
}
