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
use crate::lexer::{PositionedToken, Token};
use crate::span::{Position, Span};

pub struct Parser<'a> {
    tokens: Vec<PositionedToken>,
    source: &'a str, // Keep source for error context
    position: usize,
    current_line: usize,
    current_column: usize,
    // Error recovery fields (Phase 3C)
    panic_mode: bool,    // Track if currently recovering from error
    errors: Vec<String>, // Collect all errors during parsing
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<PositionedToken>, source: &'a str) -> Self {
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
        self.tokens
            .get(self.position)
            .map(|pt| &pt.token)
            .unwrap_or(&Token::Eof)
    }

    fn current_position(&self) -> (usize, usize) {
        self.tokens
            .get(self.position)
            .map(|pt| (pt.line, pt.column))
            .unwrap_or((1, 1))
    }

    #[allow(dead_code)]
    fn peek(&self, offset: usize) -> &Token {
        self.tokens
            .get(self.position + offset)
            .map(|pt| &pt.token)
            .unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let token = self.current().clone();
        if self.position < self.tokens.len() {
            // Update current_line and current_column from token position
            if let Some(pt) = self.tokens.get(self.position) {
                self.current_line = pt.line;
                self.current_column = pt.column;
            }
            self.position += 1;
        }
        token
    }

    fn expect(&mut self, expected: Token) -> Result<Token, String> {
        let current = self.current();
        let (line, column) = self.current_position();
        if std::mem::discriminant(current) == std::mem::discriminant(&expected) {
            Ok(self.advance())
        } else {
            let base_msg = format!(
                "Expected {}, found {} at line {}, column {}",
                expected.name(),
                current.name(),
                line,
                column
            );
            Err(format_error_with_code(
                ErrorCode::E100,
                &base_msg,
                self.source,
                line,
                column,
                &format!("Expected {}", expected.name()),
            ))
        }
    }

    fn span(&self) -> Span {
        // TODO(v0.0.5): Track actual byte offsets during parsing
        // For now, use offset 0 (unknown) and create zero-length spans
        let pos = Position::new(self.current_line, self.current_column, 0);
        Span::point(pos)
    }

    /// Create a span from a start position to the current position.
    ///
    /// This is used when parsing multi-token constructs to create a span
    /// that covers the entire construct.
    ///
    /// # Arguments
    ///
    /// * `start_line` - The line where the construct started
    /// * `start_column` - The column where the construct started
    ///
    /// # Returns
    ///
    /// A span from the start position to the current position
    #[allow(dead_code)]
    fn span_from(&self, start_line: usize, start_column: usize) -> Span {
        // TODO(v0.0.5): Track actual byte offsets during parsing
        // For now, use offset 0 (unknown)
        let start_pos = Position::new(start_line, start_column, 0);
        let end_pos = Position::new(self.current_line, self.current_column, 0);
        Span::new(start_pos, end_pos)
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
                if let Some(pt) = self.tokens.get(prev_idx) {
                    if matches!(pt.token, Token::Semicolon) {
                        self.panic_mode = false;
                        return;
                    }
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
            // Check if it's a global let statement (with or without @export)
            if matches!(self.current(), Token::Let | Token::At) {
                match self.parse_global_var() {
                    Ok(global_var) => program.global_vars.push(global_var),
                    Err(e) => {
                        self.record_error(e);
                        self.synchronize();
                        // Continue parsing to find more errors
                    }
                }
            } else if matches!(self.current(), Token::Signal) {
                match self.parse_signal_declaration() {
                    Ok(signal) => program.signals.push(signal),
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
                    "Expected 'fn', 'let', or 'signal' at top level, found {} at line {}, column {}",
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

    /// Parse @export annotation with optional property hints
    ///
    /// Supports:
    /// - `@export` - No hint
    /// - `@export(range(min, max, step))` - Range hint for numeric sliders
    /// - `@export(file("*.ext1", "*.ext2"))` - File picker hint with extensions
    /// - `@export(enum("Value1", "Value2"))` - Dropdown hint with predefined values
    fn parse_export_annotation(&mut self) -> Result<Option<ExportAnnotation>, String> {
        if !matches!(self.current(), Token::At) {
            return Ok(None);
        }

        let span = self.span();
        self.expect(Token::At)?;
        self.expect(Token::Export)?;

        // Check for property hint in parentheses
        let hint = if matches!(self.current(), Token::LParen) {
            self.advance(); // consume '('

            // Parse hint type (identifier)
            if let Token::Ident(hint_type) = self.current() {
                let hint_name = hint_type.clone();
                self.advance();

                match hint_name.as_str() {
                    "range" => {
                        // Parse range(min, max, step)
                        self.expect(Token::LParen)?;

                        // Parse min
                        let min = self.parse_number("range hint min value")?;
                        self.expect(Token::Comma)?;

                        // Parse max
                        let max = self.parse_number("range hint max value")?;
                        self.expect(Token::Comma)?;

                        // Parse step
                        let step = self.parse_number("range hint step value")?;

                        self.expect(Token::RParen)?; // close range()
                        self.expect(Token::RParen)?; // close @export()

                        PropertyHint::Range { min, max, step }
                    }
                    "file" => {
                        // Parse file("*.ext1", "*.ext2", ...)
                        self.expect(Token::LParen)?;

                        let mut extensions = Vec::new();

                        // Parse at least one extension
                        if let Token::StringLit(ext) = self.current() {
                            extensions.push(ext.clone());
                            self.advance();
                        } else {
                            return Err(format!(
                                "Expected string literal for file extension, found {}",
                                self.current().name()
                            ));
                        }

                        // Parse additional extensions separated by commas
                        while matches!(self.current(), Token::Comma) {
                            self.advance(); // consume comma

                            if let Token::StringLit(ext) = self.current() {
                                extensions.push(ext.clone());
                                self.advance();
                            } else {
                                return Err(format!(
                                    "Expected string literal for file extension after comma, found {}",
                                    self.current().name()
                                ));
                            }
                        }

                        self.expect(Token::RParen)?; // close file()
                        self.expect(Token::RParen)?; // close @export()

                        PropertyHint::File { extensions }
                    }
                    "enum" => {
                        // Parse enum("Value1", "Value2", ...)
                        self.expect(Token::LParen)?;

                        let mut values = Vec::new();

                        // Parse at least one value
                        if let Token::StringLit(val) = self.current() {
                            values.push(val.clone());
                            self.advance();
                        } else {
                            return Err(format!(
                                "Expected string literal for enum value, found {}",
                                self.current().name()
                            ));
                        }

                        // Parse additional values separated by commas
                        while matches!(self.current(), Token::Comma) {
                            self.advance(); // consume comma

                            if let Token::StringLit(val) = self.current() {
                                values.push(val.clone());
                                self.advance();
                            } else {
                                return Err(format!(
                                    "Expected string literal for enum value after comma, found {}",
                                    self.current().name()
                                ));
                            }
                        }

                        self.expect(Token::RParen)?; // close enum()
                        self.expect(Token::RParen)?; // close @export()

                        PropertyHint::Enum { values }
                    }
                    _ => {
                        return Err(format!(
                            "Unknown property hint '{}'. Expected 'range', 'file', or 'enum'",
                            hint_name
                        ));
                    }
                }
            } else {
                return Err(format!(
                    "Expected property hint name after @export(, found {}",
                    self.current().name()
                ));
            }
        } else {
            PropertyHint::None
        };

        Ok(Some(ExportAnnotation { hint, span }))
    }

    /// Helper to parse a numeric literal for property hints
    fn parse_number(&mut self, context: &str) -> Result<f32, String> {
        match self.current() {
            Token::Number(val) => {
                let num = *val;
                self.advance();
                Ok(num)
            }
            Token::Minus => {
                self.advance();
                match self.current() {
                    Token::Number(val) => {
                        let num = -*val;
                        self.advance();
                        Ok(num)
                    }
                    _ => Err(format!(
                        "Expected number for {}, found {}",
                        context,
                        self.current().name()
                    )),
                }
            }
            _ => Err(format!(
                "Expected number for {}, found {}",
                context,
                self.current().name()
            )),
        }
    }

    fn parse_global_var(&mut self) -> Result<GlobalVar, String> {
        let span = self.span();

        // Check for @export annotation before 'let'
        let export = self.parse_export_annotation()?;

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
            export, // Parsed in Checkpoint 1.2
            span,
        })
    }

    fn parse_signal_declaration(&mut self) -> Result<Signal, String> {
        let span = self.span();
        self.expect(Token::Signal)?;

        let name = match self.advance() {
            Token::Ident(n) => n,
            t => {
                let base_msg = format!(
                    "Expected signal name, found {} at line {}, column {}",
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
                    "Signal name must be an identifier",
                ));
            }
        };

        self.expect(Token::LParen)?;

        let mut parameters = Vec::new();
        while !matches!(self.current(), Token::RParen) {
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
                        ErrorCode::E109,
                        &base_msg,
                        self.source,
                        self.current_line,
                        self.current_column,
                        "Signal parameter name must be an identifier",
                    ));
                }
            };

            self.expect(Token::Colon)?;

            let param_type = match self.advance() {
                Token::Ident(t) => t,
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
                        "Signal parameter type must be a valid type name (e.g., i32, f32, bool)",
                    ));
                }
            };

            parameters.push((param_name, param_type));

            if !matches!(self.current(), Token::RParen) {
                self.expect(Token::Comma)?;
            }
        }

        self.expect(Token::RParen)?;
        self.expect(Token::Semicolon)?;

        Ok(Signal {
            name,
            parameters,
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
            Token::Let | Token::At => self.parse_let_statement(),
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

        // Check for @export annotation before 'let'
        let export = self.parse_export_annotation()?;

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
            export, // Parsed in Checkpoint 1.2
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

                // Check for struct literal: Identifier '{' (only if identifier starts with uppercase)
                // This prevents parsing `if x { ... }` as a struct literal
                if matches!(self.current(), Token::LBrace)
                    && ident.chars().next().is_some_and(|c| c.is_uppercase())
                {
                    return self.parse_struct_literal(ident, span);
                }

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

    /// Parse struct literal: `TypeName { field1: expr1, field2: expr2 }`
    /// MVP: Does NOT support nested struct literals (e.g., Rect2 { position: Vector2 { x: 0.0, y: 0.0 } })
    /// Use variable references instead: let pos = ...; Rect2 { position: pos, ... }
    fn parse_struct_literal(&mut self, type_name: String, span: Span) -> Result<Expr, String> {
        // Already consumed TypeName, now expect '{'
        self.expect(Token::LBrace)?;

        let mut fields = Vec::new();

        // Parse fields: field_name: expr, field_name: expr, ...
        loop {
            // Check for closing brace
            if matches!(self.current(), Token::RBrace) {
                break;
            }

            // Parse field name
            let field_name = match self.current() {
                Token::Ident(name) => name.clone(),
                t => {
                    return Err(format!(
                        "Error[E704]: Expected field name in {} literal, found '{}' at line {}, column {}",
                        type_name,
                        t.name(),
                        self.current_line,
                        self.current_column
                    ))
                }
            };
            self.advance();

            // Expect colon
            self.expect(Token::Colon)?;

            // Parse field value expression
            // MVP: Parse simple expressions only (no nested struct literals for now)
            let field_expr = self.parse_expression(0)?;

            fields.push((field_name, field_expr));

            // Check for comma or end
            if matches!(self.current(), Token::Comma) {
                self.advance();
                // Allow trailing comma
                if matches!(self.current(), Token::RBrace) {
                    break;
                }
            } else {
                // No comma means we should see closing brace
                break;
            }
        }

        self.expect(Token::RBrace)?;

        Ok(Expr::StructLiteral {
            type_name,
            fields,
            span,
        })
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
    // Convert tokens to positioned tokens for backwards compatibility
    let positioned_tokens: Vec<PositionedToken> = tokens
        .iter()
        .map(|t| PositionedToken::new(t.clone(), 1, 1))
        .collect();
    let mut parser = Parser::new(positioned_tokens, source);
    parser.parse_program()
}

/// Parse positioned tokens (with line/column info) into an AST program.
///
/// This function provides accurate error reporting with correct line and column numbers
/// by using tokens that carry their source position information.
///
/// # Performance
///
/// - Simple functions: ~600ns
/// - Complex programs: ~8μs
/// - O(n) complexity where n = number of tokens
pub fn parse_positioned(tokens: &[PositionedToken], source: &str) -> Result<Program, String> {
    let mut parser = Parser::new(tokens.to_vec(), source);
    parser.parse_program()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    // Helper function to convert tokens to positioned tokens for testing
    fn to_positioned(tokens: Vec<Token>) -> Vec<PositionedToken> {
        tokens
            .into_iter()
            .map(|t| PositionedToken::new(t, 1, 1))
            .collect()
    }

    #[test]
    fn test_parse_empty() {
        let source = "";
        let tokens = vec![Token::Eof];
        let program = parse(&tokens, source).unwrap();
        assert_eq!(program.functions.len(), 0);
        assert_eq!(program.global_vars.len(), 0);
        assert_eq!(program.signals.len(), 0);
    }

    #[test]
    fn test_parse_signal_no_params() {
        let input = "signal player_died();";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.signals.len(), 1);
        let signal = &program.signals[0];
        assert_eq!(signal.name, "player_died");
        assert_eq!(signal.parameters.len(), 0);
    }

    #[test]
    fn test_parse_signal_one_param() {
        let input = "signal health_changed(new_health: i32);";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.signals.len(), 1);
        let signal = &program.signals[0];
        assert_eq!(signal.name, "health_changed");
        assert_eq!(signal.parameters.len(), 1);
        assert_eq!(signal.parameters[0].0, "new_health");
        assert_eq!(signal.parameters[0].1, "i32");
    }

    #[test]
    fn test_parse_signal_multiple_params() {
        let input = "signal score_changed(old: i32, new: i32, reason: String);";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.signals.len(), 1);
        let signal = &program.signals[0];
        assert_eq!(signal.name, "score_changed");
        assert_eq!(signal.parameters.len(), 3);
        assert_eq!(signal.parameters[0], ("old".to_string(), "i32".to_string()));
        assert_eq!(signal.parameters[1], ("new".to_string(), "i32".to_string()));
        assert_eq!(
            signal.parameters[2],
            ("reason".to_string(), "String".to_string())
        );
    }

    #[test]
    fn test_parse_signal_missing_semicolon() {
        let input = "signal player_died()";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Expected ;"));
    }

    #[test]
    fn test_parse_signal_missing_parens() {
        let _input = "signal player_died;";
        let tokens = vec![
            Token::Signal,
            Token::Ident("player_died".to_string()),
            Token::Semicolon,
            Token::Eof,
        ];
        let result = parse(&tokens, "signal player_died;");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Expected ("));
    }

    #[test]
    fn test_parse_signal_invalid_param_syntax() {
        let input = "signal test(x y);";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_err());
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
        let input = "fn test() { ~ }";
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
        let input = "~ fn test() {}";
        let tokens_result = tokenize(input);

        // Lexer should catch the ~ symbol first
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
        let input = "let broken = ~ fn test() {}";
        let tokens_result = tokenize(input);

        // Lexer catches ~ first
        assert!(tokens_result.is_err());
    }

    #[test]
    fn test_recovery_sync_on_let_keyword() {
        // Parser should sync to 'let' keyword in function body
        let input = "fn test() { ~ let x = 5; }";
        let tokens_result = tokenize(input);

        // Lexer catches ~ first
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
        let mut parser = Parser::new(to_positioned(tokens), "let x = 1; fn foo() {} ");
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
        let mut parser = Parser::new(to_positioned(tokens), "let x = 1} ");
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
        let mut parser = Parser::new(to_positioned(tokens), "let x = 1; ");
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
        let mut parser = Parser::new(to_positioned(tokens), "oops let x = 1; ");
        let result = parser.parse_program();
        // Should collect error and continue parsing, but return error due to API compatibility
        assert!(result.is_err());
        assert_eq!(parser.errors.len(), 1);
        assert!(parser.errors[0].contains("Expected 'fn', 'let', or 'signal' at top level"));
        // Note: parse_program returns Err with first error, so we can't check the program structure
        // The important thing is that we collected the error and continued parsing
    }

    // ========================================
    // Parser Error Recovery Tests - Phase 3
    // ========================================

    #[test]
    fn test_parser_recovery_sync_after_semicolon() {
        // Parser should sync after semicolon
        let input = "fn test() { let x = 5; let y = 10; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parser_recovery_sync_after_rbrace() {
        // Parser should sync after right brace
        let input = "fn test() { let x = 5; } fn other() { let y = 10; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parser_recovery_batch_errors() {
        // Parser should collect multiple errors without cascading
        let input = "fn test() { let x = 5 let y = 10 let z = 15; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_err());
        // Should report first error only (API limitation)
    }

    #[test]
    fn test_parser_recovery_unclosed_brace_sync_to_fn() {
        // Parser should sync to 'fn' after unclosed brace
        let input = "fn broken() { let x = 5; fn other() {}";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_recovery_sync_to_let() {
        // Parser should sync to 'let' keyword
        let input = "fn test() { let x = 5 let y = 10; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_recovery_missing_semicolon_batch() {
        // Parser should handle multiple missing semicolons
        let input = "fn test() { let x = 5 let y = 10 let z = 15 }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_recovery_invalid_expression_sync() {
        // Parser should recover from invalid expression
        let input = "fn test() { let x = ; let y = 10; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_recovery_missing_function_param_type() {
        // Parser should handle missing parameter type
        let input = "fn test(x) {}";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_recovery_missing_return_type() {
        // Parser should handle missing return type after arrow
        let input = "fn test() -> {}";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_recovery_cascading_suppression() {
        // Parser should suppress cascading errors in panic mode
        let tokens = vec![
            Token::Let,
            Token::Ident("x".to_string()),
            Token::Equal,
            // Missing value
            Token::Semicolon,
            Token::Eof,
        ];
        let mut parser = Parser::new(to_positioned(tokens), "let x = ;");
        let result = parser.parse_program();
        assert!(result.is_err());
        // Should only record first error due to panic mode
    }

    #[test]
    fn test_parser_recovery_top_level_vs_function_level() {
        // Parser should differentiate top-level and function-level errors
        let input = "fn test() { let x = 5 } fn other() { let y = 10; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_recovery_nested_blocks() {
        // Parser should recover in nested blocks
        let input = "fn test() { if (true) { let x = 5 } }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_recovery_while_loop_error() {
        // Parser should recover from while loop errors
        let input = "fn test() { while (true) { let x = 5 } }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_recovery_multiple_functions() {
        // Parser should recover across multiple function definitions
        let input = r#"
fn first() { let x = 5; }
fn second() { let y = 10 }
fn third() { let z = 15; }
"#;
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_recovery_global_let_error() {
        // Parser should recover from global variable errors
        let input = "let x = 5 let y = 10;";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_recovery_sync_at_eof() {
        // Parser should handle sync at EOF gracefully
        let tokens = vec![
            Token::Ident("invalid".to_string()),
            Token::Number(1.0),
            // No sync points, should reach EOF
            Token::Eof,
        ];
        let mut parser = Parser::new(to_positioned(tokens), "invalid 1");
        parser.synchronize();
        assert!(!parser.panic_mode);
        assert_eq!(parser.current(), &Token::Eof);
    }

    #[test]
    fn test_parser_recovery_function_call_error() {
        // Parser should recover from function call errors
        let input = "fn test() { foo( }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_recovery_binary_op_error() {
        // Parser should recover from binary operation errors
        let input = "fn test() { let x = 5 + ; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_recovery_member_access_error() {
        // Parser should recover from member access errors
        let input = "fn test() { let x = obj. ; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_panic_mode_clears_on_sync() {
        // Panic mode should clear when reaching sync point
        let tokens = vec![
            Token::Fn,
            Token::Ident("test".to_string()),
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::Let,
            Token::Ident("x".to_string()),
            Token::Equal,
            Token::Number(5.0),
            // Missing semicolon
            Token::RBrace,
            Token::Eof,
        ];
        let mut parser = Parser::new(to_positioned(tokens), "fn test() { let x = 5 }");
        parser.panic_mode = true;
        parser.synchronize();
        assert!(!parser.panic_mode); // Should be cleared after sync
    }

    #[test]
    fn test_parser_recovery_continue_after_error() {
        // Parser should continue parsing after error
        let input = "fn test() { let x = 5 let y = 10; }";
        let tokens = tokenize(input).unwrap();
        let mut parser = Parser::new(to_positioned(tokens.clone()), input);
        let result = parser.parse_program();
        assert!(result.is_err());
        // Parser collected at least one error
        assert!(!parser.errors.is_empty());
    }

    #[test]
    fn test_parser_recovery_multiple_sync_points() {
        // Parser should handle multiple sync points
        let input = "fn test() { let x = 5; let y = 10; let z = 15; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parser_recovery_empty_statement() {
        // Parser should handle empty statements gracefully
        let input = "fn test() { ; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_recovery_invalid_return() {
        // Parser should recover from invalid return statement
        let input = "fn test() { return }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_with_leading_blank_line() {
        let input = "\n\nfn test() {\n    print(\"hello\");\n}";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_ok(), "Should parse file with leading blank line");
    }

    #[test]
    fn test_parse_file_starting_with_comment() {
        let input = "// This is a comment\nfn test() {\n    print(\"hello\");\n}";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_ok(), "Should parse file starting with comment");
    }

    #[test]
    fn test_parse_file_starting_with_blank_and_comment() {
        let input = "\n// Comment after blank line\nfn test() {\n    print(\"hello\");\n}";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(
            result.is_ok(),
            "Should parse file with blank line and comment"
        );
    }

    #[test]
    fn test_parse_with_crlf_line_endings() {
        // Test with Windows-style CRLF line endings
        let input = "\r\n\r\n// TESTING THINGS\r\nfn assert(cond: bool, msg: str) {\r\n    print(\"hello\");\r\n}";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(result.is_ok(), "Should parse file with CRLF line endings");
    }

    #[test]
    fn test_parse_signal_first() {
        let input = "signal test_signal();\n\nfn test() {\n    print(\"hello\");\n}";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        if let Err(e) = &result {
            eprintln!("Parse error: {}", e);
        }
        assert!(
            result.is_ok(),
            "Should parse file with signal declaration first"
        );
    }

    #[test]
    fn test_parse_multiple_blank_lines() {
        let input = "\n\n\n\n\nfn test() {\n    print(\"hello\");\n}";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(
            result.is_ok(),
            "Should parse file with multiple blank lines"
        );
    }

    #[test]
    fn test_parse_comment_only_then_code() {
        let input = "// Header comment\n// Another comment\n// Third comment\nfn test() {\n    print(\"hello\");\n}";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);
        assert!(
            result.is_ok(),
            "Should parse file with multiple leading comments"
        );
    }

    // ========================================================================
    // PHASE 2: PARSER EDGE CASE TESTS
    // ========================================================================
    // These tests cover parser-specific edge cases including:
    // - Dangling-else ambiguity
    // - Deeply nested constructs
    // - Invalid nesting patterns
    // - Operator precedence edge cases
    // - Missing delimiters and recovery
    // - Expression parsing boundaries

    #[test]
    fn test_parser_dangling_else_ambiguity() {
        // Classic dangling-else: which if does the else belong to?
        // FerrisScript requires braces, which eliminates this ambiguity
        let input = "fn test() { if (true) { if (false) { let x = 1; } else { let y = 2; } } }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        // Should parse successfully - braces make nesting unambiguous
        assert!(result.is_ok(), "Parser should handle nested if-else");
    }

    #[test]
    fn test_parser_deeply_nested_if_statements() {
        // Test deeply nested if-else chains (10 levels deep)
        let input = "fn test() {
            if (a) { if (b) { if (c) { if (d) { if (e) {
                if (f) { if (g) { if (h) { if (i) { if (j) {
                    let x = 42;
                } } } } }
            } } } } }
        }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_ok(), "Should handle deeply nested if statements");
    }

    #[test]
    fn test_parser_deeply_nested_expressions() {
        // Test deeply nested arithmetic expressions
        let input = "fn test() { let x = 1 + (2 * (3 - (4 / (5 + (6 - (7 * (8 + 9))))))); }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_ok(), "Should handle deeply nested expressions");
    }

    #[test]
    fn test_parser_mixed_operators_precedence() {
        // Test complex operator precedence scenarios
        let input = "fn test() { let x = 1 + 2 * 3 - 4 / 2 + 5 * 6 - 7; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_ok(), "Should handle mixed operator precedence");
    }

    #[test]
    fn test_parser_comparison_and_logical_precedence() {
        // Test precedence between comparison and logical operators
        // a < b && c > d || e == f
        let input = "fn test() { if (a < b && c > d || e == f) { let x = 1; } }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(
            result.is_ok(),
            "Should handle comparison and logical precedence"
        );
    }

    #[test]
    fn test_parser_unary_operators_precedence() {
        // Test unary operators with various precedence scenarios
        let input = "fn test() { let x = -a + !b && -c * d; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_ok(), "Should handle unary operator precedence");
    }

    #[test]
    fn test_parser_missing_closing_brace_in_function() {
        // Test missing closing brace - should error but not panic
        let input = "fn test() { let x = 5;";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err(), "Should error on missing closing brace");
    }

    #[test]
    fn test_parser_missing_opening_brace_in_function() {
        // Test missing opening brace
        let input = "fn test() let x = 5; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err(), "Should error on missing opening brace");
    }

    #[test]
    fn test_parser_mismatched_braces() {
        // Test mismatched brace types (though lexer handles this)
        let input = "fn test() { if (true) { let x = 5; } ";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err(), "Should error on mismatched braces");
    }

    #[test]
    fn test_parser_missing_semicolon_after_statement() {
        // Test missing semicolon in statement sequence
        let input = "fn test() { let x = 5 let y = 10; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err(), "Should error on missing semicolon");
    }

    #[test]
    fn test_parser_missing_comma_in_function_params() {
        // Test missing comma between function parameters
        let input = "fn test(a: int b: float) { let x = a; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err(), "Should error on missing comma in params");
    }

    #[test]
    fn test_parser_trailing_comma_in_function_params() {
        // Test trailing comma in function parameters (may or may not be allowed)
        let input = "fn test(a: int, b: float,) { let x = a; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        // Document current behavior - likely errors
        // Future: Could allow trailing commas
        match result {
            Err(err) => {
                assert!(err.contains("Expected") || err.contains("Unexpected"));
            }
            Ok(_) => {
                // If trailing commas are supported, this is fine
            }
        }
    }

    #[test]
    fn test_parser_empty_function_body() {
        // Test function with empty body (just braces)
        let input = "fn test() { }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_ok(), "Should allow empty function body");
    }

    #[test]
    fn test_parser_empty_if_body() {
        // Test if statement with empty body
        let input = "fn test() { if (true) { } }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_ok(), "Should allow empty if body");
    }

    #[test]
    fn test_parser_empty_while_body() {
        // Test while loop with empty body
        let input = "fn test() { while (true) { } }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_ok(), "Should allow empty while body");
    }

    #[test]
    fn test_parser_if_without_braces_error() {
        // Test if statement without braces (should error - braces required)
        let input = "fn test() { if (true) let x = 1; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        // FerrisScript requires braces for if bodies
        assert!(result.is_err(), "Should error on if without braces");
    }

    #[test]
    fn test_parser_nested_while_loops() {
        // Test nested while loops
        let input = "fn test() { while (a) { while (b) { while (c) { let x = 1; } } } }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_ok(), "Should handle nested while loops");
    }

    #[test]
    fn test_parser_if_else_if_else_chain() {
        // Test long if-else-if-else chain (nested else { if pattern)
        // FerrisScript requires braces after else, so else-if is nested
        let input = "fn test() {
            if (a) { let x = 1; }
            else { if (b) { let x = 2; }
            else { if (c) { let x = 3; }
            else { if (d) { let x = 4; }
            else { let x = 5; } } } }
        }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_ok(), "Should handle nested if-else chains");
    }

    #[test]
    fn test_parser_expression_as_statement() {
        // Test expressions used as statements (function calls, field access)
        let input = "fn test() { foo(); bar.baz; x + y; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_ok(), "Should allow expressions as statements");
    }

    #[test]
    fn test_parser_chained_comparisons() {
        // Test chained comparison expressions (not all languages support this)
        // In most languages: a < b < c parses as (a < b) < c
        let input = "fn test() { if (a < b < c) { let x = 1; } }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        // This should parse as (a < b) < c, which may be semantically invalid
        // but syntactically valid
        assert!(
            result.is_ok(),
            "Should parse chained comparisons syntactically"
        );
    }

    #[test]
    fn test_parser_invalid_assignment_target() {
        // Test assignment to invalid lvalue (literal)
        let input = "fn test() { 5 = x; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        // Parser may or may not catch this (might be type checker's job)
        // Document behavior
        match result {
            Err(err) => {
                assert!(err.contains("Expected") || err.contains("assignment"));
            }
            Ok(_) => {
                // If parser allows it, type checker should catch it
            }
        }
    }

    #[test]
    fn test_parser_missing_condition_in_if() {
        // Test if statement with missing condition
        let input = "fn test() { if { let x = 1; } }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err(), "Should error on missing if condition");
    }

    #[test]
    fn test_parser_missing_condition_in_while() {
        // Test while loop with missing condition
        let input = "fn test() { while { let x = 1; } }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err(), "Should error on missing while condition");
    }

    #[test]
    fn test_parser_return_in_nested_blocks() {
        // Test return statements in various nested contexts
        let input = "fn test() -> int {
            if (true) {
                while (false) {
                    if (x) {
                        return 42;
                    }
                }
            }
            return 0;
        }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_ok(), "Should handle return in nested blocks");
    }

    #[test]
    fn test_parser_multiple_consecutive_operators() {
        // Test multiple operators in sequence (error case)
        let input = "fn test() { let x = 5 + + 3; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        // Should parse as 5 + (+3) with unary plus
        // Or error depending on implementation
        match result {
            Err(err) => {
                assert!(err.contains("Expected") || err.contains("Unexpected"));
            }
            Ok(_) => {
                // May parse as unary operator - that's fine
            }
        }
    }

    #[test]
    fn test_parser_operator_at_end_of_expression() {
        // Test operator with missing right operand
        let input = "fn test() { let x = 5 +; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(
            result.is_err(),
            "Should error on operator with missing operand"
        );
    }

    #[test]
    fn test_parser_unclosed_parentheses() {
        // Test unclosed parentheses in expression
        let input = "fn test() { let x = (5 + 3; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err(), "Should error on unclosed parentheses");
    }

    #[test]
    fn test_parser_extra_closing_parenthesis() {
        // Test extra closing parenthesis
        let input = "fn test() { let x = (5 + 3)); }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err(), "Should error on extra closing parenthesis");
    }

    #[test]
    fn test_parser_nested_function_definitions() {
        // Test nested function definitions (not typically allowed)
        let input = "fn outer() { fn inner() { let x = 5; } }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        // ⚠️ CURRENT LIMITATION: Nested functions not supported
        // Future enhancement: Could support closures/nested functions
        assert!(result.is_err(), "Nested functions not currently supported");
    }

    #[test]
    fn test_parser_function_with_no_params_no_parens() {
        // Test function definition without parentheses
        let input = "fn test { let x = 5; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(
            result.is_err(),
            "Should error on missing parameter parentheses"
        );
    }

    #[test]
    fn test_parser_very_long_function_body() {
        // Test function with many statements (stress test)
        let mut statements = Vec::new();
        for i in 0..100 {
            statements.push(format!("let x{} = {};", i, i));
        }
        let input = format!("fn test() {{ {} }}", statements.join(" "));
        let tokens = tokenize(&input).unwrap();
        let result = parse(&tokens, &input);

        assert!(
            result.is_ok(),
            "Should handle functions with many statements"
        );
    }

    #[test]
    fn test_parser_global_statement_invalid() {
        // Test invalid statement at global scope (only fns and globals allowed)
        let input = "if (true) { let x = 5; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(
            result.is_err(),
            "Should error on if statement at global scope"
        );
    }

    #[test]
    fn test_parser_while_at_global_scope() {
        // Test while loop at global scope (should error)
        let input = "while (true) { let x = 5; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(
            result.is_err(),
            "Should error on while loop at global scope"
        );
    }

    #[test]
    fn test_parser_return_at_global_scope() {
        // Test return statement at global scope (should error)
        let input = "return 42;";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err(), "Should error on return at global scope");
    }

    #[test]
    fn test_parser_mixed_valid_and_invalid_top_level() {
        // Test mix of valid and invalid top-level declarations
        let input = "fn valid() { } if (true) { } fn another() { }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        // Should error on the if statement, but may continue parsing
        assert!(
            result.is_err(),
            "Should error on invalid top-level statement"
        );
    }

    #[test]
    fn test_parser_field_access_on_call_result() {
        // Test field access on function call result
        let input = "fn test() { let x = get_object().field; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_ok(), "Should parse field access on call result");
    }

    #[test]
    fn test_parser_chained_method_calls() {
        // Test chained method/function calls
        // ⚠️ CURRENT LIMITATION: Method chaining on call results not supported
        // obj.method1().method2() would require field access on call expressions
        let input = "fn test() { obj.method1().method2(); }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        // Future enhancement: Support chaining method calls
        assert!(
            result.is_err(),
            "Method chaining on call results not currently supported"
        );
    }

    #[test]
    fn test_parser_assignment_to_field_access() {
        // Test assignment to field access (lvalue)
        let input = "fn test() { obj.field = 42; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_ok(), "Should parse assignment to field");
    }

    #[test]
    fn test_parser_compound_assignment_to_field() {
        // Test compound assignment to field access
        let input = "fn test() { obj.field += 10; }";
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_ok(), "Should parse compound assignment to field");
    }

    // Checkpoint 1.2: Basic @export annotation tests
    #[test]
    fn test_parse_export_annotation_global_var() {
        // Test @export on global variable
        let input = "@export let speed: f32 = 10.0;";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.global_vars.len(), 1);
        let global_var = &program.global_vars[0];
        assert_eq!(global_var.name, "speed");
        assert!(
            global_var.export.is_some(),
            "Global variable should have export annotation"
        );

        // Export annotation should have PropertyHint::None (hints deferred to checkpoints 1.4-1.6)
        if let Some(export) = &global_var.export {
            assert!(
                matches!(export.hint, crate::ast::PropertyHint::None),
                "Export should have no hint in checkpoint 1.2"
            );
        }
    }

    #[test]
    fn test_parse_export_annotation_local_let() {
        // Test @export on local let statement inside function
        let input = r#"
fn _ready() {
    @export let health: i32 = 100;
}
"#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.functions.len(), 1);
        let func = &program.functions[0];
        assert_eq!(func.body.len(), 1);

        if let crate::ast::Stmt::Let {
            name, export, ty, ..
        } = &func.body[0]
        {
            assert_eq!(name, "health");
            assert_eq!(ty, &Some("i32".to_string()));
            assert!(
                export.is_some(),
                "Local let statement should have export annotation"
            );

            if let Some(exp) = export {
                assert!(
                    matches!(exp.hint, crate::ast::PropertyHint::None),
                    "Export should have no hint in checkpoint 1.2"
                );
            }
        } else {
            panic!("Expected Stmt::Let");
        }
    }

    #[test]
    fn test_parse_no_export_annotation() {
        // Test that variables without @export work normally
        let input = "let normal_var: i32 = 42;";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.global_vars.len(), 1);
        let global_var = &program.global_vars[0];
        assert_eq!(global_var.name, "normal_var");
        assert!(
            global_var.export.is_none(),
            "Normal variable should not have export annotation"
        );
    }

    // Checkpoint 1.4: Range property hint tests
    #[test]
    fn test_parse_export_range_hint() {
        // Test @export with range hint
        let input = "@export(range(0.0, 100.0, 0.1)) let speed: f32 = 10.0;";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.global_vars.len(), 1);
        let global_var = &program.global_vars[0];
        assert_eq!(global_var.name, "speed");
        assert!(global_var.export.is_some());

        if let Some(export) = &global_var.export {
            match &export.hint {
                crate::ast::PropertyHint::Range { min, max, step } => {
                    assert_eq!(*min, 0.0);
                    assert_eq!(*max, 100.0);
                    assert_eq!(*step, 0.1);
                }
                _ => panic!("Expected PropertyHint::Range"),
            }
        }
    }

    #[test]
    fn test_parse_export_range_hint_negative_values() {
        // Test @export with range hint using negative values
        let input = "@export(range(-10.0, 10.0, 1.0)) let offset: f32 = 0.0;";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.global_vars.len(), 1);
        let global_var = &program.global_vars[0];

        if let Some(export) = &global_var.export {
            match &export.hint {
                crate::ast::PropertyHint::Range { min, max, step } => {
                    assert_eq!(*min, -10.0);
                    assert_eq!(*max, 10.0);
                    assert_eq!(*step, 1.0);
                }
                _ => panic!("Expected PropertyHint::Range"),
            }
        }
    }

    #[test]
    fn test_parse_export_range_hint_integer_values() {
        // Test @export with range hint using integer values (should convert to f32)
        let input = "@export(range(0, 100, 5)) let count: i32 = 50;";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.global_vars.len(), 1);
        let global_var = &program.global_vars[0];

        if let Some(export) = &global_var.export {
            match &export.hint {
                crate::ast::PropertyHint::Range { min, max, step } => {
                    assert_eq!(*min, 0.0);
                    assert_eq!(*max, 100.0);
                    assert_eq!(*step, 5.0);
                }
                _ => panic!("Expected PropertyHint::Range"),
            }
        }
    }

    // Checkpoint 1.5: File property hint tests
    #[test]
    fn test_parse_export_file_hint_single_extension() {
        // Test @export with file hint - single extension
        let input = r#"@export(file("*.png")) let texture_path: String = "";"#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.global_vars.len(), 1);
        let global_var = &program.global_vars[0];
        assert_eq!(global_var.name, "texture_path");
        assert!(global_var.export.is_some());

        if let Some(export) = &global_var.export {
            match &export.hint {
                crate::ast::PropertyHint::File { extensions } => {
                    assert_eq!(extensions.len(), 1);
                    assert_eq!(extensions[0], "*.png");
                }
                _ => panic!("Expected PropertyHint::File"),
            }
        }
    }

    #[test]
    fn test_parse_export_file_hint_multiple_extensions() {
        // Test @export with file hint - multiple extensions
        let input = r#"@export(file("*.png", "*.jpg", "*.jpeg")) let image_path: String = "";"#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.global_vars.len(), 1);
        let global_var = &program.global_vars[0];

        if let Some(export) = &global_var.export {
            match &export.hint {
                crate::ast::PropertyHint::File { extensions } => {
                    assert_eq!(extensions.len(), 3);
                    assert_eq!(extensions[0], "*.png");
                    assert_eq!(extensions[1], "*.jpg");
                    assert_eq!(extensions[2], "*.jpeg");
                }
                _ => panic!("Expected PropertyHint::File"),
            }
        }
    }

    #[test]
    fn test_parse_export_file_hint_specific_extensions() {
        // Test @export with file hint - specific file extensions without wildcards
        let input = r#"@export(file(".tscn", ".scn")) let scene_path: String = "";"#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.global_vars.len(), 1);
        let global_var = &program.global_vars[0];

        if let Some(export) = &global_var.export {
            match &export.hint {
                crate::ast::PropertyHint::File { extensions } => {
                    assert_eq!(extensions.len(), 2);
                    assert_eq!(extensions[0], ".tscn");
                    assert_eq!(extensions[1], ".scn");
                }
                _ => panic!("Expected PropertyHint::File"),
            }
        }
    }

    // Checkpoint 1.6: Enum property hint tests
    #[test]
    fn test_parse_export_enum_hint_two_values() {
        // Test @export with enum hint - two values
        let input = r#"@export(enum("Easy", "Hard")) let difficulty: String = "Easy";"#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.global_vars.len(), 1);
        let global_var = &program.global_vars[0];
        assert_eq!(global_var.name, "difficulty");
        assert!(global_var.export.is_some());

        if let Some(export) = &global_var.export {
            match &export.hint {
                crate::ast::PropertyHint::Enum { values } => {
                    assert_eq!(values.len(), 2);
                    assert_eq!(values[0], "Easy");
                    assert_eq!(values[1], "Hard");
                }
                _ => panic!("Expected PropertyHint::Enum"),
            }
        }
    }

    #[test]
    fn test_parse_export_enum_hint_multiple_values() {
        // Test @export with enum hint - multiple values
        let input =
            r#"@export(enum("North", "South", "East", "West")) let direction: String = "North";"#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.global_vars.len(), 1);
        let global_var = &program.global_vars[0];

        if let Some(export) = &global_var.export {
            match &export.hint {
                crate::ast::PropertyHint::Enum { values } => {
                    assert_eq!(values.len(), 4);
                    assert_eq!(values[0], "North");
                    assert_eq!(values[1], "South");
                    assert_eq!(values[2], "East");
                    assert_eq!(values[3], "West");
                }
                _ => panic!("Expected PropertyHint::Enum"),
            }
        }
    }

    #[test]
    fn test_parse_export_enum_hint_numeric_strings() {
        // Test @export with enum hint - numeric string values
        let input = r#"@export(enum("1", "2", "5", "10")) let multiplier: String = "1";"#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.global_vars.len(), 1);
        let global_var = &program.global_vars[0];

        if let Some(export) = &global_var.export {
            match &export.hint {
                crate::ast::PropertyHint::Enum { values } => {
                    assert_eq!(values.len(), 4);
                    assert_eq!(values[0], "1");
                    assert_eq!(values[1], "2");
                    assert_eq!(values[2], "5");
                    assert_eq!(values[3], "10");
                }
                _ => panic!("Expected PropertyHint::Enum"),
            }
        }
    }

    // ========== Checkpoint 1.7: Error Recovery Tests ==========

    #[test]
    fn test_parse_export_error_unknown_hint_type() {
        // Test @export with unknown hint type
        let input = r#"@export(color(255, 0, 0)) let tint: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };"#;
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Unknown property hint 'color'"));
        assert!(error.contains("Expected 'range', 'file', or 'enum'"));
    }

    #[test]
    fn test_parse_export_error_missing_hint_name() {
        // Test @export with opening paren but no hint name
        let input = r#"@export(123) let value: i32 = 0;"#;
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected property hint name after @export("));
    }

    #[test]
    fn test_parse_export_error_range_missing_comma_after_min() {
        // Test @export with range hint missing comma after min
        let input = r#"@export(range(0.0 100.0, 1.0)) let value: f32 = 0.0;"#;
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected ,"));
    }

    #[test]
    fn test_parse_export_error_range_missing_comma_after_max() {
        // Test @export with range hint missing comma after max
        let input = r#"@export(range(0.0, 100.0 1.0)) let value: f32 = 0.0;"#;
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected ,"));
    }

    #[test]
    fn test_parse_export_error_range_missing_closing_paren() {
        // Test @export with range hint missing closing parenthesis
        let input = r#"@export(range(0.0, 100.0, 1.0) let value: f32 = 0.0;"#;
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected )"));
    }

    #[test]
    fn test_parse_export_error_range_wrong_type_string_for_number() {
        // Test @export with range hint using string instead of number
        let input = r#"@export(range("zero", 100.0, 1.0)) let value: f32 = 0.0;"#;
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected number"));
        assert!(error.contains("range hint min value"));
    }

    #[test]
    fn test_parse_export_error_file_missing_string_literal() {
        // Test @export with file hint missing string literal
        let input = r#"@export(file(png, jpg)) let texture: String = "";"#;
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected string literal for file extension"));
    }

    #[test]
    fn test_parse_export_error_file_number_instead_of_string() {
        // Test @export with file hint using number instead of string
        let input = r#"@export(file(123)) let texture: String = "";"#;
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected string literal for file extension"));
    }

    #[test]
    fn test_parse_export_error_file_wrong_type_after_comma() {
        // Test @export with file hint using wrong type after comma
        let input = r#"@export(file("*.png", 456)) let texture: String = "";"#;
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected string literal for file extension after comma"));
    }

    #[test]
    fn test_parse_export_error_enum_missing_string_literal() {
        // Test @export with enum hint missing string literal
        let input = r#"@export(enum(Easy, Hard)) let difficulty: String = "Easy";"#;
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected string literal for enum value"));
    }

    #[test]
    fn test_parse_export_error_enum_number_instead_of_string() {
        // Test @export with enum hint using number instead of string
        let input = r#"@export(enum(1, 2, 3)) let level: String = "1";"#;
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected string literal for enum value"));
    }

    #[test]
    fn test_parse_export_error_enum_wrong_type_after_comma() {
        // Test @export with enum hint using wrong type after comma
        let input = r#"@export(enum("Easy", true)) let difficulty: String = "Easy";"#;
        let tokens = tokenize(input).unwrap();
        let result = parse(&tokens, input);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected string literal for enum value after comma"));
    }

    // ========== Checkpoint 1.8: Integration Tests ==========

    #[test]
    fn test_parse_export_multiple_annotations_same_file() {
        // Test multiple @export annotations with different hint types in same file
        let input = r#"
            @export let simple: i32 = 0;
            @export(range(0.0, 100.0, 1.0)) let speed: f32 = 10.0;
            @export(file("*.png", "*.jpg")) let texture: String = "";
            @export(enum("Easy", "Normal", "Hard")) let difficulty: String = "Normal";
        "#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.global_vars.len(), 4);

        // Check first export (no hint)
        assert!(program.global_vars[0].export.is_some());
        if let Some(export) = &program.global_vars[0].export {
            assert!(matches!(export.hint, crate::ast::PropertyHint::None));
        }

        // Check second export (range hint)
        assert!(program.global_vars[1].export.is_some());
        if let Some(export) = &program.global_vars[1].export {
            match &export.hint {
                crate::ast::PropertyHint::Range { min, max, step } => {
                    assert_eq!(*min, 0.0);
                    assert_eq!(*max, 100.0);
                    assert_eq!(*step, 1.0);
                }
                _ => panic!("Expected PropertyHint::Range"),
            }
        }

        // Check third export (file hint)
        assert!(program.global_vars[2].export.is_some());
        if let Some(export) = &program.global_vars[2].export {
            match &export.hint {
                crate::ast::PropertyHint::File { extensions } => {
                    assert_eq!(extensions.len(), 2);
                    assert_eq!(extensions[0], "*.png");
                    assert_eq!(extensions[1], "*.jpg");
                }
                _ => panic!("Expected PropertyHint::File"),
            }
        }

        // Check fourth export (enum hint)
        assert!(program.global_vars[3].export.is_some());
        if let Some(export) = &program.global_vars[3].export {
            match &export.hint {
                crate::ast::PropertyHint::Enum { values } => {
                    assert_eq!(values.len(), 3);
                    assert_eq!(values[0], "Easy");
                    assert_eq!(values[1], "Normal");
                    assert_eq!(values[2], "Hard");
                }
                _ => panic!("Expected PropertyHint::Enum"),
            }
        }
    }

    #[test]
    fn test_parse_export_with_signals_and_functions() {
        // Test @export annotations alongside signals and functions
        let input = r#"
            signal player_died();
            
            @export let health: i32 = 100;
            @export(range(0.0, 10.0, 0.1)) let speed: f32 = 5.0;
            
            fn ready() {
                let x: i32 = 0;
            }
            
            @export(enum("Red", "Blue", "Green")) let team: String = "Red";
            
            fn process(delta: f32) {
                @export let local_export: i32 = 0;
            }
        "#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        // Check signals
        assert_eq!(program.signals.len(), 1);
        assert_eq!(program.signals[0].name, "player_died");

        // Check global exports
        assert_eq!(program.global_vars.len(), 3);
        assert!(program.global_vars[0].export.is_some());
        assert!(program.global_vars[1].export.is_some());
        assert!(program.global_vars[2].export.is_some());

        // Check functions
        assert_eq!(program.functions.len(), 2);
        assert_eq!(program.functions[0].name, "ready");
        assert_eq!(program.functions[1].name, "process");

        // Check local export in function
        let process_fn = &program.functions[1];
        if let crate::ast::Stmt::Let { export, .. } = &process_fn.body[0] {
            assert!(export.is_some());
        } else {
            panic!("Expected let statement with export");
        }
    }

    #[test]
    fn test_parse_export_mixed_with_non_exported_vars() {
        // Test mix of exported and non-exported variables
        let input = r#"
            let normal_var: i32 = 0;
            @export let exported_var: i32 = 1;
            let another_normal: f32 = 2.0;
            @export(range(0.0, 1.0, 0.1)) let exported_range: f32 = 0.5;
            let final_normal: bool = true;
        "#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.global_vars.len(), 5);

        // Check which are exported
        assert!(program.global_vars[0].export.is_none());
        assert!(program.global_vars[1].export.is_some());
        assert!(program.global_vars[2].export.is_none());
        assert!(program.global_vars[3].export.is_some());
        assert!(program.global_vars[4].export.is_none());

        // Verify the exported ones have correct hints
        if let Some(export) = &program.global_vars[1].export {
            assert!(matches!(export.hint, crate::ast::PropertyHint::None));
        }

        if let Some(export) = &program.global_vars[3].export {
            match &export.hint {
                crate::ast::PropertyHint::Range { min, max, step } => {
                    assert_eq!(*min, 0.0);
                    assert_eq!(*max, 1.0);
                    assert_eq!(*step, 0.1);
                }
                _ => panic!("Expected PropertyHint::Range"),
            }
        }
    }

    #[test]
    fn test_parse_export_all_hint_types_comprehensive() {
        // Comprehensive test of all hint types with various edge cases
        let input = r#"
            @export let no_hint: i32 = 0;
            @export(range(-100.0, 100.0, 0.5)) let negative_range: f32 = 0.0;
            @export(range(0, 10, 1)) let integer_range: f32 = 5.0;
            @export(file("*.png")) let single_ext: String = "";
            @export(file("*.png", "*.jpg", "*.jpeg", "*.bmp")) let many_exts: String = "";
            @export(file(".tscn", ".scn")) let godot_scenes: String = "";
            @export(enum("A", "B")) let two_values: String = "A";
            @export(enum("North", "South", "East", "West")) let directions: String = "North";
            @export(enum("1", "10", "100", "1000")) let numeric_enum: String = "1";
        "#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.global_vars.len(), 9);

        // Verify all are exported
        for global_var in &program.global_vars {
            assert!(
                global_var.export.is_some(),
                "Variable {} should be exported",
                global_var.name
            );
        }

        // Spot check a few specific ones
        assert!(matches!(
            program.global_vars[0].export.as_ref().unwrap().hint,
            crate::ast::PropertyHint::None
        ));

        if let Some(export) = &program.global_vars[1].export {
            match &export.hint {
                crate::ast::PropertyHint::Range { min, max, step } => {
                    assert_eq!(*min, -100.0);
                    assert_eq!(*max, 100.0);
                    assert_eq!(*step, 0.5);
                }
                _ => panic!("Expected PropertyHint::Range"),
            }
        }

        if let Some(export) = &program.global_vars[4].export {
            match &export.hint {
                crate::ast::PropertyHint::File { extensions } => {
                    assert_eq!(extensions.len(), 4);
                }
                _ => panic!("Expected PropertyHint::File"),
            }
        }
    }

    #[test]
    fn test_parse_export_in_complex_program() {
        // Test @export in a realistic complex program structure
        let input = r#"signal health_changed(new_health: i32);
signal died();
@export let max_health: i32 = 100;
@export(range(0.0, 20.0, 0.5)) let move_speed: f32 = 5.0;
@export(file("*.png", "*.jpg")) let sprite_texture: String = "";
@export(enum("Player", "Enemy", "NPC")) let entity_type: String = "Player";
let current_health: i32 = 100;
fn ready() {
    current_health = max_health;
}
fn take_damage(amount: i32) {
    current_health = current_health - amount;
}
fn process(delta: f32) {
    let movement: Vector2 = Vector2 { x: 0.0, y: 0.0 };
}"#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        // Verify structure
        assert_eq!(program.signals.len(), 2);
        assert_eq!(program.global_vars.len(), 5);
        assert_eq!(program.functions.len(), 3);

        // Verify exports
        assert!(program.global_vars[0].export.is_some());
        assert!(program.global_vars[1].export.is_some());
        assert!(program.global_vars[2].export.is_some());
        assert!(program.global_vars[3].export.is_some());
        assert!(program.global_vars[4].export.is_none()); // current_health is not exported

        // Verify hint types
        assert!(matches!(
            program.global_vars[0].export.as_ref().unwrap().hint,
            crate::ast::PropertyHint::None
        ));
        assert!(matches!(
            program.global_vars[1].export.as_ref().unwrap().hint,
            crate::ast::PropertyHint::Range { .. }
        ));
        assert!(matches!(
            program.global_vars[2].export.as_ref().unwrap().hint,
            crate::ast::PropertyHint::File { .. }
        ));
        assert!(matches!(
            program.global_vars[3].export.as_ref().unwrap().hint,
            crate::ast::PropertyHint::Enum { .. }
        ));
    }

    #[test]
    fn test_parse_export_edge_case_empty_file_after_export() {
        // Test that parser handles export followed by EOF gracefully
        let input = r#"@export let value: i32 = 0;"#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.global_vars.len(), 1);
        assert!(program.global_vars[0].export.is_some());
    }

    #[test]
    fn test_parse_export_edge_case_only_exports() {
        // Test file containing only exported variables
        let input = r#"
            @export let a: i32 = 1;
            @export let b: i32 = 2;
            @export let c: i32 = 3;
        "#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.global_vars.len(), 3);
        for global_var in &program.global_vars {
            assert!(global_var.export.is_some());
        }
    }

    #[test]
    fn test_parse_export_with_comments() {
        // Test @export with comments nearby
        let input = r#"// Player configuration
@export let health: i32 = 100; // Maximum health

@export(range(0.0, 10.0, 0.1)) let speed: f32 = 5.0; // Movement speed
"#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();

        assert_eq!(program.global_vars.len(), 2);
        assert!(program.global_vars[0].export.is_some());
        assert!(program.global_vars[1].export.is_some());
    }
}
