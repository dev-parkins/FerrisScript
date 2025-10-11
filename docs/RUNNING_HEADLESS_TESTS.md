# Running Headless Godot Integration Tests

> **⚠️ ARCHIVAL NOTICE**: This document is preserved for historical reference.
>
> **For Current Testing Practices**: See **`docs/TESTING_GUIDE.md`** (Single Source of Truth)

**Status**: Archival - Superceded by TESTING_GUIDE.md

This guide explained how to run headless Godot integration tests for the `ferrisscript_godot_bind` crate.

**Current Documentation**: See `docs/TESTING_GUIDE.md`:
- "Pattern 3: GDExtension Testing" for the testing pattern
- "Running Tests" section for commands
- "Configuration" section for ferris-test.toml setup
- "Troubleshooting" section for common issues

---

## Historical Content

## Prerequisites

### 1. Install Godot 4.3+

Download Godot from the official website:
- **Windows**: https://godotengine.org/download/windows/
- **Linux**: https://godotengine.org/download/linux/
- **macOS**: https://godotengine.org/download/macos/

You can use either:
- Standard Godot executable (works with `--headless` flag)
- Dedicated headless build (slightly smaller, no GUI support)

### 2. Build the FerrisScript GDExtension

The tests require the compiled GDExtension library:

```bash
cd crates/godot_bind
cargo build --release
```

This creates `ferrisscript.dll` (Windows) or `ferrisscript.so` (Linux) in `target/release/`.

### 3. Set GODOT_BIN Environment Variable

Tell the tests where to find Godot:

**Windows (PowerShell)**:
```powershell
$env:GODOT_BIN = "C:\Path\To\Godot_v4.3-stable_win64.exe"
```

**Windows (CMD)**:
```cmd
set GODOT_BIN=C:\Path\To\Godot_v4.3-stable_win64.exe
```

**Linux/macOS (Bash)**:
```bash
export GODOT_BIN=/usr/local/bin/godot
```

**Permanent Setup (Windows)**:
1. Right-click "This PC" → Properties → Advanced System Settings
2. Click "Environment Variables"
3. Add new User Variable:
   - Name: `GODOT_BIN`
   - Value: `C:\Path\To\Godot_v4.3-stable_win64.exe`

## Running Tests

### Run All Headless Tests

```bash
cargo test --package ferrisscript_godot_bind --test headless_integration -- --ignored --nocapture
```

### Run Specific Test

```bash
cargo test --package ferrisscript_godot_bind --test headless_integration test_godot_headless_basic -- --ignored --nocapture
```

### Run in Verbose Mode

```bash
RUST_LOG=debug cargo test --package ferrisscript_godot_bind --test headless_integration -- --ignored --nocapture
```

## Expected Output

```
running 1 test
Godot executable: C:\Godot\Godot_v4.3-stable_win64.exe
Project path: Y:\cpark\Projects\FerrisScript\godot_test

=== GODOT OUTPUT ===
Godot Engine v4.3.stable.official - https://godotengine.org
[TEST_START] godot_bind_tests

[TEST] basic_node_creation
[ASSERT] Node creation: not null
[ACTUAL] Node creation: not null
[PASS] basic_node_creation

[TEST] property_hint_enum
[ASSERT] PropertyHint.NONE value: expected=0
[ACTUAL] PropertyHint.NONE value: actual=0
[ASSERT] PropertyHint.RANGE value: expected=1
[ACTUAL] PropertyHint.RANGE value: actual=1
[PASS] property_hint_enum

[SUMMARY] Total: 2, Passed: 2, Failed: 0
All tests passed!
[TEST_END]
===================

Test Results:
  Total: 2
  Passed: 2
  Failed: 0

Individual Tests:
  ✓ - basic_node_creation
  ✓ - property_hint_enum

test test_godot_headless_basic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
```

## Troubleshooting

### "Godot executable not found"

**Problem**: Test can't find Godot executable.

**Solutions**:
1. Set `GODOT_BIN` environment variable (see above)
2. Verify path is correct: `Test-Path $env:GODOT_BIN` (PowerShell)
3. Try absolute path instead of relative
4. Check file permissions (Linux/macOS)

### "godot_test project not found"

**Problem**: Test can't find the Godot project.

**Solutions**:
1. Run from repository root: `cd Y:\cpark\Projects\FerrisScript`
2. Verify `godot_test/` directory exists
3. Check that `project.godot` is present

### "Test scene exited with error code 1"

**Problem**: Godot scene crashed or tests failed.

**Solutions**:
1. Check Godot output in stderr for error messages
2. Run with `--nocapture` to see full output
3. Try running scene manually in Godot editor for debugging:
   ```bash
   cd godot_test
   godot --path . --scene test_godot_bind.tscn
   ```

### "GDExtension could not be loaded"

**Problem**: FerrisScript GDExtension failed to load.

**Solutions**:
1. Rebuild GDExtension: `cargo build --release --package ferrisscript_godot_bind`
2. Check `godot_test/ferrisscript.gdextension` configuration
3. Verify library file exists in correct location
4. Check for missing dependencies (Visual C++ Runtime on Windows)

### "No tests were executed"

**Problem**: Test scene ran but no tests were parsed.

**Solutions**:
1. Check for `[TEST_START]` and `[TEST_END]` markers in output
2. Verify `godot_bind_tests.gd` is attached to scene root node
3. Check GDScript syntax errors in Godot output

## Manual Testing

You can also run the test scene manually in Godot:

### With GUI (for debugging)

```bash
cd godot_test
$env:GODOT_BIN --path . --scene test_godot_bind.tscn
```

### Headless (same as automated tests)

```bash
cd godot_test
$env:GODOT_BIN --headless --quit --path . --scene test_godot_bind.tscn
```

## Adding New Tests

See `HEADLESS_GODOT_SETUP.md` for detailed instructions on:
1. Adding test methods to GDScript runner
2. Creating assertion helpers
3. Parsing custom output formats
4. Testing GDExtension functionality

## CI/CD Integration

For automated testing in GitHub Actions, see `.github/workflows/test.yml` (to be added).

Key considerations:
- Use Ubuntu runner for headless Godot
- Cache Godot binary between runs
- Build GDExtension before running tests
- Parse test output for GitHub annotations

## Current Test Status

- ✅ **test_godot_headless_basic**: Validates Godot can run headlessly
- ⏳ **test_godot_bind_property_info**: Awaiting FerrisScriptTestNode implementation

**Next Step**: Add FerrisScriptTestNode GDExtension class with test methods for the 10 ignored unit tests.

## References

- **Setup Guide**: `docs/HEADLESS_GODOT_SETUP.md`
- **Test Harness**: `crates/test_harness/src/godot_cli.rs`
- **GDScript Tests**: `godot_test/scripts/godot_bind_tests.gd`
- **Integration Tests**: `crates/godot_bind/tests/headless_integration.rs`
