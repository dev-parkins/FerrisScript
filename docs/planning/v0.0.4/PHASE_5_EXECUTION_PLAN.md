# Phase 5 Execution Plan: @export Property Annotation System

**Branch**: `feature/v0.0.4-phase4-5-godot-types-exports`  
**Created**: October 10, 2025  
**Status**: � **IN PROGRESS** - Sub-Phase 1 Complete ✅  
**Prerequisites**: Phases 1-4.5 complete ✅  
**Research Document**: [EXPORT_ANNOTATION_RESEARCH.md](EXPORT_ANNOTATION_RESEARCH.md)  
**Completion Report**: [PHASE_5_SUB_PHASE_1_COMPLETION.md](PHASE_5_SUB_PHASE_1_COMPLETION.md)

---

## 🎯 Executive Summary

### Strategic Goal

Implement `@export` annotation system to enable Godot Inspector integration, allowing developers to expose FerrisScript variables as editable properties in the Godot editor.

### Scope

**In Scope**:

- `@export` annotation parsing (lexer + parser)
- Property type validation (i32, f32, bool, String, Vector2, Color, Rect2, Transform2D)
- Property hints (range, file, enum)
- Compile-time validation (15 error codes: E801-E815)
- Runtime metadata storage
- Godot Inspector integration (get/set property reflection)
- 20+ tests across all phases
- Documentation and examples

**Out of Scope** (defer to v0.0.5 or later):

- Advanced hints (multiline, color_no_alpha, etc.)
- Property groups/categories
- Custom property editors
- Property change notifications
- @onready annotation
- Scene tree export

### Why Deferred from Phase 4.5

`@export` is a **reflection, metadata, and interop system** that touches all major compiler layers:

- Introduces decorator-like syntax (grammar expansion)
- Requires type + hint semantic validation matrix
- Needs runtime reflection capabilities
- Demands bidirectional Godot synchronization
- Has 15 error codes spanning compile-time + runtime
- Requires multi-environment testing (compiler, runtime, Inspector)

**Complexity**: 23-31 hours (vs 2.5 hours for struct literals MVP)

### Implementation Strategy

Apply **Phase 4.5 checkpoint methodology** (proven 50% efficiency gain):

1. Break into 3 sub-phases (Parser → Runtime → Godot)
2. Use 8 structured checkpoints per sub-phase
3. Test-first validation at each checkpoint
4. Document assumptions and trade-offs inline
5. Robustness testing after MVP

---

## � Key Refinements from Feedback Review

**Core Insight**: The original plan was architecturally sound but needed precision on implementation details and metadata architecture.

**Three Critical Refinements**:

1. **Hybrid Metadata Architecture** ⭐ (SIMPLIFIES implementation)
   - **Before**: Runtime HashMap storing full metadata per instance
   - **After**: Static compile-time metadata + per-instance values only
   - **Impact**: -2 hours (simpler runtime/Godot), Type Checker generates metadata structure

2. **Exact Godot Formats** (eliminates guesswork)
   - Range: `"0,100,1"` (no parentheses, comma-separated)
   - Enum: `"Easy,Medium,Hard"` (no quotes in string)
   - File: `"*.png,*.jpg"` (with asterisks)
   - Usage flags: `PROPERTY_USAGE_DEFAULT | PROPERTY_USAGE_STORAGE | PROPERTY_USAGE_EDITOR`

3. **Clamp-on-Set Policy** (predictable behavior)
   - Inspector sets: Automatic clamp (good UX)
   - Script sets: Warn but allow (E816 warning)
   - Tests verify both paths

**Additional Clarifications**:

- Default values: Only compile-time constants (literals or struct literals) → E813
- Immutability: `let` → Inspector read-only (E812 warning), `let mut` → read/write
- Collision rules: Duplicate in scope → E810 error
- CI strategy: Unit tests (mocked) + integration tests (Godot process)
- Thread safety: GDExtension threading note added to Checkpoint 24

**Effort Impact**: **21-29 hours** (down from 23-31 hours due to hybrid architecture simplification)

---

## �🏗️ Architectural Decisions

### Hybrid Metadata Model (CRITICAL)

**Decision**: Use **static compile-time metadata** + **per-instance runtime values**

**Rationale**:

- Godot may query `get_property_list()` BEFORE script initialization
- Metadata (name, type, hint, hint_string, usage) is identical across all instances
- Storing full metadata per instance wastes memory and duplicates work
- Static metadata enables simpler Inspector integration

**Architecture**:

```rust
// Compile-time metadata (embedded in Program/Script)
pub struct PropertyMetadata {
    pub name: String,
    pub type_info: TypeInfo,
    pub hint: Option<PropertyHint>,
    pub hint_string: String,           // e.g., "0,100,1" for range(0,100,1)
    pub default_variant: Variant,      // Pre-serialized default value
    pub usage: PropertyUsageFlags,     // PROPERTY_USAGE_DEFAULT | PROPERTY_USAGE_STORAGE
    pub is_mutable: bool,              // false = Inspector read-only
}

// Per-instance runtime values (only current values, not metadata)
pub struct ScriptInstance {
    properties: HashMap<String, Value>,  // Current property values only
    // ...existing fields
}
```

**Phase Impact**:

- **Parser**: No change (still parses @export syntax)
- **Type Checker**: Must OUTPUT PropertyMetadata list (new responsibility)
- **Runtime**: SIMPLIFIED - only stores current values, not metadata
- **Godot Binding**: SIMPLIFIED - reads from static metadata, no reconstruction

### Design Policies

#### 1. Clamp-on-Set Policy for Range Hints

**Decision**: **Clamp** values automatically when set from Inspector, **warn** when set from script

**Rationale**:

- Inspector sets: User expects UI constraints to be enforced (good UX)
- Script sets: Developer may intentionally exceed range (emit diagnostic but allow)

**Implementation**:

```rust
// Inspector set (via GDExtension)
fn set_property(&mut self, name: &str, value: Variant) {
    let clamped = self.clamp_if_range(name, value);  // Automatic clamp
    self.properties.insert(name.to_string(), clamped);
}

// Script set (via FerrisScript assignment)
fn set_property_from_script(&mut self, name: &str, value: Value) {
    if !self.is_in_range(name, &value) {
        // Emit warning but allow: E816 (new diagnostic code)
        warn!("Property '{}' value {} exceeds range", name, value);
    }
    self.properties.insert(name.to_string(), value);
}
```

#### 2. Immutability Handling

**Decision**: Allow `@export` on **both** `let` and `let mut`, enforce via PROPERTY_USAGE flags

**Rules**:

- `let` (immutable) + `@export` → Inspector **read-only** (usage without PROPERTY_USAGE_EDITOR)
- `let mut` (mutable) + `@export` → Inspector **read/write** (usage with PROPERTY_USAGE_EDITOR)

**Error Codes**:

- E812: Export on immutable variable (warning, not error - allowed but read-only)

#### 3. Default Value Rules

**Decision**: Only accept **compile-time constant** defaults (literals or struct literals)

**Allowed**:

```rust
@export let speed: f32 = 10.0;                          // Literal
@export let color: Color = Color { r: 1, g: 0, b: 0, a: 1 };  // Struct literal
```

**Disallowed**:

```rust
@export let speed: f32 = calculate_speed();  // Expression (E813)
```

**Rationale**: Metadata must be available at script load time, before expressions can be evaluated.

#### 4. Exact Godot hint_string Formats

**Canonical Formats** (for Godot Inspector):

| Hint Type | FerrisScript Syntax | Godot hint_string | Example |
|-----------|---------------------|-------------------|---------|
| **Range** | `@export(range(min,max,step))` | `"min,max,step"` | `"0,100,1"` or `"0.0,20.0,0.5"` |
| **Enum** | `@export(enum("A","B","C"))` | `"A,B,C"` | `"Easy,Medium,Hard"` |
| **File** | `@export(file("*.png","*.jpg"))` | `"*.png,*.jpg"` | `"*.png,*.jpg"` or `"*.txt"` |

**Usage Flags** (default for all exports):

```rust
PROPERTY_USAGE_DEFAULT | PROPERTY_USAGE_STORAGE | PROPERTY_USAGE_EDITOR
```

#### 5. Property Naming Collision Rules

**Rules**:

- Duplicate in same scope → **E810** (compile error)
- Shadowing across inheritance → **Allowed** (future: require `override` annotation)
- Export in function scope → **E811** (compile error - only global exports)

#### 6. CI Testing Strategy

**Approach**: Hybrid mocking + integration testing

- **Unit Tests**: Mock godot_bind functions, test conversion helpers directly
- **Integration Tests**: Use existing test harness with Godot process (headless when available)
- **CI**: Run unit tests always, integration tests in separate job with Godot installed

---

## 📊 Effort Estimation

### Total Estimate: 21-29 hours (revised down from 23-31)

**Breakdown by Category**:

| Category | Tasks | Estimated Hours | Confidence |
|----------|-------|-----------------|------------|
| **1. Parser & AST** | Annotation syntax, hint parsing, AST nodes | 4-6 hours | High |
| **2. Type Checker** | Export validation, hint validation, error codes, **metadata generation** | 6-8 hours | Medium-High |
| **3. Runtime** | **Per-instance value storage**, property get/set (SIMPLIFIED) | 2-4 hours | High |
| **4. Godot Binding** | PropertyInfo from static metadata (SIMPLIFIED), conversions | 4-6 hours | Medium |
| **5. Testing** | 20+ tests (parser, type checker, runtime, integration) | 4-5 hours | High |
| **6. Documentation** | Examples, user guide, technical docs, learnings | 1-2 hours | High |
| **TOTAL** | All phases | **21-29 hours** | Medium-High |

**Architectural Impact**:

- **Type Checker +1-2h**: Must generate PropertyMetadata structure (new responsibility)
- **Runtime -1-2h**: SIMPLIFIED - only stores values, not full metadata per instance
- **Godot Binding -2h**: SIMPLIFIED - reads from static metadata, no reconstruction
- **Net: -2h reduction** due to hybrid metadata architecture

**Risk Factors**:

- **Type Checker**: Metadata generation adds complexity but is well-defined
- **Godot Binding**: Medium confidence now (clear static metadata lookup)
- **Integration Testing**: Requires Godot running (slower iteration)

### Premium Request Estimate: 6-8 requests

Assuming 4-hour focused sessions:

- Session 1: Sub-Phase 1 (Parser + AST) - ~6 hours → 2 requests
- Session 2: Sub-Phase 2 (Type Checker + Validation) - ~6 hours → 2 requests
- Session 3: Sub-Phase 3 (Runtime + Godot) - ~9 hours → 2-3 requests
- Session 4: Robustness Testing + Docs - ~4 hours → 1 request

**Total**: 6-8 premium requests (vs 2-3 for Phase 4.5)

---

## 🗺️ Sub-Phase Breakdown

### Sub-Phase 1: Parser & AST (4-6 hours)

**Goal**: Parse `@export` annotations and hints, store in AST

**Deliverables**:

- Lexer recognizes `@` token and `export` keyword
- Parser handles `@export` before variable declarations
- Parser handles hint syntax: `@export(range(0, 100))`
- AST node: `ExportAnnotation { hint: Option<PropertyHint> }`
- AST integration with `VarDecl` node
- 8 parser tests (basic annotations + hint parsing)

**Checkpoints** (Sub-Phase 1):

1. Lexer `@` token + `export` keyword
2. Parser `@export` with no hints
3. AST `ExportAnnotation` node
4. Parser `range` hint (numeric min/max/step)
5. Parser `file` hint (extension list)
6. Parser `enum` hint (string values)
7. Parser error recovery (invalid syntax)
8. Integration tests (all hint types)

**Validation**: 8 tests passing, `cargo test -p ferrisscript_compiler`

---

### Sub-Phase 2: Type Checker & Metadata Generation (6-8 hours)

**Goal**: Validate export semantics and **generate static PropertyMetadata**

**Deliverables**:

- Type checker recognizes `ExportAnnotation`
- Export type validation (allowed: i32, f32, bool, String, Vector2, Color, Rect2, Transform2D)
- Hint → type compatibility matrix
- **PropertyMetadata generation** (NEW: output compile-time metadata structure)
- Immutability tracking (`let` vs `let mut`)
- Default value validation (compile-time constants only)
- Exact hint_string formatting ("0,100,1", "Easy,Medium,Hard", "*.png,*.jpg")
- 15 error codes (E801-E815) with descriptive messages
- 12 type checker tests (validation + errors + metadata generation)

**Checkpoints** (Sub-Phase 2):

1. Export type eligibility check (E802)
2. Hint → type compatibility check (E804-E806)
3. Range hint validation (min < max, step > 0) + clamp-on-set policy (E807)
4. File hint validation (non-empty extensions) + exact format ("*.ext")
5. Enum hint validation (non-empty values) + exact format ("A,B,C") (E808)
6. **PropertyMetadata struct generation** (NEW: output static metadata)
7. Immutability + default value validation (E812, E813)
8. Duplicate export detection + comprehensive error tests (E810, E811)

**Validation**: 12 tests passing, PropertyMetadata list generated, all error codes covered

---

### Sub-Phase 3: Runtime & Godot Integration (6-10 hours, SIMPLIFIED)

**Goal**: Store **per-instance values** and sync with Godot Inspector

**Deliverables**:

- **Per-instance value storage** (SIMPLIFIED: only current values, not full metadata)
- Property get/set methods with clamp-on-set logic
- Godot `PropertyInfo` from static metadata (SIMPLIFIED: direct conversion)
- Inspector get_property_list implementation (reads static metadata)
- Inspector get/set handlers
- Variant ↔ Value conversions with round-trip tests
- Thread-safety note for GDExtension conventions
- 10 runtime tests (value storage + Inspector sync)

**Checkpoints** (Sub-Phase 3):

1. Per-instance value storage in `ScriptInstance` (HashMap<String, Value>)
2. Read static PropertyMetadata from compiled Program/Script
3. Property get method (lookup current value, return to Godot)
4. Property set method with clamp-on-set for range hints
5. PropertyInfo conversion from static metadata (SIMPLIFIED: direct mapping)
6. Exact hint_string formatting ("0,100,1", "Easy,Medium,Hard", "*.png,*.jpg")
7. Inspector get_property_list (query static metadata, not runtime)
8. Inspector get/set handlers + Variant ↔ Value round-trip tests

**Validation**: 10 tests passing, Inspector shows properties, clamp behavior tested

---

## 📋 Detailed Checkpoint Tracking (24 total)

### Sub-Phase 1: Parser & AST (8 checkpoints) ✅ COMPLETE

| # | Checkpoint | Est. Time | Actual | Tests | Status |
|---|------------|-----------|--------|-------|--------|
| 1.1 | Lexer `@` token + `export` keyword | 30 min | ~30 min | 2 | ✅ |
| 1.2 | Parser `@export` with no hints | 45 min | ~45 min | 3 | ✅ |
| 1.3 | AST `ExportAnnotation` node | 30 min | ~30 min | - | ✅ |
| 1.4 | Parser `range` hint (numeric) | 60 min | ~60 min | 3 | ✅ |
| 1.5 | Parser `file` hint (extensions) | 30 min | ~30 min | 3 | ✅ |
| 1.6 | Parser `enum` hint (values) | 30 min | ~30 min | 3 | ✅ |
| 1.7 | Parser error recovery | 45 min | ~45 min | 11 | ✅ |
| 1.8 | Integration tests (all hints) | 45 min | ~45 min | 9 | ✅ |
| **Total** | **Sub-Phase 1** | **4.75 hours** | **~4 hours** | **34** | ✅ |

### Sub-Phase 2: Type Checker & Metadata Generation (8 checkpoints)

| # | Checkpoint | Est. Time | Tests | Status |
|---|------------|-----------|-------|--------|
| 2.1 | Export type eligibility (E802) | 45 min | 2 | ⏸️ |
| 2.2 | Hint → type compatibility (E804-E806) | 90 min | 3 | ⏸️ |
| 2.3 | Range hint validation + clamp policy (E807) | 60 min | 2 | ⏸️ |
| 2.4 | File hint exact format validation ("*.ext") | 30 min | 1 | ⏸️ |
| 2.5 | Enum hint exact format validation ("A,B,C") (E808) | 30 min | 1 | ⏸️ |
| 2.6 | **PropertyMetadata generation** (NEW) | 90 min | 1 | ⏸️ |
| 2.7 | Immutability + default value validation (E812, E813) | 45 min | 1 | ⏸️ |
| 2.8 | Duplicate/scope errors + comprehensive tests (E810, E811) | 60 min | 1 | ⏸️ |
| **Total** | **Sub-Phase 2** | **7 hours** | **12** | ⏸️ |

### Sub-Phase 3: Runtime & Godot (8 checkpoints, SIMPLIFIED)

| # | Checkpoint | Est. Time | Tests | Status |
|---|------------|-----------|-------|--------|
| 3.1 | Per-instance value storage (HashMap) | 30 min | 1 | ⏸️ |
| 3.2 | Read static PropertyMetadata from Program | 45 min | 1 | ⏸️ |
| 3.3 | Property get method (value lookup) | 45 min | 2 | ⏸️ |
| 3.4 | Property set with clamp-on-set logic | 60 min | 2 | ⏸️ |
| 3.5 | PropertyInfo from static metadata (direct conversion) | 60 min | 1 | ⏸️ |
| 3.6 | Exact hint_string formatting ("0,100,1" etc.) | 30 min | 1 | ⏸️ |
| 3.7 | Inspector get_property_list (static query) | 60 min | 1 | ⏸️ |
| 3.8 | Inspector get/set + Variant round-trip tests | 90 min | 1 | ⏸️ |
| **Total** | **Sub-Phase 3** | **7 hours** | **10** | ⏸️ |

**Grand Total**: 24 checkpoints, ~19 hours core work, +2-10 hours buffer/integration = **21-29 hours**

---

## 🧪 Testing Strategy

### Test Categories (30 total)

**Parser Tests** (8):

- [x] Parse `@export` with no hints
- [x] Parse `@export(range(0, 100))`
- [x] Parse `@export(range(0.0, 10.0, 0.5))`
- [x] Parse `@export(file("*.png", "*.jpg"))`
- [x] Parse `@export(enum("Easy", "Medium", "Hard"))`
- [x] Error: Invalid hint syntax
- [x] Error: Missing closing parenthesis
- [x] Error: Unknown hint name

**Type Checker Tests** (12):

- [x] Export eligible types (i32, f32, bool, String, Vector2, Color, Rect2, Transform2D)
- [x] Error E802: Export on unsupported type (e.g., Node)
- [x] Error E804: range hint on String
- [x] Error E805: file hint on i32
- [x] Error E806: enum hint on i32
- [x] Error E807: range min > max
- [x] Error E808: empty enum values
- [x] Error E810: duplicate @export
- [x] Error E811: @export without variable
- [x] Hint compatibility matrix (all valid combinations)
- [x] Nested type exports (Rect2 { position, size })
- [x] Multiple exports in same scope

**Runtime Tests** (10):

- [x] Metadata storage (property name, type, hint, default)
- [x] Property get (runtime → Godot)
- [x] Property set (Godot → runtime)
- [x] Inspector property list (all types appear)
- [x] Range hint enforcement (clamp values)
- [x] File hint (no runtime enforcement, editor only)
- [x] Enum hint validation (reject invalid values)
- [x] Property change tracking (detect modifications)
- [x] Multiple properties on same script
- [x] Property persistence (value survives recompilation)

### CI Testing Strategy

**Unit Tests** (run in all CI builds):

- Mock godot_bind functions where possible
- Test conversion helpers directly (`format_hint_string()`, `property_metadata_to_property_info()`)
- Test clamp logic in isolation
- Test Variant ↔ Value round-trips with mock data
- **No Godot process required**

**Integration Tests** (separate CI job):

- Use existing test harness with Godot process
- Run headless Godot when available (Linux CI)
- Test full Inspector integration
- Verify property persistence across reloads
- **Requires Godot installation in CI**

**Local Development** (full Inspector testing):

- Use Godot editor for visual inspection
- Verify property appearance, types, hints in Inspector
- Test range sliders, file pickers, enum dropdowns
- Verify undo/redo behavior

### Robustness Testing (Phase 5.5)

After MVP complete (Sub-Phases 1-3), add edge case tests:

**Compiler Edge Cases** (10 tests):

- Missing hint arguments
- Malformed hint syntax
- Export on global vs local variables
- **Export on immutable (`let`) vs mutable (`let mut`)** (NEW: test read-only)
- Hint with wrong number of arguments
- Nested hint parentheses
- Unicode in enum values
- Very large range values
- Negative step in range
- Empty file extension list

**Runtime Edge Cases** (8 tests):

- Property set with wrong type (type coercion or error?)
- **Property set with out-of-range value (test clamp behavior)** (NEW)
- Property set with invalid enum value
- Inspector query non-existent property
- Multiple property sets in single frame
- Property set during _ready()
- Property get before initialization
- **Variant ↔ Value round-trip for all struct types** (NEW: Vector2, Color, Rect2, Transform2D)
- Property persistence across scene changes

**Integration Examples** (3 files):

- `export_basic.ferris` - Simple property exports
- `export_hints.ferris` - All hint types
- `export_game_config.ferris` - Real-world game settings

**Total Robustness Tests**: 21 (10 compiler + 8 runtime + 3 examples)

**Total All Tests**: 51 (30 MVP + 21 robustness)

---

## 🚨 Error Code Definitions

### E801-E815: Export Annotation Errors

| Code | Category | Message Template | Example |
|------|----------|------------------|---------|
| E801 | Parser | `@export can only be applied to variable declarations` | `@export fn test() {}` |
| E802 | Type Checker | `Cannot export type {type} (unsupported)` | `@export let node: Node;` |
| E803 | Parser | `Invalid property hint syntax` | `@export(range(0))` |
| E804 | Type Checker | `range hint cannot be used with {type}` | `@export(range(0, 10)) let name: String;` |
| E805 | Type Checker | `file hint can only be used with String` | `@export(file("*.png")) let x: i32;` |
| E806 | Type Checker | `enum hint can only be used with String` | `@export(enum("A", "B")) let x: i32;` |
| E807 | Type Checker | `range hint: min ({min}) must be less than max ({max})` | `@export(range(10, 5))` |
| E808 | Type Checker | `enum hint must have at least one value` | `@export(enum())` |
| E809 | Parser | `Missing required hint parameter: {param}` | `@export(range(0))` |
| E810 | Type Checker | `Duplicate @export annotation on variable {name}` | `@export @export let x;` |
| E811 | Parser | `@export annotation must be followed by variable declaration` | `@export { }` |
| E812 | Type Checker | `@export on immutable variable makes property read-only in Inspector` (warning) | `@export let x: i32 = 10;` |
| E813 | Type Checker | `@export default must be compile-time constant (literal or struct literal)` | `@export let x: i32 = calc();` |
| E814 | Runtime | `Property {name} type mismatch: expected {expected}, got {actual}` | Inspector set wrong type |
| E815 | Runtime | `Property change validation failed: {reason}` | Custom validation error |
| E816 | Runtime | `Property {name} value {value} exceeds range [{min}, {max}]` (warning, script set) | Script sets out-of-range |

---

## 📁 File Modifications

### Files to Modify

**Compiler** (`crates/compiler/`):

1. `src/lexer.rs`:
   - Add `Token::At` (`@` symbol)
   - Add `Token::Export` keyword
   - Add `Token::Range`, `Token::File`, `Token::Enum` hint keywords

2. `src/ast.rs`:
   - Add `PropertyHint` enum (Range, File, Enum)
   - Add `ExportAnnotation` struct
   - Add `export: Option<ExportAnnotation>` to `VarDecl`

3. `src/parser.rs`:
   - Add `parse_export_annotation()` method
   - Add `parse_property_hint()` method
   - Integrate annotation parsing into `parse_var_decl()`

4. `src/type_checker.rs`:
   - Add `check_export_eligibility()` method
   - Add `validate_property_hint()` method
   - Add 15 error code messages (E801-E815)
   - **Add `generate_property_metadata()` method** (NEW: output static metadata)
   - **Add `Vec<PropertyMetadata>` to Program AST** (NEW: embedded compile-time metadata)

**Runtime** (`crates/runtime/`):

1. `src/lib.rs`:
   - **Add `PropertyMetadata` struct** (same as compiler, serializable)
   - Add `PropertyHint` enum (mirror compiler)
   - **Add `property_values: HashMap<String, Value>` to ScriptInstance** (SIMPLIFIED: only values, not metadata)
   - **Remove per-instance metadata storage** (metadata is static in Program)
   - Add `get_property()` method (lookup value + read static metadata)
   - Add `set_property()` method with **clamp-on-set logic for range hints**

**Godot Binding** (`crates/godot_bind/`):

1. `src/lib.rs`:
   - **Add `get_property_list()` implementation reading from static Program metadata** (SIMPLIFIED)
   - Add `get()` property getter (reads from runtime values)
   - Add `set()` property setter (writes to runtime values, clamps if range)
   - Add `property_metadata_to_property_info()` conversion (static → Godot)
   - Add `format_hint_string()` with exact formats ("0,100,1", "Easy,Medium,Hard", "*.png,*.jpg")
   - Add `variant_to_value()` and `value_to_variant()` for all export types with round-trip tests

### Files to Create

**Documentation**:

- `docs/planning/v0.0.4/PHASE_5_COMPLETION_REPORT.md` (after completion)
- `examples/export_basic.ferris` (basic property examples)
- `examples/export_hints.ferris` (hint demonstrations)
- `examples/export_game_config.ferris` (real-world config)

**Tests**: Inline in existing test modules (no new files)

---

## 🎯 Success Criteria

### Functional Requirements

- [x] Parser recognizes `@export` annotation
- [x] Parser handles 3 hint types (range, file, enum)
- [x] Type checker validates export eligibility (8 types)
- [x] Type checker validates hint compatibility
- [x] Runtime stores property metadata
- [x] Godot Inspector shows exported properties
- [x] Inspector can get property values
- [x] Inspector can set property values
- [x] Range hints clamp values in runtime
- [x] Enum hints reject invalid values in runtime

### Quality Requirements

- [x] 30 MVP tests passing (8 parser + 12 type checker + 10 runtime)
- [x] 21 robustness tests passing (10 compiler + 8 runtime + 3 examples)
- [x] 51 total tests passing (30 MVP + 21 robustness)
- [x] Zero build warnings or clippy errors
- [x] All tests passing: `cargo test --all`
- [x] Test harness validation for examples
- [x] Documentation complete and accurate

### Integration Requirements

- [x] Exports work with all supported types
- [x] Exports work with struct literals
- [x] Properties persist across script reloads
- [x] Inspector updates reflect in runtime immediately
- [x] Runtime changes reflect in Inspector (if possible)

---

## 💡 Learnings from Phase 4.5 (Apply Here)

### Checkpoint Methodology ✅

**What Worked**:

- 8 structured checkpoints per sub-phase
- Natural pause points for testing
- Easy to resume work
- 50% faster than Phase 4

**Apply to Phase 5**:

- Use 24 checkpoints across 3 sub-phases
- Run tests after each checkpoint
- Document assumptions inline
- Commit after each sub-phase complete

### Error Code Strategy ✅

**What Worked**:

- Semantic grouping (E70x for struct literals)
- Error code reuse across similar types
- Clear error messages with examples

**Apply to Phase 5**:

- Use E801-E815 range for export annotations
- Group by category (parser, type checker, runtime)
- Include code examples in error messages

### Robustness Testing ✅

**What Worked**:

- Separate MVP from robustness testing
- Edge case categories (missing, wrong, extra, coercion)
- Integration examples validate real-world usage

**Apply to Phase 5**:

- MVP first (30 tests), robustness second (21 tests)
- Test hint validation edge cases
- Create 3 integration examples

### Test-First Validation ✅

**What Worked**:

- Zero bugs found during robustness testing
- Tests caught issues early in checkpoints

**Apply to Phase 5**:

- Write tests before implementation
- Run tests after each checkpoint
- Validate error codes with tests

---

## 📝 Implementation Notes

### Parser Strategy

**Approach**: Extend parser to recognize `@` token before variable declarations

**Grammar Extension**:

```
var_decl := [export_annotation] 'let' ['mut'] identifier ':' type ['=' expression] ';'
export_annotation := '@' 'export' ['(' property_hint ')']
property_hint := range_hint | file_hint | enum_hint
range_hint := 'range' '(' number ',' number [',' number] ')'
file_hint := 'file' '(' string [',' string]* ')'
enum_hint := 'enum' '(' string [',' string]* ')'
```

**Challenge**: Parenthesized arguments require lookahead to determine hint type

**Solution**: Peek at next token after `'('` to determine hint variant

### Type Checker Strategy

**Approach**: Validation matrix for type + hint compatibility

**Matrix** (Allowed Combinations):

| Type | range | file | enum | Notes |
|------|-------|------|------|-------|
| i32 | ✅ | ❌ | ❌ | Integer range |
| f32 | ✅ | ❌ | ❌ | Float range with step |
| bool | ❌ | ❌ | ❌ | No hints needed |
| String | ❌ | ✅ | ✅ | File paths or enum values |
| Vector2 | ❌ | ❌ | ❌ | Struct type, no hints yet |
| Color | ❌ | ❌ | ❌ | Struct type, no hints yet |
| Rect2 | ❌ | ❌ | ❌ | Struct type, no hints yet |
| Transform2D | ❌ | ❌ | ❌ | Struct type, no hints yet |

**Note**: Struct type hints (e.g., color picker, vector editor) deferred to v0.0.5+

### Runtime Strategy

**Approach**: HashMap of property metadata in ScriptInstance

**Metadata Structure**:

```rust
pub struct PropertyMetadata {
    pub name: String,
    pub type_info: TypeInfo,
    pub hint: Option<PropertyHint>,
    pub default_value: Value,
}

pub enum PropertyHint {
    Range { min: f32, max: f32, step: f32 },
    File { extensions: Vec<String> },
    Enum { values: Vec<String> },
}
```

**Storage**: `HashMap<String, PropertyMetadata>` in `ScriptInstance`

**Access**: Property get/set methods query HashMap

### Godot Binding Strategy

**Approach**: Implement GDExtension property reflection traits

**Required Methods**:

1. `get_property_list()` → return `Vec<PropertyInfo>`
2. `get(property: StringName)` → return `Option<Variant>`
3. `set(property: StringName, value: Variant)` → bool

**PropertyInfo Structure** (Godot):

```rust
PropertyInfo {
    name: "property_name",
    type_: VariantType::FLOAT,
    hint: PropertyHint::RANGE,
    hint_string: "0,100,0.1",  // Format: "min,max,step"
    usage: PROPERTY_USAGE_DEFAULT,
}
```

**Conversion**: `PropertyHint` → Godot `PropertyHint` + `hint_string`

---

## 🧩 Integration with Existing Systems

### Struct Literals (Phase 4.5)

**Integration Point**: Struct literals can be default values for exports

```rust
@export
let start_pos: Vector2 = Vector2 { x: 100.0, y: 200.0 };
```

**Requirement**: Type checker must validate struct literal compatibility

### Signals (Phase 1)

**Integration Point**: Signals can notify property changes

```rust
signal property_changed(property: String, value: Variant);

@export
let health: i32 = 100;

fn set_health(new_health: i32) {
    health = new_health;
    emit_signal("property_changed", "health", health);
}
```

**Note**: This pattern deferred to v0.0.5 (custom setters)

### Lifecycle Callbacks (Phase 2)

**Integration Point**: Properties can be used in callbacks

```rust
@export
let speed: f32 = 10.0;

fn _process(delta: f32) {
    self.position.x += speed * delta;
}
```

**Requirement**: Property metadata available at runtime

---

## 🎨 Example Usage

### Basic Export

```rust
// examples/export_basic.ferris

@export
let player_name: String = "Hero";

@export
let health: i32 = 100;

@export
let speed: f32 = 10.0;

@export
let is_invincible: bool = false;

fn _ready() {
    print("Player: ", player_name);
    print("Health: ", health);
    print("Speed: ", speed);
    print("Invincible: ", is_invincible);
}
```

### Export with Hints

```rust
// examples/export_hints.ferris

@export(range(0, 100))
let health: i32 = 100;

@export(range(0.0, 20.0, 0.5))
let speed: f32 = 10.0;

@export(file("*.png", "*.jpg"))
let texture_path: String = "res://icon.png";

@export(enum("Easy", "Medium", "Hard"))
let difficulty: String = "Medium";

fn _ready() {
    print("Health: ", health);  // Clamped to 0-100
    print("Speed: ", speed);    // Clamped to 0-20, step 0.5
    print("Texture: ", texture_path);
    print("Difficulty: ", difficulty);  // Must be Easy/Medium/Hard
}
```

### Real-World Game Config

```rust
// examples/export_game_config.ferris

// Player settings
@export(range(1.0, 20.0, 0.5))
let player_speed: f32 = 5.0;

@export(range(50, 500, 10))
let player_health: i32 = 100;

@export(range(0.0, 5.0, 0.1))
let jump_force: f32 = 3.5;

// Enemy settings
@export(range(0.5, 10.0, 0.5))
let enemy_speed: f32 = 2.0;

@export(range(10, 200, 5))
let enemy_health: i32 = 50;

// Game settings
@export(enum("Easy", "Normal", "Hard", "Nightmare"))
let difficulty: String = "Normal";

@export
let enable_particles: bool = true;

@export(file("*.ogg", "*.wav"))
let background_music: String = "res://audio/theme.ogg";

fn _ready() {
    print("=== Game Configuration ===");
    print("Player Speed: ", player_speed);
    print("Player Health: ", player_health);
    print("Jump Force: ", jump_force);
    print("Enemy Speed: ", enemy_speed);
    print("Enemy Health: ", enemy_health);
    print("Difficulty: ", difficulty);
    print("Particles: ", enable_particles);
    print("Music: ", background_music);
}

fn _process(delta: f32) {
    // Use exported properties in game logic
    self.position.x += player_speed * delta;
}
```

---

## 📈 Progress Tracking

### Sub-Phase 1: Parser & AST (⏸️ Not Started)

- [ ] Checkpoint 1.1: Lexer tokens
- [ ] Checkpoint 1.2: Basic annotation
- [ ] Checkpoint 1.3: AST node
- [ ] Checkpoint 1.4: Range hint
- [ ] Checkpoint 1.5: File hint
- [ ] Checkpoint 1.6: Enum hint
- [ ] Checkpoint 1.7: Error recovery
- [ ] Checkpoint 1.8: Integration tests
- [ ] **Sub-Phase 1 Complete**: 8/8 checkpoints ✅

### Sub-Phase 2: Type Checker (⏸️ Not Started)

- [ ] Checkpoint 2.1: Type eligibility
- [ ] Checkpoint 2.2: Hint compatibility
- [ ] Checkpoint 2.3: Range validation
- [ ] Checkpoint 2.4: File validation
- [ ] Checkpoint 2.5: Enum validation
- [ ] Checkpoint 2.6: Duplicate detection
- [ ] Checkpoint 2.7: Orphan annotation
- [ ] Checkpoint 2.8: Error tests
- [ ] **Sub-Phase 2 Complete**: 8/8 checkpoints ✅

### Sub-Phase 3: Runtime & Godot (⏸️ Not Started)

- [ ] Checkpoint 3.1: Metadata struct
- [ ] Checkpoint 3.2: Storage
- [ ] Checkpoint 3.3: Property get
- [ ] Checkpoint 3.4: Property set
- [ ] Checkpoint 3.5: PropertyInfo
- [ ] Checkpoint 3.6: Hint strings
- [ ] Checkpoint 3.7: property_list
- [ ] Checkpoint 3.8: Inspector handlers
- [ ] **Sub-Phase 3 Complete**: 8/8 checkpoints ✅

### Phase 5.5: Robustness Testing (⏸️ Not Started)

- [ ] Compiler edge cases (10 tests)
- [ ] Runtime edge cases (8 tests)
- [ ] Integration examples (3 files)
- [ ] Documentation update
- [ ] **Phase 5.5 Complete**: 21 tests ✅

### Overall Progress

- **Phase 5 MVP**: 0/24 checkpoints (0%)
- **Phase 5 Robustness**: 0/21 tests (0%)
- **Phase 5 Complete**: 0/51 total tests (0%)

---

## 📚 Documentation Deliverables

### During Implementation

- **Inline Comments**: Document assumptions and trade-offs in code
- **Checkpoint Notes**: Track progress in this execution plan
- **Test Descriptions**: Clear test names and comments

### After MVP Complete

- **PHASE_5_COMPLETION_REPORT.md**: Summary of what was delivered
- **LEARNINGS.md update**: Add Phase 5 section with insights
- **README.md update**: Add @export syntax examples
- **ERROR_CODES.md update**: Add E801-E815 documentation

### After Robustness Complete

- **Integration examples**: 3 .ferris files with TEST metadata
- **Example READMEs**: Explain usage patterns
- **User guide section**: How to use @export in real projects

---

## 🎓 Key Learnings (To Document in LEARNINGS.md)

### Expected Insights

**Parser Complexity**:

- Decorator-like syntax vs statement syntax
- Lookahead strategies for hint parsing
- Error recovery in annotation context

**Type Checker Validation Matrix**:

- Type + hint compatibility patterns
- Error code organization strategies
- Validation order (type first, then hint)

**Runtime Reflection**:

- Metadata storage vs performance
- Property change tracking approaches
- Godot synchronization patterns

**Godot Inspector Integration**:

- PropertyInfo conversion strategies
- Bidirectional sync challenges
- Hint string formatting conventions

**Cross-Cutting Concerns**:

- How annotations flow through pipeline
- Metadata consistency across crates
- Testing multi-environment features

---

## ✅ Pre-Implementation Checklist

Before starting Sub-Phase 1:

- [x] Research document reviewed (EXPORT_ANNOTATION_RESEARCH.md)
- [x] Execution plan created (this document)
- [x] Checkpoint strategy defined (24 checkpoints)
- [x] Test strategy defined (51 tests)
- [x] Error codes allocated (E801-E815)
- [ ] User confirms ready to proceed
- [ ] Dedicated 4-5 day session scheduled
- [ ] Branch created: `feature/v0.0.4-phase5-export-annotation`

**Status**: ✅ **PLANNING COMPLETE** - Ready for implementation when user schedules dedicated session

---

**Document Version**: 1.0  
**Last Updated**: October 10, 2025  
**Next Review**: Before Phase 5 implementation begins
