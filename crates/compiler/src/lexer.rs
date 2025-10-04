use crate::error_context::format_error_with_context;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Fn,
    Let,
    Mut,
    If,
    Else,
    While,
    Return,
    True,
    False,

    // Literals
    Ident(String),
    Number(f32),
    StringLit(String),

    // Delimiters
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    Comma,     // ,
    Semicolon, // ;
    Dot,       // .
    Colon,     // :

    // Operators
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Equal,        // =
    EqualEqual,   // ==
    NotEqual,     // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=
    And,          // &&
    Or,           // ||
    Not,          // !
    PlusEqual,    // +=
    MinusEqual,   // -=

    // Special
    Eof,
}

impl Token {
    /// Returns a human-readable name for the token
    pub fn name(&self) -> &str {
        match self {
            Token::Fn => "fn",
            Token::Let => "let",
            Token::Mut => "mut",
            Token::If => "if",
            Token::Else => "else",
            Token::While => "while",
            Token::Return => "return",
            Token::True => "true",
            Token::False => "false",
            Token::Ident(_) => "identifier",
            Token::Number(_) => "number",
            Token::StringLit(_) => "string",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::Comma => ",",
            Token::Semicolon => ";",
            Token::Dot => ".",
            Token::Colon => ":",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Star => "*",
            Token::Slash => "/",
            Token::Equal => "=",
            Token::EqualEqual => "==",
            Token::NotEqual => "!=",
            Token::Less => "<",
            Token::LessEqual => "<=",
            Token::Greater => ">",
            Token::GreaterEqual => ">=",
            Token::And => "&&",
            Token::Or => "||",
            Token::Not => "!",
            Token::PlusEqual => "+=",
            Token::MinusEqual => "-=",
            Token::Eof => "end of file",
        }
    }
}

struct Lexer<'a> {
    input: Vec<char>,
    source: &'a str, // Keep original source for error context
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().collect(),
            source: input,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn current(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.input.get(self.position + offset).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.current()?;
        self.position += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current() {
            if ch.is_whitespace() {
                self.advance();
            } else if ch == '/' && self.peek(1) == Some('/') {
                // Skip line comment
                while let Some(ch) = self.current() {
                    if ch == '\n' {
                        break;
                    }
                    self.advance();
                }
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(ch) = self.current() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        ident
    }

    fn read_number(&mut self) -> Result<Token, String> {
        let mut num_str = String::new();
        let start_line = self.line;
        let start_col = self.column;

        while let Some(ch) = self.current() {
            if ch.is_numeric() || ch == '.' {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        num_str.parse::<f32>().map(Token::Number).map_err(|_| {
            format!(
                "Invalid number '{}' at line {}, column {}",
                num_str, start_line, start_col
            )
        })
    }

    fn read_string(&mut self) -> Result<Token, String> {
        let start_line = self.line;
        let start_col = self.column;

        self.advance(); // consume opening quote
        let mut string = String::new();

        loop {
            match self.current() {
                None => {
                    let base_msg = format!(
                        "Unterminated string at line {}, column {}",
                        start_line, start_col
                    );
                    return Err(format_error_with_context(
                        &base_msg,
                        self.source,
                        start_line,
                        start_col,
                        "String must be closed with \"",
                    ));
                }
                Some('"') => {
                    self.advance();
                    break;
                }
                Some('\\') => {
                    self.advance();
                    match self.current() {
                        Some('n') => {
                            string.push('\n');
                            self.advance();
                        }
                        Some('t') => {
                            string.push('\t');
                            self.advance();
                        }
                        Some('r') => {
                            string.push('\r');
                            self.advance();
                        }
                        Some('\\') => {
                            string.push('\\');
                            self.advance();
                        }
                        Some('"') => {
                            string.push('"');
                            self.advance();
                        }
                        Some(ch) => {
                            let base_msg = format!(
                                "Invalid escape sequence '\\{}' at line {}, column {}",
                                ch, self.line, self.column
                            );
                            return Err(format_error_with_context(
                                &base_msg,
                                self.source,
                                self.line,
                                self.column,
                                &format!("Unknown escape '\\{}', valid escapes are \\n \\t \\r \\\\ \\\"", ch),
                            ));
                        }
                        None => {
                            let base_msg = format!(
                                "Unterminated string at line {}, column {}",
                                start_line, start_col
                            );
                            return Err(format_error_with_context(
                                &base_msg,
                                self.source,
                                start_line,
                                start_col,
                                "String started here but never closed",
                            ));
                        }
                    }
                }
                Some(ch) => {
                    string.push(ch);
                    self.advance();
                }
            }
        }

        Ok(Token::StringLit(string))
    }

    fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        let ch = match self.current() {
            Some(c) => c,
            None => return Ok(Token::Eof),
        };

        // Identifiers and keywords
        if ch.is_alphabetic() || ch == '_' {
            let ident = self.read_identifier();
            let token = match ident.as_str() {
                "fn" => Token::Fn,
                "let" => Token::Let,
                "mut" => Token::Mut,
                "if" => Token::If,
                "else" => Token::Else,
                "while" => Token::While,
                "return" => Token::Return,
                "true" => Token::True,
                "false" => Token::False,
                _ => Token::Ident(ident),
            };
            return Ok(token);
        }

        // Numbers
        if ch.is_numeric() {
            return self.read_number();
        }

        // String literals
        if ch == '"' {
            return self.read_string();
        }

        // Multi-character operators
        let token = match ch {
            '+' => {
                self.advance();
                if self.current() == Some('=') {
                    self.advance();
                    Token::PlusEqual
                } else {
                    Token::Plus
                }
            }
            '-' => {
                self.advance();
                if self.current() == Some('=') {
                    self.advance();
                    Token::MinusEqual
                } else {
                    Token::Minus
                }
            }
            '*' => {
                self.advance();
                Token::Star
            }
            '/' => {
                self.advance();
                Token::Slash
            }
            '=' => {
                self.advance();
                if self.current() == Some('=') {
                    self.advance();
                    Token::EqualEqual
                } else {
                    Token::Equal
                }
            }
            '!' => {
                self.advance();
                if self.current() == Some('=') {
                    self.advance();
                    Token::NotEqual
                } else {
                    Token::Not
                }
            }
            '<' => {
                self.advance();
                if self.current() == Some('=') {
                    self.advance();
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            '>' => {
                self.advance();
                if self.current() == Some('=') {
                    self.advance();
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
            '&' => {
                let error_line = self.line;
                let error_col = self.column;
                self.advance();
                if self.current() == Some('&') {
                    self.advance();
                    Token::And
                } else {
                    let base_msg = format!(
                        "Unexpected character '&' at line {}, column {}",
                        error_line, error_col
                    );
                    return Err(format_error_with_context(
                        &base_msg,
                        self.source,
                        error_line,
                        error_col,
                        "Did you mean '&&' for logical AND?",
                    ));
                }
            }
            '|' => {
                let error_line = self.line;
                let error_col = self.column;
                self.advance();
                if self.current() == Some('|') {
                    self.advance();
                    Token::Or
                } else {
                    let base_msg = format!(
                        "Unexpected character '|' at line {}, column {}",
                        error_line, error_col
                    );
                    return Err(format_error_with_context(
                        &base_msg,
                        self.source,
                        error_line,
                        error_col,
                        "Did you mean '||' for logical OR?",
                    ));
                }
            }
            '(' => {
                self.advance();
                Token::LParen
            }
            ')' => {
                self.advance();
                Token::RParen
            }
            '{' => {
                self.advance();
                Token::LBrace
            }
            '}' => {
                self.advance();
                Token::RBrace
            }
            ',' => {
                self.advance();
                Token::Comma
            }
            ';' => {
                self.advance();
                Token::Semicolon
            }
            '.' => {
                self.advance();
                Token::Dot
            }
            ':' => {
                self.advance();
                Token::Colon
            }
            _ => {
                let base_msg = format!(
                    "Unexpected character '{}' at line {}, column {}",
                    ch, self.line, self.column
                );
                return Err(format_error_with_context(
                    &base_msg,
                    self.source,
                    self.line,
                    self.column,
                    "This character is not valid in FerrisScript",
                ));
            }
        };

        Ok(token)
    }

    fn tokenize_all(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            let is_eof = matches!(token, Token::Eof);
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        Ok(tokens)
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer::new(input);
    lexer.tokenize_all()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_empty() {
        let tokens = tokenize("").unwrap();
        assert_eq!(tokens, vec![Token::Eof]);
    }

    #[test]
    fn test_tokenize_whitespace_only() {
        let tokens = tokenize("   \n\t  ").unwrap();
        assert_eq!(tokens, vec![Token::Eof]);
    }

    #[test]
    fn test_tokenize_keywords() {
        let tokens = tokenize("fn let mut if else while return true false").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Fn,
                Token::Let,
                Token::Mut,
                Token::If,
                Token::Else,
                Token::While,
                Token::Return,
                Token::True,
                Token::False,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_tokenize_identifiers() {
        let tokens = tokenize("foo bar _ready _process self").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Ident("foo".to_string()),
                Token::Ident("bar".to_string()),
                Token::Ident("_ready".to_string()),
                Token::Ident("_process".to_string()),
                Token::Ident("self".to_string()),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_tokenize_numbers() {
        let tokens = tokenize("42 3.14 0.5 100.0").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(42.0),
                Token::Number(3.14),
                Token::Number(0.5),
                Token::Number(100.0),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_tokenize_strings() {
        let tokens = tokenize(r#""hello" "world" "Hello from FerrisScript!""#).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::StringLit("hello".to_string()),
                Token::StringLit("world".to_string()),
                Token::StringLit("Hello from FerrisScript!".to_string()),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_tokenize_string_escapes() {
        let tokens = tokenize(r#""line1\nline2" "tab\there" "quote\"test""#).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::StringLit("line1\nline2".to_string()),
                Token::StringLit("tab\there".to_string()),
                Token::StringLit("quote\"test".to_string()),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_tokenize_operators() {
        let tokens = tokenize("+ - * / = == != < <= > >= && || !").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Plus,
                Token::Minus,
                Token::Star,
                Token::Slash,
                Token::Equal,
                Token::EqualEqual,
                Token::NotEqual,
                Token::Less,
                Token::LessEqual,
                Token::Greater,
                Token::GreaterEqual,
                Token::And,
                Token::Or,
                Token::Not,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_tokenize_compound_assignment() {
        let tokens = tokenize("+= -=").unwrap();
        assert_eq!(
            tokens,
            vec![Token::PlusEqual, Token::MinusEqual, Token::Eof]
        );
    }

    #[test]
    fn test_tokenize_delimiters() {
        let tokens = tokenize("( ) { } , ; . :").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::RParen,
                Token::LBrace,
                Token::RBrace,
                Token::Comma,
                Token::Semicolon,
                Token::Dot,
                Token::Colon,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_tokenize_hello_example() {
        let input = r#"fn _ready() {
    print("Hello from FerrisScript!");
}"#;
        let tokens = tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Fn,
                Token::Ident("_ready".to_string()),
                Token::LParen,
                Token::RParen,
                Token::LBrace,
                Token::Ident("print".to_string()),
                Token::LParen,
                Token::StringLit("Hello from FerrisScript!".to_string()),
                Token::RParen,
                Token::Semicolon,
                Token::RBrace,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_tokenize_move_example() {
        let input = r#"fn _process(delta: f32) {
    self.position.x += 50.0 * delta;
}"#;
        let tokens = tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Fn,
                Token::Ident("_process".to_string()),
                Token::LParen,
                Token::Ident("delta".to_string()),
                Token::Colon,
                Token::Ident("f32".to_string()),
                Token::RParen,
                Token::LBrace,
                Token::Ident("self".to_string()),
                Token::Dot,
                Token::Ident("position".to_string()),
                Token::Dot,
                Token::Ident("x".to_string()),
                Token::PlusEqual,
                Token::Number(50.0),
                Token::Star,
                Token::Ident("delta".to_string()),
                Token::Semicolon,
                Token::RBrace,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_tokenize_bounce_example() {
        let input = r#"let mut dir: f32 = 1.0;

fn _process(delta: f32) {
    self.position.x += dir * 100.0 * delta;

    if self.position.x > 10.0 {
        dir = -1.0;
    }
    if self.position.x < -10.0 {
        dir = 1.0;
    }
}"#;
        let tokens = tokenize(input).unwrap();

        // Verify key tokens are present
        assert!(tokens.contains(&Token::Let));
        assert!(tokens.contains(&Token::Mut));
        assert!(tokens.contains(&Token::If));
        assert!(tokens.contains(&Token::Fn));
        assert!(tokens.contains(&Token::Ident("dir".to_string())));
        assert!(tokens.contains(&Token::Number(1.0)));
        assert!(tokens.contains(&Token::Number(100.0)));
        assert!(tokens.contains(&Token::Greater));
        assert!(tokens.contains(&Token::Less));
        assert!(tokens.last().unwrap() == &Token::Eof);
    }

    #[test]
    fn test_tokenize_line_comment() {
        let input = r#"// This is a comment
fn test() {
    // Another comment
    let x = 5; // inline comment
}"#;
        let tokens = tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Fn,
                Token::Ident("test".to_string()),
                Token::LParen,
                Token::RParen,
                Token::LBrace,
                Token::Let,
                Token::Ident("x".to_string()),
                Token::Equal,
                Token::Number(5.0),
                Token::Semicolon,
                Token::RBrace,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_tokenize_field_access_chain() {
        let tokens = tokenize("self.position.x").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Ident("self".to_string()),
                Token::Dot,
                Token::Ident("position".to_string()),
                Token::Dot,
                Token::Ident("x".to_string()),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_error_unterminated_string() {
        let result = tokenize(r#""unterminated"#);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unterminated string"));
    }

    #[test]
    fn test_error_invalid_escape() {
        let result = tokenize(r#""invalid\x escape""#);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid escape sequence"));
    }

    #[test]
    fn test_error_unexpected_character() {
        let result = tokenize("@");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unexpected character"));
    }

    #[test]
    fn test_error_single_ampersand() {
        let result = tokenize("a & b");
        assert!(result.is_err());
        let error = result.unwrap_err();
        // Error now includes context, check for key parts
        assert!(error.contains("Unexpected character '&'"));
        assert!(error.contains("&&"));
        assert!(error.contains("logical AND"));
    }

    #[test]
    fn test_error_single_pipe() {
        let result = tokenize("a | b");
        assert!(result.is_err());
        let error = result.unwrap_err();
        // Error now includes context, check for key parts
        assert!(error.contains("Unexpected character '|'"));
        assert!(error.contains("||"));
        assert!(error.contains("logical OR"));
    }

    #[test]
    fn test_token_name() {
        assert_eq!(Token::Fn.name(), "fn");
        assert_eq!(Token::Ident("foo".to_string()).name(), "identifier");
        assert_eq!(Token::Number(42.0).name(), "number");
        assert_eq!(Token::StringLit("test".to_string()).name(), "string");
        assert_eq!(Token::EqualEqual.name(), "==");
    }

    // ===== Edge Case Tests (v0.0.2) =====

    #[test]
    fn test_edge_case_comments_only_file() {
        // Test file with only comments (no actual code)
        let input = "// This is a comment\n// Another comment\n// More comments";
        let tokens = tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![Token::Eof],
            "File with only comments should produce only EOF token"
        );
    }

    #[test]
    fn test_edge_case_large_number_max() {
        // Test parsing very large numbers (approaching i64::MAX when converted)
        let input = "9223372036854775807"; // i64::MAX
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 2); // Number + EOF
        match &tokens[0] {
            Token::Number(n) => {
                assert!(*n > 0.0, "Large number should be positive");
                assert!(n.is_finite(), "Large number should be finite");
            }
            _ => panic!("Expected Number token"),
        }
    }

    #[test]
    fn test_edge_case_large_number_negative() {
        // Test parsing very large negative numbers
        let input = "-9223372036854775808"; // i64::MIN (as negative literal)
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 3); // Minus + Number + EOF
        assert_eq!(tokens[0], Token::Minus);
        match &tokens[1] {
            Token::Number(n) => {
                assert!(*n > 0.0, "Number part should be positive");
                assert!(n.is_finite(), "Large number should be finite");
            }
            _ => panic!("Expected Number token"),
        }
    }

    #[test]
    fn test_edge_case_very_small_number() {
        // Test parsing very small decimal numbers
        let input = "0.000000001";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 2);
        match &tokens[0] {
            Token::Number(n) => {
                assert!(*n > 0.0, "Small number should be positive");
                assert!(*n < 0.001, "Number should be very small");
                assert!(n.is_finite(), "Small number should be finite");
            }
            _ => panic!("Expected Number token"),
        }
    }

    #[test]
    fn test_edge_case_very_long_identifier() {
        // Test identifier with 1000 characters (stress test)
        let long_name = "a".repeat(1000);
        let tokens = tokenize(&long_name).unwrap();
        assert_eq!(tokens.len(), 2); // Identifier + EOF
        match &tokens[0] {
            Token::Ident(name) => {
                assert_eq!(name.len(), 1000, "Identifier should preserve full length");
            }
            _ => panic!("Expected Identifier token"),
        }
    }

    #[test]
    fn test_edge_case_unicode_identifier() {
        // Test identifiers with Unicode characters
        let input = "函数名称 переменная μ α β";
        let result = tokenize(input);
        // Current implementation may not support Unicode identifiers
        // This test documents the behavior
        match result {
            Ok(tokens) => {
                // If it succeeds, verify tokens are created
                assert!(tokens.len() > 1, "Should produce some tokens");
            }
            Err(e) => {
                // If it fails, that's okay - documents that Unicode isn't supported yet
                assert!(
                    e.contains("Unexpected character") || e.contains("Invalid"),
                    "Should give clear error for unsupported Unicode"
                );
            }
        }
    }

    #[test]
    fn test_edge_case_mixed_comments_and_code() {
        // Test file with comments interspersed with code
        let input = "// Start\nlet x = 5; // inline comment\n// Middle\nlet y = 10;\n// End";
        let tokens = tokenize(input).unwrap();
        // Should have: let, x, =, 5, ;, let, y, =, 10, ;, EOF
        assert_eq!(tokens.len(), 11);
        assert_eq!(tokens[0], Token::Let);
        assert_eq!(tokens[1], Token::Ident("x".to_string()));
    }

    #[test]
    fn test_edge_case_empty_string_literal() {
        // Test empty string literal ""
        let input = r#""""#;
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 2); // String + EOF
        match &tokens[0] {
            Token::StringLit(s) => {
                assert_eq!(s, "", "Empty string should be preserved");
            }
            _ => panic!("Expected String token"),
        }
    }

    #[test]
    fn test_edge_case_multiple_dots() {
        // Test multiple dots (could be confused with field access or range)
        let input = "x...y";
        let tokens = tokenize(input).unwrap();
        // Should tokenize as: x, ., ., ., y
        assert!(tokens.len() >= 5);
        assert_eq!(tokens[0], Token::Ident("x".to_string()));
        assert_eq!(tokens[1], Token::Dot);
        assert_eq!(tokens[2], Token::Dot);
        assert_eq!(tokens[3], Token::Dot);
    }

    #[test]
    fn test_edge_case_consecutive_operators() {
        // Test sequences of operators that shouldn't combine
        let input = "+-*/";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 5); // 4 operators + EOF
        assert_eq!(tokens[0], Token::Plus);
        assert_eq!(tokens[1], Token::Minus);
        assert_eq!(tokens[2], Token::Star);
        assert_eq!(tokens[3], Token::Slash);
    }
}
