# Phase 5: VS Code Hover & Problem Panel - Execution Plan

**Date**: October 7, 2025  
**Agent**: GitHub Copilot  
**Status**: In Progress  
**Branch**: `feature/v0.0.3-phase-5-hover`  
**PR**: *(To be created)*

---

## 🎯 Context

### Workstream Goal

Add hover tooltips and problem panel integration to the FerrisScript VS Code extension, providing IDE-like experience without full LSP implementation.

### Strategic Context

- **Version**: v0.0.3 (Editor Experience Alpha)
- **Phase**: 5 of 9
- **Dependencies**: Phase 4 (completion infrastructure) ✅
- **Enables**: Phase 6 (development scripts), prepares for Phase 9 (LSP foundation)
- **Type**: Feature addition to existing VS Code extension

### Prior Work

- ✅ Phase 4: Code completion provider with context detection
- ✅ TypeScript extension infrastructure established
- ✅ Context detection utilities (`src/utils/context.ts`)
- ✅ Keyword/type/function data structures in place
- ❌ No hover functionality yet
- ❌ No problem panel integration yet

### Constraints

- **No LSP yet**: Simple hover provider, not full language server (deferred to v0.0.5)
- **No compiler integration**: Static hover content (no type inference from code)
- **Limited diagnostics**: Manual error mapping, not real-time compilation
- **Keep lightweight**: Minimal dependencies, use VS Code built-in APIs only

---

## 📋 Requirements Summary

### Functional Requirements

1. **Hover Tooltips**: Show helpful information when hovering over code
   - **Keywords**: Show description, syntax, and example for each keyword (let, fn, if, etc.)
   - **Types**: Show type information and example usage (i32, Vector2, etc.)
   - **Functions**: Show function signature and parameter descriptions (print)
   - **Format**: Markdown with syntax highlighting in examples

2. **Problem Panel Integration**: Display compiler errors and warnings
   - **Error Collection**: Create diagnostic collection for FerrisScript errors
   - **Error Parsing**: Parse compiler output and map to source locations
   - **Inline Display**: Show errors as red squiggles in editor
   - **Problem Panel**: Populate VS Code's "Problems" panel with issues
   - **Quick Fixes**: Suggest fixes for common errors (e.g., typos with "Did you mean?")

3. **File Icons**: Professional appearance for `.ferris` files
   - Custom file icon in file explorer
   - Distinct from generic text file icon

4. **Marketplace Polish**: Professional extension presentation
   - Improved description with feature highlights
   - Screenshots showing hover and problem panel
   - Clear installation and usage instructions
   - Updated README with Phase 5 features

### Non-Functional Requirements

- **Performance**: Hover tooltips appear within 100ms
- **Accuracy**: Hover content is helpful and accurate
- **UX**: Hover styling follows VS Code conventions (Markdown format)
- **Maintainability**: Hover content defined in structured data (easy to extend)

### Out of Scope (Deferred to v0.0.5 LSP)

- ❌ Type inference from code (requires type checker integration)
- ❌ Go-to-definition
- ❌ Find all references
- ❌ Real-time compilation on every keystroke
- ❌ Advanced quick fixes (auto-import, refactoring)

---

## 🏗️ Technical Architecture

### Extension Structure Updates

```
extensions/vscode/
├── src/
│   ├── extension.ts              # Updated: register hover & diagnostics
│   ├── hover/
│   │   ├── provider.ts           # NEW: HoverProvider implementation
│   │   ├── keywords.ts           # NEW: Keyword hover content
│   │   ├── types.ts              # NEW: Type hover content
│   │   └── functions.ts          # NEW: Function hover content
│   ├── diagnostics/
│   │   ├── provider.ts           # NEW: DiagnosticProvider
│   │   └── parser.ts             # NEW: Parse compiler error output
│   ├── completion/               # Existing from Phase 4
│   └── utils/                    # Existing from Phase 4
├── resources/
│   └── icons/
│       └── ferrisscript.svg      # NEW: File icon
├── package.json                  # Updated: icon theme, activation events
└── README.md                     # Updated: document Phase 5 features
```

### Key Components

#### 1. Hover Provider

**File**: `src/hover/provider.ts`

```typescript
import * as vscode from 'vscode';
import { getKeywordHover } from './keywords';
import { getTypeHover } from './types';
import { getFunctionHover } from './functions';

export class FerrisScriptHoverProvider implements vscode.HoverProvider {
    provideHover(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken
    ): vscode.ProviderResult<vscode.Hover> {
        // Get word at position
        const wordRange = document.getWordRangeAtPosition(position);
        if (!wordRange) {
            return undefined;
        }

        const word = document.getText(wordRange);

        // Try to find hover content
        let hoverContent: vscode.MarkdownString | undefined;

        // Check keywords first
        hoverContent = getKeywordHover(word);
        if (hoverContent) {
            return new vscode.Hover(hoverContent, wordRange);
        }

        // Check types
        hoverContent = getTypeHover(word);
        if (hoverContent) {
            return new vscode.Hover(hoverContent, wordRange);
        }

        // Check functions
        hoverContent = getFunctionHover(word);
        if (hoverContent) {
            return new vscode.Hover(hoverContent, wordRange);
        }

        return undefined;
    }
}
```

#### 2. Keyword Hover Content

**File**: `src/hover/keywords.ts`

```typescript
import * as vscode from 'vscode';

interface KeywordHoverData {
    keyword: string;
    description: string;
    syntax: string;
    example: string;
}

const KEYWORD_HOVERS: KeywordHoverData[] = [
    {
        keyword: 'let',
        description: 'Declares an immutable variable',
        syntax: 'let name: type = value;',
        example: 'let speed: f32 = 100.0;'
    },
    {
        keyword: 'fn',
        description: 'Declares a function',
        syntax: 'fn name(param: type) -> return_type { ... }',
        example: 'fn update(delta: f32) -> void {\n    // function body\n}'
    },
    // ... more keywords
];

export function getKeywordHover(word: string): vscode.MarkdownString | undefined {
    const data = KEYWORD_HOVERS.find(kw => kw.keyword === word);
    if (!data) {
        return undefined;
    }

    const md = new vscode.MarkdownString();
    md.appendMarkdown(`**${data.keyword}** - ${data.description}\n\n`);
    md.appendMarkdown(`**Syntax**: \`${data.syntax}\`\n\n`);
    md.appendMarkdown(`**Example**:\n`);
    md.appendCodeblock(data.example, 'ferrisscript');
    md.isTrusted = true;

    return md;
}
```

#### 3. Diagnostic Provider

**File**: `src/diagnostics/provider.ts`

```typescript
import * as vscode from 'vscode';
import * as cp from 'child_process';
import { parseCompilerErrors } from './parser';

export class FerrisScriptDiagnosticProvider {
    private diagnosticCollection: vscode.DiagnosticCollection;

    constructor() {
        this.diagnosticCollection = vscode.languages.createDiagnosticCollection('ferrisscript');
    }

    public updateDiagnostics(document: vscode.TextDocument): void {
        if (document.languageId !== 'ferrisscript') {
            return;
        }

        // Clear existing diagnostics
        this.diagnosticCollection.clear();

        // Run compiler and parse errors
        // Note: This is simplified - real implementation would need compiler path config
        const errors = this.runCompiler(document.uri.fsPath);
        const diagnostics = parseCompilerErrors(errors, document);

        this.diagnosticCollection.set(document.uri, diagnostics);
    }

    private runCompiler(filePath: string): string {
        try {
            // Execute ferrisscript compiler
            const result = cp.execSync(`ferrisscript "${filePath}"`, {
                encoding: 'utf-8',
                timeout: 5000
            });
            return result;
        } catch (error: any) {
            // Compiler errors are returned in stderr
            return error.stderr || error.stdout || '';
        }
    }

    public dispose(): void {
        this.diagnosticCollection.dispose();
    }
}
```

#### 4. Error Parser

**File**: `src/diagnostics/parser.ts`

```typescript
import * as vscode from 'vscode';

export function parseCompilerErrors(
    output: string,
    document: vscode.TextDocument
): vscode.Diagnostic[] {
    const diagnostics: vscode.Diagnostic[] = [];
    
    // Parse FerrisScript error format:
    // Error[E201]: Undefined variable 'velocty'
    //   --> move.ferris:5:10
    
    const errorRegex = /Error\[(\w+)\]: (.*?)\n\s*--> .*?:(\d+):(\d+)/g;
    let match;

    while ((match = errorRegex.exec(output)) !== null) {
        const [, code, message, line, column] = match;
        
        const lineNum = parseInt(line) - 1; // VS Code is 0-indexed
        const colNum = parseInt(column) - 1;
        
        const range = new vscode.Range(
            lineNum,
            colNum,
            lineNum,
            colNum + 10 // Approximate error length
        );

        const diagnostic = new vscode.Diagnostic(
            range,
            `${code}: ${message}`,
            vscode.DiagnosticSeverity.Error
        );
        
        diagnostic.code = code;
        diagnostic.source = 'ferrisscript';
        
        diagnostics.push(diagnostic);
    }

    return diagnostics;
}
```

#### 5. Extension Activation Updates

**File**: `src/extension.ts`

```typescript
import * as vscode from 'vscode';
import { FerrisScriptCompletionProvider } from './completion/provider';
import { FerrisScriptHoverProvider } from './hover/provider';
import { FerrisScriptDiagnosticProvider } from './diagnostics/provider';

let diagnosticProvider: FerrisScriptDiagnosticProvider;

export function activate(context: vscode.ExtensionContext) {
    // Register completion provider (existing from Phase 4)
    const completionProvider = new FerrisScriptCompletionProvider();
    context.subscriptions.push(
        vscode.languages.registerCompletionItemProvider(
            'ferrisscript',
            completionProvider,
            ':', ' ', '.'
        )
    );

    // Register hover provider (NEW)
    const hoverProvider = new FerrisScriptHoverProvider();
    context.subscriptions.push(
        vscode.languages.registerHoverProvider('ferrisscript', hoverProvider)
    );

    // Register diagnostic provider (NEW)
    diagnosticProvider = new FerrisScriptDiagnosticProvider();
    context.subscriptions.push(diagnosticProvider);

    // Update diagnostics on file save
    context.subscriptions.push(
        vscode.workspace.onDidSaveTextDocument((document) => {
            diagnosticProvider.updateDiagnostics(document);
        })
    );

    // Update diagnostics on file open
    context.subscriptions.push(
        vscode.workspace.onDidOpenTextDocument((document) => {
            diagnosticProvider.updateDiagnostics(document);
        })
    );
}

export function deactivate() {
    if (diagnosticProvider) {
        diagnosticProvider.dispose();
    }
}
```

### File Icon Configuration

**Update**: `package.json`

```json
{
  "contributes": {
    "iconThemes": [
      {
        "id": "ferrisscript-icons",
        "label": "FerrisScript Icons",
        "path": "./resources/icons/ferrisscript-icon-theme.json"
      }
    ]
  }
}
```

**Create**: `resources/icons/ferrisscript-icon-theme.json`

```json
{
  "iconDefinitions": {
    "ferrisscript-file": {
      "iconPath": "./ferrisscript.svg"
    }
  },
  "fileExtensions": {
    "ferris": "ferrisscript-file"
  }
}
```

---

## ✅ Acceptance Criteria

### Criterion 1: Keyword Hover Works

**Given**: User opens a `.ferris` file with keywords  
**When**: User hovers over `let` keyword  
**Then**: Tooltip shows description, syntax, and example for `let`

**Evidence**: Manual testing with all keywords

---

### Criterion 2: Type Hover Shows Info

**Given**: User opens a `.ferris` file with type annotations  
**When**: User hovers over `Vector2` type  
**Then**: Tooltip shows type description and example usage

**Evidence**: Manual testing with all types

---

### Criterion 3: Function Hover Shows Signature

**Given**: User opens a `.ferris` file with function call  
**When**: User hovers over `print` function  
**Then**: Tooltip shows function signature: `print(message: String) -> void`

**Evidence**: Manual testing with built-in functions

---

### Criterion 4: Problem Panel Shows Errors

**Given**: User opens a `.ferris` file with syntax error  
**When**: User saves the file  
**Then**: Error appears in VS Code Problems panel with correct line/column

**Evidence**: Manual testing with intentional errors

---

### Criterion 5: Inline Error Squiggles

**Given**: User opens a `.ferris` file with undefined variable  
**When**: User saves the file  
**Then**: Red squiggle appears under undefined variable name

**Evidence**: Manual testing with type errors

---

### Criterion 6: File Icon Displays

**Given**: User has `.ferris` files in workspace  
**When**: User views file explorer  
**Then**: `.ferris` files show custom icon (not generic text icon)

**Evidence**: Visual inspection in VS Code

---

### Criterion 7: Hover Format is Professional

**Given**: Any hover tooltip  
**When**: User views hover content  
**Then**: Content uses proper Markdown, syntax highlighting, and VS Code conventions

**Evidence**: Visual inspection of all hover types

---

### Criterion 8: Diagnostics Clear on Fix

**Given**: File has error, problem panel shows it  
**When**: User fixes error and saves  
**Then**: Error disappears from problem panel and inline squiggle removed

**Evidence**: Manual testing of error fixing workflow

---

### Criterion 9: Extension Compiles and Loads

**Given**: TypeScript source code complete  
**When**: Run `npm run compile`  
**Then**: Compiles without errors, extension loads in VS Code

**Evidence**: Build validation and extension activation

---

### Criterion 10: Documentation is Updated

**Given**: Phase 5 implementation complete  
**When**: User reads extension README  
**Then**: README documents hover and problem panel features with examples

**Evidence**: Documentation review

---

## 🧪 Test Strategy

### Manual Testing Checklist

See [PHASE_5_MANUAL_TESTING.md](./PHASE_5_MANUAL_TESTING.md) for comprehensive testing guide.

**High-Level Test Areas**:

- [ ] **Keyword Hover**: Test all 9 keywords (let, fn, if, else, while, return, mut, true, false)
- [ ] **Type Hover**: Test all 7 types (i32, f32, bool, String, Vector2, Node, void)
- [ ] **Function Hover**: Test built-in functions (print)
- [ ] **Problem Panel**: Test lexical, syntax, type, and semantic errors
- [ ] **Error Recovery**: Test fixing errors and diagnostics clearing
- [ ] **File Icons**: Visual inspection of file explorer
- [ ] **Performance**: Hover response time < 100ms
- [ ] **Edge Cases**: Hover on non-keyword, hover on whitespace, hover on comment

### Automated Testing

⚠️ **Limitation**: VS Code extension testing requires complex setup. For Phase 5, we'll rely on:

- Manual testing (comprehensive checklist)
- TypeScript compilation as validation
- Extension loads without errors

**Future**: Add VS Code extension test suite in v0.0.5 when LSP work begins.

---

## 📦 Implementation Phases

### Phase 5A: Hover Provider Infrastructure ✅

**Tasks**:

1. Create `src/hover/provider.ts` with HoverProvider class
2. Register hover provider in `extension.ts`
3. Implement word detection at cursor position
4. Wire hover provider to language registration
5. Test hover activation (even with empty content)

**Files Created/Modified**:

- `extensions/vscode/src/hover/provider.ts` (new)
- `extensions/vscode/src/extension.ts` (modified)

**Validation**: Extension compiles, hover provider registered

---

### Phase 5B: Keyword & Type Hover Content

**Tasks**:

1. Create `src/hover/keywords.ts` with keyword hover data
2. Create `src/hover/types.ts` with type hover data
3. Implement Markdown formatting for hover content
4. Add syntax-highlighted code examples
5. Test hover content for keywords and types

**Files Created/Modified**:

- `extensions/vscode/src/hover/keywords.ts` (new)
- `extensions/vscode/src/hover/types.ts` (new)
- `extensions/vscode/src/hover/provider.ts` (modified)

**Validation**: Hover shows helpful content for keywords and types

---

### Phase 5C: Function Hover Content

**Tasks**:

1. Create `src/hover/functions.ts` with function hover data
2. Add parameter descriptions and return type info
3. Wire function hover into provider
4. Test hover content for functions

**Files Created/Modified**:

- `extensions/vscode/src/hover/functions.ts` (new)
- `extensions/vscode/src/hover/provider.ts` (modified)

**Validation**: Hover shows function signatures correctly

---

### Phase 5D: Diagnostic Provider Infrastructure

**Tasks**:

1. Create `src/diagnostics/provider.ts` with DiagnosticCollection
2. Register diagnostic provider in `extension.ts`
3. Wire up document save event handlers
4. Test diagnostic collection creation

**Files Created/Modified**:

- `extensions/vscode/src/diagnostics/provider.ts` (new)
- `extensions/vscode/src/extension.ts` (modified)

**Validation**: Diagnostic collection created, events wired

---

### Phase 5E: Error Parser & Compiler Integration

**Tasks**:

1. Create `src/diagnostics/parser.ts` with error parsing logic
2. Implement regex to parse FerrisScript error format
3. Map errors to VS Code Diagnostic objects
4. Test with sample compiler output
5. Handle edge cases (no errors, multiple errors)

**Files Created/Modified**:

- `extensions/vscode/src/diagnostics/parser.ts` (new)
- `extensions/vscode/src/diagnostics/provider.ts` (modified)

**Validation**: Errors parse correctly and appear in problem panel

---

### Phase 5F: File Icons

**Tasks**:

1. Create `resources/icons/ferrisscript.svg` file icon
2. Create `resources/icons/ferrisscript-icon-theme.json`
3. Update `package.json` with icon theme contribution
4. Test file icon display in file explorer

**Files Created/Modified**:

- `extensions/vscode/resources/icons/ferrisscript.svg` (new)
- `extensions/vscode/resources/icons/ferrisscript-icon-theme.json` (new)
- `extensions/vscode/package.json` (modified)

**Validation**: `.ferris` files show custom icon

---

### Phase 5G: Marketplace Polish

**Tasks**:

1. Update `README.md` with Phase 5 features
2. Update `CHANGELOG.md` with v0.0.3 Phase 5 changes
3. Create screenshots of hover and problem panel
4. Improve extension description in `package.json`
5. Add usage examples to README

**Files Created/Modified**:

- `extensions/vscode/README.md` (modified)
- `extensions/vscode/CHANGELOG.md` (modified)
- `extensions/vscode/package.json` (modified)
- `extensions/vscode/screenshots/` (new folder with images)

**Validation**: Extension marketplace page looks professional

---

### Phase 5H: Testing & Validation

**Tasks**:

1. Create `PHASE_5_MANUAL_TESTING.md` with test cases
2. Run all manual tests
3. Document test results
4. Fix any issues found
5. Verify all acceptance criteria met

**Files Created/Modified**:

- `docs/planning/v0.0.3/PHASE_5_MANUAL_TESTING.md` (new)
- Various bug fixes as needed

**Validation**: All tests pass, acceptance criteria met

---

### Phase 5I: Documentation Updates

**Tasks**:

1. Update `docs/planning/v0.0.3/README.md` Phase 5 status to complete
2. Update `docs/planning/v0.0.3/v0.0.3-roadmap.md` with Phase 5 completion
3. Update `docs/planning/v0.0.3/LEARNINGS.md` with Phase 5 insights
4. Create `PHASE_5_LESSONS_LEARNED.md` if needed
5. Verify all documentation links and linting

**Files Created/Modified**:

- `docs/planning/v0.0.3/README.md` (modified)
- `docs/planning/v0.0.3/v0.0.3-roadmap.md` (modified)
- `docs/planning/v0.0.3/LEARNINGS.md` (modified)
- `docs/planning/v0.0.3/PHASE_5_LESSONS_LEARNED.md` (new, if applicable)

**Validation**: All documentation accurate, links work, linting passes

---

## 🎯 Success Metrics

### Quantitative Goals

- [ ] Hover works for 100% of keywords (9/9)
- [ ] Hover works for 100% of types (7/7)
- [ ] Hover works for 100% of built-in functions (1/1 - print)
- [ ] Hover response time < 100ms
- [ ] Problem panel shows all compiler errors correctly
- [ ] 10+ manual test cases created and passing

### Qualitative Goals

- [ ] Hover tooltips are helpful and informative
- [ ] Problem panel integration feels native to VS Code
- [ ] File icons look professional
- [ ] Extension README clearly explains features
- [ ] Overall editor experience feels polished

---

## 🔄 Dependencies

### Upstream Dependencies

- ✅ Phase 4: Completion infrastructure
- ✅ TypeScript extension setup
- ✅ Context detection utilities

### Downstream Enables

- ✅ Prepares for Phase 9: LSP implementation
- ✅ Improves developer experience for Phase 6-8
- ✅ Marketplace readiness for public release

---

## 📝 Notes

### Applying Phase 4 Learnings

Based on [PHASE_4_LESSONS_LEARNED.md](./PHASE_4_LESSONS_LEARNED.md):

1. **Create testing documentation upfront**: PHASE_5_MANUAL_TESTING.md before implementation
2. **Use context detection testing matrix**: Apply template from CONTEXT_DETECTION_TESTING.md
3. **Document VS Code behavior**: Follow pattern from PREFIX_FILTERING_BEHAVIOR.md for any non-obvious behavior
4. **Structured testing approach**: Use checklist format with clear expected results
5. **Document issues before fixing**: If bugs found, create analysis document first

### Simplified Approach (No LSP)

Phase 5 uses lightweight VS Code APIs without full LSP:

- **HoverProvider**: Simple interface, no protocol overhead
- **DiagnosticCollection**: VS Code built-in API for problem panel
- **No Language Server**: Compiler called on save, not real-time
- **Static Content**: Hover content is pre-defined, not inferred from code

This approach is intentional for v0.0.3. Full LSP comes in v0.0.5 with:

- Real-time compilation and diagnostics
- Type inference for hover content
- Go-to-definition and find references
- Advanced quick fixes and refactoring

---

## 🚀 Ready to Execute

This execution plan is ready for implementation. Follow phases 5A-5I systematically, updating this document with completion status as work progresses.

**Next Step**: Create feature branch and begin Phase 5A (Hover Provider Infrastructure).
