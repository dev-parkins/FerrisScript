use clap::{Arg, ArgAction, Command};
use ferrisscript_test_harness::{TestConfig, TestHarness, TestResult};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let matches = Command::new("ferris-test")
        .version("0.0.3")
        .author("FerrisScript Project")
        .about("Headless test runner for FerrisScript with Godot")
        .arg(
            Arg::new("script")
                .short('s')
                .long("script")
                .value_name("FILE")
                .help("Run a single .ferris script")
                .conflicts_with("all"),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .action(ArgAction::SetTrue)
                .help("Run all .ferris scripts in the project"),
        )
        .arg(
            Arg::new("filter")
                .short('f')
                .long("filter")
                .value_name("PATTERN")
                .help("Filter tests by name pattern (regex)"),
        )
        .arg(
            Arg::new("format")
                .long("format")
                .value_name("FORMAT")
                .default_value("console")
                .value_parser(["console", "json", "tap"])
                .help("Output format for test results"),
        )
        .arg(
            Arg::new("godot")
                .long("godot")
                .value_name("PATH")
                .help("Path to Godot executable (overrides config)"),
        )
        .arg(
            Arg::new("project")
                .long("project")
                .value_name("PATH")
                .help("Path to Godot project directory (overrides config)"),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Path to config file (default: ferris-test.toml)"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("Enable verbose output"),
        )
        .arg(
            Arg::new("scripts-dir")
                .long("scripts-dir")
                .value_name("DIR")
                .help("Directory containing .ferris scripts (default: godot_test/scripts)"),
        )
        .get_matches();

    // Load configuration
    let mut config = if let Some(config_file) = matches.get_one::<String>("config") {
        TestConfig::from_file(&PathBuf::from(config_file))?
    } else {
        // Try default config file, otherwise use defaults
        TestConfig::from_file(&PathBuf::from("ferris-test.toml"))
            .unwrap_or_else(|_| TestConfig::default())
    };

    // Apply CLI overrides
    if let Some(godot_path) = matches.get_one::<String>("godot") {
        config.godot_executable = PathBuf::from(godot_path);
    }
    if let Some(project_path) = matches.get_one::<String>("project") {
        config.project_path = PathBuf::from(project_path);
    }
    if matches.get_flag("verbose") {
        config.verbose = true;
    }
    if let Some(format) = matches.get_one::<String>("format") {
        config.output_format = match format.as_str() {
            "json" => ferrisscript_test_harness::OutputFormat::Json,
            "tap" => ferrisscript_test_harness::OutputFormat::Tap,
            _ => ferrisscript_test_harness::OutputFormat::Console,
        };
    }

    // Apply environment overrides
    config = config.with_env_overrides();

    // Initialize test harness
    let harness = TestHarness::new(config)?;

    // Execute tests
    let results = if let Some(script_path) = matches.get_one::<String>("script") {
        // Single script mode
        vec![harness.run_script(&PathBuf::from(script_path))?]
    } else if matches.get_flag("all") {
        // All scripts mode
        let scripts_dir = matches
            .get_one::<String>("scripts-dir")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("godot_test/scripts"));
        harness.run_all_scripts(&scripts_dir)?
    } else {
        eprintln!("Error: Must specify --script or --all");
        std::process::exit(1);
    };

    // Apply filter if specified
    let results: Vec<TestResult> = if let Some(filter_pattern) = matches.get_one::<String>("filter")
    {
        let regex = regex::Regex::new(filter_pattern)?;
        results
            .into_iter()
            .filter(|r| regex.is_match(&r.script_name))
            .collect()
    } else {
        results
    };

    // Output results based on format
    match matches.get_one::<String>("format").map(|s| s.as_str()) {
        Some("json") => print_json(&results)?,
        Some("tap") => print_tap(&results),
        _ => {
            harness.print_summary(&results);

            // Show detailed output if verbose
            if matches.get_flag("verbose") {
                for result in &results {
                    println!("\n--- {} ---", result.script_name);
                    println!("{}", result.output.stdout);
                    if !result.output.stderr.is_empty() {
                        println!("STDERR:\n{}", result.output.stderr);
                    }
                }
            }
        }
    }

    // Exit with non-zero code if any tests failed
    let failed_count = results.iter().filter(|r| !r.passed).count();
    if failed_count > 0 {
        std::process::exit(1);
    }

    Ok(())
}

fn print_json(results: &[TestResult]) -> anyhow::Result<()> {
    #[derive(serde::Serialize)]
    struct JsonOutput {
        total: usize,
        passed: usize,
        failed: usize,
        tests: Vec<JsonTest>,
    }

    #[derive(serde::Serialize)]
    struct JsonTest {
        name: String,
        passed: bool,
        duration_ms: u64,
        assertions: usize,
        failures: usize,
    }

    let output = JsonOutput {
        total: results.len(),
        passed: results.iter().filter(|r| r.passed).count(),
        failed: results.iter().filter(|r| !r.passed).count(),
        tests: results
            .iter()
            .map(|r| JsonTest {
                name: r.script_name.clone(),
                passed: r.passed,
                duration_ms: r.duration_ms,
                assertions: r.passed_count + r.failed_count,
                failures: r.failed_count,
            })
            .collect(),
    };

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

fn print_tap(results: &[TestResult]) {
    println!("TAP version 13");
    println!("1..{}", results.len());

    for (i, result) in results.iter().enumerate() {
        let status = if result.passed { "ok" } else { "not ok" };
        println!(
            "{} {} - {} ({} ms)",
            status,
            i + 1,
            result.script_name,
            result.duration_ms
        );

        if !result.passed {
            println!("  ---");
            println!("  passed: {}", result.passed_count);
            println!("  failed: {}", result.failed_count);
            println!("  ...");
        }
    }
}
