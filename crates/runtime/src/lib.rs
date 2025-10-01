use rustyscript_compiler::ast;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Nil,
}

pub struct Env {
    scopes: Vec<HashMap<String, Value>>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            scopes: vec![HashMap::new()],
        }
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
}

pub fn execute(program: &ast::Program, env: &mut Env) -> Result<(), String> {
    // Placeholder: walk AST and execute statements
    // TODO: Implement proper runtime execution in Phase 5
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
