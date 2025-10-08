# Context Detection Testing Guide

**Purpose**: Test matrix for context-aware completion features  
**Created**: October 7, 2025  
**Applies To**: VS Code extensions, LSP implementations, auto-complete features

---

## 🎯 Why This Matters

Context-aware features need to handle:

- **Exact cursor positions** (after `:`, at statement start)
- **Partial user input** (typing `V` after `:`, typing `pr` for `print`)
- **Negative cases** (what should NOT appear in each context)

Testing only "exact positions" misses edge cases that real users encounter.

---

## 📋 Test Matrix Template

Use this matrix when implementing context-aware completion:

| Input Context | Cursor Position | User Input | Expected Completions | Should NOT Show | Priority |
|---------------|-----------------|------------|---------------------|-----------------|----------|
| Type annotation | `let x:` (after colon) | (none) | All types | Keywords, functions | P0 |
| Type annotation | `let x:` (space after) | (none) | All types | Keywords, functions | P0 |
| Type annotation | `let x: i` | "i" | `i32` (filtered) | Keywords, functions | P1 |
| Type annotation | `let x: Vec` | "Vec" | `Vector2` (filtered) | Keywords, functions | P1 |
| Statement start | (empty line) | (none) | Statement keywords | Expression-only keywords | P0 |
| Statement start | `` (indented) | (none) | Statement keywords | Expression-only keywords | P0 |
| Statement start | `i` | "i" | `if`, `ifelse` | Expression keywords | P1 |
| Expression | `let x =` | (none) | Keywords, functions | Statement-only keywords | P0 |
| Expression | `let x = pr` | "pr" | `print` (filtered) | Statement keywords, types | P1 |
| Function call | `print(` | (none) | All valid expressions | Statement keywords | P0 |

**Priority Levels**:

- **P0**: Core functionality - must work
- **P1**: Edge cases - user has typed partial input
- **P2**: Nice-to-have - complex scenarios

---

## 🧪 Example: FerrisScript VS Code Extension

### Test Case: Type Position Detection

```typescript
// Test exact position
detectContext('let x: ', 7)
// Expected: TypePosition
// Should show: i32, f32, bool, String, Vector2, Node, void
// Should NOT show: fn, let, if, while, return, print

// Test with partial input - "i"
detectContext('let x: i', 8)
// Expected: TypePosition (still in type context!)
// Should show: i32 (filtered by "i" prefix)
// Should NOT show: if, ifelse (even though they start with "i")

// Test with partial input - "Vec"
detectContext('let x: Vec', 11)
// Expected: TypePosition
// Should show: Vector2 (filtered by "Vec" prefix)
// Should NOT show: Keywords or functions

// Test function return type
detectContext('fn foo() -> v', 13)
// Expected: TypePosition
// Should show: void (filtered by "v" prefix)
// Should NOT show: Keywords
```

### Test Case: Statement vs Expression

```typescript
// Statement start (empty line)
detectContext('    ', 4)
// Expected: StatementStart
// Should show: fn, let, if, while, return
// Should NOT show: true, false (expression-only)

// Expression context
detectContext('let x = ', 8)
// Expected: Expression
// Should show: if, else, mut, true, false, print
// Should NOT show: fn, let, while, return (statement-only)

// Expression with partial input
detectContext('let x = pr', 10)
// Expected: Expression
// Should show: print (filtered by "pr" prefix)
// Should NOT show: fn, let, while, return
```

---

## ✅ Validation Checklist

When implementing context-aware features:

### During Implementation

- [ ] Created test matrix for all contexts
- [ ] Tested exact cursor positions (P0 cases)
- [ ] Tested with partial user input (P1 cases)
- [ ] Tested negative cases (what should NOT show)
- [ ] Tested prefix filtering interaction

### During Testing

- [ ] Type nothing → all completions appear
- [ ] Type single character → filtered completions
- [ ] Type longer prefix → further filtered
- [ ] Press `Ctrl+Space` without typing → all completions
- [ ] Verify context detection with partial input

### Edge Cases to Test

- [ ] Empty string `""`
- [ ] Single space `" "`
- [ ] Multiple spaces `"   "`
- [ ] Tabs `"\t"`
- [ ] Mixed whitespace `"  \t "`
- [ ] Partial keywords/types (1-3 characters)
- [ ] Full keywords/types
- [ ] Invalid characters after context trigger

---

## 🔧 Implementation Pattern

### Good Context Detection (Handles Partial Input)

```typescript
// ✅ Detects type position even with partial input
if (/:\s*\w*$/.test(beforeCursor)) {
    return CompletionContext.TypePosition;
}
// Matches: "let x: ", "let x: i", "let x: Vec"
```

### Bad Context Detection (Only Exact Position)

```typescript
// ❌ Only works immediately after colon
if (/:\s*$/.test(beforeCursor)) {
    return CompletionContext.TypePosition;
}
// Matches: "let x: " ✅
// Doesn't match: "let x: i" ❌ (user has typed!)
```

---

## 📊 Testing Workflow

1. **Create Matrix**: Fill out test matrix before implementation
2. **Implement**: Write context detection logic
3. **Unit Tests**: Test with all matrix rows (exact + partial)
4. **Manual Tests**: Type in actual editor, verify filtering
5. **Document**: Add notes about prefix filtering behavior

**Time Investment**: 5 minutes to create matrix  
**Time Saved**: 1-2 hours debugging edge cases later

---

## 🔗 Related Documents

- `PHASE_4_LESSONS_LEARNED.md` - Context detection lessons from Phase 4
- `PHASE_4_MANUAL_TESTING.md` - Real-world test cases
- `PREFIX_FILTERING_BEHAVIOR.md` - VS Code prefix filtering documentation

---

## 📝 Template for New Features

Copy this template when starting context-aware feature:

```markdown
## Context Detection Test Matrix

| Context | Cursor | Input | Expected | NOT Expected | Priority |
|---------|--------|-------|----------|--------------|----------|
| [Your context 1] | [Position] | (none) | [Items] | [Excluded] | P0 |
| [Your context 1] | [Position] | "a" | [Filtered] | [Excluded] | P1 |
| [Your context 2] | [Position] | (none) | [Items] | [Excluded] | P0 |

## Test Cases

- [ ] Test exact positions (P0)
- [ ] Test with single character input (P1)
- [ ] Test with longer prefix (P1)
- [ ] Test negative cases (what should NOT show)
- [ ] Test with Ctrl+Space (no filtering)
```

---

**Usage**: Reference this guide at the start of any context-aware feature implementation. Create your test matrix BEFORE writing code.
