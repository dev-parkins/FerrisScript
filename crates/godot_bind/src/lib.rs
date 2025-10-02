use godot::prelude::*;
use godot::classes::{FileAccess, file_access::ModeFlags};
use rustyscript_compiler::{ast, compile};
use rustyscript_runtime::{Env, Value, call_function, execute};

/// Godot-specific print function that outputs to Godot's console
fn godot_print_builtin(args: &[Value]) -> Result<Value, String> {
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
    
    godot_print!("{}", output);
    Ok(Value::Nil)
}

struct RustyScriptExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustyScriptExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct RustyScriptNode {
    base: Base<Node>,
    
    /// Path to the .rscr script file (e.g., "res://scripts/hello.rscr")
    #[export(file = "*.rscr")]
    script_path: GString,
    
    // Runtime state
    env: Option<Env>,
    program: Option<ast::Program>,
    script_loaded: bool,
}

#[godot_api]
impl INode for RustyScriptNode {
    fn init(base: Base<Node>) -> Self {
        RustyScriptNode {
            base,
            script_path: GString::new(),
            env: None,
            program: None,
            script_loaded: false,
        }
    }

    fn ready(&mut self) {
        // Load and compile script if path is set
        if !self.script_path.is_empty() {
            self.load_script();
        }
        
        // Execute _ready function if it exists
        if self.script_loaded {
            self.call_script_function("_ready", &[]);
        }
    }

    fn process(&mut self, _delta: f64) {
        // TODO: Execute _process function from script in Phase 7
    }
}

#[godot_api]
impl RustyScriptNode {
    /// Load and compile the RustyScript file
    fn load_script(&mut self) {
        let path_gstring = self.script_path.clone();
        let path = path_gstring.to_string();
        
        // Use Godot's FileAccess to read the file (handles res:// paths correctly)
        let file = match FileAccess::open(path_gstring.clone(), ModeFlags::READ) {
            Some(f) => f,
            None => {
                godot_error!("Failed to open script file '{}': File not found or cannot be accessed", path);
                return;
            }
        };
        
        // Read the entire file as a string
        let source = file.get_as_text().to_string();
        
        // Compile the script
        let program = match compile(&source) {
            Ok(prog) => prog,
            Err(e) => {
                godot_error!("Failed to compile script '{}': {}", path, e);
                return;
            }
        };
        
        // Create runtime environment and execute initialization
        let mut env = Env::new();
        
        // Override print() to use Godot's console
        env.register_builtin("print".to_string(), godot_print_builtin);
        
        if let Err(e) = execute(&program, &mut env) {
            godot_error!("Failed to initialize script '{}': {}", path, e);
            return;
        }
        
        self.program = Some(program);
        self.env = Some(env);
        self.script_loaded = true;
        
        godot_print!("Successfully loaded RustyScript: {}", path);
    }
    
    /// Call a function in the loaded script
    fn call_script_function(&mut self, function_name: &str, args: &[Value]) -> Option<Value> {
        if !self.script_loaded {
            godot_warn!("Cannot call function '{}': no script loaded", function_name);
            return None;
        }
        
        let env = self.env.as_mut()?;
        
        match call_function(function_name, args, env) {
            Ok(value) => Some(value),
            Err(e) => {
                godot_error!("Error calling function '{}': {}", function_name, e);
                None
            }
        }
    }
    
    /// Reload the script (useful for hot-reloading in development)
    #[func]
    pub fn reload_script(&mut self) {
        self.script_loaded = false;
        self.env = None;
        self.program = None;
        self.load_script();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
