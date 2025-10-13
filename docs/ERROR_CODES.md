# FerrisScript Error Codes Reference

This document provides a comprehensive reference for all error codes in FerrisScript. Each error includes a unique code, description, common causes, examples, and guidance on how to fix it.

## Table of Contents

- [Overview](#overview)
- [Error Format](#error-format)
- [Error Categories](#error-categories)
  - [Lexical Errors (E001-E099)](#lexical-errors-e001-e099)
  - [Syntax Errors (E100-E199)](#syntax-errors-e100-e199)
  - [Type Errors (E200-E299)](#type-errors-e200-e299)
  - [Semantic Errors (E300-E399)](#semantic-errors-e300-e399)
  - [Runtime Errors (E400-E499)](#runtime-errors-e400-e499)

## Overview

FerrisScript uses structured error codes to help you quickly identify and fix issues in your code. Each error code follows the format `Error[EXXX]:` where `XXX` is a unique number in a specific range:

- **E001-E099**: Lexical/tokenization errors
- **E100-E199**: Syntax/parsing errors
- **E200-E299**: Type checking errors
- **E300-E399**: Semantic/signal errors
- **E200-E299**: Type checking errors
- **E400-E499**: Runtime errors

## Error Format

Compiler errors (lexical, syntax, type) include rich context with source code highlighting:

```
Error[E200]: Type mismatch
  Expected: i32
  Found: f32
  |
5 | let x: i32 = 3.14;
  |              ^^^^ Expected 'i32', found 'f32'
```

Runtime errors use a simpler format:

```
Error[E413]: Division by zero
Error[E401]: Undefined variable: 'foo'
```

---

## Error Categories

### Lexical Errors (E001-E099)

Errors that occur during tokenization of the source code.

#### E001: Invalid Character

**Description**: An invalid or unexpected character was encountered in the source code.

**Common Causes**:

- Using special characters that aren't part of FerrisScript syntax
- Copy-pasting code with non-ASCII characters
- Encoding issues

**Example**:

```ferris
let x = 5 @ 3;  // @ is not a valid operator
```

**Error Message**:

```
Error[E001]: Invalid character
  |
1 | let x = 5 @ 3;
  |           ^ Unexpected character '@' at line 1, column 11
```

**How to Fix**:

- Check that you're using valid FerrisScript operators and syntax
- Remove or replace invalid characters
- Ensure proper file encoding (UTF-8)

**See Also**: [E002](#e002-unterminated-string), [E003](#e003-invalid-number-format)

**Related Codes**: E002, E003

---

#### E002: Unterminated String

**Description**: A string literal was started but never closed with a closing quote.

**Common Causes**:

- Missing closing quote
- Newline inside string (strings must be on one line)
- Unescaped quotes within string

**Example**:

```ferris
let msg = "Hello world;  // Missing closing quote
```

**Error Message**:

```
Error[E002]: Unterminated string
  |
1 | let msg = "Hello world;
  |           ^^^^^^^^^^^^^ String literal not closed at line 1, column 11
```

**How to Fix**:

- Add the missing closing quote: `"Hello world"`
- For multi-line text, use multiple string concatenations
- Escape internal quotes if needed

**See Also**: [E001](#e001-invalid-character)

**Related Codes**: E001

---

#### E003: Invalid Number Format

**Description**: A number literal has an invalid format.

**Common Causes**:

- Multiple decimal points in a number
- Invalid digits in number
- Number too large to represent

**Example**:

```ferris
let x = 3.14.159;  // Two decimal points
```

**Error Message**:

```
Error[E003]: Invalid number format
  |
1 | let x = 3.14.159;
  |         ^^^^^^^^ Invalid number format at line 1, column 9
```

**How to Fix**:

- Use only one decimal point per number: `3.14159`
- Check for typos in numeric literals
- Ensure numbers are within valid range for i32 or f32

**Related Codes**: E001

---

### Syntax Errors (E100-E199)

Errors that occur during parsing of the token stream into an Abstract Syntax Tree (AST).

#### E100: Unexpected Token

**Description**: The parser encountered a token that doesn't fit the expected syntax.

**Common Causes**:

- Missing semicolons, commas, or other punctuation
- Incorrect keyword usage
- Malformed expressions or statements

**Example**:

```ferris
let x = 5
let y = 10;  // Missing semicolon on previous line
```

**Error Message**:

```
Error[E100]: Unexpected token
  |
2 | let y = 10;
  | ^^^ Expected ';', found 'let' at line 2, column 1
```

**How to Fix**:

- Add missing punctuation (semicolons, commas, braces)
- Check syntax against FerrisScript grammar
- Ensure proper statement and expression formatting

**See Also**: [E101](#e101-invalid-top-level-item), [E102](#e102-expected-expression)

**Related Codes**: E101, E102

---

#### E101: Invalid Top-Level Item

**Description**: An invalid statement or declaration was found at the top level of the program.

**Common Causes**:

- Using expressions or statements outside of functions
- Incorrect function or global variable syntax
- Missing required keywords

**Example**:

```ferris
x = 5;  // Assignment at top level (use 'let' or 'let mut')
```

**Error Message**:

```
Error[E101]: Invalid top-level item
  |
1 | x = 5;
  | ^ Expected function or global variable declaration at line 1, column 1
```

**How to Fix**:

- Move code inside a function
- Use `let` or `let mut` for global variables
- Use `fn` keyword for function declarations

**Related Codes**: E100, E102

---

#### E102: Expected Expression

**Description**: The parser expected an expression but found something else.

**Common Causes**:

- Missing operand in binary operation
- Empty parentheses in contexts requiring expressions
- Incomplete expression syntax

**Example**:

```ferris
let x = 5 + ;  // Missing right operand
```

**Error Message**:

```
Error[E102]: Expected expression
  |
1 | let x = 5 + ;
  |             ^ Expected expression, found ';' at line 1, column 13
```

**How to Fix**:

- Complete the expression with required operands
- Remove extra operators
- Check for missing values or variables

**Related Codes**: E100, E103

---

#### E103: Expected Field Name

**Description**: A field name was expected in field access or object construction but wasn't found.

**Common Causes**:

- Missing field name after dot operator
- Invalid field identifier
- Incomplete field access expression

**Example**:

```ferris
let val = vector2.;  // Missing field name
```

**Error Message**:

```
Error[E103]: Expected field name
  |
1 | let val = vector2.;
  |                   ^ Expected field name after '.', found ';' at line 1, column 19
```

**How to Fix**:

- Add the field name: `vector2.x` or `vector2.y`
- Ensure field names are valid identifiers
- Check object type has the field you're accessing

**Related Codes**: E102, E215

---

#### E104: Expected Statement

**Description**: The parser expected a statement but found something else.

**Common Causes**:

- Invalid syntax in function body
- Misplaced expressions
- Incomplete control flow structures

**Example**:

```ferris
fn test() {
    +  // Expression without context
}
```

**Error Message**:

```
Error[E104]: Expected statement
  |
2 |     +
  |     ^ Expected statement, found '+' at line 2, column 5
```

**How to Fix**:

- Use complete statements
- Assign expressions to variables
- Remove stray tokens

**Related Codes**: E100, E102

---

#### E105: Expected Type

**Description**: A type annotation was expected but not provided or invalid.

**Common Causes**:

- Missing type after colon
- Invalid type name
- Incomplete type annotation

**Example**:

```ferris
fn add(a: , b: i32) -> i32 {  // Missing type for 'a'
    return a + b;
}
```

**Error Message**:

```
Error[E105]: Expected type
  |
1 | fn add(a: , b: i32) -> i32 {
  |           ^ Expected type annotation, found ',' at line 1, column 11
```

**How to Fix**:

- Provide type annotation: `a: i32`
- Use valid type names: `i32`, `f32`, `bool`, `String`, `Vector2`
- Complete the type annotation

**Related Codes**: E110, E218

---

#### E106: Expected Identifier

**Description**: An identifier (name) was expected but not found.

**Common Causes**:

- Missing variable, function, or parameter name
- Using keywords as identifiers
- Invalid identifier syntax

**Example**:

```ferris
let = 5;  // Missing variable name
```

**Error Message**:

```
Error[E106]: Expected identifier
  |
1 | let = 5;
  |     ^ Expected identifier, found '=' at line 1, column 5
```

**How to Fix**:

- Provide a valid identifier name
- Use alphanumeric characters and underscores
- Don't start identifiers with numbers

**Related Codes**: E109

---

#### E107: Expected Block

**Description**: A code block (enclosed in curly braces) was expected but not found.

**Common Causes**:

- Missing braces in function body
- Incomplete if/while statement
- Single-line statements where blocks are required

**Example**:

```ferris
fn test()
    return 5;  // Missing braces around function body
```

**Error Message**:

```
Error[E107]: Expected block
  |
2 |     return 5;
  |     ^^^^^^ Expected '{', found 'return' at line 2, column 5
```

**How to Fix**:

- Add curly braces: `fn test() { return 5; }`
- Ensure all required blocks are present
- Check matching braces

**Related Codes**: E100

---

#### E108: Expected Parameter

**Description**: A function parameter was expected but not properly formed.

**Common Causes**:

- Missing parameter name or type
- Invalid parameter syntax
- Extra commas in parameter list

**Example**:

```ferris
fn add(a: i32, ) -> i32 {  // Trailing comma without parameter
    return a + 1;
}
```

**Error Message**:

```
Error[E108]: Expected parameter
  |
1 | fn add(a: i32, ) -> i32 {
  |                ^ Expected parameter, found ')' at line 1, column 16
```

**How to Fix**:

- Remove trailing commas
- Complete parameter declarations
- Use format: `name: type`

**Related Codes**: E111

---

#### E109: Invalid Identifier

**Description**: An identifier name is invalid or uses a reserved keyword.

**Common Causes**:

- Using FerrisScript keywords as names
- Invalid characters in identifier
- Reserved words as variable/function names

**Example**:

```ferris
let fn = 5;  // 'fn' is a keyword
```

**Error Message**:

```
Error[E109]: Invalid identifier
  |
1 | let fn = 5;
  |     ^^ 'fn' is a reserved keyword and cannot be used as an identifier at line 1, column 5
```

**How to Fix**:

- Choose a different name
- Avoid keywords: `fn`, `let`, `mut`, `if`, `else`, `while`, `return`, `true`, `false`
- Use descriptive, non-reserved names

**Related Codes**: E106

---

#### E110: Invalid Type

**Description**: An invalid or unknown type was specified.

**Common Causes**:

- Typo in type name
- Using undefined custom types
- Incorrect type syntax

**Example**:

```ferris
let x: int = 5;  // 'int' is not valid, should be 'i32'
```

**Error Message**:

```
Error[E110]: Invalid type
  |
1 | let x: int = 5;
  |        ^^^ Unknown type 'int' at line 1, column 8
```

**How to Fix**:

- Use valid types: `i32`, `f32`, `bool`, `String`, `Vector2`
- Check for typos in type names
- Refer to type documentation

**Related Codes**: E105, E218

---

#### E111: Invalid Parameter

**Description**: A function parameter has invalid syntax or structure.

**Common Causes**:

- Missing colon between name and type
- Invalid parameter format
- Duplicate parameter names

**Example**:

```ferris
fn add(a i32, b: i32) -> i32 {  // Missing colon
    return a + b;
}
```

**Error Message**:

```
Error[E111]: Invalid parameter
  |
1 | fn add(a i32, b: i32) -> i32 {
  |        ^^^^^ Expected ':' after parameter name at line 1, column 8
```

**How to Fix**:

- Use format: `name: type`
- Ensure each parameter is properly formatted
- Check for duplicate names

**Related Codes**: E108

---

#### E112: Invalid Return Type

**Description**: The return type of a function has invalid syntax.

**Common Causes**:

- Missing or invalid type after `->`
- Typo in return type
- Incorrect return type syntax

**Example**:

```ferris
fn test() -> {  // Missing return type
    return 5;
}
```

**Error Message**:

```
Error[E112]: Invalid return type
  |
1 | fn test() -> {
  |              ^ Expected return type after '->', found '{' at line 1, column 14
```

**How to Fix**:

- Specify return type: `-> i32`
- Use valid type names
- Omit `->` for void functions

**Related Codes**: E110

---

#### E113: Invalid Operator

**Description**: An invalid or unexpected operator was encountered.

**Common Causes**:

- Using undefined operators
- Operator in wrong context
- Typo in operator

**Example**:

```ferris
let x = 5 ** 2;  // '**' is not a valid operator
```

**Error Message**:

```
Error[E113]: Invalid operator
  |
1 | let x = 5 ** 2;
  |           ^^ Unsupported or invalid operator at line 1, column 11
```

**How to Fix**:

- Use valid operators: `+`, `-`, `*`, `/`, `==`, `!=`, `<`, `<=`, `>`, `>=`, `&&`, `||`, `!`
- Check operator documentation
- Verify operator usage

**Related Codes**: E100

---

### Type Errors (E200-E299)

Errors that occur during type checking of the AST.

#### E200: Type Mismatch

**Description**: An expression or value has a type that doesn't match what's expected.

**Common Causes**:

- Assigning wrong type to variable
- Passing wrong type to function
- Type incompatibility in operations

**Example**:

```ferris
let x: i32 = 3.14;  // Assigning f32 to i32 variable
```

**Error Message**:

```
Error[E200]: Type mismatch
  Expected: i32
  Found: f32
  |
1 | let x: i32 = 3.14;
  |              ^^^^ Expected 'i32', found 'f32'
```

**How to Fix**:

- Change variable type to match value: `let x: f32 = 3.14;`
- Convert value to correct type if needed
- Ensure type annotations match actual types

**See Also**: [E219](#e219-incompatible-types-in-assignment), [E205](#e205-incorrect-argument-type), [E211](#e211-condition-must-be-boolean)

**Related Codes**: E219, E205

---

#### E201: Undefined Variable

**Description**: A variable is used before it's declared or is not in scope.

**Common Causes**:

- Typo in variable name
- Using variable before declaration
- Variable out of scope

**Example**:

```ferris
fn test() {
    let x = y + 5;  // 'y' is not defined
}
```

**Error Message**:

```
Error[E201]: Undefined variable 'y'
  |
2 |     let x = y + 5;
  |             ^ Variable must be declared before use
```

**How to Fix**:

- Declare the variable before use: `let y = 10;`
- Check for typos in variable name
- Ensure variable is in scope

**See Also**: [E401](#e401-undefined-variable) (runtime version)

**Related Codes**: E401

---

#### E202: Undefined Function

**Description**: A function is called but hasn't been defined.

**Common Causes**:

- Typo in function name
- Function not defined yet
- Function in wrong scope

**Example**:

```ferris
fn main() {
    let x = add(5, 3);  // 'add' function not defined
}
```

**Error Message**:

```
Error[E202]: Undefined function 'add'
  |
2 |     let x = add(5, 3);
  |             ^^^ Function not found
```

**How to Fix**:

- Define the function before calling
- Check for typos in function name
- Ensure function is declared at top level

**See Also**: [E415](#e415-undefined-function) (runtime version), [E402](#e402-unknown-built-in-function)

**Related Codes**: E415

---

#### E204: Wrong Number of Arguments

**Description**: A function is called with the wrong number of arguments.

**Common Causes**:

- Missing arguments in function call
- Too many arguments provided
- Misunderstanding function signature

**Example**:

```ferris
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    let x = add(5);  // Missing second argument
}
```

**Error Message**:

```
Error[E204]: Wrong number of arguments
  Function 'add' expects 2 arguments, got 1
  |
6 |     let x = add(5);
  |             ^^^^^^ Expected 2 arguments
```

**How to Fix**:

- Provide all required arguments: `add(5, 3)`
- Check function signature
- Ensure argument count matches

**Related Codes**: E416

---

#### E205: Incorrect Argument Type

**Description**: An argument passed to a function has the wrong type.

**Common Causes**:

- Passing wrong type to parameter
- Type mismatch in function call
- Incorrect literal type

**Example**:

```ferris
fn greet(name: String) {
    print(name);
}

fn main() {
    greet(42);  // Passing i32 instead of String
}
```

**Error Message**:

```
Error[E205]: Incorrect argument type
  Parameter 'name' expects String, got i32
  |
6 |     greet(42);
  |           ^^ Expected 'String', found 'i32'
```

**How to Fix**:

- Pass correct type: `greet("Alice")`
- Check function parameter types
- Convert argument if necessary

**Related Codes**: E200

---

#### E209: Invalid Field Access

**Description**: Attempting to access a field on a value that doesn't support field access.

**Common Causes**:

- Trying to access field on primitive type
- Field access on non-object value
- Wrong value type

**Example**:

```ferris
let x: i32 = 42;
let y = x.field;  // Can't access field on i32
```

**Error Message**:

```
Error[E209]: Invalid field access
  Cannot access field 'field' on i32
  |
2 | let y = x.field;
  |         ^ i32 does not have fields
```

**How to Fix**:

- Use field access only on objects like Vector2
- Check value type supports field access
- Verify value is correct type

**Related Codes**: E215, E408

---

#### E211: Condition Must Be Boolean

**Description**: A condition in an if statement or while loop must be a boolean expression.

**Common Causes**:

- Using non-boolean in condition
- Missing comparison operator
- Type error in condition

**Example**:

```ferris
let x = 5;
if x {  // 'x' is i32, not bool
    print("true");
}
```

**Error Message**:

```
Error[E211]: Condition must be boolean
  Condition in if/while must be type bool, got i32
  |
2 | if x {
  |    ^ Expected bool, found i32
```

**How to Fix**:

- Use boolean expression: `if x > 0 {`
- Add comparison: `if x == 5 {`
- Ensure condition evaluates to bool

**Related Codes**: E200

---

#### E212: Binary Operation Type Error

**Description**: A binary operation has operands of incompatible types.

**Common Causes**:

- Mixing incompatible types in operation
- Wrong operand types for operator
- Missing type conversion

**Example**:

```ferris
let x = "hello" + 42;  // Can't add String and i32
```

**Error Message**:

```
Error[E212]: Binary operation type error
  Cannot apply '+' to String and i32
  |
1 | let x = "hello" + 42;
  |         ^^^^^^^^^^^^^ Incompatible types
```

**How to Fix**:

- Use compatible types for operation
- Convert types if needed
- Check operator requirements

**Related Codes**: E200

---

#### E213: Unary Operation Type Error

**Description**: A unary operation is applied to an incompatible type.

**Common Causes**:

- Wrong operand type for unary operator
- Using `-` on non-numeric type
- Using `!` on non-boolean type

**Example**:

```ferris
let x = -"hello";  // Can't negate a string
```

**Error Message**:

```
Error[E213]: Unary operation type error
  Cannot apply '-' to String
  |
1 | let x = -"hello";
  |         ^^^^^^^^ '-' requires numeric type
```

**How to Fix**:

- Use correct type for operator
- `-` works on i32 and f32
- `!` works on bool

**Related Codes**: E414

---

#### E215: Field Not Found

**Description**: Attempting to access a field that doesn't exist on the object.

**Common Causes**:

- Typo in field name
- Field doesn't exist on type
- Wrong object type

**Example**:

```ferris
let vec = Vector2 { x: 1.0, y: 2.0 };
let z = vec.z;  // Vector2 doesn't have 'z' field
```

**Error Message**:

```
Error[E215]: Field not found
  Vector2 has no field 'z'
  |
2 | let z = vec.z;
  |             ^ Valid fields are: x, y
```

**How to Fix**:

- Check field name spelling
- Use valid fields for type
- For Vector2: only `x` and `y` are valid

**See Also**: [E407](#e407-vector2-has-no-field) (runtime version), [E103](#e103-expected-field-name), [E209](#e209-invalid-field-access)

**Related Codes**: E407, E103

---

#### E218: Type Annotation Required

**Description**: A type cannot be inferred and must be explicitly annotated.

**Common Causes**:

- Variable initialized without value or type
- Ambiguous type inference
- Missing type annotation

**Example**:

```ferris
let x;  // No type or initial value
```

**Error Message**:

```
Error[E218]: Type annotation required
  Cannot infer type for 'x'
  |
1 | let x;
  |     ^ Provide type annotation or initial value
```

**How to Fix**:

- Add type annotation: `let x: i32;`
- Provide initial value: `let x = 0;`
- Do both: `let x: i32 = 0;`

**Related Codes**: E105, E110

---

#### E219: Incompatible Types in Assignment

**Description**: Cannot assign a value to a variable due to type incompatibility.

**Common Causes**:

- Assigning wrong type to existing variable
- Type changed after declaration
- Incompatible types in reassignment

**Example**:

```ferris
let mut x = 5;
x = "hello";  // Can't assign String to i32 variable
```

**Error Message**:

```
Error[E219]: Incompatible types in assignment
  Cannot assign String to variable of type i32
  |
2 | x = "hello";
  |     ^^^^^^^ Expected i32, found String
```

**How to Fix**:

- Assign value of correct type
- Check variable type
- Use new variable if type needs to change

**Related Codes**: E200

---

### Semantic Errors (E300-E399)

Errors related to signal declarations and usage.

#### E301: Signal Already Defined

**Description**: A signal with the same name has already been declared in the current scope.

**Common Causes**:

- Declaring the same signal twice
- Copy-pasting signal declarations
- Name collision with existing signal

**Example**:

```ferris
signal health_changed(old: i32, new: i32);
signal health_changed(value: i32);  // Error: signal already defined
```

**Error Message**:

```
Error[E301]: Signal already defined
  Signal 'health_changed' is already defined
  |
2 | signal health_changed(value: i32);
  |        ^^^^^^^^^^^^^^ Signal already declared at line 1
```

**How to Fix**:

- Remove duplicate signal declaration
- Rename one of the signals
- Check for existing signals with the same name

**Related Codes**: E302, E303, E304

---

#### E302: Signal Not Defined

**Description**: Attempting to emit a signal that has not been declared.

**Common Causes**:

- Typo in signal name
- Signal not declared before use
- Signal declared in different scope

**Example**:

```ferris
fn take_damage() {
    emit_signal("health_change", 100, 75);  // Typo: should be "health_changed"
}
```

**Error Message**:

```
Error[E302]: Signal not defined
  Signal 'health_change' is not defined
  |
2 |     emit_signal("health_change", 100, 75);
  |                 ^^^^^^^^^^^^^^^ Signal not declared
  |
  = help: Did you mean 'health_changed'?
```

**How to Fix**:

- Declare the signal before using it
- Check signal name spelling
- Verify signal is in scope

**Related Codes**: E301, E303, E304

---

#### E303: Signal Parameter Count Mismatch

**Description**: The number of arguments provided to `emit_signal` doesn't match the signal's declared parameter count.

**Common Causes**:

- Missing arguments in emit_signal call
- Too many arguments provided
- Incorrect signal signature

**Example**:

```ferris
signal health_changed(old: i32, new: i32);

fn take_damage() {
    emit_signal("health_changed", 75);  // Missing 'old' parameter
}
```

**Error Message**:

```
Error[E303]: Signal parameter count mismatch
  Signal 'health_changed' expects 2 parameters, but 1 provided
  |
4 |     emit_signal("health_changed", 75);
  |                 ^^^^^^^^^^^^^^^^^^^^^^ Expected 2 arguments
```

**How to Fix**:

- Provide all required parameters
- Check signal declaration
- Verify argument count matches declaration

**Related Codes**: E301, E302, E304

---

#### E304: Signal Parameter Type Mismatch

**Description**: An argument provided to `emit_signal` doesn't match the expected parameter type.

**Common Causes**:

- Wrong type passed as signal parameter
- Type confusion
- Missing type coercion

**Example**:

```ferris
signal score_updated(score: i32);

fn add_score() {
    emit_signal("score_updated", "100");  // String instead of i32
}
```

**Error Message**:

```
Error[E304]: Signal parameter type mismatch
  Signal 'score_updated' parameter 1 expects i32, but String provided
  |
4 |     emit_signal("score_updated", "100");
  |                                  ^^^^^ Expected i32, found String
```

**How to Fix**:

- Use correct parameter type
- Check signal declaration
- Convert value to expected type
- Note: i32 can be implicitly converted to f32

**Related Codes**: E301, E302, E303, E200

---

### Runtime Errors (E400-E499)

Errors that occur during program execution.

#### E400: Cannot Assign to Immutable Variable

**Description**: Attempting to assign to a variable that wasn't declared as mutable.

**Common Causes**:

- Missing `mut` keyword in variable declaration
- Trying to modify immutable variable
- Confusion about mutability

**Example**:

```ferris
let x = 5;
x = 10;  // 'x' is immutable
```

**Error Message**:

```
Error[E400]: Cannot assign to immutable variable 'x'
```

**How to Fix**:

- Declare variable as mutable: `let mut x = 5;`
- Or don't reassign: use a new variable
- Check if mutation is necessary

**See Also**: [E405](#e405-cannot-assign-to-field-of-immutable-variable)

**Related Codes**: E405

---

#### E401: Undefined Variable

**Description**: Runtime reference to a variable that doesn't exist.

**Common Causes**:

- Variable used before runtime initialization
- Variable went out of scope
- Typo in variable name

**Example**:

```ferris
fn test() {
    print(undefined_var);  // Variable not defined
}
```

**Error Message**:

```
Error[E401]: Undefined variable: undefined_var
```

**How to Fix**:

- Define the variable first
- Check variable scope
- Verify variable name spelling

**See Also**: [E201](#e201-undefined-variable) (compile-time version)

**Related Codes**: E201

---

#### E402: Unknown Built-in Function

**Description**: Calling a built-in function that doesn't exist.

**Common Causes**:

- Typo in built-in function name
- Using undefined built-in
- Wrong function name

**Example**:

```ferris
fn main() {
    println("Hello");  // 'println' doesn't exist, use 'print'
}
```

**Error Message**:

```
Error[E402]: Unknown built-in function: println
```

**How to Fix**:

- Use correct built-in name: `print`
- Check built-in function documentation
- Verify function exists

**Related Codes**: E202, E415

---

#### E403: Invalid Assignment Target

**Description**: Attempting to assign to something that can't be assigned to.

**Common Causes**:

- Assigning to literal value
- Invalid left-hand side of assignment
- Wrong expression type

**Example**:

```ferris
fn test() {
    5 = x;  // Can't assign to literal
}
```

**Error Message**:

```
Error[E403]: Invalid assignment target
```

**How to Fix**:

- Assign to variables, not literals
- Check left side of assignment is valid
- Use proper assignment syntax

**Related Codes**: None

---

#### E404: Cannot Set Self Properties (No Setter)

**Description**: Attempting to set a property on `self` when no property setter is registered.

**Common Causes**:

- Godot integration not set up
- Missing property setter callback
- Runtime environment issue

**Example**:

```ferris
fn _ready() {
    self.position = Vector2 { x: 0.0, y: 0.0 };
}
```

**Error Message**:

```
Error[E404]: Cannot set self properties: no property setter registered
```

**How to Fix**:

- Ensure Godot environment is properly set up
- Check FerrisScript integration
- Verify property setter callback registration

**Related Codes**: E410, E417

---

#### E405: Cannot Assign to Field of Immutable Variable

**Description**: Attempting to modify a field of an immutable variable.

**Common Causes**:

- Variable not declared as mutable
- Trying to modify immutable object field
- Missing `mut` keyword

**Example**:

```ferris
let vec = Vector2 { x: 1.0, y: 2.0 };
vec.x = 5.0;  // 'vec' is immutable
```

**Error Message**:

```
Error[E405]: Cannot assign to field of immutable variable 'vec'
```

**How to Fix**:

- Declare variable as mutable: `let mut vec = ...`
- Create new variable instead of modifying
- Consider mutability requirements

**Related Codes**: E400

---

#### E406: Cannot Assign Value to Vector2 Field

**Description**: Attempting to assign an incompatible value to a Vector2 field.

**Common Causes**:

- Assigning non-numeric value to x or y
- Wrong value type
- Type mismatch in field assignment

**Example**:

```ferris
let mut vec = Vector2 { x: 1.0, y: 2.0 };
vec.x = "hello";  // Can't assign String to float field
```

**Error Message**:

```
Error[E406]: Cannot assign String to Vector2.x
```

**How to Fix**:

- Assign numeric value: `vec.x = 5.0;`
- Use i32 or f32 for fields
- Check value type

**Related Codes**: E200, E407

---

#### E407: Vector2 Has No Field

**Description**: Attempting to access a field that doesn't exist on Vector2.

**Common Causes**:

- Typo in field name
- Using invalid field
- Wrong field for type

**Example**:

```ferris
let vec = Vector2 { x: 1.0, y: 2.0 };
let z = vec.z;  // Vector2 only has x and y
```

**Error Message**:

```
Error[E407]: Vector2 has no field 'z'
```

**How to Fix**:

- Use valid fields: `x` or `y`
- Check field name spelling
- Refer to Vector2 documentation

**See Also**: [E215](#e215-field-not-found) (compile-time version), [E406](#e406-cannot-assign-value-to-vector2-field)

**Related Codes**: E215

---

#### E408: Cannot Access Field on Non-Object

**Description**: Attempting to access a field on a value that doesn't support fields.

**Common Causes**:

- Field access on primitive type
- Wrong value type
- Type confusion

**Example**:

```ferris
let x = 42;
let y = x.field;  // i32 doesn't have fields
```

**Error Message**:

```
Error[E408]: Cannot access field 'field' on i32
```

**How to Fix**:

- Use field access only on objects
- Check value is correct type
- Verify value supports fields

**Related Codes**: E209

---

#### E409: Property Is Not a Vector2

**Description**: Expected a Vector2 property but got a different type.

**Common Causes**:

- Wrong property type from Godot
- Type mismatch in property access
- Incorrect property

**Example**:

```ferris
fn _process(delta: f32) {
    self.name.x = 5.0;  // 'name' is String, not Vector2
}
```

**Error Message**:

```
Error[E409]: Property 'name' is not a Vector2
```

**How to Fix**:

- Use correct property type
- Check Godot property types
- Access appropriate properties

**Related Codes**: E200

---

#### E410: Cannot Get Self Properties (No Getter)

**Description**: Attempting to get a property from `self` when no property getter is registered.

**Common Causes**:

- Godot integration not set up
- Missing property getter callback
- Runtime environment issue

**Example**:

```ferris
fn test() {
    let pos = self.position;
}
```

**Error Message**:

```
Error[E410]: Cannot get self properties: no property getter registered
```

**How to Fix**:

- Ensure Godot environment is set up
- Check FerrisScript integration
- Verify property getter callback

**Related Codes**: E404, E417

---

#### E411: Nested Field Assignment Not Yet Implemented

**Description**: Nested field assignments on regular variables are not yet supported.

**Common Causes**:

- Using complex nested field access
- Feature not implemented
- Unsupported syntax

**Example**:

```ferris
let mut obj = ...;
obj.field.subfield = value;  // Not yet supported
```

**Error Message**:

```
Error[E411]: Nested field assignment on regular variables not yet implemented
```

**How to Fix**:

- Use simpler field access patterns
- Access fields individually
- Wait for feature implementation

**Related Codes**: E412

---

#### E412: Complex Field Assignment Not Yet Implemented

**Description**: Complex field assignment patterns are not yet supported.

**Common Causes**:

- Using advanced field assignment syntax
- Feature not implemented
- Complex expressions in field access

**Example**:

```ferris
complex_expression().field = value;
```

**Error Message**:

```
Error[E412]: Complex field assignment not yet implemented
```

**How to Fix**:

- Simplify field assignment
- Use intermediate variables
- Wait for feature implementation

**Related Codes**: E411

---

#### E413: Division by Zero

**Description**: Attempting to divide by zero.

**Common Causes**:

- Literal zero in division
- Variable with zero value
- Calculation resulting in zero divisor

**Example**:

```ferris
let x = 10 / 0;  // Division by zero
```

**Error Message**:

```
Error[E413]: Division by zero
```

**How to Fix**:

- Check divisor is not zero before division
- Add conditional checks
- Validate input values

**Related Codes**: None

---

#### E414: Cannot Negate Non-Numeric Value

**Description**: Attempting to use unary negation on a non-numeric type.

**Common Causes**:

- Using `-` on String, bool, or other non-numeric
- Wrong operand type
- Type error in expression

**Example**:

```ferris
let x = -"hello";  // Can't negate String
```

**Error Message**:

```
Error[E414]: Cannot negate non-numeric value
```

**How to Fix**:

- Use negation only on i32 or f32
- Check operand type
- Ensure numeric value

**Related Codes**: E213

---

#### E415: Undefined Function

**Description**: Runtime call to a function that doesn't exist.

**Common Causes**:

- Function not defined
- Typo in function name
- Function not initialized

**Example**:

```ferris
fn main() {
    call_function("undefined_func", []);
}
```

**Error Message**:

```
Error[E415]: Undefined function: undefined_func
```

**How to Fix**:

- Define the function
- Check function name spelling
- Ensure function is registered

**Related Codes**: E202, E402

---

#### E416: Wrong Number of Arguments

**Description**: Runtime function call with wrong number of arguments.

**Common Causes**:

- Missing arguments
- Too many arguments
- Argument count mismatch

**Example**:

```ferris
fn add(a: i32, b: i32) -> i32 { return a + b; }

fn main() {
    let result = add(5, 10, 15);  // Too many arguments
}
```

**Error Message**:

```
Error[E416]: Function 'add' expects 2 arguments, got 3
```

**How to Fix**:

- Provide correct number of arguments
- Check function signature
- Remove extra arguments

**Related Codes**: E204

---

#### E417: Cannot Access Self Properties (No Property Getter)

**Description**: Attempting to access `self` properties when no property getter is registered.

**Common Causes**:

- Godot integration not configured
- Missing property getter callback
- Runtime setup issue

**Example**:

```ferris
fn test() {
    let pos = self.position;
}
```

**Error Message**:

```
Error[E417]: Cannot access self properties: no property getter registered
```

**How to Fix**:

- Configure Godot integration properly
- Register property getter callback
- Check runtime environment setup

**Related Codes**: E404, E410

---

#### E418: Assignment Expressions Should Be Statements

**Description**: Assignment used as an expression in invalid context.

**Common Causes**:

- Using assignment in expression context
- Wrong syntax usage
- Internal parser/runtime issue

**Example**:

```ferris
let x = (y = 5);  // Assignment as expression
```

**Error Message**:

```
Error[E418]: Assignment expressions should be statements
```

**How to Fix**:

- Use assignments as statements, not expressions
- Separate assignment from expression
- Use proper statement syntax

**Related Codes**: None

---

#### E501: emit_signal Requires Signal Name

**Description**: `emit_signal` was called without providing a signal name as the first argument.

**Common Causes**:

- Calling emit_signal with no arguments
- Missing signal name parameter
- Incorrect function call syntax

**Example**:

```ferris
fn trigger_event() {
    emit_signal();  // Missing signal name
}
```

**Error Message**:

```
Error[E501]: emit_signal requires at least a signal name
```

**How to Fix**:

- Provide signal name as first argument
- Ensure signal name is a string literal
- Check emit_signal call syntax

**Correct Usage**:

```ferris
emit_signal("player_died");
emit_signal("health_changed", 100, 75);
```

**Related Codes**: E502, E302, E303

---

#### E502: emit_signal Signal Name Must Be String

**Description**: The first argument to `emit_signal` must be a string literal containing the signal name.

**Common Causes**:

- Passing non-string value as signal name
- Using variable instead of string literal
- Type error in first argument

**Example**:

```ferris
fn trigger_event() {
    emit_signal(123, 456);  // First argument must be string
}
```

**Error Message**:

```
Error[E502]: emit_signal first argument must be a string
```

**How to Fix**:

- Use string literal for signal name
- Check first argument type
- Signal name must be known at compile time

**Correct Usage**:

```ferris
emit_signal("score_updated", 100);
emit_signal("player_died");
```

**Related Codes**: E501, E302

---

## Getting More Help

If you encounter an error code not listed here or need additional help:

1. Check the [FerrisScript Documentation](../README.md)
2. Review the [FAQ](./FAQ.md)
3. Search for issues on [GitHub Issues](https://github.com/dev-parkins/FerrisScript/issues)
4. Ask in the community discussions

## Contributing

Found an error code that's unclear or missing information? Please submit a PR to improve this documentation!
