use rustyscript_compiler::ast::{self, BinaryOp, UnaryOp};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Vector2 { x: f32, y: f32 },
    Nil,
}

impl Value {
    /// Coerce value to float if possible
    pub fn to_float(&self) -> Option<f32> {
        match self {
            Value::Int(i) => Some(*i as f32),
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Coerce value to bool
    pub fn to_bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::Nil => false,
            _ => true,
        }
    }
}

pub struct Env {
    scopes: Vec<HashMap<String, Value>>,
    functions: HashMap<String, ast::Function>,
    builtin_fns: HashMap<String, fn(&[Value]) -> Result<Value, String>>,
}

impl Env {
    pub fn new() -> Self {
        let mut env = Env {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
            builtin_fns: HashMap::new(),
        };
        
        // Register built-in functions
        env.builtin_fns.insert("print".to_string(), builtin_print);
        
        env
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn set(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value);
            }
        }
        None
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Value> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(value) = scope.get_mut(name) {
                return Some(value);
            }
        }
        None
    }

    pub fn define_function(&mut self, name: String, func: ast::Function) {
        self.functions.insert(name, func);
    }

    pub fn get_function(&self, name: &str) -> Option<&ast::Function> {
        self.functions.get(name)
    }

    pub fn call_builtin(&self, name: &str, args: &[Value]) -> Result<Value, String> {
        if let Some(func) = self.builtin_fns.get(name) {
            func(args)
        } else {
            Err(format!("Unknown built-in function: {}", name))
        }
    }

    pub fn is_builtin(&self, name: &str) -> bool {
        self.builtin_fns.contains_key(name)
    }
}

// Built-in function implementations
fn builtin_print(args: &[Value]) -> Result<Value, String> {
    let output = args.iter()
        .map(|v| match v {
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::String(s) => s.clone(),
            Value::Vector2 { x, y } => format!("Vector2({}, {})", x, y),
            Value::Nil => "nil".to_string(),
        })
        .collect::<Vec<_>>()
        .join(" ");
    
    println!("{}", output);
    Ok(Value::Nil)
}

/// Control flow result
#[derive(Debug, Clone, PartialEq)]
enum FlowControl {
    None,
    Return(Value),
}

pub fn execute(program: &ast::Program, env: &mut Env) -> Result<(), String> {
    // Initialize global variables
    for global in &program.global_vars {
        let value = evaluate_expr(&global.value, env)?;
        env.set(global.name.clone(), value);
    }

    // Register all functions
    for func in &program.functions {
        env.define_function(func.name.clone(), func.clone());
    }

    Ok(())
}

fn execute_stmt(stmt: &ast::Stmt, env: &mut Env) -> Result<FlowControl, String> {
    match stmt {
        ast::Stmt::Let { name, value, .. } => {
            let val = evaluate_expr(value, env)?;
            env.set(name.clone(), val);
            Ok(FlowControl::None)
        }
        
        ast::Stmt::Assign { target, value, .. } => {
            let val = evaluate_expr(value, env)?;
            
            // Handle field access assignment (e.g., self.position.x = value)
            if let ast::Expr::FieldAccess(object, field, _) = target {
                assign_field(object, field, val, env)?;
            } else if let ast::Expr::Variable(name, _) = target {
                if let Some(var) = env.get_mut(name) {
                    *var = val;
                } else {
                    return Err(format!("Undefined variable: {}", name));
                }
            } else {
                return Err("Invalid assignment target".to_string());
            }
            
            Ok(FlowControl::None)
        }
        
        ast::Stmt::If { cond, then_branch, else_branch, .. } => {
            let cond_val = evaluate_expr(cond, env)?;
            
            if cond_val.to_bool() {
                for stmt in then_branch {
                    let flow = execute_stmt(stmt, env)?;
                    if flow != FlowControl::None {
                        return Ok(flow);
                    }
                }
            } else if !else_branch.is_empty() {
                for stmt in else_branch {
                    let flow = execute_stmt(stmt, env)?;
                    if flow != FlowControl::None {
                        return Ok(flow);
                    }
                }
            }
            
            Ok(FlowControl::None)
        }
        
        ast::Stmt::While { cond, body, .. } => {
            loop {
                let cond_val = evaluate_expr(cond, env)?;
                if !cond_val.to_bool() {
                    break;
                }
                
                for stmt in body {
                    let flow = execute_stmt(stmt, env)?;
                    if flow != FlowControl::None {
                        return Ok(flow);
                    }
                }
            }
            
            Ok(FlowControl::None)
        }
        
        ast::Stmt::Return { value, .. } => {
            let val = if let Some(expr) = value {
                evaluate_expr(expr, env)?
            } else {
                Value::Nil
            };
            Ok(FlowControl::Return(val))
        }
        
        ast::Stmt::Expr(expr) => {
            evaluate_expr(expr, env)?;
            Ok(FlowControl::None)
        }
    }
}

fn assign_field(object: &ast::Expr, field: &str, value: Value, env: &mut Env) -> Result<(), String> {
    match object {
        ast::Expr::Variable(name, _) => {
            if let Some(var) = env.get_mut(name) {
                match var {
                    Value::Vector2 { x, y } => {
                        match field {
                            "x" => {
                                if let Some(f) = value.to_float() {
                                    *x = f;
                                } else {
                                    return Err(format!("Cannot assign {:?} to Vector2.x", value));
                                }
                            }
                            "y" => {
                                if let Some(f) = value.to_float() {
                                    *y = f;
                                } else {
                                    return Err(format!("Cannot assign {:?} to Vector2.y", value));
                                }
                            }
                            _ => return Err(format!("Vector2 has no field '{}'", field)),
                        }
                    }
                    _ => return Err(format!("Cannot access field '{}' on {:?}", field, var)),
                }
                Ok(())
            } else {
                Err(format!("Undefined variable: {}", name))
            }
        }
        
        ast::Expr::FieldAccess(object, parent_field, _) => {
            // Handle nested field access (e.g., self.position.x)
            if let ast::Expr::Variable(name, _) = &**object {
                if let Some(var) = env.get_mut(name) {
                    match var {
                        Value::Vector2 { .. } => {
                            // Access nested field
                            if parent_field == "position" || parent_field == "velocity" {
                                // For now, treat these as Vector2 fields
                                match field {
                                    "x" | "y" => {
                                        if let Some(_f) = value.to_float() {
                                            // Simplified: in real impl, would need nested structure
                                            return Err("Nested field assignment not fully implemented".to_string());
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            Err("Complex field assignment not yet implemented".to_string())
        }
        
        _ => Err("Invalid assignment target".to_string()),
    }
}

fn evaluate_expr(expr: &ast::Expr, env: &mut Env) -> Result<Value, String> {
    match expr {
        ast::Expr::Literal(lit, _) => {
            Ok(match lit {
                ast::Literal::Int(i) => Value::Int(*i),
                ast::Literal::Float(f) => Value::Float(*f),
                ast::Literal::Bool(b) => Value::Bool(*b),
                ast::Literal::Str(s) => Value::String(s.clone()),
            })
        }
        
        ast::Expr::Variable(name, _) => {
            env.get(name)
                .cloned()
                .ok_or_else(|| format!("Undefined variable: {}", name))
        }
        
        ast::Expr::Binary(left, op, right, _) => {
            let left_val = evaluate_expr(left, env)?;
            let right_val = evaluate_expr(right, env)?;
            
            match op {
                BinaryOp::Add => {
                    match (&left_val, &right_val) {
                        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                        _ => {
                            let a = left_val.to_float().ok_or("Cannot add non-numeric values")?;
                            let b = right_val.to_float().ok_or("Cannot add non-numeric values")?;
                            Ok(Value::Float(a + b))
                        }
                    }
                }
                
                BinaryOp::Sub => {
                    match (&left_val, &right_val) {
                        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
                        _ => {
                            let a = left_val.to_float().ok_or("Cannot subtract non-numeric values")?;
                            let b = right_val.to_float().ok_or("Cannot subtract non-numeric values")?;
                            Ok(Value::Float(a - b))
                        }
                    }
                }
                
                BinaryOp::Mul => {
                    match (&left_val, &right_val) {
                        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
                        _ => {
                            let a = left_val.to_float().ok_or("Cannot multiply non-numeric values")?;
                            let b = right_val.to_float().ok_or("Cannot multiply non-numeric values")?;
                            Ok(Value::Float(a * b))
                        }
                    }
                }
                
                BinaryOp::Div => {
                    match (&left_val, &right_val) {
                        (Value::Int(a), Value::Int(b)) => {
                            if *b == 0 {
                                return Err("Division by zero".to_string());
                            }
                            Ok(Value::Int(a / b))
                        }
                        _ => {
                            let a = left_val.to_float().ok_or("Cannot divide non-numeric values")?;
                            let b = right_val.to_float().ok_or("Cannot divide non-numeric values")?;
                            if b == 0.0 {
                                return Err("Division by zero".to_string());
                            }
                            Ok(Value::Float(a / b))
                        }
                    }
                }
                
                BinaryOp::Eq => Ok(Value::Bool(left_val == right_val)),
                BinaryOp::Ne => Ok(Value::Bool(left_val != right_val)),
                
                BinaryOp::Lt => {
                    match (&left_val, &right_val) {
                        (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
                        _ => {
                            let a = left_val.to_float().ok_or("Cannot compare non-numeric values")?;
                            let b = right_val.to_float().ok_or("Cannot compare non-numeric values")?;
                            Ok(Value::Bool(a < b))
                        }
                    }
                }
                
                BinaryOp::Le => {
                    match (&left_val, &right_val) {
                        (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
                        _ => {
                            let a = left_val.to_float().ok_or("Cannot compare non-numeric values")?;
                            let b = right_val.to_float().ok_or("Cannot compare non-numeric values")?;
                            Ok(Value::Bool(a <= b))
                        }
                    }
                }
                
                BinaryOp::Gt => {
                    match (&left_val, &right_val) {
                        (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
                        _ => {
                            let a = left_val.to_float().ok_or("Cannot compare non-numeric values")?;
                            let b = right_val.to_float().ok_or("Cannot compare non-numeric values")?;
                            Ok(Value::Bool(a > b))
                        }
                    }
                }
                
                BinaryOp::Ge => {
                    match (&left_val, &right_val) {
                        (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
                        _ => {
                            let a = left_val.to_float().ok_or("Cannot compare non-numeric values")?;
                            let b = right_val.to_float().ok_or("Cannot compare non-numeric values")?;
                            Ok(Value::Bool(a >= b))
                        }
                    }
                }
                
                BinaryOp::And => {
                    let a = left_val.to_bool();
                    let b = right_val.to_bool();
                    Ok(Value::Bool(a && b))
                }
                
                BinaryOp::Or => {
                    let a = left_val.to_bool();
                    let b = right_val.to_bool();
                    Ok(Value::Bool(a || b))
                }
            }
        }
        
        ast::Expr::Unary(op, operand, _) => {
            let val = evaluate_expr(operand, env)?;
            
            match op {
                UnaryOp::Neg => {
                    match val {
                        Value::Int(i) => Ok(Value::Int(-i)),
                        Value::Float(f) => Ok(Value::Float(-f)),
                        _ => Err("Cannot negate non-numeric value".to_string()),
                    }
                }
                
                UnaryOp::Not => {
                    Ok(Value::Bool(!val.to_bool()))
                }
            }
        }
        
        ast::Expr::Call(name, args, _) => {
            // Evaluate arguments
            let arg_values: Result<Vec<_>, _> = args.iter()
                .map(|arg| evaluate_expr(arg, env))
                .collect();
            let arg_values = arg_values?;
            
            // Check if it's a built-in function
            if env.is_builtin(name) {
                return env.call_builtin(name, &arg_values);
            }
            
            // Look up user-defined function
            let func = env.get_function(name)
                .ok_or_else(|| format!("Undefined function: {}", name))?
                .clone();
            
            // Check arity
            if func.params.len() != arg_values.len() {
                return Err(format!(
                    "Function {} expects {} arguments, got {}",
                    name,
                    func.params.len(),
                    arg_values.len()
                ));
            }
            
            // Create new scope for function
            env.push_scope();
            
            // Bind parameters
            for (param, arg_val) in func.params.iter().zip(arg_values.iter()) {
                env.set(param.name.clone(), arg_val.clone());
            }
            
            // Execute function body
            let mut return_val = Value::Nil;
            for stmt in &func.body {
                match execute_stmt(stmt, env)? {
                    FlowControl::Return(val) => {
                        return_val = val;
                        break;
                    }
                    FlowControl::None => {}
                }
            }
            
            env.pop_scope();
            
            Ok(return_val)
        }
        
        ast::Expr::FieldAccess(object, field, _) => {
            let obj_val = evaluate_expr(object, env)?;
            
            match obj_val {
                Value::Vector2 { x, y } => {
                    match field.as_str() {
                        "x" => Ok(Value::Float(x)),
                        "y" => Ok(Value::Float(y)),
                        _ => Err(format!("Vector2 has no field '{}'", field)),
                    }
                }
                _ => Err(format!("Cannot access field '{}' on {:?}", field, obj_val)),
            }
        }
        
        // Compound assignment and regular assignment expressions not used in runtime
        // They are desugared to Stmt::Assign at parse time
        ast::Expr::Assign(_, _, _) | ast::Expr::CompoundAssign(_, _, _, _) => {
            Err("Assignment expressions should be statements".to_string())
        }
    }
}

/// Execute a function by name (used for Godot callbacks)
pub fn call_function(name: &str, args: &[Value], env: &mut Env) -> Result<Value, String> {
    if env.is_builtin(name) {
        return env.call_builtin(name, args);
    }
    
    let func = env.get_function(name)
        .ok_or_else(|| format!("Undefined function: {}", name))?
        .clone();
    
    if func.params.len() != args.len() {
        return Err(format!(
            "Function {} expects {} arguments, got {}",
            name,
            func.params.len(),
            args.len()
        ));
    }
    
    env.push_scope();
    
    for (param, arg_val) in func.params.iter().zip(args.iter()) {
        env.set(param.name.clone(), arg_val.clone());
    }
    
    let mut return_val = Value::Nil;
    for stmt in &func.body {
        match execute_stmt(stmt, env)? {
            FlowControl::Return(val) => {
                return_val = val;
                break;
            }
            FlowControl::None => {}
        }
    }
    
    env.pop_scope();
    
    Ok(return_val)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustyscript_compiler::compile;

    #[test]
    fn test_env_basic() {
        let mut env = Env::new();
        env.set("x".to_string(), Value::Int(42));
        assert_eq!(env.get("x"), Some(&Value::Int(42)));
    }

    #[test]
    fn test_env_scopes() {
        let mut env = Env::new();
        env.set("x".to_string(), Value::Int(1));
        
        env.push_scope();
        env.set("x".to_string(), Value::Int(2));
        assert_eq!(env.get("x"), Some(&Value::Int(2)));
        
        env.pop_scope();
        assert_eq!(env.get("x"), Some(&Value::Int(1)));
    }

    #[test]
    fn test_value_coercion() {
        assert_eq!(Value::Int(42).to_float(), Some(42.0));
        assert_eq!(Value::Float(3.14).to_float(), Some(3.14));
        assert_eq!(Value::Bool(true).to_float(), None);
        
        assert!(Value::Bool(true).to_bool());
        assert!(!Value::Bool(false).to_bool());
        assert!(Value::Int(1).to_bool());
        assert!(!Value::Int(0).to_bool());
        assert!(!Value::Nil.to_bool());
    }

    #[test]
    fn test_builtin_print() {
        let env = Env::new();
        let args = vec![Value::String("Hello".to_string()), Value::Int(42)];
        let result = env.call_builtin("print", &args);
        assert_eq!(result, Ok(Value::Nil));
    }

    #[test]
    fn test_evaluate_literals() {
        let mut env = Env::new();
        
        let source = "fn test() { let x: i32 = 42; }";
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        
        // Test that we can call the function
        call_function("test", &[], &mut env).unwrap();
    }

    #[test]
    fn test_arithmetic_operations() {
        let mut env = Env::new();
        
        let source = r#"
            fn test() -> i32 {
                let a: i32 = 10;
                let b: i32 = 5;
                return a + b * 2;
            }
        "#;
        
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        
        let result = call_function("test", &[], &mut env).unwrap();
        assert_eq!(result, Value::Int(20));
    }

    #[test]
    fn test_comparison_operations() {
        let mut env = Env::new();
        
        let source = r#"
            fn test() -> bool {
                let x: i32 = 10;
                return x > 5;
            }
        "#;
        
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        
        let result = call_function("test", &[], &mut env).unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_logical_operations() {
        let mut env = Env::new();
        
        let source = r#"
            fn test() -> bool {
                let a: bool = true;
                let b: bool = false;
                return a && !b;
            }
        "#;
        
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        
        let result = call_function("test", &[], &mut env).unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_if_statement() {
        let mut env = Env::new();
        
        let source = r#"
            fn test(x: i32) -> i32 {
                if x > 10 {
                    return 1;
                } else {
                    return 0;
                }
            }
        "#;
        
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        
        let result1 = call_function("test", &[Value::Int(15)], &mut env).unwrap();
        assert_eq!(result1, Value::Int(1));
        
        let result2 = call_function("test", &[Value::Int(5)], &mut env).unwrap();
        assert_eq!(result2, Value::Int(0));
    }

    #[test]
    fn test_while_loop() {
        let mut env = Env::new();
        
        let source = r#"
            fn test() -> i32 {
                let mut count: i32 = 0;
                while count < 5 {
                    count = count + 1;
                }
                return count;
            }
        "#;
        
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        
        let result = call_function("test", &[], &mut env).unwrap();
        assert_eq!(result, Value::Int(5));
    }

    #[test]
    fn test_global_variables() {
        let mut env = Env::new();
        
        let source = r#"
            let mut counter: i32 = 0;
            
            fn increment() {
                counter = counter + 1;
            }
            
            fn get_counter() -> i32 {
                return counter;
            }
        "#;
        
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        
        // Initial value
        let result1 = call_function("get_counter", &[], &mut env).unwrap();
        assert_eq!(result1, Value::Int(0));
        
        // After increment
        call_function("increment", &[], &mut env).unwrap();
        let result2 = call_function("get_counter", &[], &mut env).unwrap();
        assert_eq!(result2, Value::Int(1));
        
        // After another increment
        call_function("increment", &[], &mut env).unwrap();
        let result3 = call_function("get_counter", &[], &mut env).unwrap();
        assert_eq!(result3, Value::Int(2));
    }

    #[test]
    fn test_function_parameters() {
        let mut env = Env::new();
        
        let source = r#"
            fn add(a: i32, b: i32) -> i32 {
                return a + b;
            }
        "#;
        
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        
        let result = call_function("add", &[Value::Int(10), Value::Int(32)], &mut env).unwrap();
        assert_eq!(result, Value::Int(42));
    }

    #[test]
    fn test_type_coercion_at_runtime() {
        let mut env = Env::new();
        
        let source = r#"
            fn test() -> f32 {
                let x: i32 = 10;
                let y: f32 = 3.14;
                return x + y;
            }
        "#;
        
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        
        let result = call_function("test", &[], &mut env).unwrap();
        // i32 should be coerced to f32
        assert_eq!(result, Value::Float(13.14));
    }

    #[test]
    fn test_vector2_field_access() {
        let mut env = Env::new();
        
        // Manually set up the Vector2 since we don't have Vector2 construction yet
        // This test demonstrates that field access works when the variable exists
        env.set("pos".to_string(), Value::Vector2 { x: 10.0, y: 20.0 });
        
        // Create a simple function manually to test field access
        env.define_function("get_x".to_string(), ast::Function {
            name: "get_x".to_string(),
            params: vec![],
            return_type: Some("f32".to_string()),
            body: vec![ast::Stmt::Return {
                value: Some(ast::Expr::FieldAccess(
                    Box::new(ast::Expr::Variable("pos".to_string(), ast::Span::unknown())),
                    "x".to_string(),
                    ast::Span::unknown()
                )),
                span: ast::Span::unknown(),
            }],
            span: ast::Span::unknown(),
        });
        
        let result = call_function("get_x", &[], &mut env).unwrap();
        assert_eq!(result, Value::Float(10.0));
    }

    #[test]
    fn test_hello_example() {
        let mut env = Env::new();
        
        let source = r#"
            fn _ready() {
                print("Hello, Godot!");
            }
        "#;
        
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        
        // Should not panic when calling print
        let result = call_function("_ready", &[], &mut env);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unary_negation() {
        let mut env = Env::new();
        
        let source = r#"
            fn test() -> i32 {
                let x: i32 = 5;
                return -x;
            }
        "#;
        
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        
        let result = call_function("test", &[], &mut env).unwrap();
        assert_eq!(result, Value::Int(-5));
    }

    #[test]
    fn test_division_by_zero() {
        let mut env = Env::new();
        
        let source = r#"
            fn test() -> i32 {
                let x: i32 = 10;
                let y: i32 = 0;
                return x / y;
            }
        "#;
        
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        
        let result = call_function("test", &[], &mut env);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Division by zero"));
    }

    #[test]
    fn test_undefined_variable() {
        let mut env = Env::new();
        
        // This should be caught by type checker, but let's test runtime behavior
        // We'll manually create a function that references an undefined variable
        env.define_function("test".to_string(), ast::Function {
            name: "test".to_string(),
            params: vec![],
            return_type: Some("i32".to_string()),
            body: vec![ast::Stmt::Return {
                value: Some(ast::Expr::Variable("undefined_var".to_string(), ast::Span::unknown())),
                span: ast::Span::unknown(),
            }],
            span: ast::Span::unknown(),
        });
        
        let result = call_function("test", &[], &mut env);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }
}
