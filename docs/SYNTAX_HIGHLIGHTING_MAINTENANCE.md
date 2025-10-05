# Syntax Highlighting Maintenance Guide

**Purpose**: Ensure VS Code syntax highlighting stays synchronized with FerrisScript language features  
**Location**: `extensions/vscode/syntaxes/ferrisscript.tmLanguage.json`  
**Created**: October 5, 2025 (v0.0.2 - Phase 5B)

---

## Overview

FerrisScript's VS Code extension uses a TextMate grammar file to provide syntax highlighting. As the language evolves, this grammar must be updated to highlight new keywords, operators, and syntax constructs.

**Key Principle**: Grammar updates should happen **in the same PR** as language feature additions.

---

## When to Update the Grammar

### Trigger Events ✅

Update the grammar file when adding:

1. **New keywords** (e.g., `match`, `for`, `struct`, `enum`, `trait`)
2. **New operators** (e.g., `%`, `**`, `??`, `?.`)
3. **New literal types** (e.g., array literals `[1, 2, 3]`, dict literals `{key: value}`)
4. **New syntax constructs** (e.g., attributes `@export`, decorators, macros)
5. **New types** (e.g., `u32`, `i64`, `Color`, `Rect2`)
6. **New built-in functions** (e.g., Godot functions like `get_node`)

### Examples

**Adding `for` loop**:

```ferrisscript
// New syntax
for item in collection {
    print(item);
}
```

**Required grammar updates**:

- Add `for` and `in` to keywords pattern
- Consider adding special highlighting for iterators

**Adding `match` expression**:

```ferrisscript
// New syntax
match value {
    1 => print("one"),
    2 => print("two"),
    _ => print("other")
}
```

**Required grammar updates**:

- Add `match` keyword
- Add `=>` operator (fat arrow)
- Add `_` wildcard pattern

---

## How to Update the Grammar

### Step 1: Identify Changes

Review the PR changes to identify new syntax elements:

```bash
# Check for new tokens in lexer
git diff main crates/compiler/src/lexer.rs

# Check for new types
git diff main crates/compiler/src/type_checker.rs
```

Look for new `Token` enum variants or `Type` enum variants.

### Step 2: Update Grammar File

Open `extensions/vscode/syntaxes/ferrisscript.tmLanguage.json`

#### Adding New Keywords

Find the `keywords` pattern and add the new keyword:

```json
{
  "name": "keyword.control.ferrisscript",
  "match": "\\b(if|else|while|return|for|match)\\b"
}
```

**Scope Naming Conventions**:

- Control flow: `keyword.control.ferrisscript` (if, else, while, for, match, return)
- Declarations: `keyword.other.ferrisscript` (fn, let, mut, struct, enum)
- Modifiers: `storage.modifier.ferrisscript` (pub, const, static)

#### Adding New Types

Find the `types` pattern and add the new type:

```json
{
  "name": "entity.name.type.primitive.ferrisscript",
  "match": "\\b(i32|f32|bool|void|u32|i64|u64)\\b"
}
```

#### Adding New Operators

Find the `operators` pattern and add the new operator:

```json
{
  "name": "keyword.operator.arithmetic.ferrisscript",
  "match": "(\\+|-|\\*|/|%)"
}
```

**Note**: Escape special regex characters: `*` → `\\*`, `.` → `\\.`, `?` → `\\?`

#### Adding Complex Patterns

For more complex syntax (like match expressions), add a new repository entry:

```json
{
  "repository": {
    "match-expression": {
      "patterns": [
        {
          "begin": "\\bmatch\\b",
          "end": "}",
          "beginCaptures": {
            "0": { "name": "keyword.control.match.ferrisscript" }
          },
          "patterns": [
            { "include": "#match-arm" },
            { "include": "$self" }
          ]
        }
      ]
    }
  }
}
```

### Step 3: Test the Grammar

#### Local Testing

1. **Install extension locally**:

   ```bash
   # Copy to VS Code extensions directory
   # Windows
   cp -r extensions/vscode ~/.vscode/extensions/ferrisscript-0.1.0
   
   # Reload VS Code (Ctrl+Shift+P → "Reload Window")
   ```

2. **Open test files**:

   ```bash
   code examples/hello.ferris
   code examples/move.ferris
   code examples/bounce.ferris
   ```

3. **Verify highlighting**:
   - New keywords are highlighted correctly
   - Colors match existing conventions
   - No broken highlighting on adjacent code

4. **Create test file** with new syntax:

   ```ferrisscript
   // test_new_syntax.ferris
   fn test_match() {
       match value {
           1 => print("one"),
           _ => print("other")
       }
   }
   ```

#### Automated Testing

Currently no automated tests for grammar. Consider adding in future:

- VS Code extension tests using `vscode-test`
- Snapshot tests for syntax highlighting output

### Step 4: Update Documentation

Update the following files:

1. **Extension CHANGELOG.md**:

   ```markdown
   ## [0.2.0] - YYYY-MM-DD
   
   ### Added
   - Syntax highlighting for `match` keyword
   - Syntax highlighting for `=>` operator (fat arrow)
   - Syntax highlighting for `_` wildcard pattern
   ```

2. **Extension README.md** (if major feature):

   ```markdown
   ### Syntax Highlighting
   
   - **Keywords**: `fn`, `let`, `mut`, `if`, `else`, `while`, `return`, `match`, `for`
   ```

3. **Commit message**:

   ```
   feat(vscode): add syntax highlighting for match expressions
   
   - Add `match` keyword highlighting
   - Add `=>` fat arrow operator
   - Add wildcard pattern `_` highlighting
   - Test on examples/pattern_matching.ferris
   
   Part of #123 (Add pattern matching support)
   ```

### Step 5: Update Version

If this is the only change, increment patch version:

```json
// extensions/vscode/package.json
{
  "version": "0.1.1"  // was 0.1.0
}
```

If part of larger feature release, version bump happens with that release.

---

## Quarterly Grammar Audit

**Frequency**: Every 3 months  
**Owner**: Maintainer or designated contributor

### Audit Checklist

- [ ] Review `crates/compiler/src/lexer.rs` Token enum
  - [ ] Compare against grammar keywords
  - [ ] Note any missing tokens
  
- [ ] Review `crates/compiler/src/type_checker.rs` Type enum
  - [ ] Compare against grammar types
  - [ ] Note any missing types
  
- [ ] Test grammar on all example scripts
  - [ ] `examples/*.ferris`
  - [ ] `godot_test/scripts/*.ferris`
  
- [ ] Check for user-reported highlighting issues
  - [ ] Search GitHub issues for "syntax highlighting"
  - [ ] Search GitHub issues for "colors"
  
- [ ] Review grammar coverage
  - [ ] Are all language features highlighted?
  - [ ] Are there any false positives?
  - [ ] Are scope names following conventions?
  
- [ ] Update grammar if discrepancies found
  - [ ] Follow "How to Update" steps above
  - [ ] Create audit summary issue
  
- [ ] Update this maintenance guide if process changed

---

## Grammar Architecture

### File Structure

```json
{
  "$schema": "...",
  "name": "FerrisScript",
  "scopeName": "source.ferrisscript",
  "patterns": [
    // Top-level patterns (order matters!)
    { "include": "#comments" },
    { "include": "#keywords" },
    { "include": "#types" },
    // ...
  ],
  "repository": {
    // Pattern definitions
    "comments": { ... },
    "keywords": { ... },
    "types": { ... }
  }
}
```

### Pattern Matching Order

**Important**: Patterns are matched in order. Put more specific patterns first:

1. **Comments** - Match first to prevent keywords inside comments
2. **Keywords** - Match before identifiers
3. **Types** - Match before identifiers
4. **Strings** - Match early to prevent escapes breaking other patterns
5. **Numbers** - Specific format before generic identifiers
6. **Functions** - Match function names before generic identifiers
7. **Operators** - Match operators
8. **Punctuation** - Match last (generic)

### Scope Naming

Follow TextMate conventions: `<category>.<subcategory>.<language>`

**Common Categories**:

- `comment` - Comments
- `keyword` - Language keywords
- `storage` - Storage types and modifiers
- `string` - String literals
- `constant` - Constants (numbers, booleans, null)
- `variable` - Variables
- `entity` - Types, classes, functions
- `punctuation` - Delimiters, operators

**Examples**:

- `keyword.control.ferrisscript` - Control flow keywords
- `entity.name.type.ferrisscript` - Type names
- `constant.numeric.ferrisscript` - Number literals

---

## Common Pitfalls

### Regex Escaping

TextMate uses regex. Escape special characters:

❌ **Wrong**:

```json
"match": "\\b(i32|f32|bool)\\b"  // Missing escapes
```

✅ **Correct**:

```json
"match": "\\\\b(i32|f32|bool)\\\\b"  // Properly escaped
```

**Quick Reference**:

- `\b` word boundary → `\\\\b`
- `.` dot → `\\\\.`
- `*` star → `\\\\*`
- `+` plus → `\\\\+`
- `?` question → `\\\\?`
- `|` pipe → `\\\\|` (in character classes)
- `(` paren → `\\\\(`

### Order Matters

Specific patterns must come before general ones:

❌ **Wrong**:

```json
"patterns": [
  { "include": "#identifiers" },  // Matches everything
  { "include": "#keywords" }       // Never reached!
]
```

✅ **Correct**:

```json
"patterns": [
  { "include": "#keywords" },      // Specific first
  { "include": "#identifiers" }    // General last
]
```

### Greedy Matching

Use non-greedy quantifiers for multi-line matches:

❌ **Wrong** (matches too much):

```json
"begin": "/\\*",
"end": "\\*/"  // Greedy - matches to last */ in file
```

✅ **Correct**:

```json
"begin": "/\\*",
"end": "\\*/",
"patterns": [
  // Nested patterns here
]
```

---

## Resources

### TextMate Grammar

- [VS Code Language Extensions Guide](https://code.visualstudio.com/api/language-extensions/syntax-highlight-guide)
- [TextMate Language Grammars](https://macromates.com/manual/en/language_grammars)
- [Scope Naming Conventions](https://www.sublimetext.com/docs/scope_naming.html)
- [TextMate Grammar Testing](https://www.apeth.com/nonblog/stories/textmatebundle.html)

### Example Grammars

- [Rust TextMate Grammar](https://github.com/microsoft/vscode/blob/main/extensions/rust/syntaxes/rust.tmLanguage.json)
- [TypeScript Grammar](https://github.com/microsoft/TypeScript-TmLanguage/blob/master/TypeScript.tmLanguage)
- [Python Grammar](https://github.com/microsoft/vscode/blob/main/extensions/python/syntaxes/MagicPython.tmLanguage.json)

### Tools

- [TextMate Scope Inspector](https://www.sublimetext.com/docs/scope_naming.html#debugging) - Debug scope names
- [VS Code Token Inspector](https://code.visualstudio.com/api/language-extensions/syntax-highlight-guide#scope-inspector) - Ctrl+Shift+P → "Inspect Editor Tokens"
- [Regex101](https://regex101.com/) - Test regex patterns

---

## Future Improvements

### v0.0.3

- [ ] Add syntax highlighting tests (snapshot-based)
- [ ] Automate grammar validation in CI

### v0.0.5 (with LSP)

- [ ] Semantic highlighting (context-aware colors)
- [ ] Dynamic token modifiers
- [ ] Error highlighting in editor

---

**Maintainer Notes**:

- Keep this guide updated as grammar complexity grows
- Add examples from real PRs that updated grammar
- Link to specific commits for reference
- Consider creating a grammar update script (automation)

---

Made with 🦀 and ❤️ for the Godot community
