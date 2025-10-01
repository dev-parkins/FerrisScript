use crate::lexer::Token;
use crate::ast::Program;

pub fn parse(tokens: &[Token]) -> Result<Program, String> {
    // Placeholder: parse tokens into AST
    // TODO: Implement proper parser in Phase 3
    Ok(Program { functions: vec![] })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let tokens = vec![Token::Eof];
        let program = parse(&tokens).unwrap();
        assert_eq!(program.functions.len(), 0);
    }
}
