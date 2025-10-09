use ferrisscript_compiler::{ast, compile};
use ferrisscript_runtime::{call_function, execute, Env, Value};
use godot::classes::{file_access::ModeFlags, FileAccess};
use godot::prelude::*;
use std::cell::RefCell;

// Signal prototype module for v0.0.4 research
mod signal_prototype;
pub use signal_prototype::SignalPrototype;

// Thread-local storage for node properties during script execution
thread_local! {
    static NODE_POSITION: RefCell<Option<Vector2>> = const { RefCell::new(None) };
}

/// Property getter for self binding (called from runtime)
fn get_node_property_tls(property_name: &str) -> Result<Value, String> {
    match property_name {
        "position" => NODE_POSITION.with(|pos| {
            pos.borrow()
                .map(|p| Value::Vector2 { x: p.x, y: p.y })
                .ok_or_else(|| "Node position not available".to_string())
        }),
        _ => Err(format!("Property '{}' not supported", property_name)),
    }
}

/// Property setter for self binding (called from runtime)
fn set_node_property_tls(property_name: &str, value: Value) -> Result<(), String> {
    match property_name {
        "position" => {
            if let Value::Vector2 { x, y } = value {
                NODE_POSITION.with(|pos| {
                    *pos.borrow_mut() = Some(Vector2::new(x, y));
                });
                Ok(())
            } else {
                Err(format!("Expected Vector2 for position, got {:?}", value))
            }
        }
        _ => Err(format!("Property '{}' not supported", property_name)),
    }
}

/// Convert FerrisScript Value to Godot Variant
fn value_to_variant(value: &Value) -> Variant {
    match value {
        Value::Int(i) => Variant::from(*i),
        Value::Float(f) => Variant::from(*f),
        Value::Bool(b) => Variant::from(*b),
        Value::String(s) => Variant::from(s.as_str()),
        Value::Vector2 { x, y } => Variant::from(Vector2::new(*x, *y)),
        Value::Nil => Variant::nil(),
        Value::SelfObject => Variant::nil(), // self cannot be passed as signal parameter
    }
}

/// Godot-specific print function that outputs to Godot's console
fn godot_print_builtin(args: &[Value]) -> Result<Value, String> {
    let output = args
        .iter()
        .map(|v| match v {
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::String(s) => s.clone(),
            Value::Vector2 { x, y } => format!("Vector2({}, {})", x, y),
            Value::Nil => "nil".to_string(),
            Value::SelfObject => "self".to_string(),
        })
        .collect::<Vec<_>>()
        .join(" ");

    godot_print!("{}", output);
    Ok(Value::Nil)
}

struct FerrisScriptExtension;

#[gdextension]
unsafe impl ExtensionLibrary for FerrisScriptExtension {}

#[derive(GodotClass)]
#[class(base=Node2D)] // Changed from Node to Node2D for position property
pub struct FerrisScriptNode {
    base: Base<Node2D>,

    /// Path to the .ferris script file (e.g., "res://scripts/hello.ferris")
    #[export(file = "*.ferris")]
    script_path: GString,

    // Runtime state
    env: Option<Env>,
    program: Option<ast::Program>,
    script_loaded: bool,
}

#[godot_api]
impl INode2D for FerrisScriptNode {
    fn init(base: Base<Node2D>) -> Self {
        FerrisScriptNode {
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

        // Register signals with Godot if script is loaded
        if self.script_loaded {
            if let Some(program) = &self.program {
                // Clone signal names to avoid borrowing issues
                let signal_names: Vec<String> =
                    program.signals.iter().map(|s| s.name.clone()).collect();

                for signal_name in signal_names {
                    self.base_mut().add_user_signal(&signal_name);
                    godot_print!("Registered signal: {}", signal_name);
                }
            }
        }

        // Execute _ready function if it exists
        if self.script_loaded {
            self.call_script_function("_ready", &[]);
        }
    }

    fn process(&mut self, delta: f64) {
        // Execute _process function if script is loaded
        if self.script_loaded {
            // Convert delta to Float (f32 for FerrisScript)
            let delta_value = Value::Float(delta as f32);
            self.call_script_function_with_self("_process", &[delta_value]);
        }
    }
}

#[godot_api]
impl FerrisScriptNode {
    /// Load and compile the FerrisScript file
    fn load_script(&mut self) {
        let path_gstring = self.script_path.clone();
        let path = path_gstring.to_string();

        // Use Godot's FileAccess to read the file (handles res:// paths correctly)
        let file = match FileAccess::open(&path_gstring, ModeFlags::READ) {
            Some(f) => f,
            None => {
                godot_error!(
                    "Failed to open script file '{}': File not found or cannot be accessed",
                    path
                );
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

        godot_print!("Successfully loaded FerrisScript: {}", path);
    }

    /// Call a function in the loaded script with self binding
    fn call_script_function_with_self(
        &mut self,
        function_name: &str,
        args: &[Value],
    ) -> Option<Value> {
        if !self.script_loaded {
            godot_warn!("Cannot call function '{}': no script loaded", function_name);
            return None;
        }

        // Get current node position and store in thread-local storage
        let position = self.base().get_position();
        NODE_POSITION.with(|pos| {
            *pos.borrow_mut() = Some(position);
        });

        // Store the node's instance ID for signal emission
        let instance_id = self.base().instance_id();

        let env = self.env.as_mut()?;

        // Set up 'self' variable and property callbacks
        env.push_scope();
        env.set("self".to_string(), Value::SelfObject);
        env.set_property_getter(get_node_property_tls);
        env.set_property_setter(set_node_property_tls);

        // Set up signal emitter callback using instance ID
        env.set_signal_emitter(Box::new(move |signal_name: &str, args: &[Value]| {
            // Convert FerrisScript Values to Godot Variants
            let variant_args: Vec<Variant> = args.iter().map(value_to_variant).collect();

            // Try to get the node by instance ID and emit signal
            match Gd::<Node2D>::try_from_instance_id(instance_id) {
                Ok(mut node) => {
                    node.emit_signal(signal_name, &variant_args);
                    Ok(())
                }
                Err(_) => Err("Node no longer exists".to_string()),
            }
        }));

        let result = match call_function(function_name, args, env) {
            Ok(value) => Some(value),
            Err(e) => {
                godot_error!("Error calling function '{}': {}", function_name, e);
                None
            }
        };

        env.pop_scope();

        // Read back position from thread-local storage and update node
        NODE_POSITION.with(|pos| {
            if let Some(new_position) = *pos.borrow() {
                self.base_mut().set_position(new_position);
            }
        });

        result
    }

    /// Call a function in the loaded script (without self binding)
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
        // Placeholder test - godot_bind integration tests run in Godot
    }
}
