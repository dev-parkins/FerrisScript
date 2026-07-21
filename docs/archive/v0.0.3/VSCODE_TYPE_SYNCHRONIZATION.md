# VS Code Extension Type Synchronization

## Overview

The VS Code extension provides code completion for FerrisScript types. These types are currently **manually maintained** in `src/completion/types.ts` and must be kept in sync with the compiler's type system.

## Current State

**Completion Types** (as of v0.0.3):

- Located in: `extensions/vscode/src/completion/types.ts`
- Types defined: `i32`, `f32`, `bool`, `String`, `Vector2`, `Node`, `void`

**Compiler Types**:

- Located in: `crates/compiler/src/lexer.rs` (TokenKind enum)
- Located in: `crates/compiler/src/type_checker.rs` (Type enum)

## Synchronization Requirements

### When to Update VS Code Types

1. **Adding a new primitive type to the language**
   - Update `crates/compiler/src/type_checker.rs` Type enum
   - Update `crates/compiler/src/lexer.rs` TokenKind if it's a keyword
   - **→ MUST UPDATE** `extensions/vscode/src/completion/types.ts`
   - **→ MUST UPDATE** `extensions/vscode/syntaxes/ferrisscript.tmLanguage.json`

2. **Adding a new Godot type binding**
   - Update `crates/godot_bind/src/types.rs` or equivalent
   - **→ MUST UPDATE** `extensions/vscode/src/completion/types.ts`

3. **Removing or renaming a type**
   - Update compiler and runtime
   - **→ MUST UPDATE** all VS Code extension type references

### Current Manual Process

1. Identify type changes in compiler/runtime
2. Open `extensions/vscode/src/completion/types.ts`
3. Add/modify/remove type entries in `TYPES` array:

   ```typescript
   {
       label: 'TypeName',
       detail: 'Short description',
       documentation: 'Full documentation with examples'
   }
   ```

4. Rebuild extension: `cd extensions/vscode && npm run compile`
5. Test completion in VS Code

## Future Automation Recommendations

### Phase 1: Validation Script (Near-term)

Create a script to detect type mismatches between compiler and extension:

```bash
# scripts/validate-vscode-types.sh
# Compare compiler Type enum with VS Code completion types
# Exit with error if mismatches found
```

**Add to CI/CD**: Run this validation on pull requests

### Phase 2: Type Generation (Long-term)

Generate VS Code types from compiler source of truth:

```bash
# scripts/generate-vscode-types.sh
# Parse crates/compiler/src/type_checker.rs
# Generate extensions/vscode/src/completion/types.ts
# Include documentation from doc comments
```

**Approaches**:

- **Option A**: Parse Rust AST using `syn` crate, extract Type enum variants
- **Option B**: Maintain types in a JSON schema, generate both Rust and TypeScript
- **Option C**: Use build.rs to export type list during compilation

### Phase 3: Full LSP Implementation (v0.0.5+)

When implementing Language Server Protocol:

- LSP server dynamically provides type information
- No manual sync needed - types always match compiler
- VS Code extension queries LSP for available types

## Roadmap Integration

### Added to v0.0.3 Recommendations

- Document type sync requirements (✅ This document)
- Add validation script to detect type mismatches

### Proposed for v0.0.4

- **Task**: Implement type validation script
- **Task**: Add pre-commit hook to run validation
- **Task**: Add CI check for type consistency

### Proposed for v0.1.0+

- **Task**: Implement type generation from compiler source
- **Task**: Automate regeneration in build pipeline
- **Task**: Consider JSON schema approach for multi-language support

### Target for v0.0.5 (LSP)

- **Task**: Replace static types with LSP-provided types
- **Result**: Full dynamic type synchronization

## References

- Compiler types: `crates/compiler/src/type_checker.rs`
- Lexer tokens: `crates/compiler/src/lexer.rs`
- VS Code types: `extensions/vscode/src/completion/types.ts`
- Syntax highlighting: `extensions/vscode/syntaxes/ferrisscript.tmLanguage.json`
- Phase 4 completion: `docs/planning/v0.0.3/PHASE_4_VS_CODE_COMPLETION.md`
