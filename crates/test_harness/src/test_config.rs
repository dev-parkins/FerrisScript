use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for the FerrisScript test harness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    /// Path to Godot executable
    pub godot_executable: PathBuf,
    
    /// Path to Godot project (godot_test/)
    pub project_path: PathBuf,
    
    /// Timeout for individual tests (seconds)
    pub timeout_seconds: u64,
    
    /// Output format (json, console, tap)
    pub output_format: OutputFormat,
    
    /// Verbose output
    pub verbose: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Json,
    Console,
    Tap,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            godot_executable: PathBuf::from(
                r"Y:\cpark\Projects\Godot\Godot_v4.5-dev4_win64.exe\Godot_v4.5-dev4_win64_console.exe"
            ),
            project_path: PathBuf::from("./godot_test"),
            timeout_seconds: 30,
            output_format: OutputFormat::Console,
            verbose: false,
        }
    }
}

impl TestConfig {
    /// Load configuration from TOML file
    pub fn from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config: TestConfig = toml::from_str(&contents)?;
        Ok(config)
    }
    
    /// Load from environment variables (overrides file config)
    pub fn with_env_overrides(mut self) -> Self {
        if let Ok(exe) = std::env::var("GODOT_EXE") {
            self.godot_executable = PathBuf::from(exe);
        }
        if let Ok(project) = std::env::var("GODOT_PROJECT") {
            self.project_path = PathBuf::from(project);
        }
        self
    }
    
    /// Validate configuration
    pub fn validate(&self) -> anyhow::Result<()> {
        if !self.godot_executable.exists() {
            anyhow::bail!("Godot executable not found: {:?}", self.godot_executable);
        }
        if !self.project_path.exists() {
            anyhow::bail!("Project path not found: {:?}", self.project_path);
        }
        Ok(())
    }
}
