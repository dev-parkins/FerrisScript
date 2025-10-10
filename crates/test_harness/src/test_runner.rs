use crate::{GodotRunner, OutputParser, SceneBuilder, TestConfig, TestOutput};
use std::path::{Path, PathBuf};

/// Orchestrates test execution
pub struct TestHarness {
    config: TestConfig,
    runner: GodotRunner,
    parser: OutputParser,
}

/// Result of a single test execution
#[derive(Debug)]
pub struct TestResult {
    pub script_name: String,
    pub passed: bool,
    pub passed_count: usize,
    pub failed_count: usize,
    pub duration_ms: u64,
    pub output: TestOutput,
    pub markers: Vec<crate::TestMarker>,
}

impl TestHarness {
    pub fn new(config: TestConfig) -> anyhow::Result<Self> {
        config.validate()?;

        let runner = GodotRunner::new(
            config.godot_executable.clone(),
            config.project_path.clone(),
            config.timeout_seconds,
        );

        let parser = OutputParser::new();

        Ok(Self {
            config,
            runner,
            parser,
        })
    }

    /// Run a single .ferris script test
    pub fn run_script(&self, script_path: &Path) -> anyhow::Result<TestResult> {
        let script_name = script_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        println!("Running test: {}", script_name);

        // Read script content to parse scene requirements
        let script_content = std::fs::read_to_string(script_path)?;

        // Build scene dynamically
        let scene_path = self.generate_test_scene(&script_name, &script_content, script_path)?;

        // Run Godot headless
        let start = std::time::Instant::now();
        let output = self.runner.run_headless(&scene_path)?;
        let duration_ms = start.elapsed().as_millis() as u64;

        // Parse results
        let results = self
            .parser
            .parse(&script_name, &output.stdout, &output.stderr);

        // Determine pass/fail
        let passed = results.failed == 0
            && self.parser.script_loaded_successfully(&output.stdout)
            && !self
                .parser
                .has_compilation_errors(&output.stdout, &output.stderr);

        Ok(TestResult {
            script_name,
            passed,
            passed_count: results.passed,
            failed_count: results.failed,
            duration_ms,
            output,
            markers: results.markers,
        })
    }

    /// Generate a test scene for the script
    fn generate_test_scene(
        &self,
        script_name: &str,
        script_content: &str,
        script_path: &Path,
    ) -> anyhow::Result<PathBuf> {
        // Create scenes directory if it doesn't exist
        let scenes_dir = self.config.project_path.join("tests/generated");
        std::fs::create_dir_all(&scenes_dir)?;

        // Generate scene file path
        let scene_filename = format!("test_{}.tscn", script_name.replace(".ferris", ""));
        let scene_path = scenes_dir.join(&scene_filename);

        // Copy script to project scripts directory
        let scripts_dir = self.config.project_path.join("scripts");
        std::fs::create_dir_all(&scripts_dir)?;
        let dest_script = scripts_dir.join(script_name);
        
        // Remove destination if it exists to avoid file lock issues
        if dest_script.exists() {
            let _ = std::fs::remove_file(&dest_script);
        }
        
        std::fs::copy(script_path, &dest_script)?;

        // Build scene based on script requirements
        let script_res_path = format!("res://scripts/{}", script_name);

        let builder = if let Some(mut parsed) =
            crate::scene_builder::parse_scene_requirements(script_content)
        {
            // Scene requirements found - prepend Main node so it's defined before children
            parsed.prepend_script_node("Main", &script_res_path, ".");
            parsed
        } else {
            // Default: simple scene with just Main node
            let mut b = SceneBuilder::new();
            b.add_script_node("Main", &script_res_path, ".");
            b
        };

        // Write scene file
        builder.write_to_file(&scene_path)?;

        if self.config.verbose {
            println!("Generated scene: {:?}", scene_path);
        }

        // Return relative path for Godot
        Ok(PathBuf::from(format!(
            "res://tests/generated/{}",
            scene_filename
        )))
    }

    /// Discover and run all .ferris scripts in a directory
    pub fn run_all_scripts(&self, scripts_dir: &Path) -> anyhow::Result<Vec<TestResult>> {
        let mut results = Vec::new();

        for entry in std::fs::read_dir(scripts_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("ferris") {
                match self.run_script(&path) {
                    Ok(result) => results.push(result),
                    Err(e) => eprintln!("Failed to run {}: {}", path.display(), e),
                }
            }
        }

        Ok(results)
    }

    /// Print summary of test results
    pub fn print_summary(&self, results: &[TestResult]) {
        let total = results.len();
        let passed = results.iter().filter(|r| r.passed).count();
        let failed = total - passed;

        println!("\n========================================");
        println!("Test Summary");
        println!("========================================");
        println!("Total:  {}", total);
        println!("Passed: {} ✓", passed);
        println!("Failed: {} ✗", failed);
        println!("========================================\n");

        // Show failed tests
        if failed > 0 {
            println!("Failed Tests:");
            for result in results.iter().filter(|r| !r.passed) {
                println!("  ✗ {} ({} ms)", result.script_name, result.duration_ms);
                if !result.markers.is_empty() {
                    for marker in &result.markers {
                        if marker.kind == crate::TestMarkerKind::Fail {
                            if let Some(msg) = &marker.message {
                                println!("    - {}", msg);
                            }
                        }
                    }
                }
            }
            println!();
        }
    }
}
