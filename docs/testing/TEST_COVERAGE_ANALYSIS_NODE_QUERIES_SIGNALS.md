# Test Coverage Analysis: Node Queries & Signals

## Executive Summary

**Analysis Date**: October 10, 2025  
**Scope**: Node query functions (`get_node`, `get_parent`, `has_node`, `find_child`) and signal system (`signal`, `emit_signal`)  
**Test Suites Analyzed**: Unit (runtime), Unit (compiler), Integration (examples), Headless (test harness)

**Key Findings**:

- ✅ **Basic functionality well-covered** across all test suites
- ⚠️ **Edge cases have gaps**, especially in headless tests
- ⚠️ **Variable inputs/configurations** not systematically tested
- ❌ **Cross-cutting concerns** (security, performance, error propagation) under-tested
- ❌ **No systematic coverage tracking mechanism** beyond code coverage

---

## Coverage Matrix

### Node Query Functions

| Feature | Unit (Runtime) | Unit (Compiler) | Integration (Examples) | Headless | Edge Cases Covered |
|---------|----------------|-----------------|------------------------|----------|-------------------|
| **get_node()** | | | | | |
| Basic usage | ✅ `test_call_get_node_function` | ✅ `test_get_node_valid` | ✅ `node_query_basic.ferris` | ✅ Implicit in Phase 2 | Simple child path |
| Wrong arg count | ❌ | ✅ `test_get_node_wrong_arg_count` | ❌ | ❌ | 0, 2+ args |
| Wrong arg type | ❌ | ✅ `test_get_node_wrong_arg_type` | ❌ | ❌ | Non-string arg |
| Missing node | ⚠️ `test_node_query_error_handling` | ❌ | ⚠️ `node_query_error_demo.ferris` | ⚠️ Error demo | Basic case only |
| No callback set | ✅ `test_node_query_without_callback` | ❌ | ❌ | ❌ | Returns SelfObject |
| Nested paths | ❌ | ❌ | ⚠️ `node_query_error_handling.ferris` | ❌ | "UI/HUD/HealthBar" |
| Relative paths | ❌ | ❌ | ❌ | ❌ | **GAP** |
| Absolute paths | ❌ | ❌ | ❌ | ❌ | **GAP** |
| Special chars in path | ❌ | ❌ | ❌ | ❌ | **GAP** |
| Very long paths | ❌ | ❌ | ❌ | ❌ | **GAP** |
| Empty string path | ❌ | ❌ | ❌ | ❌ | **GAP** |
| Unicode in path | ❌ | ❌ | ❌ | ❌ | **GAP** |
| Path with spaces | ❌ | ❌ | ❌ | ❌ | **GAP** |
| **get_parent()** | | | | | |
| Basic usage | ✅ `test_call_get_parent_function` | ✅ `test_get_parent_valid` | ✅ `node_query_basic.ferris` | ✅ Implicit | Returns parent |
| With args (error) | ❌ | ✅ `test_get_parent_with_args` | ❌ | ❌ | Expects 0 args |
| No callback set | ❌ | ❌ | ❌ | ❌ | **GAP** |
| At root node | ❌ | ❌ | ❌ | ❌ | **GAP** |
| **has_node()** | | | | | |
| Basic usage (exists) | ✅ `test_call_has_node_function` | ✅ `test_has_node_valid` | ✅ `node_query_validation.ferris` | ✅ Implicit | Returns true |
| Node doesn't exist | ⚠️ Same test | ❌ | ⚠️ Same example | ❌ | Returns false |
| Wrong arg count | ❌ | ✅ `test_has_node_wrong_arg_count` | ❌ | ❌ | 0, 2+ args |
| Wrong arg type | ❌ | ✅ `test_has_node_wrong_arg_type` | ❌ | ❌ | Non-string arg |
| No callback set | ❌ | ❌ | ❌ | ❌ | **GAP** |
| Nested paths | ❌ | ❌ | ⚠️ Optional in validation example | ❌ | "Enemies/Boss" |
| Edge case paths | ❌ | ❌ | ❌ | ❌ | **GAP** |
| **find_child()** | | | | | |
| Basic usage | ✅ `test_call_find_child_function` | ✅ `test_find_child_valid` | ✅ `node_query_search.ferris` | ✅ Implicit | Recursive search |
| Wrong arg count | ❌ | ✅ `test_find_child_wrong_arg_count` | ❌ | ❌ | 0, 2+ args |
| Wrong arg type | ❌ | ✅ `test_find_child_wrong_arg_type` | ❌ | ❌ | Non-string arg |
| No callback set | ❌ | ❌ | ❌ | ❌ | **GAP** |
| Not found | ❌ | ❌ | ❌ | ❌ | **GAP** |
| Multiple matches | ❌ | ❌ | ❌ | ❌ | **GAP** (returns first?) |
| Deep nesting | ❌ | ❌ | ⚠️ Implied in example | ❌ | Performance? |
| Case sensitivity | ❌ | ❌ | ❌ | ❌ | **GAP** |

**Summary**:

- ✅ **Covered**: 15 test cases
- ⚠️ **Partially Covered**: 8 test cases
- ❌ **Not Covered**: 23 test cases (major gaps)

---

### Signal System

| Feature | Unit (Runtime) | Unit (Compiler) | Integration (Examples) | Headless | Edge Cases Covered |
|---------|----------------|-----------------|------------------------|----------|-------------------|
| **signal Declaration** | | | | | |
| No params | ✅ `test_signal_declaration_in_program` | ✅ `test_signal_no_params` | ✅ `signals.ferris` | ❌ | Basic declaration |
| With params | ✅ `test_register_signal` | ✅ `test_signal_declaration_valid` | ✅ `signals.ferris` | ❌ | Typed parameters |
| Duplicate signal | ❌ | ✅ `test_signal_duplicate_name_error` | ❌ | ❌ | Error E401 |
| Undefined type | ❌ | ✅ `test_signal_undefined_type_error` | ❌ | ❌ | Error E402 |
| Invalid syntax | ❌ | ✅ Multiple parser tests | ❌ | ❌ | Missing parens, semicolon |
| Signal ordering | ❌ | ❌ | ⚠️ signals.ferris shows top | ❌ | Must be at top? |
| Many signals | ❌ | ❌ | ❌ | ❌ | **GAP** |
| **emit_signal()** | | | | | |
| Basic emission | ✅ `test_emit_signal_in_function` | ✅ `test_emit_signal_valid` | ✅ `signals.ferris` | ❌ | With params |
| No params | ✅ `test_emit_signal_with_no_params` | ⚠️ Implied | ✅ `signals.ferris` | ❌ | player_died |
| Builtin exists | ✅ `test_emit_signal_builtin_exists` | ❌ | ❌ | ❌ | Function available |
| Callback invoked | ✅ `test_signal_emitter_callback_invoked` | ❌ | ❌ | ❌ | Callback mechanism |
| All param types | ✅ `test_signal_emitter_callback_all_types` | ❌ | ⚠️ signals.ferris | ❌ | i32, f32, bool, String, Vector2 |
| No callback set | ✅ `test_signal_emitter_without_callback` | ❌ | ❌ | ❌ | Returns Nil |
| Error handling | ✅ `test_signal_emitter_error_handling` | ❌ | ❌ | ❌ | Callback errors |
| No signal name | ✅ `test_emit_signal_error_no_signal_name` | ❌ | ❌ | ❌ | Error E501 |
| Invalid name type | ✅ `test_emit_signal_error_invalid_signal_name_type` | ❌ | ❌ | ❌ | Error E502 |
| Undefined signal | ❌ | ✅ `test_emit_signal_undefined_error` | ❌ | ❌ | Error E403 |
| Param count mismatch | ❌ | ✅ `test_emit_signal_param_count_mismatch` | ❌ | ❌ | Error E404 |
| Param type mismatch | ❌ | ✅ `test_emit_signal_param_type_mismatch` | ❌ | ❌ | Error E405 |
| Type coercion | ❌ | ✅ `test_emit_signal_type_coercion` | ❌ | ❌ | i32→f32 |
| Emit from lifecycle | ❌ | ❌ | ⚠️ signals.ferris (_ready, _process) | ❌ | Common pattern |
| Emit in conditional | ❌ | ❌ | ⚠️ signals.ferris (if health<=0) | ❌ | Common pattern |
| Emit in loop | ❌ | ❌ | ❌ | ❌ | **GAP** |
| Multiple emissions | ❌ | ❌ | ⚠️ signals.ferris (multiple funcs) | ❌ | Chaining behavior |
| Signal name as variable | ❌ | ❌ | ❌ | ❌ | **GAP** |
| Very long signal name | ❌ | ❌ | ❌ | ❌ | **GAP** |
| Unicode in signal name | ❌ | ❌ | ❌ | ❌ | **GAP** |

**Summary**:

- ✅ **Covered**: 22 test cases
- ⚠️ **Partially Covered**: 6 test cases
- ❌ **Not Covered**: 7 test cases

---

## Identified Gaps by Category

### 1. Path Handling Edge Cases (Node Queries)

**Current**: Only test simple child paths like "Player", "UI", nested like "UI/HUD/HealthBar"

**Missing**:

- ❌ Relative paths: `"../Sibling"`, `"./Child"`
- ❌ Absolute paths: `"/root/Main/Player"`
- ❌ Path with trailing slash: `"UI/"`
- ❌ Path with leading slash: `"/Player"`
- ❌ Empty string: `""`
- ❌ Path with spaces: `"My Player"`
- ❌ Path with special chars: `"Player-1"`, `"UI@2"`
- ❌ Very long paths (100+ chars)
- ❌ Unicode in paths: `"玩家"`, `"Игрок"`
- ❌ Path with dots: `"..."`
- ❌ Path components that look like relative: `"NotRelative../Child"`

**Impact**: High - Path handling is core functionality, security risk if not validated

**Recommendation**: Add dedicated test suite for path edge cases

### 2. Callback Behavior (Node Queries)

**Current**: Only `test_node_query_without_callback` in runtime

**Missing**:

- ❌ `get_node()` without callback (should return SelfObject)
- ❌ `get_parent()` without callback
- ❌ `has_node()` without callback (should return false?)
- ❌ `find_child()` without callback
- ❌ Behavior when callback returns error
- ❌ Behavior when callback returns None
- ❌ Callback performance with many queries

**Impact**: Medium - Affects developer experience, could cause confusion

**Recommendation**: Test each node query function with/without callback

### 3. Error Propagation & Recovery

**Current**: Basic error tests exist, but error propagation not tested

**Missing**:

- ❌ Error from `get_node()` stops execution?
- ❌ Can catch node query errors with try/catch (when implemented)?
- ❌ Error messages are clear and actionable?
- ❌ Errors include context (file, line, function)?
- ❌ Multiple errors in sequence?
- ❌ Error during callback execution?

**Impact**: Medium - Affects debugging experience

**Recommendation**: Add error propagation integration tests

### 4. Performance & Scale

**Current**: No performance tests

**Missing**:

- ❌ Many sequential node queries (100+)
- ❌ `find_child()` on deep trees (100+ levels)
- ❌ `find_child()` on wide trees (1000+ siblings)
- ❌ Signal emission in hot loop (1000+ per frame)
- ❌ Many signal declarations (100+)
- ❌ Many signal parameters (10+)
- ❌ Memory leak from callback closures?

**Impact**: Medium - Could cause performance issues in production

**Recommendation**: Add performance benchmarks, not just correctness tests

### 5. Signal Edge Cases

**Current**: Good coverage of basic signal operations

**Missing**:

- ❌ Signal name as runtime variable: `let name = "health_changed"; emit_signal(name, ...)`
- ❌ Emitting undefined signal (runtime error vs compile error)
- ❌ Very long signal names (100+ chars)
- ❌ Unicode in signal names
- ❌ Special characters in signal names
- ❌ Signal parameters at maximum (10+?)
- ❌ Nested signal emissions (signal handler emits signal)
- ❌ Recursive signal emissions (signal A→B→A)

**Impact**: Low-Medium - Less common but could cause issues

**Recommendation**: Add signal edge case test suite

### 6. Integration Scenarios

**Current**: Examples show basic usage patterns

**Missing**:

- ❌ Node queries in signal handlers
- ❌ Signals emitted from node query callbacks
- ❌ Chained node queries (`get_parent().get_node()` - not supported?)
- ❌ Node queries with mutable state
- ❌ Concurrent node queries (if async added)
- ❌ Node queries across scene changes
- ❌ Signal emissions during scene transitions

**Impact**: Medium - Real-world usage patterns

**Recommendation**: Add integration test scenarios

### 7. Type System Interactions

**Current**: Compiler tests cover basic type checking

**Missing**:

- ❌ Node query return type used in expressions
- ❌ Signal parameters with complex types (custom structs?)
- ❌ Type inference with node queries
- ❌ Generic types in signal parameters (when added)
- ❌ Nullable vs non-nullable node returns

**Impact**: Low - Type system is stable, but interactions not tested

**Recommendation**: Add type system integration tests

---

## Systematic Coverage Tracking Mechanism

### Problem

Current approach relies on:

1. **Code coverage** - Shows lines executed, not scenarios covered
2. **Manual review** - Labor-intensive, error-prone, not scalable
3. **Ad-hoc testing** - Easy to miss edge cases

### Proposed Solution: Test Matrix Document

Create a living document that systematically tracks test coverage:

**File**: `docs/testing/TEST_MATRIX_NODE_QUERIES_SIGNALS.md`

**Structure**:

```markdown
## Test Matrix: Node Queries

| Scenario | Description | Input | Expected Output | Unit Test | Integration Test | Headless Test | Status |
|----------|-------------|-------|-----------------|-----------|------------------|---------------|--------|
| NQ-001 | Basic get_node | "Player" | Node object | ✅ test_call_get_node_function | ✅ node_query_basic | ✅ Implicit | PASS |
| NQ-002 | Missing node | "Missing" | Error E603 | ⚠️ test_node_query_error_handling | ✅ node_query_error_demo | ⚠️ Error demo | PASS |
| NQ-003 | Empty path | "" | Error E602? | ❌ | ❌ | ❌ | TODO |
| NQ-004 | Relative path | "../Sibling" | Node or error | ❌ | ❌ | ❌ | TODO |
...
```

**Benefits**:

- ✅ Single source of truth for what's tested
- ✅ Easy to identify gaps (scan for ❌)
- ✅ Maps scenarios to specific tests
- ✅ Can track status (TODO, IN PROGRESS, PASS, FAIL)
- ✅ Can assign test IDs (NQ-001, SIG-042)
- ✅ Can link to issues/PRs
- ✅ Version controlled, reviewable

### Implementation Plan

**Phase 1: Create Initial Matrix** (1-2 hours)

1. Extract all existing test cases
2. Categorize by feature/scenario
3. Assign test IDs
4. Mark current coverage status
5. Identify immediate gaps

**Phase 2: Add Edge Cases** (2-3 hours)

1. Brainstorm edge cases per feature
2. Add to matrix with status=TODO
3. Prioritize by impact/likelihood
4. Create issues for high-priority gaps

**Phase 3: Integrate into Workflow** (ongoing)

1. Reference test IDs in test code comments
2. Update matrix when adding tests
3. Review matrix in PR process
4. Use in test planning sessions

**Phase 4: Automate** (future)

1. Parse test code for test IDs
2. Auto-update matrix from test results
3. Generate coverage reports from matrix
4. CI integration (fail if coverage drops)

---

## Edge Case Discovery Strategies

### Beyond Code Coverage

Code coverage shows **which lines** run, not **which scenarios** are tested.

**Strategies for discovering untested edge cases**:

### 1. **Boundary Value Analysis**

For each input parameter, test:

- Minimum value
- Maximum value
- Just below minimum
- Just above maximum
- Zero (if numeric)
- Negative (if numeric)
- Empty (if string/collection)

**Example: get_node(path: String)**

- Empty string: `""`
- Single char: `"A"`
- Very long: `"A" * 1000`
- Max path length: System dependent (260 on Windows?)
- Unicode: `"🚀🎮"`
- Null character: `"Player\0Hacker"`

### 2. **Equivalence Partitioning**

Group inputs into classes that should behave similarly, test one from each:

**Example: Node paths**

- **Simple child**: `"Player"`, `"Enemy"`
- **Nested**: `"UI/HUD"`, `"World/Level1/Boss"`
- **Relative**: `"../Sibling"`, `"./Child"`
- **Absolute**: `"/root/Main"`, `"/root"`
- **Invalid**: `""`, `"///"`, `"."`, `".."`

### 3. **State-Based Testing**

Test how features behave in different system states:

**Example: Node queries**

- Before `_ready()` called
- During `_ready()`
- During `_process()`
- After scene change
- With/without callback registered
- With/without parent node
- At root vs leaf nodes

### 4. **Error Guessing**

Based on experience, guess where errors might occur:

**Common patterns**:

- Off-by-one errors
- Null/undefined handling
- Empty collection handling
- Unicode/encoding issues
- Case sensitivity
- Whitespace handling
- Resource exhaustion

### 5. **Combinatorial Testing**

Test combinations of inputs (not just individual inputs):

**Example: emit_signal()**

- Signal name: defined vs undefined
- Parameters: 0, 1, many
- Parameter types: matching vs mismatching
- Callback: set vs unset
- Context: _ready vs _process vs custom function

### 6. **Property-Based Testing**

Define properties that should always hold, test with random inputs:

**Example: has_node() property**

```
Property: If has_node(path) returns true, then get_node(path) should succeed
Test: Generate 1000 random valid paths, verify property holds
```

### 7. **Mutation Testing**

Change code slightly, verify tests catch the bug:

**Example**: Change `if args.len() == 1` to `if args.len() <= 1`

- Should: Test fails (detects mutation)
- If not: Missing test for 0 args case

### 8. **Documentation-Driven Testing**

Test every example in documentation:

**Example**: If docs say "paths can be relative", test relative paths

### 9. **User Story Testing**

Test realistic user scenarios:

**Example**:

```
As a game developer
I want to query nodes by name
So I can access them in my script

Scenarios:
- Simple access in _ready
- Access in _process (performance?)
- Access after dynamic spawn
- Access after node rename/move
```

### 10. **Security Testing**

Test for potential security issues:

**Example: Path injection**

- Can path escape scene tree? `"../../../etc/passwd"`
- Can path access internal nodes? `"__internal__"`
- Can path cause DOS? `"A" * 1000000`

---

## Recommended Next Steps

### Immediate (This Sprint)

1. **Create Test Matrix Document** (Priority: High)
   - Map all existing tests to scenarios
   - Assign test IDs
   - Identify top 10 gaps

2. **Add Top 5 Missing Edge Case Tests** (Priority: High)
   - Empty string path
   - `get_node()` without callback
   - `has_node()` for missing node
   - Signal name validation
   - Path with special characters

3. **Add Headless Tests for Error Demos** (Priority: Medium)
   - Currently have `node_query_error_demo.ferris`
   - Need structured test with EXPECT: error
   - Validate error message content

### Short-term (Next 2 Sprints)

4. **Expand Integration Tests** (Priority: Medium)
   - Add 5 more real-world scenarios
   - Test node queries in signal handlers
   - Test signal emissions in callbacks

5. **Add Performance Benchmarks** (Priority: Medium)
   - `find_child()` on deep/wide trees
   - Signal emission in loops
   - Many sequential queries

6. **Implement Property-Based Tests** (Priority: Low)
   - Use PropTest crate
   - Test node query invariants
   - Test signal parameter validation

### Long-term (Future Sprints)

7. **Automate Test Matrix Updates** (Priority: Low)
   - Parse test code for IDs
   - Generate reports
   - CI integration

8. **Security Testing** (Priority: Medium)
   - Path injection tests
   - Resource exhaustion tests
   - Fuzzing with cargo-fuzz

9. **Mutation Testing** (Priority: Low)
   - Use cargo-mutants
   - Verify test quality
   - Improve weak tests

---

## Coverage Metrics

### Current Coverage (Estimated)

**Node Queries**:

- Unit Tests (Runtime): 40% coverage (6/15 scenarios)
- Unit Tests (Compiler): 60% coverage (9/15 scenarios)
- Integration Tests: 30% coverage (4/15 scenarios via examples)
- Headless Tests: 20% coverage (basic assertions only)
- **Overall**: ~45% scenario coverage

**Signals**:

- Unit Tests (Runtime): 70% coverage (11/15 core scenarios)
- Unit Tests (Compiler): 80% coverage (12/15 compile-time scenarios)
- Integration Tests: 40% coverage (signals.ferris covers main patterns)
- Headless Tests: 0% coverage (no structured signal tests)
- **Overall**: ~60% scenario coverage

### Target Coverage

**Near-term Goals** (6 months):

- Unit Tests: 80% scenario coverage
- Integration Tests: 60% scenario coverage
- Headless Tests: 50% scenario coverage
- **Overall**: 70% scenario coverage

**Long-term Goals** (1 year):

- Unit Tests: 95% scenario coverage
- Integration Tests: 80% scenario coverage
- Headless Tests: 70% scenario coverage
- **Overall**: 85% scenario coverage

---

## Conclusion

**Current State**:

- ✅ Good foundation with unit and integration tests
- ⚠️ Significant gaps in edge cases and variable inputs
- ❌ No systematic tracking beyond code coverage

**Recommendations**:

1. **Create Test Matrix** - Systematic tracking of scenarios
2. **Add Edge Case Tests** - Focus on path handling, callbacks, errors
3. **Expand Headless Tests** - Structured tests with metadata
4. **Use Multiple Discovery Strategies** - Beyond just code coverage
5. **Set Coverage Goals** - Track improvement over time

**Next Action**: ~~Create `TEST_MATRIX_NODE_QUERIES_SIGNALS.md` to begin systematic tracking.~~ ✅ COMPLETED

---

## Implementation Update (October 10, 2025)

### Actions Completed

**1. Created Test Matrix Document** ✅

- File: `TEST_MATRIX_NODE_QUERIES_SIGNALS.md`
- 64 test scenarios mapped to test IDs
- Coverage tracked across all test suites
- Status tracking (PASS/PARTIAL/TODO/FAIL)
- Priority TODO list created

**2. Implemented Top 5 Missing Edge Case Tests** ✅

Added 5 new runtime tests to `crates/runtime/src/lib.rs`:

1. **NQ-008**: `test_get_node_empty_string` - Validates that get_node("") returns Error E603
2. **NQ-022**: `test_get_parent_without_callback` - Validates Error E606 when no callback set
3. **NQ-035**: `test_has_node_without_callback` - Validates Error E609 when no callback set
4. **NQ-037**: `test_has_node_empty_string` - Validates callback rejection of empty paths
5. **SIG-037**: `test_emit_signal_name_as_variable` - Documents that signal names must be string literals (Error E205)

**Test Results**:

```
running 85 tests (was 80)
test result: ok. 85 passed; 0 failed
```

**3. Updated Test Matrix with New Coverage** ✅

Coverage improvements:

- **Node Queries**: 30% PASS (was 18%) - +4 tests
- **Signals**: 32% PASS (was 29%) - +1 test
- **Overall**: 31% PASS (was 23%) - +5 tests

**Key Findings from New Tests**:

1. **Empty path validation exists** - Runtime checks for empty paths in `get_node()` (E603)
2. **Callback requirement is strict** - All node query functions require callback or return error
3. **has_node() doesn't validate empty paths** - Passes to callback for validation
4. **Signal names must be compile-time literals** - Variables not supported (design decision)

### Next Steps

**Immediate (Next Sprint)**:

1. Add remaining High Priority tests (NQ-045, NQ-044, NQ-010, NQ-046, NQ-023)
2. Create integration tests for new edge cases
3. Update examples to demonstrate edge case handling

**Short-term**:
4. Add Medium Priority edge cases
5. Expand headless test coverage
6. Create performance benchmarks for node queries

**Long-term**:
7. Automate test matrix updates from test code
8. CI integration for coverage tracking
9. Property-based testing implementation
