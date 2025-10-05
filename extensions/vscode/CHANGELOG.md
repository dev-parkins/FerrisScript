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

### Planned for v0.2.0

- Language Server Protocol (LSP) integration
- IntelliSense (completion, hover, go-to-definition)
- Real-time error checking
- Semantic highlighting
- Code actions and quick fixes

---

[0.1.0]: https://github.com/dev-parkins/FerrisScript/releases/tag/v0.0.2
[Unreleased]: https://github.com/dev-parkins/FerrisScript/compare/v0.0.2...HEAD
