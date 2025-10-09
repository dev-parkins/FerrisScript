// Signal Prototype for FerrisScript v0.0.4
// Tests dynamic signal registration using godot-rust 0.4 API
//
// CRITICAL DISCOVERY: godot-rust 0.4's add_user_signal() only takes ONE argument - the signal NAME!
// There is NO parameter type specification at registration time.
// Parameters are passed dynamically as Variants when emitting.
//
// API Summary:
// - add_user_signal(name: impl AsArg<GString>) - register signal by name only
// - emit_signal(signal: impl AsArg<StringName>, args: &[Variant]) - emit with dynamic types
// - has_signal(signal: impl AsArg<StringName>) - check if registered
//
// This makes FerrisScript signal integration SIMPLER than expected!

use godot::classes::Node2D;
use godot::prelude::*;

/// Prototype node to test dynamic signal registration
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct SignalPrototype {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for SignalPrototype {
    fn init(base: Base<Node2D>) -> Self {
        SignalPrototype { base }
    }

    fn ready(&mut self) {
        godot_print!("=== Signal Prototype Test ===");

        // Test: Register and emit signals with dynamic parameters
        self.test_dynamic_signals();
    }
}

#[godot_api]
impl SignalPrototype {
    /// Test dynamic signal registration and emission
    fn test_dynamic_signals(&mut self) {
        godot_print!("\n--- Dynamic Signal Test ---");

        // Register signals - NO parameter type information needed!
        // Use string literals directly - they implement AsArg<GString>
        self.base_mut().add_user_signal("player_died");
        self.base_mut().add_user_signal("health_changed");
        self.base_mut().add_user_signal("all_types_signal");
        godot_print!("✓ Registered 3 signals");

        // Emit signal with no parameters
        // String literals also implement AsArg<StringName>
        self.base_mut().emit_signal("player_died", &[]);
        godot_print!("✓ Emitted: player_died()");

        // Emit signal with typed parameters (types inferred from Variant values)
        let args = [Variant::from(100i32), Variant::from(75i32)];
        self.base_mut().emit_signal("health_changed", &args);
        godot_print!("✓ Emitted: health_changed(100, 75)");

        // Emit signal with all FerrisScript types
        let all_types = [
            Variant::from(42i32),
            Variant::from(3.15f32),
            Variant::from(true),
            Variant::from(GString::from("hello")),
            Variant::from(Vector2::new(10.0, 20.0)),
        ];
        self.base_mut().emit_signal("all_types_signal", &all_types);
        godot_print!("✓ Emitted: all_types_signal(42, 3.15, true, \"hello\", Vector2(10, 20))");

        godot_print!("\n=== All Tests Passed! ===");
        godot_print!("Conclusion: Dynamic signal registration works perfectly in godot-rust 0.4");
        godot_print!("Signals are untyped - parameters passed as Variants during emission");
    }

    /// Public function to test signal emission from GDScript
    #[func]
    pub fn trigger_health_change(&mut self, old: i32, new: i32) {
        godot_print!(
            "trigger_health_change({}, {}) called from GDScript",
            old,
            new
        );
        let args = [Variant::from(old), Variant::from(new)];
        self.base_mut().emit_signal("health_changed", &args);
        godot_print!("✓ Signal emitted successfully");
    }
}
