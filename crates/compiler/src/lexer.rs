#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Fn,
    Let,
    Mut,
    If,
    Else,
    While,
    
    // Literals
    Ident(String),
    Number(f32),
    Bool(bool),
    String(String),
    
    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Dot,
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Not,
    
    // Special
    Eof,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    // Placeholder: tokenize input source
    // TODO: Implement proper lexer in Phase 2
    Ok(vec![Token::Eof])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_empty() {
        let tokens = tokenize("").unwrap();
        assert_eq!(tokens, vec![Token::Eof]);
    }
}
