# FerrisScript for Visual Studio Code

Syntax highlighting and code snippets for FerrisScript - a Rust-inspired scripting language for Godot 4.x.

## Features

### Code Completion ✨ NEW in v0.0.3

- **Keyword Completion**: Auto-complete FerrisScript keywords as you type
  - Control flow: `if`, `else`, `while`, `return`
  - Declarations: `fn`, `let`, `mut`
  - Literals: `true`, `false`
- **Type Completion**: Auto-complete type names in type positions
  - Primitives: `i32`, `f32`, `bool`, `String`
  - Godot types: `Vector2`, `Node`, `void`
- **Function Completion**: Auto-complete built-in functions
  - `print(message: String)` with parameter hints
- **Context-Aware**: Shows relevant completions based on cursor position
  - After `:` → shows type completions
  - At statement start → shows statement-level keywords
  - In expressions → shows all keywords and functions

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
3. Use code completion (Ctrl+Space) or snippets (type prefix and press Tab)

### Example: Code Completion

```ferrisscript
// Type 'let' and press Tab - completion expands to full declaration
let speed: f32 = 100.0;

// Type 'fn' and press Tab - completion creates function template
fn update(delta: f32) {
    // Type 'print' - completion shows function signature
    print("Delta: " + delta);
}

// After ':' in type position - only types are suggested
let position: Vector2 = Vector2(0.0, 0.0);
```

### Example: Snippets

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

- **Basic completion only**: Completion limited to keywords, types, and built-in functions (no symbol resolution yet)
- **No hover tooltips yet**: Type information and documentation on hover coming in Phase 5
- **No real-time error checking**: Syntax errors shown after compilation only (problem panel integration coming in Phase 5)
- **No go-to-definition**: Full IntelliSense features coming in v0.0.5 with LSP
- **Basic highlighting only**: Semantic highlighting (context-aware colors) coming with LSP

## Roadmap

- **v0.0.2**: Basic syntax highlighting and snippets ✅
- **v0.0.3** (Current): Enhanced diagnostics, code completion, hover tooltips ⏳
  - Phase 4: Code completion ✅
  - Phase 5: Hover tooltips and problem panel (in progress)
- **v0.0.5**: Language Server Protocol (LSP) with full IntelliSense
- **v0.1.0**: Full editor integration with debugging support

## Contributing

See [CONTRIBUTING.md](https://github.com/dev-parkins/FerrisScript/blob/main/CONTRIBUTING.md) for contribution guidelines.

### Grammar Maintenance

When adding new language features, update the syntax highlighting grammar:

1. Edit `syntaxes/ferrisscript.tmLanguage.json`
2. Test on example `.ferris` files
3. Update `CHANGELOG.md`

See [SYNTAX_HIGHLIGHTING_MAINTENANCE.md](../../docs/SYNTAX_HIGHLIGHTING_MAINTENANCE.md) for detailed instructions.

## License

MIT License - See [LICENSE](https://github.com/dev-parkins/FerrisScript/blob/main/LICENSE)

## Links

- [FerrisScript GitHub](https://github.com/dev-parkins/FerrisScript)
- [Documentation](https://github.com/dev-parkins/FerrisScript/tree/main/docs)
- [Report Issues](https://github.com/dev-parkins/FerrisScript/issues)
- [FAQ](https://github.com/dev-parkins/FerrisScript/blob/main/docs/FAQ.md)

---

Made with 🦀 and ❤️ for the Godot community
