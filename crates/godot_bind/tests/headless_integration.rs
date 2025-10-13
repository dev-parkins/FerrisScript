//! Headless Godot Integration Tests for godot_bind
//!
//! Tests GDExtension functions that require Godot engine runtime.
//! Uses the existing test_harness infrastructure and ferris-test.toml configuration.
//!
//! ## Pattern: GDExtension Testing
//!
//! This demonstrates the **GDExtension Testing Pattern** for any crate that needs to test
//! Godot bindings requiring engine initialization (godot::init()).
//!
//! **When to use**: Testing Rust functions that construct Godot types (GString, PropertyInfo, etc.)
//!
//! **How it works**:
//! 1. Create GDScript test runner that calls your GDExtension functions
//! 2. Run tests via existing TestConfig/GodotRunner infrastructure
//! 3. Parse output markers ([PASS]/[FAIL]) for validation
//!
//! See: docs/TESTING_GUIDE.md > "GDExtension Testing Pattern"
//!
//! ## Running Tests
//!
//! ```bash
//! # Uses ferris-test.toml configuration automatically
//! cargo test --package ferrisscript_godot_bind --test headless_integration -- --ignored --nocapture
//! ```
//!
//! ## Configuration
//!
//! Tests use the existing ferris-test.toml at workspace root:
//! - godot_executable: Path to Godot console executable
//! - project_path: godot_test directory
//! - timeout_seconds: Test timeout (default: 30)
//!
//! Override via environment: GODOT_BIN, GODOT_PROJECT_PATH

use ferrisscript_test_harness::{TestConfig, TestOutput};
use std::path::PathBuf;

/// Load test configuration from ferris-test.toml or environment
fn get_test_config() -> Result<TestConfig, String> {
    // Try to load from workspace root
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    let config_path = workspace_root.join("ferris-test.toml");

    let mut config = if config_path.exists() {
        TestConfig::from_file(&config_path)
            .map_err(|e| format!("Failed to load ferris-test.toml: {}", e))?
    } else {
        // Fallback to defaults
        TestConfig::default()
    };

    // Apply environment overrides (GODOT_BIN, etc.)
    config = config.with_env_overrides();

    Ok(config)
}

/// Parse test output for pass/fail results
fn parse_test_results(output: &TestOutput) -> Result<TestResults, String> {
    let mut results = TestResults {
        total: 0,
        passed: 0,
        failed: 0,
        test_names: Vec::new(),
    };

    let combined_output = format!("{}\n{}", output.stdout, output.stderr);

    // Check for test start/end markers
    if !combined_output.contains("[TEST_START]") {
        return Err("Test did not start (missing [TEST_START])".to_string());
    }

    if !combined_output.contains("[TEST_END]") {
        return Err("Test did not complete (missing [TEST_END])".to_string());
    }

    // Parse individual test results
    for line in combined_output.lines() {
        if line.starts_with("[PASS]") {
            let test_name = line
                .strip_prefix("[PASS] ")
                .unwrap_or("unknown")
                .to_string();
            results.test_names.push((test_name, true));
            results.passed += 1;
        } else if line.starts_with("[FAIL]") {
            let test_name = line
                .strip_prefix("[FAIL] ")
                .unwrap_or("unknown")
                .split(" - ")
                .next()
                .unwrap_or("unknown")
                .to_string();
            results.test_names.push((test_name, false));
            results.failed += 1;
        }
    }

    results.total = results.passed + results.failed;

    Ok(results)
}

#[derive(Debug)]
struct TestResults {
    total: u32,
    passed: u32,
    failed: u32,
    test_names: Vec<(String, bool)>, // (name, passed)
}

/// Test basic Godot headless functionality
///
/// This test validates that:
/// - Godot executable is available (from ferris-test.toml)
/// - Godot can run in headless mode
/// - Test scene loads and executes
/// - Basic GDScript functionality works
///
/// Uses existing test_harness infrastructure (TestConfig, GodotRunner)
#[test]
#[ignore = "Requires Godot executable - configure in ferris-test.toml"]
fn test_godot_headless_basic() {
    // Load configuration from ferris-test.toml
    let config = get_test_config().expect("Failed to load test configuration");

    println!("Godot executable: {}", config.godot_executable.display());
    println!("Project path: {}", config.project_path.display());

    // Verify Godot executable exists
    if config.godot_executable.is_absolute() {
        assert!(
            config.godot_executable.exists(),
            "Godot executable not found: {}. Configure in ferris-test.toml or set GODOT_BIN.",
            config.godot_executable.display()
        );
    } else {
        println!(
            "Warning: Using relative Godot path '{}' - assuming it's in PATH",
            config.godot_executable.display()
        );
    }

    // Verify project exists
    assert!(
        config.project_path.exists(),
        "Project not found at: {}",
        config.project_path.display()
    );

    // Create runner from existing test_harness
    let runner = ferrisscript_test_harness::GodotRunner::new(
        config.godot_executable,
        config.project_path,
        config.timeout_seconds,
    );

    // Run test scene
    let test_scene = PathBuf::from("test_godot_bind.tscn");
    let output = runner
        .run_headless(&test_scene)
        .expect("Failed to run Godot test scene");

    println!("\n=== GODOT OUTPUT ===");
    println!("{}", output.stdout);
    if !output.stderr.is_empty() {
        println!("\n=== GODOT STDERR ===");
        println!("{}", output.stderr);
    }
    println!("===================\n");

    // Parse results
    let results = parse_test_results(&output).expect("Failed to parse test output");

    println!("Test Results:");
    println!("  Total: {}", results.total);
    println!("  Passed: {}", results.passed);
    println!("  Failed: {}", results.failed);
    println!("\nIndividual Tests:");
    for (name, passed) in &results.test_names {
        println!("  {} - {}", if *passed { "✓" } else { "✗" }, name);
    }

    // Assert exit code
    assert_eq!(
        output.exit_code, 0,
        "Test scene exited with error code {}",
        output.exit_code
    );

    // Assert no failures
    assert_eq!(results.failed, 0, "{} test(s) failed", results.failed);

    // Assert we ran some tests
    assert!(results.total > 0, "No tests were executed");
}

/// Test that demonstrates the expected workflow for GDExtension tests
///
/// Once FerrisScriptTestNode is added, this test will validate:
/// - map_hint() functions
/// - metadata_to_property_info() function
/// - PropertyInfo construction
/// - GString handling
#[test]
#[ignore = "Requires FerrisScriptTestNode GDExtension implementation"]
fn test_godot_bind_property_info() {
    // This test will be enabled once we add FerrisScriptTestNode
    // to the GDExtension with test methods for:
    // - test_map_hint_range()
    // - test_map_hint_enum()
    // - test_map_hint_file()
    // - test_metadata_to_property_info()

    println!("This test is a placeholder for future GDExtension tests");
    println!("See HEADLESS_GODOT_SETUP.md for implementation plan");
}
