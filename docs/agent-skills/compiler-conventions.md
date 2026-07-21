# Compiler Conventions

**Load this skill when working in `crates/compiler/`**

## Architecture Overview

```
src/
├── lib.rs           # Public API: compile() function
├── lexer.rs         # Tokenization (Token enum, Lexer struct)
├── parser.rs        # Recursive descent parser → AST
├── type_checker.rs  # Static type validation
└── ast.rs           # AST node definitions (Expression, Statement, Program)
```

## Lexer Patterns

**Token enum structure:**

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Let, Mut, Fn, If, Else, While, Return,
    // Types
    I32, F32, Bool, String, Vector2, Color,
    // Literals
    IntLiteral(i32), FloatLiteral(f32), StringLiteral(String),
    // Operators
    Plus, Minus, Star, Slash, Equal, EqualEqual,
    // Delimiters
    LeftParen, RightParen, LeftBrace, RightBrace,
    // Special
    Identifier(String), Eof,
}
```

**Lexer implementation:**

- Use `char::is_whitespace()` for whitespace skipping
- Track line/column for error reporting
- Return `Result<Vec<Token>, LexerError>` from `tokenize()`
- Include position in every Token for error messages

## Parser Patterns

**Recursive descent structure:**

```rust
impl Parser {
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut program = Program::new();
        while !self.is_at_end() {
            let decl = self.parse_declaration()?;
            program.add_declaration(decl);
        }
        Ok(program)
    }
    
    fn parse_declaration(&mut self) -> Result<Declaration, ParseError> {
        if self.match_token(&Token::Fn) {
            self.parse_function()
        } else if self.match_token(&Token::Let) {
            self.parse_variable()
        } else {
            Err(self.error("Expected declaration"))
        }
    }
}
```

**Key patterns:**

- `match_token()` consumes token if it matches, returns bool
- `expect_token()` consumes or returns error
- `peek()` looks at current token without consuming
- `advance()` consumes and returns current token
- Error recovery: skip to next statement on error (synchronize on `;` or `}`)

## AST Node Conventions

**Naming:**

- Statements: `VariableDecl`, `FunctionDecl`, `ExpressionStatement`, `ReturnStatement`
- Expressions: `BinaryOp`, `UnaryOp`, `Literal`, `Identifier`, `FunctionCall`
- Types: `Type::I32`, `Type::F32`, `Type::Bool`, `Type::String`, `Type::Vector2`

**Structure:**

```rust
#[derive(Debug, Clone)]
pub struct VariableDecl {
    pub name: String,
    pub mutable: bool,
    pub type_annotation: Option<Type>,
    pub initializer: Option<Expression>,
    pub span: Span,  // For error reporting
}
```

**Always include `span: Span`** in AST nodes for precise error messages.

## Type Checker Patterns

**Type environment:**

```rust
struct TypeEnv {
    scopes: Vec<HashMap<String, Type>>,
}

impl TypeEnv {
    fn define(&mut self, name: String, ty: Type) { ... }
    fn lookup(&self, name: &str) -> Option<&Type> { ... }
    fn enter_scope(&mut self) { ... }
    fn exit_scope(&mut self) { ... }
}
```

**Type checking flow:**

1. Enter function scope
2. Check parameter types match annotations
3. Check function body expressions
4. Verify return type matches annotation
5. Exit scope

**Error reporting:**

```rust
pub enum TypeError {
    Mismatch {
        expected: Type,
        found: Type,
        span: Span,
    },
    UndefinedVariable {
        name: String,
        span: Span,
    },
}
```

## Error Code System

**Allocation:**

- `E001-E099`: Lexer errors
- `E100-E199`: Parser errors
- `E200-E299`: Type checker errors
- `E300-E399`: Signal errors
- `E400-E499`: Reserved
- `E500-E599`: Runtime errors
- `E600-E699`: Node query errors
- `E700-E799`: Godot type errors
- `E800-E899`: Export/Inspector errors

**Error structure:**

```rust
pub struct FerrisError {
    pub code: ErrorCode,  // e.g., E201
    pub message: String,
    pub span: Span,
    pub hint: Option<String>,  // Suggestion for fixing
}
```

**When adding new errors:**

1. Check `docs/ERROR_CODES.md` for next available code in range
2. Add to `ErrorCode` enum
3. Include helpful hint in error message
4. Add test case in `tests/` directory

## Testing Patterns

**Unit test structure:**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lexer_basic_tokens() {
        let source = "let x: i32 = 42;";
        let tokens = Lexer::new(source).tokenize().unwrap();
        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens[0], Token::Let);
    }
    
    #[test]
    fn test_parser_variable_declaration() {
        let source = "let x: i32 = 42;";
        let ast = Parser::new(source).parse().unwrap();
        assert_eq!(ast.declarations.len(), 1);
    }
    
    #[test]
    fn test_type_checker_mismatch() {
        let source = "let x: i32 = \"hello\";";
        let result = compile(source);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, ErrorCode::E201);
    }
}
```

**Test naming:** `test_<component>_<scenario>` (e.g., `test_parser_nested_functions`)

## Common Tasks

### Adding a new keyword

1. Add to `Token` enum in `lexer.rs`
2. Add to keyword map in `Lexer::next_token()`
3. Add parsing logic in `parser.rs`
4. Add AST node if needed in `ast.rs`
5. Add type checking in `type_checker.rs`
6. Update VS Code syntax highlighting in `extensions/vscode/syntaxes/`
7. Add tests for each layer
8. Update `docs/ERROR_CODES.md` if adding new error

### Adding a new type

1. Add to `Type` enum in `ast.rs`
2. Add type literal parsing in `parser.rs` (e.g., `Vector2 { x: 1.0, y: 2.0 }`)
3. Add type checking rules in `type_checker.rs`
4. Add runtime representation in `crates/runtime/`
5. Add Godot conversion in `crates/godot_bind/`
6. Add integration test in `godot_test/scripts/`

### Adding a new error code

1. Check `docs/ERROR_CODES.md` for next available code
2. Add variant to error enum (e.g., `TypeError::NewError`)
3. Include `span: Span` for precise location
4. Write helpful error message with hint
5. Add test case that triggers the error
6. Update `docs/ERROR_CODES.md` with description

## Performance Considerations

- Lexer: ~384 ns - 3.74 μs per script
- Parser: ~600 ns - 7.94 μs per script
- Type checker: ~851 ns - 3.58 μs per script
- Total compilation: sub-millisecond for typical scripts

**Optimization tips:**

- Avoid unnecessary allocations (use `&str` where possible)
- Pre-allocate vectors with `Vec::with_capacity()`
- Use `#[derive(Clone)]` sparingly on large AST nodes
