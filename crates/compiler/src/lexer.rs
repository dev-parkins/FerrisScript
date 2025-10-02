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
    LParen,     // (
    RParen,     // )
    LBrace,     // {
    RBrace,     // }
    Comma,      // ,
    Semicolon,  // ;
    Dot,        // .
    Colon,      // :
    
    // Operators
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Equal,          // =
    EqualEqual,     // ==
    NotEqual,       // !=
    Less,           // <
    LessEqual,      // <=
    Greater,        // >
    GreaterEqual,   // >=
    And,            // &&
    Or,             // ||
    Not,            // !
    PlusEqual,      // +=
    MinusEqual,     // -=
    
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

struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
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

        num_str
            .parse::<f32>()
            .map(Token::Number)
            .map_err(|_| {
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
                    return Err(format!(
                        "Unterminated string at line {}, column {}",
                        start_line, start_col
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
                            return Err(format!(
                                "Invalid escape sequence '\\{}' at line {}, column {}",
                                ch, self.line, self.column
                            ));
                        }
                        None => {
                            return Err(format!(
                                "Unterminated string at line {}, column {}",
                                start_line, start_col
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
                self.advance();
                if self.current() == Some('&') {
                    self.advance();
                    Token::And
                } else {
                    return Err(format!(
                        "Unexpected character '&' at line {}, column {}. Did you mean '&&'?",
                        self.line, self.column - 1
                    ));
                }
            }
            '|' => {
                self.advance();
                if self.current() == Some('|') {
                    self.advance();
                    Token::Or
                } else {
                    return Err(format!(
                        "Unexpected character '|' at line {}, column {}. Did you mean '||'?",
                        self.line, self.column - 1
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
                return Err(format!(
                    "Unexpected character '{}' at line {}, column {}",
                    ch, self.line, self.column
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
        assert!(result.unwrap_err().contains("Did you mean '&&'?"));
    }

    #[test]
    fn test_error_single_pipe() {
        let result = tokenize("a | b");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Did you mean '||'?"));
    }

    #[test]
    fn test_token_name() {
        assert_eq!(Token::Fn.name(), "fn");
        assert_eq!(Token::Ident("foo".to_string()).name(), "identifier");
        assert_eq!(Token::Number(42.0).name(), "number");
        assert_eq!(Token::StringLit("test".to_string()).name(), "string");
        assert_eq!(Token::EqualEqual.name(), "==");
    }
}
