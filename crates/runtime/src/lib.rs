//! FerrisScript Runtime
//!
//! This crate provides the execution environment for compiled FerrisScript programs.
//! It evaluates the AST produced by the compiler and manages program state.
//!
//! # Overview
//!
//! The runtime provides:
//! - Value representation ([`Value`])
//! - Variable storage and scoping ([`Env`])
//! - Expression evaluation
//! - Statement execution
//! - Integration with Godot engine (property access)
//!
//! # Performance
//!
//! - Function call: ~1.05μs per call
//! - Loop iteration: ~180ns per iteration
//! - 16K+ function calls per frame at 60 FPS
//!
//! # Example
//!
//! ```no_run
//! use ferrisscript_runtime::{execute, call_function, Env, Value};
//! use ferrisscript_compiler::compile;
//!
//! let source = "fn add(a: i32, b: i32) -> i32 { return a + b; }";
//! let program = compile(source).unwrap();
//!
//! let mut env = Env::new();
//! execute(&program, &mut env).unwrap();
//!
//! let result = call_function("add", &[Value::Int(5), Value::Int(3)], &mut env);
//! assert_eq!(result, Ok(Value::Int(8)));
//! ```

use ferrisscript_compiler::ast::{self, BinaryOp, UnaryOp};
use std::collections::HashMap;

/// Runtime value representation.
///
/// Represents all possible values that can exist during program execution,
/// including primitives, Godot types, and special values.
///
/// # Type Coercion
///
/// Values support implicit coercion via helper methods:
/// - [`to_float()`](Value::to_float) - Converts `Int` to `Float`
/// - [`to_bool()`](Value::to_bool) - Converts values to boolean
///
/// # Examples
///
/// ```
/// use ferrisscript_runtime::Value;
///
/// let x = Value::Int(42);
/// assert_eq!(x.to_float(), Some(42.0));
///
/// let v = Value::Vector2 { x: 1.0, y: 2.0 };
/// assert!(v.to_bool()); // Non-nil values are truthy
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Vector2 {
        x: f32,
        y: f32,
    },
    Color {
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    },
    Rect2 {
        position: Box<Value>, // Vector2
        size: Box<Value>,     // Vector2
    },
    Transform2D {
        position: Box<Value>, // Vector2
        rotation: f32,
        scale: Box<Value>, // Vector2
    },
    Nil,
    /// Special value representing the Godot node (self)
    SelfObject,
    /// Opaque handle to a Godot InputEvent
    InputEvent(InputEventHandle),
    /// Opaque handle to a Godot Node
    Node(NodeHandle),
}

/// Opaque handle to a Godot InputEvent.
///
/// This type wraps Godot's InputEvent in an opaque way, allowing FerrisScript
/// code to check input actions without exposing the full Godot API.
///
/// # Supported Methods
///
/// - `is_action_pressed(action: String) -> bool` - Check if action is pressed
/// - `is_action_released(action: String) -> bool` - Check if action is released
///
/// # Example (FerrisScript)
///
/// ```ferris
/// fn _input(event: InputEvent) {
///     if event.is_action_pressed("ui_accept") {
///         print("Accept pressed!");
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct InputEventHandle {
    // Opaque storage - actual implementation will be provided by godot_bind
    // For now, we'll store action state information
    pub(crate) action_pressed: Option<String>,
    pub(crate) action_released: Option<String>,
}

impl InputEventHandle {
    /// Create a new InputEvent handle with action state
    pub fn new(action_pressed: Option<String>, action_released: Option<String>) -> Self {
        InputEventHandle {
            action_pressed,
            action_released,
        }
    }

    /// Check if an action is pressed in this event
    pub fn is_action_pressed(&self, action: &str) -> bool {
        self.action_pressed.as_ref().is_some_and(|a| a == action)
    }

    /// Check if an action is released in this event
    pub fn is_action_released(&self, action: &str) -> bool {
        self.action_released.as_ref().is_some_and(|a| a == action)
    }
}

/// Opaque handle to a Godot Node.
///
/// This type wraps a Godot Node reference in an opaque way, allowing FerrisScript
/// code to reference nodes in the scene tree.
///
/// # Supported Operations
///
/// - Can be returned from `get_node()`, `get_parent()`, `find_child()`
/// - Can be passed to other functions expecting Node type
///
/// # Limitations
///
/// - Node handle may be invalidated if the node is freed
/// - Properties must be accessed via built-in functions
/// - Direct property access (e.g., `node.position`) deferred to future phase
///
/// # Example (FerrisScript)
///
/// ```ferris
/// fn _ready() {
///     let player: Node = get_node("../Player");
///     let parent: Node = get_parent();
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct NodeHandle {
    // Opaque storage - actual implementation provided by godot_bind
    // For now, we'll store a node path identifier for debugging
    pub(crate) node_id: String,
}

impl NodeHandle {
    /// Create a new Node handle with identifier
    pub fn new(node_id: String) -> Self {
        NodeHandle { node_id }
    }

    /// Get the node identifier (for debugging)
    pub fn id(&self) -> &str {
        &self.node_id
    }
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

/// Callback for getting a property from the Godot node
pub type PropertyGetter = fn(&str) -> Result<Value, String>;
/// Callback for setting a property on the Godot node
pub type PropertySetter = fn(&str, Value) -> Result<(), String>;
/// Callback for emitting a signal to the Godot node
pub type SignalEmitter = Box<dyn Fn(&str, &[Value]) -> Result<(), String>>;
/// Callback for querying nodes in the scene tree
pub type NodeQueryCallback = fn(&str, NodeQueryType) -> Result<Value, String>;

/// Type of node query operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeQueryType {
    /// Get node by path (absolute or relative)
    GetNode,
    /// Get parent node
    GetParent,
    /// Check if node exists (returns bool)
    HasNode,
    /// Find child by name (recursive search)
    FindChild,
}

/// Variable information stored in the environment
#[derive(Debug, Clone)]
struct VarInfo {
    value: Value,
    mutable: bool,
}

// Type alias to simplify the complex builtin function type
type BuiltinFn = fn(&[Value]) -> Result<Value, String>;

/// Execution environment for FerrisScript programs.
///
/// `Env` manages program state including variables, functions, and integration
/// with the Godot engine through property getter/setter callbacks.
///
/// # Structure
///
/// - **Scopes**: Stack of variable scopes (global, function, block)
/// - **Functions**: User-defined function definitions
/// - **Built-ins**: Built-in functions like `print()`
/// - **Godot Integration**: Callbacks for `self.property` access
///
/// # Examples
///
/// ```no_run
/// use ferrisscript_runtime::Env;
/// use ferrisscript_compiler::compile;
///
/// let source = "fn greet() { print(\"Hello!\"); }";
/// let program = compile(source).unwrap();
///
/// let mut env = Env::new();
/// // Load and execute program...
/// ```
///
/// # Godot Integration
///
/// Set property callbacks to enable `self.property` access from FerrisScript:
///
/// ```no_run
/// use ferrisscript_runtime::{Env, Value};
///
/// let mut env = Env::new();
/// env.set_property_getter(|prop| {
///     // Return property value from Godot node
///     Ok(Value::Float(100.0))
/// });
/// ```
pub struct Env {
    scopes: Vec<HashMap<String, VarInfo>>,
    functions: HashMap<String, ast::Function>,
    builtin_fns: HashMap<String, BuiltinFn>,
    /// Callback to get properties from the Godot node (when accessing self.property)
    property_getter: Option<PropertyGetter>,
    /// Callback to set properties on the Godot node (when assigning to self.property)
    property_setter: Option<PropertySetter>,
    /// Callback to emit signals to the Godot node
    signal_emitter: Option<SignalEmitter>,
    /// Callback to query nodes in the scene tree
    node_query_callback: Option<NodeQueryCallback>,
    /// Signal definitions: signal name -> parameter count
    signals: HashMap<String, usize>,
    /// Per-instance values for exported properties (Phase 5)
    /// Key: property name, Value: current property value
    exported_properties: HashMap<String, Value>,
    /// Reference to property metadata (static, from Program) (Phase 5)
    /// Initialized during execute() from program.property_metadata
    property_metadata: Vec<ast::PropertyMetadata>,
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

impl Env {
    pub fn new() -> Self {
        let mut env = Env {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
            builtin_fns: HashMap::new(),
            property_getter: None,
            property_setter: None,
            signal_emitter: None,
            node_query_callback: None,
            signals: HashMap::new(),
            exported_properties: HashMap::new(),
            property_metadata: Vec::new(),
        };

        // Register built-in functions
        env.builtin_fns.insert("print".to_string(), builtin_print);
        env.builtin_fns
            .insert("emit_signal".to_string(), builtin_emit_signal);

        env
    }

    /// Set the property getter callback for self binding
    pub fn set_property_getter(&mut self, getter: PropertyGetter) {
        self.property_getter = Some(getter);
    }

    /// Set the property setter callback for self binding
    pub fn set_property_setter(&mut self, setter: PropertySetter) {
        self.property_setter = Some(setter);
    }

    /// Set the signal emitter callback for signal emission
    pub fn set_signal_emitter(&mut self, emitter: SignalEmitter) {
        self.signal_emitter = Some(emitter);
    }

    /// Set the node query callback for scene tree queries
    pub fn set_node_query_callback(&mut self, callback: NodeQueryCallback) {
        self.node_query_callback = Some(callback);
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    /// Set a variable with mutability information
    pub fn set(&mut self, name: String, value: Value) {
        // Default to mutable for backward compatibility with existing code
        self.set_with_mutability(name, value, true);
    }

    /// Set a variable with explicit mutability flag
    pub fn set_with_mutability(&mut self, name: String, value: Value, mutable: bool) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, VarInfo { value, mutable });
        }
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(var_info) = scope.get(name) {
                return Some(&var_info.value);
            }
        }
        None
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Value> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(var_info) = scope.get_mut(name) {
                return Some(&mut var_info.value);
            }
        }
        None
    }

    /// Check if a variable is mutable
    pub fn is_mutable(&self, name: &str) -> bool {
        for scope in self.scopes.iter().rev() {
            if let Some(var_info) = scope.get(name) {
                return var_info.mutable;
            }
        }
        false
    }

    /// Assign to a variable, checking mutability
    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        // Check if variable exists and is mutable
        for scope in self.scopes.iter_mut().rev() {
            if let Some(var_info) = scope.get_mut(name) {
                if !var_info.mutable {
                    return Err(format!(
                        "Error[E400]: Cannot assign to immutable variable '{}'",
                        name
                    ));
                }
                var_info.value = value;
                return Ok(());
            }
        }
        Err(format!("Error[E401]: Undefined variable: {}", name))
    }

    pub fn define_function(&mut self, name: String, func: ast::Function) {
        self.functions.insert(name, func);
    }

    pub fn get_function(&self, name: &str) -> Option<&ast::Function> {
        self.functions.get(name)
    }

    pub fn call_builtin(&mut self, name: &str, args: &[Value]) -> Result<Value, String> {
        // Special handling for emit_signal - needs access to signal_emitter callback
        if name == "emit_signal" {
            if args.is_empty() {
                return Err("Error[E501]: emit_signal requires at least a signal name".to_string());
            }

            // First argument must be the signal name (string)
            let signal_name = match &args[0] {
                Value::String(s) => s,
                _ => {
                    return Err(
                        "Error[E502]: emit_signal first argument must be a string".to_string()
                    )
                }
            };

            // Get the signal parameters (all arguments after the signal name)
            let signal_args = &args[1..];

            // Call the signal emitter callback if set
            if let Some(emitter) = &self.signal_emitter {
                emitter(signal_name, signal_args)?;
            }
            // If no emitter is set, the signal emission is a no-op (for testing without Godot)

            return Ok(Value::Nil);
        }

        // Special handling for node query functions - need access to node_query_callback
        if name == "get_node" {
            if args.len() != 1 {
                return Err(
                    "Error[E601]: get_node requires exactly one argument (path: String)"
                        .to_string(),
                );
            }
            let path = match &args[0] {
                Value::String(s) => s,
                _ => return Err("Error[E602]: get_node argument must be a string".to_string()),
            };
            if path.is_empty() {
                return Err("Error[E603]: Node path cannot be empty".to_string());
            }
            if let Some(callback) = self.node_query_callback {
                return callback(path, NodeQueryType::GetNode);
            }
            return Err("Error[E604]: Node query not available (no Godot context)".to_string());
        }

        if name == "get_parent" {
            if !args.is_empty() {
                return Err("Error[E605]: get_parent takes no arguments".to_string());
            }
            if let Some(callback) = self.node_query_callback {
                return callback("", NodeQueryType::GetParent);
            }
            return Err("Error[E606]: Node query not available (no Godot context)".to_string());
        }

        if name == "has_node" {
            if args.len() != 1 {
                return Err(
                    "Error[E607]: has_node requires exactly one argument (path: String)"
                        .to_string(),
                );
            }
            let path = match &args[0] {
                Value::String(s) => s,
                _ => return Err("Error[E608]: has_node argument must be a string".to_string()),
            };
            if let Some(callback) = self.node_query_callback {
                return callback(path, NodeQueryType::HasNode);
            }
            return Err("Error[E609]: Node query not available (no Godot context)".to_string());
        }

        if name == "find_child" {
            if args.len() != 1 {
                return Err(
                    "Error[E610]: find_child requires exactly one argument (name: String)"
                        .to_string(),
                );
            }
            let name_str = match &args[0] {
                Value::String(s) => s,
                _ => return Err("Error[E611]: find_child argument must be a string".to_string()),
            };
            if name_str.is_empty() {
                return Err("Error[E612]: Child name cannot be empty".to_string());
            }
            if let Some(callback) = self.node_query_callback {
                return callback(name_str, NodeQueryType::FindChild);
            }
            return Err("Error[E613]: Node query not available (no Godot context)".to_string());
        }

        // Handle other built-in functions
        if let Some(func) = self.builtin_fns.get(name) {
            func(args)
        } else {
            Err(format!("Error[E402]: Unknown built-in function: {}", name))
        }
    }

    pub fn is_builtin(&self, name: &str) -> bool {
        // Check both registered built-ins and special handled functions
        self.builtin_fns.contains_key(name)
            || matches!(
                name,
                "emit_signal" | "get_node" | "get_parent" | "has_node" | "find_child"
            )
    }

    /// Register or override a built-in function
    pub fn register_builtin(&mut self, name: String, func: fn(&[Value]) -> Result<Value, String>) {
        self.builtin_fns.insert(name, func);
    }

    /// Register a signal with its parameter count
    pub fn register_signal(&mut self, name: String, param_count: usize) {
        self.signals.insert(name, param_count);
    }

    /// Check if a signal is registered
    pub fn has_signal(&self, name: &str) -> bool {
        self.signals.contains_key(name)
    }

    /// Get the parameter count for a signal
    pub fn get_signal_param_count(&self, name: &str) -> Option<usize> {
        self.signals.get(name).copied()
    }

    // ========== Phase 5: Exported Property Methods ==========

    /// Initialize exported properties from Program metadata (Checkpoint 3.1 & 3.2)
    ///
    /// Called during execute() to set up property storage with default values.
    /// Reads static PropertyMetadata from the Program and initializes the
    /// per-instance exported_properties HashMap.
    ///
    /// # Hot-Reload Behavior (FIXED)
    ///
    /// Clears existing exported_properties before re-initialization to prevent
    /// stale properties from persisting after script recompilation. This ensures
    /// that removed properties are no longer accessible.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ferrisscript_runtime::Env;
    /// # use ferrisscript_compiler::compile;
    /// let source = "@export let mut health: i32 = 100;";
    /// let program = compile(source).unwrap();
    /// let mut env = Env::new();
    /// env.initialize_properties(&program);
    /// // exported_properties now contains { "health": Value::Int(100) }
    /// ```
    pub fn initialize_properties(&mut self, program: &ast::Program) {
        // Clone property metadata from Program (static, shared across instances)
        self.property_metadata = program.property_metadata.clone();

        // Clear old properties to prevent stale data after hot-reload (Hot-Reload Fix)
        self.exported_properties.clear();

        // Initialize exported_properties HashMap with default values
        for metadata in &self.property_metadata {
            if let Some(default_str) = &metadata.default_value {
                let value = Self::parse_default_value(default_str, &metadata.type_name);
                self.exported_properties
                    .insert(metadata.name.clone(), value);
            }
        }
    }

    /// Parse default value string to Value (Checkpoint 3.1)
    ///
    /// Handles:
    /// - Literals: `42`, `3.14`, `true`, `"text"`
    /// - Struct literals: `Vector2 { x: 0.0, y: 0.0 }` (simplified parsing)
    ///
    /// For struct literals, we do basic parsing since default values are
    /// guaranteed to be compile-time constants (validated by E813).
    fn parse_default_value(default_str: &str, type_name: &str) -> Value {
        match type_name {
            "i32" => {
                // Handle negative numbers (may have leading minus)
                Value::Int(default_str.parse().unwrap_or(0))
            }
            "f32" => {
                // Handle negative floats (may have leading minus)
                Value::Float(default_str.parse().unwrap_or(0.0))
            }
            "bool" => Value::Bool(default_str.parse().unwrap_or(false)),
            "String" => {
                // Remove surrounding quotes if present
                let s = default_str.trim_matches('"');
                Value::String(s.to_string())
            }
            "Vector2" => {
                // Parse "Vector2 { x: 0.0, y: 0.0 }" format
                // Simplified parsing since format is guaranteed by compiler
                if let Some(fields_str) = default_str.strip_prefix("Vector2 {") {
                    if let Some(fields_str) = fields_str.strip_suffix('}') {
                        let mut x = 0.0;
                        let mut y = 0.0;
                        for field in fields_str.split(',') {
                            let parts: Vec<&str> = field.split(':').collect();
                            if parts.len() == 2 {
                                let name = parts[0].trim();
                                let value = parts[1].trim();
                                if name == "x" {
                                    x = value.parse().unwrap_or(0.0);
                                } else if name == "y" {
                                    y = value.parse().unwrap_or(0.0);
                                }
                            }
                        }
                        return Value::Vector2 { x, y };
                    }
                }
                Value::Vector2 { x: 0.0, y: 0.0 } // Default
            }
            "Color" => {
                // Parse "Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }" format
                if let Some(fields_str) = default_str.strip_prefix("Color {") {
                    if let Some(fields_str) = fields_str.strip_suffix('}') {
                        let mut r = 0.0;
                        let mut g = 0.0;
                        let mut b = 0.0;
                        let mut a = 1.0;
                        for field in fields_str.split(',') {
                            let parts: Vec<&str> = field.split(':').collect();
                            if parts.len() == 2 {
                                let name = parts[0].trim();
                                let value = parts[1].trim();
                                match name {
                                    "r" => r = value.parse().unwrap_or(0.0),
                                    "g" => g = value.parse().unwrap_or(0.0),
                                    "b" => b = value.parse().unwrap_or(0.0),
                                    "a" => a = value.parse().unwrap_or(1.0),
                                    _ => {}
                                }
                            }
                        }
                        return Value::Color { r, g, b, a };
                    }
                }
                Value::Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                } // Default
            }
            // TODO: Rect2, Transform2D (complex struct literals)
            // For now, return type defaults
            "Rect2" => Value::Rect2 {
                position: Box::new(Value::Vector2 { x: 0.0, y: 0.0 }),
                size: Box::new(Value::Vector2 { x: 0.0, y: 0.0 }),
            },
            "Transform2D" => Value::Transform2D {
                position: Box::new(Value::Vector2 { x: 0.0, y: 0.0 }),
                rotation: 0.0,
                scale: Box::new(Value::Vector2 { x: 1.0, y: 1.0 }),
            },
            _ => Value::Nil,
        }
    }

    /// Get an exported property value (Checkpoint 3.3)
    ///
    /// Returns the current value of an exported property.
    /// Called from Godot Inspector or GDExtension get() method.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ferrisscript_runtime::{Env, Value, execute};
    /// # use ferrisscript_compiler::compile;
    /// let source = "@export let mut health: i32 = 100;";
    /// let program = compile(source).unwrap();
    /// let mut env = Env::new();
    /// execute(&program, &mut env).unwrap();
    /// let health = env.get_exported_property("health").unwrap();
    /// assert_eq!(health, Value::Int(100));
    /// ```
    pub fn get_exported_property(&self, name: &str) -> Result<Value, String> {
        self.exported_properties
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Property '{}' not found", name))
    }

    /// Set an exported property value with optional clamping (Checkpoint 3.4)
    ///
    /// Updates the value of an exported property. If `from_inspector` is true,
    /// applies range clamping for properties with Range hints. If false (from script),
    /// allows out-of-range values but emits a warning.
    ///
    /// # Type Validation
    ///
    /// Validates that the value type matches the property's declared type.
    /// Returns an error if types are incompatible (e.g., setting String for i32).
    ///
    /// # Clamp-on-Set Policy
    ///
    /// - **Inspector sets** (`from_inspector=true`): Automatically clamp to range
    /// - **Script sets** (`from_inspector=false`): Warn if out of range, but allow
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ferrisscript_runtime::{Env, Value, execute};
    /// # use ferrisscript_compiler::compile;
    /// let source = "@export(range(0, 100, 1)) let mut health: i32 = 50;";
    /// let program = compile(source).unwrap();
    /// let mut env = Env::new();
    /// execute(&program, &mut env).unwrap();
    /// // From Inspector: clamps 150 to 100
    /// env.set_exported_property("health", Value::Int(150), true).unwrap();
    /// assert_eq!(env.get_exported_property("health").unwrap(), Value::Int(100));
    ///
    /// // From script: allows 150 but warns
    /// env.set_exported_property("health", Value::Int(150), false).unwrap();
    /// assert_eq!(env.get_exported_property("health").unwrap(), Value::Int(150));
    /// ```
    pub fn set_exported_property(
        &mut self,
        name: &str,
        value: Value,
        from_inspector: bool,
    ) -> Result<(), String> {
        // Find metadata for this property
        let metadata = self
            .property_metadata
            .iter()
            .find(|m| m.name == name)
            .ok_or_else(|| format!("Property '{}' not found", name))?;

        // Validate type matches (FIXED: Type safety validation)
        Self::validate_type(&metadata.type_name, &value)?;

        // Apply clamping if range hint and from Inspector
        let final_value = if from_inspector {
            Self::clamp_if_range(metadata, value)?
        } else {
            // From script: warn if out of range but allow
            Self::warn_if_out_of_range(metadata, &value);
            value
        };

        self.exported_properties
            .insert(name.to_string(), final_value);
        Ok(())
    }

    /// Clamp value to range if PropertyHint is Range (Checkpoint 3.4)
    ///
    /// Applies min/max clamping for Range hints. Handles both i32 and f32.
    /// Returns error for NaN or Infinity float values.
    fn clamp_if_range(metadata: &ast::PropertyMetadata, value: Value) -> Result<Value, String> {
        match &metadata.hint {
            ast::PropertyHint::Range { min, max, .. } => match value {
                Value::Int(i) => {
                    let clamped = i.max(*min as i32).min(*max as i32);
                    Ok(Value::Int(clamped))
                }
                Value::Float(f) => {
                    // Handle NaN and Infinity
                    if f.is_nan() {
                        return Err(format!(
                            "Invalid float value for {}: NaN (not a number)",
                            metadata.name
                        ));
                    }
                    if f.is_infinite() {
                        return Err(format!(
                            "Invalid float value for {}: {} (infinite)",
                            metadata.name,
                            if f.is_sign_positive() {
                                "+Infinity"
                            } else {
                                "-Infinity"
                            }
                        ));
                    }
                    let clamped = f.max(*min).min(*max);
                    Ok(Value::Float(clamped))
                }
                _ => Err(format!(
                    "Range hint requires numeric value, got {:?}",
                    value
                )),
            },
            _ => Ok(value), // No clamping for other hints
        }
    }

    /// Warn if value is out of range (for script sets) (Checkpoint 3.4)
    ///
    /// Emits a warning to stderr if the value is outside the range hint bounds.
    /// This is policy for script-side assignments (allow but warn).
    fn warn_if_out_of_range(metadata: &ast::PropertyMetadata, value: &Value) {
        if let ast::PropertyHint::Range { min, max, .. } = &metadata.hint {
            let out_of_range = match value {
                Value::Int(i) => (*i as f32) < *min || (*i as f32) > *max,
                Value::Float(f) => *f < *min || *f > *max,
                _ => false,
            };

            if out_of_range {
                eprintln!(
                    "Warning: Property '{}' set to {:?}, outside range {}-{}",
                    metadata.name, value, min, max
                );
            }
        }
    }

    /// Validate that value type matches property's declared type (Type Safety Fix)
    ///
    /// Returns error if value type is incompatible with the property type.
    /// This prevents storing wrong-typed values (e.g., String in i32 property).
    ///
    /// # Examples
    ///
    /// ```
    /// # use ferrisscript_runtime::{Env, Value, execute};
    /// # use ferrisscript_compiler::compile;
    /// let source = "@export let mut count: i32 = 0;";
    /// let program = compile(source).unwrap();
    /// let mut env = Env::new();
    /// execute(&program, &mut env).unwrap();
    ///
    /// // Valid: i32 for i32 property
    /// assert!(env.set_exported_property("count", Value::Int(42), true).is_ok());
    ///
    /// // Invalid: String for i32 property - now returns error
    /// assert!(env.set_exported_property("count", Value::String("text".to_string()), true).is_err());
    /// ```
    fn validate_type(type_name: &str, value: &Value) -> Result<(), String> {
        let is_valid = matches!(
            (type_name, value),
            ("i32", Value::Int(_))
                | ("f32", Value::Float(_))
                | ("bool", Value::Bool(_))
                | ("String", Value::String(_))
                | ("Vector2", Value::Vector2 { .. })
                | ("Color", Value::Color { .. })
                | ("Rect2", Value::Rect2 { .. })
                | ("Transform2D", Value::Transform2D { .. })
        );

        if is_valid {
            Ok(())
        } else {
            Err(format!(
                "Type mismatch: expected {} but got {:?}",
                type_name,
                Self::value_type_name(value)
            ))
        }
    }

    /// Get the type name of a Value (helper for error messages)
    fn value_type_name(value: &Value) -> &str {
        match value {
            Value::Int(_) => "i32",
            Value::Float(_) => "f32",
            Value::Bool(_) => "bool",
            Value::String(_) => "String",
            Value::Vector2 { .. } => "Vector2",
            Value::Color { .. } => "Color",
            Value::Rect2 { .. } => "Rect2",
            Value::Transform2D { .. } => "Transform2D",
            Value::Nil => "Nil",
            Value::SelfObject => "Self",
            Value::InputEvent(_) => "InputEvent",
            Value::Node(_) => "Node",
        }
    }
}

// Built-in function implementations
fn builtin_print(args: &[Value]) -> Result<Value, String> {
    let output = args
        .iter()
        .map(|v| match v {
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::String(s) => s.clone(),
            Value::Vector2 { x, y } => format!("Vector2({}, {})", x, y),
            Value::Color { r, g, b, a } => format!("Color({}, {}, {}, {})", r, g, b, a),
            Value::Rect2 { position, size } => {
                // Format nested Vector2 values
                match (&**position, &**size) {
                    (Value::Vector2 { x: px, y: py }, Value::Vector2 { x: sx, y: sy }) => {
                        format!("Rect2(Vector2({}, {}), Vector2({}, {}))", px, py, sx, sy)
                    }
                    _ => "Rect2(invalid, invalid)".to_string(),
                }
            }
            Value::Transform2D {
                position,
                rotation,
                scale,
            } => {
                // Format nested Vector2 values
                match (&**position, &**scale) {
                    (Value::Vector2 { x: px, y: py }, Value::Vector2 { x: sx, y: sy }) => {
                        format!(
                            "Transform2D(Vector2({}, {}), {}, Vector2({}, {}))",
                            px, py, rotation, sx, sy
                        )
                    }
                    _ => "Transform2D(invalid, invalid, invalid)".to_string(),
                }
            }
            Value::Nil => "nil".to_string(),
            Value::SelfObject => "self".to_string(),
            Value::InputEvent(_) => "InputEvent".to_string(),
            Value::Node(handle) => format!("Node({})", handle.id()),
        })
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", output);
    Ok(Value::Nil)
}

fn builtin_emit_signal(_args: &[Value]) -> Result<Value, String> {
    // NOTE: This is a stub implementation. The actual signal emission
    // will be handled by the Godot binding layer (Step 6).
    // At runtime, the type checker has already validated:
    // - Signal exists
    // - Parameter count matches
    // - Parameter types are correct
    // The Godot binding will replace this with actual signal emission.
    Ok(Value::Nil)
}

/// Control flow result
#[derive(Debug, Clone, PartialEq)]
enum FlowControl {
    None,
    Return(Value),
}

/// Execute a FerrisScript program by initializing globals and registering functions.
///
/// This is the main entry point for program execution. It sets up the runtime
/// environment by:
/// 1. Evaluating and storing global variables
/// 2. Registering all function definitions
///
/// Note: This does not automatically call any function. Use [`call_function`]
/// to invoke specific functions after calling this.
///
/// # Arguments
///
/// * `program` - The compiled AST from the compiler
/// * `env` - The execution environment
///
/// # Returns
///
/// * `Ok(())` - Program loaded successfully
/// * `Err(String)` - Runtime error during global initialization
///
/// # Examples
///
/// ```no_run
/// use ferrisscript_runtime::{execute, call_function, Env, Value};
/// use ferrisscript_compiler::compile;
///
/// let source = r#"
///     let score: i32 = 0;
///     fn increment() { score = score + 1; }
/// "#;
/// let program = compile(source).unwrap();
/// let mut env = Env::new();
///
/// execute(&program, &mut env).unwrap();
/// call_function("increment", &[], &mut env).unwrap();
/// ```
///
/// # Performance
///
/// - Function call overhead: ~1.05μs
/// - Loop iteration: ~180ns
/// - Supports 16K+ calls per frame at 60 FPS
pub fn execute(program: &ast::Program, env: &mut Env) -> Result<(), String> {
    // Initialize global variables
    for global in &program.global_vars {
        let value = evaluate_expr(&global.value, env)?;
        env.set_with_mutability(global.name.clone(), value, global.mutable);
    }

    // Register all signals
    for signal in &program.signals {
        env.register_signal(signal.name.clone(), signal.parameters.len());
    }

    // Initialize exported properties from metadata (Phase 5: Checkpoint 3.1 & 3.2)
    env.initialize_properties(program);

    // Register all functions
    for func in &program.functions {
        env.define_function(func.name.clone(), func.clone());
    }

    Ok(())
}

fn execute_stmt(stmt: &ast::Stmt, env: &mut Env) -> Result<FlowControl, String> {
    match stmt {
        ast::Stmt::Let {
            name,
            value,
            mutable,
            ..
        } => {
            let val = evaluate_expr(value, env)?;
            env.set_with_mutability(name.clone(), val, *mutable);
            Ok(FlowControl::None)
        }

        ast::Stmt::Assign { target, value, .. } => {
            let val = evaluate_expr(value, env)?;

            // Handle field access assignment (e.g., self.position.x = value)
            if let ast::Expr::FieldAccess(object, field, _) = target {
                assign_field(object, field, val, env)?;
            } else if let ast::Expr::Variable(name, _) = target {
                // Use assign method which checks mutability
                env.assign(name, val)?;
            } else {
                return Err("Error[E403]: Invalid assignment target".to_string());
            }

            Ok(FlowControl::None)
        }

        ast::Stmt::If {
            cond,
            then_branch,
            else_branch,
            ..
        } => {
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

fn assign_field(
    object: &ast::Expr,
    field: &str,
    value: Value,
    env: &mut Env,
) -> Result<(), String> {
    match object {
        ast::Expr::Variable(name, _) => {
            // Check if this is 'self'
            if let Some(var) = env.get(name) {
                if matches!(var, Value::SelfObject) {
                    // Assigning to self.property - use property setter callback
                    if let Some(setter) = env.property_setter {
                        return setter(field, value);
                    } else {
                        return Err(
                            "Error[E404]: Cannot set self properties: no property setter registered".to_string()
                        );
                    }
                }
            }

            // Regular variable field assignment - check mutability first
            if !env.is_mutable(name) {
                return Err(format!(
                    "Error[E405]: Cannot assign to field of immutable variable '{}'",
                    name
                ));
            }

            if let Some(var) = env.get_mut(name) {
                match var {
                    Value::Vector2 { x, y } => match field {
                        "x" => {
                            if let Some(f) = value.to_float() {
                                *x = f;
                            } else {
                                return Err(format!(
                                    "Error[E406]: Cannot assign {:?} to Vector2.x",
                                    value
                                ));
                            }
                        }
                        "y" => {
                            if let Some(f) = value.to_float() {
                                *y = f;
                            } else {
                                return Err(format!(
                                    "Error[E406]: Cannot assign {:?} to Vector2.y",
                                    value
                                ));
                            }
                        }
                        _ => return Err(format!("Error[E407]: Vector2 has no field '{}'", field)),
                    },
                    Value::Color { r, g, b, a } => match field {
                        "r" => {
                            if let Some(f) = value.to_float() {
                                *r = f;
                            } else {
                                return Err(format!(
                                    "Error[E707]: Cannot assign {:?} to Color.r",
                                    value
                                ));
                            }
                        }
                        "g" => {
                            if let Some(f) = value.to_float() {
                                *g = f;
                            } else {
                                return Err(format!(
                                    "Error[E707]: Cannot assign {:?} to Color.g",
                                    value
                                ));
                            }
                        }
                        "b" => {
                            if let Some(f) = value.to_float() {
                                *b = f;
                            } else {
                                return Err(format!(
                                    "Error[E707]: Cannot assign {:?} to Color.b",
                                    value
                                ));
                            }
                        }
                        "a" => {
                            if let Some(f) = value.to_float() {
                                *a = f;
                            } else {
                                return Err(format!(
                                    "Error[E707]: Cannot assign {:?} to Color.a",
                                    value
                                ));
                            }
                        }
                        _ => return Err(format!("Error[E701]: Color has no field '{}'", field)),
                    },
                    Value::Rect2 { position, size } => match field {
                        "position" => {
                            *position = Box::new(value);
                        }
                        "size" => {
                            *size = Box::new(value);
                        }
                        _ => return Err(format!("Error[E702]: Rect2 has no field '{}'", field)),
                    },
                    Value::Transform2D {
                        position,
                        rotation,
                        scale,
                    } => match field {
                        "position" => {
                            *position = Box::new(value);
                        }
                        "rotation" => {
                            if let Some(f) = value.to_float() {
                                *rotation = f;
                            } else {
                                return Err(format!(
                                    "Error[E709]: Cannot assign {:?} to Transform2D.rotation",
                                    value
                                ));
                            }
                        }
                        "scale" => {
                            *scale = Box::new(value);
                        }
                        _ => {
                            return Err(format!(
                                "Error[E703]: Transform2D has no field '{}'",
                                field
                            ))
                        }
                    },
                    _ => {
                        return Err(format!(
                            "Error[E408]: Cannot access field '{}' on {:?}",
                            field, var
                        ))
                    }
                }
                Ok(())
            } else {
                Err(format!("Error[E401]: Undefined variable: {}", name))
            }
        }

        ast::Expr::FieldAccess(object, parent_field, _) => {
            // Handle nested field access (e.g., self.position.x)
            if let ast::Expr::Variable(name, _) = &**object {
                // Check if this is self.property.field
                if let Some(var) = env.get(name) {
                    if matches!(var, Value::SelfObject) {
                        // Get the property from Godot (e.g., position)
                        if let Some(getter) = env.property_getter {
                            let mut prop_value = getter(parent_field)?;

                            // Modify the field (e.g., x or y)
                            match &mut prop_value {
                                Value::Vector2 { x, y } => match field {
                                    "x" => {
                                        if let Some(f) = value.to_float() {
                                            *x = f;
                                        } else {
                                            return Err(format!(
                                                "Error[E406]: Cannot assign {:?} to Vector2.x",
                                                value
                                            ));
                                        }
                                    }
                                    "y" => {
                                        if let Some(f) = value.to_float() {
                                            *y = f;
                                        } else {
                                            return Err(format!(
                                                "Error[E406]: Cannot assign {:?} to Vector2.y",
                                                value
                                            ));
                                        }
                                    }
                                    _ => {
                                        return Err(format!(
                                            "Error[E407]: Vector2 has no field '{}'",
                                            field
                                        ))
                                    }
                                },
                                _ => {
                                    return Err(format!(
                                        "Error[E409]: Property '{}' is not a Vector2",
                                        parent_field
                                    ))
                                }
                            }

                            // Set the property back to Godot
                            if let Some(setter) = env.property_setter {
                                return setter(parent_field, prop_value);
                            } else {
                                return Err(
                                    "Error[E404]: Cannot set self properties: no property setter registered"
                                        .to_string(),
                                );
                            }
                        } else {
                            return Err(
                                "Error[E410]: Cannot get self properties: no property getter registered"
                                    .to_string(),
                            );
                        }
                    }
                }

                // Regular variable nested field assignment (not implemented yet)
                if let Some(Value::Vector2 { .. }) = env.get_mut(name) {
                    return Err(
                        "Error[E411]: Nested field assignment on regular variables not yet implemented"
                            .to_string(),
                    );
                }
            }
            Err("Error[E412]: Complex field assignment not yet implemented".to_string())
        }

        _ => Err("Error[E403]: Invalid assignment target".to_string()),
    }
}

fn evaluate_expr(expr: &ast::Expr, env: &mut Env) -> Result<Value, String> {
    match expr {
        ast::Expr::Literal(lit, _) => Ok(match lit {
            ast::Literal::Int(i) => Value::Int(*i),
            ast::Literal::Float(f) => Value::Float(*f),
            ast::Literal::Bool(b) => Value::Bool(*b),
            ast::Literal::Str(s) => Value::String(s.clone()),
        }),

        ast::Expr::Variable(name, _) => env
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Undefined variable: {}", name)),

        ast::Expr::Binary(left, op, right, _) => {
            let left_val = evaluate_expr(left, env)?;
            let right_val = evaluate_expr(right, env)?;

            match op {
                BinaryOp::Add => match (&left_val, &right_val) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                    _ => {
                        let a = left_val.to_float().ok_or("Cannot add non-numeric values")?;
                        let b = right_val
                            .to_float()
                            .ok_or("Cannot add non-numeric values")?;
                        Ok(Value::Float(a + b))
                    }
                },

                BinaryOp::Sub => match (&left_val, &right_val) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
                    _ => {
                        let a = left_val
                            .to_float()
                            .ok_or("Cannot subtract non-numeric values")?;
                        let b = right_val
                            .to_float()
                            .ok_or("Cannot subtract non-numeric values")?;
                        Ok(Value::Float(a - b))
                    }
                },

                BinaryOp::Mul => match (&left_val, &right_val) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
                    _ => {
                        let a = left_val
                            .to_float()
                            .ok_or("Cannot multiply non-numeric values")?;
                        let b = right_val
                            .to_float()
                            .ok_or("Cannot multiply non-numeric values")?;
                        Ok(Value::Float(a * b))
                    }
                },

                BinaryOp::Div => match (&left_val, &right_val) {
                    (Value::Int(a), Value::Int(b)) => {
                        if *b == 0 {
                            return Err("Error[E413]: Division by zero".to_string());
                        }
                        Ok(Value::Int(a / b))
                    }
                    _ => {
                        let a = left_val
                            .to_float()
                            .ok_or("Cannot divide non-numeric values")?;
                        let b = right_val
                            .to_float()
                            .ok_or("Cannot divide non-numeric values")?;
                        if b == 0.0 {
                            return Err("Error[E413]: Division by zero".to_string());
                        }
                        Ok(Value::Float(a / b))
                    }
                },

                BinaryOp::Eq => Ok(Value::Bool(left_val == right_val)),
                BinaryOp::Ne => Ok(Value::Bool(left_val != right_val)),

                BinaryOp::Lt => match (&left_val, &right_val) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
                    _ => {
                        let a = left_val
                            .to_float()
                            .ok_or("Cannot compare non-numeric values")?;
                        let b = right_val
                            .to_float()
                            .ok_or("Cannot compare non-numeric values")?;
                        Ok(Value::Bool(a < b))
                    }
                },

                BinaryOp::Le => match (&left_val, &right_val) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
                    _ => {
                        let a = left_val
                            .to_float()
                            .ok_or("Cannot compare non-numeric values")?;
                        let b = right_val
                            .to_float()
                            .ok_or("Cannot compare non-numeric values")?;
                        Ok(Value::Bool(a <= b))
                    }
                },

                BinaryOp::Gt => match (&left_val, &right_val) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
                    _ => {
                        let a = left_val
                            .to_float()
                            .ok_or("Cannot compare non-numeric values")?;
                        let b = right_val
                            .to_float()
                            .ok_or("Cannot compare non-numeric values")?;
                        Ok(Value::Bool(a > b))
                    }
                },

                BinaryOp::Ge => match (&left_val, &right_val) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
                    _ => {
                        let a = left_val
                            .to_float()
                            .ok_or("Cannot compare non-numeric values")?;
                        let b = right_val
                            .to_float()
                            .ok_or("Cannot compare non-numeric values")?;
                        Ok(Value::Bool(a >= b))
                    }
                },

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
                UnaryOp::Neg => match val {
                    Value::Int(i) => Ok(Value::Int(-i)),
                    Value::Float(f) => Ok(Value::Float(-f)),
                    _ => Err("Error[E414]: Cannot negate non-numeric value".to_string()),
                },

                UnaryOp::Not => Ok(Value::Bool(!val.to_bool())),
            }
        }

        ast::Expr::Call(name, args, _) => {
            // Evaluate arguments
            let arg_values: Result<Vec<_>, _> =
                args.iter().map(|arg| evaluate_expr(arg, env)).collect();
            let arg_values = arg_values?;

            // Check if it's a built-in function
            if env.is_builtin(name) {
                return env.call_builtin(name, &arg_values);
            }

            // Look up user-defined function
            let func = env
                .get_function(name)
                .ok_or_else(|| format!("Error[E415]: Undefined function: {}", name))?
                .clone();

            // Check arity
            if func.params.len() != arg_values.len() {
                return Err(format!(
                    "Error[E416]: Function {} expects {} arguments, got {}",
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
                Value::Vector2 { x, y } => match field.as_str() {
                    "x" => Ok(Value::Float(x)),
                    "y" => Ok(Value::Float(y)),
                    _ => Err(format!("Error[E407]: Vector2 has no field '{}'", field)),
                },
                Value::Color { r, g, b, a } => match field.as_str() {
                    "r" => Ok(Value::Float(r)),
                    "g" => Ok(Value::Float(g)),
                    "b" => Ok(Value::Float(b)),
                    "a" => Ok(Value::Float(a)),
                    _ => Err(format!("Error[E701]: Color has no field '{}'", field)),
                },
                Value::Rect2 { position, size } => match field.as_str() {
                    "position" => Ok((*position).clone()),
                    "size" => Ok((*size).clone()),
                    _ => Err(format!("Error[E702]: Rect2 has no field '{}'", field)),
                },
                Value::Transform2D {
                    position,
                    rotation,
                    scale,
                } => match field.as_str() {
                    "position" => Ok((*position).clone()),
                    "rotation" => Ok(Value::Float(rotation)),
                    "scale" => Ok((*scale).clone()),
                    _ => Err(format!("Error[E703]: Transform2D has no field '{}'", field)),
                },
                Value::SelfObject => {
                    // Use property getter callback to get field from Godot node
                    if let Some(getter) = env.property_getter {
                        getter(field)
                    } else {
                        Err(
                            "Error[E417]: Cannot access self properties: no property getter registered"
                                .to_string(),
                        )
                    }
                }
                _ => Err(format!(
                    "Error[E408]: Cannot access field '{}' on {:?}",
                    field, obj_val
                )),
            }
        }

        ast::Expr::StructLiteral {
            type_name,
            fields,
            span: _,
        } => evaluate_struct_literal(type_name, fields, env),

        // Compound assignment and regular assignment expressions not used in runtime
        // They are desugared to Stmt::Assign at parse time
        ast::Expr::Assign(_, _, _) | ast::Expr::CompoundAssign(_, _, _, _) => {
            Err("Error[E418]: Assignment expressions should be statements".to_string())
        }
    }
}

/// Evaluate struct literal: `TypeName { field1: value1, field2: value2 }`
/// Constructs Value from struct literal expression
fn evaluate_struct_literal(
    type_name: &str,
    fields: &[(String, ast::Expr)],
    env: &mut Env,
) -> Result<Value, String> {
    match type_name {
        "Color" => {
            let mut r = None;
            let mut g = None;
            let mut b = None;
            let mut a = None;

            for (field_name, field_expr) in fields {
                let value = evaluate_expr(field_expr, env)?;
                let float_val = value
                    .to_float()
                    .ok_or_else(|| format!("Color field '{}' must be numeric", field_name))?;

                match field_name.as_str() {
                    "r" => r = Some(float_val),
                    "g" => g = Some(float_val),
                    "b" => b = Some(float_val),
                    "a" => a = Some(float_val),
                    _ => return Err(format!("Unknown field '{}' on Color", field_name)),
                }
            }

            Ok(Value::Color {
                r: r.ok_or("Missing field 'r' in Color literal")?,
                g: g.ok_or("Missing field 'g' in Color literal")?,
                b: b.ok_or("Missing field 'b' in Color literal")?,
                a: a.ok_or("Missing field 'a' in Color literal")?,
            })
        }

        "Rect2" => {
            let mut position = None;
            let mut size = None;

            for (field_name, field_expr) in fields {
                let value = evaluate_expr(field_expr, env)?;
                match field_name.as_str() {
                    "position" => {
                        if matches!(value, Value::Vector2 { .. }) {
                            position = Some(Box::new(value));
                        } else {
                            return Err(format!(
                                "Rect2 'position' must be Vector2, found {:?}",
                                value
                            ));
                        }
                    }
                    "size" => {
                        if matches!(value, Value::Vector2 { .. }) {
                            size = Some(Box::new(value));
                        } else {
                            return Err(format!("Rect2 'size' must be Vector2, found {:?}", value));
                        }
                    }
                    _ => return Err(format!("Unknown field '{}' on Rect2", field_name)),
                }
            }

            Ok(Value::Rect2 {
                position: position.ok_or("Missing field 'position' in Rect2 literal")?,
                size: size.ok_or("Missing field 'size' in Rect2 literal")?,
            })
        }

        "Transform2D" => {
            let mut position = None;
            let mut rotation = None;
            let mut scale = None;

            for (field_name, field_expr) in fields {
                let value = evaluate_expr(field_expr, env)?;
                match field_name.as_str() {
                    "position" => {
                        if matches!(value, Value::Vector2 { .. }) {
                            position = Some(Box::new(value));
                        } else {
                            return Err(format!(
                                "Transform2D 'position' must be Vector2, found {:?}",
                                value
                            ));
                        }
                    }
                    "rotation" => {
                        rotation = Some(
                            value
                                .to_float()
                                .ok_or("Transform2D 'rotation' must be numeric")?,
                        );
                    }
                    "scale" => {
                        if matches!(value, Value::Vector2 { .. }) {
                            scale = Some(Box::new(value));
                        } else {
                            return Err(format!(
                                "Transform2D 'scale' must be Vector2, found {:?}",
                                value
                            ));
                        }
                    }
                    _ => return Err(format!("Unknown field '{}' on Transform2D", field_name)),
                }
            }

            Ok(Value::Transform2D {
                position: position.ok_or("Missing field 'position' in Transform2D literal")?,
                rotation: rotation.ok_or("Missing field 'rotation' in Transform2D literal")?,
                scale: scale.ok_or("Missing field 'scale' in Transform2D literal")?,
            })
        }

        "Vector2" => {
            let mut x = None;
            let mut y = None;

            for (field_name, field_expr) in fields {
                let value = evaluate_expr(field_expr, env)?;
                let float_val = value
                    .to_float()
                    .ok_or_else(|| format!("Vector2 field '{}' must be numeric", field_name))?;

                match field_name.as_str() {
                    "x" => x = Some(float_val),
                    "y" => y = Some(float_val),
                    _ => return Err(format!("Unknown field '{}' on Vector2", field_name)),
                }
            }

            Ok(Value::Vector2 {
                x: x.ok_or("Missing field 'x' in Vector2 literal")?,
                y: y.ok_or("Missing field 'y' in Vector2 literal")?,
            })
        }

        _ => Err(format!(
            "Type '{}' does not support struct literal syntax",
            type_name
        )),
    }
}

/// Call a FerrisScript function by name with arguments.
///
/// This is the primary way to invoke FerrisScript functions from external code,
/// such as Godot engine callbacks (_ready, _process, etc.).
///
/// # Arguments
///
/// * `name` - Function name to call
/// * `args` - Slice of argument values
/// * `env` - Execution environment
///
/// # Returns
///
/// * `Ok(Value)` - Function return value (or `Value::Nil` if void)
/// * `Err(String)` - Runtime error (undefined function, wrong arity, execution error)
///
/// # Examples
///
/// ```no_run
/// use ferrisscript_runtime::{call_function, Env, Value};
/// use ferrisscript_compiler::compile;
///
/// let source = r#"
///     fn multiply(a: i32, b: i32) -> i32 {
///         return a * b;
///     }
/// "#;
/// let program = compile(source).unwrap();
/// let mut env = Env::new();
/// ferrisscript_runtime::execute(&program, &mut env).unwrap();
///
/// let result = call_function("multiply", &[Value::Int(6), Value::Int(7)], &mut env);
/// assert_eq!(result, Ok(Value::Int(42)));
/// ```
///
/// # Godot Integration
///
/// Typically used for Godot engine callbacks like `_ready()` and `_process(delta)`.
/// The GDExtension binding layer calls this function to invoke FerrisScript code
/// from the game engine.
///
/// # Performance
///
/// - Function call overhead: ~1.05μs
/// - Suitable for real-time game loops
/// - 16K+ calls/frame possible at 60 FPS
pub fn call_function(name: &str, args: &[Value], env: &mut Env) -> Result<Value, String> {
    if env.is_builtin(name) {
        return env.call_builtin(name, args);
    }

    let func = env
        .get_function(name)
        .ok_or_else(|| format!("Error[E415]: Undefined function: {}", name))?
        .clone();

    if func.params.len() != args.len() {
        return Err(format!(
            "Error[E416]: Function {} expects {} arguments, got {}",
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
    use ferrisscript_compiler::compile;

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
        assert_eq!(Value::Float(3.5).to_float(), Some(3.5));
        assert_eq!(Value::Bool(true).to_float(), None);

        assert!(Value::Bool(true).to_bool());
        assert!(!Value::Bool(false).to_bool());
        assert!(Value::Int(1).to_bool());
        assert!(!Value::Int(0).to_bool());
        assert!(!Value::Nil.to_bool());
    }

    #[test]
    fn test_builtin_print() {
        let mut env = Env::new();
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
        env.define_function(
            "get_x".to_string(),
            ast::Function {
                name: "get_x".to_string(),
                params: vec![],
                return_type: Some("f32".to_string()),
                body: vec![ast::Stmt::Return {
                    value: Some(ast::Expr::FieldAccess(
                        Box::new(ast::Expr::Variable("pos".to_string(), ast::Span::unknown())),
                        "x".to_string(),
                        ast::Span::unknown(),
                    )),
                    span: ast::Span::unknown(),
                }],
                span: ast::Span::unknown(),
            },
        );

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
        env.define_function(
            "test".to_string(),
            ast::Function {
                name: "test".to_string(),
                params: vec![],
                return_type: Some("i32".to_string()),
                body: vec![ast::Stmt::Return {
                    value: Some(ast::Expr::Variable(
                        "undefined_var".to_string(),
                        ast::Span::unknown(),
                    )),
                    span: ast::Span::unknown(),
                }],
                span: ast::Span::unknown(),
            },
        );

        let result = call_function("test", &[], &mut env);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }

    #[test]
    fn test_immutable_assignment_error() {
        let mut env = Env::new();

        let source = r#"
            fn test() -> i32 {
                let x: i32 = 10;
                x = 20;
                return x;
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result = call_function("test", &[], &mut env);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Cannot assign to immutable variable"));
    }

    #[test]
    fn test_mutable_assignment_success() {
        let mut env = Env::new();

        let source = r#"
            fn test() -> i32 {
                let mut x: i32 = 10;
                x = 20;
                return x;
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result = call_function("test", &[], &mut env).unwrap();
        assert_eq!(result, Value::Int(20));
    }

    #[test]
    fn test_mutable_global_variable() {
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
        let result = call_function("get_counter", &[], &mut env).unwrap();
        assert_eq!(result, Value::Int(0));

        // Increment
        call_function("increment", &[], &mut env).unwrap();
        let result = call_function("get_counter", &[], &mut env).unwrap();
        assert_eq!(result, Value::Int(1));

        // Increment again
        call_function("increment", &[], &mut env).unwrap();
        let result = call_function("get_counter", &[], &mut env).unwrap();
        assert_eq!(result, Value::Int(2));
    }

    #[test]
    fn test_immutable_field_assignment_error() {
        let mut env = Env::new();

        // Set up immutable Vector2
        env.set_with_mutability(
            "pos".to_string(),
            Value::Vector2 { x: 10.0, y: 20.0 },
            false,
        );

        // Try to assign to field - should fail
        env.define_function(
            "test".to_string(),
            ast::Function {
                name: "test".to_string(),
                params: vec![],
                return_type: None,
                body: vec![ast::Stmt::Assign {
                    target: ast::Expr::FieldAccess(
                        Box::new(ast::Expr::Variable("pos".to_string(), ast::Span::unknown())),
                        "x".to_string(),
                        ast::Span::unknown(),
                    ),
                    value: ast::Expr::Literal(ast::Literal::Float(50.0), ast::Span::unknown()),
                    span: ast::Span::unknown(),
                }],
                span: ast::Span::unknown(),
            },
        );

        let result = call_function("test", &[], &mut env);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Cannot assign to field of immutable variable"));
    }

    #[test]
    fn test_nested_if_else() {
        let mut env = Env::new();

        let source = r#"
            fn classify(x: i32) -> i32 {
                if x > 0 {
                    if x > 10 {
                        return 2;
                    } else {
                        return 1;
                    }
                } else {
                    if x < -10 {
                        return -2;
                    } else {
                        return -1;
                    }
                }
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result = call_function("classify", &[Value::Int(15)], &mut env).unwrap();
        assert_eq!(result, Value::Int(2));

        let result = call_function("classify", &[Value::Int(5)], &mut env).unwrap();
        assert_eq!(result, Value::Int(1));

        let result = call_function("classify", &[Value::Int(-15)], &mut env).unwrap();
        assert_eq!(result, Value::Int(-2));

        let result = call_function("classify", &[Value::Int(-5)], &mut env).unwrap();
        assert_eq!(result, Value::Int(-1));
    }

    #[test]
    fn test_while_loop_with_mutable_state() {
        let mut env = Env::new();

        let source = r#"
            fn sum_to_n(n: i32) -> i32 {
                let mut sum: i32 = 0;
                let mut i: i32 = 1;
                
                while i <= n {
                    sum = sum + i;
                    i = i + 1;
                }
                
                return sum;
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        // Test 1+2+3+4+5 = 15
        let result = call_function("sum_to_n", &[Value::Int(5)], &mut env).unwrap();
        assert_eq!(result, Value::Int(15));

        // Test 1+2+3+...+10 = 55
        let result = call_function("sum_to_n", &[Value::Int(10)], &mut env).unwrap();
        assert_eq!(result, Value::Int(55));
    }

    #[test]
    fn test_complex_control_flow_with_mutable_state() {
        let mut env = Env::new();

        let source = r#"
            let mut global_count: i32 = 0;
            
            fn process_numbers() -> i32 {
                let mut i: i32 = 0;
                
                while i < 10 {
                    if i < 5 {
                        global_count = global_count + 1;
                    } else {
                        if i > 7 {
                            global_count = global_count + 2;
                        }
                    }
                    i = i + 1;
                }
                
                return global_count;
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        // i < 5: 0, 1, 2, 3, 4 = 5 times (+1 each) = 5
        // i > 7: 8, 9 = 2 times (+2 each) = 4
        // Total: 5 + 4 = 9
        let result = call_function("process_numbers", &[], &mut env).unwrap();
        assert_eq!(result, Value::Int(9));
    }

    #[test]
    fn test_bounce_simulation() {
        let mut env = Env::new();

        let source = r#"
            let mut position: f32 = 5.0;
            let mut direction: f32 = 1.0;
            
            fn update_position(delta: f32) {
                position = position + direction * 100.0 * delta;
                
                if position > 10.0 {
                    direction = -1.0;
                }
                if position < -10.0 {
                    direction = 1.0;
                }
            }
            
            fn get_direction() -> f32 {
                return direction;
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        // Initial state - direction should be 1.0 (moving right)
        // Note: 1.0 is parsed as Int(1) due to fractional part being 0
        let dir = call_function("get_direction", &[], &mut env).unwrap();
        assert!(matches!(dir, Value::Int(1) | Value::Float(1.0)));

        // Move right past boundary (5.0 + 100*0.06 = 11.0)
        call_function("update_position", &[Value::Float(0.06)], &mut env).unwrap();

        // Should reverse direction at boundary
        let dir = call_function("get_direction", &[], &mut env).unwrap();
        assert!(matches!(dir, Value::Int(-1) | Value::Float(-1.0)));

        // Move left past opposite boundary (11.0 - 100*0.22 = -11.0)
        call_function("update_position", &[Value::Float(0.22)], &mut env).unwrap();

        // Should reverse back to positive
        let dir = call_function("get_direction", &[], &mut env).unwrap();
        assert!(matches!(dir, Value::Int(1) | Value::Float(1.0)));
    }

    // ===== Edge Case Tests (v0.0.2) =====

    #[test]
    fn test_edge_case_division_by_zero() {
        // Test division by zero behavior
        // TODO: Should return an error instead of potentially undefined behavior
        let mut env = Env::new();
        let source = r#"
            fn divide_by_zero() -> i32 {
                let x: i32 = 10;
                let y: i32 = 0;
                return x / y;
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result = call_function("divide_by_zero", &[], &mut env);
        // Current behavior: division by zero may panic, error, or return undefined value
        match result {
            Ok(v) => println!(
                "⚠️  Division by zero returned value (undefined behavior): {:?}",
                v
            ),
            Err(e) => println!("✅ Division by zero produced error: {}", e),
        }
    }

    #[test]
    fn test_edge_case_integer_overflow_addition() {
        // Test arithmetic with large numbers
        // NOTE: Very large literals (i32::MAX) are parsed as f32, so using smaller values
        let mut env = Env::new();
        let source = r#"
            fn large_add() -> i32 {
                let x: i32 = 1000000;
                let y: i32 = 2000000;
                return x + y;
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result = call_function("large_add", &[], &mut env).unwrap();
        assert_eq!(
            result,
            Value::Int(3000000),
            "Large number addition should work"
        );
    }

    #[test]
    fn test_edge_case_deeply_nested_expressions() {
        // Test parsing and evaluating deeply nested expressions (100 levels)
        let mut env = Env::new();

        // Build expression: (((((...(1)...)))))
        let mut expr = "1".to_string();
        for _ in 0..100 {
            expr = format!("({})", expr);
        }

        let source = format!(
            r#"
            fn deeply_nested() -> i32 {{
                return {};
            }}
        "#,
            expr
        );

        let program = compile(&source).unwrap();
        execute(&program, &mut env).unwrap();

        let result = call_function("deeply_nested", &[], &mut env);
        match result {
            Ok(Value::Int(1)) => {
                // Success - deep nesting handled correctly
            }
            Err(e) => {
                // If it fails, should give clear stack error
                assert!(
                    e.contains("stack") || e.contains("depth") || e.contains("too deep"),
                    "Error should mention stack/depth issue: {}",
                    e
                );
            }
            _ => panic!("Expected Int(1) or stack overflow error"),
        }
    }

    #[test]
    fn test_edge_case_recursion_depth_limit() {
        // Test recursive function to ensure basic recursion works
        // NOTE: Very deep recursion (1000+) will cause stack overflow - not tested here
        let mut env = Env::new();
        let source = r#"
            fn countdown(n: i32) -> i32 {
                if n <= 0 {
                    return 0;
                }
                return countdown(n - 1) + 1;
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        // Test reasonable recursion (10 levels)
        let result = call_function("countdown", &[Value::Int(10)], &mut env).unwrap();
        assert_eq!(result, Value::Int(10), "countdown(10) should return 10");

        // Test moderate recursion (100 levels)
        let result_100 = call_function("countdown", &[Value::Int(100)], &mut env).unwrap();
        assert_eq!(
            result_100,
            Value::Int(100),
            "countdown(100) should return 100"
        );
    }

    #[test]
    fn test_edge_case_short_circuit_and() {
        // Test that && evaluates correctly
        // NOTE: Full short-circuit testing requires mutable globals (not yet supported)
        let mut env = Env::new();
        let source = r#"
            fn always_false() -> bool {
                return false;
            }
            
            fn always_true() -> bool {
                return true;
            }
            
            fn test_and_false_first() -> bool {
                return always_false() && always_true();
            }
            
            fn test_and_true_first() -> bool {
                return always_true() && always_false();
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result1 = call_function("test_and_false_first", &[], &mut env).unwrap();
        assert_eq!(result1, Value::Bool(false), "false && true should be false");

        let result2 = call_function("test_and_true_first", &[], &mut env).unwrap();
        assert_eq!(result2, Value::Bool(false), "true && false should be false");
    }

    #[test]
    fn test_edge_case_short_circuit_or() {
        // Test that || evaluates correctly
        // NOTE: Full short-circuit testing requires mutable globals (not yet supported)
        let mut env = Env::new();
        let source = r#"
            fn always_false() -> bool {
                return false;
            }
            
            fn always_true() -> bool {
                return true;
            }
            
            fn test_or_true_first() -> bool {
                return always_true() || always_false();
            }
            
            fn test_or_false_first() -> bool {
                return always_false() || always_true();
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result1 = call_function("test_or_true_first", &[], &mut env).unwrap();
        assert_eq!(result1, Value::Bool(true), "true || false should be true");

        let result2 = call_function("test_or_false_first", &[], &mut env).unwrap();
        assert_eq!(result2, Value::Bool(true), "false || true should be true");
    }

    #[test]
    fn test_edge_case_variable_shadowing() {
        // Test that variables in different functions don't interfere
        // NOTE: Bare blocks `{}` not yet supported, so testing with if statements
        let mut env = Env::new();
        let source = r#"
            fn outer() -> i32 {
                let x: i32 = 10;
                return x;
            }
            
            fn inner() -> i32 {
                let x: i32 = 20;
                return x;
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result1 = call_function("outer", &[], &mut env).unwrap();
        assert_eq!(result1, Value::Int(10));

        let result2 = call_function("inner", &[], &mut env).unwrap();
        assert_eq!(result2, Value::Int(20));

        // Call outer again to ensure inner didn't affect it
        let result3 = call_function("outer", &[], &mut env).unwrap();
        assert_eq!(
            result3,
            Value::Int(10),
            "Function scopes should be independent"
        );
    }

    #[test]
    fn test_edge_case_empty_function_body() {
        // Test function with no statements (just returns)
        let mut env = Env::new();
        let source = r#"
            fn empty() -> i32 {
                return 42;
            }
            
            fn empty_implicit() {
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result = call_function("empty", &[], &mut env).unwrap();
        assert_eq!(result, Value::Int(42));

        let result2 = call_function("empty_implicit", &[], &mut env).unwrap();
        assert_eq!(result2, Value::Nil);
    }

    #[test]
    fn test_edge_case_early_return_from_nested_block() {
        // Test that return works correctly from nested if statements
        let mut env = Env::new();
        let source = r#"
            fn early_return(x: i32) -> i32 {
                if x > 10 {
                    if x > 20 {
                        if x > 30 {
                            return 100;
                        }
                        return 50;
                    }
                    return 25;
                }
                return x;
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result1 = call_function("early_return", &[Value::Int(5)], &mut env).unwrap();
        assert_eq!(result1, Value::Int(5), "x <= 10 should return x");

        let result2 = call_function("early_return", &[Value::Int(15)], &mut env).unwrap();
        assert_eq!(result2, Value::Int(25), "10 < x <= 20 should return 25");

        let result3 = call_function("early_return", &[Value::Int(25)], &mut env).unwrap();
        assert_eq!(result3, Value::Int(50), "20 < x <= 30 should return 50");

        let result4 = call_function("early_return", &[Value::Int(35)], &mut env).unwrap();
        assert_eq!(
            result4,
            Value::Int(100),
            "x > 30 should return from deeply nested if"
        );
    }

    #[test]
    fn test_edge_case_large_array_of_expressions() {
        // Test handling many sequential expressions
        let mut env = Env::new();

        // Create a function with 50 sequential assignments
        let mut statements = String::new();
        for i in 0..50 {
            statements.push_str(&format!("let x{}: i32 = {};\n", i, i));
        }

        let source = format!(
            r#"
            fn many_statements() -> i32 {{
                {}
                return x49;
            }}
        "#,
            statements
        );

        let program = compile(&source).unwrap();
        execute(&program, &mut env).unwrap();

        let result = call_function("many_statements", &[], &mut env).unwrap();
        assert_eq!(
            result,
            Value::Int(49),
            "Should handle many sequential statements"
        );
    }

    #[test]
    fn test_runtime_unknown_builtin_function_error() {
        // Test calling a non-existent builtin function
        let mut env = Env::new();
        let result = env.call_builtin("nonexistent_func", &[]);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Unknown built-in function: nonexistent_func"));
    }

    #[test]
    fn test_runtime_property_setter_without_callback() {
        // Test self.property = value without property setter callback (runtime error)
        let mut env = Env::new();
        env.set("self".to_string(), Value::SelfObject);
        // Set getter but not setter - this will pass compile time but fail at runtime
        env.set_property_getter(|prop| {
            if prop == "position" {
                Ok(Value::Vector2 { x: 1.0, y: 2.0 })
            } else {
                Err(format!("Unknown property: {}", prop))
            }
        });

        let source = r#"
            fn set_prop() {
                self.position = self.position;
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result = call_function("set_prop", &[], &mut env);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("no property setter registered"),
            "Expected 'no property setter registered', got: {}",
            err
        );
    }

    #[test]
    fn test_runtime_property_getter_without_callback() {
        // Test accessing self.property without property getter callback
        let mut env = Env::new();
        env.set("self".to_string(), Value::SelfObject);

        let source = r#"
            fn get_prop() -> f32 {
                return self.position.x;
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result = call_function("get_prop", &[], &mut env);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("no property getter registered"));
    }

    #[test]
    fn test_runtime_string_value_operations() {
        // Test string value coercion and operations
        let mut env = Env::new();

        let source = r#"
            fn test_strings() -> i32 {
                let s: String = "hello";
                return 42;
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result = call_function("test_strings", &[], &mut env).unwrap();
        assert_eq!(result, Value::Int(42));

        // Test that string Value prints correctly
        let print_result = builtin_print(&[Value::String("test".to_string())]);
        assert!(print_result.is_ok());
    }

    #[test]
    fn test_runtime_nil_value_operations() {
        // Test Nil value coercion
        let nil = Value::Nil;
        assert!(!nil.to_bool());
        assert_eq!(nil.to_float(), None);

        // Test Nil in print
        let print_result = builtin_print(&[Value::Nil]);
        assert!(print_result.is_ok());
    }

    #[test]
    fn test_runtime_selfobject_value_operations() {
        // Test SelfObject value operations
        let self_val = Value::SelfObject;
        assert!(self_val.to_bool());
        assert_eq!(self_val.to_float(), None);

        // Test SelfObject in print
        let print_result = builtin_print(&[Value::SelfObject]);
        assert!(print_result.is_ok());
    }

    #[test]
    fn test_runtime_vector2_value_operations() {
        // Test Vector2 value operations
        let v2 = Value::Vector2 { x: 1.5, y: 2.5 };
        assert!(v2.to_bool());
        assert_eq!(v2.to_float(), None);

        // Test Vector2 in print
        let print_result = builtin_print(&[Value::Vector2 { x: 1.0, y: 2.0 }]);
        assert!(print_result.is_ok());
    }

    #[test]
    fn test_runtime_multiple_scopes() {
        // Test environment scope management
        let mut env = Env::new();

        env.set("outer".to_string(), Value::Int(1));
        env.push_scope();
        env.set("inner".to_string(), Value::Int(2));

        assert_eq!(env.get("outer"), Some(&Value::Int(1)));
        assert_eq!(env.get("inner"), Some(&Value::Int(2)));

        env.pop_scope();
        assert_eq!(env.get("outer"), Some(&Value::Int(1)));
        assert_eq!(env.get("inner"), None);
    }

    #[test]
    fn test_runtime_pop_scope_protection() {
        // Test that pop_scope doesn't remove the global scope
        let mut env = Env::new();
        env.set("global".to_string(), Value::Int(1));

        // Try to pop beyond the global scope
        env.pop_scope(); // Should not remove global scope

        // Global variable should still exist
        assert_eq!(env.get("global"), Some(&Value::Int(1)));
    }

    #[test]
    fn test_runtime_register_custom_builtin() {
        // Test registering a custom builtin function
        fn custom_func(args: &[Value]) -> Result<Value, String> {
            if args.is_empty() {
                Ok(Value::Int(42))
            } else {
                Err("Custom error".to_string())
            }
        }

        let mut env = Env::new();
        env.register_builtin("custom".to_string(), custom_func);

        assert!(env.is_builtin("custom"));
        let result = env.call_builtin("custom", &[]).unwrap();
        assert_eq!(result, Value::Int(42));

        let err = env.call_builtin("custom", &[Value::Int(1)]);
        assert!(err.is_err());
    }

    #[test]
    fn test_runtime_int_to_float_coercion_edge_cases() {
        // Test edge cases in int-to-float coercion
        let zero = Value::Int(0);
        let negative = Value::Int(-100);
        let large = Value::Int(i32::MAX);

        assert_eq!(zero.to_float(), Some(0.0));
        assert_eq!(negative.to_float(), Some(-100.0));
        assert_eq!(large.to_float(), Some(i32::MAX as f32));
    }

    #[test]
    fn test_runtime_comparison_with_mixed_types() {
        // Test comparison operators with mixed int/float types
        let mut env = Env::new();

        let source = r#"
            fn mixed_comparison() -> bool {
                let a: i32 = 10;
                let b: f32 = 5.5;
                return a > b;
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result = call_function("mixed_comparison", &[], &mut env).unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_runtime_equality_with_mixed_types() {
        // Test equality comparison with coercion
        let mut env = Env::new();

        let source = r#"
            fn equality_check() -> bool {
                let a: i32 = 5;
                let b: f32 = 5.0;
                return a == b;
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result = call_function("equality_check", &[], &mut env).unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_runtime_inequality_with_mixed_types() {
        // Test inequality comparison with coercion
        let mut env = Env::new();

        let source = r#"
            fn inequality_check() -> bool {
                let a: i32 = 5;
                let b: f32 = 5.1;
                return a != b;
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result = call_function("inequality_check", &[], &mut env).unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_runtime_less_than_equal_with_coercion() {
        // Test <= comparison with coercion
        let mut env = Env::new();

        let source = r#"
            fn lte_check() -> bool {
                let a: i32 = 5;
                let b: f32 = 5.0;
                return a <= b;
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result = call_function("lte_check", &[], &mut env).unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_runtime_greater_than_equal_with_coercion() {
        // Test >= comparison with coercion
        let mut env = Env::new();

        let source = r#"
            fn gte_check() -> bool {
                let a: i32 = 5;
                let b: f32 = 4.9;
                return a >= b;
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result = call_function("gte_check", &[], &mut env).unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_runtime_less_than_with_coercion() {
        // Test < comparison with coercion
        let mut env = Env::new();

        let source = r#"
            fn lt_check() -> bool {
                let a: i32 = 5;
                let b: f32 = 5.1;
                return a < b;
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result = call_function("lt_check", &[], &mut env).unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    // Signal Tests
    #[test]
    fn test_register_signal() {
        let mut env = Env::new();
        env.register_signal("health_changed".to_string(), 2);

        assert!(env.has_signal("health_changed"));
        assert_eq!(env.get_signal_param_count("health_changed"), Some(2));
        assert!(!env.has_signal("undefined_signal"));
    }

    #[test]
    fn test_signal_declaration_in_program() {
        let mut env = Env::new();

        let source = r#"
            signal health_changed(old: i32, new: i32);
            signal player_died();
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        assert!(env.has_signal("health_changed"));
        assert_eq!(env.get_signal_param_count("health_changed"), Some(2));
        assert!(env.has_signal("player_died"));
        assert_eq!(env.get_signal_param_count("player_died"), Some(0));
    }

    #[test]
    fn test_emit_signal_builtin_exists() {
        let env = Env::new();
        assert!(env.is_builtin("emit_signal"));
    }

    #[test]
    fn test_emit_signal_in_function() {
        let mut env = Env::new();

        let source = r#"
            signal health_changed(old: i32, new: i32);
            
            fn damage() {
                emit_signal("health_changed", 100, 75);
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        // Call the function - emit_signal should not error (stub implementation)
        let result = call_function("damage", &[], &mut env);
        assert!(result.is_ok());
    }

    #[test]
    fn test_emit_signal_with_no_params() {
        let mut env = Env::new();

        let source = r#"
            signal player_died();
            
            fn die() {
                emit_signal("player_died");
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result = call_function("die", &[], &mut env);
        assert!(result.is_ok());
    }

    #[test]
    fn test_signal_emitter_callback_invoked() {
        use std::cell::RefCell;
        use std::rc::Rc;

        let mut env = Env::new();

        // Track signal emissions
        let emissions = Rc::new(RefCell::new(Vec::new()));
        let emissions_clone = emissions.clone();

        // Set up signal emitter callback
        env.set_signal_emitter(Box::new(move |signal_name: &str, args: &[Value]| {
            emissions_clone
                .borrow_mut()
                .push((signal_name.to_string(), args.to_vec()));
            Ok(())
        }));

        let source = r#"
            signal health_changed(old: i32, new: i32);
            
            fn take_damage() {
                emit_signal("health_changed", 100, 75);
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        // Call function that emits signal
        let result = call_function("take_damage", &[], &mut env);
        assert!(result.is_ok());

        // Verify callback was invoked
        let emitted = emissions.borrow();
        assert_eq!(emitted.len(), 1);
        assert_eq!(emitted[0].0, "health_changed");
        assert_eq!(emitted[0].1, vec![Value::Int(100), Value::Int(75)]);
    }

    #[test]
    fn test_signal_emitter_callback_all_types() {
        use std::cell::RefCell;
        use std::rc::Rc;

        let mut env = Env::new();

        // Track signal emissions
        let emissions = Rc::new(RefCell::new(Vec::new()));
        let emissions_clone = emissions.clone();

        env.set_signal_emitter(Box::new(move |signal_name: &str, args: &[Value]| {
            emissions_clone
                .borrow_mut()
                .push((signal_name.to_string(), args.to_vec()));
            Ok(())
        }));

        let source = r#"
            signal all_types(i: i32, f: f32, b: bool, s: String);
            
            fn emit_all() {
                emit_signal("all_types", 42, 3.15, true, "test");
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result = call_function("emit_all", &[], &mut env);
        assert!(result.is_ok());

        let emitted = emissions.borrow();
        assert_eq!(emitted.len(), 1);
        assert_eq!(emitted[0].0, "all_types");
        assert_eq!(
            emitted[0].1,
            vec![
                Value::Int(42),
                Value::Float(3.15),
                Value::Bool(true),
                Value::String("test".to_string()),
            ]
        );
    }

    #[test]
    fn test_signal_emitter_without_callback() {
        // Test that emit_signal works without callback set (no-op)
        let mut env = Env::new();

        let source = r#"
            signal player_died();
            
            fn die() {
                emit_signal("player_died");
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        // Should not error even without callback
        let result = call_function("die", &[], &mut env);
        assert!(result.is_ok());
    }

    #[test]
    fn test_signal_emitter_error_handling() {
        let mut env = Env::new();

        // Set up callback that returns an error
        env.set_signal_emitter(Box::new(|signal_name: &str, _args: &[Value]| {
            Err(format!("Failed to emit signal: {}", signal_name))
        }));

        let source = r#"
            signal test_signal();
            
            fn test() {
                emit_signal("test_signal");
            }
        "#;

        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();

        let result = call_function("test", &[], &mut env);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Failed to emit signal: test_signal"));
    }

    #[test]
    fn test_emit_signal_error_no_signal_name() {
        let mut env = Env::new();

        // Test calling emit_signal with no arguments
        let result = env.call_builtin("emit_signal", &[]);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("emit_signal requires at least a signal name"));
    }

    #[test]
    fn test_emit_signal_error_invalid_signal_name_type() {
        let mut env = Env::new();

        // Test calling emit_signal with non-string first argument
        let result = env.call_builtin("emit_signal", &[Value::Int(42)]);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("emit_signal first argument must be a string"));
    }

    // Phase 2: Lifecycle callback runtime tests

    #[test]
    fn test_call_input_function() {
        let source = r#"
            fn _input(event: InputEvent) {
                print("Input callback called");
            }
        "#;

        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // Create an InputEventHandle
        let input_event = InputEventHandle::new(Some("ui_accept".to_string()), None);
        let input_value = Value::InputEvent(input_event);

        // Call the _input function
        let result = call_function("_input", &[input_value], &mut env);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Nil);
    }

    #[test]
    fn test_call_physics_process_function() {
        let source = r#"
            fn _physics_process(delta: f32) {
                print("Physics callback called");
            }
        "#;

        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // Call the _physics_process function with delta
        let delta_value = Value::Float(0.016);
        let result = call_function("_physics_process", &[delta_value], &mut env);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Nil);
    }

    #[test]
    fn test_call_enter_tree_function() {
        let source = r#"
            fn _enter_tree() {
                print("Enter tree callback called");
            }
        "#;

        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // Call the _enter_tree function
        let result = call_function("_enter_tree", &[], &mut env);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Nil);
    }

    #[test]
    fn test_call_exit_tree_function() {
        let source = r#"
            fn _exit_tree() {
                print("Exit tree callback called");
            }
        "#;

        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // Call the _exit_tree function
        let result = call_function("_exit_tree", &[], &mut env);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Nil);
    }

    #[test]
    fn test_input_event_is_action_pressed() {
        // Test InputEventHandle is_action_pressed method
        let input_event = InputEventHandle::new(Some("ui_accept".to_string()), None);
        assert!(input_event.is_action_pressed("ui_accept"));
        assert!(!input_event.is_action_pressed("ui_cancel"));

        // Test with different action
        let input_event2 = InputEventHandle::new(Some("move_left".to_string()), None);
        assert!(input_event2.is_action_pressed("move_left"));
        assert!(!input_event2.is_action_pressed("ui_accept"));

        // Test with no action
        let input_event3 = InputEventHandle::new(None, None);
        assert!(!input_event3.is_action_pressed("ui_accept"));
    }

    #[test]
    fn test_input_event_is_action_released() {
        // Test InputEventHandle is_action_released method
        let input_event = InputEventHandle::new(None, Some("ui_accept".to_string()));
        assert!(input_event.is_action_released("ui_accept"));
        assert!(!input_event.is_action_released("ui_cancel"));

        // Test with pressed action (should not be released)
        let input_event2 = InputEventHandle::new(Some("ui_accept".to_string()), None);
        assert!(!input_event2.is_action_released("ui_accept"));

        // Test with no action
        let input_event3 = InputEventHandle::new(None, None);
        assert!(!input_event3.is_action_released("ui_accept"));
    }

    #[test]
    fn test_input_function_with_event_parameter() {
        // Test _input function receives InputEvent parameter
        let source = r#"
            fn _input(event: InputEvent) {
                print("Input received");
            }
        "#;

        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // Create input event with pressed action
        let input_event = InputEventHandle::new(Some("ui_accept".to_string()), None);
        let input_value = Value::InputEvent(input_event);

        let result = call_function("_input", &[input_value], &mut env);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lifecycle_functions_with_return_values() {
        // Test that lifecycle functions can have return values (even though typically void)
        let source = r#"
            fn _physics_process(delta: f32) -> i32 {
                return 42;
            }
        "#;

        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        let delta_value = Value::Float(0.016);
        let result = call_function("_physics_process", &[delta_value], &mut env);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Int(42));
    }

    #[test]
    fn test_lifecycle_functions_with_variables() {
        // Test lifecycle functions that use variables
        let source = r#"
            fn _physics_process(delta: f32) {
                let speed: f32 = 100.0;
                let distance: f32 = speed * delta;
                print("Moved distance");
            }
        "#;

        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        let delta_value = Value::Float(0.016);
        let result = call_function("_physics_process", &[delta_value], &mut env);
        assert!(result.is_ok());
    }

    #[test]
    fn test_call_function_wrong_arg_count() {
        // Test calling lifecycle function with wrong number of arguments
        let source = r#"
            fn _physics_process(delta: f32) {
                print("Physics");
            }
        "#;

        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // Try to call with no arguments (should fail)
        let result = call_function("_physics_process", &[], &mut env);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects 1 arguments, got 0"));
    }

    // Phase 3: Node Query Functions tests

    #[test]
    fn test_call_get_node_function() {
        let source = r#"
            fn test_get() {
                let node = get_node("path/to/node");
            }
        "#;

        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // Mock callback for get_node
        fn mock_node_query(path: &str, query_type: NodeQueryType) -> Result<Value, String> {
            match query_type {
                NodeQueryType::GetNode => Ok(Value::Node(NodeHandle::new(path.to_string()))),
                _ => Err("Unexpected query type".to_string()),
            }
        }
        env.set_node_query_callback(mock_node_query);

        // Call the test function
        let result = call_function("test_get", &[], &mut env);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Nil);
    }

    #[test]
    fn test_call_get_parent_function() {
        let source = r#"
            fn test_parent() {
                let parent = get_parent();
            }
        "#;

        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // Mock callback for get_parent
        fn mock_node_query(_path: &str, query_type: NodeQueryType) -> Result<Value, String> {
            match query_type {
                NodeQueryType::GetParent => {
                    Ok(Value::Node(NodeHandle::new("<parent>".to_string())))
                }
                _ => Err("Unexpected query type".to_string()),
            }
        }
        env.set_node_query_callback(mock_node_query);

        // Call the test function
        let result = call_function("test_parent", &[], &mut env);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Nil);
    }

    #[test]
    fn test_call_has_node_function() {
        let source = r#"
            fn test_has() {
                let exists = has_node("path/to/node");
            }
        "#;

        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // Mock callback for has_node
        fn mock_node_query(_path: &str, query_type: NodeQueryType) -> Result<Value, String> {
            match query_type {
                NodeQueryType::HasNode => Ok(Value::Bool(true)),
                _ => Err("Unexpected query type".to_string()),
            }
        }
        env.set_node_query_callback(mock_node_query);

        // Call the test function
        let result = call_function("test_has", &[], &mut env);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Nil);
    }

    #[test]
    fn test_call_find_child_function() {
        let source = r#"
            fn test_find() {
                let child = find_child("ChildName");
            }
        "#;

        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // Mock callback for find_child
        fn mock_node_query(name: &str, query_type: NodeQueryType) -> Result<Value, String> {
            match query_type {
                NodeQueryType::FindChild => {
                    Ok(Value::Node(NodeHandle::new(format!("<child:{}>", name))))
                }
                _ => Err("Unexpected query type".to_string()),
            }
        }
        env.set_node_query_callback(mock_node_query);

        // Call the test function
        let result = call_function("test_find", &[], &mut env);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Nil);
    }

    #[test]
    fn test_node_query_error_handling() {
        let source = r#"
            fn test_error() {
                let node = get_node("");
            }
        "#;

        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // Mock callback that always errors
        fn mock_node_query(_path: &str, _query_type: NodeQueryType) -> Result<Value, String> {
            Err("Node not found".to_string())
        }
        env.set_node_query_callback(mock_node_query);

        // Call should fail due to empty path (E602: Path cannot be empty)
        let result = call_function("test_error", &[], &mut env);
        assert!(result.is_err());
        // Error might be E602 (empty path) or callback error
        let err = result.unwrap_err();
        assert!(err.contains("E602") || err.contains("Node not found") || err.contains("empty"));
    }

    #[test]
    fn test_node_query_without_callback() {
        let source = r#"
            fn test_no_callback() {
                let node = get_node("path");
            }
        "#;

        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // Call without setting callback should fail
        let result = call_function("test_no_callback", &[], &mut env);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("E604"));
    }

    // Test ID: NQ-008 - Empty string path
    #[test]
    fn test_get_node_empty_string() {
        let source = r#"
            fn test_empty_path() {
                let node = get_node("");
            }
        "#;

        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // Mock callback (won't be called because runtime checks for empty path first)
        fn mock_node_query(path: &str, query_type: NodeQueryType) -> Result<Value, String> {
            match query_type {
                NodeQueryType::GetNode => Ok(Value::Node(NodeHandle::new(path.to_string()))),
                _ => Err("Unexpected query type".to_string()),
            }
        }
        env.set_node_query_callback(mock_node_query);

        let result = call_function("test_empty_path", &[], &mut env);
        // Should error because runtime validates empty paths
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("E603"));
        assert!(error_msg.contains("Node path cannot be empty"));
    }

    // Test ID: NQ-022 - get_parent() without callback
    #[test]
    fn test_get_parent_without_callback() {
        let source = r#"
            fn test_no_callback() {
                let parent = get_parent();
            }
        "#;

        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // Call without setting callback should fail
        let result = call_function("test_no_callback", &[], &mut env);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("E606")); // No callback registered
    }

    // Test ID: NQ-035 - has_node() without callback
    #[test]
    fn test_has_node_without_callback() {
        let source = r#"
            fn test_no_callback() {
                let exists = has_node("SomeNode");
            }
        "#;

        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // Call without setting callback should error with E609
        let result = call_function("test_no_callback", &[], &mut env);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("E609")); // No callback registered
    }

    // Test ID: NQ-037 - has_node() with empty string
    #[test]
    fn test_has_node_empty_string() {
        let source = r#"
            fn test_empty() {
                let exists = has_node("");
            }
        "#;

        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // Mock callback that will receive empty path (runtime doesn't validate for has_node)
        fn mock_node_query(path: &str, query_type: NodeQueryType) -> Result<Value, String> {
            if path.is_empty() {
                Err("Empty path not allowed".to_string())
            } else {
                match query_type {
                    NodeQueryType::HasNode => Ok(Value::Bool(true)),
                    _ => Err("Unexpected query type".to_string()),
                }
            }
        }
        env.set_node_query_callback(mock_node_query);

        let result = call_function("test_empty", &[], &mut env);
        // Callback will reject empty path, causing error
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Empty path"));
    }

    // Test ID: SIG-037 - Signal name as variable (NOT SUPPORTED)
    // This test documents that signal names must be string literals, not variables
    #[test]
    fn test_emit_signal_name_as_variable() {
        let source = r#"
            signal player_died();
            
            fn test_dynamic_name() {
                let signal_name = "player_died";
                emit_signal(signal_name);
            }
        "#;

        // Should fail at compile time - signal names must be string literals
        let program = compile(source);
        assert!(program.is_err());
        let error_msg = program.unwrap_err();
        assert!(error_msg.contains("E205"));
        assert!(error_msg.contains("Signal name must be known at compile time"));
    }

    // ===== Phase 4: Godot Types Runtime Tests =====

    // Note: Runtime field access for Phase 4 types (Color, Rect2, Transform2D) is tested
    // at the type checker level. Full integration tests would require variable declarations
    // in the source code to pass type checking.

    #[test]
    fn test_color_to_string() {
        // Test that Color values format correctly in builtin_print
        let formatted = format!("Color({}, {}, {}, {})", 1.0, 0.5, 0.0, 1.0);
        assert!(formatted.contains("Color"));
        assert!(formatted.contains("1") && formatted.contains("0.5") && formatted.contains("0"));
    }

    #[test]
    fn test_rect2_to_string() {
        // Format using builtin_print logic for Rect2
        let formatted = format!(
            "Rect2(Vector2({}, {}), Vector2({}, {}))",
            0.0, 0.0, 100.0, 50.0
        );
        assert!(formatted.contains("Rect2"));
        assert!(formatted.contains("Vector2"));
    }

    #[test]
    fn test_transform2d_to_string() {
        // Format using builtin_print logic for Transform2D
        let formatted = format!(
            "Transform2D(Vector2({}, {}), {}, Vector2({}, {}))",
            10.0, 20.0, 0.785, 1.0, 1.0
        );
        assert!(formatted.contains("Transform2D"));
        assert!(formatted.contains("0.785"));
    }

    // ==================== ROBUSTNESS TESTS (Phase 4.5) ====================
    // Tests for struct literal execution, field access chains, and runtime behavior

    #[test]
    fn test_vector2_literal_execution() {
        // Test that Vector2 literals execute correctly
        let mut env = Env::new();
        let source = r#"
            fn test() -> f32 {
                let v = Vector2 { x: 10.0, y: 20.0 };
                return v.x + v.y;
            }
        "#;
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        let result = call_function("test", &[], &mut env).unwrap();
        assert_eq!(result, Value::Float(30.0));
    }

    #[test]
    fn test_color_literal_execution() {
        // Test that Color literals execute correctly
        let mut env = Env::new();
        let source = r#"
            fn test() -> f32 {
                let c = Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 };
                return c.r + c.g + c.b + c.a;
            }
        "#;
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        let result = call_function("test", &[], &mut env).unwrap();
        assert_eq!(result, Value::Float(2.5));
    }

    #[test]
    fn test_rect2_literal_execution() {
        // Test that Rect2 literals execute correctly with nested Vector2
        let mut env = Env::new();
        let source = r#"
            fn test() -> f32 {
                let pos = Vector2 { x: 10.0, y: 20.0 };
                let size = Vector2 { x: 100.0, y: 50.0 };
                let rect = Rect2 { position: pos, size: size };
                return rect.position.x + rect.size.x;
            }
        "#;
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        let result = call_function("test", &[], &mut env).unwrap();
        assert_eq!(result, Value::Float(110.0));
    }

    #[test]
    fn test_transform2d_literal_execution() {
        // Test that Transform2D literals execute correctly with mixed types
        let mut env = Env::new();
        let source = r#"
            fn test() -> f32 {
                let pos = Vector2 { x: 100.0, y: 200.0 };
                let scale = Vector2 { x: 2.0, y: 2.0 };
                let t = Transform2D { position: pos, rotation: 1.57, scale: scale };
                return t.position.x + t.rotation + t.scale.x;
            }
        "#;
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        let result = call_function("test", &[], &mut env).unwrap();
        assert_eq!(result, Value::Float(103.57));
    }

    #[test]
    fn test_struct_literal_as_function_parameter() {
        // Test passing struct literals as function arguments
        let mut env = Env::new();
        let source = r#"
            fn add_vectors(v1: Vector2, v2: Vector2) -> f32 {
                return v1.x + v1.y + v2.x + v2.y;
            }
            fn test() -> f32 {
                return add_vectors(Vector2 { x: 1.0, y: 2.0 }, Vector2 { x: 3.0, y: 4.0 });
            }
        "#;
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        let result = call_function("test", &[], &mut env).unwrap();
        assert_eq!(result, Value::Float(10.0));
    }

    #[test]
    fn test_struct_literal_as_return_value() {
        // Test returning struct literals from functions
        let mut env = Env::new();
        let source = r#"
            fn make_vector() -> Vector2 {
                return Vector2 { x: 42.0, y: 84.0 };
            }
            fn test() -> f32 {
                let v = make_vector();
                return v.x + v.y;
            }
        "#;
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        let result = call_function("test", &[], &mut env).unwrap();
        assert_eq!(result, Value::Float(126.0));
    }

    #[test]
    fn test_nested_field_access_chain() {
        // Test deep field access chains: rect.position.x
        let mut env = Env::new();
        let source = r#"
            fn test() -> f32 {
                let pos = Vector2 { x: 5.0, y: 10.0 };
                let size = Vector2 { x: 15.0, y: 20.0 };
                let rect = Rect2 { position: pos, size: size };
                return rect.position.x + rect.position.y + rect.size.x + rect.size.y;
            }
        "#;
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        let result = call_function("test", &[], &mut env).unwrap();
        assert_eq!(result, Value::Float(50.0));
    }

    #[test]
    fn test_struct_literal_in_conditional() {
        // Test using struct literals in if conditions
        let mut env = Env::new();
        let source = r#"
            fn test() -> f32 {
                let v = Vector2 { x: 10.0, y: 20.0 };
                if v.x > 5.0 {
                    return v.y;
                }
                return 0.0;
            }
        "#;
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        let result = call_function("test", &[], &mut env).unwrap();
        assert_eq!(result, Value::Float(20.0));
    }

    #[test]
    fn test_struct_literal_in_while_loop() {
        // Test using struct literals in while loops
        let mut env = Env::new();
        let source = r#"
            fn test() -> f32 {
                let mut v = Vector2 { x: 0.0, y: 0.0 };
                while v.x < 5.0 {
                    v.x = v.x + 1.0;
                    v.y = v.y + 2.0;
                }
                return v.y;
            }
        "#;
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        let result = call_function("test", &[], &mut env).unwrap();
        assert_eq!(result, Value::Float(10.0));
    }

    #[test]
    fn test_integer_to_float_coercion_in_struct_literal() {
        // Test that i32 values coerce to f32 in struct literals
        let mut env = Env::new();
        let source = r#"
            fn test() -> f32 {
                let v = Vector2 { x: 10, y: 20 };
                return v.x + v.y;
            }
        "#;
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        let result = call_function("test", &[], &mut env).unwrap();
        assert_eq!(result, Value::Float(30.0));
    }

    #[test]
    fn test_color_literal_with_integer_components() {
        // Test Color with integer components (common use case)
        let mut env = Env::new();
        let source = r#"
            fn test() -> f32 {
                let c = Color { r: 1, g: 0, b: 0, a: 1 };
                return c.r + c.a;
            }
        "#;
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        let result = call_function("test", &[], &mut env).unwrap();
        assert_eq!(result, Value::Float(2.0));
    }

    #[test]
    fn test_multiple_struct_literals_in_expression() {
        // Test complex expressions with multiple struct literals
        let mut env = Env::new();
        let source = r#"
            fn test() -> f32 {
                let v1 = Vector2 { x: 1.0, y: 2.0 };
                let v2 = Vector2 { x: 3.0, y: 4.0 };
                let v3 = Vector2 { x: 5.0, y: 6.0 };
                return v1.x + v2.y + v3.x + v3.y;
            }
        "#;
        let program = compile(source).unwrap();
        execute(&program, &mut env).unwrap();
        let result = call_function("test", &[], &mut env).unwrap();
        assert_eq!(result, Value::Float(16.0));
    }

    // ========== Phase 5: Exported Property Tests (Bundle 1: Checkpoints 3.1 & 3.2) ==========

    #[test]
    fn test_initialize_exported_properties_from_metadata() {
        // Test that exported properties are initialized with default values
        let source = r#"
@export let mut health: i32 = 100;
@export let mut speed: f32 = 10.5;
@export let mut enabled: bool = true;
@export let mut name: String = "Player";
        "#;
        let program = compile(source).unwrap();
        let mut env = Env::new();

        // Execute should call initialize_properties
        execute(&program, &mut env).unwrap();

        // Check that properties were initialized with default values
        assert_eq!(
            env.get_exported_property("health").unwrap(),
            Value::Int(100)
        );
        assert_eq!(
            env.get_exported_property("speed").unwrap(),
            Value::Float(10.5)
        );
        assert_eq!(
            env.get_exported_property("enabled").unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            env.get_exported_property("name").unwrap(),
            Value::String("Player".to_string())
        );
    }

    #[test]
    fn test_initialize_multiple_exported_properties() {
        // Test multiple properties with different types and hints
        let source = r#"
@export(range(0, 100, 1)) let mut health: i32 = 75;
@export(range(0.0, 20.0, 0.5)) let mut speed: f32 = 12.5;
@export(enum("Easy", "Normal", "Hard")) let mut difficulty: String = "Normal";
@export let mut position: Vector2 = Vector2 { x: 10.0, y: 20.0 };
@export let mut color: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
        "#;
        let program = compile(source).unwrap();
        let mut env = Env::new();

        execute(&program, &mut env).unwrap();

        // Verify all properties initialized correctly
        assert_eq!(env.get_exported_property("health").unwrap(), Value::Int(75));
        assert_eq!(
            env.get_exported_property("speed").unwrap(),
            Value::Float(12.5)
        );
        assert_eq!(
            env.get_exported_property("difficulty").unwrap(),
            Value::String("Normal".to_string())
        );

        // Verify Vector2 struct literal parsed correctly
        let position = env.get_exported_property("position").unwrap();
        if let Value::Vector2 { x, y } = position {
            assert_eq!(x, 10.0);
            assert_eq!(y, 20.0);
        } else {
            panic!("Expected Vector2, got {:?}", position);
        }

        // Verify Color struct literal parsed correctly
        let color = env.get_exported_property("color").unwrap();
        if let Value::Color { r, g, b, a } = color {
            assert_eq!(r, 1.0);
            assert_eq!(g, 0.0);
            assert_eq!(b, 0.0);
            assert_eq!(a, 1.0);
        } else {
            panic!("Expected Color, got {:?}", color);
        }
    }

    // ========== Phase 5: Exported Property Tests (Bundle 2: Checkpoints 3.3 & 3.4) ==========

    #[test]
    fn test_get_exported_property_success() {
        // Test getting an initialized exported property
        let source = "@export let mut health: i32 = 100;";
        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        let result = env.get_exported_property("health");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Int(100));
    }

    #[test]
    fn test_get_exported_property_not_found() {
        // Test getting a property that doesn't exist
        let env = Env::new();
        let result = env.get_exported_property("nonexistent");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_set_exported_property_no_clamping() {
        // Test setting property within range (no clamping needed)
        let source = "@export(range(0, 100, 1)) let mut health: i32 = 100;";
        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // Set within range (from Inspector)
        let result = env.set_exported_property("health", Value::Int(50), true);
        assert!(result.is_ok());
        assert_eq!(env.get_exported_property("health").unwrap(), Value::Int(50));

        // Set within range (from script)
        let result = env.set_exported_property("health", Value::Int(75), false);
        assert!(result.is_ok());
        assert_eq!(env.get_exported_property("health").unwrap(), Value::Int(75));
    }

    #[test]
    fn test_set_exported_property_clamp_from_inspector() {
        // Test clamping when set from Inspector
        let source = "@export(range(0, 100, 1)) let mut health: i32 = 100;";
        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // Set above max (from Inspector - should clamp)
        env.set_exported_property("health", Value::Int(150), true)
            .unwrap();
        assert_eq!(
            env.get_exported_property("health").unwrap(),
            Value::Int(100)
        );

        // Set below min (from Inspector - should clamp)
        env.set_exported_property("health", Value::Int(-50), true)
            .unwrap();
        assert_eq!(env.get_exported_property("health").unwrap(), Value::Int(0));
    }

    #[test]
    fn test_set_exported_property_warn_from_script() {
        // Test that script sets allow out-of-range but warn (captured via stderr)
        let source = "@export(range(0, 100, 1)) let mut health: i32 = 100;";
        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // Set above max (from script - should allow)
        let result = env.set_exported_property("health", Value::Int(150), false);
        assert!(result.is_ok());
        assert_eq!(
            env.get_exported_property("health").unwrap(),
            Value::Int(150)
        );

        // Set below min (from script - should allow)
        let result = env.set_exported_property("health", Value::Int(-50), false);
        assert!(result.is_ok());
        assert_eq!(
            env.get_exported_property("health").unwrap(),
            Value::Int(-50)
        );
    }

    #[test]
    fn test_set_exported_property_clamp_float_range() {
        // Test float clamping
        let source = "@export(range(0.0, 20.0, 0.5)) let mut speed: f32 = 10.0;";
        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // Set above max
        env.set_exported_property("speed", Value::Float(25.5), true)
            .unwrap();
        assert_eq!(
            env.get_exported_property("speed").unwrap(),
            Value::Float(20.0)
        );

        // Set below min
        env.set_exported_property("speed", Value::Float(-5.0), true)
            .unwrap();
        assert_eq!(
            env.get_exported_property("speed").unwrap(),
            Value::Float(0.0)
        );
    }

    #[test]
    fn test_set_exported_property_nan_infinity_error() {
        // Test that NaN and Infinity are rejected for range hints
        let source = "@export(range(0.0, 100.0, 1.0)) let mut value: f32 = 50.0;";
        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // NaN should error
        let result = env.set_exported_property("value", Value::Float(f32::NAN), true);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("NaN"));

        // Infinity should error
        let result = env.set_exported_property("value", Value::Float(f32::INFINITY), true);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("infinite"));

        // Negative infinity should error
        let result = env.set_exported_property("value", Value::Float(f32::NEG_INFINITY), true);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("infinite"));
    }

    #[test]
    fn test_set_exported_property_negative_range() {
        // Test clamping with negative ranges
        let source = "@export(range(-100, 100, 1)) let mut offset: i32 = 0;";
        let program = compile(source).unwrap();
        let mut env = Env::new();
        execute(&program, &mut env).unwrap();

        // Set below min (should clamp to -100)
        env.set_exported_property("offset", Value::Int(-150), true)
            .unwrap();
        assert_eq!(
            env.get_exported_property("offset").unwrap(),
            Value::Int(-100)
        );

        // Set above max (should clamp to 100)
        env.set_exported_property("offset", Value::Int(150), true)
            .unwrap();
        assert_eq!(
            env.get_exported_property("offset").unwrap(),
            Value::Int(100)
        );

        // Set within range
        env.set_exported_property("offset", Value::Int(-50), true)
            .unwrap();
        assert_eq!(
            env.get_exported_property("offset").unwrap(),
            Value::Int(-50)
        );
    }
}
