pub mod lexer;
pub mod parser;
pub mod type_checker;
pub mod ast;

/// Compile RustyScript source to AST
pub fn compile(source: &str) -> Result<ast::Program, String> {
    let tokens = lexer::tokenize(source)?;
    let ast = parser::parse(&tokens)?;
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
        let source = std::fs::read_to_string(example_path("hello.rscr")).unwrap();
        assert!(compile(&source).is_ok());
    }

    #[test]
    fn test_compile_move() {
        let source = std::fs::read_to_string(example_path("move.rscr")).unwrap();
        assert!(compile(&source).is_ok());
    }

    #[test]
    fn test_compile_bounce() {
        let source = std::fs::read_to_string(example_path("bounce.rscr")).unwrap();
        assert!(compile(&source).is_ok());
    }

    #[test]
    fn test_compile_branch() {
        let source = std::fs::read_to_string(example_path("branch.rscr")).unwrap();
        let result = compile(&source);
        if let Err(e) = &result {
            eprintln!("branch.rscr error: {}", e);
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_loop() {
        let source = std::fs::read_to_string(example_path("loop.rscr")).unwrap();
        assert!(compile(&source).is_ok());
    }

    #[test]
    fn test_compile_functions() {
        let source = std::fs::read_to_string(example_path("functions.rscr")).unwrap();
        assert!(compile(&source).is_ok());
    }

    #[test]
    fn test_compile_type_error() {
        let source = std::fs::read_to_string(example_path("type_error.rscr")).unwrap();
        let result = compile(&source);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Type mismatch"));
    }

    #[test]
    fn test_compile_scene() {
        let source = std::fs::read_to_string(example_path("scene.rscr")).unwrap();
        assert!(compile(&source).is_ok());
    }

    #[test]
    fn test_compile_reload() {
        let source = std::fs::read_to_string(example_path("reload.rscr")).unwrap();
        assert!(compile(&source).is_ok());
    }
}
