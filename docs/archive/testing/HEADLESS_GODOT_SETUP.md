# Headless Godot Testing Setup

> **⚠️ ARCHIVAL NOTICE**: This document is preserved for historical reference.
>
> **For Current Testing Practices**: See **`docs/TESTING_GUIDE.md`** (Single Source of Truth)

**Date**: 2025-10-10  
**Status**: Archival - Superceded by TESTING_GUIDE.md  
**Phase**: Phase 5 Sub-Phase 4 (Historical)

## Overview

This document describes the initial architecture for GDExtension testing. The implementation now uses existing `test_harness` infrastructure and `ferris-test.toml` configuration.

**Current Documentation**: `docs/TESTING_GUIDE.md` > "Pattern 3: GDExtension Testing"

## Problem Statement

The `ferrisscript_godot_bind` crate has 10 tests that are currently ignored because they require Godot engine runtime:

1. `test_map_hint_none` - PropertyHintInfo::NONE construction
2. `test_map_hint_range` - Range hint formatting
3. `test_map_hint_enum` - Enum hint formatting
4. `test_map_hint_file_with_dots` - File hint with `.ext` format
5. `test_map_hint_file_with_wildcards` - File hint with `*.ext` format
6. `test_map_hint_file_without_dots` - File hint normalization
7. `test_metadata_basic_property` - Basic PropertyInfo construction
8. `test_metadata_with_range_hint` - PropertyInfo with range
9. `test_metadata_with_enum_hint` - PropertyInfo with enum
10. `test_metadata_with_file_hint` - PropertyInfo with file hint

**Why They're Ignored**: These tests construct Godot types (`GString`, `PropertyHintInfo`, `PropertyInfo`) which require `godot::init()` to have been called. Standard Rust unit tests don't initialize the Godot engine.

## Solution: Integration Testing with Headless Godot

Instead of unit tests, we use **integration tests** that:

1. Build the FerrisScript GDExtension library
2. Run Godot in headless mode (`--headless`)
3. Load test scenes that exercise the functions
4. Parse output to validate results
5. Exit with appropriate status codes

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Rust Integration Test (crates/godot_bind/tests/...)       │
│  - Uses test_harness::GodotRunner                          │
│  - Builds GDExtension if needed                            │
│  - Runs headless Godot scenes                              │
│  - Parses output for assertions                            │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ↓
┌─────────────────────────────────────────────────────────────┐
│  Godot Headless Runtime                                     │
│  $ godot --headless --quit --path godot_test --scene ...   │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ↓
┌─────────────────────────────────────────────────────────────┐
│  Test Scene (godot_test/test_godot_bind.tscn)             │
│  - FerrisScriptTestNode (GDExtension)                      │
│  - Calls map_hint(), metadata_to_property_info()          │
│  - Prints results in parseable format                      │
└─────────────────────────────────────────────────────────────┘
```

## Implementation Components

### 1. GDExtension Test Node (`crates/godot_bind/src/lib.rs`)

```rust
#[derive(GodotClass)]
#[class(base=Node)]
pub struct FerrisScriptTestNode {
    base: Base<Node>,
}

#[godot_api]
impl FerrisScriptTestNode {
    /// Test map_hint function with Range hint
    #[func]
    fn test_map_hint_range(&self) -> Dictionary {
        let hint = ast::PropertyHint::Range { 
            min: 0.0, max: 100.0, step: 1.0 
        };
        let result = map_hint(&hint);
        
        let mut dict = Dictionary::new();
        dict.set("hint_type", result.hint.ord());
        dict.set("hint_string", result.hint_string);
        dict
    }
    
    // ... similar for other test functions
}
```

### 2. GDScript Test Runner (`godot_test/scripts/godot_bind_tests.gd`)

```gdscript
extends Node

func _ready():
    print("[TEST_START]")
    
    # Test 1: map_hint_range
    var test_node = FerrisScriptTestNode.new()
    var result = test_node.test_map_hint_range()
    
    print("[TEST] test_map_hint_range")
    print("[ASSERT] hint_type == ", PropertyHint.RANGE)
    print("[ACTUAL] hint_type == ", result["hint_type"])
    
    if result["hint_type"] == PropertyHint.RANGE:
        print("[PASS] test_map_hint_range")
    else:
        print("[FAIL] test_map_hint_range")
        get_tree().quit(1)
    
    # ... more tests ...
    
    print("[TEST_END]")
    get_tree().quit(0)
```

### 3. Rust Integration Test (`crates/godot_bind/tests/headless_integration.rs`)

```rust
use ferrisscript_test_harness::GodotRunner;
use std::path::PathBuf;

#[test]
#[ignore = "Requires Godot executable - set GODOT_BIN env var"]
fn test_godot_bind_headless() {
    // Get Godot executable path
    let godot_exe = std::env::var("GODOT_BIN")
        .unwrap_or_else(|_| "godot".to_string());
    
    // Setup runner
    let runner = GodotRunner::new(
        PathBuf::from(godot_exe),
        PathBuf::from("../../godot_test"),
        30, // timeout seconds
    );
    
    // Run test scene
    let output = runner.run_headless(
        &PathBuf::from("test_godot_bind.tscn")
    ).expect("Failed to run Godot test");
    
    // Parse output
    assert!(output.stdout.contains("[TEST_START]"));
    assert!(output.stdout.contains("[PASS] test_map_hint_range"));
    assert!(!output.stdout.contains("[FAIL]"));
    assert_eq!(output.exit_code, 0, "Test scene exited with error");
}
```

### 4. Output Parser Extension (`crates/test_harness/src/output_parser.rs`)

```rust
pub fn parse_godot_test_output(output: &str) -> TestResult {
    let mut passed = Vec::new();
    let mut failed = Vec::new();
    
    for line in output.lines() {
        if line.starts_with("[PASS]") {
            let test_name = line.strip_prefix("[PASS] ").unwrap();
            passed.push(test_name.to_string());
        }
        if line.starts_with("[FAIL]") {
            let test_name = line.strip_prefix("[FAIL] ").unwrap();
            failed.push(test_name.to_string());
        }
    }
    
    TestResult { passed, failed }
}
```

## Directory Structure

```
FerrisScript/
├── crates/
│   ├── godot_bind/
│   │   ├── src/
│   │   │   └── lib.rs  (Add FerrisScriptTestNode)
│   │   └── tests/
│   │       └── headless_integration.rs  (NEW)
│   └── test_harness/
│       └── src/
│           └── output_parser.rs  (Extend)
└── godot_test/
    ├── test_godot_bind.tscn  (NEW)
    └── scripts/
        └── godot_bind_tests.gd  (NEW)
```

## Running Tests

### Locally

```bash
# 1. Build GDExtension
cd crates/godot_bind
cargo build --release

# 2. Set Godot path (Windows)
$env:GODOT_BIN = "C:\Godot\Godot_v4.3-stable_win64.exe"

# 3. Run integration tests
cargo test --test headless_integration -- --ignored

# 4. Or run specific test
cargo test test_godot_bind_headless -- --ignored --nocapture
```

### CI/CD (GitHub Actions)

```yaml
name: Headless Godot Tests

on: [push, pull_request]

jobs:
  godot-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Install Godot Headless
        run: |
          wget https://downloads.tuxfamily.org/godotengine/4.3/Godot_v4.3-stable_linux.x86_64.zip
          unzip Godot_v4.3-stable_linux.x86_64.zip
          chmod +x Godot_v4.3-stable_linux.x86_64
          sudo mv Godot_v4.3-stable_linux.x86_64 /usr/local/bin/godot
      
      - name: Build GDExtension
        run: |
          cd crates/godot_bind
          cargo build --release
      
      - name: Run Headless Tests
        env:
          GODOT_BIN: godot
        run: |
          cargo test --package ferrisscript_godot_bind \
            --test headless_integration -- --ignored
```

## Test Coverage Goals

Once headless testing is set up, we'll have:

### Current (Unit Tests Only)

- ✅ 11 passing unit tests (type mapping, basic functions)
- ❌ 10 ignored tests (require Godot runtime)
- **Coverage**: ~52% (11/21 tests)

### After Headless Setup

- ✅ 11 passing unit tests
- ✅ 10 passing integration tests (headless Godot)
- **Coverage**: 100% (21/21 tests)

## Benefits

1. **Automated Testing**: CI can validate GDExtension functionality
2. **Regression Detection**: Catch PropertyInfo/hint issues before merge
3. **Real Integration**: Tests actual Godot behavior, not mocks
4. **Extensible**: Can add more complex integration scenarios
5. **Cross-Platform**: Works on Windows, Linux, macOS with headless Godot

## Limitations

1. **Requires Godot Install**: CI needs Godot binary
2. **Slower Than Unit Tests**: ~1-2 seconds per test vs <1ms
3. **Build Dependency**: Must build GDExtension before testing
4. **Exit Code Parsing**: Less granular than Rust test framework

## Future Enhancements

1. **Parallel Execution**: Run multiple test scenes concurrently
2. **Test Discovery**: Auto-generate test scenes from Rust test functions
3. **Rich Output**: Structured JSON output from test scenes
4. **Property Hook Tests**: Validate bidirectional sync
5. **Hot-Reload Tests**: Simulate script recompilation in Godot

## References

- **Test Harness**: `crates/test_harness/src/godot_cli.rs`
- **Strategy Doc**: `docs/planning/v0.0.4/TESTING_STRATEGY_PHASE5.md`
- **Integration Report**: `docs/INTEGRATION_TESTS_REPORT.md`
- **Godot Headless Docs**: https://docs.godotengine.org/en/stable/tutorials/editor/command_line_tutorial.html

## Status Tracking

- [ ] Create FerrisScriptTestNode GDExtension class
- [ ] Write GDScript test runner (`godot_bind_tests.gd`)
- [ ] Create test scene (`test_godot_bind.tscn`)
- [ ] Implement Rust integration test
- [ ] Extend output parser for Godot test format
- [ ] Test locally with Windows Godot
- [ ] Configure CI/CD with headless Godot
- [ ] Enable all 10 ignored tests
- [ ] Document test authoring workflow
- [ ] Update test count in README

---

**Next Steps**: Begin implementation with FerrisScriptTestNode
