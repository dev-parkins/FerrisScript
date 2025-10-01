use gdnative::prelude::*;
use rustyscript_runtime::Env;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct RustyScriptNode {
    env: Env,
}

#[methods]
impl RustyScriptNode {
    fn new(_owner: &Node) -> Self {
        RustyScriptNode {
            env: Env::new(),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        godot_print!("RustyScript _ready hook - placeholder");
        // TODO: Load and compile .rscr file in Phase 6
    }

    #[export]
    fn _process(&mut self, _owner: &Node, _delta: f64) {
        // TODO: Execute _process function from script in Phase 7
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<RustyScriptNode>();
}

godot_init!(init);

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
