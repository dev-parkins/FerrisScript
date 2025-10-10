# Compiler Development Best Practices

**Source**: Learnings from FerrisScript v0.0.2 - v0.0.3 development  
**Last Updated**: October 8, 2025  
**Audience**: Contributors, compiler developers, language implementers

---

## 🎯 Purpose

This document captures generalizable best practices, patterns, and anti-patterns discovered during FerrisScript development. These lessons apply to compiler development, language design, and software engineering in general.

---

## 🔧 Compiler Implementation Patterns

### Error Recovery: Always Advance Before Sync

**Pattern**: Panic-mode error recovery

**The Rule**: When implementing parser error recovery, **always advance at least one token** before calling `synchronize()`.

**Why**: If `synchronize()` finds a sync point immediately (current token is already at sync point), and you haven't advanced, you create an infinite loop. Parser stays at the same position forever.

**Correct Pattern**:

```rust
fn parse_something(&mut self) -> Result<Node, String> {
    if !self.check_expected_token() {
        let error = format!("Expected X, found {:?}", self.current);
        self.record_error(error);    // 1. Record the error
        self.advance();              // 2. CRITICAL: Advance past bad token
        self.synchronize();          // 3. Find safe recovery point
        return Err("Parse error");
    }
    // ... normal parsing
}
```

**Incorrect Pattern** (causes infinite loop):

```rust
fn parse_something(&mut self) -> Result<Node, String> {
    if !self.check_expected_token() {
        self.record_error("Error");
        self.synchronize();  // ❌ BUG: Didn't advance first!
        return Err("Error");
    }
}
```

**Real-World Impact**: This bug caused FerrisScript parser to consume all RAM when encountering unexpected top-level tokens.

**Sync Point Selection**: Choose sync points that align with grammar structure:

- Statement boundaries (`;`, `}`)
- Declaration keywords (`fn`, `let`, `class`, `struct`)
- Block terminators (`}`, `end`, closing tags)

**References**:

- [Engineering a Compiler (Cooper & Torczon), Chapter 3: Parsers]
- [Crafting Interpreters (Nystrom), Chapter 6: Parsing Expressions]
- FerrisScript LEARNINGS.md Phase 3C

---

### Adaptive Thresholds for Typo Detection

**Pattern**: String similarity with length-based thresholds

**The Problem**: Single similarity threshold doesn't work for all identifier lengths:

- Short names: `x` vs `y` = 1 edit, 50% similarity (not similar!)
- Long names: `player_health` vs `player_wealth` = 1 edit, 92% similarity (very similar!)

**Solution**: Adaptive thresholds based on identifier length

```rust
fn is_similar_identifier(candidate: &str, target: &str) -> bool {
    let distance = levenshtein(candidate, target);
    let max_len = candidate.len().max(target.len());
    
    if max_len <= 4 {
        // Very short: require exact match or 1 edit
        distance <= 1
    } else if max_len <= 8 {
        // Short: allow up to 2 edits
        distance <= 2
    } else {
        // Long: use percentage similarity (70%+)
        let similarity = 1.0 - (distance as f32 / max_len as f32);
        similarity >= 0.70
    }
}
```

**Key Insights**:

- **Short identifiers** (1-4 chars): Strict edit distance (≤1 edit)
- **Medium identifiers** (5-8 chars): Moderate edit distance (≤2 edits)
- **Long identifiers** (9+ chars): Percentage similarity (≥70%)

**Why This Works**:

- Short names have fewer edit combinations, false positives are common
- Long names have more context, percentage similarity better captures "feels similar"
- 70% threshold empirically validated across 20+ test cases

**Usage**: "Did you mean?" suggestions for undefined variables, functions, types

**References**:

- FerrisScript LEARNINGS.md Phase 2
- [Levenshtein distance on Wikipedia](https://en.wikipedia.org/wiki/Levenshtein_distance)

---

### Test Actual Behavior, Not Assumptions

**Pattern**: Empirical test-driven development

**The Problem**: Compiler behavior doesn't always match your mental model. Writing tests based on assumptions leads to false failures.

**Example** (FerrisScript Phase 1):

```rust
// ❌ Assumed Behavior (test failed):
let code = "let x: int = 5;";  // Using invalid type
assert_error_code(&code, "E110");  // Expected parser error

// ✅ Actual Behavior (test passed):
let code = "let x: unknown_type = 5;";
assert_error_code(&code, "E218");  // Type checker catches it first
```

**Why**: Parser accepts `int` as identifier, type checker rejects it later. Error codes assigned by **which stage catches it first**, not "most appropriate" category.

**Best Practice**:

1. **Write exploratory test** → compile code → observe actual error
2. **Document actual behavior** → understand why it works this way
3. **Write test for reality** → test what compiler actually does
4. **Refine later if needed** → change behavior intentionally, not accidentally

**Anti-Pattern**: Writing tests based on "how it should work" without validating actual behavior first.

**References**: FerrisScript LEARNINGS.md Phase 1

---

## 🧪 Testing Strategies

### Integration Over Unit for User-Facing Features

**Pattern**: Test user experience, not implementation details

**Principle**: For features users interact with directly (error messages, suggestions, completions), **integration tests are more valuable than unit tests**.

**Why**:

- **Unit tests** verify algorithm correctness (Levenshtein distance calculation)
- **Integration tests** verify user value (do users see helpful suggestions?)

**Example** (FerrisScript Phase 2):

```rust
// ✅ Integration Test (high value):
#[test]
fn test_variable_suggestion_typo() {
    let code = r#"
        let player_health: i32 = 100;
        print(player_helth);  // Typo
    "#;
    let result = compile(code);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("did you mean 'player_health'?"));
}

// ⚠️ Unit Test (lower value):
#[test]
fn test_levenshtein_distance() {
    assert_eq!(levenshtein("kitten", "sitting"), 3);
}
```

**When to Prefer Integration Tests**:

- Error message formatting
- Suggestion quality
- IDE feature UX (completion, hover, diagnostics)
- End-to-end workflows

**When Unit Tests Still Valuable**:

- Algorithm correctness (parsers, type checkers, optimizers)
- Edge case coverage (overflow, unicode, pathological inputs)
- Performance benchmarking

**Balance**: 70% integration, 30% unit for user-facing features. Reverse for internal algorithms.

**References**: FerrisScript LEARNINGS.md Phase 2

---

### Manual Testing for UI Features

**Pattern**: Human validation for subjective experience

**Principle**: For UI-heavy features (VS Code extensions, IDE integrations), **manual testing guides are more valuable than automated tests**.

**Why**:

- Automated tests verify *logic* (does completion return correct items?)
- Manual testing verifies *experience* (does this feel good to use?)

**What Manual Testing Covers**:

- Visual appearance (colors, icons, formatting)
- Interaction patterns (tab completion, hover timing, menu navigation)
- Edge cases users actually encounter (incomplete code, cursor positions)
- Integration with real editor (not mocked VS Code API)

**Example Structure** (FerrisScript Phase 4):

```markdown
## Test Case 3: Keyword Completion

**Scenario**: Trigger completion at statement start  
**Steps**:
1. Open `.ferris` file
2. Type `l` at beginning of line
3. Press Ctrl+Space

**Expected Result**:
- Completion menu appears
- Shows `let` keyword with variable declaration icon
- Detail text: "Variable declaration"

**Actual Result**: [Tester fills in]  
**Status**: [Pass/Fail]  
**Notes**: [Any observations]
```

**When to Use Manual Testing**:

- VS Code/IDE extensions
- CLI tools (terminal output, colors, progress bars)
- Error message formatting (readability, clarity)
- Documentation website UX

**When Automated Testing Better**:

- API behavior (does function return correct values?)
- Protocol compliance (LSP message format)
- Regression prevention (ensure bugs stay fixed)

**Best Practice**: Create **structured manual testing guides** with specific scenarios, expected results, and tracking tables. Not ad-hoc "play around with it" testing.

**References**: FerrisScript LEARNINGS.md Phase 4, PHASE_4_MANUAL_TESTING.md

---

## 🏗️ Architecture & Design

### Defer Complexity Until Needed

**Pattern**: YAGNI (You Aren't Gonna Need It) for compilers

**Principle**: Don't implement advanced features until you have infrastructure to support them properly.

**Examples** (FerrisScript):

| Feature | Deferred From | Deferred To | Reason |
|---------|---------------|-------------|--------|
| Scope-aware completion | Phase 4 | v0.0.5 LSP | Simple static lists sufficient for v0.0.3 |
| Automated extension tests | Phase 4 | v0.0.5 LSP | Manual testing adequate, setup complex |
| Keyword suggestions | Phase 2 | v0.0.4 | Requires lexer changes, low value/effort ratio |
| Multi-error reporting | Phase 3C | v0.0.4 | Foundation (recovery) needed first |

**Decision Process**:

```
1. Is this feature needed NOW? 
   → No → Defer
   
2. Do we have infrastructure for this?
   → No → Defer until infrastructure ready
   
3. Is simpler alternative good enough?
   → Yes → Implement simple version, defer complexity
   
4. What's the value/effort ratio?
   → Low → Defer to future version
```

**Anti-Pattern**: Implementing "complete" LSP when simple completion provider works. Result: 10x development time for 20% more value.

**Key Quote**: "Resist temptation to implement advanced features in Phase 4. Save for LSP implementation when you'll have proper infrastructure. YAGNI principle applies." (FerrisScript LEARNINGS.md)

**References**: FerrisScript LEARNINGS.md Phases 2, 4

---

### Separate Concerns for Maintainability

**Pattern**: Modular code organization

**Example** (VS Code Extension):

```
extensions/vscode/src/
├── extension.ts           # Entry point (activate/deactivate)
├── providers/
│   ├── completion.ts      # CompletionItemProvider
│   ├── hover.ts           # HoverProvider
│   └── diagnostics.ts     # DiagnosticProvider
├── data/
│   ├── keywords.ts        # Keyword list + completion items
│   ├── types.ts           # Type list + completion items
│   └── functions.ts       # Built-in function list
└── utils/
    ├── context.ts         # Context detection (regex patterns)
    └── compiler.ts        # Compiler invocation
```

**Benefits**:

- **Easy to extend**: Add new keyword? Edit `keywords.ts`. New provider? Add to `providers/`.
- **Clear responsibilities**: Each file has single purpose
- **Testable**: Can test `keywords.ts` without loading entire extension
- **Navigable**: Developers find code quickly

**Anti-Pattern**: Single `extension.ts` with all logic mixed together (1000+ lines).

**References**: FerrisScript LEARNINGS.md Phase 4

---

## ⚙️ Development Workflow

### Incremental Compilation for TypeScript

**Pattern**: Build files in dependency order

**Problem**: Compiling all TypeScript files at once → dozens of errors at once → overwhelming

**Solution**: Build incrementally, one file at a time, bottom-up dependency order

**Example** (VS Code Extension):

```bash
# 1. Data modules (no dependencies)
npx tsc src/data/keywords.ts --outDir out

# 2. Utilities (depend on data modules)
npx tsc src/utils/context.ts --outDir out

# 3. Providers (depend on data + utils)
npx tsc src/providers/completion.ts --outDir out

# 4. Extension entry point (depends on everything)
npx tsc src/extension.ts --outDir out

# 5. Full build (after fixing all errors)
npm run compile
```

**Benefits**:

- **Fast feedback**: Errors in one file, not dozens
- **Clear error context**: Know exactly where error comes from
- **Easier debugging**: Isolate problems to specific modules

**When to Use**: Any multi-file TypeScript/JavaScript project, especially during initial setup.

**References**: FerrisScript LEARNINGS.md Phase 4

---

### Quality Gates Are Non-Negotiable

**Pattern**: Strict automated quality checks before merge

**The Gates**:

```bash
# 1. Tests
cargo test --workspace

# 2. Strict Clippy (catches issues regular clippy misses)
cargo clippy --workspace --all-targets --all-features -- -D warnings

# 3. Formatting
cargo fmt --all -- --check

# 4. Documentation Links (if markdown changed)
npx markdown-link-check **/*.md
```

**Why `-D warnings` is Critical**:

Regular `cargo clippy` only shows warnings. `-D warnings` treats warnings as errors, forcing you to fix them.

**Example Issues Caught** (FerrisScript Phase 1):

- `useless_vec![]` (should use arrays in tests)
- Deprecated `criterion::black_box` (should use `std::hint::black_box`)
- Unnecessary clones in benchmarks

**Best Practice**: Run these checks **before declaring work complete**, not after PR review.

**Automation**: Use pre-commit hooks (FerrisScript has `scripts/pre-commit.sh`) to enforce gates locally.

**References**: FerrisScript LEARNINGS.md Phase 1

---

### Document as You Go

**Pattern**: Write documentation alongside code

**When to Document**:

- **During implementation**: Capture design decisions while fresh
- **Before PR**: Add usage examples and API docs
- **After challenges**: Document bugs and solutions in LEARNINGS.md

**What to Document**:

| Document Type | When | Example |
|---------------|------|---------|
| API Docs (rustdoc) | While writing code | `/// Returns the Levenshtein distance...` |
| Phase Plans | Before implementation | `PHASE_4_VS_CODE_COMPLETION.md` |
| Learnings | After solving problems | "Critical Infinite Loop Bug: ..." |
| Examples | With new features | `examples/error_recovery.ferris` |

**Anti-Pattern**: "We'll document it later" → Never happens, or happens when you've forgotten details.

**Quality Metric**: If someone else can understand your code from docs alone (without reading implementation), docs are good enough.

**References**: FerrisScript LEARNINGS.md Phase 1

---

## 📊 Project Management

### Feature Grouping for Focused PRs

**Pattern**: Group related work into coherent phases

**Example** (FerrisScript v0.0.3):

- **Phase 1**: Error codes (all categories together, not separate PRs per category)
- **Phase 2**: Suggestions (variables + functions + types in one PR)
- **Phase 6+7**: Dev tooling + benchmarks (combined when overlap discovered)

**Benefits**:

- **Cohesive changes**: Related code changed together
- **Easier review**: Reviewers understand full context
- **Atomic delivery**: Feature complete in one PR, not half-done across many

**When to Split**:

- PR becomes too large (>15 files changed)
- Components truly independent (no coupling)
- Deliverables span multiple weeks

**References**: FerrisScript planning documents

---

### Deferred Items Tracking

**Pattern**: Explicit tracking of what's not done

**Why**: Prevents "lost work" where features are discussed but never implemented.

**Structure**:

```markdown
## Deferred Items

| Item | Target Version | Rationale | Tracking |
|------|---------------|-----------|----------|
| Multi-error reporting | v0.0.4 | Foundation needed | v0.0.4-roadmap.md |
```

**Best Practice**: Every time you defer something:

1. Document **what** was deferred
2. Document **why** it was deferred
3. Document **when** it will be reconsidered (target version)
4. Add to appropriate roadmap document

**References**: FerrisScript DEFERRED_ITEMS_TRACKING.md

---

## 🎓 Lessons for Language Design

### Error Codes Enable Maintainability

**Insight**: Structured error codes (E001, E202, etc.) improve long-term maintainability more than user experience.

**Benefits**:

- **Search/filterable**: Users can Google "FerrisScript E218"
- **Documentation linkable**: Error message → docs → solution
- **Versioning**: Track error code changes across versions
- **Analytics**: Which errors do users encounter most?

**Cost**: Extra infrastructure (error code enum, documentation, tests).

**ROI**: High for any language with >100 users. Low for personal projects.

**References**: FerrisScript ERROR_CODES.md, LEARNINGS.md Phase 1

---

### Progressive Disclosure of Features

**Insight**: Simple features first, advanced features later.

**Example** (FerrisScript):

- v0.0.3: Static completion lists (keywords, types)
- v0.0.5: LSP with scope-aware completion (variables in current scope)
- v0.1.0: Intelligent completion (type-based filtering, context ranking)

**Why**: Users learn incrementally. Advanced features require infrastructure. Ship value early.

**Anti-Pattern**: Waiting until LSP is perfect before shipping any completion.

**References**: FerrisScript v0.0.3 roadmap

---

## 📚 References

- [Engineering a Compiler (2nd Edition)](https://www.elsevier.com/books/engineering-a-compiler/cooper/978-0-12-088478-0) - Cooper & Torczon
- [Crafting Interpreters](https://craftinginterpreters.com/) - Bob Nystrom
- [The Rust Programming Language](https://doc.rust-lang.org/book/) - Error handling patterns
- [TypeScript Compiler](https://github.com/microsoft/TypeScript) - Incremental compilation, error recovery
- FerrisScript LEARNINGS.md - Project-specific insights

---

## 🤝 Contributing

Found a best practice not covered here? Learned something from FerrisScript development?

1. Document your insight in `docs/planning/v0.0.X/LEARNINGS.md` for the relevant phase
2. Extract generalizable patterns to this document
3. Include references and examples
4. Submit a PR with the "documentation" label

---

**Last Updated**: October 8, 2025  
**Maintained By**: FerrisScript core team  
**License**: MIT (same as FerrisScript project)
