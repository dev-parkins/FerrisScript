use godot::prelude::*;
use rustyscript_runtime::Env;

struct RustyScriptExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustyScriptExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct RustyScriptNode {
    base: Base<Node>,
    env: Env,
}

#[godot_api]
impl INode for RustyScriptNode {
    fn init(base: Base<Node>) -> Self {
        RustyScriptNode {
            base,
            env: Env::new(),
        }
    }

    fn ready(&mut self) {
        godot_print!("RustyScript _ready hook - placeholder");
        // TODO: Load and compile .rscr file in Phase 6
    }

    fn process(&mut self, _delta: f64) {
        // TODO: Execute _process function from script in Phase 7
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
