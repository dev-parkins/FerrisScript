use crate::ast::*;
use std::collections::HashMap;

/// Type representation for type checking
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
struct TypeChecker {
    // Variable types in current scope (stack of scopes)
    scopes: Vec<HashMap<String, Type>>,
    // Function signatures
    functions: HashMap<String, FunctionSignature>,
    // Current errors
    errors: Vec<String>,
}

impl TypeChecker {
    fn new() -> Self {
        let mut checker = TypeChecker {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
            errors: Vec::new(),
        };

        // Register built-in functions
        checker.functions.insert(
            "print".to_string(),
            FunctionSignature {
                params: vec![Type::String],
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

    fn error(&mut self, message: String) {
        self.errors.push(message);
    }

    fn check_program(&mut self, program: &Program) {
        // Register global variables
        for var in &program.global_vars {
            let ty = if let Some(type_name) = &var.ty {
                Type::from_string(type_name)
            } else {
                self.infer_expr(&var.value)
            };

            if ty == Type::Unknown {
                self.error(format!(
                    "Cannot infer type for global variable '{}' at {}",
                    var.name, var.span
                ));
            }

            self.declare_variable(var.name.clone(), ty.clone());

            // Check that initializer matches declared type
            let init_ty = self.check_expr(&var.value);
            if !init_ty.can_coerce_to(&ty) {
                self.error(format!(
                    "Type mismatch in global variable '{}': expected {}, found {} at {}",
                    var.name,
                    ty.name(),
                    init_ty.name(),
                    var.span
                ));
            }
        }

        // Register all functions first
        for func in &program.functions {
            let param_types = func
                .params
                .iter()
                .map(|p| Type::from_string(&p.ty))
                .collect();

            let return_type = func
                .return_type
                .as_ref()
                .map(|s| Type::from_string(s))
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
                    Type::from_string(type_name)
                } else {
                    self.infer_expr(value)
                };

                if declared_ty == Type::Unknown {
                    self.error(format!(
                        "Cannot infer type for variable '{}' at {}",
                        name, span
                    ));
                }

                let value_ty = self.check_expr(value);
                if !value_ty.can_coerce_to(&declared_ty) {
                    self.error(format!(
                        "Type mismatch in let binding '{}': expected {}, found {} at {}",
                        name,
                        declared_ty.name(),
                        value_ty.name(),
                        span
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
                    self.error(format!(
                        "Type mismatch in assignment: expected {}, found {} at {}",
                        target_ty.name(),
                        value_ty.name(),
                        span
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
                    self.error(format!(
                        "If condition must be bool, found {} at {}",
                        cond_ty.name(),
                        span
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
                    self.error(format!(
                        "While condition must be bool, found {} at {}",
                        cond_ty.name(),
                        span
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
                    self.error(format!("Undefined variable '{}' at {}", name, span));
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
                            self.error(format!(
                                "Binary operation {} requires numeric types, found {} and {} at {}",
                                op,
                                left_ty.name(),
                                right_ty.name(),
                                span
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
                            self.error(format!(
                                "Comparison {} requires numeric types, found {} and {} at {}",
                                op,
                                left_ty.name(),
                                right_ty.name(),
                                span
                            ));
                            Type::Bool
                        }
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        // Logical operations require bool types
                        if left_ty != Type::Bool || right_ty != Type::Bool {
                            self.error(format!(
                                "Logical operation {} requires bool types, found {} and {} at {}",
                                op,
                                left_ty.name(),
                                right_ty.name(),
                                span
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
                            self.error(format!(
                                "Unary negation requires numeric type, found {} at {}",
                                expr_ty.name(),
                                span
                            ));
                        }
                        expr_ty
                    }
                    UnaryOp::Not => {
                        if expr_ty != Type::Bool {
                            self.error(format!(
                                "Logical not requires bool type, found {} at {}",
                                expr_ty.name(),
                                span
                            ));
                        }
                        Type::Bool
                    }
                }
            }
            Expr::Call(name, args, span) => {
                if let Some(sig) = self.functions.get(name).cloned() {
                    if args.len() != sig.params.len() {
                        self.error(format!(
                            "Function '{}' expects {} arguments, found {} at {}",
                            name,
                            sig.params.len(),
                            args.len(),
                            span
                        ));
                    } else {
                        for (i, (arg, expected_ty)) in args.iter().zip(sig.params.iter()).enumerate()
                        {
                            let arg_ty = self.check_expr(arg);
                            if !arg_ty.can_coerce_to(expected_ty) {
                                self.error(format!(
                                    "Function '{}' argument {} has wrong type: expected {}, found {} at {}",
                                    name,
                                    i,
                                    expected_ty.name(),
                                    arg_ty.name(),
                                    span
                                ));
                            }
                        }
                    }
                    sig.return_type
                } else {
                    self.error(format!("Undefined function '{}' at {}", name, span));
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
                            self.error(format!(
                                "Vector2 has no field '{}' at {}",
                                field, span
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
                        self.error(format!(
                            "Type {} has no fields at {}",
                            obj_ty.name(),
                            span
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

pub fn check(program: &Program) -> Result<(), String> {
    let mut checker = TypeChecker::new();
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
        let program = Program::new();
        assert!(check(&program).is_ok());
    }

    #[test]
    fn test_check_simple_function() {
        let input = "fn test() { let x: i32 = 5; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens).unwrap();
        assert!(check(&program).is_ok());
    }

    #[test]
    fn test_check_type_mismatch() {
        let input = "fn test() { let x: i32 = true; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens).unwrap();
        let result = check(&program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Type mismatch"));
    }

    #[test]
    fn test_check_undefined_variable() {
        let input = "fn test() { let x = y; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens).unwrap();
        let result = check(&program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }

    #[test]
    fn test_check_binary_expression() {
        let input = "fn test() { let x = 5 + 3; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens).unwrap();
        assert!(check(&program).is_ok());
    }

    #[test]
    fn test_check_binary_type_mismatch() {
        let input = "fn test() { let x = 5 + true; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens).unwrap();
        let result = check(&program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("numeric types"));
    }

    #[test]
    fn test_check_if_condition_must_be_bool() {
        let input = "fn test() { if 5 { let x = 1; } }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens).unwrap();
        let result = check(&program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be bool"));
    }

    #[test]
    fn test_check_function_call() {
        let input = r#"fn test() { print("hello"); }"#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens).unwrap();
        assert!(check(&program).is_ok());
    }

    #[test]
    fn test_check_undefined_function() {
        let input = "fn test() { foo(); }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens).unwrap();
        let result = check(&program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined function"));
    }

    #[test]
    fn test_check_field_access() {
        let input = "fn test() { let x = self.position; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens).unwrap();
        assert!(check(&program).is_ok());
    }

    #[test]
    fn test_check_chained_field_access() {
        let input = "fn test() { let x = self.position.x; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens).unwrap();
        assert!(check(&program).is_ok());
    }

    #[test]
    fn test_check_hello_example() {
        let input = r#"fn _ready() {
    print("Hello from RustyScript!");
}"#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens).unwrap();
        assert!(check(&program).is_ok());
    }

    #[test]
    fn test_check_move_example() {
        let input = r#"fn _process(delta: f32) {
    self.position.x += 50.0 * delta;
}"#;
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens).unwrap();
        assert!(check(&program).is_ok());
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
        let program = parse(&tokens).unwrap();
        assert!(check(&program).is_ok());
    }

    #[test]
    fn test_check_type_coercion() {
        let input = "fn test() { let x: f32 = 5; }"; // i32 to f32 coercion
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens).unwrap();
        assert!(check(&program).is_ok());
    }

    #[test]
    fn test_check_comparison_operators() {
        let input = "fn test() { let x = 5 > 3; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens).unwrap();
        assert!(check(&program).is_ok());
    }

    #[test]
    fn test_check_logical_operators() {
        let input = "fn test() { let x = true && false; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens).unwrap();
        assert!(check(&program).is_ok());
    }

    #[test]
    fn test_check_unary_operators() {
        let input = "fn test() { let x = -5; let y = !true; }";
        let tokens = tokenize(input).unwrap();
        let program = parse(&tokens).unwrap();
        assert!(check(&program).is_ok());
    }
}
