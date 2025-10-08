# FerrisScript for Visual Studio Code

Full IDE support for FerrisScript - a Rust-inspired scripting language for Godot 4.x. Features code completion, hover tooltips, error diagnostics, and syntax highlighting.

## Features

### Hover Tooltips ✨ NEW in v0.0.3 Phase 5

- **Keyword Documentation**: Hover over keywords to see descriptions, syntax, and examples
  - `let`, `fn`, `if`, `else`, `while`, `return`, `mut`, `true`, `false`
- **Type Information**: Hover over types to see descriptions and usage
  - Primitives: `i32`, `f32`, `bool`, `String`
  - Godot types: `Vector2`, `Node`, `void`
- **Function Signatures**: Hover over functions to see parameters and return types
  - `print(message: String) -> void`
- **Formatted with Examples**: All hover content includes syntax-highlighted code examples

### Error Diagnostics ✨ NEW in v0.0.3 Phase 5

⚠️ **Note**: Diagnostic features require a standalone FerrisScript CLI executable (planned for future release). The infrastructure is in place and ready to use once the CLI is available.

- **Real-time Error Detection**: Compiler errors shown inline as you save (when CLI available)
- **Problem Panel Integration**: All errors and warnings appear in VS Code's Problems panel (when CLI available)
- **Inline Squiggles**: Red underlines highlight error locations (when CLI available)
- **Error Codes**: Each error includes FerrisScript error code (E001-E499)
- **Quick Access**: Click on error in Problems panel to jump to location

### Code Completion (Phase 4)

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
   - **Windows**: `%USERPROFILE%\.vscode\extensions\ferrisscript-0.0.3`
   - **macOS/Linux**: `~/.vscode/extensions/ferrisscript-0.0.3`
3. Reload VS Code

### From VSIX Package (Recommended)

1. Build the VSIX package:

   ```bash
   cd extensions/vscode
   npm install -g @vscode/vsce
   vsce package
   ```

2. Install the generated `.vsix` file:
   - Open VS Code
   - Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on macOS)
   - Type "Install from VSIX" and select the command
   - Choose the `ferrisscript-0.0.3.vsix` file
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

- **Static completion only**: Completion limited to keywords, types, and built-in functions (no symbol resolution from code yet)
- **Static hover content**: Hover shows pre-defined documentation (no type inference from code yet)
- **Compiler must be available**: Error diagnostics require FerrisScript compiler in PATH or workspace
- **Save-triggered diagnostics**: Errors update on file save (not real-time as you type)
- **No go-to-definition**: Full IntelliSense features coming in v0.0.5 with LSP
- **Basic highlighting only**: Semantic highlighting (context-aware colors) coming with LSP

## Roadmap

- **v0.0.2**: Basic syntax highlighting and snippets ✅
- **v0.0.3** (Current): Enhanced diagnostics, code completion, hover tooltips ✅
  - Phase 4: Code completion ✅
  - Phase 5: Hover tooltips and problem panel ✅
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

### Type Completion Maintenance

When adding new types to the FerrisScript language, update the VS Code completion provider:

1. Edit `src/completion/types.ts` - add type to `TYPES` array
2. Edit `syntaxes/ferrisscript.tmLanguage.json` - add to type highlighting
3. Rebuild extension: `npm run compile`
4. Test completion in VS Code

**Important**: VS Code completion types must stay synchronized with compiler types. See [VSCODE_TYPE_SYNCHRONIZATION.md](../../docs/planning/v0.0.3/VSCODE_TYPE_SYNCHRONIZATION.md) for:

- Current synchronization requirements
- Manual update process
- Future automation recommendations (validation scripts, type generation, LSP integration)

### Extension Architecture Notes

**Activation**: The extension auto-activates when a `.ferris` file is opened. VS Code generates activation events from the `contributes.languages` section in `package.json`, so no explicit `activationEvents` array is needed (as of VS Code 1.75+).

**Build Process**: TypeScript code in `src/` compiles to JavaScript in `out/`. The extension entry point is `out/extension.js`.

### Best Practices (As-of October 2025, VS Code 1.75+)

**Note**: VS Code extension development practices evolve with new releases. The recommendations below are current as of **VS Code 1.75.0** (October 2025). Always check the [VS Code Extension API documentation](https://code.visualstudio.com/api) for the latest best practices.

**Current Recommendations**:

- **No Explicit `activationEvents`**: VS Code 1.75+ auto-generates activation events from `contributes` declarations. Explicit `activationEvents` arrays are redundant and trigger deprecation warnings.

- **Use `@vscode/vsce` for Packaging**: The new scoped package `@vscode/vsce` replaces the legacy `vsce` package for extension packaging and publishing.

- **TypeScript 5.x+**: Use TypeScript 5.0 or later for modern language features and improved type checking. Target ES2020 or later for broad compatibility.

- **ESLint with TypeScript**: Use `@typescript-eslint/eslint-plugin` and `@typescript-eslint/parser` for consistent code quality.

- **Extension Testing**: Use `@vscode/test-electron` for automated extension testing (as of 1.70+). The legacy `vscode-test` package is deprecated.

**Version-Specific Notes**:

- **VS Code 1.75+**: Auto-activation from `contributes` (removes need for explicit `activationEvents`)
- **VS Code 1.70+**: New testing framework (`@vscode/test-electron`)
- **VS Code 1.60+**: Native support for TypeScript 4.4+ decorators

**Future Changes**: Monitor VS Code release notes for deprecations and new APIs. Major version updates (e.g., 1.75 → 1.80) may introduce breaking changes or new best practices.

## License

MIT License - See [LICENSE](https://github.com/dev-parkins/FerrisScript/blob/main/LICENSE)

## Links

- [FerrisScript GitHub](https://github.com/dev-parkins/FerrisScript)
- [Documentation](https://github.com/dev-parkins/FerrisScript/tree/main/docs)
- [Report Issues](https://github.com/dev-parkins/FerrisScript/issues)
- [FAQ](https://github.com/dev-parkins/FerrisScript/blob/main/docs/FAQ.md)

---

Made with 🦀 and ❤️ for the Godot community
