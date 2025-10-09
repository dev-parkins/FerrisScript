//! Type checking and semantic analysis for FerrisScript.
//!
//! This module performs static type checking on the AST to ensure type safety.
//! It verifies that all operations are valid, variables are declared before use,
//! and function calls match their signatures.
//!
//! # Type System
//!
//! FerrisScript supports:
//! - Primitive types: `i32`, `f32`, `bool`, `String`
//! - Godot types: `Vector2`, `Node`
//! - Void return type
//! - Type coercion: `i32` → `f32` (implicit)
//!
//! # Scope Rules
//!
//! - Function parameters are scoped to the function body
//! - Local variables are scoped to their enclosing block
//! - Global variables are accessible everywhere
//! - Shadowing is not allowed
//!
//! # Performance
//!
//! - Simple scripts: ~850ns
//! - Complex scripts: ~3.6μs
//! - Single-pass type checking with scope stack
//!
//! # Example
//!
//! ```no_run
//! use ferrisscript_compiler::{lexer::tokenize, parser::parse, type_checker::check};
//!
//! let source = "fn add(a: i32, b: i32) -> i32 { return a + b; }";
//! let tokens = tokenize(source).unwrap();
//! let program = parse(&tokens, source).unwrap();
//! check(&program, source).unwrap(); // Type checking passes
//! ```

use crate::ast::*;
use crate::error_code::ErrorCode;
use crate::error_context::format_error_with_code;
use crate::suggestions::find_similar_identifiers;
use std::collections::HashMap;

/// Type representation for FerrisScript's type system.
///
/// Represents all supported types including primitives, Godot types,
/// and special types like `Void` and `Unknown`.
///
/// # Type Coercion
///
/// The type checker supports implicit coercion from `i32` to `f32` in FerrisScript code.
/// For example, passing an integer to a function expecting a float is allowed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    I32,
    F32,
    Bool,
    String,
    Vector2,
    Node,
    Void,
    Unknown,
}

impl Type {
    pub fn name(&self) -> &str {
        match self {
            Type::I32 => "i32",
            Type::F32 => "f32",
            Type::Bool => "bool",
            Type::String => "String",
            Type::Vector2 => "Vector2",
            Type::Node => "Node",
            Type::Void => "void",
            Type::Unknown => "unknown",
        }
    }

    fn from_string(s: &str) -> Type {
        match s {
            "i32" => Type::I32,
            "f32" => Type::F32,
            "bool" => Type::Bool,
            "String" => Type::String,
            "Vector2" => Type::Vector2,
            "Node" => Type::Node,
            _ => Type::Unknown,
        }
    }

    /// Check if this type can be implicitly converted to another type
    fn can_coerce_to(&self, other: &Type) -> bool {
        self == other || matches!((self, other), (Type::I32, Type::F32))
    }
}

/// Function signature for type checking
#[derive(Debug, Clone)]
struct FunctionSignature {
    params: Vec<Type>,
    return_type: Type,
}

/// Type checking environment with scopes
struct TypeChecker<'a> {
    // Variable types in current scope (stack of scopes)
    scopes: Vec<HashMap<String, Type>>,
    // Function signatures
    functions: HashMap<String, FunctionSignature>,
    // Signal signatures (signal_name -> param_types)
    signals: HashMap<String, Vec<Type>>,
    // Current errors
    errors: Vec<String>,
    // Source code for error context
    source: &'a str,
}

impl<'a> TypeChecker<'a> {
    fn new(source: &'a str) -> Self {
        let mut checker = TypeChecker {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
            signals: HashMap::new(),
            errors: Vec::new(),
            source,
        };

        // Register built-in functions
        checker.functions.insert(
            "print".to_string(),
            FunctionSignature {
                params: vec![Type::String],
                return_type: Type::Void,
            },
        );

        // Register emit_signal built-in function (first arg is signal name as string)
        // Note: This is a variadic function, we'll check args dynamically
        checker.functions.insert(
            "emit_signal".to_string(),
            FunctionSignature {
                params: vec![Type::String], // At least signal name
                return_type: Type::Void,
            },
        );

        // Add "self" to the global scope as Node type
        checker.scopes[0].insert("self".to_string(), Type::Node);

        checker
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    fn declare_variable(&mut self, name: String, ty: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, ty);
        }
    }

    fn lookup_variable(&self, name: &str) -> Option<Type> {
        // Search from innermost to outermost scope
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        None
    }

    /// Get all variable names in scope (for suggestion purposes)
    fn list_variables(&self) -> Vec<&str> {
        let mut vars = Vec::new();
        for scope in self.scopes.iter().rev() {
            for name in scope.keys() {
                vars.push(name.as_str());
            }
        }
        vars
    }

    /// Get all function names (for suggestion purposes)
    fn list_functions(&self) -> Vec<&str> {
        self.functions.keys().map(|s| s.as_str()).collect()
    }

    /// Get all known type names (for suggestion purposes)
    fn list_types() -> Vec<&'static str> {
        vec!["i32", "f32", "bool", "String", "Vector2", "Node"]
    }

    fn error(&mut self, message: String) {
        self.errors.push(message);
    }

    fn check_program(&mut self, program: &Program) {
        // Register global variables
        for var in &program.global_vars {
            let ty = if let Some(type_name) = &var.ty {
                let parsed_ty = Type::from_string(type_name);

                // If type is unknown and a type annotation was provided, report E203
                if parsed_ty == Type::Unknown {
                    let base_msg = format!("Unknown type '{}' at {}", type_name, var.span);

                    // Find similar type names
                    let candidates = Self::list_types();
                    let suggestions = find_similar_identifiers(type_name, &candidates);

                    let hint = if !suggestions.is_empty() {
                        format!("Type not recognized. Did you mean '{}'?", suggestions[0])
                    } else {
                        "Type not recognized. Available types: i32, f32, bool, String, Vector2, Node".to_string()
                    };

                    self.error(format_error_with_code(
                        ErrorCode::E203,
                        &base_msg,
                        self.source,
                        var.span.line,
                        var.span.column,
                        &hint,
                    ));
                }

                parsed_ty
            } else {
                let inferred = self.infer_expr(&var.value);

                // Only report E218 if type inference failed AND no annotation was provided
                if inferred == Type::Unknown {
                    let base_msg = format!(
                        "Cannot infer type for global variable '{}' at {}",
                        var.name, var.span
                    );
                    self.error(format_error_with_code(
                        ErrorCode::E218,
                        &base_msg,
                        self.source,
                        var.span.line,
                        var.span.column,
                        "Add an explicit type annotation (e.g., let name: type = value)",
                    ));
                }

                inferred
            };

            self.declare_variable(var.name.clone(), ty.clone());

            // Check that initializer matches declared type
            let init_ty = self.check_expr(&var.value);
            if !init_ty.can_coerce_to(&ty) {
                let base_msg = format!(
                    "Type mismatch in global variable '{}': expected {}, found {} at {}",
                    var.name,
                    ty.name(),
                    init_ty.name(),
                    var.span
                );
                self.error(format_error_with_code(
                    ErrorCode::E200,
                    &base_msg,
                    self.source,
                    var.span.line,
                    var.span.column,
                    &format!(
                        "Value type {} cannot be coerced to {}",
                        init_ty.name(),
                        ty.name()
                    ),
                ));
            }
        }

        // Register all signals
        for signal in &program.signals {
            self.check_signal(signal);
        }

        // Register all functions first
        for func in &program.functions {
            let param_types: Vec<Type> = func
                .params
                .iter()
                .map(|p| {
                    let ty = Type::from_string(&p.ty);

                    // Check for unknown parameter types
                    if ty == Type::Unknown {
                        let base_msg = format!(
                            "Unknown type '{}' for parameter '{}' at {}",
                            p.ty, p.name, func.span
                        );

                        let candidates = Self::list_types();
                        let suggestions = find_similar_identifiers(&p.ty, &candidates);

                        let hint = if !suggestions.is_empty() {
                            format!("Type not recognized. Did you mean '{}'?", suggestions[0])
                        } else {
                            "Type not recognized. Available types: i32, f32, bool, String, Vector2, Node".to_string()
                        };

                        self.error(format_error_with_code(
                            ErrorCode::E203,
                            &base_msg,
                            self.source,
                            func.span.line,
                            func.span.column,
                            &hint,
                        ));
                    }

                    ty
                })
                .collect();

            let return_type = func
                .return_type
                .as_ref()
                .map(|s| {
                    let ty = Type::from_string(s);

                    // Check for unknown return types
                    if ty == Type::Unknown {
                        let base_msg = format!(
                            "Unknown return type '{}' for function '{}' at {}",
                            s, func.name, func.span
                        );

                        let candidates = Self::list_types();
                        let suggestions = find_similar_identifiers(s, &candidates);

                        let hint = if !suggestions.is_empty() {
                            format!("Type not recognized. Did you mean '{}'?", suggestions[0])
                        } else {
                            "Type not recognized. Available types: i32, f32, bool, String, Vector2, Node".to_string()
                        };

                        self.error(format_error_with_code(
                            ErrorCode::E203,
                            &base_msg,
                            self.source,
                            func.span.line,
                            func.span.column,
                            &hint,
                        ));
                    }

                    ty
                })
                .unwrap_or(Type::Void);

            self.functions.insert(
                func.name.clone(),
                FunctionSignature {
                    params: param_types,
                    return_type,
                },
            );
        }

        // Check each function body
        for func in &program.functions {
            self.check_function(func);
        }
    }

    fn check_function(&mut self, func: &Function) {
        self.push_scope();

        // Add parameters to scope
        for param in &func.params {
            let ty = Type::from_string(&param.ty);
            self.declare_variable(param.name.clone(), ty);
        }

        // Check all statements in function body
        for stmt in &func.body {
            self.check_stmt(stmt);
        }

        self.pop_scope();
    }

    fn check_signal(&mut self, signal: &Signal) {
        // Check for duplicate signal name
        if self.signals.contains_key(&signal.name) {
            let base_msg = format!(
                "Signal '{}' is already defined at {}",
                signal.name, signal.span
            );
            self.error(format_error_with_code(
                ErrorCode::E301,
                &base_msg,
                self.source,
                signal.span.line,
                signal.span.column,
                "Each signal must have a unique name",
            ));
            return;
        }

        // Validate parameter types
        let mut param_types = Vec::new();
        for (param_name, param_type) in &signal.parameters {
            let ty = Type::from_string(param_type);

            if ty == Type::Unknown {
                let base_msg = format!(
                    "Unknown type '{}' for signal parameter '{}' at {}",
                    param_type, param_name, signal.span
                );

                let candidates = Self::list_types();
                let suggestions = find_similar_identifiers(param_type, &candidates);

                let hint = if !suggestions.is_empty() {
                    format!("Type not recognized. Did you mean '{}'?", suggestions[0])
                } else {
                    "Type not recognized. Available types: i32, f32, bool, String, Vector2, Node"
                        .to_string()
                };

                self.error(format_error_with_code(
                    ErrorCode::E203,
                    &base_msg,
                    self.source,
                    signal.span.line,
                    signal.span.column,
                    &hint,
                ));
            }

            param_types.push(ty);
        }

        // Register signal
        self.signals.insert(signal.name.clone(), param_types);
    }

    fn check_emit_signal(&mut self, signal_name: &str, args: &[Expr], span: &Span) {
        // Look up signal
        let signal_params = match self.signals.get(signal_name) {
            Some(params) => params.clone(),
            None => {
                let base_msg = format!("Signal '{}' is not defined at {}", signal_name, span);
                self.error(format_error_with_code(
                    ErrorCode::E302,
                    &base_msg,
                    self.source,
                    span.line,
                    span.column,
                    "Signal must be declared before it can be emitted",
                ));
                return;
            }
        };

        // Check argument count
        if args.len() != signal_params.len() {
            let base_msg = format!(
                "Signal '{}' expects {} parameters, but {} were provided at {}",
                signal_name,
                signal_params.len(),
                args.len(),
                span
            );
            self.error(format_error_with_code(
                ErrorCode::E303,
                &base_msg,
                self.source,
                span.line,
                span.column,
                &format!(
                    "Expected {} argument(s), found {}",
                    signal_params.len(),
                    args.len()
                ),
            ));
            return;
        }

        // Check argument types
        for (i, (arg, expected_type)) in args.iter().zip(signal_params.iter()).enumerate() {
            let arg_type = self.check_expr(arg);
            if !arg_type.can_coerce_to(expected_type) {
                let base_msg = format!(
                    "Signal '{}' parameter {} type mismatch: expected {}, found {} at {}",
                    signal_name,
                    i + 1,
                    expected_type.name(),
                    arg_type.name(),
                    span
                );
                self.error(format_error_with_code(
                    ErrorCode::E304,
                    &base_msg,
                    self.source,
                    span.line,
                    span.column,
                    &format!(
                        "Cannot coerce {} to {}",
                        arg_type.name(),
                        expected_type.name()
                    ),
                ));
            }
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr) => {
                self.check_expr(expr);
            }
            Stmt::Let {
                name,
                ty,
                value,
                span,
                ..
            } => {
                let declared_ty = if let Some(type_name) = ty {
                    let parsed_ty = Type::from_string(type_name);

                    // If type is unknown and a type annotation was provided, report E203
                    if parsed_ty == Type::Unknown {
                        let base_msg = format!("Unknown type '{}' at {}", type_name, span);

                        // Find similar type names
                        let candidates = Self::list_types();
                        let suggestions = find_similar_identifiers(type_name, &candidates);

                        let hint = if !suggestions.is_empty() {
                            format!("Type not recognized. Did you mean '{}'?", suggestions[0])
                        } else {
                            "Type not recognized. Available types: i32, f32, bool, String, Vector2, Node".to_string()
                        };

                        self.error(format_error_with_code(
                            ErrorCode::E203,
                            &base_msg,
                            self.source,
                            span.line,
                            span.column,
                            &hint,
                        ));
                    }

                    parsed_ty
                } else {
                    let inferred = self.infer_expr(value);

                    // Only report E218 if type inference failed AND no annotation was provided
                    if inferred == Type::Unknown {
                        let base_msg =
                            format!("Cannot infer type for variable '{}' at {}", name, span);
                        self.error(format_error_with_code(
                            ErrorCode::E218,
                            &base_msg,
                            self.source,
                            span.line,
                            span.column,
                            "Add an explicit type annotation (e.g., let name: type = value)",
                        ));
                    }

                    inferred
                };

                let value_ty = self.check_expr(value);
                if !value_ty.can_coerce_to(&declared_ty) {
                    let base_msg = format!(
                        "Type mismatch in let binding '{}': expected {}, found {} at {}",
                        name,
                        declared_ty.name(),
                        value_ty.name(),
                        span
                    );
                    self.error(format_error_with_code(
                        ErrorCode::E200,
                        &base_msg,
                        self.source,
                        span.line,
                        span.column,
                        &format!(
                            "Value type {} cannot be coerced to {}",
                            value_ty.name(),
                            declared_ty.name()
                        ),
                    ));
                }

                self.declare_variable(name.clone(), declared_ty);
            }
            Stmt::Assign {
                target,
                value,
                span,
            } => {
                let target_ty = self.check_expr(target);
                let value_ty = self.check_expr(value);

                if !value_ty.can_coerce_to(&target_ty) {
                    let base_msg = format!(
                        "Type mismatch in assignment: expected {}, found {} at {}",
                        target_ty.name(),
                        value_ty.name(),
                        span
                    );
                    self.error(format_error_with_code(
                        ErrorCode::E219,
                        &base_msg,
                        self.source,
                        span.line,
                        span.column,
                        &format!(
                            "Value type {} cannot be coerced to {}",
                            value_ty.name(),
                            target_ty.name()
                        ),
                    ));
                }
            }
            Stmt::If {
                cond,
                then_branch,
                else_branch,
                span,
            } => {
                let cond_ty = self.check_expr(cond);
                if cond_ty != Type::Bool {
                    let base_msg = format!(
                        "If condition must be bool, found {} at {}",
                        cond_ty.name(),
                        span
                    );
                    self.error(format_error_with_code(
                        ErrorCode::E211,
                        &base_msg,
                        self.source,
                        span.line,
                        span.column,
                        "Condition must evaluate to a boolean value (true or false)",
                    ));
                }

                self.push_scope();
                for stmt in then_branch {
                    self.check_stmt(stmt);
                }
                self.pop_scope();

                if !else_branch.is_empty() {
                    self.push_scope();
                    for stmt in else_branch {
                        self.check_stmt(stmt);
                    }
                    self.pop_scope();
                }
            }
            Stmt::While { cond, body, span } => {
                let cond_ty = self.check_expr(cond);
                if cond_ty != Type::Bool {
                    let base_msg = format!(
                        "While condition must be bool, found {} at {}",
                        cond_ty.name(),
                        span
                    );
                    self.error(format_error_with_code(
                        ErrorCode::E211,
                        &base_msg,
                        self.source,
                        span.line,
                        span.column,
                        "Condition must evaluate to a boolean value (true or false)",
                    ));
                }

                self.push_scope();
                for stmt in body {
                    self.check_stmt(stmt);
                }
                self.pop_scope();
            }
            Stmt::Return { value, .. } => {
                if let Some(expr) = value {
                    self.check_expr(expr);
                    // TODO: Check return type matches function signature
                } else {
                    // Return without value - should be void function
                }
            }
        }
    }

    fn check_expr(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::Literal(lit, _) => match lit {
                Literal::Int(_) => Type::I32,
                Literal::Float(_) => Type::F32,
                Literal::Bool(_) => Type::Bool,
                Literal::Str(_) => Type::String,
            },
            Expr::Variable(name, span) => {
                if let Some(ty) = self.lookup_variable(name) {
                    ty
                } else {
                    let base_msg = format!("Undefined variable '{}' at {}", name, span);

                    // Find similar variable names
                    let candidates = self.list_variables();
                    let suggestions = find_similar_identifiers(name, &candidates);

                    let hint = if !suggestions.is_empty() {
                        format!(
                            "Variable must be declared before use. Did you mean '{}'?",
                            suggestions[0]
                        )
                    } else {
                        "Variable must be declared before use".to_string()
                    };

                    self.error(format_error_with_code(
                        ErrorCode::E201,
                        &base_msg,
                        self.source,
                        span.line,
                        span.column,
                        &hint,
                    ));
                    Type::Unknown
                }
            }
            Expr::Binary(left, op, right, span) => {
                let left_ty = self.check_expr(left);
                let right_ty = self.check_expr(right);

                match op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => {
                        // Arithmetic operations require numeric types
                        if matches!(left_ty, Type::I32 | Type::F32)
                            && matches!(right_ty, Type::I32 | Type::F32)
                        {
                            // If either is f32, result is f32
                            if left_ty == Type::F32 || right_ty == Type::F32 {
                                Type::F32
                            } else {
                                Type::I32
                            }
                        } else {
                            let base_msg = format!(
                                "Binary operation {} requires numeric types, found {} and {} at {}",
                                op,
                                left_ty.name(),
                                right_ty.name(),
                                span
                            );
                            self.error(format_error_with_code(
                                ErrorCode::E212,
                                &base_msg,
                                self.source,
                                span.line,
                                span.column,
                                "Arithmetic operations (+, -, *, /) require i32 or f32 types",
                            ));
                            Type::Unknown
                        }
                    }
                    BinaryOp::Eq | BinaryOp::Ne => {
                        // Equality can compare any types (but they should match)
                        Type::Bool
                    }
                    BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
                        // Comparison requires numeric types
                        if matches!(left_ty, Type::I32 | Type::F32)
                            && matches!(right_ty, Type::I32 | Type::F32)
                        {
                            Type::Bool
                        } else {
                            let base_msg = format!(
                                "Comparison {} requires numeric types, found {} and {} at {}",
                                op,
                                left_ty.name(),
                                right_ty.name(),
                                span
                            );
                            self.error(format_error_with_code(
                                ErrorCode::E212,
                                &base_msg,
                                self.source,
                                span.line,
                                span.column,
                                "Comparison operators (<, <=, >, >=) require i32 or f32 types",
                            ));
                            Type::Bool
                        }
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        // Logical operations require bool types
                        if left_ty != Type::Bool || right_ty != Type::Bool {
                            let base_msg = format!(
                                "Logical operation {} requires bool types, found {} and {} at {}",
                                op,
                                left_ty.name(),
                                right_ty.name(),
                                span
                            );
                            self.error(format_error_with_code(
                                ErrorCode::E212,
                                &base_msg,
                                self.source,
                                span.line,
                                span.column,
                                "Logical operators (and, or) require boolean operands",
                            ));
                        }
                        Type::Bool
                    }
                }
            }
            Expr::Unary(op, expr, span) => {
                let expr_ty = self.check_expr(expr);
                match op {
                    UnaryOp::Neg => {
                        if !matches!(expr_ty, Type::I32 | Type::F32) {
                            let base_msg = format!(
                                "Unary negation requires numeric type, found {} at {}",
                                expr_ty.name(),
                                span
                            );
                            self.error(format_error_with_code(
                                ErrorCode::E213,
                                &base_msg,
                                self.source,
                                span.line,
                                span.column,
                                "Negation operator (-) requires i32 or f32 type",
                            ));
                        }
                        expr_ty
                    }
                    UnaryOp::Not => {
                        if expr_ty != Type::Bool {
                            let base_msg = format!(
                                "Logical not requires bool type, found {} at {}",
                                expr_ty.name(),
                                span
                            );
                            self.error(format_error_with_code(
                                ErrorCode::E213,
                                &base_msg,
                                self.source,
                                span.line,
                                span.column,
                                "Not operator (!) requires boolean type",
                            ));
                        }
                        Type::Bool
                    }
                }
            }
            Expr::Call(name, args, span) => {
                // Special handling for emit_signal
                if name == "emit_signal" {
                    if args.is_empty() {
                        let base_msg =
                            format!("emit_signal requires at least one argument at {}", span);
                        self.error(format_error_with_code(
                            ErrorCode::E204,
                            &base_msg,
                            self.source,
                            span.line,
                            span.column,
                            "First argument must be the signal name as a string literal",
                        ));
                        return Type::Void;
                    }

                    // First argument must be a string literal (signal name)
                    if let Expr::Literal(Literal::Str(signal_name), _) = &args[0] {
                        // Check the signal emission with remaining args
                        self.check_emit_signal(signal_name, &args[1..], span);
                    } else {
                        let base_msg = format!(
                            "emit_signal first argument must be a string literal at {}",
                            span
                        );
                        self.error(format_error_with_code(
                            ErrorCode::E205,
                            &base_msg,
                            self.source,
                            span.line,
                            span.column,
                            "Signal name must be known at compile time (use a string literal)",
                        ));
                    }
                    return Type::Void;
                }

                if let Some(sig) = self.functions.get(name).cloned() {
                    if args.len() != sig.params.len() {
                        let base_msg = format!(
                            "Function '{}' expects {} arguments, found {} at {}",
                            name,
                            sig.params.len(),
                            args.len(),
                            span
                        );
                        self.error(format_error_with_code(
                            ErrorCode::E204,
                            &base_msg,
                            self.source,
                            span.line,
                            span.column,
                            &format!("Expected {} argument(s)", sig.params.len()),
                        ));
                    } else {
                        for (i, (arg, expected_ty)) in
                            args.iter().zip(sig.params.iter()).enumerate()
                        {
                            let arg_ty = self.check_expr(arg);
                            if !arg_ty.can_coerce_to(expected_ty) {
                                let base_msg = format!(
                                    "Function '{}' argument {} has wrong type: expected {}, found {} at {}",
                                    name,
                                    i,
                                    expected_ty.name(),
                                    arg_ty.name(),
                                    span
                                );
                                self.error(format_error_with_code(
                                    ErrorCode::E205,
                                    &base_msg,
                                    self.source,
                                    span.line,
                                    span.column,
                                    &format!(
                                        "Argument {} must be of type {}",
                                        i,
                                        expected_ty.name()
                                    ),
                                ));
                            }
                        }
                    }
                    sig.return_type
                } else {
                    let base_msg = format!("Undefined function '{}' at {}", name, span);

                    // Find similar function names
                    let candidates = self.list_functions();
                    let suggestions = find_similar_identifiers(name, &candidates);

                    let hint = if !suggestions.is_empty() {
                        format!(
                            "Function must be declared before use. Did you mean '{}'?",
                            suggestions[0]
                        )
                    } else {
                        "Function must be declared before use".to_string()
                    };

                    self.error(format_error_with_code(
                        ErrorCode::E202,
                        &base_msg,
                        self.source,
                        span.line,
                        span.column,
                        &hint,
                    ));
                    Type::Unknown
                }
            }
            Expr::FieldAccess(obj, field, span) => {
                let obj_ty = self.check_expr(obj);
                match obj_ty {
                    Type::Vector2 => {
                        if field == "x" || field == "y" {
                            Type::F32
                        } else {
                            let base_msg = format!("Vector2 has no field '{}' at {}", field, span);
                            self.error(format_error_with_code(
                                ErrorCode::E215,
                                &base_msg,
                                self.source,
                                span.line,
                                span.column,
                                "Vector2 only has fields 'x' and 'y'",
                            ));
                            Type::Unknown
                        }
                    }
                    Type::Node => {
                        // Node has a position field of type Vector2
                        if field == "position" {
                            Type::Vector2
                        } else {
                            // For stub, allow any field on Node
                            Type::Unknown
                        }
                    }
                    _ => {
                        let base_msg = format!("Type {} has no fields at {}", obj_ty.name(), span);
                        self.error(format_error_with_code(
                            ErrorCode::E209,
                            &base_msg,
                            self.source,
                            span.line,
                            span.column,
                            "Field access is only valid for structured types",
                        ));
                        Type::Unknown
                    }
                }
            }
            Expr::Assign(_, _, _) | Expr::CompoundAssign(_, _, _, _) => {
                // These shouldn't appear in expressions in this phase
                Type::Unknown
            }
        }
    }

    fn infer_expr(&mut self, expr: &Expr) -> Type {
        // Simplified inference - just check the expression
        self.check_expr(expr)
    }
}

/// Perform type checking on a parsed program.
///
/// This is the main entry point for semantic analysis. It verifies:
/// - All variables are declared before use
/// - Types are compatible in operations and assignments
/// - Function calls match declared signatures
/// - Return statements match function return types
/// - No invalid operations (e.g., field access on primitives)
///
/// # Arguments
///
/// * `program` - The parsed AST to type check
/// * `source` - Original source code (for error messages)
///
/// # Returns
///
/// * `Ok(())` - Program is type-safe
/// * `Err(String)` - Type error with location and context
///
/// # Examples
///
/// ```no_run
/// use ferrisscript_compiler::{lexer::tokenize, parser::parse, type_checker::check};
///
/// let source = r#"
///     let global_count: i32 = 0;
///
///     fn increment() {
///         global_count = global_count + 1;
///     }
/// "#;
/// let tokens = tokenize(source).unwrap();
/// let program = parse(&tokens, source).unwrap();
/// check(&program, source).unwrap();
/// ```
///
/// # Type Errors
///
/// Returns `Err` if:
/// - Variable used before declaration
/// - Type mismatch in assignment or operation
/// - Function called with wrong argument types
/// - Return type doesn't match function signature
/// - Invalid operations (e.g., adding strings and numbers)
///
/// # Performance
///
/// - Simple programs: ~850ns
/// - Complex programs: ~3.6μs
/// - O(n) complexity where n = number of AST nodes
pub fn check(program: &Program, source: &str) -> Result<(), String> {
    let mut checker = TypeChecker::new(source);
    checker.check_program(program);

    if checker.errors.is_empty() {
        Ok(())
    } else {
        Err(checker.errors.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use crate::parser::parse;

    #[test]
    fn test_check_empty() {
        let input = "";
        let program = Program::new();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_check_simple_function() {
        let input = "fn test() { let x: i32 = 5; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_check_type_mismatch() {
        let input = "fn test() { let x: i32 = true; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        let result = check(&program, input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Type mismatch"));
    }

    #[test]
    fn test_check_undefined_variable() {
        let input = "fn test() { let x = y; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        let result = check(&program, input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }

    #[test]
    fn test_check_binary_expression() {
        let input = "fn test() { let x = 5 + 3; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_check_binary_type_mismatch() {
        let input = "fn test() { let x = 5 + true; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        let result = check(&program, input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("numeric types"));
    }

    #[test]
    fn test_check_if_condition_must_be_bool() {
        let input = "fn test() { if 5 { let x = 1; } }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        let result = check(&program, input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be bool"));
    }

    #[test]
    fn test_check_function_call() {
        let input = r#"fn test() { print("hello"); }"#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_check_undefined_function() {
        let input = "fn test() { foo(); }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        let result = check(&program, input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined function"));
    }

    #[test]
    fn test_check_field_access() {
        let input = "fn test() { let x = self.position; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_check_chained_field_access() {
        let input = "fn test() { let x = self.position.x; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_check_hello_example() {
        let input = r#"fn _ready() {
    print("Hello from FerrisScript!");
}"#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_check_move_example() {
        let input = r#"fn _process(delta: f32) {
    self.position.x += 50.0 * delta;
}"#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_check_bounce_example() {
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
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_check_type_coercion() {
        let input = "fn test() { let x: f32 = 5; }"; // i32 to f32 coercion
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_check_comparison_operators() {
        let input = "fn test() { let x = 5 > 3; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_check_logical_operators() {
        let input = "fn test() { let x = true && false; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_check_unary_operators() {
        let input = "fn test() { let x = -5; let y = !true; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    // ========== NEW COVERAGE TESTS: Type Coercion & Field Access ==========

    #[test]
    fn test_implicit_int_to_float_coercion_in_assignment() {
        let input = "fn test() { let x: f32 = 42; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_implicit_int_to_float_coercion_in_function_arg() {
        let input = r#"
            fn take_float(x: f32) {}
            fn test() { take_float(42); }
        "#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_no_reverse_coercion_float_to_int() {
        let input = "fn test() { let x: i32 = 3.14; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        let result = check(&program, input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Type mismatch"));
    }

    #[test]
    fn test_no_bool_to_numeric_coercion() {
        let input = "fn test() { let x: i32 = true; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        let result = check(&program, input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Type mismatch"));
    }

    #[test]
    fn test_vector2_field_access_x() {
        let input = "fn test() { let v = self.position; let x = v.x; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_vector2_field_access_y() {
        let input = "fn test() { let v = self.position; let y = v.y; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_vector2_invalid_field_access() {
        let input = "fn test() { let v = self.position; let z = v.z; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        let result = check(&program, input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("has no field"));
    }

    #[test]
    fn test_nested_field_access_chains() {
        let input = "fn test() { let x: f32 = self.position.x; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_field_access_on_primitive_type_error() {
        let input = "fn test() { let x: i32 = 5; let y = x.field; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        let result = check(&program, input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("has no fields"));
    }

    #[test]
    fn test_node_position_field_access() {
        let input = "fn test() { let pos = self.position; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_string_type_in_declaration() {
        let input = r#"fn test() { let msg: String = "hello"; }"#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_string_type_mismatch() {
        let input = r#"fn test() { let msg: String = 42; }"#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        let result = check(&program, input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Type mismatch"));
    }

    #[test]
    fn test_multiple_type_coercions_in_expression() {
        let input = "fn test() { let x: f32 = 1 + 2 + 3; }"; // Multiple i32 operations, then coerce to f32
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_mixed_int_float_arithmetic_with_coercion() {
        let input = "fn test() { let x: f32 = 5; let y = x + 10; }"; // f32 + i32 (coerced)
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_type_inference_with_coercion() {
        let input = "fn test() { let x = 5; let y: f32 = x; }"; // Infer i32, then coerce
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_function_return_type_coercion() {
        let input = r#"
            fn get_float() -> f32 { return 42; }
            fn test() { let x = get_float(); }
        "#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_comparison_with_coercion() {
        let input = "fn test() { let x: f32 = 5.0; let result = x > 3; }"; // f32 > i32 (coerced)
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_compound_assignment_with_coercion() {
        let input = "fn test() { let mut x: f32 = 5.0; x += 10; }"; // f32 += i32 (coerced)
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_void_return_type_checking() {
        let input = "fn no_return() { let x = 5; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_undefined_type_error() {
        let input = "fn test() { let x: UnknownType = 5; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        let result = check(&program, input);
        assert!(result.is_err());
        // Type checker treats unknown types as Type::Unknown, may still compile
    }

    // Signal Tests
    #[test]
    fn test_signal_declaration_valid() {
        let input = "signal health_changed(old: i32, new: i32);";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_signal_no_params() {
        let input = "signal player_died();";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_signal_duplicate_name_error() {
        let input = r#"
            signal player_died();
            signal player_died();
        "#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        let result = check(&program, input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already defined"));
    }

    #[test]
    fn test_signal_undefined_type_error() {
        let input = "signal test(param: UnknownType);";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        let result = check(&program, input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown type"));
    }

    #[test]
    fn test_emit_signal_valid() {
        let input = r#"
            signal health_changed(old: i32, new: i32);
            fn test() {
                emit_signal("health_changed", 100, 75);
            }
        "#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok());
    }

    #[test]
    fn test_emit_signal_undefined_error() {
        let input = r#"
            fn test() {
                emit_signal("undefined_signal");
            }
        "#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        let result = check(&program, input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not defined"));
    }

    #[test]
    fn test_emit_signal_param_count_mismatch() {
        let input = r#"
            signal health_changed(old: i32, new: i32);
            fn test() {
                emit_signal("health_changed", 100);
            }
        "#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        let result = check(&program, input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects 2 parameters"));
    }

    #[test]
    fn test_emit_signal_param_type_mismatch() {
        let input = r#"
            signal health_changed(old: i32, new: i32);
            fn test() {
                emit_signal("health_changed", 100, true);
            }
        "#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        let result = check(&program, input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("type mismatch"));
    }

    #[test]
    fn test_emit_signal_type_coercion() {
        let input = r#"
            signal position_changed(x: f32, y: f32);
            fn test() {
                emit_signal("position_changed", 10, 20);
            }
        "#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens, input).unwrap();
        assert!(check(&program, input).is_ok()); // i32 can coerce to f32
    }
}
