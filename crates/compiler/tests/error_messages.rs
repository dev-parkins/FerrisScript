//! Error message validation tests
//!
//! These tests verify that all compiler errors include proper position information
//! (line and column numbers) to help users debug their scripts.

#[cfg(test)]
mod parser_errors {
    use ferrisscript_compiler::{lexer, parser};

    #[test]
    fn test_expected_identifier_after_let_includes_position() {
        let source = "let 123 = 5;";
        let tokens = lexer::tokenize(source).unwrap();
        let result = parser::parse(&tokens, source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected identifier after 'let'"));
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }

    #[test]
    fn test_expected_type_includes_position() {
        let source = "let x: 123 = 5;";
        let tokens = lexer::tokenize(source).unwrap();
        let result = parser::parse(&tokens, source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected type"));
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }

    #[test]
    fn test_expected_function_name_includes_position() {
        let source = "fn 123() {}";
        let tokens = lexer::tokenize(source).unwrap();
        let result = parser::parse(&tokens, source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected function name"));
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }

    #[test]
    fn test_expected_parameter_name_includes_position() {
        let source = "fn test(123: i32) {}";
        let tokens = lexer::tokenize(source).unwrap();
        let result = parser::parse(&tokens, source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected parameter name"));
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }

    #[test]
    fn test_expected_parameter_type_includes_position() {
        let source = "fn test(x: 123) {}";
        let tokens = lexer::tokenize(source).unwrap();
        let result = parser::parse(&tokens, source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected parameter type"));
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }

    #[test]
    fn test_expected_return_type_includes_position() {
        let source = "fn test() -> 123 {}";
        let tokens = lexer::tokenize(source).unwrap();
        let result = parser::parse(&tokens, source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected return type"));
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }

    #[test]
    fn test_expected_arrow_after_dash_includes_position() {
        let source = "fn test() - {}";
        let tokens = lexer::tokenize(source).unwrap();
        let result = parser::parse(&tokens, source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected '>' after '-'"));
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }

    #[test]
    fn test_expected_field_name_includes_position() {
        let source = "fn test() { let x = self.123; }";
        let tokens = lexer::tokenize(source).unwrap();
        let result = parser::parse(&tokens, source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected field name after '.'"));
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }

    #[test]
    fn test_top_level_syntax_error_includes_position() {
        let source = "123";
        let tokens = lexer::tokenize(source).unwrap();
        let result = parser::parse(&tokens, source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected 'fn' or 'let' at top level"));
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }

    #[test]
    fn test_unexpected_token_in_expression_includes_position() {
        let source = "fn test() { let x = }; }";
        let tokens = lexer::tokenize(source).unwrap();
        let result = parser::parse(&tokens, source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Unexpected token in expression"));
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }
}

#[cfg(test)]
mod lexer_errors {
    use ferrisscript_compiler::lexer;

    #[test]
    fn test_invalid_number_includes_position() {
        let source = "let x = 123.456.789;";
        let result = lexer::tokenize(source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Invalid number"));
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }

    #[test]
    fn test_unterminated_string_includes_position() {
        let source = r#"let x = "hello"#;
        let result = lexer::tokenize(source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Unterminated string"));
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }

    #[test]
    fn test_invalid_escape_sequence_includes_position() {
        let source = r#"let x = "hello\x";"#;
        let result = lexer::tokenize(source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Invalid escape sequence"));
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }

    #[test]
    fn test_unexpected_character_includes_position() {
        let source = "let x = @;";
        let result = lexer::tokenize(source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Unexpected character"));
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }

    #[test]
    fn test_unexpected_ampersand_includes_position() {
        let source = "let x = 5 & 3;";
        let result = lexer::tokenize(source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Unexpected character '&'"));
        assert!(error.contains("&&")); // Hint mentions &&
        assert!(error.contains("logical AND")); // Full hint text
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }

    #[test]
    fn test_unexpected_pipe_includes_position() {
        let source = "let x = true | false;";
        let result = lexer::tokenize(source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Unexpected character '|'"));
        assert!(error.contains("||")); // Hint mentions ||
        assert!(error.contains("logical OR")); // Full hint text
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }
}

#[cfg(test)]
mod type_checker_errors {
    use ferrisscript_compiler::compile;

    #[test]
    fn test_undefined_variable_includes_position() {
        let source = "fn test() { let x = undefined_var; }";
        let result = compile(source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Undefined variable"));
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }

    #[test]
    fn test_type_mismatch_includes_position() {
        let source = r#"fn test() { let x: i32 = "hello"; }"#;
        let result = compile(source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Type mismatch"));
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }

    #[test]
    fn test_if_condition_type_error_includes_position() {
        let source = "fn test() { if 5 { return 1; } }";
        let result = compile(source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("If condition must be bool"));
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }

    #[test]
    fn test_while_condition_type_error_includes_position() {
        let source = "fn test() { while 5 { return 1; } }";
        let result = compile(source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("While condition must be bool"));
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }

    #[test]
    fn test_binary_operation_type_error_includes_position() {
        let source = "fn test() { let x = true + 5; }";
        let result = compile(source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Binary operation"));
        assert!(error.contains("numeric types"));
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }

    #[test]
    fn test_undefined_function_includes_position() {
        let source = "fn test() { let x = undefined_func(); }";
        let result = compile(source);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Undefined function"));
        assert!(error.contains("line"));
        assert!(error.contains("column"));
    }
}
