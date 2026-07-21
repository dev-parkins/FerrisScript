# Runtime Patterns

**Load this skill when working in `crates/runtime/`**

## Architecture Overview

```
src/
└── lib.rs  # Runtime interpreter, variable scoping, builtins, evaluation
```

The runtime is a **tree-walking interpreter** that directly evaluates the AST produced by the compiler. It's not a bytecode VM.

## Core Concepts

### Environment (Variable Scoping)

```rust
struct Environment {
    scopes: Vec<HashMap<String, Value>>,
}

impl Environment {
    fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()],  // Global scope
        }
    }
    
    fn define(&mut self, name: String, value: Value, mutable: bool) {
        self.scopes.last_mut().unwrap().insert(name, value);
    }
    
    fn get(&self, name: &str) -> Option<&Value> {
        // Search from innermost to outermost scope
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value);
            }
        }
        None
    }
    
    fn set(&mut self, name: &str, value: Value) -> Result<(), RuntimeError> {
        // Find the scope where variable is defined and update it
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }
        Err(RuntimeError::UndefinedVariable(name.to_string()))
    }
    
    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    
    fn exit_scope(&mut self) {
        self.scopes.pop();
    }
}
```

**Key patterns:**

- Scopes are a stack of HashMaps
- `get()` searches from innermost to outermost (lexical scoping)
- `set()` finds the defining scope and updates it
- `enter_scope()` / `exit_scope()` manage scope lifecycle

### Value Representation

```rust
#[derive(Debug, Clone)]
pub enum Value {
    I32(i32),
    F32(f32),
    Bool(bool),
    String(String),
    Vector2 { x: f32, y: f32 },
    Color { r: f32, g: f32, b: f32, a: f32 },
    Rect2 { position: Value, size: Value },
    Transform2D { position: Value, rotation: f32, scale: Value },
    Node(NodeRef),
    Null,
}
```

**Type conversions:**

- `i32` → `f32`: Automatic widening (safe)
- `f32` → `i32`: Truncation (may lose precision)
- Other conversions: Explicit only, or runtime error

### Evaluation Flow

```rust
impl Runtime {
    pub fn execute(&mut self, program: &Program) -> Result<Value, RuntimeError> {
        // 1. Execute global variable declarations
        for decl in &program.global_vars {
            self.execute_variable_decl(decl)?;
        }
        
        // 2. Register function definitions
        for func in &program.functions {
            self.register_function(func);
        }
        
        // 3. Call _ready() if it exists
        if self.has_function("_ready") {
            self.call_function("_ready", &[])?;
        }
        
        Ok(Value::Null)
    }
    
    pub fn call_function(&mut self, name: &str, args: &[Value]) -> Result<Value, RuntimeError> {
        let func = self.get_function(name)?;
        
        // Enter function scope
        self.env.enter_scope();
        
        // Bind parameters
        for (param, arg) in func.params.iter().zip(args.iter()) {
            self.env.define(param.name.clone(), arg.clone(), true);
        }
        
        // Execute function body
        let result = self.execute_block(&func.body)?;
        
        // Exit function scope
        self.env.exit_scope();
        
        Ok(result)
    }
    
    fn execute_statement(&mut self, stmt: &Statement) -> Result<ControlFlow, RuntimeError> {
        match stmt {
            Statement::VariableDecl(decl) => {
                let value = self.evaluate(&decl.initializer)?;
                self.env.define(decl.name.clone(), value, decl.mutable);
                Ok(ControlFlow::Continue)
            }
            Statement::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(ControlFlow::Continue)
            }
            Statement::Return(expr) => {
                let value = self.evaluate(expr)?;
                Ok(ControlFlow::Return(value))
            }
            Statement::If(if_stmt) => {
                let condition = self.evaluate(&if_stmt.condition)?;
                if self.is_truthy(&condition) {
                    self.execute_block(&if_stmt.then_branch)
                } else if let Some(else_branch) = &if_stmt.else_branch {
                    self.execute_block(else_branch)
                } else {
                    Ok(ControlFlow::Continue)
                }
            }
            Statement::While(while_stmt) => {
                loop {
                    let condition = self.evaluate(&while_stmt.condition)?;
                    if !self.is_truthy(&condition) {
                        break;
                    }
                    match self.execute_block(&while_stmt.body)? {
                        ControlFlow::Return(val) => return Ok(ControlFlow::Return(val)),
                        ControlFlow::Break => break,
                        ControlFlow::Continue => continue,
                    }
                }
                Ok(ControlFlow::Continue)
            }
        }
    }
    
    fn evaluate(&mut self, expr: &Expression) -> Result<Value, RuntimeError> {
        match expr {
            Expression::Literal(value) => Ok(value.clone()),
            Expression::Identifier(name) => {
                self.env.get(name)
                    .cloned()
                    .ok_or_else(|| RuntimeError::UndefinedVariable(name.clone()))
            }
            Expression::BinaryOp { left, op, right } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.evaluate_binary_op(left_val, op, right_val)
            }
            Expression::FunctionCall { name, args } => {
                let arg_values: Vec<Value> = args.iter()
                    .map(|arg| self.evaluate(arg))
                    .collect::<Result<_, _>>()?;
                self.call_function(name, &arg_values)
            }
        }
    }
}
```

**Control flow:**

- `ControlFlow::Continue` — keep executing
- `ControlFlow::Return(Value)` — return from function
- `ControlFlow::Break` — exit loop

## Builtin Functions

```rust
impl Runtime {
    fn call_builtin(&mut self, name: &str, args: &[Value]) -> Result<Value, RuntimeError> {
        match name {
            "print" => {
                for arg in args {
                    println!("{}", self.value_to_string(arg));
                }
                Ok(Value::Null)
            }
            "get_node" => {
                let path = self.expect_string(&args[0])?;
                self.godot_interface.get_node(&path)
            }
            "has_node" => {
                let path = self.expect_string(&args[0])?;
                Ok(Value::Bool(self.godot_interface.has_node(&path)))
            }
            "emit_signal" => {
                let signal_name = self.expect_string(&args[0])?;
                let signal_args = &args[1..];
                self.godot_interface.emit_signal(&signal_name, signal_args)
            }
            _ => Err(RuntimeError::UndefinedFunction(name.to_string())),
        }
    }
}
```

**Adding new builtins:**

1. Add match arm in `call_builtin()`
2. Validate argument count and types
3. Return appropriate `Value`
4. Add tests in `crates/runtime/src/lib.rs`

## Godot Integration

The runtime communicates with Godot through a `GodotInterface`:

```rust
pub trait GodotInterface {
    fn get_node(&self, path: &str) -> Result<Value, RuntimeError>;
    fn has_node(&self, path: &str) -> bool;
    fn set_property(&mut self, node: &str, property: &str, value: Value) -> Result<(), RuntimeError>;
    fn get_property(&self, node: &str, property: &str) -> Result<Value, RuntimeError>;
    fn emit_signal(&mut self, signal: &str, args: &[Value]) -> Result<(), RuntimeError>;
}
```

**Self binding:**

- `self.position` → calls `get_property("self", "position")`
- `self.position.x = 10.0` → calls `set_property("self", "position.x", Value::F32(10.0))`

## Lifecycle Callbacks

```rust
pub fn run_frame(&mut self, delta: f32) -> Result<(), RuntimeError> {
    // Call _process(delta) if it exists
    if self.has_function("_process") {
        self.call_function("_process", &[Value::F32(delta)])?;
    }
    
    // Call _physics_process(delta) if it exists
    if self.has_function("_physics_process") {
        self.call_function("_physics_process", &[Value::F32(delta)])?;
    }
    
    Ok(())
}
```

**Lifecycle order:**

1. `_ready()` — called once when node enters scene tree
2. `_process(delta)` — called every frame
3. `_physics_process(delta)` — called every physics frame
4. `_input(event)` — called on input events
5. `_enter_tree()` — called when node is added to scene
6. `_exit_tree()` — called when node is removed from scene

## Performance Characteristics

| Operation | Performance | Notes |
|-----------|-------------|-------|
| Function call | ~1.05 μs | Per-call overhead |
| Loop iteration | ~180 ns | Per-iteration overhead |
| Variable lookup | ~50 ns | HashMap lookup |
| Property access | ~200 ns | Includes Godot FFI call |

**Optimization tips:**

- Cache `self.position` in local variables to avoid repeated FFI calls
- Minimize cross-boundary calls (Rust ↔ Godot)
- Use `f32` for game math (matches Godot's internal representation)

## Testing Patterns

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_variable_scoping() {
        let source = r#"
            let x: i32 = 10;
            fn test() -> i32 {
                let y: i32 = 20;
                return x + y;
            }
        "#;
        let mut runtime = Runtime::new(source);
        let result = runtime.execute().unwrap();
        assert_eq!(result, Value::I32(30));
    }
    
    #[test]
    fn test_function_calls() {
        let source = r#"
            fn add(a: i32, b: i32) -> i32 {
                return a + b;
            }
            let result: i32 = add(5, 7);
        "#;
        let mut runtime = Runtime::new(source);
        runtime.execute().unwrap();
        assert_eq!(runtime.get_variable("result"), Some(&Value::I32(12)));
    }
    
    #[test]
    fn test_immutability() {
        let source = r#"
            let x: i32 = 10;
            x = 20;  // Should fail — x is not mutable
        "#;
        let mut runtime = Runtime::new(source);
        let result = runtime.execute();
        assert!(result.is_err());
    }
}
```

## Common Tasks

### Adding a new builtin function

1. Add match arm in `call_builtin()`
2. Validate arguments (count, types)
3. Implement logic
4. Return appropriate `Value`
5. Add unit tests
6. Update `AGENTS.md` if it's a commonly used builtin

### Adding a new Godot type

1. Add variant to `Value` enum
2. Implement type literal parsing in compiler
3. Add type checking rules
4. Implement evaluation in runtime
5. Add Godot conversion in `crates/godot_bind/`
6. Add integration test

### Debugging runtime issues

```rust
// Add debug prints
eprintln!("Executing: {:?}", stmt);
eprintln!("Environment: {:?}", self.env);

// Use godot_print! for Godot console output
godot_print!("Debug: {}", value);
```

## Error Handling

```rust
pub enum RuntimeError {
    UndefinedVariable(String),
    UndefinedFunction(String),
    TypeMismatch { expected: Type, found: Type },
    DivisionByZero,
    InvalidOperation(String),
    GodotError(String),
}
```

**Always include context in error messages:**

```rust
RuntimeError::TypeMismatch {
    expected: Type::I32,
    found: Type::String,
}
```
