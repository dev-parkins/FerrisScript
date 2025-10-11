# Phase 1: Signal Support 🔥

**Version**: v0.0.4  
**Phase**: 1 of 5  
**Priority**: Critical (Core Godot Feature)  
**Status**: Not Started  
**Branch**: `feature/v0.0.4-signals`  
**Estimated Effort**: 5-7 days

---

## 🎯 Overview

**Goal**: Implement signal support in FerrisScript to enable event-driven programming patterns essential for Godot game development.

**Strategic Importance**: Signals are the foundation of event-driven architecture in Godot. This feature enables:

- Decoupled game logic (observers don't need direct references to subjects)
- UI event handling (button clicks, slider changes)
- Custom gameplay events (player_damaged, enemy_spawned, level_complete)
- Godot editor integration (connect signals visually)

**Scope**: Full signal lifecycle - definition, emission, connection (from code and editor), disconnection, parameter passing.

---

## 📋 Acceptance Criteria

### 1. Signal Definition ✅

**Requirement**: Signals can be defined at module level with typed parameters

**Example**:

```rust
signal health_changed(old: i32, new: i32);
signal player_died;  // No parameters
signal level_complete(score: i32, time: f32, stars: bool);
```

**Verification**:

- [ ] Parser recognizes `signal` keyword
- [ ] AST node created for signal declarations
- [ ] Type checker validates parameter types
- [ ] Multiple parameters supported (0 to 5+)
- [ ] Signals stored in environment/symbol table
- [ ] Error on duplicate signal names

---

### 2. Signal Emission ✅

**Requirement**: Signals can be emitted with matching parameters

**Example**:

```rust
let old_health: i32 = 100;
let new_health: i32 = 75;
emit_signal("health_changed", old_health, new_health);

emit_signal("player_died");  // No arguments for parameterless signal
```

**Verification**:

- [ ] `emit_signal` built-in function recognized
- [ ] First argument must be string literal (signal name)
- [ ] Parameter count matches signal definition
- [ ] Parameter types match signal definition (or coercible)
- [ ] Runtime emits signal through Godot binding
- [ ] Error on undefined signal name
- [ ] Error on parameter mismatch

---

### 3. Signal Connection (Godot Editor) ✅

**Requirement**: Signals can be connected to methods from Godot Inspector

**Example** (Godot editor workflow):

1. Select node with FerrisScript attached
2. Open "Node" tab → "Signals"
3. See `health_changed(old: i32, new: i32)` in list
4. Connect to method `on_health_changed(old: i32, new: i32)`

**Verification**:

- [ ] Signals exposed to Godot's signal system via GDExtension
- [ ] Signal parameters visible in Godot Inspector
- [ ] Connection from editor triggers FerrisScript method
- [ ] Parameters passed correctly from emission to receiver

---

### 4. Signal Connection (FerrisScript Code) ✅

**Requirement**: Signals can be connected programmatically

**Example**:

```rust
fn _ready() {
    // Connect own signal to own method
    connect("health_changed", self, "on_health_changed");
    
    // Connect to another node's signal
    let ui: Node = get_node("UI/HealthBar");
    ui.connect("value_changed", self, "on_health_bar_changed");
}

fn on_health_changed(old: i32, new: i32) {
    print("Health changed from ", old, " to ", new);
}
```

**Verification**:

- [ ] `connect` method available on nodes
- [ ] Three arguments: signal_name (String), target (Node), method_name (String)
- [ ] Runtime establishes connection through Godot
- [ ] Connected methods called when signal emitted
- [ ] Self-connections work (signal and method on same node)
- [ ] Cross-node connections work
- [ ] Error on invalid method name

---

### 5. Signal Disconnection ✅

**Requirement**: Signal connections can be broken

**Example**:

```rust
fn _exit_tree() {
    disconnect("health_changed", self, "on_health_changed");
}
```

**Verification**:

- [ ] `disconnect` method available on nodes
- [ ] Same argument signature as `connect`
- [ ] Runtime breaks connection through Godot
- [ ] Disconnected methods no longer called
- [ ] Graceful handling if connection doesn't exist

---

### 6. Error Handling ✅

**Compile-Time Errors**:

- [ ] E301: Signal already defined
- [ ] E302: Signal not defined (emit_signal with undefined name)
- [ ] E303: Signal parameter count mismatch
- [ ] E304: Signal parameter type mismatch

**Runtime Errors**:

- [ ] E501: Signal connection failed (invalid method name)
- [ ] E502: Signal emission failed (Godot error)

---

## 🏗️ Technical Approach

### Component Changes

#### 1. Lexer (`crates/compiler/src/lexer.rs`)

**Change**: Add `signal` keyword

```rust
// In Token enum
pub enum Token {
    // ... existing tokens ...
    Signal,  // "signal" keyword
}

// In tokenize() match
"signal" => tokens.push(Token::Signal),
```

**Tests Required**:

- [ ] Tokenize signal keyword correctly
- [ ] Distinguish from identifier "signal"
- [ ] Case-sensitive ("Signal" should be identifier, not keyword)

---

#### 2. Parser (`crates/compiler/src/parser.rs`)

**Change**: Add signal declaration parsing

```rust
// New AST node
pub enum Statement {
    // ... existing variants ...
    Signal {
        name: String,
        parameters: Vec<(String, Type)>,  // (param_name, param_type)
        span: Span,
    },
}

// In parse_statement()
fn parse_statement(&mut self) -> Result<Statement, ParserError> {
    match self.current_token {
        // ... existing cases ...
        Token::Signal => self.parse_signal_declaration(),
    }
}

fn parse_signal_declaration(&mut self) -> Result<Statement, ParserError> {
    self.advance(); // Consume 'signal'
    
    let name = self.expect_identifier()?;
    self.expect(Token::LeftParen)?;
    
    let parameters = self.parse_parameter_list()?;  // Reuse from function parsing
    
    self.expect(Token::RightParen)?;
    self.expect(Token::Semicolon)?;
    
    Ok(Statement::Signal { name, parameters, span })
}
```

**Tests Required**:

- [ ] Parse signal with no parameters
- [ ] Parse signal with one parameter
- [ ] Parse signal with multiple parameters
- [ ] Error on missing semicolon
- [ ] Error on missing parentheses
- [ ] Error on invalid parameter syntax

---

#### 3. Type Checker (`crates/compiler/src/type_checker.rs`)

**Change**: Validate signal declarations and emissions

```rust
// In Environment or symbol table
pub struct Environment {
    // ... existing fields ...
    signals: HashMap<String, Vec<Type>>,  // signal_name -> parameter_types
}

impl TypeChecker {
    fn check_signal(&mut self, name: &str, parameters: &[(String, Type)]) -> Result<(), TypeCheckError> {
        // Check for duplicate signal
        if self.env.signals.contains_key(name) {
            return Err(TypeCheckError::DuplicateSignal(name.to_string()));
        }
        
        // Validate parameter types
        for (_, param_type) in parameters {
            self.validate_type(param_type)?;
        }
        
        // Register signal
        let param_types = parameters.iter().map(|(_, t)| t.clone()).collect();
        self.env.signals.insert(name.to_string(), param_types);
        
        Ok(())
    }
    
    fn check_emit_signal(&mut self, signal_name: &str, args: &[Expr]) -> Result<(), TypeCheckError> {
        // Look up signal
        let signal_params = self.env.signals.get(signal_name)
            .ok_or_else(|| TypeCheckError::UndefinedSignal(signal_name.to_string()))?;
        
        // Check argument count
        if args.len() != signal_params.len() {
            return Err(TypeCheckError::SignalParameterCountMismatch {
                signal: signal_name.to_string(),
                expected: signal_params.len(),
                actual: args.len(),
            });
        }
        
        // Check argument types
        for (arg, expected_type) in args.iter().zip(signal_params) {
            let arg_type = self.check_expr(arg)?;
            if !self.is_compatible(&arg_type, expected_type) {
                return Err(TypeCheckError::SignalParameterTypeMismatch {
                    signal: signal_name.to_string(),
                    expected: expected_type.clone(),
                    actual: arg_type,
                });
            }
        }
        
        Ok(())
    }
}
```

**Tests Required**:

- [ ] Accept valid signal declarations
- [ ] Error on duplicate signal names
- [ ] Error on undefined type in parameters
- [ ] Accept valid emit_signal calls
- [ ] Error on undefined signal in emit_signal
- [ ] Error on parameter count mismatch
- [ ] Error on parameter type mismatch
- [ ] Allow type coercion (i32 → f32)

---

#### 4. Runtime (`crates/runtime/src/runtime.rs`)

**Change**: Store signals and handle emission

```rust
pub struct Runtime {
    // ... existing fields ...
    signals: HashMap<String, Vec<Value>>,  // signal_name -> parameter_types (as Values for simplicity)
}

impl Runtime {
    pub fn register_signal(&mut self, name: String, param_types: Vec<Type>) {
        // Signal registration happens during initialization
        // Store parameter type information for validation
        self.signals.insert(name, param_types.iter().map(|_| Value::Nil).collect());
    }
    
    pub fn emit_signal(&mut self, name: &str, args: Vec<Value>) -> Result<(), RuntimeError> {
        // Validate signal exists
        if !self.signals.contains_key(name) {
            return Err(RuntimeError::UndefinedSignal(name.to_string()));
        }
        
        // Delegate to Godot binding for actual emission
        // Godot handles notification of connected slots
        self.godot_emit_signal(name, args)?;
        
        Ok(())
    }
}
```

**Tests Required**:

- [ ] Register signals successfully
- [ ] Emit signals with correct parameters
- [ ] Error on undefined signal
- [ ] Verify Godot binding called correctly

---

#### 5. Godot Binding (`crates/godot_bind/src/lib.rs`)

**Change**: Expose signals to Godot and implement connect/emit

```rust
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct FerrisScriptNode {
    #[base]
    base: Base<Node>,
    
    // Signals defined in FerrisScript
    signals: Vec<SignalDefinition>,
}

struct SignalDefinition {
    name: String,
    parameters: Vec<(String, VariantType)>,
}

#[godot_api]
impl FerrisScriptNode {
    // Register signals with Godot during class initialization
    #[signal]
    fn health_changed(old: i32, new: i32);
    
    // Dynamic signal registration (called from FerrisScript runtime)
    fn register_signal(&mut self, name: &str, params: Vec<(String, VariantType)>) {
        self.signals.push(SignalDefinition {
            name: name.to_string(),
            parameters: params,
        });
        
        // Register with Godot's signal system
        // Note: This may require godot-rust API investigation
    }
    
    // Emit signal (called from FerrisScript runtime)
    fn emit_ferris_signal(&mut self, name: &str, args: &[Variant]) {
        // Call Godot's emit_signal
        self.base.emit_signal(name.into(), args);
    }
}
```

**Challenges**:

- Godot signals typically declared with `#[signal]` attribute (compile-time)
- Dynamic signal registration may require workaround
- Need to investigate godot-rust 0.4 API for dynamic signals

**Research Required**:

- [ ] Can godot-rust 0.4 register signals dynamically?
- [ ] Alternative: Generate Rust code with #[signal] attributes from FerrisScript?
- [ ] How to pass typed parameters through Variant boundary?

**Tests Required**:

- [ ] Signal visible in Godot Inspector
- [ ] Signal connection from editor works
- [ ] Signal emission triggers connected methods
- [ ] Parameters passed correctly

---

## 🧪 Test Coverage Requirements

### Unit Tests (Target: 80%+)

**Lexer Tests** (`crates/compiler/src/lexer/tests.rs`):

- [ ] `test_tokenize_signal_keyword`
- [ ] `test_signal_vs_identifier_case_sensitivity`

**Parser Tests** (`crates/compiler/src/parser/tests.rs`):

- [ ] `test_parse_signal_no_params`
- [ ] `test_parse_signal_one_param`
- [ ] `test_parse_signal_multiple_params`
- [ ] `test_parse_signal_missing_semicolon`
- [ ] `test_parse_signal_missing_parens`
- [ ] `test_parse_signal_invalid_param_syntax`

**Type Checker Tests** (`crates/compiler/src/type_checker/tests.rs`):

- [ ] `test_signal_declaration_valid`
- [ ] `test_signal_duplicate_name_error`
- [ ] `test_signal_undefined_type_error`
- [ ] `test_emit_signal_valid`
- [ ] `test_emit_signal_undefined_error`
- [ ] `test_emit_signal_param_count_mismatch`
- [ ] `test_emit_signal_param_type_mismatch`
- [ ] `test_emit_signal_type_coercion`

**Runtime Tests** (`crates/runtime/src/tests.rs`):

- [ ] `test_register_signal`
- [ ] `test_emit_signal_valid`
- [ ] `test_emit_signal_undefined_error`

---

### Integration Tests (Target: Full Coverage)

**Compilation Tests** (`crates/compiler/tests/integration_tests.rs`):

- [ ] `test_compile_signal_declaration`
- [ ] `test_compile_signal_emission`
- [ ] `test_compile_signal_with_godot_callback`

**Godot Binding Tests** (Manual - requires Godot runtime):

- [ ] Signal appears in Godot Inspector
- [ ] Signal connection from editor
- [ ] Signal emission triggers method
- [ ] Parameters passed correctly
- [ ] Multiple connections work
- [ ] Disconnection works

---

## 🚧 Implementation Plan

### Step 1: Lexer & Parser (Day 1)

1. Add `signal` keyword to lexer
2. Implement `parse_signal_declaration()` in parser
3. Add `Statement::Signal` AST node
4. Write lexer + parser unit tests
5. Verify tests pass

**Acceptance**: Parser can parse signal declarations into AST

---

### Step 2: Type Checker (Day 2)

1. Add signal storage to Environment
2. Implement `check_signal()` method
3. Implement `check_emit_signal()` method
4. Add error types (E301-E304)
5. Write type checker unit tests
6. Verify tests pass

**Acceptance**: Type checker validates signal declarations and emissions

---

### Step 3: Runtime Basics (Day 3)

1. Add signal storage to Runtime
2. Implement `register_signal()` method
3. Implement `emit_signal()` stub (no Godot yet)
4. Add `emit_signal` built-in function
5. Write runtime unit tests
6. Verify tests pass

**Acceptance**: Runtime can store and "emit" signals (without Godot integration)

---

### Step 4: Godot Binding Research (Day 4)

1. Research godot-rust 0.4 signal API
2. Investigate dynamic signal registration
3. Test signal emission with hardcoded signal
4. Prototype parameter passing
5. Document findings

**Acceptance**: Clear understanding of godot-rust signal integration approach

---

### Step 5: Godot Binding Implementation (Day 5)

1. Implement signal registration in `FerrisScriptNode`
2. Implement signal emission in `emit_ferris_signal()`
3. Connect runtime to Godot binding
4. Test in minimal Godot project
5. Verify signal visible in Inspector

**Acceptance**: Signals defined in FerrisScript appear in Godot Inspector

---

### Step 6: Connection & Testing (Day 6)

1. Implement `connect()` method
2. Implement `disconnect()` method
3. Test editor-based connections
4. Test code-based connections
5. Write comprehensive integration tests

**Acceptance**: Signals can be connected and disconnected successfully

---

### Step 7: Polish & Documentation (Day 7)

1. Fix bugs from testing
2. Add error code documentation
3. Update ERROR_CODES.md
4. Create example scripts
5. Update CHANGELOG.md
6. Final quality gate checks (clippy, fmt, links)

**Acceptance**: Ready for PR

---

## 🎯 Quality Gates

### Before PR

- [ ] All unit tests passing (`cargo test --workspace`)
- [ ] All integration tests passing
- [ ] Strict clippy passing (`cargo clippy --workspace --all-targets --all-features -- -D warnings`)
- [ ] Code formatted (`cargo fmt --all -- --check`)
- [ ] Documentation linting passing (`npm run docs:lint`)
- [ ] All markdown links validated
- [ ] Phase 1 acceptance criteria verified

### PR Requirements

- [ ] PR description includes:
  - Signal syntax examples
  - Test coverage summary
  - Manual testing steps
  - Godot integration verification
- [ ] At least 1 reviewer approval
- [ ] CI passing (all workflows green)

---

## 🔗 Dependencies

**No blocking dependencies** - Phase 1 can start immediately

**Enables**:

- Phase 2: Callbacks may use signals for events
- Phase 5: Property exports may emit change signals

---

## 📚 References

- Godot Signal Documentation: https://docs.godotengine.org/en/stable/getting_started/step_by_step/signals.html
- godot-rust Documentation: https://godot-rust.github.io/book/ (v0.4 signals research needed)
- GDScript Signal Syntax: https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html#signals
- v0.0.3 Error Recovery Patterns: `docs/archive/v0.0.3/LEARNINGS.md`
- Quality Gates: `docs/LEARNINGS.md` (v0.0.3 section)

---

## 📝 Notes

### Design Decisions

**Signal Storage**: Store in Environment (type checker) and Runtime separately

- Type checker: For compile-time validation
- Runtime: For emission tracking (if needed for debugging)

**Parameter Passing**: Use existing type coercion system

- i32 → f32 allowed (existing behavior)
- No implicit conversions for other types

**Connection Syntax**: Match Godot's `connect(signal, target, method)` pattern

- Familiar to Godot developers
- Simple API without complexity

### Known Limitations

**Dynamic Signal Registration Challenge**:

- Godot signals typically compile-time with `#[signal]` attribute
- May need workaround for FerrisScript's runtime signal definition
- Investigate: Code generation vs dynamic registration

**Signal Visibility in Editor**:

- Requires full Godot integration testing
- May need Godot project updates for testing
- Manual verification required

### Future Enhancements (Post-Phase 1)

- Signal groups (emit to multiple listeners at once)
- Signal flags (one-shot, deferred)
- Signal introspection (list all signals on a node)
- Signal parameter validation helpers

---

**Status**: Ready to begin  
**Next Action**: Create feature branch and start Step 1 (Lexer & Parser)
