# Phase 4: VS Code Completion - Execution Plan

**Date**: October 7, 2025  
**Agent**: GitHub Copilot  
**Status**: In Progress  
**Branch**: `feature/v0.0.3-phase-4-completion`  
**PR**: *(To be created)*

---

## 🎯 Context

### Workstream Goal

Add code completion functionality to the FerrisScript VS Code extension, providing IntelliSense-like experience for keywords, types, and built-in functions.

### Strategic Context

- **Version**: v0.0.3 (Editor Experience Alpha)
- **Phase**: 4 of 9
- **Dependencies**: None (extension foundation exists from v0.0.2)
- **Enables**: Phase 5 (hover tooltips and problem panel integration)
- **Type**: Feature addition to existing VS Code extension

### Prior Work

- ✅ v0.0.2: Basic syntax highlighting and snippets established
- ✅ Extension structure exists: `extensions/vscode/`
- ✅ TextMate grammar working: `syntaxes/ferrisscript.tmLanguage.json`
- ❌ No TypeScript extension code yet (only declarative JSON)
- ❌ No IntelliSense or completion functionality

### Constraints

- **No LSP yet**: This is a lightweight completion provider, not full LSP (deferred to v0.0.5)
- **No compiler integration**: Completions are static (keyword lists, type lists)
- **Simple context awareness**: Based on cursor position and surrounding text patterns
- **Keep lightweight**: No external dependencies, use VS Code built-in APIs only

---

## 📋 Requirements Summary

### Functional Requirements

1. **Keyword Completion**: Complete FerrisScript keywords as user types
   - Control flow: `if`, `else`, `while`, `return`
   - Declarations: `fn`, `let`, `mut`
   - Literals: `true`, `false`

2. **Type Completion**: Complete type names in type position contexts
   - Primitives: `i32`, `f32`, `bool`, `String`
   - Godot types: `Vector2`, `Node`, `void`

3. **Function Completion**: Complete built-in function names
   - `print` with parameter hint `(message: String)`

4. **Context-Aware Triggering**: Show relevant completions based on cursor context
   - After `:` → Show type completions
   - At statement start → Show `let`, `fn`, `if`, `while`, `return`
   - In expression context → Show all keywords + functions

### Non-Functional Requirements

- **Performance**: Completion suggestions appear within 50ms
- **Accuracy**: Only show contextually relevant completions
- **UX**: Include descriptions and documentation for each completion item
- **Maintainability**: Completions defined in structured data (easy to extend)

### Out of Scope (Deferred to v0.0.5 LSP)

- ❌ Variable/function name completion (requires symbol table)
- ❌ Smart type inference (requires type checker integration)
- ❌ Go-to-definition
- ❌ Find references
- ❌ Rename refactoring

---

## 🏗️ Technical Architecture

### Extension Structure

```
extensions/vscode/
├── src/
│   ├── extension.ts           # Entry point, activate/deactivate
│   ├── completion/
│   │   ├── provider.ts        # Main CompletionItemProvider
│   │   ├── keywords.ts        # Keyword completions data
│   │   ├── types.ts           # Type completions data
│   │   └── functions.ts       # Function completions data
│   └── utils/
│       └── context.ts         # Context detection utilities
├── package.json               # Updated with activation events
├── tsconfig.json              # TypeScript configuration
├── .vscodeignore              # Updated build artifacts
└── README.md                  # Updated with completion docs
```

### Key Components

#### 1. Extension Activation

**File**: `src/extension.ts`

```typescript
import * as vscode from 'vscode';
import { FerrisScriptCompletionProvider } from './completion/provider';

export function activate(context: vscode.ExtensionContext) {
    const provider = new FerrisScriptCompletionProvider();
    
    const disposable = vscode.languages.registerCompletionItemProvider(
        { scheme: 'file', language: 'ferrisscript' },
        provider,
        '.', // Trigger on dot for member access (future)
        ':' // Trigger on colon for type hints
    );
    
    context.subscriptions.push(disposable);
}

export function deactivate() {}
```

#### 2. Completion Provider

**File**: `src/completion/provider.ts`

```typescript
import * as vscode from 'vscode';
import { getKeywordCompletions } from './keywords';
import { getTypeCompletions } from './types';
import { getFunctionCompletions } from './functions';
import { detectContext, CompletionContext } from '../utils/context';

export class FerrisScriptCompletionProvider implements vscode.CompletionItemProvider {
    provideCompletionItems(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken,
        context: vscode.CompletionContext
    ): vscode.ProviderResult<vscode.CompletionItem[] | vscode.CompletionList> {
        const ctx = detectContext(document, position);
        
        switch (ctx) {
            case CompletionContext.TypePosition:
                return getTypeCompletions();
            case CompletionContext.StatementStart:
                return getKeywordCompletions(true); // Statement-level keywords only
            case CompletionContext.Expression:
                return [
                    ...getKeywordCompletions(false), // All keywords
                    ...getFunctionCompletions()
                ];
            default:
                return [];
        }
    }
}
```

#### 3. Keyword Completions

**File**: `src/completion/keywords.ts`

```typescript
import * as vscode from 'vscode';

interface KeywordData {
    label: string;
    detail: string;
    documentation: string;
    insertText: string;
    statementLevel: boolean;
}

const KEYWORDS: KeywordData[] = [
    {
        label: 'fn',
        detail: 'function declaration',
        documentation: 'Declares a new function\n\nExample:\nfn my_function(param: i32) -> i32 {\n    return param + 1;\n}',
        insertText: 'fn ${1:name}($2) {\n    $0\n}',
        statementLevel: true
    },
    {
        label: 'let',
        detail: 'variable declaration',
        documentation: 'Declares an immutable variable\n\nExample:\nlet x: i32 = 42;',
        insertText: 'let ${1:name}: ${2:i32} = $0;',
        statementLevel: true
    },
    // ... more keywords
];

export function getKeywordCompletions(statementLevelOnly: boolean): vscode.CompletionItem[] {
    const filtered = statementLevelOnly 
        ? KEYWORDS.filter(k => k.statementLevel)
        : KEYWORDS;
    
    return filtered.map(kw => {
        const item = new vscode.CompletionItem(kw.label, vscode.CompletionItemKind.Keyword);
        item.detail = kw.detail;
        item.documentation = new vscode.MarkdownString(kw.documentation);
        item.insertText = new vscode.SnippetString(kw.insertText);
        return item;
    });
}
```

#### 4. Context Detection

**File**: `src/utils/context.ts`

```typescript
import * as vscode from 'vscode';

export enum CompletionContext {
    TypePosition,      // After `: ` in declarations
    StatementStart,    // At start of line (after whitespace)
    Expression,        // Inside expression context
    Unknown
}

export function detectContext(
    document: vscode.TextDocument,
    position: vscode.Position
): CompletionContext {
    const line = document.lineAt(position.line).text;
    const beforeCursor = line.substring(0, position.character);
    
    // Type position: "let x: |" or "fn foo(param: |"
    if (/:\s*$/.test(beforeCursor)) {
        return CompletionContext.TypePosition;
    }
    
    // Statement start: "    |" (only whitespace before cursor)
    if (/^\s*$/.test(beforeCursor)) {
        return CompletionContext.StatementStart;
    }
    
    // Default to expression context
    return CompletionContext.Expression;
}
```

### Activation Events

**Update**: `package.json`

```json
{
  "activationEvents": [
    "onLanguage:ferrisscript"
  ],
  "main": "./out/extension.js",
  "scripts": {
    "vscode:prepublish": "npm run compile",
    "compile": "tsc -p ./",
    "watch": "tsc -watch -p ./",
    "lint": "eslint src --ext ts"
  },
  "devDependencies": {
    "@types/vscode": "^1.75.0",
    "@types/node": "^18.x",
    "typescript": "^5.0.0",
    "eslint": "^8.0.0",
    "@typescript-eslint/eslint-plugin": "^5.0.0",
    "@typescript-eslint/parser": "^5.0.0"
  }
}
```

---

## ✅ Acceptance Criteria

### Criterion 1: Keyword Completion Works

**Given**: User opens a `.ferris` file and starts typing  
**When**: User types `l` at statement start  
**Then**: Completion suggests `let` with proper snippet expansion

**Evidence**: Manual testing with VS Code extension

---

### Criterion 2: Type Completion Context-Aware

**Given**: User types `let x:` in editor  
**When**: User triggers completion (Ctrl+Space) or types  
**Then**: Only type completions shown (i32, f32, bool, String, Vector2, Node, void)

**Evidence**: Manual testing with type declarations

---

### Criterion 3: Function Completion Shows Parameters

**Given**: User types `p` in expression context  
**When**: User selects `print` completion  
**Then**: Snippet expands to `print($0)` with cursor inside parentheses

**Evidence**: Manual testing with function calls

---

### Criterion 4: Completion Descriptions Helpful

**Given**: User hovers over completion suggestion  
**When**: Completion item is focused  
**Then**: Shows detail (e.g., "variable declaration") and documentation with examples

**Evidence**: Manual inspection of completion items

---

### Criterion 5: No Errors in Extension Host

**Given**: Extension is activated  
**When**: User interacts with completions  
**Then**: No errors logged in VS Code Extension Host output panel

**Evidence**: Check "Developer: Show Logs... → Extension Host"

---

### Criterion 6: Build and Package Successfully

**Given**: TypeScript source code complete  
**When**: Run `npm run compile`  
**Then**: Compiles without errors, generates `out/extension.js`

**Evidence**: CI-like local build check

---

## 🧪 Test Strategy

### Manual Testing Checklist

- [ ] **Keyword Completion**:
  - [ ] Type `l` → suggests `let` and `let mut`
  - [ ] Type `f` at statement start → suggests `fn`
  - [ ] Type `i` at statement start → suggests `if`
  - [ ] Type `w` at statement start → suggests `while`
  - [ ] Type `r` → suggests `return`

- [ ] **Type Completion**:
  - [ ] Type `let x:` → suggests only types (i32, f32, bool, String, Vector2, Node, void)
  - [ ] Type `fn foo() ->` → suggests return types
  - [ ] Type `fn bar(param:` → suggests parameter types

- [ ] **Function Completion**:
  - [ ] Type `p` in function body → suggests `print`
  - [ ] Select `print` → expands to `print($0)` with cursor positioned

- [ ] **Context Awareness**:
  - [ ] Completions at statement start don't show expression-only keywords
  - [ ] Type context doesn't show keywords
  - [ ] Expression context shows full range

- [ ] **Documentation Quality**:
  - [ ] Each completion shows helpful detail text
  - [ ] Markdown documentation renders correctly
  - [ ] Examples in documentation are accurate

### Automated Testing

⚠️ **Limitation**: VS Code extension testing requires complex setup. For Phase 4, we'll rely on:

- Manual testing (checklist above)
- TypeScript compilation as validation
- Extension loads without errors

**Future**: Add VS Code extension test suite in Phase 5 when LSP work begins.

---

## 📦 Implementation Phases

### Phase 4A: TypeScript Infrastructure ✅

**Tasks**:

1. Create `tsconfig.json` for VS Code extension
2. Update `package.json` with TypeScript dependencies and scripts
3. Create `src/extension.ts` entry point
4. Add npm scripts: `compile`, `watch`, `lint`
5. Test extension activates without errors

**Files Created/Modified**:

- `extensions/vscode/tsconfig.json` (new)
- `extensions/vscode/package.json` (modified)
- `extensions/vscode/src/extension.ts` (new)
- `extensions/vscode/.vscodeignore` (updated)

**Validation**: `npm run compile` succeeds, extension activates in development host

---

### Phase 4B: Keyword Completion ✅

**Tasks**:

1. Create `src/completion/keywords.ts` with keyword data
2. Implement `getKeywordCompletions()` function
3. Add snippet support for common keywords
4. Test keyword completion manually

**Files Created/Modified**:

- `extensions/vscode/src/completion/keywords.ts` (new)
- `extensions/vscode/src/completion/provider.ts` (new/modified)

**Validation**: Type `let` → completion works, snippet expands correctly

---

### Phase 4C: Type Completion ✅

**Tasks**:

1. Create `src/completion/types.ts` with type data
2. Implement `getTypeCompletions()` function
3. Add documentation for each type
4. Test type completion in type position

**Files Created/Modified**:

- `extensions/vscode/src/completion/types.ts` (new)
- `extensions/vscode/src/completion/provider.ts` (modified)

**Validation**: Type `let x:` → only types suggested

---

### Phase 4D: Function Completion ✅

**Tasks**:

1. Create `src/completion/functions.ts` with function data
2. Implement `getFunctionCompletions()` function
3. Add parameter hints to completion items
4. Test function completion in expression context

**Files Created/Modified**:

- `extensions/vscode/src/completion/functions.ts` (new)
- `extensions/vscode/src/completion/provider.ts` (modified)

**Validation**: Type `print` → completion works with parameter snippet

---

### Phase 4E: Context Detection ✅

**Tasks**:

1. Create `src/utils/context.ts` with context detection logic
2. Implement regex-based context detection
3. Wire context detection into completion provider
4. Test context-aware completion behavior

**Files Created/Modified**:

- `extensions/vscode/src/utils/context.ts` (new)
- `extensions/vscode/src/completion/provider.ts` (modified)

**Validation**: Completions adapt to cursor position correctly

---

### Phase 4F: Documentation & Testing ✅

**Tasks**:

1. Update `extensions/vscode/README.md` with completion docs
2. Update `extensions/vscode/CHANGELOG.md` with v0.0.3 changes
3. Run full manual testing checklist
4. Verify no errors in Extension Host logs

**Files Created/Modified**:

- `extensions/vscode/README.md` (modified)
- `extensions/vscode/CHANGELOG.md` (modified)

**Validation**: All manual tests pass, documentation accurate

---

### Phase 4G: Roadmap Alignment ✅

**Tasks**:

1. Update `docs/planning/v0.0.3/README.md` Phase 4 status to complete
2. Update `docs/planning/v0.0.3/v0.0.3-roadmap.md` to match sub-phasing structure
3. Update `docs/planning/v0.0.3/LEARNINGS.md` with Phase 4 insights

**Files Created/Modified**:

- `docs/planning/v0.0.3/README.md` (modified)
- `docs/planning/v0.0.3/v0.0.3-roadmap.md` (modified)
- `docs/planning/v0.0.3/LEARNINGS.md` (modified)

**Validation**: Roadmap documents consistent, Phase 4 marked complete

---

## 🔍 Quality Gates

### Pre-Commit Checks

- [ ] TypeScript compiles without errors: `npm run compile`
- [ ] No TypeScript linting violations: `npm run lint`
- [ ] Extension activates in development host without errors
- [ ] All manual testing checklist items pass
- [ ] No errors in Extension Host output panel

### Documentation Checks

- [ ] README.md updated with completion features
- [ ] CHANGELOG.md has v0.0.3 Phase 4 entry
- [ ] Markdown linting passes: `npm run docs:lint`
- [ ] All links valid: `npx markdown-link-check extensions/vscode/README.md`

### Integration Checks

- [ ] Extension works with existing syntax highlighting
- [ ] Completions don't interfere with existing snippets
- [ ] Extension package size reasonable (<100KB)

---

## 📊 Estimated Effort

**Complexity**: Medium  
**Total Effort**: 6-8 hours

**Breakdown**:

- Phase 4A (Infrastructure): 1-2 hours
- Phase 4B (Keywords): 1 hour
- Phase 4C (Types): 0.5 hours
- Phase 4D (Functions): 0.5 hours
- Phase 4E (Context): 1-2 hours
- Phase 4F (Docs/Testing): 1-2 hours
- Phase 4G (Roadmap): 0.5 hours

**Risk Factors**:

- ⚠️ First time adding TypeScript to extension (learning curve)
- ⚠️ VS Code API unfamiliarity (may need API documentation lookups)
- ✅ Well-defined scope (no LSP complexity)
- ✅ Manual testing sufficient (no automated test infrastructure needed)

---

## 🔗 Dependencies

### Prerequisites

- ✅ v0.0.2 extension structure exists
- ✅ Syntax highlighting working
- ✅ Language configuration in place

### Enables

- Phase 5: Hover tooltips (uses similar provider pattern)
- Phase 5: Problem panel integration (diagnostics provider)
- Future: LSP migration (completion provider → LSP completion)

### No Blockers

- Can proceed immediately (all prerequisites met)
- Does not depend on Phases 1-3 (error system independent)

---

## 📝 Assumptions

⚠️ **ASSUMPTION 1**: Users will install Node.js and npm to build the extension  
**Reasoning**: Standard for VS Code extension development

⚠️ **ASSUMPTION 2**: TypeScript 5.x compatible with VS Code 1.75.0+  
**Reasoning**: VS Code extension API stable, TypeScript backward compatible

⚠️ **ASSUMPTION 3**: Context detection using regex patterns sufficient for v0.0.3  
**Reasoning**: Full parsing deferred to LSP in v0.0.5, regex adequate for basic patterns

⚠️ **ASSUMPTION 4**: Manual testing adequate for Phase 4 validation  
**Reasoning**: Automated extension testing complex, not worth setup for simple completion provider

⚠️ **ASSUMPTION 5**: Completion provider pattern won't need refactoring for LSP  
**Reasoning**: VS Code's CompletionItemProvider can coexist with LSP or be replaced cleanly

---

## 🎯 Success Criteria Summary

**Phase 4 is COMPLETE when**:

1. ✅ TypeScript extension compiles without errors
2. ✅ Extension activates in VS Code without errors
3. ✅ Keyword completion works (let, fn, if, while, return, etc.)
4. ✅ Type completion works in type position (i32, f32, bool, String, Vector2, Node, void)
5. ✅ Function completion works (print with parameter hint)
6. ✅ Context-aware completion shows relevant suggestions
7. ✅ All completion items have helpful documentation
8. ✅ Manual testing checklist 100% pass
9. ✅ Documentation updated (README, CHANGELOG)
10. ✅ v0.0.3 roadmap documents aligned

**Ready for Phase 5**: Hover tooltips and problem panel integration

---

## 📚 References

- [VS Code Extension API - Completion Provider](https://code.visualstudio.com/api/language-extensions/programmatic-language-features#show-code-completion-proposals)
- [VS Code Extension Samples](https://github.com/microsoft/vscode-extension-samples)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)
- FerrisScript v0.0.3 Roadmap: `docs/planning/v0.0.3/v0.0.3-roadmap.md`

---

**Status**: 🟡 Ready to Execute  
**Next Action**: Begin Phase 4A (TypeScript Infrastructure Setup)
