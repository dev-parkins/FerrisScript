pub mod ast;
pub mod error_context;
pub mod lexer;
pub mod parser;
pub mod type_checker;

/// Compile RustyScript source to AST
pub fn compile(source: &str) -> Result<ast::Program, String> {
    let tokens = lexer::tokenize(source)?;
    let ast = parser::parse(&tokens, source)?;
    type_checker::check(&ast)?;
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
}
