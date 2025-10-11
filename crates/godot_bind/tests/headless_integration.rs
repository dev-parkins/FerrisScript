//! Headless Godot Integration Tests for godot_bind
//!
//! These tests run actual Godot scenes in headless mode to validate
//! GDExtension functionality that requires the Godot runtime.
//!
//! ## Running Tests
//!
//! ```bash
//! # Set Godot executable path (Windows)
//! $env:GODOT_BIN = "C:\Path\To\Godot_v4.3-stable_win64.exe"
//!
//! # Run tests
//! cargo test --package ferrisscript_godot_bind --test headless_integration -- --ignored --nocapture
//! ```
//!
//! ## Requirements
//!
//! - Godot 4.3+ executable (headless or standard)
//! - GODOT_BIN environment variable set to executable path
//! - FerrisScript GDExtension built (automatic via cargo)

use ferrisscript_test_harness::godot_cli::{GodotRunner, TestOutput};
use std::path::PathBuf;

/// Get Godot executable path from environment or default
fn get_godot_exe() -> PathBuf {
    let godot_path = std::env::var("GODOT_BIN").unwrap_or_else(|_| {
        // Try common paths on Windows
        if cfg!(target_os = "windows") {
            "godot.exe".to_string()
        } else {
            "godot".to_string()
        }
    });

    PathBuf::from(godot_path)
}

/// Get godot_test project path
fn get_project_path() -> PathBuf {
    // From crates/godot_bind/tests/ -> godot_test/
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("godot_test")
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
/// - Godot executable is available
/// - Godot can run in headless mode
/// - Test scene loads and executes
/// - Basic GDScript functionality works
#[test]
#[ignore = "Requires Godot executable - set GODOT_BIN env var"]
fn test_godot_headless_basic() {
    let godot_exe = get_godot_exe();
    let project_path = get_project_path();

    println!("Godot executable: {}", godot_exe.display());
    println!("Project path: {}", project_path.display());

    // Verify Godot executable exists (skip check if relative path - will fail later if wrong)
    if godot_exe.is_absolute() {
        assert!(
            godot_exe.exists(),
            "Godot executable not found: {}. Set GODOT_BIN environment variable.",
            godot_exe.display()
        );
    } else {
        println!(
            "Warning: Using relative Godot path '{}' - assuming it's in PATH",
            godot_exe.display()
        );
    }

    // Verify project exists
    assert!(
        project_path.exists(),
        "godot_test project not found at: {}",
        project_path.display()
    );

    // Create runner
    let runner = GodotRunner::new(godot_exe, project_path, 30);

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
