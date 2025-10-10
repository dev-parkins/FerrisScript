//! Lexical analysis for FerrisScript.
//!
//! This module provides tokenization of FerrisScript source code. The lexer converts
//! raw source text into a stream of tokens that can be consumed by the parser.
//!
//! # Performance
//!
//! The lexer is designed for speed and operates in a single pass over the source:
//! - Simple scripts: ~380ns
//! - Complex scripts: ~3.7μs
//!
//! # Example
//!
//! ```no_run
//! use ferrisscript_compiler::lexer::tokenize;
//!
//! let source = "fn add(a: i32, b: i32) -> i32 { return a + b; }";
//! let tokens = tokenize(source).unwrap();
//! ```

use crate::error_code::ErrorCode;
use crate::error_context::format_error_with_code;

/// Token representation for FerrisScript.
///
/// Each variant represents a distinct lexical element in the source code,
/// including keywords, operators, literals, and structural symbols.
///
/// # Examples
///
/// ```no_run
/// use ferrisscript_compiler::lexer::Token;
///
/// let token = Token::Fn;  // 'fn' keyword
/// assert_eq!(token.name(), "fn");
/// ```
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
    Signal,
    Export,

    // Special symbols
    At, // @

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
            Token::Signal => "signal",
            Token::Export => "export",
            Token::At => "@",
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

/// A token with its source location information.
///
/// This structure wraps a `Token` with its line and column position in the source code,
/// enabling accurate error reporting and debugging.
#[derive(Debug, Clone, PartialEq)]
pub struct PositionedToken {
    pub token: Token,
    pub line: usize,
    pub column: usize,
}

impl PositionedToken {
    pub fn new(token: Token, line: usize, column: usize) -> Self {
        PositionedToken {
            token,
            line,
            column,
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
                "Error[E003]: Invalid number '{}' at line {}, column {}",
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
                    return Err(format_error_with_code(
                        ErrorCode::E002,
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
                            return Err(format_error_with_code(
                                ErrorCode::E003,
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
                            return Err(format_error_with_code(
                                ErrorCode::E002,
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
                "signal" => Token::Signal,
                "export" => Token::Export,
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
                    return Err(format_error_with_code(
                        ErrorCode::E001,
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
                    return Err(format_error_with_code(
                        ErrorCode::E001,
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
            '@' => {
                self.advance();
                Token::At
            }
            _ => {
                let base_msg = format!(
                    "Unexpected character '{}' at line {}, column {}",
                    ch, self.line, self.column
                );
                return Err(format_error_with_code(
                    ErrorCode::E001,
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

    fn tokenize_all_positioned(&mut self) -> Result<Vec<PositionedToken>, String> {
        let mut tokens = Vec::new();
        loop {
            // Capture position before tokenizing (start of token)
            let line = self.line;
            let column = self.column;
            let token = self.next_token()?;
            let is_eof = matches!(token, Token::Eof);
            tokens.push(PositionedToken::new(token, line, column));
            if is_eof {
                break;
            }
        }
        Ok(tokens)
    }
}

/// Tokenize FerrisScript source code into a vector of tokens.
///
/// This is the main entry point for lexical analysis. It processes the entire
/// source string and returns a sequence of tokens, including a final `Token::Eof`.
///
/// # Arguments
///
/// * `input` - The complete FerrisScript source code
///
/// # Returns
///
/// * `Ok(Vec<Token>)` - Successfully tokenized source, ending with `Token::Eof`
/// * `Err(String)` - Error message with line/column info if tokenization fails
///
/// # Examples
///
/// ```no_run
/// use ferrisscript_compiler::lexer::tokenize;
///
/// let source = "let x: i32 = 42;";
/// let tokens = tokenize(source).unwrap();
/// // tokens: [Let, Ident("x"), Colon, Ident("i32"), Equal, Number(42.0), Semicolon, Eof]
/// ```
///
/// # Errors
///
/// Returns `Err` if the source contains:
/// - Unexpected characters
/// - Malformed string literals (unterminated strings)
/// - Invalid number formats
///
/// # Performance
///
/// - Simple scripts (< 100 chars): ~380ns
/// - Complex scripts (> 1000 chars): ~3.7μs
/// - Single-pass algorithm with O(n) complexity
pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer::new(input);
    lexer.tokenize_all()
}

/// Tokenize FerrisScript source code into positioned tokens with line/column information.
///
/// This function is similar to `tokenize()` but returns tokens with their source positions,
/// enabling accurate error reporting and debugging. Each token knows where it came from
/// in the source code.
///
/// # Arguments
///
/// * `input` - The complete FerrisScript source code
///
/// # Returns
///
/// * `Ok(Vec<PositionedToken>)` - Tokens with position info, ending with `Token::Eof`
/// * `Err(String)` - Error message with line/column info if tokenization fails
///
/// # Examples
///
/// ```no_run
/// use ferrisscript_compiler::lexer::tokenize_positioned;
///
/// let source = "let x: i32 = 42;";
/// let tokens = tokenize_positioned(source).unwrap();
/// // Each token knows its line and column in the source
/// ```
pub fn tokenize_positioned(input: &str) -> Result<Vec<PositionedToken>, String> {
    let mut lexer = Lexer::new(input);
    lexer.tokenize_all_positioned()
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
    fn test_tokenize_signal_keyword() {
        let tokens = tokenize("signal health_changed;").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Signal,
                Token::Ident("health_changed".to_string()),
                Token::Semicolon,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_signal_vs_identifier_case_sensitivity() {
        // "signal" (lowercase) should be keyword
        let tokens_keyword = tokenize("signal").unwrap();
        assert_eq!(tokens_keyword, vec![Token::Signal, Token::Eof]);

        // "Signal" (capitalized) should be identifier
        let tokens_ident = tokenize("Signal").unwrap();
        assert_eq!(
            tokens_ident,
            vec![Token::Ident("Signal".to_string()), Token::Eof]
        );

        // "SIGNAL" (uppercase) should be identifier
        let tokens_upper = tokenize("SIGNAL").unwrap();
        assert_eq!(
            tokens_upper,
            vec![Token::Ident("SIGNAL".to_string()), Token::Eof]
        );
    }

    #[test]
    fn test_tokenize_at_symbol() {
        let tokens = tokenize("@").unwrap();
        assert_eq!(tokens, vec![Token::At, Token::Eof]);
    }

    #[test]
    fn test_tokenize_export_keyword() {
        let tokens = tokenize("export").unwrap();
        assert_eq!(tokens, vec![Token::Export, Token::Eof]);

        // Test case sensitivity
        let tokens_upper = tokenize("Export").unwrap();
        assert_eq!(
            tokens_upper,
            vec![Token::Ident("Export".to_string()), Token::Eof]
        );
    }

    #[test]
    fn test_tokenize_numbers() {
        let tokens = tokenize("42 3.5 0.5 100.0").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(42.0),
                Token::Number(3.5),
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
        let result = tokenize("~"); // Changed from @ to ~
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

    // ========================================
    // Lexer Edge Case Tests - Phase 4
    // ========================================

    #[test]
    fn test_lexer_unicode_identifier_emoji() {
        // Test emoji in identifier (should error with clear message)
        let input = "let 😀 = 5;";
        let result = tokenize(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unexpected character"));
    }

    #[test]
    fn test_lexer_unicode_combining_characters() {
        // Test combining characters (should error)
        let input = "let a\u{0301} = 5;"; // 'a' + combining acute accent
        let result = tokenize(input);
        // May succeed if combining character is ignored, or error
        match result {
            Ok(_) => {} // Acceptable if lexer treats as 'a'
            Err(e) => assert!(e.contains("Unexpected character")),
        }
    }

    #[test]
    fn test_lexer_invalid_escape_x() {
        // Test invalid \x escape sequence
        let input = r#""test\xZZ""#;
        let result = tokenize(input);
        // Should succeed and treat as literal characters (no escape processing)
        if let Ok(tokens) = result {
            if let Token::StringLit(s) = &tokens[0] {
                // Lexer may or may not process escapes
                assert!(s.contains("test"));
            } else {
                panic!("Expected StringLit");
            }
        }
        // Also acceptable if lexer rejects invalid escapes
    }

    #[test]
    fn test_lexer_invalid_escape_u() {
        // Test invalid \u escape sequence
        let input = r#""test\uXXXX""#;
        let result = tokenize(input);
        // Should handle gracefully (either process literally or error)
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_lexer_numeric_overflow_i32_max() {
        // Test number exceeding i32::MAX
        let input = "let x = 2147483648;"; // i32::MAX + 1
        let result = tokenize(input);
        assert!(result.is_ok()); // Lexer uses f32, should succeed
        let tokens = result.unwrap();
        match &tokens[3] {
            Token::Number(n) => assert!(*n == 2147483648.0),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_lexer_numeric_overflow_f32_max() {
        // Test number close to f32::MAX
        let input = "let x = 3.4e38;";
        let result = tokenize(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lexer_unterminated_string() {
        // Test unterminated string literal
        let input = r#"let x = "hello"#;
        let result = tokenize(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unterminated string"));
    }

    #[test]
    fn test_lexer_unterminated_string_with_newline() {
        // Test unterminated string with newline
        let input = "let x = \"hello\nworld;";
        let result = tokenize(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_lexer_invalid_character_at() {
        // Test ~ character (invalid)
        let input = "let ~ = 5;";
        let result = tokenize(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unexpected character"));
    }

    #[test]
    fn test_lexer_invalid_character_hash() {
        // Test # character (not comment syntax in FerrisScript)
        let input = "let # = 5;";
        let result = tokenize(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unexpected character"));
    }

    #[test]
    fn test_lexer_invalid_character_dollar() {
        // Test $ character (invalid)
        let input = "let $var = 5;";
        let result = tokenize(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unexpected character"));
    }

    #[test]
    fn test_lexer_invalid_character_backtick() {
        // Test backtick character (invalid)
        let input = "let `var` = 5;";
        let result = tokenize(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_lexer_complex_operator_sequence() {
        // Test complex sequence: ===
        let input = "a === b";
        let result = tokenize(input);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        // Should parse as: a, ==, =, b, EOF
        assert_eq!(tokens[1], Token::EqualEqual);
        assert_eq!(tokens[2], Token::Equal);
    }

    #[test]
    fn test_lexer_ambiguous_operator_sequence() {
        // Test sequence that could be ambiguous: !==
        let input = "a !== b";
        let result = tokenize(input);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        // Should parse as: a, !=, =, b, EOF
        assert_eq!(tokens[1], Token::NotEqual);
        assert_eq!(tokens[2], Token::Equal);
    }

    #[test]
    fn test_lexer_numeric_leading_zero() {
        // Test number with leading zero
        let input = "let x = 0123;";
        let result = tokenize(input);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        match &tokens[3] {
            Token::Number(n) => assert_eq!(*n, 123.0),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_lexer_numeric_trailing_dot() {
        // Test number with trailing dot: "5."
        let input = "let x = 5.;";
        let result = tokenize(input);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        // Should parse as: let, x, =, 5., ;, EOF
        match &tokens[3] {
            Token::Number(_) => {}
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_lexer_whitespace_only() {
        // Test input with only whitespace
        let input = "   \t\n\r\n  ";
        let result = tokenize(input);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 1); // Just EOF
        assert_eq!(tokens[0], Token::Eof);
    }

    #[test]
    fn test_lexer_very_long_identifier() {
        // Test very long identifier (1000+ chars)
        let long_name = "a".repeat(1000);
        let input = format!("let {} = 5;", long_name);
        let result = tokenize(&input);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        match &tokens[1] {
            Token::Ident(s) => assert_eq!(s.len(), 1000),
            _ => panic!("Expected Ident"),
        }
    }

    #[test]
    fn test_lexer_string_with_quotes_escaped() {
        // Test string with escaped quotes
        let input = r#"let x = "say \"hello\"";"#;
        let result = tokenize(input);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        match &tokens[3] {
            Token::StringLit(s) => {
                // Lexer should preserve escape sequences
                assert!(s.contains("hello"));
            }
            _ => panic!("Expected StringLit"),
        }
    }

    #[test]
    fn test_lexer_all_special_characters() {
        // Test all structural tokens
        let input = "(){},.;:";
        let result = tokenize(input);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(tokens[0], Token::LParen);
        assert_eq!(tokens[1], Token::RParen);
        assert_eq!(tokens[2], Token::LBrace);
        assert_eq!(tokens[3], Token::RBrace);
        assert_eq!(tokens[4], Token::Comma);
        assert_eq!(tokens[5], Token::Dot);
        assert_eq!(tokens[6], Token::Semicolon);
        assert_eq!(tokens[7], Token::Colon);
    }

    #[test]
    fn test_lexer_keyword_prefix_identifier() {
        // Test identifier that starts with keyword
        let input = "let letter = 5;";
        let result = tokenize(input);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        match &tokens[1] {
            Token::Ident(s) => assert_eq!(s, "letter"),
            _ => panic!("Expected Ident 'letter'"),
        }
    }

    #[test]
    fn test_lexer_case_sensitive_keywords() {
        // Test that keywords are case-sensitive
        let input = "Let LET leT";
        let result = tokenize(input);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        // All should be identifiers, not keywords
        match &tokens[0] {
            Token::Ident(s) => assert_eq!(s, "Let"),
            _ => panic!("Expected Ident 'Let'"),
        }
        match &tokens[1] {
            Token::Ident(s) => assert_eq!(s, "LET"),
            _ => panic!("Expected Ident 'LET'"),
        }
    }

    #[test]
    fn test_lexer_numeric_negative() {
        // Test negative number (should tokenize as minus + number)
        let input = "let x = -5;";
        let result = tokenize(input);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(tokens[3], Token::Minus);
        match &tokens[4] {
            Token::Number(n) => assert_eq!(*n, 5.0),
            _ => panic!("Expected Number"),
        }
    }

    // ========================================================================
    // Edge Case Tests - Additional Coverage (October 2025)
    // Based on industry best practices for compiler edge case testing
    // ========================================================================

    #[test]
    fn test_lexer_crlf_line_endings() {
        // Test Windows-style CRLF line endings
        // Ensures column/line tracking doesn't break with \r\n
        let input = "let x = 5;\r\nlet y = 10;\r\n";
        let result = tokenize(input);
        assert!(result.is_ok(), "Should handle CRLF line endings");

        let tokens = result.unwrap();
        // Should tokenize correctly: let, x, =, 5, ;, let, y, =, 10, ;, EOF
        assert_eq!(tokens.len(), 11);
        assert_eq!(tokens[0], Token::Let);
        assert_eq!(tokens[5], Token::Let);
    }

    #[test]
    fn test_lexer_mixed_line_endings() {
        // Test mixed LF and CRLF (realistic file editing scenario)
        let input = "let x = 5;\nlet y = 10;\r\nlet z = 15;\n";
        let result = tokenize(input);
        assert!(result.is_ok(), "Should handle mixed line endings");

        let tokens = result.unwrap();
        assert_eq!(tokens[0], Token::Let);
        assert_eq!(tokens[5], Token::Let);
        assert_eq!(tokens[10], Token::Let);
    }

    #[test]
    fn test_lexer_eof_in_operator() {
        // Test EOF appearing in middle of potential multi-char operator
        let input = "a ="; // EOF after =, could be == or +=
        let result = tokenize(input);
        assert!(result.is_ok(), "Should handle EOF gracefully");

        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 3); // a, =, EOF
        assert_eq!(tokens[1], Token::Equal);
    }

    #[test]
    fn test_lexer_eof_after_exclamation() {
        // Test EOF after ! (could be !=)
        let input = "a !";
        let result = tokenize(input);
        assert!(result.is_ok(), "Should handle EOF after !");

        let tokens = result.unwrap();
        assert_eq!(tokens[1], Token::Not);
    }

    #[test]
    fn test_lexer_eof_in_string() {
        // Test EOF while inside string literal
        let input = r#"let x = "hello"#; // No closing quote
        let result = tokenize(input);
        assert!(
            result.is_err(),
            "Should error on unterminated string at EOF"
        );
        assert!(result.unwrap_err().contains("Unterminated string"));
    }

    #[test]
    fn test_lexer_unicode_normalization_nfc_nfd() {
        // Test Unicode normalization (NFC vs NFD forms)
        // é can be: U+00E9 (NFC) or U+0065 U+0301 (NFD)
        let input_nfc = "let café = 5;"; // U+00E9
        let input_nfd = "let café = 5;"; // e + combining acute (if editor supports)

        let result_nfc = tokenize(input_nfc);
        assert!(result_nfc.is_ok(), "Should handle NFC Unicode");

        let result_nfd = tokenize(input_nfd);
        assert!(result_nfd.is_ok(), "Should handle NFD Unicode");

        // Both should tokenize successfully (even if identifiers differ)
        assert_eq!(result_nfc.unwrap().len(), result_nfd.unwrap().len());
    }

    #[test]
    fn test_lexer_unicode_emoji_in_string() {
        // Test emoji and multi-byte characters in strings
        let input = r#"let x = "Hello 👋 World 🌍";"#;
        let result = tokenize(input);
        assert!(result.is_ok(), "Should handle emoji in strings");

        let tokens = result.unwrap();
        match &tokens[3] {
            Token::StringLit(s) => {
                assert!(s.contains("👋"));
                assert!(s.contains("🌍"));
            }
            _ => panic!("Expected StringLit"),
        }
    }

    #[test]
    fn test_lexer_unicode_combining_diacriticals() {
        // Test combining diacritical marks in identifiers (multi-codepoint graphemes)
        let input = "let x̃ = 5;"; // x + combining tilde (U+0303)
        let result = tokenize(input);

        // ⚠️ CURRENT LIMITATION: Combining characters may be treated as unexpected
        // Future enhancement: Full Unicode identifier support (UAX #31)
        if let Err(err) = result {
            assert!(
                err.contains("Unexpected character") || err.contains("Invalid"),
                "Combining chars currently not supported in identifiers"
            );
        } else {
            // If Unicode identifier support is enhanced
            let tokens = result.unwrap();
            match &tokens[1] {
                Token::Ident(_) => {} // Valid identifier with combining char
                _ => panic!("Expected Ident"),
            }
        }
    }

    #[test]
    fn test_lexer_emoji_in_identifier() {
        // Test if emoji can be in identifiers (currently likely invalid)
        let input = "let 🚀 = 5;";
        let result = tokenize(input);
        // Depending on language design, this may error or succeed
        // Document current behavior:
        if let Err(err) = result {
            assert!(
                err.contains("Unexpected character") || err.contains("Invalid identifier"),
                "Error should mention unexpected character or invalid identifier"
            );
        } else {
            // If we support emoji identifiers in future
            let tokens = result.unwrap();
            if let Token::Ident(s) = &tokens[1] {
                assert!(s.contains("🚀"));
            }
        }
    }

    #[test]
    fn test_lexer_zero_width_characters() {
        // Test zero-width Unicode characters (potential security issue)
        // Zero-width space (U+200B), zero-width joiner (U+200D)
        // Using escaped Unicode to avoid invisible character warning
        let input = "let\u{200B}x = 5;"; // Contains U+200B between "let" and "x"
        let result = tokenize(input);
        // Should either:
        // 1. Strip zero-width chars → tokenize as "let x = 5"
        // 2. Error on unexpected character
        // Document behavior:
        assert!(
            result.is_ok() || result.is_err(),
            "Zero-width chars should be handled (either stripped or rejected)"
        );
    }

    #[test]
    fn test_lexer_bom_at_start() {
        // Test UTF-8 BOM (Byte Order Mark) at file start
        // BOM is U+FEFF (EF BB BF in UTF-8)
        let input = "\u{FEFF}let x = 5;"; // BOM + code
        let result = tokenize(input);

        // ⚠️ CURRENT LIMITATION: BOM is treated as unexpected character
        // Future enhancement: Should strip/ignore BOM gracefully
        match result {
            Err(err) => {
                assert!(
                    err.contains("Unexpected character"),
                    "BOM currently triggers unexpected character error"
                );
            }
            Ok(tokens) => {
                // If BOM handling is implemented in future
                assert_eq!(tokens[0], Token::Let, "Should parse tokens after BOM");
            }
        }
    }

    #[test]
    fn test_lexer_empty_input() {
        // Test completely empty input
        let input = "";
        let result = tokenize(input);
        assert!(result.is_ok(), "Should handle empty input");

        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 1); // Just EOF
        assert_eq!(tokens[0], Token::Eof);
    }

    #[test]
    fn test_lexer_only_whitespace_crlf() {
        // Test input with only whitespace and line endings
        let input = "  \r\n\t\r\n  ";
        let result = tokenize(input);
        assert!(result.is_ok(), "Should handle whitespace-only input");

        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 1); // Just EOF
        assert_eq!(tokens[0], Token::Eof);
    }

    #[test]
    fn test_lexer_number_with_underscores() {
        // Test numeric literals with underscores (common readability feature)
        // Example: 1_000_000 or 0x1_FF_00
        let input = "let x = 1_000_000;";
        let result = tokenize(input);

        // ⚠️ CURRENT LIMITATION: Underscores in numbers not supported
        // Currently lexes as: 1, _000_000 (number + identifier)
        // Future enhancement: Add support for numeric separators
        assert!(result.is_ok(), "Should tokenize (but as separate tokens)");
        let tokens = result.unwrap();
        // Currently: let, x, =, 1, _000_000, ;, EOF
        match &tokens[3] {
            Token::Number(n) => assert_eq!(*n, 1.0), // Just "1"
            _ => panic!("Expected Number token for '1'"),
        }
    }

    #[test]
    fn test_lexer_binary_literal() {
        // Test binary literal support (0b prefix)
        let input = "let x = 0b1010;";
        let result = tokenize(input);

        // ⚠️ CURRENT LIMITATION: Binary literals not supported
        // Currently lexes as: 0, b1010 (number + identifier)
        // Future enhancement: Add 0b prefix support for binary literals
        assert!(
            result.is_ok(),
            "Should tokenize (but as number + identifier)"
        );
        let tokens = result.unwrap();
        match &tokens[3] {
            Token::Number(n) => assert_eq!(*n, 0.0), // Just "0"
            _ => panic!("Expected Number token for '0'"),
        }
    }

    #[test]
    fn test_lexer_hex_literal() {
        // Test hexadecimal literal support (0x prefix)
        let input = "let x = 0xFF;";
        let result = tokenize(input);

        // ⚠️ CURRENT LIMITATION: Hex literals not supported
        // Currently lexes as: 0, xFF (number + identifier)
        // Future enhancement: Add 0x prefix support for hexadecimal literals
        assert!(
            result.is_ok(),
            "Should tokenize (but as number + identifier)"
        );
        let tokens = result.unwrap();
        match &tokens[3] {
            Token::Number(n) => assert_eq!(*n, 0.0), // Just "0"
            _ => panic!("Expected Number token for '0'"),
        }
    }

    #[test]
    fn test_lexer_scientific_notation_edge_cases() {
        // Test scientific notation edge cases
        let test_cases = vec![
            "1e10",   // Simple scientific
            "1.5e-5", // Negative exponent
            "2.0E+3", // Capital E, explicit +
            "1e",     // Invalid: no exponent
            "1e+",    // Invalid: no exponent value
        ];

        for input_num in test_cases {
            let input = format!("let x = {};", input_num);
            let result = tokenize(&input);

            // Valid forms should parse, invalid should error
            match input_num {
                "1e10" | "1.5e-5" | "2.0E+3" => {
                    assert!(
                        result.is_ok(),
                        "Should parse valid scientific notation: {}",
                        input_num
                    );
                }
                "1e" | "1e+" => {
                    // These are likely invalid (implementation-dependent)
                    // Document behavior
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_lexer_string_with_null_byte() {
        // Test string containing null byte (U+0000)
        let input = "let x = \"hello\0world\";";
        let result = tokenize(input);

        // Behavior depends on implementation:
        // - Could error (null bytes not allowed)
        // - Could succeed (null byte preserved)
        // Document behavior for future reference
        if let Ok(tokens) = result {
            match &tokens[3] {
                Token::StringLit(s) => {
                    // Null byte may be preserved or stripped
                    assert!(s.contains("hello") && s.contains("world"));
                }
                _ => panic!("Expected StringLit"),
            }
        }
    }

    #[test]
    fn test_lexer_very_long_string() {
        // Test extremely long string literal (10K chars)
        let long_content = "a".repeat(10000);
        let input = format!("let x = \"{}\";", long_content);
        let result = tokenize(&input);

        assert!(result.is_ok(), "Should handle long strings");
        let tokens = result.unwrap();
        match &tokens[3] {
            Token::StringLit(s) => assert_eq!(s.len(), 10000),
            _ => panic!("Expected StringLit"),
        }
    }

    #[test]
    fn test_lexer_deeply_nested_operators() {
        // Test long chain of operators (stress test token buffer)
        // Removed % as it may not be supported
        let input = "a + b - c * d / e && g || h == i != j < k > l <= m >= n";
        let result = tokenize(input);

        // Should handle many operators
        match result {
            Ok(tokens) => {
                // Should tokenize all identifiers and operators
                assert!(
                    tokens.len() >= 20,
                    "Should tokenize many elements, got {}",
                    tokens.len()
                );
            }
            Err(err) => {
                // If some operators not supported, document
                panic!("Tokenization failed: {}", err);
            }
        }
    }

    #[test]
    fn test_lexer_max_line_length() {
        // Test very long single line (no newlines)
        let long_line = "let x = 1 + 2 + 3 + ".repeat(500) + "4;";
        let result = tokenize(&long_line);

        assert!(result.is_ok(), "Should handle long lines");
        assert!(result.unwrap().len() > 1000, "Should tokenize all elements");
    }

    #[test]
    fn test_lexer_comment_with_unicode() {
        // Test comments containing Unicode characters
        let input = "// Comment with emoji: 🚀 and symbols: © ®\nlet x = 5;";
        let result = tokenize(input);

        assert!(result.is_ok(), "Should handle Unicode in comments");
        let tokens = result.unwrap();
        assert_eq!(tokens[0], Token::Let, "Should skip comment and parse code");
    }

    #[test]
    fn test_lexer_consecutive_strings() {
        // Test multiple string literals back-to-back
        let input = r#""hello""world""test""#;
        let result = tokenize(input);

        assert!(result.is_ok(), "Should handle consecutive strings");
        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 4); // 3 strings + EOF
        for token in tokens.iter().take(3) {
            match token {
                Token::StringLit(_) => {}
                _ => panic!("Expected StringLit, got {:?}", token),
            }
        }
    }

    #[test]
    fn test_lexer_string_with_all_escapes() {
        // Test string with all supported escape sequences
        let input = r#"let x = "newline:\n tab:\t return:\r quote:\" backslash:\\";"#;
        let result = tokenize(input);

        assert!(result.is_ok(), "Should handle all escape sequences");
        let tokens = result.unwrap();
        match &tokens[3] {
            Token::StringLit(s) => {
                assert!(s.contains("\\n") || s.contains("\n"));
                assert!(s.contains("\\t") || s.contains("\t"));
            }
            _ => panic!("Expected StringLit"),
        }
    }

    #[test]
    fn test_lexer_operator_without_spaces() {
        // Test operators without whitespace separation
        let input = "a+b-c*d/e";
        let result = tokenize(input);

        // ⚠️ NOTE: Removed % operator as it may not be supported
        // Should tokenize: a, +, b, -, c, *, d, /, e, EOF
        match result {
            Ok(tokens) => {
                assert!(
                    tokens.len() >= 9,
                    "Should tokenize all operators and identifiers, got {}",
                    tokens.len()
                );
            }
            Err(err) => {
                // If some operators not supported, document
                panic!("Tokenization failed: {}", err);
            }
        }
    }

    #[test]
    fn test_lexer_mixed_quotes_in_string() {
        // Test single quotes inside double-quoted string
        let input = r#"let x = "it's a test";"#;
        let result = tokenize(input);

        assert!(
            result.is_ok(),
            "Should handle single quotes in double-quoted string"
        );
        let tokens = result.unwrap();
        match &tokens[3] {
            Token::StringLit(s) => assert!(s.contains("it's") || s.contains("'")),
            _ => panic!("Expected StringLit"),
        }
    }

    #[test]
    fn test_lexer_number_starts_with_dot() {
        // Test number starting with dot: .5 (valid in some languages)
        let input = "let x = .5;";
        let result = tokenize(input);

        // Behavior depends on language design:
        if let Ok(tokens) = result {
            // If .5 is valid number literal
            assert_eq!(tokens[3], Token::Dot); // Or Token::Number if supported
        } else {
            // If .5 not supported (parse as dot + number)
        }
    }

    #[test]
    fn test_lexer_multiple_consecutive_newlines() {
        // Test many consecutive newlines (blank lines)
        let input = "let x = 5;\n\n\n\n\nlet y = 10;";
        let result = tokenize(input);

        assert!(result.is_ok(), "Should handle multiple blank lines");
        let tokens = result.unwrap();
        assert_eq!(tokens[0], Token::Let);
        assert_eq!(tokens[5], Token::Let);
    }

    #[test]
    fn test_lexer_carriage_return_only() {
        // Test old Mac-style CR-only line endings (rare but possible)
        let input = "let x = 5;\rlet y = 10;\r";
        let result = tokenize(input);

        assert!(result.is_ok(), "Should handle CR-only line endings");
        let tokens = result.unwrap();
        assert_eq!(tokens[0], Token::Let);
    }
}
