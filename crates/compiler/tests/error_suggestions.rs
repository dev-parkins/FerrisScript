//! Integration tests for error suggestions ("Did you mean?" feature).
//!
//! These tests verify that the compiler provides helpful suggestions for:
//! - Undefined variables (E201)
//! - Undefined functions (E202)
//! - Unknown types (E203)

use ferrisscript_compiler::compile;

#[test]
fn test_variable_suggestion_close_typo() {
    // Test: Close typo (1 edit) should suggest correction
    let source = r#"
        let velocity: f32 = 100.0;
        let result = velocty;  // typo: missing 'i'
    "#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("E201"));
    assert!(error.contains("Did you mean 'velocity'?"));
}

#[test]
fn test_variable_suggestion_no_suggestion_for_distant_typo() {
    // Test: Distant typo (many edits) should not suggest
    let source = r#"
        let velocity: f32 = 100.0;
        let result = xyz;  // completely different
    "#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("E201"));
    assert!(!error.contains("Did you mean"));
}

#[test]
fn test_variable_suggestion_case_difference() {
    // Test: Case difference (1 edit) should suggest
    let source = r#"
        let velocity: f32 = 100.0;
        let result = Velocity;  // wrong case
    "#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("E201"));
    assert!(error.contains("Did you mean 'velocity'?"));
}

#[test]
fn test_variable_suggestion_multiple_candidates() {
    // Test: Multiple similar variables - should suggest closest
    let source = r#"
        let velocity: f32 = 100.0;
        let velocities: f32 = 200.0;
        let result = velocty;  // typo
    "#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("E201"));
    // Should suggest 'velocity' (1 edit) over 'velocities' (2 edits)
    assert!(error.contains("Did you mean 'velocity'?"));
}

#[test]
fn test_variable_suggestion_short_identifier() {
    // Test: Short identifier (≤4 chars) with close typo
    let source = r#"
        let pos: i32 = 10;
        let result = poz;  // 1 edit
    "#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("E201"));
    assert!(error.contains("Did you mean 'pos'?"));
}

#[test]
fn test_variable_suggestion_long_identifier() {
    // Test: Long identifier (>8 chars) with typo
    let source = r#"
        let initialization: i32 = 42;
        let result = initialisation;  // 1 edit (s vs z)
    "#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("E201"));
    // Should suggest due to high similarity percentage
    assert!(error.contains("Did you mean 'initialization'?"));
}

#[test]
fn test_variable_no_suggestion_empty_scope() {
    // Test: No variables in scope - no suggestion
    let source = r#"
        fn test() {
            let result = velocity;  // no velocity defined
        }
    "#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("E201"));
    assert!(!error.contains("Did you mean"));
}

#[test]
fn test_function_suggestion_close_typo() {
    // Test: Close function name typo should suggest
    let source = r#"
        fn test() {
            pirnt("Hello");  // typo: print
        }
    "#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("E202"));
    assert!(error.contains("Did you mean 'print'?"));
}

#[test]
fn test_function_suggestion_no_suggestion_for_distant() {
    // Test: Very different function name - no suggestion
    let source = r#"
        fn test() {
            xyz();  // completely different from "print"
        }
    "#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("E202"));
    assert!(!error.contains("Did you mean"));
}

#[test]
fn test_function_suggestion_user_defined() {
    // Test: User-defined function with typo
    let source = r#"
        fn calculate(x: i32) -> i32 {
            return x * 2;
        }
        
        fn test() {
            let result = calcuate(10);  // typo: missing 'l'
        }
    "#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("E202"));
    assert!(error.contains("Did you mean 'calculate'?"));
}

#[test]
fn test_type_suggestion_vector2_typo() {
    // Test: Built-in type with typo (using function parameter)
    let source = r#"
        fn test(pos: Vectorr2) {  // typo: extra 'r'
        }
    "#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("E203"));
    assert!(error.contains("Did you mean 'Vector2'?"));
}

#[test]
fn test_type_suggestion_primitive_typo() {
    // Test: Primitive type with typo (using function parameter)
    let source = r#"
        fn test(x: i3) {  // typo: i32 (missing '2')
        }
    "#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("E203"));
    assert!(error.contains("Did you mean 'i32'?"));
}

#[test]
fn test_type_suggestion_string_typo() {
    // Test: String type with typo (using function parameter)
    let source = r#"
        fn test(name: Strng) {  // typo: missing 'i'
        }
    "#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("E203"));
    assert!(error.contains("Did you mean 'String'?"));
}

#[test]
fn test_type_suggestion_no_suggestion_for_distant() {
    // Test: Very different type name - no suggestion, list available types
    let source = r#"
        fn test(x: XYZ) {  // completely unknown
        }
    "#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("E203"));
    assert!(!error.contains("Did you mean"));
    assert!(error.contains("Available types"));
}

#[test]
fn test_type_suggestion_node_typo() {
    // Test: Node type with typo (using function parameter)
    let source = r#"
        fn test(obj: Nodee) {  // typo: extra 'e'
        }
    "#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("E203"));
    assert!(error.contains("Did you mean 'Node'?"));
}

#[test]
fn test_type_suggestion_bool_typo() {
    // Test: Bool type with typo (using function parameter)
    let source = r#"
        fn test(flag: boool) {  // typo: extra 'o'
        }
    "#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("E203"));
    assert!(error.contains("Did you mean 'bool'?"));
}

#[test]
fn test_multiple_errors_with_suggestions() {
    // Test: Multiple errors each get suggestions
    let source = r#"
        let velocity: f32 = 100.0;
        
        fn test(y: Vectorr2) {  // E203 - type typo
            let x = velocty;    // E201 - variable typo
            pirnt("Hello");     // E202 - function typo
        }
    "#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    // All three error codes should appear
    assert!(error.contains("E201"));
    assert!(error.contains("E202"));
    assert!(error.contains("E203"));

    // All three suggestions should appear
    assert!(error.contains("Did you mean 'velocity'?"));
    assert!(error.contains("Did you mean 'print'?"));
    assert!(error.contains("Did you mean 'Vector2'?"));
}

#[test]
fn test_suggestion_with_self_variable() {
    // Test: 'self' is available as a variable
    let source = r#"
        fn test() {
            let x = slf;  // typo: self
        }
    "#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("E201"));
    assert!(error.contains("Did you mean 'self'?"));
}

#[test]
fn test_exact_match_no_error() {
    // Test: Exact match should not produce error (sanity check)
    let source = r#"
        let velocity: f32 = 100.0;
        let result = velocity;  // exact match - no error
    "#;

    let result = compile(source);
    assert!(result.is_ok());
}

#[test]
fn test_transposition_typo() {
    // Test: Transposition errors (2 edits) in medium-length identifiers
    let source = r#"
        let position: i32 = 10;
        let x = posiition;  // transposition + insertion
    "#;

    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert!(error.contains("E201"));
    // Should suggest due to being within threshold for 8-char identifier
    assert!(error.contains("Did you mean 'position'?"));
}
