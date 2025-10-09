# Phase 2: Additional Callbacks - Preparation

**Date**: October 8, 2025  
**Phase**: 2 of 5  
**Status**: 📋 **PLANNING** (Ready to Start After Phase 1 Merge)  
**Branch**: `feature/v0.0.4-callbacks` (to be created)  
**Estimated Effort**: 3-4 days

---

## 🎯 Overview

**Goal**: Implement additional Godot lifecycle callbacks to enable input handling, physics processing, and scene tree events.

**Strategic Importance**: These callbacks are essential for interactive game development:
- `_input()` - Handle player input (keyboard, mouse, gamepad)
- `_physics_process()` - Fixed timestep physics and movement
- `_enter_tree()` - Node initialization when added to scene
- `_exit_tree()` - Cleanup when removed from scene

**Dependencies**: None (Phase 1 complete, can proceed independently)

---

## 📋 Scope

### Callbacks to Implement

#### 1. `_input(event: InputEvent)` 🎮

**Purpose**: Handle user input events

**Example Usage**:
```rust
fn _input(event: InputEvent) {
    if event.is_action_pressed("jump") {
        velocity.y = -300.0;
    }
    if event.is_action_pressed("shoot") {
        spawn_bullet();
    }
}
```

**Implementation Notes**:
- Requires InputEvent type (simplified version)
- `is_action_pressed(action: String) -> bool` method
- `is_action_released(action: String) -> bool` method
- Input actions defined in Godot project settings

**Estimated Effort**: 1 day

---

#### 2. `_physics_process(delta: f32)` ⏱️

**Purpose**: Fixed timestep updates for physics and movement

**Example Usage**:
```rust
fn _physics_process(delta: f32) {
    // Physics calculations
    self.position.x += velocity.x * delta;
    self.position.y += velocity.y * delta;
    
    // Apply gravity
    velocity.y += 980.0 * delta;
    
    // Check collisions
    if check_ground_collision() {
        velocity.y = 0.0;
    }
}
```

**Implementation Notes**:
- Called at fixed 60 FPS by default
- Already have `_process(delta)` pattern to follow
- No new types required

**Estimated Effort**: 0.5 days (straightforward, similar to `_process`)

---

#### 3. `_enter_tree()` 🌳

**Purpose**: Called when node enters scene tree

**Example Usage**:
```rust
fn _enter_tree() {
    print("Node entered the scene tree");
    // Initialize connections
    // Set up references to other nodes
}
```

**Implementation Notes**:
- No parameters
- Called before `_ready()`
- Useful for early initialization

**Estimated Effort**: 0.5 days

---

#### 4. `_exit_tree()` 🚪

**Purpose**: Called when node exits scene tree

**Example Usage**:
```rust
fn _exit_tree() {
    print("Node exiting the scene tree");
    // Cleanup resources
    // Disconnect signals
    // Free allocated objects
}
```

**Implementation Notes**:
- No parameters
- Called after node removed from tree
- Useful for cleanup

**Estimated Effort**: 0.5 days

---

## 🏗️ Technical Approach

### Component Changes

#### 1. InputEvent Type (`crates/runtime/src/value.rs`)

**Option A: Simplified InputEvent**
```rust
pub enum Value {
    // ... existing variants ...
    InputEvent {
        event_type: String,  // "key", "mouse_button", "mouse_motion", etc.
        action: Option<String>,  // For action checks
    },
}
```

**Option B: Opaque Handle**
```rust
pub enum Value {
    // ... existing variants ...
    InputEvent(InputEventHandle),  // Wraps Godot's InputEvent
}

pub struct InputEventHandle {
    godot_event: Box<dyn Any>,  // Opaque Godot event
}
```

**Recommendation**: Option B (opaque handle)
- Avoids reimplementing Godot's complex InputEvent hierarchy
- Delegates to Godot methods via FFI
- Simpler to maintain

---

#### 2. Godot Binding (`crates/godot_bind/src/lib.rs`)

**Add New Lifecycle Methods**:
```rust
#[godot_api]
impl INode2D for FerrisScriptNode {
    // ... existing methods ...
    
    fn input(&mut self, event: Gd<InputEvent>) {
        if let Some(env) = &mut self.environment {
            // Convert Godot InputEvent to FerrisScript Value
            let event_value = convert_input_event(event);
            
            // Call FerrisScript _input function if defined
            if env.has_function("_input") {
                let _ = env.call_function("_input", vec![event_value]);
            }
        }
    }
    
    fn physics_process(&mut self, delta: f64) {
        if let Some(env) = &mut self.environment {
            if env.has_function("_physics_process") {
                let _ = env.call_function("_physics_process", vec![Value::Float(delta as f32)]);
            }
        }
    }
    
    fn enter_tree(&mut self) {
        if let Some(env) = &mut self.environment {
            if env.has_function("_enter_tree") {
                let _ = env.call_function("_enter_tree", vec![]);
            }
        }
    }
    
    fn exit_tree(&mut self) {
        if let Some(env) = &mut self.environment {
            if env.has_function("_exit_tree") {
                let _ = env.call_function("_exit_tree", vec![]);
            }
        }
    }
}
```

---

#### 3. Type Checker (`crates/compiler/src/type_checker.rs`)

**Add Lifecycle Function Validation**:
```rust
fn check_lifecycle_function(&mut self, name: &str, params: &[Parameter]) -> Result<(), TypeCheckError> {
    match name {
        "_input" => {
            if params.len() != 1 {
                return Err(TypeCheckError::WrongParameterCount { 
                    function: name.to_string(), 
                    expected: 1, 
                    actual: params.len() 
                });
            }
            if params[0].param_type != Type::InputEvent {
                return Err(TypeCheckError::WrongParameterType {
                    function: name.to_string(),
                    parameter: params[0].name.clone(),
                    expected: Type::InputEvent,
                    actual: params[0].param_type.clone(),
                });
            }
        }
        "_physics_process" => {
            if params.len() != 1 {
                return Err(TypeCheckError::WrongParameterCount { 
                    function: name.to_string(), 
                    expected: 1, 
                    actual: params.len() 
                });
            }
            if params[0].param_type != Type::Float {
                return Err(TypeCheckError::WrongParameterType {
                    function: name.to_string(),
                    parameter: params[0].name.clone(),
                    expected: Type::Float,
                    actual: params[0].param_type.clone(),
                });
            }
        }
        "_enter_tree" | "_exit_tree" => {
            if !params.is_empty() {
                return Err(TypeCheckError::WrongParameterCount { 
                    function: name.to_string(), 
                    expected: 0, 
                    actual: params.len() 
                });
            }
        }
        _ => {}
    }
    Ok(())
}
```

---

## 🧪 Test Coverage Plan

### Unit Tests

**Type Checker Tests** (`crates/compiler/src/type_checker/tests.rs`):
- [ ] `test_input_function_valid`
- [ ] `test_input_function_wrong_param_count`
- [ ] `test_input_function_wrong_param_type`
- [ ] `test_physics_process_function_valid`
- [ ] `test_physics_process_wrong_param_count`
- [ ] `test_physics_process_wrong_param_type`
- [ ] `test_enter_tree_function_valid`
- [ ] `test_enter_tree_with_params_error`
- [ ] `test_exit_tree_function_valid`
- [ ] `test_exit_tree_with_params_error`

**Runtime Tests** (`crates/runtime/src/tests.rs`):
- [ ] `test_call_input_function`
- [ ] `test_call_physics_process_function`
- [ ] `test_call_enter_tree_function`
- [ ] `test_call_exit_tree_function`

**Target**: 14+ new tests

---

### Integration Tests

**Manual Godot Testing**:
1. Create test scene with FerrisScript node
2. Implement all 4 lifecycle callbacks
3. Verify `_input()` responds to keyboard input
4. Verify `_physics_process()` called 60 times per second
5. Verify `_enter_tree()` called on scene start
6. Verify `_exit_tree()` called on scene end

---

## 📚 Documentation Plan

### Code Documentation

**Error Codes** (`docs/ERROR_CODES.md`):
- [ ] E305: Invalid Lifecycle Function Signature
- [ ] E306: Lifecycle Function Wrong Parameter Count
- [ ] E307: Lifecycle Function Wrong Parameter Type

### User Documentation

**Example File** (`examples/callbacks.ferris`):
```rust
// Example demonstrating all lifecycle callbacks

fn _ready() {
    print("Node is ready!");
}

fn _enter_tree() {
    print("Node entered the tree");
}

fn _exit_tree() {
    print("Node exited the tree");
}

fn _process(delta: f32) {
    print("Frame update: ", delta);
}

fn _physics_process(delta: f32) {
    print("Physics update: ", delta);
}

fn _input(event: InputEvent) {
    if event.is_action_pressed("jump") {
        print("Jump pressed!");
    }
}
```

**CHANGELOG.md**:
- [ ] Add Phase 2 entry for v0.0.4

---

## 🎯 Acceptance Criteria

### 1. `_input()` Callback ✅

**Verification**:
- [ ] Type checker validates function signature
- [ ] InputEvent value created from Godot event
- [ ] Function called when input occurs in Godot
- [ ] `is_action_pressed()` method works
- [ ] `is_action_released()` method works

---

### 2. `_physics_process()` Callback ✅

**Verification**:
- [ ] Type checker validates function signature
- [ ] Function called at fixed 60 FPS
- [ ] Delta parameter accurate (approximately 0.0166s)
- [ ] Can modify position based on physics

---

### 3. `_enter_tree()` Callback ✅

**Verification**:
- [ ] Type checker validates function signature (no params)
- [ ] Function called when node enters scene tree
- [ ] Called before `_ready()`
- [ ] Can access node properties

---

### 4. `_exit_tree()` Callback ✅

**Verification**:
- [ ] Type checker validates function signature (no params)
- [ ] Function called when node exits scene tree
- [ ] Called after node removed from parent
- [ ] Can perform cleanup operations

---

## 🚧 Implementation Plan

### Step 1: InputEvent Type (Day 1)

1. Add InputEvent variant to Value enum
2. Implement opaque handle wrapper
3. Add `is_action_pressed()` and `is_action_released()` methods
4. Write unit tests
5. Verify tests pass

**Acceptance**: InputEvent type functional in runtime

---

### Step 2: Type Checker Validation (Day 1)

1. Add lifecycle function validation
2. Add error codes (E305-E307)
3. Write type checker unit tests
4. Verify tests pass

**Acceptance**: Type checker validates all 4 lifecycle functions

---

### Step 3: Godot Binding Implementation (Day 2)

1. Implement `input()` method
2. Implement `physics_process()` method
3. Implement `enter_tree()` method
4. Implement `exit_tree()` method
5. Add InputEvent conversion helper
6. Test in minimal Godot project

**Acceptance**: All callbacks functional in Godot

---

### Step 4: Testing & Documentation (Day 3)

1. Write comprehensive unit tests
2. Create example script (`callbacks.ferris`)
3. Update ERROR_CODES.md
4. Update CHANGELOG.md
5. Manual Godot testing
6. Quality gate checks

**Acceptance**: Ready for PR

---

## 🔗 Dependencies

**No Blocking Dependencies**:
- Phase 1 complete (but not required for Phase 2)
- Can start immediately after Phase 1 PR created

**Optional Dependencies**:
- Phase 3 (Node Queries) - Could use `get_node()` in examples, but not required

---

## 📝 Notes

### Design Decisions

**InputEvent as Opaque Handle**:
- Avoids reimplementing complex Godot type hierarchy
- Simpler to maintain
- Delegates to Godot's existing implementation
- Trade-off: Less transparent than native type

**Lifecycle Function Naming**:
- Use Godot's exact naming convention (`_input`, `_physics_process`, etc.)
- Familiar to Godot developers
- Clear documentation link to Godot docs

### Known Challenges

**InputEvent Complexity**:
- Godot has 10+ InputEvent subclasses
- Each subclass has unique properties
- Solution: Start with action checks only, expand later

**Physics Process Timing**:
- Must ensure called at correct frequency
- Godot handles this, but verify in testing

---

## 🚀 Ready to Start

**Prerequisites Met**:
- ✅ Phase 1 implementation complete
- ✅ Clear scope and acceptance criteria
- ✅ Technical approach defined
- ✅ Test plan prepared
- ✅ Documentation plan prepared

**Next Action**: 
1. Wait for Phase 1 PR review/merge
2. Create `feature/v0.0.4-callbacks` branch
3. Begin Step 1 (InputEvent Type)

---

**Status**: 📋 Ready for implementation after Phase 1 merge
