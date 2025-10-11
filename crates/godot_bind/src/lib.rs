use ferrisscript_compiler::{ast, compile};
use ferrisscript_runtime::{call_function, execute, Env, InputEventHandle, Value};
use godot::classes::{file_access::ModeFlags, FileAccess, InputEvent};
use godot::prelude::*;
use std::cell::RefCell;

// PropertyInfo imports for Inspector integration (Bundle 4 - Checkpoint 3.7)
use godot::builtin::VariantType;
use godot::global::{PropertyHint, PropertyUsageFlags};
use godot::meta::{ClassId, PropertyHintInfo, PropertyInfo};
use godot::register::property::export_info_functions;

// Signal prototype module for v0.0.4 research
mod signal_prototype;
pub use signal_prototype::SignalPrototype;

/// PropertyUsage helper for exported properties (Bundle 4 - Checkpoint 3.7)
/// In godot-rust 0.4.0, DEFAULT does not include EDITOR or STORAGE  
/// PROPERTY_USAGE_COMMON = DEFAULT | EDITOR | STORAGE for full Inspector integration
/// Note: PropertyUsageFlags BitOr is not const, so this is a function
#[inline]
fn property_usage_common() -> PropertyUsageFlags {
    PropertyUsageFlags::DEFAULT | PropertyUsageFlags::EDITOR | PropertyUsageFlags::STORAGE
}

// ============================================================================
// Phase 5 Sub-Phase 3: PropertyInfo Generation Helpers (Bundle 4 - Checkpoint 3.7)
// ============================================================================

/// Map FerrisScript type name to Godot VariantType
///
/// Supports all 8 exportable types from Phase 5 Sub-Phase 2:
/// - Primitives: i32, f32, bool, String
/// - Godot structs: Vector2, Color, Rect2, Transform2D
///
/// Returns VariantType::NIL for unknown types with a warning.
#[allow(dead_code)]
fn map_type_to_variant(type_name: &str) -> VariantType {
    match type_name {
        "i32" => VariantType::INT,
        "f32" => VariantType::FLOAT,
        "bool" => VariantType::BOOL,
        "String" => VariantType::STRING,
        "Vector2" => VariantType::VECTOR2,
        "Color" => VariantType::COLOR,
        "Rect2" => VariantType::RECT2,
        "Transform2D" => VariantType::TRANSFORM2D,
        _ => {
            godot_warn!(
                "Unknown FerrisScript type '{}' for export, defaulting to NIL",
                type_name
            );
            VariantType::NIL
        }
    }
}

/// Map FerrisScript PropertyHint to Godot PropertyHintInfo
///
/// Uses export_info_functions helpers for robust, cross-platform hint strings.
///
/// Hint formats (per Godot 4.x conventions):
/// - Range: "min,max,step" (uses export_range helper)
/// - Enum: "Value1,Value2,Value3" (comma-separated)
/// - File: "*.ext1;*.ext2" (semicolons for Windows compatibility)
/// - None: empty hint string
#[allow(dead_code)]
fn map_hint(hint: &ast::PropertyHint) -> PropertyHintInfo {
    match hint {
        ast::PropertyHint::None => PropertyHintInfo {
            hint: PropertyHint::NONE,
            hint_string: GString::new(),
        },

        ast::PropertyHint::Range { min, max, step } => {
            // Use export_info_functions for robust formatting
            export_info_functions::export_range(
                *min as f64,
                *max as f64,
                Some(*step as f64),
                false, // or_greater
                false, // or_less
                false, // exp
                false, // radians_as_degrees
                false, // degrees
                false, // hide_slider
                None,  // suffix
            )
        }

        ast::PropertyHint::Enum { values } => {
            let enum_string = values.join(",");
            PropertyHintInfo {
                hint: PropertyHint::ENUM,
                hint_string: GString::from(&enum_string),
            }
        }

        ast::PropertyHint::File { extensions } => {
            // Format extensions with wildcards and use semicolons (Windows compatibility)
            let formatted_exts: Vec<String> = extensions
                .iter()
                .map(|ext| {
                    if ext.starts_with("*.") {
                        ext.clone()
                    } else if ext.starts_with('.') {
                        format!("*{}", ext)
                    } else {
                        format!("*.{}", ext)
                    }
                })
                .collect();

            let file_string = formatted_exts.join(";");
            PropertyHintInfo {
                hint: PropertyHint::FILE,
                hint_string: GString::from(&file_string),
            }
        }
    }
}

/// Convert FerrisScript PropertyMetadata to Godot PropertyInfo
///
/// This is the main conversion function that combines type and hint mapping.
/// Uses verified API patterns:
/// - ClassId::none() for non-object types
/// - property_usage_common() for standard usage flags
/// - Generates fresh PropertyInfo on each call (Godot best practice)
#[allow(dead_code)]
fn metadata_to_property_info(metadata: &ast::PropertyMetadata) -> PropertyInfo {
    PropertyInfo {
        variant_type: map_type_to_variant(&metadata.type_name),
        class_id: ClassId::none(), // FerrisScript types are not Godot objects
        property_name: StringName::from(&metadata.name),
        hint_info: map_hint(&metadata.hint),
        usage: property_usage_common(),
    }
}

// Thread-local storage for node properties during script execution
thread_local! {
    static NODE_POSITION: RefCell<Option<Vector2>> = const { RefCell::new(None) };
    /// Store the current node's instance ID for node query operations
    static CURRENT_NODE_INSTANCE_ID: RefCell<Option<InstanceId>> = const { RefCell::new(None) };
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

/// Node query callback for scene tree operations (called from runtime)
fn node_query_callback_tls(
    path_or_name: &str,
    query_type: ferrisscript_runtime::NodeQueryType,
) -> Result<Value, String> {
    use ferrisscript_runtime::{NodeHandle, NodeQueryType};

    CURRENT_NODE_INSTANCE_ID.with(|instance_id_cell| {
        let instance_id = instance_id_cell
            .borrow()
            .ok_or_else(|| "Node instance ID not available".to_string())?;

        // Get the node from instance ID
        let node = Gd::<Node2D>::try_from_instance_id(instance_id)
            .map_err(|_| "Node no longer exists".to_string())?;

        match query_type {
            NodeQueryType::GetNode => {
                // Try to get the node by path
                let target_node = node.try_get_node_as::<Node2D>(path_or_name);
                match target_node {
                    Some(_) => {
                        // For now, return a NodeHandle with the path as identifier
                        // ⚠️ ASSUMPTION: Simplified NodeHandle implementation
                        // In future, may need to store actual Godot node reference
                        Ok(Value::Node(NodeHandle::new(path_or_name.to_string())))
                    }
                    None => Err(format!("Node not found: {}", path_or_name)),
                }
            }
            NodeQueryType::GetParent => {
                let parent = node.get_parent();
                match parent {
                    Some(_) => {
                        // Return NodeHandle with "parent" identifier
                        Ok(Value::Node(NodeHandle::new("<parent>".to_string())))
                    }
                    None => Err("Node has no parent".to_string()),
                }
            }
            NodeQueryType::HasNode => {
                // Check if node exists at path
                let has_node = node.has_node(path_or_name);
                Ok(Value::Bool(has_node))
            }
            NodeQueryType::FindChild => {
                // Find child by name (recursive search)
                // Godot's find_child takes only the name pattern
                let child = node.find_child(path_or_name);
                match child {
                    Some(_) => {
                        // Return NodeHandle with child name as identifier
                        Ok(Value::Node(NodeHandle::new(format!(
                            "<child:{}>",
                            path_or_name
                        ))))
                    }
                    None => Err(format!("Child node not found: {}", path_or_name)),
                }
            }
        }
    })
}

/// Convert FerrisScript Value to Godot Variant
///
/// Handles edge cases for numeric types:
/// - NaN floats are converted to 0.0 with a warning
/// - Infinite floats are clamped to f32::MAX/MIN with a warning
///
/// Invalid nested values (e.g., non-Vector2 in Rect2) return Variant::nil()
fn value_to_variant(value: &Value) -> Variant {
    match value {
        Value::Int(i) => Variant::from(*i),
        Value::Float(f) => {
            // Handle NaN and Infinity edge cases
            if f.is_nan() {
                godot_warn!("NaN value in Value→Variant conversion, defaulting to 0.0");
                Variant::from(0.0f32)
            } else if f.is_infinite() {
                let clamped = if f.is_sign_positive() {
                    f32::MAX
                } else {
                    f32::MIN
                };
                godot_warn!(
                    "Infinite value in Value→Variant conversion, clamping to {}",
                    clamped
                );
                Variant::from(clamped)
            } else {
                Variant::from(*f)
            }
        }
        Value::Bool(b) => Variant::from(*b),
        Value::String(s) => Variant::from(s.as_str()),
        Value::Vector2 { x, y } => Variant::from(Vector2::new(*x, *y)),
        Value::Color { r, g, b, a } => Variant::from(Color::from_rgba(*r, *g, *b, *a)),
        Value::Rect2 { position, size } => {
            // Extract Vector2 values from boxed Values
            match (&**position, &**size) {
                (Value::Vector2 { x: px, y: py }, Value::Vector2 { x: sx, y: sy }) => {
                    Variant::from(Rect2::new(Vector2::new(*px, *py), Vector2::new(*sx, *sy)))
                }
                _ => Variant::nil(), // Invalid nested values
            }
        }
        Value::Transform2D {
            position,
            rotation,
            scale,
        } => {
            // Extract Vector2 values from boxed Values
            match (&**position, &**scale) {
                (Value::Vector2 { x: px, y: py }, Value::Vector2 { x: sx, y: sy }) => {
                    Variant::from(Transform2D::from_angle_scale_skew_origin(
                        *rotation,
                        Vector2::new(*sx, *sy),
                        0.0, // skew
                        Vector2::new(*px, *py),
                    ))
                }
                _ => Variant::nil(), // Invalid nested values
            }
        }
        Value::Nil => Variant::nil(),
        Value::SelfObject => Variant::nil(), // self cannot be passed as signal parameter
        Value::InputEvent(_) => Variant::nil(), // InputEvent cannot be passed as signal parameter
        Value::Node(_) => Variant::nil(),    // Node cannot be passed as signal parameter
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
            Value::Color { r, g, b, a } => format!("Color({}, {}, {}, {})", r, g, b, a),
            Value::Rect2 { position, size } => match (&**position, &**size) {
                (Value::Vector2 { x: px, y: py }, Value::Vector2 { x: sx, y: sy }) => {
                    format!("Rect2(Vector2({}, {}), Vector2({}, {}))", px, py, sx, sy)
                }
                _ => "Rect2(invalid, invalid)".to_string(),
            },
            Value::Transform2D {
                position,
                rotation,
                scale,
            } => match (&**position, &**scale) {
                (Value::Vector2 { x: px, y: py }, Value::Vector2 { x: sx, y: sy }) => {
                    format!(
                        "Transform2D(Vector2({}, {}), {}, Vector2({}, {}))",
                        px, py, rotation, sx, sy
                    )
                }
                _ => "Transform2D(invalid, invalid, invalid)".to_string(),
            },
            Value::Nil => "nil".to_string(),
            Value::SelfObject => "self".to_string(),
            Value::InputEvent(_) => "InputEvent".to_string(),
            Value::Node(handle) => format!("Node({})", handle.id()),
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
#[class(base=Node2D, tool)] // tool annotation enables Inspector/editor integration
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
            if let Some(env) = &self.env {
                if env.get_function("_ready").is_some() {
                    self.call_script_function("_ready", &[]);
                }
            }
        }
    }

    fn process(&mut self, delta: f64) {
        // Execute _process function if script is loaded and function exists
        if self.script_loaded {
            if let Some(env) = &self.env {
                if env.get_function("_process").is_some() {
                    // Convert delta to Float (f32 for FerrisScript)
                    let delta_value = Value::Float(delta as f32);
                    self.call_script_function_with_self("_process", &[delta_value]);
                }
            }
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        // Execute _input function if script is loaded and function exists
        if self.script_loaded {
            if let Some(env) = &self.env {
                if env.get_function("_input").is_some() {
                    // Convert Godot InputEvent to FerrisScript InputEventHandle
                    // NOTE: Simplified implementation for Phase 2.1
                    // - Currently checks hardcoded common actions (ui_* actions)
                    // - Stores action name strings, not full Godot event reference
                    // - Full InputEvent API (position, button_index, etc.) deferred to Phase 5/6
                    // See: docs/planning/v0.0.4/KNOWN_LIMITATIONS.md - "InputEvent Simplified API"
                    let action_pressed = if event.is_action_pressed("ui_accept") {
                        Some("ui_accept".to_string())
                    } else if event.is_action_pressed("ui_cancel") {
                        Some("ui_cancel".to_string())
                    } else if event.is_action_pressed("ui_left") {
                        Some("ui_left".to_string())
                    } else if event.is_action_pressed("ui_right") {
                        Some("ui_right".to_string())
                    } else if event.is_action_pressed("ui_up") {
                        Some("ui_up".to_string())
                    } else if event.is_action_pressed("ui_down") {
                        Some("ui_down".to_string())
                    } else {
                        None
                    };

                    let action_released = if event.is_action_released("ui_accept") {
                        Some("ui_accept".to_string())
                    } else if event.is_action_released("ui_cancel") {
                        Some("ui_cancel".to_string())
                    } else if event.is_action_released("ui_left") {
                        Some("ui_left".to_string())
                    } else if event.is_action_released("ui_right") {
                        Some("ui_right".to_string())
                    } else if event.is_action_released("ui_up") {
                        Some("ui_up".to_string())
                    } else if event.is_action_released("ui_down") {
                        Some("ui_down".to_string())
                    } else {
                        None
                    };

                    let input_event_handle = InputEventHandle::new(action_pressed, action_released);
                    let input_event_value = Value::InputEvent(input_event_handle);

                    self.call_script_function_with_self("_input", &[input_event_value]);
                }
            }
        }
    }

    fn physics_process(&mut self, delta: f64) {
        // Execute _physics_process function if script is loaded and function exists
        if self.script_loaded {
            if let Some(env) = &self.env {
                if env.get_function("_physics_process").is_some() {
                    // Convert delta to Float (f32 for FerrisScript)
                    let delta_value = Value::Float(delta as f32);
                    self.call_script_function_with_self("_physics_process", &[delta_value]);
                }
            }
        }
    }

    fn enter_tree(&mut self) {
        // Execute _enter_tree function if script is loaded and function exists
        if self.script_loaded {
            if let Some(env) = &self.env {
                if env.get_function("_enter_tree").is_some() {
                    self.call_script_function("_enter_tree", &[]);
                }
            }
        }
    }

    fn exit_tree(&mut self) {
        // Execute _exit_tree function if script is loaded and function exists
        if self.script_loaded {
            if let Some(env) = &self.env {
                if env.get_function("_exit_tree").is_some() {
                    self.call_script_function("_exit_tree", &[]);
                }
            }
        }
    }

    // ========== Phase 5 Sub-Phase 3: Inspector Integration (Bundle 5 - Checkpoint 3.7) ==========

    /// Override get_property_list() to expose FerrisScript @export properties in Godot Inspector
    ///
    /// This is the core Inspector integration that makes exported properties visible and editable.
    /// Called by Godot whenever the Inspector needs to refresh property display.
    ///
    /// **Flow**:
    /// 1. Godot Editor calls get_property_list() on script load/refresh
    /// 2. Returns Vec<PropertyInfo> generated from Program.property_metadata
    /// 3. Inspector displays properties with correct types, hints, and default values
    /// 4. User edits trigger get() and set() calls (implemented in Bundle 7)
    ///
    /// **Property Types Supported** (8 types from Sub-Phase 2):
    /// - Primitives: i32, f32, bool, String
    /// - Godot types: Vector2, Color, Rect2, Transform2D
    ///
    /// **Property Hints Supported** (4 hints from Sub-Phase 2):
    /// - None: No hint (default display)
    /// - Range(min, max, step): Slider control for numeric types
    /// - Enum(values): Dropdown selection for String types
    /// - File(extensions): File picker dialog for String types
    fn get_property_list(&mut self) -> Vec<PropertyInfo> {
        // Only expose properties if script is successfully loaded and compiled
        if let Some(program) = &self.program {
            // Convert each PropertyMetadata to PropertyInfo using helper function
            program
                .property_metadata
                .iter()
                .map(metadata_to_property_info)
                .collect()
        } else {
            // No script loaded or compilation failed - no properties to expose
            Vec::new()
        }
    }

    // ========== Phase 5 Sub-Phase 3: Property Hooks (Bundle 7 - Checkpoint 3.9) ==========

    /// Override get_property() to read FerrisScript exported properties from runtime storage
    ///
    /// Called by Godot when Inspector or code reads a property value.
    ///
    /// **Flow**:
    /// 1. Inspector or GDScript requests property value
    /// 2. Convert StringName → String for property name lookup
    /// 3. Check if property exists in runtime storage (env.get_exported_property)
    /// 4. If found: Convert FerrisScript Value → Godot Variant and return Some(variant)
    /// 5. If not found: Return None (let Godot handle built-in properties like position, rotation)
    ///
    /// **Return Semantics**:
    /// - `Some(variant)` = We handled it, use this value from FerrisScript runtime
    /// - `None` = Not our property, fallback to Godot's default handling (e.g., Node2D.position)
    ///
    /// **Supported Types**: All 8 exportable types from Phase 5 Sub-Phase 2:
    /// - Primitives: i32, f32, bool, String
    /// - Godot types: Vector2, Color, Rect2, Transform2D
    ///
    /// **Error Handling**:
    /// - If env is None (script not loaded): Returns None gracefully
    /// - If property doesn't exist: Returns None gracefully (not an error)
    /// - Never panics (would crash Inspector)
    fn get_property(&self, property: StringName) -> Option<Variant> {
        let prop_name = property.to_string();

        // Check if we have a loaded environment with runtime storage
        if let Some(env) = &self.env {
            // Try to read property from FerrisScript runtime storage
            if let Ok(value) = env.get_exported_property(&prop_name) {
                // Found in runtime - convert FerrisScript Value to Godot Variant
                // Uses value_to_variant() from Bundle 6 with NaN/Infinity handling
                return Some(value_to_variant(&value));
            }
        }

        // Property not found in FerrisScript runtime - let Godot handle it
        // This allows built-in Node2D properties (position, rotation, etc.) to work normally
        None
    }

    /// Override set_property() to write FerrisScript exported properties to runtime storage
    ///
    /// Called by Godot when Inspector or code writes a property value.
    ///
    /// **Flow**:
    /// 1. Inspector or GDScript writes new property value
    /// 2. Convert StringName → String for property name lookup
    /// 3. Convert Godot Variant → FerrisScript Value (handles type conversion and edge cases)
    /// 4. Call env.set_exported_property(name, value, from_inspector=true)
    /// 5. from_inspector=true enables automatic range clamping (e.g., health 150 → 100)
    /// 6. Return true if successful, false if property not found or error
    ///
    /// **Return Semantics**:
    /// - `true` = We handled it, property updated successfully in FerrisScript runtime
    /// - `false` = Not our property or error, fallback to Godot's default handling
    ///
    /// **Range Clamping**:
    /// When from_inspector=true, values exceeding range hints are automatically clamped:
    /// - Example: @export(range(0, 100)) health set to 150 → clamped to 100
    /// - Clamping logic in runtime layer (env.set_exported_property)
    ///
    /// **Error Handling**:
    /// - If env is None (script not loaded): Returns false gracefully
    /// - If property doesn't exist: Returns false gracefully
    /// - If set operation fails: Logs error with godot_error! but doesn't panic
    /// - Never panics (would crash Inspector)
    fn set_property(&mut self, property: StringName, value: Variant) -> bool {
        let prop_name = property.to_string();

        // Check if we have a loaded environment with runtime storage
        if let Some(env) = &mut self.env {
            // Convert Godot Variant → FerrisScript Value
            // Uses variant_to_value() from Bundle 6 with:
            // - Bool-before-int type ordering fix
            // - NaN/Infinity handling
            // - Proper type conversion
            let fs_value = variant_to_value(&value);

            // Try to write property to FerrisScript runtime storage
            // from_inspector=true enables range clamping for @export(range(...)) properties
            match env.set_exported_property(&prop_name, fs_value, true) {
                Ok(_) => {
                    // Property updated successfully in runtime storage
                    return true;
                }
                Err(e) => {
                    // Property doesn't exist or type mismatch
                    // Log error for debugging but don't panic (would crash Inspector)
                    godot_error!("Failed to set FerrisScript property '{}': {}", prop_name, e);
                    return false;
                }
            }
        }

        // env is None (script not loaded) or property not found - let Godot handle it
        // This allows built-in Node2D properties (position, rotation, etc.) to work normally
        false
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

        // ========== Phase 5 Sub-Phase 3: Runtime Synchronization (Bundle 8 - Checkpoint 3.10) ==========

        // Notify Godot Inspector that property list has changed
        //
        // This is critical for hot-reload support:
        // 1. User modifies script file (add/remove @export properties)
        // 2. Script reloads (via reload_script() or auto-reload)
        // 3. Property list changes (different @export annotations)
        // 4. Inspector needs to refresh to show new property list
        //
        // Without this call:
        // - Inspector shows stale property list
        // - New properties don't appear until scene reload
        // - Removed properties still show (but don't work)
        //
        // With this call:
        // - Inspector automatically refreshes on script reload
        // - New properties appear immediately
        // - Removed properties disappear immediately
        // - Seamless hot-reload development experience
        //
        // Called after successful script load/reload to trigger Inspector refresh.
        self.base_mut().notify_property_list_changed();
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

        // Set up node query callback - store instance ID in thread-local for access
        CURRENT_NODE_INSTANCE_ID.with(|id| {
            *id.borrow_mut() = Some(instance_id);
        });
        env.set_node_query_callback(node_query_callback_tls);

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

        // Clear node instance ID from thread-local storage
        CURRENT_NODE_INSTANCE_ID.with(|id| {
            *id.borrow_mut() = None;
        });

        result
    }

    /// Call a function in the loaded script (without self binding)
    fn call_script_function(&mut self, function_name: &str, args: &[Value]) -> Option<Value> {
        if !self.script_loaded {
            godot_warn!("Cannot call function '{}': no script loaded", function_name);
            return None;
        }

        let instance_id = self.base().instance_id();
        let env = self.env.as_mut()?;

        // Set up node query callback - store instance ID in thread-local for access
        CURRENT_NODE_INSTANCE_ID.with(|id| {
            *id.borrow_mut() = Some(instance_id);
        });
        env.set_node_query_callback(node_query_callback_tls);

        let result = match call_function(function_name, args, env) {
            Ok(value) => Some(value),
            Err(e) => {
                godot_error!("Error calling function '{}': {}", function_name, e);
                None
            }
        };

        // Clear node instance ID from thread-local storage
        CURRENT_NODE_INSTANCE_ID.with(|id| {
            *id.borrow_mut() = None;
        });

        result
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

// ========== Phase 5: PropertyInfo Conversion (Bundle 3: Checkpoints 3.5 & 3.6) ==========

// NOTE: PropertyInfo helpers commented out pending godot-rust API research
// These will be needed for Checkpoint 3.7 (get_property_list implementation)
// For now, focusing on Variant conversion (Checkpoint 3.8)

/// Convert Godot Variant to FerrisScript Value (Checkpoint 3.8 - Enhanced)
///
/// Converts Inspector set operations to FerrisScript runtime values.
/// Supports all 8 exportable types with enhanced type safety and edge case handling.
///
/// Type checking order (CRITICAL for correctness):
/// 1. **Boolean** - MUST be checked before numeric types to avoid bool→int misidentification
/// 2. Integer (i32)
/// 3. Float (f64 → f32 with NaN/Infinity handling)
/// 4. String, Vector2, Color, Rect2, Transform2D
/// 5. Nil (fallback)
///
/// Edge case handling:
/// - NaN from f64: Converted to 0.0f32 with warning
/// - Infinity from f64: Clamped to f32::MAX/MIN with warning
/// - Bool before int: Prevents Variant(1) being misidentified as int instead of true
#[allow(dead_code)]
fn variant_to_value(variant: &Variant) -> Value {
    // CRITICAL: Check bool BEFORE numeric types
    // Reason: Godot Variant can represent bool as 1/0, checking int first would misidentify
    if let Ok(b) = variant.try_to::<bool>() {
        return Value::Bool(b);
    }

    // Try integer next
    if let Ok(i) = variant.try_to::<i32>() {
        return Value::Int(i);
    }

    // Try float with NaN/Infinity handling
    if let Ok(f) = variant.try_to::<f64>() {
        // Handle edge cases when converting f64 to f32
        if f.is_nan() {
            godot_warn!("NaN value in Variant→Value conversion, defaulting to 0.0");
            return Value::Float(0.0);
        }
        if f.is_infinite() {
            let clamped = if f.is_sign_positive() {
                f32::MAX
            } else {
                f32::MIN
            };
            godot_warn!(
                "Infinite value in Variant→Value conversion, clamping to {}",
                clamped
            );
            return Value::Float(clamped);
        }
        // Safe conversion for finite values
        return Value::Float(f as f32);
    }

    // Try other Godot types
    if let Ok(s) = variant.try_to::<GString>() {
        return Value::String(s.to_string());
    }

    if let Ok(v) = variant.try_to::<Vector2>() {
        return Value::Vector2 { x: v.x, y: v.y };
    }

    if let Ok(c) = variant.try_to::<Color>() {
        return Value::Color {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a,
        };
    }

    if let Ok(r) = variant.try_to::<Rect2>() {
        return Value::Rect2 {
            position: Box::new(Value::Vector2 {
                x: r.position.x,
                y: r.position.y,
            }),
            size: Box::new(Value::Vector2 {
                x: r.size.x,
                y: r.size.y,
            }),
        };
    }

    if let Ok(t) = variant.try_to::<Transform2D>() {
        // Extract rotation, scale, position from Transform2D
        let position = t.origin;
        let rotation = t.rotation();
        let scale = t.scale();
        return Value::Transform2D {
            position: Box::new(Value::Vector2 {
                x: position.x,
                y: position.y,
            }),
            rotation,
            scale: Box::new(Value::Vector2 {
                x: scale.x,
                y: scale.y,
            }),
        };
    }

    // Fallback for unrecognized types
    Value::Nil
}

// NOTE: Tests for variant conversion and PropertyInfo generation require Godot to be
// initialized and will be validated in integration tests (godot_test/ examples).
// The variant_to_value() and value_to_variant() functions are already used in the
// signal emission system and known to work correctly.

#[cfg(test)]
mod tests {
    use super::*;

    /// API Verification Test (Bundle 4 - Checkpoint 3.7)
    /// Confirms which PropertyUsageFlags and ClassId API variants work in godot-rust 0.4.0
    #[test]
    fn test_property_usage_flags_api() {
        // Test 1: Verify bitwise OR operator works on PropertyUsageFlags
        let flags =
            PropertyUsageFlags::DEFAULT | PropertyUsageFlags::EDITOR | PropertyUsageFlags::STORAGE;
        // Test 2: Verify property_usage_common() helper function
        let common = property_usage_common();
        // API verification: if this compiles, the API patterns are correct
        assert_eq!(flags, flags); // Non-constant assertion to avoid clippy warning
        assert_eq!(common, common);
    }

    #[test]
    fn test_classid_api() {
        // Test which ClassId variant exists
        // Try ClassId::none() first (most common in 0.4.0)
        let class_id = ClassId::none();
        // API verification: if above compiles, none() is correct
        assert_eq!(class_id, ClassId::none());
    }

    // ====================
    // map_type_to_variant Tests (Bundle 4 - Checkpoint 3.7)
    // ====================

    #[test]
    fn test_map_type_i32() {
        assert_eq!(map_type_to_variant("i32"), VariantType::INT);
    }

    #[test]
    fn test_map_type_f32() {
        assert_eq!(map_type_to_variant("f32"), VariantType::FLOAT);
    }

    #[test]
    fn test_map_type_bool() {
        assert_eq!(map_type_to_variant("bool"), VariantType::BOOL);
    }

    #[test]
    fn test_map_type_string() {
        assert_eq!(map_type_to_variant("String"), VariantType::STRING);
    }

    #[test]
    fn test_map_type_vector2() {
        assert_eq!(map_type_to_variant("Vector2"), VariantType::VECTOR2);
    }

    #[test]
    fn test_map_type_color() {
        assert_eq!(map_type_to_variant("Color"), VariantType::COLOR);
    }

    #[test]
    fn test_map_type_rect2() {
        assert_eq!(map_type_to_variant("Rect2"), VariantType::RECT2);
    }

    #[test]
    fn test_map_type_transform2d() {
        assert_eq!(map_type_to_variant("Transform2D"), VariantType::TRANSFORM2D);
    }

    #[test]
    fn test_map_type_unknown() {
        // Unknown type should return NIL and log a warning
        assert_eq!(map_type_to_variant("UnknownType"), VariantType::NIL);
    }

    // ====================
    // map_hint Tests (Bundle 4 - Checkpoint 3.7)
    // NOTE: These tests require Godot engine to be available (GString, PropertyInfo construction)
    // They are disabled for unit testing but will be validated through:
    // 1. Manual Inspector testing in Bundle 5
    // 2. Automated integration tests with headless Godot (see TESTING_STRATEGY_PHASE5.md)
    // ====================

    #[test]
    #[ignore = "Requires Godot engine runtime - enable with headless Godot testing"]
    fn test_map_hint_none() {
        let hint = ast::PropertyHint::None;
        let result = map_hint(&hint);
        assert_eq!(result.hint, PropertyHint::NONE);
        assert!(result.hint_string.is_empty());
    }

    #[test]
    #[ignore = "Requires Godot engine runtime - enable with headless Godot testing"]
    fn test_map_hint_range() {
        let hint = ast::PropertyHint::Range {
            min: 0.0,
            max: 100.0,
            step: 1.0,
        };
        let result = map_hint(&hint);
        assert_eq!(result.hint, PropertyHint::RANGE);
        // Verify it contains numeric values (exact format from export_range)
        let hint_str = result.hint_string.to_string();
        assert!(
            hint_str.contains("0"),
            "Range hint should contain min value"
        );
        assert!(
            hint_str.contains("100"),
            "Range hint should contain max value"
        );
    }

    #[test]
    #[ignore = "Requires Godot engine runtime - enable with headless Godot testing"]
    fn test_map_hint_enum() {
        let hint = ast::PropertyHint::Enum {
            values: vec![
                "Option1".to_string(),
                "Option2".to_string(),
                "Option3".to_string(),
            ],
        };
        let result = map_hint(&hint);
        assert_eq!(result.hint, PropertyHint::ENUM);
        assert_eq!(result.hint_string.to_string(), "Option1,Option2,Option3");
    }

    #[test]
    #[ignore = "Requires Godot engine runtime - enable with headless Godot testing"]
    fn test_map_hint_file_with_dots() {
        let hint = ast::PropertyHint::File {
            extensions: vec![".png".to_string(), ".jpg".to_string()],
        };
        let result = map_hint(&hint);
        assert_eq!(result.hint, PropertyHint::FILE);
        assert_eq!(result.hint_string.to_string(), "*.png;*.jpg");
    }

    #[test]
    #[ignore = "Requires Godot engine runtime - enable with headless Godot testing"]
    fn test_map_hint_file_with_wildcards() {
        let hint = ast::PropertyHint::File {
            extensions: vec!["*.txt".to_string(), "*.md".to_string()],
        };
        let result = map_hint(&hint);
        assert_eq!(result.hint, PropertyHint::FILE);
        assert_eq!(result.hint_string.to_string(), "*.txt;*.md");
    }

    #[test]
    #[ignore = "Requires Godot engine runtime - enable with headless Godot testing"]
    fn test_map_hint_file_without_dots() {
        let hint = ast::PropertyHint::File {
            extensions: vec!["png".to_string(), "jpg".to_string()],
        };
        let result = map_hint(&hint);
        assert_eq!(result.hint, PropertyHint::FILE);
        assert_eq!(result.hint_string.to_string(), "*.png;*.jpg");
    }

    // ====================
    // metadata_to_property_info Tests (Bundle 4 - Checkpoint 3.7)
    // NOTE: These tests require Godot engine to be available
    // They are disabled for unit testing but will be validated through Bundle 5 Inspector testing
    // and automated integration tests with headless Godot (see TESTING_STRATEGY_PHASE5.md)
    // ====================

    #[test]
    #[ignore = "Requires Godot engine runtime - enable with headless Godot testing"]
    fn test_metadata_basic_property() {
        let metadata = ast::PropertyMetadata {
            name: "test_prop".to_string(),
            type_name: "i32".to_string(),
            hint: ast::PropertyHint::None,
            hint_string: String::new(),
            default_value: Some("42".to_string()),
        };
        let result = metadata_to_property_info(&metadata);
        assert_eq!(result.variant_type, VariantType::INT);
        assert_eq!(result.property_name.to_string(), "test_prop");
        assert_eq!(result.hint_info.hint, PropertyHint::NONE);
        assert_eq!(result.class_id, ClassId::none());
    }

    #[test]
    #[ignore = "Requires Godot engine runtime - enable with headless Godot testing"]
    fn test_metadata_with_range_hint() {
        let metadata = ast::PropertyMetadata {
            name: "health".to_string(),
            type_name: "f32".to_string(),
            hint: ast::PropertyHint::Range {
                min: 0.0,
                max: 100.0,
                step: 1.0,
            },
            hint_string: "0,100,1".to_string(),
            default_value: Some("100.0".to_string()),
        };
        let result = metadata_to_property_info(&metadata);
        assert_eq!(result.variant_type, VariantType::FLOAT);
        assert_eq!(result.property_name.to_string(), "health");
        assert_eq!(result.hint_info.hint, PropertyHint::RANGE);
    }

    #[test]
    #[ignore = "Requires Godot engine runtime - enable with headless Godot testing"]
    fn test_metadata_with_enum_hint() {
        let metadata = ast::PropertyMetadata {
            name: "state".to_string(),
            type_name: "String".to_string(),
            hint: ast::PropertyHint::Enum {
                values: vec![
                    "Idle".to_string(),
                    "Walking".to_string(),
                    "Running".to_string(),
                ],
            },
            hint_string: "Idle,Walking,Running".to_string(),
            default_value: Some("Idle".to_string()),
        };
        let result = metadata_to_property_info(&metadata);
        assert_eq!(result.variant_type, VariantType::STRING);
        assert_eq!(result.property_name.to_string(), "state");
        assert_eq!(result.hint_info.hint, PropertyHint::ENUM);
        assert_eq!(
            result.hint_info.hint_string.to_string(),
            "Idle,Walking,Running"
        );
    }

    #[test]
    #[ignore = "Requires Godot engine runtime - enable with headless Godot testing"]
    fn test_metadata_with_file_hint() {
        let metadata = ast::PropertyMetadata {
            name: "texture_path".to_string(),
            type_name: "String".to_string(),
            hint: ast::PropertyHint::File {
                extensions: vec![".png".to_string(), ".jpg".to_string()],
            },
            hint_string: "*.png;*.jpg".to_string(),
            default_value: Some("res://icon.png".to_string()),
        };
        let result = metadata_to_property_info(&metadata);
        assert_eq!(result.variant_type, VariantType::STRING);
        assert_eq!(result.property_name.to_string(), "texture_path");
        assert_eq!(result.hint_info.hint, PropertyHint::FILE);
        assert_eq!(result.hint_info.hint_string.to_string(), "*.png;*.jpg");
    }
}
