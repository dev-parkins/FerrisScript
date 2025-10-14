# FerrisScript Troubleshooting Guide

**Last Updated:** October 2, 2025  
**Version:** v0.0.2

This guide helps you resolve common issues when building, testing, or using FerrisScript.

> **Quick Links:**  
> [Windows Issues](#windows-issues) | [macOS Issues](#macos-issues) | [Linux Issues](#linux-issues) | [Build Errors](#common-build-errors) | [Godot Integration](#godot-integration) | [Runtime Errors](#runtime-errors)

---

## Table of Contents

- [Platform-Specific Issues](#platform-specific-issues)
  - [Windows Issues](#windows-issues)
  - [macOS Issues](#macos-issues)
  - [Linux Issues](#linux-issues)
- [Common Build Errors](#common-build-errors)
- [Godot Integration](#godot-integration)
- [Runtime Errors](#runtime-errors)
- [Getting More Help](#getting-more-help)

---

## Platform-Specific Issues

### Windows Issues

#### Issue: "error: linker `link.exe` not found"

**Cause:** Missing MSVC (Microsoft Visual C++) build tools required by Rust.

**Solution:**

1. **Option A: Install Visual Studio 2019 or later** (Recommended)
   - Download: https://visualstudio.microsoft.com/downloads/
   - Choose "Community Edition" (free)
   - During installation, select:
     - ✅ "Desktop development with C++"
     - ✅ "MSVC v142 or later" build tools
     - ✅ "Windows 10 SDK" or later
   - Restart your computer after installation

2. **Option B: Install Build Tools for Visual Studio** (Lighter)
   - Download: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
   - Run the installer
   - Select "C++ build tools"
   - Install

3. **Verify installation:**

   ```powershell
   # Check if MSVC is available
   where link.exe
   
   # Should output something like:
   # C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Tools\MSVC\...\bin\Hostx64\x64\link.exe
   ```

4. **Rebuild FerrisScript:**

   ```powershell
   cargo clean
   cargo build
   ```

**Alternative:** Use GNU toolchain (not recommended for Godot compatibility)

---

#### Issue: "error: could not find `windows_x86_64_msvc` crate"

**Cause:** Corrupted Rust toolchain or missing Windows target.

**Solution:**

1. Update Rust toolchain:

   ```powershell
   rustup update
   ```

2. Add Windows MSVC target (usually installed by default):

   ```powershell
   rustup target add x86_64-pc-windows-msvc
   ```

3. Clean and rebuild:

   ```powershell
   cargo clean
   cargo build
   ```

---

#### Issue: Build freezes or takes extremely long

**Cause:** Low memory, antivirus interference, or disk space issues.

**Solution:**

1. **Close unnecessary applications** - Cargo build is memory-intensive

2. **Check disk space:**
   - Need ~2GB free for Rust compilation
   - `target/` directory grows large during build

3. **Temporarily disable antivirus** - Windows Defender may scan every compiled file
   - Settings → Virus & threat protection → Exclusions
   - Add: `Y:\cpark\Projects\RustyScript\target`

4. **Use fewer parallel jobs:**

   ```powershell
   cargo build -j 2  # Limit to 2 parallel jobs
   ```

5. **Check RAM usage** (Task Manager → Performance)
   - Minimum 4GB RAM recommended
   - 8GB+ for faster builds

---

#### Issue: "error: command in project.json failed to run"

**Cause:** PowerShell execution policy or path issues.

**Solution:**

1. Check execution policy:

   ```powershell
   Get-ExecutionPolicy
   ```

2. If "Restricted", allow scripts:

   ```powershell
   Set-ExecutionPolicy -Scope CurrentUser RemoteSigned
   ```

3. Verify Rust is in PATH:

   ```powershell
   echo $env:PATH | Select-String cargo
   ```

4. If not found, add manually:
   - Press Win + R, type `sysdm.cpl`
   - Advanced → Environment Variables
   - Add: `C:\Users\YOUR_USERNAME\.cargo\bin`

---

### macOS Issues

#### Issue: "xcrun: error: invalid active developer path"

**Cause:** Missing Xcode Command Line Tools (required for compiling Rust on macOS).

**Solution:**

1. Install Xcode Command Line Tools:

   ```bash
   xcode-select --install
   ```

2. Accept license agreement:

   ```bash
   sudo xcodebuild -license accept
   ```

3. Verify installation:

   ```bash
   xcode-select -p
   # Should output: /Library/Developer/CommandLineTools
   ```

4. Rebuild FerrisScript:

   ```bash
   cargo clean
   cargo build
   ```

**Alternative:** Install full Xcode from App Store (larger download, includes more tools).

---

#### Issue: "ld: library not found for -lSystem"

**Cause:** Incomplete or corrupted Xcode installation.

**Solution:**

1. Reinstall Command Line Tools:

   ```bash
   sudo rm -rf /Library/Developer/CommandLineTools
   xcode-select --install
   ```

2. Reset Xcode developer directory:

   ```bash
   sudo xcode-select --reset
   ```

3. Clean and rebuild:

   ```bash
   cargo clean
   cargo build
   ```

---

#### Issue: "error: linking with `cc` failed"

**Cause:** Incompatible SDK version or missing libraries.

**Solution:**

1. Check macOS SDK:

   ```bash
   xcrun --show-sdk-path
   ```

2. Update Homebrew (if using):

   ```bash
   brew update
   brew upgrade
   ```

3. Install required libraries:

   ```bash
   brew install llvm
   ```

4. Update Rust:

   ```bash
   rustup update
   ```

---

#### Issue: Case-sensitive filesystem issues

**Cause:** macOS uses case-insensitive filesystem by default, but git preserves case.

**Error Example:**

```bash
cd ferrisscript  # ERROR: No such file or directory
```

**Solution:**

Use the correct capitalization:

```bash
cd FerrisScript  # Correct
```

**Note:** This was a bug in earlier README versions (fixed in v0.0.2).

---

### Linux Issues

#### Issue: "error: linking with `cc` failed" or "cannot find -lclang"

**Cause:** Missing build essentials or libclang (required for gdext bindings).

**Solution (Ubuntu/Debian):**

```bash
sudo apt update
sudo apt install build-essential libclang-dev pkg-config
```

**Solution (Fedora/RHEL):**

```bash
sudo dnf install gcc clang-devel pkg-config
```

**Solution (Arch Linux):**

```bash
sudo pacman -S base-devel clang
```

**Verify installation:**

```bash
clang --version
pkg-config --version
```

---

#### Issue: "error: could not compile `gdext`"

**Cause:** Incompatible or missing libclang version.

**Solution:**

1. Install specific libclang version:

   ```bash
   # Ubuntu/Debian
   sudo apt install libclang-14-dev
   
   # Fedora
   sudo dnf install clang-devel
   ```

2. Set LIBCLANG_PATH environment variable:

   ```bash
   export LIBCLANG_PATH=/usr/lib/llvm-14/lib
   cargo build
   ```

3. Make permanent (add to `~/.bashrc` or `~/.zshrc`):

   ```bash
   echo 'export LIBCLANG_PATH=/usr/lib/llvm-14/lib' >> ~/.bashrc
   source ~/.bashrc
   ```

---

#### Issue: "directory not found: ferrisscript"

**Cause:** Case-sensitive filesystem + incorrect directory name.

**Error:**

```bash
cd ferrisscript  # ERROR (lowercase)
```

**Solution:**

Use the correct capitalization:

```bash
cd FerrisScript  # Correct (capital F and S)
```

**Why?** Linux filesystems are case-sensitive, unlike Windows. The repository name is `FerrisScript`.

---

#### Issue: Permission denied errors

**Cause:** Insufficient permissions or incorrect ownership.

**Solution:**

1. **For build directory issues:**

   ```bash
   # Take ownership of target directory
   sudo chown -R $USER:$USER target/
   ```

2. **For cargo cache issues:**

   ```bash
   # Reset cargo permissions
   sudo chown -R $USER:$USER ~/.cargo/
   ```

3. **Never use sudo with cargo:**

   ```bash
   # DON'T do this:
   sudo cargo build  # ❌ Bad - creates permission issues
   
   # Instead:
   cargo build  # ✅ Good
   ```

---

## Common Build Errors

### Error: "rustc version mismatch"

**Cause:** Rust version too old or rustup out of date.

**Solution:**

```bash
# Update Rust to latest stable
rustup update stable

# Verify version (need 1.70+)
rustc --version

# Should output: rustc 1.70.0 or higher
```

---

### Error: "failed to resolve: use of undeclared crate"

**Cause:** Missing dependencies or corrupted Cargo.lock.

**Solution:**

```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Rebuild
cargo build --workspace
```

**If still failing:**

1. Delete Cargo.lock and target directory:

   ```bash
   rm Cargo.lock
   rm -rf target/
   cargo build
   ```

2. Check internet connection (cargo needs to download crates)

---

### Error: "multiple definition of `__getreent`" (Windows)

**Cause:** Conflict between MinGW and MSVC toolchains.

**Solution:**

Use MSVC toolchain consistently:

```powershell
# Set default toolchain
rustup default stable-x86_64-pc-windows-msvc

# Remove MinGW if present
rustup toolchain uninstall stable-x86_64-pc-windows-gnu

# Rebuild
cargo clean
cargo build
```

---

### Error: "error: cannot find macro `gdextension_api` in this scope"

**Cause:** `godot_bind` crate has dependency issues with gdext.

**Solution:**

1. Clean rebuild:

   ```bash
   cargo clean
   cargo build --package ferrisscript_godot_bind
   ```

2. Update gdext dependency (if on main branch):

   ```bash
   cargo update -p gdext
   ```

3. If still failing, check gdext compatibility:
   - FerrisScript v0.0.1 uses gdext v0.1.x
   - Godot 4.2+ required

---

### Error: Build succeeds but tests fail

**Symptoms:**

```bash
cargo test  # Some tests fail
```

**Solution:**

1. **Check if specific crate has issues:**

   ```bash
   cargo test --package rustyscript_compiler  # Test compiler only
   cargo test --package rustyscript_runtime    # Test runtime only
   ```

2. **Run with backtrace for details:**

   ```bash
   RUST_BACKTRACE=1 cargo test
   ```

3. **Check test output:**

   ```bash
   cargo test -- --nocapture  # Show println! output
   ```

4. **Report bug if tests fail on clean install:**
   - [Open an issue](https://github.com/dev-parkins/FerrisScript/issues/new?template=bug_report.md)
   - Include test output and `rustc --version`

---

## Godot Integration

> **Note:** Godot integration testing is partially deferred in v0.0.1. Full automation planned for v0.0.3+. See [FUTURE_AUTOMATION.md](FUTURE_AUTOMATION.md).

### Issue: Godot doesn't recognize `.gdextension` file

**Symptoms:**

- `.ferris` scripts don't appear in Godot
- No "FerrisScript" option in script creation dialog
- Errors in Godot console about missing extension

**Solution:**

1. **Verify `.gdextension` file location:**

   ```
   godot_test/addons/ferrisscript/ferrisscript.gdextension
   ```

2. **Check `.gdextension` syntax:**

   ```ini
   [configuration]
   entry_symbol = "gdext_rust_init"

   [libraries]
   linux.x86_64 = "res://addons/ferrisscript/bin/libferrisscript_godot_bind.so"
   macos.x86_64 = "res://addons/ferrisscript/bin/libferrisscript_godot_bind.dylib"
   windows.x86_64 = "res://addons/ferrisscript/bin/ferrisscript_godot_bind.dll"
   ```

3. **Verify library file exists:**

   ```bash
   # Linux
   ls godot_test/addons/ferrisscript/bin/*.so
   
   # macOS
   ls godot_test/addons/ferrisscript/bin/*.dylib
   
   # Windows
   dir godot_test\addons\ferrisscript\bin\*.dll
   ```

4. **Restart Godot editor** - GDExtensions load at startup

5. **Check Godot version** - Must be 4.2 or later

   ```
   Godot → Help → About
   ```

---

### Issue: "Entry point not found" error in Godot

**Cause:** Library built for wrong platform or architecture.

**Solution:**

1. **Verify your platform:**

   ```bash
   rustc --version --verbose
   # Look for "host:" line
   ```

2. **Build for correct target:**

   ```bash
   # Linux
   cargo build --package ferrisscript_godot_bind --target x86_64-unknown-linux-gnu
   
   # macOS
   cargo build --package ferrisscript_godot_bind --target x86_64-apple-darwin
   
   # Windows
   cargo build --package ferrisscript_godot_bind --target x86_64-pc-windows-msvc
   ```

3. **Rebuild and replace library:**

   ```bash
   cargo build --package ferrisscript_godot_bind --release
   # Copy from target/release/ to godot_test/addons/ferrisscript/bin/
   ```

---

### Issue: `.ferris` scripts cause Godot to crash

**Symptoms:**

- Godot crashes when loading scene with FerrisScript
- "Segmentation fault" or similar errors

**Possible Causes:**

1. **Runtime panic in FerrisScript code**
2. **Invalid Godot API usage**
3. **Memory corruption (rare in v0.0.1)**

**Solution:**

1. **Enable debug logging:**
   - Add `println!` statements in your `.ferris` file
   - Check Godot Output console

2. **Test script in isolation:**

   ```bash
   cargo run --bin rustyscript_runtime your_script.ferris
   ```

3. **Simplify script to minimal reproduction:**

   ```ferris
   fn main() {
       println!("Hello");  // Start simple
   }
   ```

4. **Check Godot console for error messages**

5. **Report bug** if crash persists:
   - [Open issue](https://github.com/dev-parkins/FerrisScript/issues/new?template=bug_report.md)
   - Include crash log, `.ferris` script, Godot version

---

### Issue: Inspector properties don't update when switching from script with type errors

**Status:** ✅ **Fixed in v0.0.5**

**What Was the Problem:**

- Attach a `.ferris` script with a type error (e.g., `@export let mut health: i32 = "Banana";`)
- Godot console shows compilation error (E200: Type mismatch)
- Inspector continued showing stale properties from previous successful compilation
- Switching to a different script didn't clear the stale properties

**How It's Fixed:**

Compilation errors now automatically clear the property list and notify the Inspector to refresh. When a script fails to compile:

1. Internal state is cleared (`program`, `env`, `script_loaded` flag)
2. Inspector is notified via `notify_property_list_changed()`
3. `get_property_list()` returns empty Vec, clearing displayed properties
4. User sees empty Inspector, making it clear the script is broken

**Previous Workaround (v0.0.4 and earlier):**

If you're still on v0.0.4, fix type errors before switching scripts:

   ```ferris
   // ✅ Correct
   @export let mut health: i32 = 100;
   ```

2. **Or manually refresh Inspector:**
   - Click on a different node
   - Click back on the FerrisScriptNode
   - Inspector should now show correct properties

3. **Or reload the scene:**
   - Close and reopen the scene
   - Properties will refresh correctly

**Status:** Known issue in v0.0.4. Will be fixed in v0.0.5 by:

- Clearing property list on compilation failure
- Calling `notify_property_list_changed()` even on error paths
- Improving error state handling in `load_script()`

**Workaround Impact:** Low - only affects development workflow when fixing type errors.

---

## Runtime Errors

### Error: "type mismatch" when running `.ferris` script

**Example:**

```
Error: Expected type i32, found f32
```

**Cause:** Type checker caught an error (working as intended).

**Solution:**

Fix type in your `.ferris` file:

```ferris
// ❌ Wrong
let x: i32 = 3.14;  // Error: f32 assigned to i32

// ✅ Correct
let x: f32 = 3.14;  // Explicit f32
// or
let x = 3;  // Integer literal (i32 by default)
```

---

### Error: "unknown identifier" at runtime

**Cause:** Variable not declared or scope issue.

**Solution:**

Check variable declaration:

```ferris
// ❌ Wrong - variable used before declaration
fn main() {
    println!(x);  // Error: unknown identifier 'x'
    let x = 5;
}

// ✅ Correct
fn main() {
    let x = 5;
    println!(x);
}
```

---

### Error: Script runs but produces wrong output

**Debugging steps:**

1. **Add println! statements:**

   ```ferris
   fn calculate(x: i32) -> i32 {
       println!("Input: {}", x);  // Debug
       let result = x * 2;
       println!("Result: {}", result);  // Debug
       result
   }
   ```

2. **Run tests:**

   ```bash
   cargo test
   ```

3. **Check type coercion:**
   - `i32` → `f32` works (automatic)
   - `f32` → `i32` requires explicit cast (not supported in v0.0.1)

4. **Verify operator precedence:**

   ```ferris
   let x = 2 + 3 * 4;  // Result: 14 (not 20)
   ```

---

## Getting More Help

### Before Asking for Help

1. ✅ Check [FAQ.md](FAQ.md)
2. ✅ Check this troubleshooting guide
3. ✅ Search [existing issues](https://github.com/dev-parkins/FerrisScript/issues)
4. ✅ Try clean rebuild: `cargo clean && cargo build`
5. ✅ Update Rust: `rustup update`

### Where to Get Help

- **Bug Reports:** [Open an issue](https://github.com/dev-parkins/FerrisScript/issues/new?template=bug_report.md)
- **Questions:** [GitHub Discussions Q&A](https://github.com/dev-parkins/FerrisScript/discussions/categories/q-a)
- **Feature Requests:** [Discussions - Ideas](https://github.com/dev-parkins/FerrisScript/discussions/categories/ideas)
- **Documentation Issues:** [Doc template](https://github.com/dev-parkins/FerrisScript/issues/new?template=documentation.md)

### Information to Include in Bug Reports

When reporting issues, include:

1. **Environment:**
   - OS and version (Windows 11, Ubuntu 22.04, macOS 13, etc.)
   - Rust version (`rustc --version`)
   - FerrisScript version or commit hash
   - Godot version (if applicable)

2. **Error message:** Copy full error output

3. **Steps to reproduce:** Minimal example that triggers the issue

4. **Expected vs actual behavior**

5. **Relevant code:** `.ferris` file or build command

---

Made with 🦀 and ❤️ for the Godot community
