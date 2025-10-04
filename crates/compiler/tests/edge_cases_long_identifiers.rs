//! Edge case tests for very long variable names
//!
//! Tests the compiler's behavior when handling extremely long identifiers:
//! - Variable names with 1000+ characters
//! - Function names with 1000+ characters
//! - Variable resolution with long names
//! - Performance characteristics

use ferrisscript_compiler::compile;

#[test]
fn test_long_variable_name_1000_chars() {
    // Test variable declaration with 1000-character name
    let long_name = "a".repeat(1000);
    let input = format!("let {} = 42;", long_name);

    let result = compile(&input);
    assert!(
        result.is_ok(),
        "Should handle 1000-character variable name: {:?}",
        result.err()
    );

    let ast = result.unwrap();
    assert_eq!(ast.global_vars.len(), 1, "Should have one global variable");
    assert_eq!(
        ast.global_vars[0].name, long_name,
        "Variable name should match"
    );
}

#[test]
fn test_long_function_name_1000_chars() {
    // Test function declaration with 1000-character name
    let long_name = "function_".to_string() + &"x".repeat(991);
    let input = format!("fn {}() {{ }}", long_name);

    let result = compile(&input);
    assert!(
        result.is_ok(),
        "Should handle 1000-character function name: {:?}",
        result.err()
    );

    let ast = result.unwrap();
    assert_eq!(ast.functions.len(), 1, "Should have one function");
    assert_eq!(
        ast.functions[0].name, long_name,
        "Function name should match"
    );
}

#[test]
fn test_long_variable_declaration_and_usage() {
    // Test declaring and using a variable with long name
    let long_name = "my_very_long_variable_".to_string() + &"z".repeat(978);
    let input = format!(
        "let {} = 10;\nfn test() {{ let x = {}; }}",
        long_name, long_name
    );

    let result = compile(&input);
    assert!(
        result.is_ok(),
        "Should handle long variable in declaration and usage: {:?}",
        result.err()
    );

    let ast = result.unwrap();
    assert_eq!(ast.global_vars.len(), 1, "Should have one global variable");
    assert_eq!(ast.functions.len(), 1, "Should have one function");
}

#[test]
fn test_multiple_long_variables() {
    // Test multiple variables with long names
    let name1 = "variable_one_".to_string() + &"a".repeat(987);
    let name2 = "variable_two_".to_string() + &"b".repeat(987);
    let name3 = "variable_three_".to_string() + &"c".repeat(985);

    let input = format!("let {} = 1;\nlet {} = 2;\nlet {} = 3;", name1, name2, name3);

    let result = compile(&input);
    assert!(
        result.is_ok(),
        "Should handle multiple long variable names: {:?}",
        result.err()
    );

    let ast = result.unwrap();
    assert_eq!(
        ast.global_vars.len(),
        3,
        "Should have three global variables"
    );
}

#[test]
fn test_long_parameter_name() {
    // Test function parameter with long name
    let long_param = "parameter_".to_string() + &"p".repeat(990);
    let input = format!("fn test({}: i32) {{ }}", long_param);

    let result = compile(&input);
    assert!(
        result.is_ok(),
        "Should handle long parameter name: {:?}",
        result.err()
    );

    let ast = result.unwrap();
    assert_eq!(ast.functions.len(), 1, "Should have one function");
    assert_eq!(
        ast.functions[0].params.len(),
        1,
        "Function should have one parameter"
    );
    assert_eq!(
        ast.functions[0].params[0].name, long_param,
        "Parameter name should match"
    );
}

#[test]
fn test_extremely_long_variable_5000_chars() {
    // Test with 5000-character variable name to check limits
    let long_name = "x".repeat(5000);
    let input = format!("let {} = 99;", long_name);

    let result = compile(&input);
    // Should either succeed or fail gracefully (not panic)
    if let Ok(ast) = result {
        assert_eq!(
            ast.global_vars.len(),
            1,
            "Should have one global variable if compiled"
        );
        assert_eq!(
            ast.global_vars[0].name, long_name,
            "Variable name should match"
        );
    }
    // If it fails, that's acceptable too - just document the limit
}
