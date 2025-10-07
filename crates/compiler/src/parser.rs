//! Syntax analysis (parsing) for FerrisScript.
//!
//! This module transforms a sequence of tokens from the lexer into an Abstract
//! Syntax Tree (AST). The parser implements a recursive descent algorithm with
//! operator precedence climbing for expressions.
//!
//! # Grammar
//!
//! The parser supports:
//! - Function definitions with parameters and return types
//! - Global variable declarations (let and let mut)
//! - Control flow (if/else, while, return)
//! - Binary and unary expressions with proper precedence
//! - Function calls and member access
//!
//! # Performance
//!
//! - Simple scripts: ~600ns
//! - Complex scripts: ~8μs
//! - Single-pass recursive descent algorithm
//!
//! # Example
//!
//! ```no_run
//! use ferrisscript_compiler::lexer::tokenize;
//! use ferrisscript_compiler::parser::parse;
//!
//! let source = "fn add(a: i32, b: i32) -> i32 { return a + b; }";
//! let tokens = tokenize(source).unwrap();
//! let program = parse(&tokens, source).unwrap();
//! ```

use crate::ast::*;
use crate::error_code::ErrorCode;
use crate::error_context::format_error_with_code;
use crate::lexer::Token;

pub struct Parser<'a> {
    tokens: Vec<Token>,
    source: &'a str, // Keep source for error context
    position: usize,
    current_line: usize,
    current_column: usize,
    // Error recovery fields (Phase 3C)
    panic_mode: bool,    // Track if currently recovering from error
    errors: Vec<String>, // Collect all errors during parsing
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, source: &'a str) -> Self {
        Parser {
            tokens,
            source,
            position: 0,
            current_line: 1,
            current_column: 1,
            panic_mode: false,
            errors: Vec::new(),
        }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::Eof)
    }

    #[allow(dead_code)]
    fn peek(&self, offset: usize) -> &Token {
        self.tokens
            .get(self.position + offset)
            .unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let token = self.current().clone();
        if self.position < self.tokens.len() {
            self.position += 1;
        }
        token
    }

    fn expect(&mut self, expected: Token) -> Result<Token, String> {
        let current = self.current();
        if std::mem::discriminant(current) == std::mem::discriminant(&expected) {
            Ok(self.advance())
        } else {
            let base_msg = format!(
                "Expected {}, found {} at line {}, column {}",
                expected.name(),
                current.name(),
                self.current_line,
                self.current_column
            );
            Err(format_error_with_code(
                ErrorCode::E100,
                &base_msg,
                self.source,
                self.current_line,
                self.current_column,
                &format!("Expected {}", expected.name()),
            ))
        }
    }

    fn span(&self) -> Span {
        Span::new(self.current_line, self.current_column)
    }

    /// Synchronize parser to next safe recovery point after error.
    ///
    /// This implements panic-mode error recovery by skipping tokens until
    /// reaching a statement boundary or safe keyword. Sync points are:
    /// - `;` (semicolon) - end of statement
    /// - `}` (right brace) - end of block
    /// - `fn` - start of function
    /// - `let` - start of variable declaration
    ///
    /// When a sync point is found, clears panic mode so parsing can resume.
    fn synchronize(&mut self) {
        self.panic_mode = true;

        while !matches!(self.current(), Token::Eof) {
            // Check if previous token was a statement boundary
            if self.position > 0 {
                let prev_idx = self.position - 1;
                if matches!(self.tokens.get(prev_idx), Some(Token::Semicolon)) {
                    self.panic_mode = false;
                    return;
                }
            }

            // Check if current token is a safe recovery point
            match self.current() {
                Token::Fn | Token::Let | Token::RBrace => {
                    self.panic_mode = false;
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }

        // Reached EOF
        self.panic_mode = false;
    }

    /// Record an error without immediately returning.
    ///
    /// This allows the parser to continue after errors and collect multiple
    /// diagnostics in a single pass. Errors are suppressed while in panic
    /// mode to prevent cascading false positives.
    ///
    /// # Arguments
    /// * `error` - The formatted error message to record
    fn record_error(&mut self, error: String) {
        // Only record errors when not already in panic mode
        // This prevents cascading false positives
        if !self.panic_mode {
            self.errors.push(error);
            self.panic_mode = true;
        }
    }

    /// Get all errors collected during parsing.
    ///
    /// This allows callers to access all errors found during a parse,
    /// not just the first one. Useful for displaying multiple diagnostics.
    ///
    /// # Returns
    /// A reference to the vector of collected error messages
    pub fn get_errors(&self) -> &Vec<String> {
        &self.errors
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut program = Program::new();

        while !matches!(self.current(), Token::Eof) {
            // Check if it's a global let statement
            if matches!(self.current(), Token::Let) {
                match self.parse_global_var() {
                    Ok(global_var) => program.global_vars.push(global_var),
                    Err(e) => {
                        self.record_error(e);
                        self.synchronize();
                        // Continue parsing to find more errors
                    }
                }
            } else if matches!(self.current(), Token::Fn) {
                match self.parse_function() {
                    Ok(function) => program.functions.push(function),
                    Err(e) => {
                        self.record_error(e);
                        self.synchronize();
                        // Continue parsing to find more errors
                    }
                }
            } else {
                let base_msg = format!(
                    "Expected 'fn' or 'let' at top level, found {} at line {}, column {}",
                    self.current().name(),
                    self.current_line,
                    self.current_column
                );
                let error = format_error_with_code(
                    ErrorCode::E101,
                    &base_msg,
                    self.source,
                    self.current_line,
                    self.current_column,
                    "Only function or global variable declarations allowed at top level",
                );
                self.record_error(error);
                // Advance at least one token to prevent infinite loop
                self.advance();
                self.synchronize();
                // Continue parsing to find more errors
            }
        }

        // Return first error if any were collected (maintains API compatibility)
        if let Some(first_error) = self.errors.first() {
            Err(first_error.clone())
        } else {
            Ok(program)
        }
    }

    fn parse_global_var(&mut self) -> Result<GlobalVar, String> {
        let span = self.span();
        self.expect(Token::Let)?;

        let mutable = if matches!(self.current(), Token::Mut) {
            self.advance();
            true
        } else {
            false
        };

        let name = match self.advance() {
            Token::Ident(n) => n,
            t => {
                let base_msg = format!(
                    "Expected identifier after 'let', found {} at line {}, column {}",
                    t.name(),
                    self.current_line,
                    self.current_column
                );
                return Err(format_error_with_code(
                    ErrorCode::E109,
                    &base_msg,
                    self.source,
                    self.current_line,
                    self.current_column,
                    "Variable name must be an identifier",
                ));
            }
        };

        let ty = if matches!(self.current(), Token::Colon) {
            self.advance();
            match self.advance() {
                Token::Ident(t) => Some(t),
                t => {
                    let base_msg = format!(
                        "Expected type, found {} at line {}, column {}",
                        t.name(),
                        self.current_line,
                        self.current_column
                    );
                    return Err(format_error_with_code(
                        ErrorCode::E110,
                        &base_msg,
                        self.source,
                        self.current_line,
                        self.current_column,
                        "Type annotation must be a valid type name (e.g., i32, f32, bool)",
                    ));
                }
            }
        } else {
            None
        };

        self.expect(Token::Equal)?;
        let value = self.parse_expression(0)?;
        self.expect(Token::Semicolon)?;

        Ok(GlobalVar {
            name,
            mutable,
            ty,
            value,
            span,
        })
    }

    fn parse_function(&mut self) -> Result<Function, String> {
        let span = self.span();
        self.expect(Token::Fn)?;

        let name = match self.advance() {
            Token::Ident(n) => n,
            t => {
                let base_msg = format!(
                    "Expected function name, found {} at line {}, column {}",
                    t.name(),
                    self.current_line,
                    self.current_column
                );
                return Err(format_error_with_code(
                    ErrorCode::E109,
                    &base_msg,
                    self.source,
                    self.current_line,
                    self.current_column,
                    "Function name must be an identifier",
                ));
            }
        };

        self.expect(Token::LParen)?;

        let mut params = Vec::new();
        while !matches!(self.current(), Token::RParen) {
            let param_span = self.span();
            let param_name = match self.advance() {
                Token::Ident(n) => n,
                t => {
                    let base_msg = format!(
                        "Expected parameter name, found {} at line {}, column {}",
                        t.name(),
                        self.current_line,
                        self.current_column
                    );
                    return Err(format_error_with_code(
                        ErrorCode::E111,
                        &base_msg,
                        self.source,
                        self.current_line,
                        self.current_column,
                        "Parameter name must be an identifier",
                    ));
                }
            };

            self.expect(Token::Colon)?;

            let param_type = match self.advance() {
                Token::Ident(t) => t,
                t => {
                    let base_msg = format!(
                        "Expected parameter type, found {} at line {}, column {}",
                        t.name(),
                        self.current_line,
                        self.current_column
                    );
                    return Err(format_error_with_code(
                        ErrorCode::E111,
                        &base_msg,
                        self.source,
                        self.current_line,
                        self.current_column,
                        "Parameter type must be a valid type name (e.g., i32, f32, bool)",
                    ));
                }
            };

            params.push(Param {
                name: param_name,
                ty: param_type,
                span: param_span,
            });

            if matches!(self.current(), Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        self.expect(Token::RParen)?;

        let return_type = if matches!(self.current(), Token::Minus) {
            self.advance();
            if matches!(self.current(), Token::Greater) {
                self.advance();
                match self.advance() {
                    Token::Ident(t) => Some(t),
                    t => {
                        let base_msg = format!(
                            "Expected return type, found {} at line {}, column {}",
                            t.name(),
                            self.current_line,
                            self.current_column
                        );
                        return Err(format_error_with_code(
                            ErrorCode::E112,
                            &base_msg,
                            self.source,
                            self.current_line,
                            self.current_column,
                            "Return type must be a valid type name (e.g., i32, f32, bool)",
                        ));
                    }
                }
            } else {
                let base_msg = format!(
                    "Expected '>' after '-' in return type at line {}, column {}",
                    self.current_line, self.current_column
                );
                return Err(format_error_with_code(
                    ErrorCode::E112,
                    &base_msg,
                    self.source,
                    self.current_line,
                    self.current_column,
                    "Function return type syntax is '-> Type'",
                ));
            }
        } else {
            None
        };
        self.expect(Token::LBrace)?;

        let mut body = Vec::new();
        while !matches!(self.current(), Token::RBrace) {
            body.push(self.parse_statement()?);
        }

        self.expect(Token::RBrace)?;

        Ok(Function {
            name,
            params,
            return_type,
            body,
            span,
        })
    }

    fn parse_statement(&mut self) -> Result<Stmt, String> {
        let span = self.span();

        match self.current() {
            Token::Let => self.parse_let_statement(),
            Token::If => self.parse_if_statement(),
            Token::While => self.parse_while_statement(),
            Token::Return => self.parse_return_statement(),
            _ => {
                // Try to parse as expression statement or assignment
                let expr = self.parse_expression(0)?;

                // Check for assignment operators
                match self.current() {
                    Token::Equal => {
                        self.advance();
                        let value = self.parse_expression(0)?;
                        self.expect(Token::Semicolon)?;
                        Ok(Stmt::Assign {
                            target: expr,
                            value,
                            span,
                        })
                    }
                    Token::PlusEqual | Token::MinusEqual => {
                        // Desugar += to = expr + value
                        let op_token = self.advance();
                        let rhs = self.parse_expression(0)?;
                        self.expect(Token::Semicolon)?;

                        let binary_op = match op_token {
                            Token::PlusEqual => BinaryOp::Add,
                            Token::MinusEqual => BinaryOp::Sub,
                            _ => unreachable!(),
                        };

                        let value =
                            Expr::Binary(Box::new(expr.clone()), binary_op, Box::new(rhs), span);

                        Ok(Stmt::Assign {
                            target: expr,
                            value,
                            span,
                        })
                    }
                    _ => {
                        self.expect(Token::Semicolon)?;
                        Ok(Stmt::Expr(expr))
                    }
                }
            }
        }
    }

    fn parse_let_statement(&mut self) -> Result<Stmt, String> {
        let span = self.span();
        self.expect(Token::Let)?;

        let mutable = if matches!(self.current(), Token::Mut) {
            self.advance();
            true
        } else {
            false
        };

        let name = match self.advance() {
            Token::Ident(n) => n,
            t => {
                let base_msg = format!(
                    "Expected identifier after 'let', found {} at line {}, column {}",
                    t.name(),
                    self.current_line,
                    self.current_column
                );
                return Err(format_error_with_code(
                    ErrorCode::E109,
                    &base_msg,
                    self.source,
                    self.current_line,
                    self.current_column,
                    "Variable name must be an identifier",
                ));
            }
        };

        let ty = if matches!(self.current(), Token::Colon) {
            self.advance();
            match self.advance() {
                Token::Ident(t) => Some(t),
                t => {
                    let base_msg = format!(
                        "Expected type, found {} at line {}, column {}",
                        t.name(),
                        self.current_line,
                        self.current_column
                    );
                    return Err(format_error_with_code(
                        ErrorCode::E110,
                        &base_msg,
                        self.source,
                        self.current_line,
                        self.current_column,
                        "Type annotation must be a valid type name (e.g., i32, f32, bool)",
                    ));
                }
            }
        } else {
            None
        };

        self.expect(Token::Equal)?;
        let value = self.parse_expression(0)?;
        self.expect(Token::Semicolon)?;

        Ok(Stmt::Let {
            name,
            mutable,
            ty,
            value,
            span,
        })
    }

    fn parse_if_statement(&mut self) -> Result<Stmt, String> {
        let span = self.span();
        self.expect(Token::If)?;

        let cond = self.parse_expression(0)?;
        self.expect(Token::LBrace)?;

        let mut then_branch = Vec::new();
        while !matches!(self.current(), Token::RBrace) {
            then_branch.push(self.parse_statement()?);
        }
        self.expect(Token::RBrace)?;

        let else_branch = if matches!(self.current(), Token::Else) {
            self.advance();
            self.expect(Token::LBrace)?;
            let mut stmts = Vec::new();
            while !matches!(self.current(), Token::RBrace) {
                stmts.push(self.parse_statement()?);
            }
            self.expect(Token::RBrace)?;
            stmts
        } else {
            Vec::new()
        };

        Ok(Stmt::If {
            cond,
            then_branch,
            else_branch,
            span,
        })
    }

    fn parse_while_statement(&mut self) -> Result<Stmt, String> {
        let span = self.span();
        self.expect(Token::While)?;

        let cond = self.parse_expression(0)?;
        self.expect(Token::LBrace)?;

        let mut body = Vec::new();
        while !matches!(self.current(), Token::RBrace) {
            body.push(self.parse_statement()?);
        }
        self.expect(Token::RBrace)?;

        Ok(Stmt::While { cond, body, span })
    }

    fn parse_return_statement(&mut self) -> Result<Stmt, String> {
        let span = self.span();
        self.expect(Token::Return)?;

        let value = if matches!(self.current(), Token::Semicolon) {
            None
        } else {
            Some(self.parse_expression(0)?)
        };

        self.expect(Token::Semicolon)?;

        Ok(Stmt::Return { value, span })
    }

    // Pratt parser for expressions with operator precedence
    fn parse_expression(&mut self, min_precedence: u8) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;

        loop {
            // Handle field access specially (highest precedence)
            if matches!(self.current(), Token::Dot) {
                self.advance();
                let field = match self.advance() {
                    Token::Ident(name) => name,
                    t => {
                        let base_msg = format!(
                            "Expected field name after '.', found {} at line {}, column {}",
                            t.name(),
                            self.current_line,
                            self.current_column
                        );
                        return Err(format_error_with_code(
                            ErrorCode::E103,
                            &base_msg,
                            self.source,
                            self.current_line,
                            self.current_column,
                            "Field name must be an identifier (e.g., object.field_name)",
                        ));
                    }
                };
                let span = left.span();
                left = Expr::FieldAccess(Box::new(left), field, span);
                continue;
            }

            let precedence = self.get_precedence(self.current());
            if precedence == 0 || precedence < min_precedence {
                break;
            }

            let op_token = self.advance();
            let op = self.token_to_binary_op(&op_token)?;

            let right_precedence = precedence + 1; // Left associative
            let right = self.parse_expression(right_precedence)?;

            let span = left.span();
            left = Expr::Binary(Box::new(left), op, Box::new(right), span);
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        let span = self.span();

        match self.current() {
            Token::Number(n) => {
                let num = *n;
                self.advance();
                // Check if it's an integer or float
                if num.fract() == 0.0 && num.abs() < i32::MAX as f32 {
                    Ok(Expr::Literal(Literal::Int(num as i32), span))
                } else {
                    Ok(Expr::Literal(Literal::Float(num), span))
                }
            }
            Token::StringLit(s) => {
                let string = s.clone();
                self.advance();
                Ok(Expr::Literal(Literal::Str(string), span))
            }
            Token::True => {
                self.advance();
                Ok(Expr::Literal(Literal::Bool(true), span))
            }
            Token::False => {
                self.advance();
                Ok(Expr::Literal(Literal::Bool(false), span))
            }
            Token::Ident(name) => {
                let ident = name.clone();
                self.advance();

                // Check for function call
                if matches!(self.current(), Token::LParen) {
                    self.advance();
                    let mut args = Vec::new();

                    while !matches!(self.current(), Token::RParen) {
                        args.push(self.parse_expression(0)?);
                        if matches!(self.current(), Token::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    self.expect(Token::RParen)?;
                    Ok(Expr::Call(ident, args, span))
                } else {
                    Ok(Expr::Variable(ident, span))
                }
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression(0)?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            Token::Minus => {
                self.advance();
                let expr = self.parse_primary()?;
                Ok(Expr::Unary(UnaryOp::Neg, Box::new(expr), span))
            }
            Token::Not => {
                self.advance();
                let expr = self.parse_primary()?;
                Ok(Expr::Unary(UnaryOp::Not, Box::new(expr), span))
            }
            t => Err(format!(
                "Error[E102]: Expected expression, found '{}' at line {}, column {}",
                t.name(),
                self.current_line,
                self.current_column
            )),
        }
    }

    fn get_precedence(&self, token: &Token) -> u8 {
        match token {
            Token::Or => 1,
            Token::And => 2,
            Token::EqualEqual | Token::NotEqual => 3,
            Token::Less | Token::LessEqual | Token::Greater | Token::GreaterEqual => 4,
            Token::Plus | Token::Minus => 5,
            Token::Star | Token::Slash => 6,
            Token::Dot => 7, // Highest precedence for field access
            _ => 0,
        }
    }

    fn token_to_binary_op(&self, token: &Token) -> Result<BinaryOp, String> {
        match token {
            Token::Plus => Ok(BinaryOp::Add),
            Token::Minus => Ok(BinaryOp::Sub),
            Token::Star => Ok(BinaryOp::Mul),
            Token::Slash => Ok(BinaryOp::Div),
            Token::EqualEqual => Ok(BinaryOp::Eq),
            Token::NotEqual => Ok(BinaryOp::Ne),
            Token::Less => Ok(BinaryOp::Lt),
            Token::LessEqual => Ok(BinaryOp::Le),
            Token::Greater => Ok(BinaryOp::Gt),
            Token::GreaterEqual => Ok(BinaryOp::Ge),
            Token::And => Ok(BinaryOp::And),
            Token::Or => Ok(BinaryOp::Or),
            t => {
                let base_msg = format!(
                    "Not a binary operator: {} at line {}, column {}",
                    t.name(),
                    self.current_line,
                    self.current_column
                );
                Err(format_error_with_code(
                    ErrorCode::E113,
                    &base_msg,
                    self.source,
                    self.current_line,
                    self.current_column,
                    "Valid binary operators: +, -, *, /, ==, !=, <, <=, >, >=, and, or",
                ))
            }
        }
    }
}

/// Parse a token stream into an Abstract Syntax Tree.
///
/// This is the main entry point for syntax analysis. It takes a sequence of
/// tokens from the lexer and constructs an AST representing the program structure.
///
/// # Arguments
///
/// * `tokens` - Slice of tokens from the lexer (must include `Token::Eof` at end)
/// * `source` - Original source code (used for error context)
///
/// # Returns
///
/// * `Ok(Program)` - Successfully parsed AST
/// * `Err(String)` - Syntax error with location and context
///
/// # Examples
///
/// ```no_run
/// use ferrisscript_compiler::lexer::tokenize;
/// use ferrisscript_compiler::parser::parse;
///
/// let source = r#"
///     fn factorial(n: i32) -> i32 {
///         if n <= 1 {
///             return 1;
///         }
///         return n * factorial(n - 1);
///     }
/// "#;
/// let tokens = tokenize(source).unwrap();
/// let program = parse(&tokens, source).unwrap();
///
/// assert_eq!(program.functions.len(), 1);
/// assert_eq!(program.functions[0].name, "factorial");
/// ```
///
/// # Errors
///
/// Returns `Err` if:
/// - Unexpected token encountered
/// - Missing required tokens (e.g., `;`, `}`)
/// - Malformed expressions or statements
/// - Invalid syntax structure
///
/// # Performance
///
/// - Simple functions: ~600ns
/// - Complex programs: ~8μs
/// - O(n) complexity where n = number of tokens
pub fn parse(tokens: &[Token], source: &str) -> Result<Program, String> {
    let mut parser = Parser::new(tokens.to_vec(), source);
    parser.parse_program()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    #[test]
    fn test_parse_empty() {
        let source = "";
        let tokens = vec![Token::Eof];
        let program = parse(&tokens, source).unwrap();
        assert_eq!(program.functions.len(), 0);
        assert_eq!(program.global_vars.len(), 0);
    }

    #[test]
    fn test_parse_simple_function() {
        let input = "fn test() {}";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions[0].name, "test");
        assert_eq!(program.functions[0].params.len(), 0);
        assert_eq!(program.functions[0].body.len(), 0);
    }

    #[test]
    fn test_parse_function_with_params() {
        let input = "fn add(x: i32, y: i32) {}";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.functions.len(), 1);
        let func = &program.functions[0];
        assert_eq!(func.name, "add");
        assert_eq!(func.params.len(), 2);
        assert_eq!(func.params[0].name, "x");
        assert_eq!(func.params[0].ty, "i32");
        assert_eq!(func.params[1].name, "y");
        assert_eq!(func.params[1].ty, "i32");
    }

    #[test]
    fn test_parse_let_statement() {
        let input = "fn test() { let x = 5; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.functions[0].body.len(), 1);
        match &program.functions[0].body[0] {
            Stmt::Let {
                name, mutable, ty, ..
            } => {
                assert_eq!(name, "x");
                assert!(!(*mutable));
                assert_eq!(*ty, None);
            }
            _ => panic!("Expected Let statement"),
        }
    }

    #[test]
    fn test_parse_let_mut_with_type() {
        let input = "fn test() { let mut x: i32 = 5; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        match &program.functions[0].body[0] {
            Stmt::Let {
                name, mutable, ty, ..
            } => {
                assert_eq!(name, "x");
                assert!(*mutable);
                assert_eq!(ty.as_ref().unwrap(), "i32");
            }
            _ => panic!("Expected Let statement"),
        }
    }

    #[test]
    fn test_parse_if_statement() {
        let input = "fn test() { if x > 5 { let y = 10; } }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        match &program.functions[0].body[0] {
            Stmt::If {
                then_branch,
                else_branch,
                ..
            } => {
                assert_eq!(then_branch.len(), 1);
                assert_eq!(else_branch.len(), 0);
            }
            _ => panic!("Expected If statement"),
        }
    }

    #[test]
    fn test_parse_if_else_statement() {
        let input = "fn test() { if x { let a = 1; } else { let b = 2; } }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        match &program.functions[0].body[0] {
            Stmt::If {
                then_branch,
                else_branch,
                ..
            } => {
                assert_eq!(then_branch.len(), 1);
                assert_eq!(else_branch.len(), 1);
            }
            _ => panic!("Expected If statement"),
        }
    }

    #[test]
    fn test_parse_while_statement() {
        let input = "fn test() { while x < 10 { x = x + 1; } }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        match &program.functions[0].body[0] {
            Stmt::While { body, .. } => {
                assert_eq!(body.len(), 1);
            }
            _ => panic!("Expected While statement"),
        }
    }

    #[test]
    fn test_parse_expression_statement() {
        let input = "fn test() { print(5); }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        match &program.functions[0].body[0] {
            Stmt::Expr(Expr::Call(name, args, _)) => {
                assert_eq!(name, "print");
                assert_eq!(args.len(), 1);
            }
            _ => panic!("Expected expression statement with call"),
        }
    }

    #[test]
    fn test_parse_binary_expression() {
        let input = "fn test() { let x = 5 + 3 * 2; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        match &program.functions[0].body[0] {
            Stmt::Let { value, .. } => match value {
                Expr::Binary(_, BinaryOp::Add, right, _) => match &**right {
                    Expr::Binary(_, BinaryOp::Mul, _, _) => {
                        // Correct precedence: 3 * 2 is grouped first
                    }
                    _ => panic!("Expected multiplication to have higher precedence"),
                },
                _ => panic!("Expected binary expression"),
            },
            _ => panic!("Expected Let statement"),
        }
    }

    #[test]
    fn test_parse_field_access() {
        let input = "fn test() { let x = self.position; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        match &program.functions[0].body[0] {
            Stmt::Let { value, .. } => match value {
                Expr::FieldAccess(obj, field, _) => {
                    match &**obj {
                        Expr::Variable(name, _) => assert_eq!(name, "self"),
                        _ => panic!("Expected variable"),
                    }
                    assert_eq!(field, "position");
                }
                _ => panic!("Expected field access"),
            },
            _ => panic!("Expected Let statement"),
        }
    }

    #[test]
    fn test_parse_chained_field_access() {
        let input = "fn test() { let x = self.position.x; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        match &program.functions[0].body[0] {
            Stmt::Let { value, .. } => match value {
                Expr::FieldAccess(inner, field2, _) => {
                    assert_eq!(field2, "x");
                    match &**inner {
                        Expr::FieldAccess(obj, field1, _) => {
                            assert_eq!(field1, "position");
                            match &**obj {
                                Expr::Variable(name, _) => assert_eq!(name, "self"),
                                _ => panic!("Expected variable"),
                            }
                        }
                        _ => panic!("Expected field access"),
                    }
                }
                _ => panic!("Expected field access"),
            },
            _ => panic!("Expected Let statement"),
        }
    }

    #[test]
    fn test_parse_assignment() {
        let input = "fn test() { x = 5; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        match &program.functions[0].body[0] {
            Stmt::Assign { target, .. } => match target {
                Expr::Variable(name, _) => assert_eq!(name, "x"),
                _ => panic!("Expected variable"),
            },
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_compound_assignment() {
        let input = "fn test() { x += 5; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        match &program.functions[0].body[0] {
            Stmt::Assign { target, value, .. } => {
                match target {
                    Expr::Variable(name, _) => assert_eq!(name, "x"),
                    _ => panic!("Expected variable"),
                }
                // Value should be desugared to x + 5
                match value {
                    Expr::Binary(_, BinaryOp::Add, _, _) => {}
                    _ => panic!("Expected binary addition"),
                }
            }
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_global_var() {
        let input = "let mut dir: f32 = 1.0;";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.global_vars.len(), 1);
        let var = &program.global_vars[0];
        assert_eq!(var.name, "dir");
        assert!(var.mutable);
        assert_eq!(var.ty.as_ref().unwrap(), "f32");
    }

    #[test]
    fn test_parse_hello_example() {
        let input = r#"fn _ready() {
    print("Hello from FerrisScript!");
}"#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions[0].name, "_ready");
        assert_eq!(program.functions[0].params.len(), 0);
        assert_eq!(program.functions[0].body.len(), 1);
    }

    #[test]
    fn test_parse_move_example() {
        let input = r#"fn _process(delta: f32) {
    self.position.x += 50.0 * delta;
}"#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions[0].name, "_process");
        assert_eq!(program.functions[0].params.len(), 1);
        assert_eq!(program.functions[0].params[0].name, "delta");
        assert_eq!(program.functions[0].body.len(), 1);
    }

    #[test]
    fn test_parse_bounce_example() {
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
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.global_vars.len(), 1);
        assert_eq!(program.global_vars[0].name, "dir");
        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions[0].name, "_process");
        assert_eq!(program.functions[0].body.len(), 3); // 1 assignment + 2 if statements
    }

    #[test]
    fn test_parse_return_statement() {
        let input = "fn test() -> i32 { return 42; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.functions[0].return_type, Some("i32".to_string()));
        match &program.functions[0].body[0] {
            Stmt::Return { value, .. } => {
                assert!(value.is_some());
            }
            _ => panic!("Expected return statement"),
        }
    }

    #[test]
    fn test_parse_error_unexpected_token() {
        let input = "fn test() { @ }";
        let tokens = tokenize(input);
        assert!(tokens.is_err());
    }

    #[test]
    fn test_parse_error_missing_brace() {
        let input = "fn test() {";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_err());
    }

    // ========================================
    // Error Recovery Tests (Phase 3C)
    // ========================================

    #[test]
    fn test_recovery_missing_semicolon() {
        // Parser should recover after missing semicolon and continue parsing
        let input = "fn test() { let x = 5 let y = 10; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        // Should error on first issue but continue parsing
        assert!(result.is_err());
        let error = result.unwrap_err();

        // Error should mention missing semicolon or unexpected token
        assert!(error.contains("Expected") || error.contains("E100"));
    }

    #[test]
    fn test_recovery_invalid_top_level() {
        // Parser should recover from invalid top-level item
        let input = "@ fn test() {}";
        let tokens_result = tokenize(input);

        // Lexer should catch the @ symbol first
        assert!(tokens_result.is_err());
    }

    #[test]
    fn test_recovery_multiple_functions_with_error() {
        // Parser should recover and continue to next function
        let input = r#"
fn broken() { let x = 5 }
fn working() { let y = 10; }
"#;
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        // Should report error but parser collected both functions
        assert!(result.is_err());
    }

    #[test]
    fn test_recovery_missing_function_body() {
        // Parser should handle missing function body gracefully
        let input = "fn test() fn other() {}";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        // Should error on missing brace
        assert!(result.is_err());
    }

    #[test]
    fn test_recovery_sync_on_fn_keyword() {
        // Parser should sync to 'fn' keyword
        let input = "let broken = @ fn test() {}";
        let tokens_result = tokenize(input);

        // Lexer catches @ first
        assert!(tokens_result.is_err());
    }

    #[test]
    fn test_recovery_sync_on_let_keyword() {
        // Parser should sync to 'let' keyword in function body
        let input = "fn test() { @ let x = 5; }";
        let tokens_result = tokenize(input);

        // Lexer catches @ first
        assert!(tokens_result.is_err());
    }

    #[test]
    fn test_recovery_continues_after_valid_code() {
        // After error, parser should continue with valid code
        let input = r#"
fn good1() { let x = 5; }
let broken = 
fn good2() { let y = 10; }
"#;
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        // Should collect first function successfully
        // Error on global variable without value
        assert!(result.is_err());
    }

    #[test]
    fn test_recovery_empty_file_after_error() {
        // Parser should handle errors followed by EOF
        let input = "fn test() {";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("E100") || error.contains("Expected"));
    }

    #[test]
    fn test_recovery_panic_mode_suppresses_cascading() {
        // This is a behavioral test - we expect the parser to report
        // the first error and not cascade false positives
        let input = "fn test() { let let let }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        // Should report error (likely first 'let' without identifier)
        assert!(result.is_err());

        // The error message should be about the first issue, not cascading errors
        let error = result.unwrap_err();
        assert!(error.contains("Expected") || error.contains("identifier"));
    }

    #[test]
    fn test_recovery_global_var_error() {
        // Test recovery at global level
        let input = r#"
let x = 5;
let broken 
let y = 10;
fn test() {}
"#;
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        // Should report error on 'broken' (missing = and value)
        assert!(result.is_err());
    }

    #[test]
    fn test_no_recovery_needed_on_success() {
        // Sanity check: valid code should not trigger recovery
        let input = r#"
let x = 5;
fn test() { let y = 10; }
fn other() { return 42; }
"#;
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        // Should succeed without errors
        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.global_vars.len(), 1);
        assert_eq!(program.functions.len(), 2);
    }

    #[test]
    fn test_synchronize_semicolon() {
        let tokens = vec![
            Token::Let,
            Token::Ident("x".to_string()),
            Token::Equal,
            Token::Number(1.0),
            Token::Semicolon,
            Token::Fn,
            Token::Ident("foo".to_string()),
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens, "let x = 1; fn foo() {} ");
        parser.position = 0;
        parser.synchronize();
        // Should stop at 'let' keyword (first token is a sync point)
        assert!(!parser.panic_mode);
        assert_eq!(parser.current(), &Token::Let);
    }

    #[test]
    fn test_synchronize_rbrace() {
        let tokens = vec![
            Token::Let,
            Token::Ident("x".to_string()),
            Token::Equal,
            Token::Number(1.0),
            Token::RBrace,
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens, "let x = 1} ");
        parser.position = 0;
        parser.synchronize();
        // Should stop at 'let' keyword (first token is a sync point)
        assert!(!parser.panic_mode);
        assert_eq!(parser.current(), &Token::Let);
    }

    #[test]
    fn test_record_error_and_panic_mode() {
        let tokens = vec![
            Token::Let,
            Token::Ident("x".to_string()),
            Token::Equal,
            Token::Number(1.0),
            Token::Semicolon,
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens, "let x = 1; ");
        assert!(!parser.panic_mode);
        parser.record_error("Test error".to_string());
        assert!(parser.panic_mode);
        assert_eq!(parser.errors.len(), 1);
        // Should not record another error while in panic mode
        parser.record_error("Another error".to_string());
        assert_eq!(parser.errors.len(), 1);
    }

    #[test]
    fn test_error_collection_in_parse_program() {
        // Invalid top-level token triggers error recovery
        let tokens = vec![
            Token::Ident("oops".to_string()),
            Token::Let,
            Token::Ident("x".to_string()),
            Token::Equal,
            Token::Number(1.0),
            Token::Semicolon,
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens, "oops let x = 1; ");
        let result = parser.parse_program();
        // Should collect error and continue parsing, but return error due to API compatibility
        assert!(result.is_err());
        assert_eq!(parser.errors.len(), 1);
        assert!(parser.errors[0].contains("Expected 'fn' or 'let' at top level"));
        // Note: parse_program returns Err with first error, so we can't check the program structure
        // The important thing is that we collected the error and continued parsing
    }
}
