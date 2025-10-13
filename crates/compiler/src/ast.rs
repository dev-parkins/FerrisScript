//! Abstract Syntax Tree (AST) definitions for FerrisScript.
//!
//! This module defines all the node types that make up a FerrisScript program.
//! The AST is built by the parser and consumed by the type checker and runtime.
//!
//! # Structure
//!
//! A FerrisScript program consists of:
//! - Global variable declarations ([`GlobalVar`])
//! - Function definitions ([`Function`])
//!
//! Functions contain:
//! - Parameters ([`Param`])
//! - Statements ([`Stmt`])
//! - Expressions ([`Expr`])
//!
//! # Example
//!
//! ```no_run
//! use ferrisscript_compiler::ast::{Program, Function, Param, Stmt, Expr};
//!
//! let program = Program::new();
//! // program.functions.push(...);
//! ```

use std::fmt;

/// Source code location (line and column numbers).
///
/// Used throughout the AST to track where each construct appears in the
/// source code. This enables precise error messages with line/column info.
///
/// # Examples
///
/// ```
/// use ferrisscript_compiler::ast::Span;
///
/// let span = Span::new(10, 15); // line 10, column 15
/// assert_eq!(span.line, 10);
/// assert_eq!(span.column, 15);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(line: usize, column: usize) -> Self {
        Span { line, column }
    }

    pub fn unknown() -> Self {
        Span { line: 0, column: 0 }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

/// Root node of a FerrisScript program.
///
/// Represents the complete program structure with global variables and functions.
/// This is the output of the parser and input to the type checker.
///
/// # Examples
///
/// ```
/// use ferrisscript_compiler::ast::Program;
///
/// let mut program = Program::new();
/// // Add functions and global variables
/// // program.functions.push(...);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    /// Global variable declarations (let and let mut)
    pub global_vars: Vec<GlobalVar>,
    /// Signal declarations
    pub signals: Vec<Signal>,
    /// Function definitions
    pub functions: Vec<Function>,
    /// Property metadata for exported variables (generated during type checking)
    pub property_metadata: Vec<PropertyMetadata>,
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl Program {
    pub fn new() -> Self {
        Program {
            global_vars: Vec::new(),
            signals: Vec::new(),
            functions: Vec::new(),
            property_metadata: Vec::new(),
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for var in &self.global_vars {
            writeln!(f, "{}", var)?;
        }
        for signal in &self.signals {
            writeln!(f, "{}", signal)?;
        }
        for func in &self.functions {
            writeln!(f, "{}", func)?;
        }
        Ok(())
    }
}

/// Property hint for exported variables.
///
/// Hints provide additional metadata for how properties should be displayed
/// and edited in the Godot Inspector.
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyHint {
    /// No hint (default Inspector widget)
    None,
    /// Range hint with min, max, step
    /// Example: `@export(range(0, 100, 1))`
    Range { min: f32, max: f32, step: f32 },
    /// File hint with allowed extensions
    /// Example: `@export(file("*.png", "*.jpg"))`
    File { extensions: Vec<String> },
    /// Enum hint with allowed values
    /// Example: `@export(enum("Easy", "Medium", "Hard"))`
    Enum { values: Vec<String> },
}

/// Export annotation for Inspector-editable properties.
///
/// The `@export` annotation exposes a variable to the Godot Inspector,
/// allowing it to be edited in the editor.
///
/// # Examples
///
/// ```text
/// @export let speed: f32 = 10.0;
/// @export(range(0, 100)) let health: i32 = 100;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ExportAnnotation {
    /// Optional property hint
    pub hint: PropertyHint,
    /// Source location
    pub span: Span,
}

/// Property metadata for exported variables.
///
/// Generated during type checking and stored in the Program for runtime access.
/// Contains all information needed to expose properties to Godot Inspector.
///
/// # Examples
///
/// ```text
/// // For: @export(range(0, 100, 1)) let mut health: i32 = 100;
/// PropertyMetadata {
///     name: "health".to_string(),
///     type_name: "i32".to_string(),
///     hint: PropertyHint::Range { min: 0.0, max: 100.0, step: 1.0 },
///     hint_string: "0,100,1".to_string(),
///     default_value: Some("100".to_string()),
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyMetadata {
    /// Property name (variable name)
    pub name: String,
    /// Type name (i32, f32, bool, String, Vector2, Color, Rect2, Transform2D)
    pub type_name: String,
    /// Property hint (range, file, enum, or none)
    pub hint: PropertyHint,
    /// Godot-compatible hint string ("0,100,1" or "Easy,Normal,Hard" or "*.png,*.jpg")
    pub hint_string: String,
    /// Default value as string representation (for Inspector reset)
    pub default_value: Option<String>,
}

impl PropertyMetadata {
    /// Generate Godot-compatible hint_string from PropertyHint
    pub fn generate_hint_string(hint: &PropertyHint) -> String {
        match hint {
            PropertyHint::None => String::new(),
            PropertyHint::Range { min, max, step } => {
                format!("{},{},{}", min, max, step)
            }
            PropertyHint::File { extensions } => extensions.join(","),
            PropertyHint::Enum { values } => values.join(","),
        }
    }
}

/// Global variable declaration.
///
/// Represents a variable declared at the program level (outside functions).
/// Global variables can be mutable or immutable and must have an initializer.
///
/// # Examples
///
/// ```text
/// let score: i32 = 0;           // immutable global
/// let mut player_health: f32 = 100.0;  // mutable global
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalVar {
    /// Variable name
    pub name: String,
    /// Whether the variable can be reassigned
    pub mutable: bool,
    /// Optional type annotation
    pub ty: Option<String>,
    /// Initializer expression
    pub value: Expr,
    /// Optional export annotation
    pub export: Option<ExportAnnotation>,
    /// Source location
    pub span: Span,
}

impl fmt::Display for GlobalVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "let ")?;
        if self.mutable {
            write!(f, "mut ")?;
        }
        write!(f, "{}", self.name)?;
        if let Some(ty) = &self.ty {
            write!(f, ": {}", ty)?;
        }
        write!(f, " = {};", self.value)
    }
}

/// Function definition.
///
/// Represents a named function with parameters, optional return type, and body.
/// Functions are the primary unit of code organization in FerrisScript.
///
/// # Examples
///
/// ```text
/// fn greet() { }                    // No parameters, no return
/// fn add(a: i32, b: i32) -> i32 { } // With parameters and return type
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    /// Function name
    pub name: String,
    /// Function parameters
    pub params: Vec<Param>,
    /// Optional return type annotation
    pub return_type: Option<String>,
    /// Function body (list of statements)
    pub body: Vec<Stmt>,
    /// Source location
    pub span: Span,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn {}(", self.name)?;
        for (i, param) in self.params.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", param)?;
        }
        write!(f, ")")?;
        if let Some(ret) = &self.return_type {
            write!(f, " -> {}", ret)?;
        }
        writeln!(f, " {{")?;
        for stmt in &self.body {
            writeln!(f, "    {}", stmt)?;
        }
        writeln!(f, "}}")
    }
}

/// Function parameter.
///
/// Represents a single parameter in a function signature, including its name
/// and optional type annotation.
///
/// # Examples
///
/// ```text
/// fn process(x: i32, y: f32) { }  // Two parameters with type annotations
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    /// Parameter name
    pub name: String,
    pub ty: String,
    pub span: Span,
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.ty)
    }
}

/// Statement node.
///
/// Represents executable statements that make up function bodies and control flow.
/// Statements don't produce values (unlike expressions).
///
/// # Variants
///
/// - `Expr` - Expression statement (e.g., function call)
/// - `Let` - Local variable declaration
/// - `Assign` - Variable assignment
/// - `If` - Conditional branching
/// - `While` - Loop
/// - `Return` - Early function return
/// - `CompoundAssign` - Combined operation and assignment (+=, -=)
///
/// # Examples
///
/// ```text
/// let x: i32 = 5;              // Let statement
/// x = x + 1;                   // Assign statement
/// if x > 10 { return x; }      // If + Return statements
/// while x < 100 { x = x * 2; } // While statement
/// x += 5;                      // CompoundAssign statement
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Let {
        name: String,
        mutable: bool,
        ty: Option<String>,
        value: Expr,
        export: Option<ExportAnnotation>,
        span: Span,
    },
    Assign {
        target: Expr,
        value: Expr,
        span: Span,
    },
    If {
        cond: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Vec<Stmt>,
        span: Span,
    },
    While {
        cond: Expr,
        body: Vec<Stmt>,
        span: Span,
    },
    Return {
        value: Option<Expr>,
        span: Span,
    },
}

/// Signal declaration (top-level only).
///
/// Signals are event declarations that can be emitted and connected to methods.
/// They must be declared at the module level (not inside functions).
///
/// # Examples
///
/// ```text
/// signal health_changed(old: i32, new: i32);
/// signal player_died;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Signal {
    /// Signal name
    pub name: String,
    /// Signal parameters (name, type)
    pub parameters: Vec<(String, String)>,
    /// Source location
    pub span: Span,
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "signal {}(", self.name)?;
        for (i, (param_name, param_type)) in self.parameters.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", param_name, param_type)?;
        }
        write!(f, ");")
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Expr(expr) => write!(f, "{};", expr),
            Stmt::Let {
                name,
                mutable,
                ty,
                value,
                ..
            } => {
                write!(f, "let ")?;
                if *mutable {
                    write!(f, "mut ")?;
                }
                write!(f, "{}", name)?;
                if let Some(t) = ty {
                    write!(f, ": {}", t)?;
                }
                write!(f, " = {};", value)
            }
            Stmt::Assign { target, value, .. } => {
                write!(f, "{} = {};", target, value)
            }
            Stmt::If {
                cond,
                then_branch,
                else_branch,
                ..
            } => {
                write!(f, "if {} {{ ", cond)?;
                for stmt in then_branch {
                    write!(f, "{} ", stmt)?;
                }
                write!(f, "}}")?;
                if !else_branch.is_empty() {
                    write!(f, " else {{ ")?;
                    for stmt in else_branch {
                        write!(f, "{} ", stmt)?;
                    }
                    write!(f, "}}")?;
                }
                Ok(())
            }
            Stmt::While { cond, body, .. } => {
                write!(f, "while {} {{ ", cond)?;
                for stmt in body {
                    write!(f, "{} ", stmt)?;
                }
                write!(f, "}}")
            }
            Stmt::Return { value, .. } => {
                write!(f, "return")?;
                if let Some(v) = value {
                    write!(f, " {}", v)?;
                }
                write!(f, ";")
            }
        }
    }
}

/// Expression node.
///
/// Represents expressions that produce values. Expressions can be used as
/// statement bodies, in assignments, as function arguments, etc.
///
/// # Variants
///
/// - `Literal` - Constant values (numbers, strings, booleans)
/// - `Variable` - Variable reference
/// - `Binary` - Binary operation (e.g., `a + b`, `x == y`)
/// - `Unary` - Unary operation (e.g., `-x`, `!flag`)
/// - `Call` - Function call
/// - `FieldAccess` - Member access (e.g., `position.x`)
/// - `Assign` - Assignment expression
/// - `CompoundAssign` - Combined operation (e.g., `x += 5`)
///
/// # Examples
///
/// ```text
/// 42                    // Literal
/// x                     // Variable
/// a + b * 2             // Binary (with precedence)
/// -velocity.y           // Unary + FieldAccess
/// sqrt(x * x + y * y)   // Call
/// position.x            // FieldAccess
/// Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 }  // StructLiteral
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal, Span),
    Variable(String, Span),
    Binary(Box<Expr>, BinaryOp, Box<Expr>, Span),
    Unary(UnaryOp, Box<Expr>, Span),
    Call(String, Vec<Expr>, Span),
    FieldAccess(Box<Expr>, String, Span),
    Assign(Box<Expr>, Box<Expr>, Span),
    CompoundAssign(Box<Expr>, CompoundOp, Box<Expr>, Span),
    /// Struct literal: `TypeName { field1: value1, field2: value2 }`
    /// Used for constructing Color, Rect2, Transform2D, etc.
    /// MVP: No nested literals (e.g., Rect2 with inline Vector2)
    StructLiteral {
        type_name: String,
        fields: Vec<(String, Expr)>,
        span: Span,
    },
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Literal(_, s) => *s,
            Expr::Variable(_, s) => *s,
            Expr::Binary(_, _, _, s) => *s,
            Expr::Unary(_, _, s) => *s,
            Expr::Call(_, _, s) => *s,
            Expr::FieldAccess(_, _, s) => *s,
            Expr::Assign(_, _, s) => *s,
            Expr::CompoundAssign(_, _, _, s) => *s,
            Expr::StructLiteral { span, .. } => *span,
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Literal(lit, _) => write!(f, "{}", lit),
            Expr::Variable(name, _) => write!(f, "{}", name),
            Expr::Binary(left, op, right, _) => write!(f, "({} {} {})", left, op, right),
            Expr::Unary(op, expr, _) => write!(f, "({}{})", op, expr),
            Expr::Call(name, args, _) => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Expr::FieldAccess(obj, field, _) => write!(f, "{}.{}", obj, field),
            Expr::Assign(target, value, _) => write!(f, "{} = {}", target, value),
            Expr::CompoundAssign(target, op, value, _) => write!(f, "{} {} {}", target, op, value),
            Expr::StructLiteral {
                type_name, fields, ..
            } => {
                write!(f, "{} {{ ", type_name)?;
                for (i, (field_name, field_expr)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", field_name, field_expr)?;
                }
                write!(f, " }}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f32),
    Bool(bool),
    Str(String),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Int(n) => write!(f, "{}", n),
            Literal::Float(n) => write!(f, "{}", n),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Str(s) => write!(f, "\"{}\"", s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Eq => "==",
            BinaryOp::Ne => "!=",
            BinaryOp::Lt => "<",
            BinaryOp::Le => "<=",
            BinaryOp::Gt => ">",
            BinaryOp::Ge => ">=",
            BinaryOp::And => "&&",
            BinaryOp::Or => "||",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            UnaryOp::Neg => "-",
            UnaryOp::Not => "!",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompoundOp {
    AddAssign,
    SubAssign,
}

impl fmt::Display for CompoundOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CompoundOp::AddAssign => "+=",
            CompoundOp::SubAssign => "-=",
        };
        write!(f, "{}", s)
    }
}

// Backwards compatibility alias
pub type Op = BinaryOp;
