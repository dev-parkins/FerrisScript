# Phase 4: VS Code Completion - Manual Testing Guide

**Date**: October 7, 2025  
**Phase**: Phase 4 - VS Code Completion  
**Tester**: Developer

---

## 🎯 Testing Objective

Manually verify that the VS Code completion provider works correctly for FerrisScript files, providing context-aware suggestions for keywords, types, and functions.

---

## 🛠️ Setup Instructions

### 1. Install the Extension Locally

```powershell
# From extensions/vscode directory
cd extensions\vscode

# Compile TypeScript (if not already done)
npm run compile

# Copy extension to VS Code extensions folder
# Windows:
$dest = "$env:USERPROFILE\.vscode\extensions\ferrisscript-0.1.0"
Remove-Item -Recurse -Force $dest -ErrorAction SilentlyContinue
Copy-Item -Recurse . $dest -Exclude node_modules,src,out,*.log

# Copy compiled output
Copy-Item -Recurse out $dest\out

# Reload VS Code window
# Press Ctrl+Shift+P, type "Reload Window", and press Enter
```

### 2. Create Test File

```powershell
# Create a test .ferris file
New-Item -ItemType File -Path "test_completion.ferris" -Force
```

---

## ✅ Manual Test Checklist

### Test 1: Keyword Completion at Statement Start

**Steps**:

1. Open `test_completion.ferris`
2. Type `l` at the start of a line
3. Press `Ctrl+Space` to trigger completion

**Expected Results**:

- [ ] Completion menu appears
- [ ] `let` appears in suggestions
- [ ] `let` has detail text: "immutable variable declaration"
- [ ] Documentation shows example usage

**Test Code**:

```ferrisscript
l
```

---

### Test 2: Function Declaration Snippet

**Steps**:

1. Type `fn` at statement start
2. Select `fn` from completion menu or press Tab

**Expected Results**:

- [ ] Snippet expands to: `fn name(params) { ... }`
- [ ] Cursor is at `name` placeholder
- [ ] Pressing Tab moves to `params` placeholder

**Test Code**:

```ferrisscript
fn
```

---

### Test 3: Type Completion After Colon

**Steps**:

1. Type `let x:`
2. Press `Ctrl+Space` or just type

**Expected Results**:

- [ ] Only type completions appear (no keywords)
- [ ] Shows: `i32`, `f32`, `bool`, `String`, `Vector2`, `Node`, `void`
- [ ] Each type has helpful documentation

**Test Code**:

```ferrisscript
let x: 
```

---

### Test 4: Function Completion in Expression

**Steps**:

1. Type `pr` inside a function body
2. Press `Ctrl+Space`

**Expected Results**:

- [ ] `print` appears in suggestions
- [ ] Detail shows: `print(message: String) -> void`
- [ ] Selecting `print` expands to: `print($0)` with cursor inside parentheses

**Test Code**:

```ferrisscript
fn test() {
    pr
}
```

---

### Test 5: Context-Aware Completion (Statement vs Expression)

**Steps**:

1. Type `i` at statement start
2. Note suggestions
3. Type `i` inside an expression (e.g., after `let x =`)
4. Note suggestions

**Expected Results - Statement Start**:

- [ ] Shows: `if`, `while`, `return` (statement-level keywords)

**Expected Results - Expression Context**:

- [ ] Shows: `if`, `true`, `false` (expression keywords)
- [ ] Does NOT show `fn`, `let` (statement-only)

**Test Code**:

```ferrisscript
i

fn test() {
    let x = i
}
```

---

### Test 6: Mut Keyword Completion

**Steps**:

1. Type `let mu` at statement start
2. Press `Ctrl+Space`

**Expected Results**:

- [ ] `mut` appears in suggestions
- [ ] Detail shows: "mutable variable modifier"
- [ ] Documentation explains mutable variables

**Test Code**:

```ferrisscript
let mu
```

---

### Test 7: Boolean Literal Completion

**Steps**:

1. Type `let is_ready: bool = tr` in a file
2. Press `Ctrl+Space`

**Expected Results**:

- [ ] `true` appears in suggestions
- [ ] `false` also available
- [ ] Both have documentation

**Test Code**:

```ferrisscript
let is_ready: bool = tr
```

---

### Test 8: While Loop Snippet

**Steps**:

1. Type `wh` at statement start
2. Select `while` from completion

**Expected Results**:

- [ ] Snippet expands to: `while condition { ... }`
- [ ] Cursor is at `condition` placeholder
- [ ] Pressing Tab moves inside loop body

**Test Code**:

```ferrisscript
wh
```

---

### Test 9: Return Statement Completion

**Steps**:

1. Inside a function, type `ret`
2. Press `Ctrl+Space`

**Expected Results**:

- [ ] `return` appears in suggestions
- [ ] Snippet expands to: `return $0;`
- [ ] Cursor is positioned after `return`

**Test Code**:

```ferrisscript
fn get_value() -> i32 {
    ret
}
```

---

### Test 10: Godot Type Completion

**Steps**:

1. Type `let pos: V` after a colon
2. Press `Ctrl+Space`

**Expected Results**:

- [ ] `Vector2` appears in suggestions
- [ ] Detail shows: "Godot 2D vector type"
- [ ] Documentation mentions `x` and `y` fields

**Test Code**:

```ferrisscript
let pos: V
```

---

## 🐛 Common Issues & Troubleshooting

### Issue: Completion Not Triggering

**Solution**:

- Verify extension is activated: Check "Output" → "Extension Host" for errors
- Reload VS Code window: `Ctrl+Shift+P` → "Reload Window"
- Ensure file has `.ferris` extension

### Issue: No Type Completions After Colon

**Solution**:

- Check context detection logic in `src/utils/context.ts`
- Verify regex pattern `/:\s*$/` matches your cursor position
- Add space after colon if needed

### Issue: Extension Not Found

**Solution**:

- Verify extension copied to correct folder:
  - Windows: `%USERPROFILE%\.vscode\extensions\ferrisscript-0.1.0`
  - Linux/macOS: `~/.vscode/extensions/ferrisscript-0.1.0`
- Check folder contains `package.json` and `out/extension.js`

---

## 📊 Test Results Summary

**Test Date**: ___________  
**Tester**: ___________  
**Extension Version**: 0.1.0  
**VS Code Version**: ___________

| Test # | Test Name | Pass/Fail | Notes |
|--------|-----------|-----------|-------|
| 1 | Keyword Completion at Statement Start | | |
| 2 | Function Declaration Snippet | | |
| 3 | Type Completion After Colon | | |
| 4 | Function Completion in Expression | | |
| 5 | Context-Aware Completion | | |
| 6 | Mut Keyword Completion | | |
| 7 | Boolean Literal Completion | | |
| 8 | While Loop Snippet | | |
| 9 | Return Statement Completion | | |
| 10 | Godot Type Completion | | |

**Overall Result**: ☐ All Pass | ☐ Some Failures | ☐ Major Issues

---

## 📝 Additional Notes

*Use this space to document any unexpected behavior, suggestions, or improvements:*

---

## ✅ Sign-Off

**Tested By**: ___________  
**Date**: ___________  
**Approved for PR**: ☐ Yes | ☐ No (see issues above)

---

**Next Steps After Testing**:

1. If all tests pass → Proceed to create PR
2. If issues found → Document in GitHub issue, fix, retest
3. Update Phase 4 status in `docs/planning/v0.0.3/README.md`
4. Merge to `develop` branch for integration testing
