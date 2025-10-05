# Phase 5B: Syntax Highlighting - Execution Plan

**Phase**: 5B - Syntax Highlighting Foundation  
**Started**: October 5, 2025  
**Status**: Planning → In Progress  
**Branch**: `feature/v0.0.2-phase5b-syntax-highlighting`

---

## Q&A: Context Gathering

### Workstream Context

**Q1: What is the primary goal?**  
A: Create VS Code syntax highlighting extension for `.ferris` files to provide basic editor support and improve developer experience.

**Q2: What version is this for?**  
A: v0.0.2 (Patch Release - Foundation & Polish)

**Q3: What type of release?**  
A: Patch release focusing on tooling and developer experience improvements

**Q4: Why is this work important?**  
A: First step in editor integration priority (reprioritized v0.1.0 roadmap). Provides immediate value to developers and establishes foundation for future LSP work.

**Q5: What's the source of requirements?**  
A: v0.0.2-roadmap.md (Section 5: Syntax Highlighting Foundation)

### About Prior Work

**Q6: Has similar work been done before?**  
A: No prior syntax highlighting work. This is the first editor integration effort.

**Q7: Are there existing tests?**  
A: Not applicable (syntax highlighting is declarative configuration)

**Q8: What documentation exists?**  
A: Example .ferris scripts in `examples/` directory show language syntax

**Q9: What patterns should I follow?**  
A: Standard VS Code extension patterns, TextMate grammar format

**Q10: What should I NOT change?**  
A: Core language syntax/keywords (read from lexer.rs)

### About Constraints

**Q11: What changes are allowed?**  
A: New extension files, documentation updates, no code changes to compiler/runtime

**Q12: What changes are NOT allowed?**  
A: Breaking changes to existing .ferris scripts, language feature additions

**Q13: Are there performance requirements?**  
A: Syntax highlighting should be instant (<100ms for typical files)

**Q14: Are there platform considerations?**  
A: VS Code extension must work on Windows, Linux, macOS

**Q15: What's the timeline?**  
A: 4-6 hours estimated (part of v0.0.2, ~15-20 hours remaining total)

### About Quality Standards

**Q16: What tests must pass?**  
A: Manual testing on example .ferris scripts, visual verification

**Q17: What linting must pass?**  
A: Markdown linting for documentation (`npm run docs:lint`)

**Q18: What's the test coverage target?**  
A: N/A (declarative configuration)

**Q19: What's the documentation requirement?**  
A: Update v0.0.2-CHECKLIST.md, v0.0.2-roadmap.md, create summary document

**Q20: What's the code review process?**  
A: Self-review, PR checklist, validation on example scripts

### About Contribution Workflow

**Q21: What branch should I create?**  
A: `feature/v0.0.2-phase5b-syntax-highlighting`

**Q22: What's the commit message format?**  
A: Conventional Commits: `feat(vscode): add syntax highlighting for .ferris files`

**Q23: Where should files go?**  
A: New directory: `extensions/vscode/` (standard VS Code extension structure)

**Q24: What documents need updating?**  
A: v0.0.2-CHECKLIST.md, v0.0.2-roadmap.md, planning/README.md, README.md

**Q25: How should I track progress?**  
A: TODO list via manage_todo_list tool

### Decisions Made

**Decision 1: Use TextMate Grammar**  

- Standard for VS Code syntax highlighting
- Well-documented, declarative format
- Easy to maintain and update

**Decision 2: Start with Core Features Only**  

- Keywords, types, operators, comments, strings
- Defer advanced features (semantic highlighting) to LSP phase
- Focus on immediate value

**Decision 3: Create Grammar Update Strategy**  

- Document grammar audit process in CONTRIBUTING.md
- Add checklist item: "Update syntax highlighting grammar" when adding language features
- Create `docs/SYNTAX_HIGHLIGHTING_MAINTENANCE.md` guide

**Trade-off 1: TextMate vs Semantic Highlighting**  

- Chose TextMate for simplicity and immediate availability
- Semantic highlighting requires LSP (deferred to v0.0.5)
- TextMate provides 80% of value for 20% of effort

**Trade-off 2: Minimal Extension vs Full Features**  

- Starting with syntax highlighting only (no completion, hover, etc.)
- Full language server features deferred to v0.0.5
- Allows faster delivery and user feedback

---

## Acceptance Criteria

### Functional Requirements

1. ✅ **TextMate grammar created** for `.ferris` files
2. ✅ **All keywords highlighted** (fn, let, mut, if, else, while, return, true, false)
3. ✅ **All types highlighted** (i32, f32, bool, String, Vector2, Node)
4. ✅ **Operators highlighted** (+, -, *, /, =, ==, !=, <, >, <=, >=, &&, ||, !, +=, -=)
5. ✅ **Comments highlighted** (line comments starting with //)
6. ✅ **Strings highlighted** (double-quoted strings)
7. ✅ **Numbers highlighted** (integer and float literals)
8. ✅ **Functions highlighted** (function names in definitions and calls)
9. ✅ **Code snippets created** (_ready,_process, let, fn, if, while)

### Quality Requirements

1. ✅ **All example scripts render correctly** (hello.ferris, move.ferris, bounce.ferris)
2. ✅ **Extension manifest valid** (package.json follows VS Code spec)
3. ✅ **Documentation updated** (v0.0.2-CHECKLIST.md, v0.0.2-roadmap.md, README.md)
4. ✅ **Grammar maintenance strategy documented**
5. ✅ **Summary document created** with learnings and next steps

### Deliverables

1. ✅ **VS Code extension structure** in `extensions/vscode/`
2. ✅ **TextMate grammar file** (ferrisscript.tmLanguage.json)
3. ✅ **Code snippets file** (ferrisscript.json)
4. ✅ **Extension manifest** (package.json)
5. ✅ **README for extension** (extensions/vscode/README.md)
6. ✅ **Maintenance documentation** (docs/SYNTAX_HIGHLIGHTING_MAINTENANCE.md)

---

## Execution Phases

### Phase 0: Planning ✅

- [x] Asked clarifying questions
- [x] Recorded Q&A
- [x] Created execution plan
- [x] Defined acceptance criteria
- [x] Analyzed language features from lexer.rs and type_checker.rs
- [x] Examined example scripts

### Phase 1: Extension Structure Setup 🔄

**Goal**: Create VS Code extension file structure

**Tasks**:

- [ ] Create branch: `feature/v0.0.2-phase5b-syntax-highlighting`
- [ ] Create directory: `extensions/vscode/`
- [ ] Create `extensions/vscode/package.json` (extension manifest)
- [ ] Create `extensions/vscode/README.md` (extension documentation)
- [ ] Create `extensions/vscode/.vscodeignore` (build exclusions)
- [ ] Create `extensions/vscode/CHANGELOG.md` (version history)

**Acceptance**: Directory structure created, manifest validates

**Estimated Time**: 30 minutes

### Phase 2: TextMate Grammar ⏸️

**Goal**: Create syntax highlighting grammar

**Tasks**:

- [ ] Create `extensions/vscode/syntaxes/ferrisscript.tmLanguage.json`
- [ ] Define file associations (`.ferris`)
- [ ] Add scope name: `source.ferrisscript`
- [ ] Define patterns for:
  - [ ] Keywords (fn, let, mut, if, else, while, return, true, false)
  - [ ] Types (i32, f32, bool, String, Vector2, Node, void)
  - [ ] Operators (+, -, *, /, =, ==, !=, <, >, <=, >=, &&, ||, !, +=, -=)
  - [ ] Comments (// line comments)
  - [ ] Strings ("double-quoted")
  - [ ] Numbers (42, 3.14, -5.0)
  - [ ] Function definitions (fn name)
  - [ ] Function calls (name())
  - [ ] Identifiers (variables)
  - [ ] Special identifiers (self)
  - [ ] Punctuation (braces, parens, semicolons)
- [ ] Test grammar on example scripts

**Acceptance**: All language features highlighted correctly

**Estimated Time**: 2 hours

### Phase 3: Code Snippets ⏸️

**Goal**: Create productivity snippets

**Tasks**:

- [ ] Create `extensions/vscode/snippets/ferrisscript.json`
- [ ] Add snippet: `_ready` function
- [ ] Add snippet: `_process` function  
- [ ] Add snippet: `let` variable declaration
- [ ] Add snippet: `let mut` mutable variable
- [ ] Add snippet: `fn` function definition
- [ ] Add snippet: `if` statement
- [ ] Add snippet: `if-else` statement
- [ ] Add snippet: `while` loop
- [ ] Test snippets in VS Code

**Acceptance**: All snippets work and follow conventions

**Estimated Time**: 45 minutes

### Phase 4: Documentation Updates ⏸️

**Goal**: Update project documentation

**Tasks**:

- [ ] Update `v0.0.2-CHECKLIST.md`:
  - [ ] Mark "Create TextMate grammar" as complete
  - [ ] Mark "Add VS Code extension manifest" as complete
  - [ ] Mark "Create basic code snippets" as complete
- [ ] Update `docs/planning/v0.0.2-roadmap.md`:
  - [ ] Mark Phase 5B as complete
  - [ ] Update progress percentage
- [ ] Update `docs/planning/README.md`:
  - [ ] Update v0.0.2 status
- [ ] Update root `README.md`:
  - [ ] Add section on editor support
  - [ ] Link to VS Code extension
- [ ] Create `docs/SYNTAX_HIGHLIGHTING_MAINTENANCE.md`:
  - [ ] Document how to update grammar
  - [ ] Add audit checklist
  - [ ] Link to TextMate grammar docs
- [ ] Update `CONTRIBUTING.md`:
  - [ ] Add section on syntax highlighting maintenance
  - [ ] Add to "When adding language features" checklist

**Acceptance**: All documentation current and accurate

**Estimated Time**: 1 hour

### Phase 5: Testing & Validation ⏸️

**Goal**: Verify highlighting works correctly

**Tasks**:

- [ ] Install extension locally in VS Code
- [ ] Open `examples/hello.ferris` - verify highlighting
- [ ] Open `examples/move.ferris` - verify highlighting
- [ ] Open `examples/bounce.ferris` - verify highlighting
- [ ] Test all code snippets
- [ ] Run documentation linting: `npm run docs:lint`
- [ ] Fix any linting issues: `npm run docs:fix`
- [ ] Verify all links in updated docs
- [ ] Screenshot examples for documentation

**Acceptance**: All examples render correctly, no linting errors

**Estimated Time**: 30 minutes

### Phase 6: Summary & PR ⏸️

**Goal**: Create completion summary and PR

**Tasks**:

- [ ] Create `docs/v0.0.2/PHASE_5B_SYNTAX_HIGHLIGHTING_SUMMARY.md`
- [ ] Document learnings and discoveries
- [ ] Document grammar maintenance strategy
- [ ] Note deferred features (semantic highlighting)
- [ ] Commit all changes
- [ ] Push to feature branch
- [ ] Create pull request
- [ ] Update PR description with feature template

**Acceptance**: PR created, summary complete

**Estimated Time**: 30 minutes

---

## Language Feature Inventory

**From `lexer.rs` Token enum:**

**Keywords** (9):

- `fn` - Function definition
- `let` - Variable declaration
- `mut` - Mutable modifier
- `if` - Conditional
- `else` - Else clause
- `while` - Loop
- `return` - Return statement
- `true` - Boolean literal
- `false` - Boolean literal

**Types** (from `type_checker.rs` Type enum):

- `i32` - 32-bit integer
- `f32` - 32-bit float
- `bool` - Boolean
- `String` - String type
- `Vector2` - Godot Vector2
- `Node` - Godot Node
- `void` - Void/unit type

**Operators**:

- Arithmetic: `+`, `-`, `*`, `/`
- Assignment: `=`, `+=`, `-=`
- Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical: `&&`, `||`, `!`

**Delimiters**:

- `(`, `)` - Parentheses
- `{`, `}` - Braces
- `,` - Comma
- `;` - Semicolon
- `.` - Dot (property access)
- `:` - Colon (type annotation)

**Literals**:

- Numbers: `42`, `3.14`, `-5.0`
- Strings: `"hello"`
- Booleans: `true`, `false`

**Special**:

- `self` - Current object reference
- Comments: `// line comment`

---

## Grammar Update Strategy

### When to Update Grammar

**Trigger Events** (add grammar update as checklist item):

1. **New keywords added** (e.g., `match`, `for`, `struct`)
2. **New operators added** (e.g., `%`, `**`, `??`)
3. **New literal types** (e.g., array literals `[1, 2, 3]`)
4. **New syntax constructs** (e.g., attributes `@export`)

### Audit Process

**Quarterly Audit** (every 3 months):

1. Review `lexer.rs` Token enum for new tokens
2. Review `type_checker.rs` Type enum for new types
3. Compare against `ferrisscript.tmLanguage.json`
4. Update grammar file with missing features
5. Test on all example scripts
6. Document changes in extension CHANGELOG.md

**On Language Feature PR**:

1. PR author checks: "Does this add new syntax?"
2. If yes: "Have you updated syntax highlighting grammar?"
3. Required files to update:
   - `extensions/vscode/syntaxes/ferrisscript.tmLanguage.json`
   - `extensions/vscode/CHANGELOG.md`
   - Test highlighting on relevant example

### Documentation Location

- **Maintenance Guide**: `docs/SYNTAX_HIGHLIGHTING_MAINTENANCE.md`
- **CONTRIBUTING.md**: Link to maintenance guide
- **Extension README**: Link to grammar file

---

## Deliverables

### Code

**New Directory Structure**:

```
extensions/
  vscode/
    syntaxes/
      ferrisscript.tmLanguage.json    # TextMate grammar
    snippets/
      ferrisscript.json                # Code snippets
    package.json                       # Extension manifest
    README.md                          # Extension docs
    CHANGELOG.md                       # Version history
    .vscodeignore                      # Build exclusions
```

### Documentation

- `docs/v0.0.2/PHASE_5B_SYNTAX_HIGHLIGHTING_SUMMARY.md` - Completion summary
- `docs/SYNTAX_HIGHLIGHTING_MAINTENANCE.md` - Maintenance guide
- Updates to:
  - `docs/v0.0.2/v0.0.2-CHECKLIST.md`
  - `docs/planning/v0.0.2-roadmap.md`
  - `docs/planning/README.md`
  - `README.md`
  - `CONTRIBUTING.md`

---

## Risk Assessment

### Low Risk

- **Grammar syntax errors**: Caught by VS Code validation
- **Missing keywords**: Easy to add incrementally
- **Broken snippets**: Quick to fix and test

### Medium Risk

- **Scope naming conflicts**: May require iteration
- **Performance on large files**: Unlikely with simple grammar
- **Cross-platform issues**: VS Code handles portability

### Mitigation Strategies

1. **Start simple**: Core keywords first, iterate
2. **Test frequently**: Reload extension after each change
3. **Reference examples**: Study other TextMate grammars (Rust, TypeScript)
4. **Document process**: Make it easy for next person

---

## Success Metrics

### Quantitative

- ✅ All 9 keywords highlighted
- ✅ All 7 types highlighted
- ✅ All operators highlighted
- ✅ Comments and strings highlighted
- ✅ 8 code snippets created
- ✅ All 3 example scripts render correctly
- ✅ Extension manifest validates
- ✅ Documentation linting passes

### Qualitative

- ✅ Syntax highlighting looks professional
- ✅ Colors match VS Code conventions
- ✅ Grammar is maintainable
- ✅ Snippets improve productivity
- ✅ Documentation is clear

---

## Next Steps (Post-Phase 5B)

### Immediate (v0.0.2)

- Phase 5C: Documentation polish (README, TESTING.md)
- Phase 6: Release preparation (CHANGELOG, version updates, tag)

### Short Term (v0.0.3)

- Publish extension to VS Code marketplace
- Add extension download badge to README
- Collect user feedback on highlighting

### Medium Term (v0.0.5)

- Language Server Protocol (LSP) implementation
- Semantic highlighting (context-aware colors)
- IntelliSense (completion, hover, go-to-definition)
- Problem panel integration (real-time errors)

### Long Term (v0.1.0+)

- Debugger integration
- Test runner integration
- Refactoring support

---

## References

### TextMate Grammar

- [VS Code Language Extensions](https://code.visualstudio.com/api/language-extensions/syntax-highlight-guide)
- [TextMate Grammar Docs](https://macromates.com/manual/en/language_grammars)
- [Scope Naming Conventions](https://www.sublimetext.com/docs/scope_naming.html)

### Example Grammars

- [Rust TextMate Grammar](https://github.com/microsoft/vscode/blob/main/extensions/rust/syntaxes/rust.tmLanguage.json)
- [TypeScript TextMate Grammar](https://github.com/microsoft/vscode/blob/main/extensions/typescript-basics/syntaxes/TypeScript.tmLanguage.json)

### VS Code Extension Development

- [Your First Extension](https://code.visualstudio.com/api/get-started/your-first-extension)
- [Extension Manifest](https://code.visualstudio.com/api/references/extension-manifest)
- [Snippets Guide](https://code.visualstudio.com/api/language-extensions/snippet-guide)

---

**Status**: Planning Complete → Ready for Phase 1  
**Next Action**: Create feature branch and extension structure
