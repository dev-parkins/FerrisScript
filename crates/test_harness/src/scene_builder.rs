use std::path::Path;

/// Builds Godot scene (.tscn) files dynamically for testing
pub struct SceneBuilder {
    root_name: String,
    root_type: String,
    nodes: Vec<NodeConfig>,
    script_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub name: String,
    pub node_type: String,
    pub parent: String,
    pub script_path: Option<String>,
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self {
            root_name: "TestRunner".to_string(),
            root_type: "Node2D".to_string(),
            nodes: Vec::new(),
            script_path: None,
        }
    }
    
    /// Set the root node configuration
    pub fn with_root(mut self, name: &str, node_type: &str) -> Self {
        self.root_name = name.to_string();
        self.root_type = node_type.to_string();
        self
    }
    
    /// Add a child node
    pub fn add_node(&mut self, name: &str, node_type: &str, parent: &str) -> &mut Self {
        self.nodes.push(NodeConfig {
            name: name.to_string(),
            node_type: node_type.to_string(),
            parent: parent.to_string(),
            script_path: None,
        });
        self
    }
    
    /// Add a node with a script attached
    pub fn add_script_node(&mut self, name: &str, script_path: &str, parent: &str) -> &mut Self {
        self.nodes.push(NodeConfig {
            name: name.to_string(),
            node_type: "FerrisScriptNode".to_string(),
            parent: parent.to_string(),
            script_path: Some(script_path.to_string()),
        });
        self
    }
    
    /// Add a node with a script attached at the beginning (before any existing nodes)
    pub fn prepend_script_node(&mut self, name: &str, script_path: &str, parent: &str) -> &mut Self {
        self.nodes.insert(0, NodeConfig {
            name: name.to_string(),
            node_type: "FerrisScriptNode".to_string(),
            parent: parent.to_string(),
            script_path: Some(script_path.to_string()),
        });
        self
    }
    
    /// Attach a script to the root node
    pub fn with_script(mut self, script_path: &str) -> Self {
        self.script_path = Some(script_path.to_string());
        self
    }
    
    /// Generate the .tscn file content
    pub fn build(&self) -> String {
        let mut tscn = String::new();
        
        // Header
        tscn.push_str("[gd_scene format=3]\n\n");
        
        // Root node
        tscn.push_str(&format!("[node name=\"{}\" type=\"{}\"]\n", self.root_name, self.root_type));
        if let Some(ref script) = self.script_path {
            tscn.push_str(&format!("script_path = \"{}\"\n", script));
        }
        tscn.push('\n');
        
        // Child nodes
        for node in &self.nodes {
            tscn.push_str(&format!("[node name=\"{}\" type=\"{}\" parent=\"{}\"]\n", 
                node.name, node.node_type, node.parent));
            if let Some(ref script) = node.script_path {
                tscn.push_str(&format!("script_path = \"{}\"\n", script));
            }
            tscn.push('\n');
        }
        
        tscn
    }
    
    /// Write the scene to a file
    pub fn write_to_file(&self, path: &Path) -> std::io::Result<()> {
        std::fs::write(path, self.build())
    }
}

impl Default for SceneBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse scene requirements from .ferris file comments
pub fn parse_scene_requirements(ferris_content: &str) -> Option<SceneBuilder> {
    // Look for GODOT SCENE SETUP section in comments
    let mut in_setup_section = false;
    let mut builder = SceneBuilder::new();
    let mut main_node_found = false;
    
    for line in ferris_content.lines() {
        let trimmed = line.trim_start_matches("//").trim();
        
        if trimmed.contains("GODOT SCENE SETUP") || trimmed.contains("Godot Scene Tree Setup") {
            in_setup_section = true;
            continue;
        }
        
        if in_setup_section {
            // Stop at end of comment block
            if !line.trim_start().starts_with("//") {
                break;
            }
            
            // Parse node hierarchy
            if trimmed.contains("└─ Main") || trimmed.contains("Main (attach this script here)") {
                main_node_found = true;
                // Main will be added by test_runner as a child of TestRunner
            } else if trimmed.contains("├─") || trimmed.contains("│") {
                // Extract node name
                if let Some(node_name) = extract_node_name(trimmed) {
                    let parent = if trimmed.contains("│    ") {
                        // Nested under another node
                        "Main/UI"
                    } else {
                        "Main"
                    };
                    builder.add_node(&node_name, "Node2D", parent);
                }
            }
        }
    }
    
    if main_node_found {
        Some(builder)
    } else {
        None
    }
}

fn extract_node_name(line: &str) -> Option<String> {
    // Remove tree characters and extract node name
    let cleaned = line
        .replace("├─", "")
        .replace("│", "")
        .replace("└─", "")
        .replace("(attach this script here)", "")
        .replace("(optional)", "")
        .replace("(required)", "")
        .trim()
        .to_string();
    
    if !cleaned.is_empty() && !cleaned.contains("/root") {
        Some(cleaned)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_scene_generation() {
        let mut builder = SceneBuilder::new();
        builder.add_node("Player", "Node2D", "TestRunner");
        
        let tscn = builder.build();
        assert!(tscn.contains("[gd_scene format=3]"));
        assert!(tscn.contains("name=\"TestRunner\""));
        assert!(tscn.contains("name=\"Player\""));
    }
    
    #[test]
    fn test_scene_with_script() {
        let builder = SceneBuilder::new()
            .with_script("res://scripts/test.ferris");
        
        let tscn = builder.build();
        assert!(tscn.contains("script_path = \"res://scripts/test.ferris\""));
    }
}
