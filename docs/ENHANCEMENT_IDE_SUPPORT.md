# IDE Support Enhancement Tracking

**Date Created**: October 4, 2025  
**Status**: 📝 Tracked for Future Implementation  
**Priority**: Medium (Quality of Life)

---

## 🎯 Enhancement Request

### Goal

Create VS Code extension and IDE support for `.ferris` files to provide:
- Syntax highlighting
- Code snippets
- Language features (autocomplete, go-to-definition, hover documentation)
- Error diagnostics in real-time

### User Need

While working on Godot bindings, `.ferris` files currently have no syntax highlighting or IDE support, making development less pleasant. This enhancement would significantly improve the developer experience.

---

## 📍 Current Roadmap Status

### v0.0.2 (Current - Patch Release)

**Status**: Foundation mentioned but **NOT scheduled for implementation**

From `v0.0.2-CHECKLIST.md` (lines 242-252):

```markdown
### Editor Support (Foundation)

- [ ] **Create syntax highlighting files**
  - [ ] TextMate grammar for .ferris
  - [ ] VS Code extension skeleton
  - [ ] Submit to VS Code marketplace

- [ ] **Add code snippets**
  - [ ] VS Code snippets file
  - [ ] Common patterns (function, if, while)
  - [ ] Godot-specific snippets (_ready,_process)
```

**Note**: These are listed as potential tasks but not actively prioritized for v0.0.2.

### v0.1.0 (Minor Release - NEW FEATURES)

**Status**: ✅ **PLANNED - High Priority**

From `v0.1.0-ROADMAP.md` (lines 472-495):

```markdown
#### 1. Language Server Protocol (LSP)

**Status**: 🔴 Not Started  
**Priority**: High  
**Rationale**: Makes FerrisScript a real development language

**Scope**:

- [ ] Basic LSP server implementation
- [ ] Syntax checking (real-time errors)
- [ ] Autocomplete (keywords, variables, functions)
- [ ] Go to definition
- [ ] Hover documentation
- [ ] VS Code extension

**Features**:

- Syntax errors as you type
- Autocomplete for variables, functions, types
- Jump to function definition
- Show type on hover

**Tests**: LSP protocol tests
**Estimated Effort**: 10-15 days (major feature)
```

---

## 🗺️ Implementation Path

### Phase 1: Basic Syntax Highlighting (v0.0.2 or v0.1.0-alpha)

**Estimated Effort**: 2-3 days

**Deliverables**:
1. **TextMate Grammar** (`ferrisscript.tmLanguage.json`)
   - Keywords: `fn`, `let`, `mut`, `if`, `else`, `while`, `return`, etc.
   - Types: `i32`, `f32`, `bool`, `String`, `Vector2`, `Node`, etc.
   - Operators: `+`, `-`, `*`, `/`, `==`, `!=`, `&&`, `||`, etc.
   - Comments: `//` single-line
   - Strings: `"..."` with escape sequences

2. **VS Code Extension (Minimal)**
   - `package.json` with language definition
   - Register `.ferris` file extension
   - Associate with TextMate grammar
   - Basic icon/logo

3. **Code Snippets**
   - `fn` - function declaration
   - `fnready` - `fn _ready()` Godot lifecycle
   - `fnprocess` - `fn _process(delta: f32)` Godot lifecycle
   - `let` - let binding
   - `if` - if statement
   - `while` - while loop

**Resources**:
- [VS Code Language Extensions Guide](https://code.visualstudio.com/api/language-extensions/syntax-highlight-guide)
- [TextMate Grammar Guide](https://macromates.com/manual/en/language_grammars)
- Example: [Rust tmLanguage.json](https://github.com/rust-lang/vscode-rust/blob/master/syntaxes/rust.tmLanguage.json)

---

### Phase 2: Language Server Protocol (v0.1.0)

**Estimated Effort**: 10-15 days (per roadmap)

**Architecture**:

```
┌─────────────────┐
│   VS Code       │  User edits .ferris file
└────────┬────────┘
         │ LSP Protocol
         ▼
┌─────────────────┐
│  LSP Server     │  Rust binary (tower-lsp crate)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Compiler API   │  Reuse existing lexer/parser/type_checker
└─────────────────┘
```

**Deliverables**:

1. **LSP Server Implementation** (`crates/lsp/`)
   - Initialize server with `tower-lsp` crate
   - Text synchronization (track document changes)
   - Diagnostics (syntax errors, type errors)
   - Completion provider (keywords, variables, functions)
   - Hover provider (show type information)
   - Go-to-definition (jump to function/variable declaration)

2. **VS Code Extension (Full)**
   - Language client activation
   - Connect to LSP server
   - Configuration settings
   - Commands (restart server, show logs)

3. **Integration with Compiler**
   - Expose incremental compilation API
   - Cache parsed ASTs for performance
   - Return structured diagnostics (line, column, severity, message)

**Dependencies**:
- `tower-lsp` - LSP framework
- `tokio` - Async runtime
- `serde_json` - JSON-RPC communication

**Resources**:
- [tower-lsp Documentation](https://docs.rs/tower-lsp/)
- [LSP Specification](https://microsoft.github.io/language-server-protocol/)
- [VS Code Language Server Extension Guide](https://code.visualstudio.com/api/language-extensions/language-server-extension-guide)

---

### Phase 3: Advanced Features (v0.2.0+)

**Future Enhancements**:
- Semantic highlighting (context-aware colors)
- Refactoring (rename symbol, extract function)
- Code actions (quick fixes, imports)
- Signature help (function parameter hints)
- Document symbols (outline view)
- Workspace symbols (find symbol across files)
- Formatting provider (auto-format code)
- Code lens (inline decorations)

---

## 🛠️ How to Implement

### Step 1: Create TextMate Grammar

**File**: `editors/vscode/syntaxes/ferrisscript.tmLanguage.json`

```json
{
  "name": "FerrisScript",
  "scopeName": "source.ferris",
  "patterns": [
    { "include": "#comments" },
    { "include": "#keywords" },
    { "include": "#types" },
    { "include": "#strings" },
    { "include": "#numbers" },
    { "include": "#operators" }
  ],
  "repository": {
    "comments": {
      "patterns": [
        {
          "name": "comment.line.double-slash.ferris",
          "match": "//.*$"
        }
      ]
    },
    "keywords": {
      "patterns": [
        {
          "name": "keyword.control.ferris",
          "match": "\\b(fn|let|mut|if|else|while|return|true|false|self)\\b"
        }
      ]
    },
    "types": {
      "patterns": [
        {
          "name": "storage.type.ferris",
          "match": "\\b(i32|f32|bool|String|Vector2|Node)\\b"
        }
      ]
    }
  }
}
```

### Step 2: Create VS Code Extension

**File**: `editors/vscode/package.json`

```json
{
  "name": "ferrisscript",
  "displayName": "FerrisScript",
  "description": "Language support for FerrisScript (.ferris)",
  "version": "0.1.0",
  "publisher": "ferrisscript",
  "engines": {
    "vscode": "^1.80.0"
  },
  "categories": ["Programming Languages"],
  "contributes": {
    "languages": [{
      "id": "ferrisscript",
      "aliases": ["FerrisScript", "ferris"],
      "extensions": [".ferris"],
      "configuration": "./language-configuration.json"
    }],
    "grammars": [{
      "language": "ferrisscript",
      "scopeName": "source.ferris",
      "path": "./syntaxes/ferrisscript.tmLanguage.json"
    }],
    "snippets": [{
      "language": "ferrisscript",
      "path": "./snippets/ferrisscript.json"
    }]
  }
}
```

### Step 3: Implement LSP Server (v0.1.0)

**File**: `crates/lsp/src/main.rs`

```rust
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

struct FerrisScriptLsp {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for FerrisScriptLsp {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions::default()),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = &params.content_changes[0].text;
        
        // Compile and get diagnostics
        match ferrisscript_compiler::compile(text) {
            Ok(_) => {
                // Clear diagnostics on success
                self.client.publish_diagnostics(uri, vec![], None).await;
            }
            Err(error) => {
                // Parse error message to extract line/column
                let diagnostic = Diagnostic {
                    range: Range::default(), // Parse from error
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: error,
                    ..Default::default()
                };
                self.client.publish_diagnostics(uri, vec![diagnostic], None).await;
            }
        }
    }
}
```

---

## 📊 Milestones

| Milestone | Version | Est. Timeline | Deliverables |
|-----------|---------|---------------|--------------|
| **Foundation** | v0.0.2 | Optional | TextMate grammar, basic VS Code extension |
| **LSP Alpha** | v0.1.0-alpha | 3-4 weeks into v0.1.0 | LSP server, diagnostics, basic completion |
| **LSP Beta** | v0.1.0-beta | After Milestone 4 | Full LSP features, VS Code marketplace |
| **Advanced** | v0.2.0+ | 6-12 months | Refactoring, semantic highlighting, formatting |

---

## 🔗 Related Roadmap Sections

**v0.1.0-ROADMAP.md**:
- Lines 30: "Add tooling (syntax highlighting, snippets)"
- Lines 472-495: LSP implementation details
- Lines 638: "LSP makes coding pleasant"
- Lines 680-683: LSP foundation timeline
- Lines 725-730: Milestone 4 - LSP Alpha
- Lines 824: LSP user guide documentation

**v0.0.2-CHECKLIST.md**:
- Lines 242-252: Editor Support (Foundation)

---

## 📝 Implementation Notes

### Godot Integration Context

- `.ferris` files are **asset files** in Godot projects (like textures, sounds)
- Not native Godot scripts (no "Create New Script" integration)
- Loaded by `FerrisScriptNode` via GDExtension
- Compiled by `ferrisscript_compiler::compile()` at runtime

### Why LSP is Important

1. **Developer Experience**: Syntax highlighting and autocomplete make coding pleasant
2. **Error Prevention**: Real-time diagnostics catch errors before Godot runtime
3. **Learning Curve**: Hover documentation helps new users understand types/functions
4. **Professional Feel**: Makes FerrisScript feel like a "real" language

### Alternatives to LSP

If LSP is too complex initially, consider:
1. **Basic TextMate grammar only** (1-2 days effort)
2. **Static analysis in VS Code extension** (no separate server)
3. **Diagnostic provider using compiler API** (5-7 days)

---

## 🚀 Next Steps

### Immediate (v0.0.2)

1. ⏸️ **DEFER** - Focus on bug fixes and documentation first
2. Track this enhancement for v0.1.0

### Short-Term (v0.1.0-alpha)

1. Create `editors/vscode/` directory structure
2. Implement TextMate grammar
3. Create minimal VS Code extension
4. Test syntax highlighting with existing `.ferris` examples

### Medium-Term (v0.1.0)

1. Create `crates/lsp/` package
2. Implement LSP server with `tower-lsp`
3. Integrate with existing compiler API
4. Add diagnostics, completion, hover
5. Publish to VS Code marketplace

### Long-Term (v0.2.0+)

1. Advanced LSP features (refactoring, formatting)
2. Support other IDEs (IntelliJ, Sublime, Neovim)
3. Debugger integration (DAP)
4. Profiler integration

---

## 📚 Resources

**Learning Resources**:
- [Writing a Language Server in Rust](https://freemasen.com/blog/writing-a-language-server-in-rust/)
- [VS Code Extension Samples](https://github.com/microsoft/vscode-extension-samples)
- [tower-lsp Examples](https://github.com/ebkalderon/tower-lsp/tree/master/examples)

**Reference Implementations**:
- [rust-analyzer](https://github.com/rust-lang/rust-analyzer) - Rust LSP (complex, feature-complete)
- [ruff-lsp](https://github.com/astral-sh/ruff-lsp) - Python linter LSP (simpler example)
- [Lua Language Server](https://github.com/LuaLS/lua-language-server) - Game scripting LSP

---

**Last Updated**: October 4, 2025  
**Tracked By**: @dev-parkins  
**Status**: 📝 Enhancement tracked, implementation deferred to v0.1.0
