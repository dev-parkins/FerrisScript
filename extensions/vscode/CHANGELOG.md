# Change Log

All notable changes to the FerrisScript VS Code extension will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-10-05

### Added

- Initial release of FerrisScript syntax highlighting
- TextMate grammar for `.ferris` files
- Syntax highlighting for keywords: `fn`, `let`, `mut`, `if`, `else`, `while`, `return`, `true`, `false`
- Syntax highlighting for types: `i32`, `f32`, `bool`, `String`, `Vector2`, `Node`, `void`
- Syntax highlighting for operators: `+`, `-`, `*`, `/`, `=`, `==`, `!=`, `<`, `>`, `<=`, `>=`, `&&`, `||`, `!`, `+=`, `-=`
- Syntax highlighting for comments (line comments `//`)
- Syntax highlighting for strings (double-quoted)
- Syntax highlighting for numbers (integers and floats)
- Code snippet: `_ready` - Godot _ready() function
- Code snippet: `_process` - Godot _process(delta) function
- Code snippet: `let` - Variable declaration
- Code snippet: `letmut` - Mutable variable declaration
- Code snippet: `fn` - Function definition
- Code snippet: `if` - If statement
- Code snippet: `ifelse` - If-else statement
- Code snippet: `while` - While loop
- Language configuration for auto-closing brackets and comment toggling

### Known Limitations

- No IntelliSense (completion, hover, go-to-definition) - coming in v0.0.5 with LSP
- No real-time error checking - errors shown after compilation only
- Basic TextMate highlighting only - semantic highlighting coming with LSP

## [Unreleased]

### Added - v0.0.3 Phase 4

- **Code Completion**: Context-aware auto-completion for FerrisScript
  - Keyword completion: `fn`, `let`, `mut`, `if`, `else`, `while`, `return`, `true`, `false`
  - Type completion: `i32`, `f32`, `bool`, `String`, `Vector2`, `Node`, `void`
  - Function completion: `print` with parameter hints
  - Smart context detection:
    - Shows only types after `:` in type annotations
    - Shows statement-level keywords at line start
    - Shows all keywords and functions in expression context
- TypeScript-based extension infrastructure
- Completion provider with VS Code CompletionItemProvider API
- Detailed documentation and examples for each completion item

### In Progress - v0.0.3 Phase 5

- Hover tooltips for type information and documentation
- Problem panel integration for compiler errors
- Quick fixes for common issues

### Planned for v0.0.5

- Language Server Protocol (LSP) integration
- Full IntelliSense (symbol resolution, go-to-definition, find references)
- Real-time error checking with LSP diagnostics
- Semantic highlighting
- Code actions and refactoring
