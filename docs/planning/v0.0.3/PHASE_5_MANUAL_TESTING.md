# Phase 5: Manual Testing Guide - VS Code Hover & Problem Panel

**Date**: October 7, 2025  
**Phase**: Phase 5 - VS Code Hover & Problem Panel  
**Tester**: *(To be filled)*  
**Extension Version**: v0.0.3

---

## 🎯 Testing Overview

This guide provides comprehensive test cases for Phase 5 features:

- Hover tooltips (keywords, types, functions)
- Problem panel integration (error diagnostics)
- File icons

**Testing Approach**: Manual testing with clear expected results for each test case.

---

## ⚙️ Setup Instructions

### Prerequisites

1. **Build Extension**:

   ```bash
   cd extensions/vscode
   npm run compile
   ```

2. **Install Extension**:
   - Copy `extensions/vscode` to `%USERPROFILE%\.vscode\extensions\ferrisscript-0.0.3`
   - OR press F5 in VS Code to launch Extension Development Host

3. **Prepare Test File**:
   - Create `test_hover.ferris` in workspace
   - Create `test_errors.ferris` in workspace

4. **Note on Diagnostic Tests** (Tests 8-12):
   - ⚠️ **Known Limitation**: FerrisScript currently has no standalone CLI executable
   - The diagnostic provider infrastructure is in place but requires a CLI to function
   - Tests 8-12 (Problem Panel) will not work until CLI is implemented
   - **Skip Tests 8-12** for this phase - mark as "Not Testable" in results
   - CLI implementation is planned for a future phase

---

## 📋 Test Cases

### Test 1: Keyword Hover - `let`

**Objective**: Verify hover tooltip shows for `let` keyword

**Steps**:

1. Open `test_hover.ferris`
2. Type: `let speed: f32 = 100.0;`
3. Hover cursor over the word `let`

**Expected Result**:

- Hover tooltip appears
- Shows: **`let`** - Declares an immutable variable
- Shows syntax: `let name: type = value;`
- Shows example code block with syntax highlighting

**Test Code**:

```ferrisscript
let speed: f32 = 100.0;
```

**Result**: [X] Pass [ ] Fail

**Notes**:

---

### Test 2: Keyword Hover - All Keywords

**Objective**: Verify hover works for all 9 keywords

**Steps**:

1. Type code with all keywords: `let`, `mut`, `fn`, `if`, `else`, `while`, `return`, `true`, `false`
2. Hover over each keyword one by one

**Expected Result**:

- Each keyword shows appropriate hover tooltip
- All tooltips have description, syntax, and example

**Test Code**:

```ferrisscript
let mut count: i32 = 0;

fn test() -> bool {
    if count > 0 {
        return true;
    } else {
        while count < 10 {
            count += 1;
        }
        return false;
    }
}
```

**Keywords to Test**:

- [X] `let` - Shows immutable variable declaration info
- [X] `mut` - Shows mutable variable info
- [X] `fn` - Shows function declaration info
- [X] `if` - Shows conditional statement info
- [X] `else` - Shows alternative branch info
- [X] `while` - Shows loop info
- [X] `return` - Shows return statement info
- [X] `true` - Shows boolean literal info
- [X] `false` - Shows boolean literal info

**Result**: [X] Pass [ ] Fail

**Notes**:

---

### Test 3: Type Hover - Primitives

**Objective**: Verify hover shows for primitive types

**Steps**:

1. Type code with primitive types
2. Hover over each type: `i32`, `f32`, `bool`, `String`

**Expected Result**:

- Each type shows hover tooltip
- Shows type category (Primitive Type)
- Shows description and example

**Test Code**:

```ferrisscript
let count: i32 = 42;
let speed: f32 = 100.5;
let is_active: bool = true;
let name: String = "Player";
```

**Types to Test**:

- [X] `i32` - Shows "32-bit signed integer"
- [X] `f32` - Shows "32-bit floating point number"
- [X] `bool` - Shows "Boolean value (true or false)"
- [X] `String` - Shows "Text string"

**Result**: [X] Pass [ ] Fail

**Notes**:

---

### Test 4: Type Hover - Godot Types

**Objective**: Verify hover shows for Godot types

**Steps**:

1. Type code with Godot types
2. Hover over each type: `Vector2`, `Node`, `void`

**Expected Result**:

- Each type shows hover tooltip
- `Vector2` and `Node` show "Godot Type" category
- `void` shows "Special Type" category

**Test Code**:

```ferrisscript
let position: Vector2 = Vector2(10.0, 20.0);

fn get_parent() -> Node {
    return self.get_parent();
}

fn update(delta: f32) -> void {
    // No return
}
```

**Types to Test**:

- [X] `Vector2` - Shows "2D vector (x, y coordinates)"
- [X] `Node` - Shows "Base Godot scene node"
- [X] `void` - Shows "No return value (used for functions)"

**Result**: [X] Pass [ ] Fail

**Notes**:

---

### Test 5: Function Hover - `print`

**Objective**: Verify hover shows function signature and parameters

**Steps**:

1. Type code with `print` function call
2. Hover over `print`

**Expected Result**:

- Shows function signature: `print(message: String) -> void`
- Shows description: "Prints a message to the console"
- Shows parameter: `message (String): The message to print`
- Shows return type: `void`
- Shows example code

**Test Code**:

```ferrisscript
print("Hello, World!");
```

**Result**: [X] Pass [ ] Fail

**Notes**:

---

### Test 6: Hover Format Quality

**Objective**: Verify hover content is well-formatted

**Steps**:

1. Hover over any keyword, type, or function
2. Inspect hover tooltip formatting

**Expected Result**:

- Markdown formatting is correct (bold, inline code, code blocks)
- Code examples have syntax highlighting (ferrisscript language)
- No raw Markdown syntax visible (e.g., no `**`, no ` ``` `)
- Content is readable and professional

**Test Code**: (Use any code from previous tests)

**Result**: [X] Pass [ ] Fail

**Notes**:

---

### Test 7: Hover Negative Cases

**Objective**: Verify hover does NOT show for non-hover targets

**Steps**:

1. Hover over whitespace
2. Hover over operators (`+`, `=`, etc.)
3. Hover over numbers (`100`)
4. Hover over strings (`"text"`)
5. Hover over comments (`// comment`)

**Expected Result**:

- No hover tooltip appears for any of these
- Hover only works on keywords, types, and functions

**Test Code**:

```ferrisscript
// This is a comment
let count: i32 = 100 + 50;
let name: String = "Player";
```

**Cases to Test**:

- [X] Whitespace - No hover
- [X] Operators (`+`, `=`) - No hover
- [X] Numbers (`100`, `50`) - No hover
- [X] Strings (`"Player"`) - No hover
- [X] Comments (`// This is a comment`) - No hover

**Result**: [X] Pass [ ] Fail

**Notes**:

---

### Test 8: Problem Panel - Undefined Variable Error

**Objective**: Verify compiler errors appear in problem panel

**⚠️ SKIP**: Requires standalone CLI executable (not yet implemented)

**Steps**:

1. Create `test_errors.ferris`
2. Type code with undefined variable
3. Save file (Ctrl+S)
4. Open VS Code Problems panel (Ctrl+Shift+M)

**Expected Result** (when CLI available):

- Problem panel shows 1 error
- Error message: `[E201] Undefined variable 'velocty'`
- Error source: `ferrisscript`
- Clicking error jumps to line 2, column 10

**Test Code**:

```ferrisscript
fn update(delta: f32) -> void {
    print(velocty); // Typo: should be 'velocity'
}
```

**Result**: [N/A] Not Testable (CLI not implemented)

**Notes**: Diagnostic provider infrastructure is in place. Will work once CLI is added.

---

### Test 9: Problem Panel - Inline Squiggles

**Objective**: Verify errors show as red squiggles inline

**⚠️ SKIP**: Requires standalone CLI executable (not yet implemented)

**Steps**:

1. Use same `test_errors.ferris` from Test 8
2. After saving, look at line 2 in editor

**Expected Result** (when CLI available):

- Red squiggle appears under `velocty`
- Hovering over squiggle shows error message
- Squiggle length approximately matches word length

**Test Code**: (Same as Test 8)

**Result**: [N/A] Not Testable (CLI not implemented)

**Notes**: Diagnostic provider infrastructure is in place. Will work once CLI is added.

---

### Test 10: Problem Panel - Error Clearing

**Objective**: Verify errors clear when fixed

**⚠️ SKIP**: Requires standalone CLI executable (not yet implemented)

**Steps**:

1. Use `test_errors.ferris` with error
2. Save file - verify error appears
3. Fix error (add `let velocity: f32 = 100.0;` before `print`)
4. Save file again

**Expected Result** (when CLI available):

- After step 2: Error appears in problem panel and as squiggle
- After step 4: Error disappears from problem panel and squiggle removed

**Test Code**:

```ferrisscript
// Before fix (error)
fn update(delta: f32) -> void {
    print(velocty);
}

// After fix (no error)
fn update(delta: f32) -> void {
    let velocity: f32 = 100.0;
    print(velocity);
}
```

**Result**: [N/A] Not Testable (CLI not implemented)

**Notes**: Diagnostic provider infrastructure is in place. Will work once CLI is added.

---

### Test 11: Problem Panel - Multiple Errors

**Objective**: Verify multiple errors shown correctly

**⚠️ SKIP**: Requires standalone CLI executable (not yet implemented)

**Steps**:

1. Type code with 3 different errors
2. Save file
3. Check problem panel

**Expected Result** (when CLI available):

- Problem panel shows 3 errors
- Each error has correct line number
- Each error has error code (E201, E100, etc.)

**Test Code**:

```ferrisscript
fn test() -> void {
    print(undefined1); // Error 1: E201 Undefined variable
    let x: i32 = "text"; // Error 2: E200 Type mismatch
    let y: = 10; // Error 3: E100 Syntax error (missing type)
}
```

**Result**: [N/A] Not Testable (CLI not implemented)

**Notes**: Diagnostic provider infrastructure is in place. Will work once CLI is added.

---

### Test 12: Problem Panel - No Compiler Available

**Objective**: Verify graceful handling when compiler not found

**Status**: ✅ **VERIFIED** - This is the current state (no CLI exists)

**Steps**:

1. Compiler not available (current state)
2. Open `.ferris` file and save
3. Check problem panel
4. Test hover and completion features

**Expected Result**:

- No errors appear (diagnostics disabled)
- No error dialogs or crashes
- Extension continues to work (hover, completion still functional)

**Result**: [X] Pass [ ] Fail

**Notes**: Extension loads successfully without compiler. Hover and completion work as expected.

---

### Test 13: File Icon Display

**Objective**: Verify `.ferris` files show custom icon

**Steps**:

1. Create multiple `.ferris` files in workspace
2. View file explorer (Ctrl+Shift+E)
3. Check icon next to `.ferris` files

**Expected Result**:

- `.ferris` files show custom FerrisScript icon (crab with Godot accent)
- Icon is distinct from generic text file icon
- Icon appears for all `.ferris` files

**Test Files**:

- `test1.ferris`
- `test2.ferris`
- `example.ferris`

**Result**: [ ] Pass [ ] Fail

**Notes**:

---

### Test 14: Hover Performance

**Objective**: Verify hover response time is acceptable

**Steps**:

1. Open file with many keywords and types
2. Quickly hover over multiple items
3. Measure approximate response time

**Expected Result**:

- Hover tooltip appears within 100ms
- No noticeable lag or delay
- Smooth user experience

**Test Code**: (Use any code with 10+ keywords/types)

**Result**: [ ] Pass [ ] Fail

**Performance Notes**:

- Approximate hover response time: _______ ms
- Noticeable lag: Yes / No

---

### Test 15: Integration - All Features Together

**Objective**: Verify all Phase 5 features work together

**Steps**:

1. Open `.ferris` file with icon visible
2. Type code with keywords, types, functions
3. Test hover on multiple elements
4. Introduce error and save
5. Check problem panel
6. Fix error and verify it clears
7. Test completion (Phase 4 regression test)

**Expected Result**:

- All features work simultaneously
- No conflicts between hover and diagnostics
- Completion still works (Phase 4 regression)
- Extension feels cohesive and professional

**Test Code**:

```ferrisscript
fn _ready() -> void {
    let position: Vector2 = Vector2(0.0, 0.0);
    print("Ready!");
}

fn update(delta: f32) -> void {
    // Test hover on keywords, types
    let speed: f32 = 100.0;
    
    // Test error (typo)
    print(spedd); // Should show error
}
```

**Result**: [ ] Pass [ ] Fail

**Notes**:

---

## 📊 Test Results Summary

**Date Tested**: ____________  
**Tester**: ____________  
**Extension Version**: v0.0.3

| Test # | Test Name | Result | Notes |
|--------|-----------|--------|-------|
| 1 | Keyword Hover - `let` | | |
| 2 | Keyword Hover - All Keywords | | |
| 3 | Type Hover - Primitives | | |
| 4 | Type Hover - Godot Types | | |
| 5 | Function Hover - `print` | | |
| 6 | Hover Format Quality | | |
| 7 | Hover Negative Cases | | |
| 8 | Problem Panel - Undefined Variable | | |
| 9 | Problem Panel - Inline Squiggles | | |
| 10 | Problem Panel - Error Clearing | | |
| 11 | Problem Panel - Multiple Errors | | |
| 12 | Problem Panel - No Compiler | | |
| 13 | File Icon Display | | |
| 14 | Hover Performance | | |
| 15 | Integration - All Features | | |

**Overall Pass Rate**: _____ / 15 tests passed

---

## 🐛 Issues Found

### Issue 1

**Test Case**: ____________  
**Description**: ____________  
**Severity**: High / Medium / Low  
**Expected**: ____________  
**Actual**: ____________

---

### Issue 2

*(Add more as needed)*

---

## ✅ Acceptance Criteria Verification

Based on [PHASE_5_VS_CODE_HOVER.md](./PHASE_5_VS_CODE_HOVER.md) acceptance criteria:

- [ ] **Criterion 1**: Keyword hover works (Test 1-2)
- [ ] **Criterion 2**: Type hover shows info (Test 3-4)
- [ ] **Criterion 3**: Function hover shows signature (Test 5)
- [ ] **Criterion 4**: Problem panel shows errors (Test 8, 11)
- [ ] **Criterion 5**: Inline error squiggles (Test 9)
- [ ] **Criterion 6**: File icon displays (Test 13)
- [ ] **Criterion 7**: Hover format is professional (Test 6)
- [ ] **Criterion 8**: Diagnostics clear on fix (Test 10)
- [ ] **Criterion 9**: Extension compiles and loads (Pre-test setup)
- [ ] **Criterion 10**: Documentation is updated (Verified separately)

**All Acceptance Criteria Met**: [ ] Yes [ ] No

---

## 📝 Additional Notes

*(Use this space for any observations, suggestions, or feedback)*

---

## 🔄 Next Steps

**If All Tests Pass**:

1. Update `PHASE_5_VS_CODE_HOVER.md` with test results
2. Mark Phase 5 as complete in roadmap documents
3. Create PR for Phase 5
4. Proceed to Phase 6 (Development Scripts)

**If Issues Found**:

1. Document issues in detail (use template above)
2. Prioritize issues (High/Medium/Low)
3. Create `PHASE_5_FIXES_VALIDATION.md` if needed
4. Fix critical issues before PR
5. Document known minor issues in README if acceptable

---

## 📚 References

- [PHASE_5_VS_CODE_HOVER.md](./PHASE_5_VS_CODE_HOVER.md) - Implementation plan and acceptance criteria
- [PHASE_4_MANUAL_TESTING.md](./PHASE_4_MANUAL_TESTING.md) - Phase 4 testing structure (reference)
- [VS Code Extension README](../../extensions/vscode/README.md) - Feature documentation
- [ERROR_CODES.md](../../docs/ERROR_CODES.md) - FerrisScript error code reference
