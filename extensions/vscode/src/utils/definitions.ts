/**
 * Shared data definitions for FerrisScript language features
 * Centralizes keyword, type, and function definitions to eliminate duplication
 * between completion and hover providers
 */

/**
 * Core language feature data structure
 */
export interface LanguageFeature {
    readonly name: string;
    readonly description: string;
    readonly syntax?: string;
    readonly example: string;
    readonly category: 'keyword' | 'type' | 'function';
}

/**
 * Keyword feature with completion metadata
 */
export interface KeywordFeature extends LanguageFeature {
    readonly category: 'keyword';
    readonly statementLevel: boolean;
    readonly insertText: string;
}

/**
 * Type feature with type system metadata
 */
export interface TypeFeature extends LanguageFeature {
    readonly category: 'type';
    readonly isBuiltin: boolean;
}

/**
 * Function feature with signature metadata
 */
export interface FunctionFeature extends LanguageFeature {
    readonly category: 'function';
    readonly signature: string;
    readonly returnType: string;
}

/**
 * FerrisScript keyword definitions
 */
export const KEYWORDS: ReadonlyArray<KeywordFeature> = [
    {
        name: 'fn',
        description: 'Declares a new function',
        syntax: 'fn name(param: type) -> return_type { ... }',
        example: 'fn add(a: i32, b: i32) -> i32 {\n    return a + b;\n}',
        category: 'keyword',
        statementLevel: true,
        insertText: 'fn ${1:name}(${2:params}) {\n\t$0\n}',
    },
    {
        name: 'let',
        description: 'Declares an immutable variable',
        syntax: 'let name: type = value;',
        example: 'let x: i32 = 42;\nlet message: String = "Hello";',
        category: 'keyword',
        statementLevel: true,
        insertText: 'let ${1:name}: ${2:i32} = $0;',
    },
    {
        name: 'mut',
        description: 'Declares a mutable variable (used with let)',
        syntax: 'let mut name: type = value;',
        example: 'let mut counter: i32 = 0;\ncounter += 1;',
        category: 'keyword',
        statementLevel: false,
        insertText: 'mut',
    },
    {
        name: 'if',
        description: 'Conditional statement - executes code if condition is true',
        syntax: 'if condition { ... }',
        example: 'if x > 0 {\n    print("positive");\n}',
        category: 'keyword',
        statementLevel: true,
        insertText: 'if ${1:condition} {\n\t$0\n}',
    },
    {
        name: 'else',
        description: 'Alternative branch for if statement',
        syntax: 'if condition { ... } else { ... }',
        example: 'if speed > 100.0 {\n    print("Fast");\n} else {\n    print("Slow");\n}',
        category: 'keyword',
        statementLevel: false,
        insertText: 'else {\n\t$0\n}',
    },
    {
        name: 'while',
        description: 'Loop that executes while condition is true',
        syntax: 'while condition { ... }',
        example: 'let mut count: i32 = 0;\nwhile count < 10 {\n    count += 1;\n}',
        category: 'keyword',
        statementLevel: true,
        insertText: 'while ${1:condition} {\n\t$0\n}',
    },
    {
        name: 'return',
        description: 'Returns a value from a function',
        syntax: 'return value;',
        example: 'fn get_value() -> i32 {\n    return 42;\n}',
        category: 'keyword',
        statementLevel: true,
        insertText: 'return $0;',
    },
    {
        name: 'true',
        description: 'Boolean true literal',
        syntax: 'true',
        example: 'let is_active: bool = true;',
        category: 'keyword',
        statementLevel: false,
        insertText: 'true',
    },
    {
        name: 'false',
        description: 'Boolean false literal',
        syntax: 'false',
        example: 'let is_active: bool = false;',
        category: 'keyword',
        statementLevel: false,
        insertText: 'false',
    },
];

/**
 * FerrisScript type definitions
 */
export const TYPES: ReadonlyArray<TypeFeature> = [
    {
        name: 'i32',
        description: '32-bit signed integer',
        syntax: 'let x: i32 = 42;',
        example: 'let score: i32 = 100;\nlet damage: i32 = -50;',
        category: 'type',
        isBuiltin: true,
    },
    {
        name: 'f32',
        description: '32-bit floating point number',
        syntax: 'let x: f32 = 3.14;',
        example: 'let speed: f32 = 100.5;\nlet pi: f32 = 3.14159;',
        category: 'type',
        isBuiltin: true,
    },
    {
        name: 'bool',
        description: 'Boolean type (true/false)',
        syntax: 'let x: bool = true;',
        example: 'let is_active: bool = true;\nlet can_move: bool = false;',
        category: 'type',
        isBuiltin: true,
    },
    {
        name: 'String',
        description: 'UTF-8 string type',
        syntax: 'let s: String = "text";',
        example: 'let name: String = "Player";\nlet message: String = "Hello, World!";',
        category: 'type',
        isBuiltin: true,
    },
    {
        name: 'void',
        description: 'No return value (for functions)',
        syntax: 'fn name() -> void { ... }',
        example: 'fn print_message() -> void {\n    print("Hello");\n}',
        category: 'type',
        isBuiltin: true,
    },
    {
        name: 'Vector2',
        description: 'Godot 2D vector type',
        syntax: 'let v: Vector2 = Vector2(x, y);',
        example: 'let position: Vector2 = Vector2(100.0, 200.0);\nlet velocity: Vector2 = Vector2(5.0, 0.0);',
        category: 'type',
        isBuiltin: false,
    },
    {
        name: 'Node',
        description: 'Base Godot node type',
        syntax: 'let node: Node;',
        example: 'let child: Node = self.get_child(0);',
        category: 'type',
        isBuiltin: false,
    },
];

/**
 * FerrisScript built-in function definitions
 */
export const FUNCTIONS: ReadonlyArray<FunctionFeature> = [
    {
        name: 'print',
        description: 'Prints a message to the console',
        syntax: 'print(message: String) -> void',
        signature: 'print(message: String)',
        returnType: 'void',
        example: 'print("Hello, World!");\nprint("Score: " + score.to_string());',
        category: 'function',
    },
];

/**
 * Get keyword by name
 */
export function getKeyword(name: string): KeywordFeature | undefined {
    return KEYWORDS.find(k => k.name === name);
}

/**
 * Get type by name
 */
export function getType(name: string): TypeFeature | undefined {
    return TYPES.find(t => t.name === name);
}

/**
 * Get function by name
 */
export function getFunction(name: string): FunctionFeature | undefined {
    return FUNCTIONS.find(f => f.name === name);
}
