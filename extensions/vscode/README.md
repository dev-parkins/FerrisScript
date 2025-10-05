# FerrisScript for Visual Studio Code

Syntax highlighting and code snippets for FerrisScript - a Rust-inspired scripting language for Godot 4.x.

## Features

### Syntax Highlighting

- **Keywords**: `fn`, `let`, `mut`, `if`, `else`, `while`, `return`, `true`, `false`
- **Types**: `i32`, `f32`, `bool`, `String`, `Vector2`, `Node`, `void`
- **Operators**: Arithmetic (`+`, `-`, `*`, `/`), comparison (`==`, `!=`, `<`, `>`, `<=`, `>=`), logical (`&&`, `||`, `!`), assignment (`=`, `+=`, `-=`)
- **Comments**: Line comments (`//`)
- **Strings**: Double-quoted strings
- **Numbers**: Integer and floating-point literals

### Code Snippets

- `_ready` - Godot _ready() function
- `_process` - Godot _process(delta) function
- `let` - Variable declaration
- `letmut` - Mutable variable declaration
- `fn` - Function definition
- `if` - If statement
- `ifelse` - If-else statement
- `while` - While loop

## Installation

### From Source (Development)

1. Clone the FerrisScript repository
2. Copy the `extensions/vscode` directory to your VS Code extensions folder:
   - **Windows**: `%USERPROFILE%\.vscode\extensions\ferrisscript-0.1.0`
   - **macOS/Linux**: `~/.vscode/extensions/ferrisscript-0.1.0`
3. Reload VS Code

### From Marketplace (Coming Soon)

The extension will be available on the VS Code Marketplace after v0.0.2 release.

## Usage

1. Create a file with `.ferris` extension
2. Start typing FerrisScript code
3. Use snippets by typing the prefix and pressing Tab

### Example

```ferrisscript
// Type '_ready' and press Tab
fn _ready() {
    print("Hello from FerrisScript!");
}

// Type '_process' and press Tab
fn _process(delta: f32) {
    self.position.x += 50.0 * delta;
}
```

## Requirements

- Visual Studio Code 1.75.0 or higher

## Known Limitations

- **No IntelliSense yet**: Completion, hover, and go-to-definition coming in v0.0.5 with LSP
- **No real-time error checking**: Syntax errors shown after compilation only
- **Basic highlighting only**: Semantic highlighting (context-aware colors) coming with LSP

## Roadmap

- **v0.0.2** (Current): Basic syntax highlighting and snippets ✅
- **v0.0.3**: Enhanced diagnostics, error codes
- **v0.0.5**: Language Server Protocol (LSP) with IntelliSense
- **v0.1.0**: Full editor integration with debugging support

## Contributing

See [CONTRIBUTING.md](https://github.com/dev-parkins/FerrisScript/blob/main/CONTRIBUTING.md) for contribution guidelines.

### Grammar Maintenance

When adding new language features, update the syntax highlighting grammar:

1. Edit `syntaxes/ferrisscript.tmLanguage.json`
2. Test on example `.ferris` files
3. Update `CHANGELOG.md`

See [SYNTAX_HIGHLIGHTING_MAINTENANCE.md](https://github.com/dev-parkins/FerrisScript/blob/main/docs/SYNTAX_HIGHLIGHTING_MAINTENANCE.md) for detailed instructions.

## License

MIT License - See [LICENSE](https://github.com/dev-parkins/FerrisScript/blob/main/LICENSE)

## Links

- [FerrisScript GitHub](https://github.com/dev-parkins/FerrisScript)
- [Documentation](https://github.com/dev-parkins/FerrisScript/tree/main/docs)
- [Report Issues](https://github.com/dev-parkins/FerrisScript/issues)
- [FAQ](https://github.com/dev-parkins/FerrisScript/blob/main/docs/FAQ.md)

---

Made with 🦀 and ❤️ for the Godot community
