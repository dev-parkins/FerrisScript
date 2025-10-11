# VS Code Completion Prefix Filtering

**Purpose**: Document VS Code's automatic prefix filtering behavior  
**Created**: October 7, 2025  
**Applies To**: VS Code extensions, completion providers, auto-complete features

---

## 🎯 Key Concept

⚠️ **Important**: VS Code **automatically filters** completion items based on what the user has typed.

This is **built-in behavior**, not something your extension controls. Understanding this prevents confusion about "missing" completions.

---

## 📋 How Prefix Filtering Works

### Example: Boolean Literals

Your completion provider returns both `true` and `false`:

```typescript
return [
    { label: 'true', ... },
    { label: 'false', ... }
];
```

**What user sees**:

| User Types | VS Code Shows | Why |
|------------|---------------|-----|
| (nothing) | `true`, `false` | No filter - all items shown |
| `Ctrl+Space` | `true`, `false` | Manual trigger - all items shown |
| `t` | `true` | Matches prefix "t" |
| `tr` | `true` | Matches prefix "tr" |
| `tru` | `true` | Matches prefix "tru" |
| `f` | `false` | Matches prefix "f" |
| `fa` | `false` | Matches prefix "fa" |
| `x` | (nothing) | No matches for "x" |

**Key Point**: User typing `tr` and **not seeing** `false` is **correct behavior**.

---

## 🧪 Real Examples from FerrisScript

### Example 1: Type Completion

Provider returns all types:

```typescript
return [
    { label: 'i32', ... },
    { label: 'f32', ... },
    { label: 'bool', ... },
    { label: 'String', ... },
    { label: 'Vector2', ... },
    { label: 'Node', ... },
    { label: 'void', ... }
];
```

**User types `let pos: V`**:

- ✅ `Vector2` appears (matches "V")
- ✅ `void` appears (matches "v" case-insensitive)
- ❌ `i32`, `f32`, `bool`, `String`, `Node` do NOT appear (don't match "V")

**This is expected**. Without typing "V" (just `let pos:`), all types appear.

### Example 2: Keywords

Provider returns all keywords:

```typescript
return [
    { label: 'if', ... },
    { label: 'else', ... },
    { label: 'while', ... },
    { label: 'return', ... },
    { label: 'let', ... }
];
```

**User types `i` at statement start**:

- ✅ `if` appears (matches "i")
- ❌ `else`, `while`, `return`, `let` do NOT appear (don't match "i")

---

## ✅ Testing Best Practices

### Test Both Scenarios

For every completion context, test:

1. **No prefix** (user hasn't typed anything)
   - Press `Ctrl+Space` at cursor position
   - Verify ALL expected items appear

2. **With prefix** (user has typed 1-3 characters)
   - Type single character (e.g., "i", "V", "t")
   - Verify ONLY matching items appear
   - Verify non-matching items do NOT appear

### Example Test Cases

```markdown
## Test: Type Completion

**Setup**: Type `let x: `

**Test 1 - No Prefix**:
- Press Ctrl+Space (don't type anything)
- Expected: All types appear (i32, f32, bool, String, Vector2, Node, void)

**Test 2 - Prefix "i"**:
- Type "i" after colon: `let x: i`
- Expected: Only i32 appears
- NOT expected: f32, bool, String, Vector2, Node, void

**Test 3 - Prefix "V"**:
- Type "V" after colon: `let x: V`
- Expected: Vector2, void appear (case-insensitive match)
- NOT expected: i32, f32, bool, String, Node
```

---

## 🐛 Common Misunderstandings

### ❌ "My completion is broken - it's not showing all items!"

**Issue**: User typed prefix, only sees matching items

**Reality**: This is **correct behavior**. VS Code is filtering by prefix.

**Fix**: Test without typing anything (Ctrl+Space)

### ❌ "false doesn't appear when I type true"

**Issue**: Expected both `true` and `false` in completion list

**Reality**: Typing "tr" filters to only items matching "tr". `false` doesn't match.

**Fix**: Type "f" to see `false`, or press Ctrl+Space to see both

### ❌ "Vector2 is the only type showing"

**Issue**: Expected all types when typing `let pos: V`

**Reality**: "V" prefix filters to only Vector2 and void

**Fix**: Don't type anything, just `let pos:` and press Ctrl+Space

---

## 📖 Documentation Pattern

When documenting completion features, always clarify prefix filtering:

### Good Documentation ✅

```markdown
**Expected Results**:
- [ ] Type `let x: ` and press Ctrl+Space → All types appear
- [ ] Type `let x: i` → Only i32 appears (prefix filtering)
- [ ] Type `let x: V` → Only Vector2 and void appear (prefix filtering)

**Note**: VS Code automatically filters completions by prefix. Type "i" to see
i32, or "V" to see Vector2. Press Ctrl+Space without typing to see all types.
```

### Bad Documentation ❌

```markdown
**Expected Results**:
- [ ] All types appear

**Note**: Sometimes only some types show up (unclear when/why)
```

---

## 🔧 Implementation Notes

### Your Provider Returns Everything

Your completion provider should return **all valid items** for the context:

```typescript
provideCompletionItems(document, position) {
    const context = detectContext(document, position);
    
    if (context === TypePosition) {
        // Return ALL types - VS Code will filter by prefix
        return [
            { label: 'i32', ... },
            { label: 'f32', ... },
            { label: 'bool', ... },
            { label: 'String', ... },
            { label: 'Vector2', ... },
            { label: 'Node', ... },
            { label: 'void', ... }
        ];
    }
}
```

**Don't** try to filter based on what user has typed - VS Code does this automatically.

### Prefix Matching Rules

VS Code matches prefixes:

- **Case-insensitive** by default
- **Substring matching** can be configured
- **Fuzzy matching** can be enabled

For FerrisScript, default case-insensitive prefix matching is sufficient.

---

## 🧪 Interactive Testing Checklist

When testing completion features:

- [ ] Open test file with `.ferris` extension
- [ ] Type trigger character (e.g., `:` for types)
- [ ] **Test 1**: Press `Ctrl+Space` without typing → verify ALL items
- [ ] **Test 2**: Type single character (e.g., "i") → verify filtered items
- [ ] **Test 3**: Type longer prefix (e.g., "Vec") → verify further filtered
- [ ] **Test 4**: Type invalid prefix (e.g., "zzz") → verify no items
- [ ] **Test 5**: Delete characters → verify items reappear as prefix shortens

---

## 📊 Troubleshooting Guide

| Symptom | Likely Cause | Solution |
|---------|--------------|----------|
| No completions at all | Context detection failed | Check regex in detectContext() |
| Only some items show | User has typed prefix | Test with Ctrl+Space (no typing) |
| Wrong items appear | Wrong context detected | Verify context detection logic |
| Items appear then disappear | Typing narrows prefix | Expected - keep typing or delete |

---

## 🔗 Related Documents

- `CONTEXT_DETECTION_TESTING.md` - Test matrix for context detection
- `PHASE_4_MANUAL_TESTING.md` - Real-world test cases
- `PHASE_4_LESSONS_LEARNED.md` - Lessons from prefix filtering confusion

---

## 📝 Quick Reference

**Remember**:

1. Your provider returns ALL valid items for context
2. VS Code filters based on user's typed prefix
3. Test BOTH with and without typing
4. Document prefix filtering behavior in tests
5. "Missing" items usually = prefix filtering working correctly

**Time Investment**: 2 minutes to add note in docs  
**Time Saved**: 15-20 minutes explaining "missing" completions later

---

**Usage**: Reference this document when implementing any completion provider. Add prefix filtering notes to all test documentation.
