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
    #[test]
    fn test_compile_placeholder() {
        // Placeholder test - will be implemented as features are added
        assert!(true);
    }
}
