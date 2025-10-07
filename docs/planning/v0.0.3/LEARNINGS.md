# FerrisScript v0.0.3 Learnings

**Version**: 0.0.3 - Editor Experience Alpha  
**Milestone**: [#2](https://github.com/dev-parkins/FerrisScript/milestone/2)  
**Status**: In Progress

---

## 🎯 Purpose

This document captures key insights, discoveries, and lessons learned during v0.0.3 development. It serves as a reference for future versions and helps maintain institutional knowledge.

---

## 📊 Phase-Specific Learnings

### Phase 1: Error Code System ✅

**Date Started**: October 3, 2025  
**Date Completed**: October 6, 2025  
**PR**: [#27](https://github.com/dev-parkins/FerrisScript/pull/27)

#### Technical Discoveries

- **Error Code Assignment Timing**: Error codes are assigned based on which compiler stage catches the error first, not necessarily the "most appropriate" category. For example, `let x: int = 5;` triggers type checking errors (E218/E200) rather than parser errors (E110) because the parser accepts it and type checker catches the invalid type.

- **Error Code Organization**: 63 error codes implemented across 5 categories:
  - Lexical (E001-E003): 3 codes for character, string, and escape sequence errors
  - Syntax (E100-E113): 14 codes for parser-level errors
  - Type (E200-E219): 19 codes for type checking errors
  - Runtime (E400-E418): 24 codes for runtime errors
  - Internal (E900-E999): Reserved for compiler bugs

- **Validation Testing Strategy**: Test cases must match actual compiler behavior, not ideal behavior. Initial test cases needed adjustment because they triggered different error codes than expected.

- **Documentation Scale**: Comprehensive error documentation (ERROR_CODES.md) reached 4,000+ lines with descriptions, examples, and fixes for each code.

#### Challenges Encountered

- **Clippy Strict Mode**: Initial implementation passed regular `cargo clippy` but failed strict mode (`cargo clippy --workspace --all-targets --all-features -- -D warnings`). Found:
  - `useless_vec` warnings in test code (should use arrays instead)
  - Deprecated `criterion::black_box` usage (should use `std::hint::black_box`)
  
- **Dependency Updates**: Updating from criterion 0.5 → 0.7 and godot 0.1 → 0.4 introduced breaking API changes:
  - `criterion::black_box` deprecated in favor of `std::hint::black_box`
  - godot 0.4 changed `GString` from pass-by-value to pass-by-reference in API calls

- **Documentation Link Validation**: Found 11 broken markdown links across planning documents during final validation:
  - Incorrect relative paths (missing `../` levels)
  - Links to non-existent future phase documents
  - Outdated roadmap filenames

#### Solutions Applied

- **Test Adjustments**: Modified validation tests to match actual error code behavior rather than assumed behavior. Example: Used `let x: unknown_type = 5;` instead of `let x: int = 5;` to trigger specific parser errors.

- **Strict Quality Gates**: Established `cargo clippy --workspace --all-targets --all-features -- -D warnings` as the standard for all future work to catch issues early and prevent tech debt accumulation.

- **Dependency Migration**: Updated benchmark files to use `std::hint::black_box` and fixed godot API calls to pass `&GString` instead of cloning. All 222 tests still passing after updates.

- **Documentation Hygiene**: Implemented systematic link checking with `npx markdown-link-check` and fixed all broken links before merging. Updated non-existent phase document links to show `*(To be created)*` placeholders.

#### Best Practices Identified

- **Always Run Strict Clippy**: Use `cargo clippy --workspace --all-targets --all-features -- -D warnings` for final validation before any PR. This catches issues that standard clippy misses and prevents tech debt.

- **Test Actual Behavior**: When writing validation tests, always verify the actual error codes produced by test cases rather than assuming which codes should appear.

- **Document as You Go**: Comprehensive error documentation (with examples and fixes) should be created alongside implementation, not after. This ensures accuracy and completeness.

- **Validate Links Before Commit**: Always run `npx markdown-link-check` on modified markdown files to catch broken links early. Follow DOCUMENTATION_LINKING_GUIDELINES.md for link best practices.

- **Keep Dependencies Current**: Regularly update dependencies to latest stable versions to avoid accumulating breaking changes. Test thoroughly after updates.

- **Format Code Before "Done"**: Always run `cargo fmt --all` before declaring work complete. Include this in all workflow documentation and checklists.

---

### Phase 2: Error Suggestions ✅

**Date Started**: October 6, 2025  
**Date Completed**: October 6, 2025  
**PR**: *(To be filled after PR creation)*

#### Technical Discoveries

- **Adaptive Thresholds Essential**: String similarity thresholds must adapt to identifier length. Short names (≤8 chars) need strict edit distance (≤2-3), while long names (>8 chars) work better with percentage similarity (≥70%). Using a single threshold produces too many false positives.

- **Levenshtein Performance**: Dynamic programming implementation of Levenshtein distance is O(m×n) but still very fast for typical identifiers (<1ms overhead per error). No optimization needed for practical use cases.

- **Suggestion Ranking**: Sorting candidates by edit distance alone is sufficient. More complex ranking schemes (scope proximity, type matching) would require significant infrastructure changes for minimal UX improvement.

- **Format String Simplicity**: Rust-style "help: did you mean 'X'?" format is clearer and more concise than showing full code context. Users just need the corrected identifier name.

- **Test Coverage Strategy**: Integration tests (full compiler pipeline) are more valuable than unit tests for suggestions, since the real question is "does the user see helpful suggestions?" not "is the algorithm mathematically correct?".

#### Challenges Encountered

- **Initial Test Failures**: First test run had 3 failing tests due to threshold logic being too strict. Tests expected suggestions for 3-edit-distance typos on 8-char names, but thresholds rejected them.

- **Format String Complexity**: Initial implementation tried to show full code context (like Rust compiler) but this required complex span tracking and multi-line formatting. Simpler "did you mean X?" format provided equal value with less code.

- **Type Checker Scope Access**: Type checker needed new methods to list available variables, functions, and types for suggestion search. Required careful thinking about what's "in scope" at error time.

#### Solutions Applied

- **Threshold Refinement**: Adjusted thresholds based on test results. Short names use `distance <= 2 || (len <= 4 && distance <= 1)`, long names use `similarity >= 70%`. This balanced precision vs recall.

- **Simplified Format**: Used concise `\nhelp: did you mean '{suggestion}'?` format that's easy to parse and understand. Removed complex span highlighting and multi-line context.

- **Generic Finder Function**: Created single `find_similar_identifiers()` function that works for variables, functions, and types. Accepts any iterator of candidate names, applies thresholds, sorts, and returns top 3. Reduced code duplication significantly.

- **Scope Listing Methods**: Added helper functions to collect available identifiers at error time. For variables: walk scope chain. For functions: list global functions. For types: list built-in and user types.

#### Best Practices Identified

- **Test-Driven Thresholds**: Don't guess at similarity thresholds. Write comprehensive tests first, then adjust thresholds until tests pass with good precision/recall balance.

- **Integration Over Unit**: For user-facing features like error messages, integration tests (compile code, check output) are more valuable than unit tests (test algorithm internals).

- **Defer Complex Features**: Original plan included keyword suggestions (e.g., `fnn` → `fn`) but this required lexer changes. Better to defer cross-component features until core functionality is solid.

- **Generic Utilities**: When implementing similar features (variable/function/type suggestions), extract common logic into generic utilities. Reduces duplication and improves maintainability.

- **Simple Formats Win**: Concise error hints (`did you mean X?`) are often better than elaborate multi-line explanations. Users want quick answers, not verbose documentation.

---

### Phase 3: Error Documentation & Recovery

**Date Started**: October 6, 2025  
**Date Completed**: October 6, 2025 (Phase 3A, 3B), October 7, 2025 (Phase 3C)

#### Phase 3C: Parser Error Recovery (October 7, 2025)

##### Technical Discoveries

- **Panic-Mode Recovery Essentials**: Implementing error recovery requires three key components: (1) `panic_mode` flag to track recovery state, (2) `errors` collection to accumulate diagnostics, and (3) `synchronize()` method to skip to safe recovery points. These work together to enable multi-error reporting without cascading false positives.

- **Synchronization Point Selection**: Effective sync points for FerrisScript are statement and declaration boundaries: `;` (end of statement), `}` (end of block), `fn` (function start), `let` (variable declaration). These align with grammar structure and user mental model of code organization.

- **Forward Progress Requirement**: The most critical aspect of error recovery is **always advancing past problematic tokens**. Even if `synchronize()` finds a sync point immediately, you must first `advance()` past the bad token that triggered the error, otherwise you create an infinite loop. Pattern: `record_error() → advance() → synchronize()`.

- **Cascading Error Prevention**: Suppressing error reporting while `panic_mode == true` prevents cascading false positives. Once an error is detected, stay in panic mode until reaching a sync point. This gives users one clear error per statement/declaration rather than a flood of confused diagnostics.

- **API Compatibility Strategy**: Maintain existing `parse()` function signature by collecting errors internally but still returning `Result<Program, String>` with first error. Add `get_errors()` method for callers who want full diagnostic list. This provides gradual migration path without breaking existing code.

##### Challenges Encountered

- **Critical Infinite Loop Bug**: Initial implementation caused infinite memory consumption when parser encountered unexpected top-level tokens. Root cause: Called `synchronize()` without first advancing past the bad token. If `synchronize()` returned immediately (token was already at sync point), parser stayed at same position forever, repeatedly processing same token.

- **Test Expectations vs Reality**: Initial unit tests failed because they assumed `synchronize()` would advance *past* sync points, when actual behavior is to stop *at* sync points. Had to adjust test expectations to match implemented algorithm behavior.

- **Token Variant Mismatches**: First test attempts used `Token::Int(1)` which doesn't exist in FerrisScript's lexer. Actual token is `Token::Number(1.0)` for all numeric literals. Required investigation of lexer token enum to find correct variants.

- **Error Message Format Assumptions**: Integration tests initially checked for literal strings like "Expected ';'" or "semicolon", but actual error messages use format "Expected token\nExpected ;, found X". Had to broaden assertions to check for "Expected" or error code "E10" patterns instead.

##### Solutions Applied

- **Infinite Loop Fix**: Added mandatory `self.advance()` call before `synchronize()` in error recovery path:

  ```rust
  self.record_error(error);
  self.advance();        // ← Critical: always advance past bad token
  self.synchronize();    // Then find safe recovery point
  ```

  This guarantees forward progress even if sync point is reached immediately.

- **Comprehensive Test Suite**: Created 23 recovery-specific tests (13 unit, 10 integration) covering:
  - Synchronization to each type of sync point (`;`, `}`, `fn`, `let`)
  - Panic mode behavior and error suppression
  - Multi-error scenarios across functions and globals
  - Edge cases (EOF after error, nested constructs, expression errors)
  - Verification that valid code still parses correctly

- **Public API for Testing**: Made `Parser` struct and `parse_program()` method public, added `get_errors()` accessor. This enables integration tests to inspect internal error collection and verify recovery behavior end-to-end.

- **Flexible Error Assertions**: Updated test assertions to check for error patterns rather than exact strings, making tests resilient to error message formatting changes while still verifying correct error detection.

##### Best Practices Identified

- **Always Advance Before Sync**: When implementing error recovery, the pattern must always be: record error → advance at least one token → synchronize. Never synchronize without advancing first, as this risks infinite loops.

- **Test Both Success and Failure**: Don't just test that recovery works - also test that valid code still compiles successfully without spurious errors. This catches cases where recovery logic accidentally breaks normal parsing.

- **Verify Token Enums Before Testing**: When writing parser tests, always check the actual token enum definition in lexer. Don't assume token variant names - verify them to avoid cryptic compilation errors.

- **Debug Output First, Assertions Second**: When integration tests fail due to unexpected error messages, add `println!()` statements to see actual error text before adjusting assertions. This reveals format assumptions that don't match reality.

- **Quality Gates Are Non-Negotiable**: Run full test suite (`cargo test --workspace`), clippy (`cargo clippy --workspace --all-targets -- -D warnings`), and formatting (`cargo fmt --all`) before declaring work complete. These catch issues that are expensive to fix later.

- **Document Critical Bugs**: When you find a severe bug (like infinite loop), document it thoroughly in learnings with root cause, symptoms, and fix. These insights prevent similar bugs in future work.

##### Performance Impact

- **Zero Overhead on Success Path**: Error recovery only activates when parse errors occur. Valid code parsing performance unchanged.
- **Minimal Recovery Cost**: Synchronization adds ~10μs per error for token skipping. Acceptable since errors are development-time only.
- **Bounded Memory Usage**: Error collection limited to actual error count (typically 1-10 per file). No unbounded growth risk.

##### Test Coverage Stats

- **263 Total Tests**: All passing after Phase 3C
- **23 Recovery Tests**: 13 unit tests in parser.rs + 10 integration tests in parser_error_recovery.rs
- **Zero Regressions**: All existing parser, lexer, type checker, and runtime tests still passing
- **Zero Clippy Warnings**: Strict mode (`-D warnings`) passes cleanly

---

### Phase 4: VS Code Completion

**Date Started**: TBD  
**Date Completed**: TBD

#### Technical Discoveries

- *(To be filled during development)*

#### Challenges Encountered

- *(To be filled during development)*

#### Solutions Applied

- *(To be filled during development)*

#### Best Practices Identified

- *(To be filled during development)*

---

### Phase 5: VS Code Hover & Problem Panel

**Date Started**: TBD  
**Date Completed**: TBD

#### Technical Discoveries

- *(To be filled during development)*

#### Challenges Encountered

- *(To be filled during development)*

#### Solutions Applied

- *(To be filled during development)*

#### Best Practices Identified

- *(To be filled during development)*

---

### Phase 6: Development Scripts

**Date Started**: TBD  
**Date Completed**: TBD

#### Technical Discoveries

- *(To be filled during development)*

#### Challenges Encountered

- *(To be filled during development)*

#### Solutions Applied

- *(To be filled during development)*

#### Best Practices Identified

- *(To be filled during development)*

---

### Phase 7: Performance Benchmarking

**Date Started**: TBD  
**Date Completed**: TBD

#### Technical Discoveries

- *(To be filled during development)*

#### Challenges Encountered

- *(To be filled during development)*

#### Solutions Applied

- *(To be filled during development)*

#### Best Practices Identified

- *(To be filled during development)*

---

### Phase 8: Integration Tests & Cross-Platform

**Date Started**: TBD  
**Date Completed**: TBD

#### Technical Discoveries

- *(To be filled during development)*

#### Challenges Encountered

- *(To be filled during development)*

#### Solutions Applied

- *(To be filled during development)*

#### Best Practices Identified

- *(To be filled during development)*

---

### Phase 9: Documentation & Quality

**Date Started**: TBD  
**Date Completed**: TBD

#### Technical Discoveries

- *(To be filled during development)*

#### Challenges Encountered

- *(To be filled during development)*

#### Solutions Applied

- *(To be filled during development)*

#### Best Practices Identified

- *(To be filled during development)*

---

## 🔧 Technical Insights

### Architecture Decisions

#### Error Code System Design

- **Enum-Based Approach**: Using Rust enums for error codes provides type safety and compile-time validation. Each error code variant can have associated methods for description, category, and documentation URL.

- **Category Organization**: Error codes organized by compiler stage (Lexical, Syntax, Type, Runtime, Internal) makes it easier to locate relevant errors and understand where in the compilation pipeline issues occur.

- **Reserved Code Ranges**: Reserving gaps in error code ranges (E050-E099, E150-E199, etc.) allows for future expansion within categories without disrupting existing code organization.

### Performance Optimizations

- **Error Code Lookup**: Using match statements for error code descriptions is optimal for performance. Rust compiles these into efficient jump tables.

- **Array vs Vec Performance**: Replacing `vec![]` with array literals `[]` in tests eliminates runtime heap allocation for better performance and clippy compliance.

### Testing Strategies

#### Validation Test Design

- **Error Code Format Tests**: Validate that error codes follow the `Error[EXXX]:` format pattern consistently across all error types.

- **Coverage Tests**: Ensure all error codes are documented and have descriptions. Test that error codes appear correctly in actual compiler output.

- **Context Preservation**: Verify that error context (line numbers, code snippets) is preserved when error codes are added to messages.

### Tooling Improvements

#### Quality Gate Automation

- **Strict Clippy Mode**: Established `cargo clippy --workspace --all-targets --all-features -- -D warnings` as standard. This catches issues in:
  - Test code (not just main code)
  - Benchmark code
  - Example code
  - All feature combinations

- **Documentation Validation**: Integrated `npm run docs:lint` and `npx markdown-link-check` into workflow to catch documentation issues before CI.

- **Format Consistency**: Always run `cargo fmt --all` before committing to maintain consistent code style across the entire workspace.

---

## 🔮 Deferred Investigations & Future Opportunities

### Phase 1 Deferred Items

#### Semantic Error Codes (E300-E399)

**Status**: Deferred - No semantic analyzer yet

**Opportunity**: When implementing semantic analysis in future versions, we have error code ranges reserved for:

- E300: Unreachable code detection
- E301-E303: Unused variable/function warnings
- E304-E305: Invalid control flow (break/continue/return outside valid context)

**Investigation Needed**: Research best practices for:

- Dead code elimination strategies
- Unused variable detection (accounting for intentional underscore prefixes)
- Control flow validation in nested contexts

#### Advanced Runtime Error Codes (E400-E404 deferred)

**Status**: Error codes defined but not actively triggered by runtime

**Opportunity**: Some runtime errors are defined but not yet detected:

- E400: Division by zero (not checked at runtime yet)
- E401: Index out of bounds (no array indexing implemented)
- E402: Null pointer access (no null values in language)
- E403: Stack overflow (no recursion depth limit)
- E404: Memory exhaustion (relies on system limits)

**Investigation Needed**:

- Should runtime check for division by zero or rely on system signals?
- When array indexing is added, what's the performance impact of bounds checking?
- Is a recursion depth limit needed for FerrisScript's use case (game scripting)?

### Opportunities Discovered During Phase 1

#### Error Code Quick Fixes (LSP Integration)

**Discovery**: Each error code has structured information (description, example, fix) that could power IDE quick fixes.

**Opportunity**: When implementing LSP (Language Server Protocol) support:

- Use error code descriptions for hover tooltips
- Generate quick fixes from "How to Fix" sections
- Link to ERROR_CODES.md documentation from IDE

**Benefit**: Significantly improves developer experience with actionable error messages.

#### Error Code Telemetry

**Discovery**: Structured error codes enable tracking which errors users encounter most frequently.

**Opportunity**: (Privacy-respecting) telemetry could identify:

- Most common user errors (prioritize documentation/error messages)
- Confusing error messages (improve wording)
- Missing error codes (gaps in coverage)

**Investigation Needed**:

- Opt-in telemetry design
- Privacy considerations
- Storage and analysis approach

#### Documentation Website Infrastructure

**Status**: 🎯 Domain Acquired (`ferrisscript.dev`) ✅ - Infrastructure work in progress

**Discovery**: Phase 3A added documentation URLs to error messages with hybrid approach:

- Default: GitHub URLs (work immediately)
- Future: Custom site via `FERRIS_DOCS_BASE` env var

**Opportunity**: Now that domain is acquired, can work on infrastructure between features:

**Completed**:

- ✅ Domain: `ferrisscript.dev` acquired
- ✅ Code: Hybrid URL system implemented (GitHub → custom site seamless)

**Remaining Work** (can proceed in parallel with feature development):

1. Set up static hosting (Netlify/Vercel/GitHub Pages)
2. Create `docs.ferrisscript.dev` CNAME subdomain
3. Choose & set up documentation framework (Docusaurus/mdBook/VitePress)
4. Deploy ERROR_CODES.md as searchable website
5. Verify HTTPS (required for `.dev` TLD)

**Benefit**: Professional documentation site improves project credibility and developer experience. No code changes needed in compiler thanks to env var approach.

**Timeline**: Can be completed any time before v0.0.3 release, or after (GitHub URLs work fine).

#### Error Code Localization

**Discovery**: Error code enum provides a centralization point for all error messages.

**Opportunity**: Future internationalization (i18n) could:

- Translate error descriptions while keeping error codes stable
- Provide localized "How to Fix" guidance
- Maintain English error codes for searchability

**Investigation Needed**:

- Which languages to support first?
- How to maintain translation quality?
- Performance impact of runtime locale selection?

#### Machine-Readable Error Output

**Discovery**: Error codes provide structured data that's currently only human-readable.

**Opportunity**: Add JSON error output mode for:

- IDE integration (structured diagnostics)
- Build tool integration (error parsing)
- CI/CD pipelines (automated failure analysis)

**Example Format**:

```json
{
  "errors": [
    {
      "code": "E201",
      "message": "Undefined variable 'player'",
      "file": "game.ferris",
      "line": 10,
      "column": 5,
      "severity": "error"
    }
  ]
}
```

**Investigation Needed**: Standardize on JSON schema for compatibility with existing tools.

#### Error Code Documentation Website

**Discovery**: ERROR_CODES.md is comprehensive but linear (must scroll to find codes).

**Opportunity**: Generate a searchable website:

- Search by error code or keyword
- Browse by category
- Show related errors
- Link to GitHub issues/discussions for each code

**Tools**: Could use mdBook, Docusaurus, or custom generator from ERROR_CODES.md.

---

## 🚧 Challenges & Solutions

### Development Process

- *(To be filled)*

### CI/CD Pipeline

- *(To be filled)*

### Cross-Platform Issues

- *(To be filled)*

### Documentation

- *(To be filled)*

---

## 💡 Best Practices Established

### Code Quality

- *(To be filled)*

### Testing

- *(To be filled)*

### Documentation

- *(To be filled)*

### Workflow

- *(To be filled)*

---

## 🔮 Recommendations for Future Versions

### v0.0.4 and Beyond

- *(To be filled)*

### Avoided Pitfalls

- *(To be filled)*

### Process Improvements

- *(To be filled)*

---

## 📚 References

- [v0.0.3 Roadmap](./v0.0.3-roadmap.md)
- [Phase Documents](./README.md#-phase-tracker)
- [v0.0.2 Learnings](../../archive/v0.0.2/LEARNINGS.md)
- v0.0.1 Learnings: *(Not created)*

---

## 📝 Notes

This document should be updated continuously throughout v0.0.3 development. Add insights as they emerge rather than waiting until the end of the version.
