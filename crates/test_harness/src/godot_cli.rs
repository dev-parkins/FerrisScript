use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Manages Godot CLI execution for testing
pub struct GodotRunner {
    pub godot_exe: PathBuf,
    pub project_path: PathBuf,
    pub timeout: Duration,
}

/// Output captured from a Godot test run
#[derive(Debug)]
pub struct TestOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub duration: Duration,
}

impl GodotRunner {
    pub fn new(godot_exe: PathBuf, project_path: PathBuf, timeout_secs: u64) -> Self {
        Self {
            godot_exe,
            project_path,
            timeout: Duration::from_secs(timeout_secs),
        }
    }
    
    /// Run a scene headlessly and capture output
    pub fn run_headless(&self, scene_path: &Path) -> anyhow::Result<TestOutput> {
        let start = Instant::now();
        
        // Build Godot command
        let mut cmd = Command::new(&self.godot_exe);
        cmd.arg("--headless")
           .arg("--quit")
           .arg("--path")
           .arg(&self.project_path)
           .arg("--scene")
           .arg(scene_path)
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());
        
        // Execute
        let output = cmd.output()?;
        let duration = start.elapsed();
        
        // Parse output
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);
        
        Ok(TestOutput {
            stdout,
            stderr,
            exit_code,
            duration,
        })
    }
    
    /// Run a scene with GUI (for debugging)
    pub fn run_with_gui(&self, scene_path: &Path) -> anyhow::Result<TestOutput> {
        let start = Instant::now();
        
        let mut cmd = Command::new(&self.godot_exe);
        cmd.arg("--path")
           .arg(&self.project_path)
           .arg("--scene")
           .arg(scene_path)
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());
        
        let output = cmd.output()?;
        let duration = start.elapsed();
        
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);
        
        Ok(TestOutput {
            stdout,
            stderr,
            exit_code,
            duration,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_godot_runner_creation() {
        let runner = GodotRunner::new(
            PathBuf::from("godot.exe"),
            PathBuf::from("./project"),
            30
        );
        assert_eq!(runner.timeout, Duration::from_secs(30));
    }
}
