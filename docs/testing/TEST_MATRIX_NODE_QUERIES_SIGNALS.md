# Test Matrix: Node Queries & Signals

**Purpose**: Systematic tracking of test scenarios for node query and signal functionality  
**Last Updated**: October 10, 2025  
**Status Legend**: ✅ PASS | ⚠️ PARTIAL | ❌ TODO | 🚧 IN PROGRESS | 💥 FAIL

---

## Node Query Tests

### get_node(path: String) → Node

| Test ID | Scenario | Input | Expected Output | Unit Test (Runtime) | Unit Test (Compiler) | Integration Test | Headless Test | Status |
|---------|----------|-------|-----------------|---------------------|----------------------|------------------|---------------|--------|
| NQ-001 | Basic child access | `"Player"` | Node object | ✅ test_call_get_node_function | ✅ test_get_node_valid | ✅ node_query_basic.ferris | ✅ Implicit | ✅ PASS |
| NQ-002 | Nested path | `"UI/HUD/HealthBar"` | Node object | ⚠️ test_node_query_error_handling | ❌ | ⚠️ node_query_error_handling.ferris | ❌ | ⚠️ PARTIAL |
| NQ-003 | Missing node | `"NonExistent"` | Error E603 | ⚠️ test_node_query_error_handling | ❌ | ✅ node_query_error_demo.ferris | ⚠️ Error demo | ✅ PASS |
| NQ-004 | Wrong arg count (0) | `get_node()` | Error E601 | ❌ | ✅ test_get_node_wrong_arg_count | ❌ | ❌ | ⚠️ PARTIAL |
| NQ-005 | Wrong arg count (2+) | `get_node("A", "B")` | Error E601 | ❌ | ✅ test_get_node_wrong_arg_count | ❌ | ❌ | ⚠️ PARTIAL |
| NQ-006 | Wrong arg type | `get_node(123)` | Error E602 | ❌ | ✅ test_get_node_wrong_arg_type | ❌ | ❌ | ⚠️ PARTIAL |
| NQ-007 | No callback set | `"Player"` | SelfObject | ✅ test_node_query_without_callback | ❌ | ❌ | ❌ | ⚠️ PARTIAL |
| NQ-008 | Empty string | `""` | Error E603 | ✅ test_get_node_empty_string | ❌ | ❌ | ❌ | ✅ PASS |
| NQ-009 | Path with spaces | `"My Player"` | Node or error | ❌ | ❌ | ❌ | ❌ | ❌ TODO |
| NQ-010 | Path with special chars | `"Player-1"` | Node or error | ❌ | ❌ | ❌ | ❌ | ❌ TODO |
| NQ-011 | Relative path | `"../Sibling"` | Node or error | ❌ | ❌ | ❌ | ❌ | ❌ TODO |
| NQ-012 | Absolute path | `"/root/Main"` | Node or error | ❌ | ❌ | ❌ | ❌ | ❌ TODO |
| NQ-013 | Very long path | `"A" * 500` | Node or error | ❌ | ❌ | ❌ | ❌ | ❌ TODO |
| NQ-014 | Unicode path | `"玩家"` | Node or error | ❌ | ❌ | ❌ | ❌ | ❌ TODO |
| NQ-015 | Trailing slash | `"Player/"` | Node or error | ❌ | ❌ | ❌ | ❌ | ❌ TODO |

### get_parent() → Node

| Test ID | Scenario | Input | Expected Output | Unit Test (Runtime) | Unit Test (Compiler) | Integration Test | Headless Test | Status |
|---------|----------|-------|-----------------|---------------------|----------------------|------------------|---------------|--------|
| NQ-020 | Basic usage | `get_parent()` | Parent node | ✅ test_call_get_parent_function | ✅ test_get_parent_valid | ✅ node_query_basic.ferris | ✅ Implicit | ✅ PASS |
| NQ-021 | With args (error) | `get_parent("arg")` | Error E605 | ❌ | ✅ test_get_parent_with_args | ❌ | ❌ | ⚠️ PARTIAL |
| NQ-022 | No callback set | `get_parent()` | Error E606 | ✅ test_get_parent_without_callback | ❌ | ❌ | ❌ | ✅ PASS |
| NQ-023 | At root node | `get_parent()` | Null/error? | ❌ | ❌ | ❌ | ❌ | ❌ TODO |

### has_node(path: String) → bool

| Test ID | Scenario | Input | Expected Output | Unit Test (Runtime) | Unit Test (Compiler) | Integration Test | Headless Test | Status |
|---------|----------|-------|-----------------|---------------------|----------------------|------------------|---------------|--------|
| NQ-030 | Node exists | `"Player"` | `true` | ✅ test_call_has_node_function | ✅ test_has_node_valid | ✅ node_query_validation.ferris | ✅ Implicit | ✅ PASS |
| NQ-031 | Node doesn't exist | `"Missing"` | `false` | ✅ test_call_has_node_function | ❌ | ✅ node_query_validation.ferris | ❌ | ✅ PASS |
| NQ-032 | Wrong arg count (0) | `has_node()` | Error E607 | ❌ | ✅ test_has_node_wrong_arg_count | ❌ | ❌ | ⚠️ PARTIAL |
| NQ-033 | Wrong arg count (2+) | `has_node("A", "B")` | Error E607 | ❌ | ✅ test_has_node_wrong_arg_count | ❌ | ❌ | ⚠️ PARTIAL |
| NQ-034 | Wrong arg type | `has_node(123)` | Error E608 | ❌ | ✅ test_has_node_wrong_arg_type | ❌ | ❌ | ⚠️ PARTIAL |
| NQ-035 | No callback set | `"Player"` | Error E609 | ✅ test_has_node_without_callback | ❌ | ❌ | ❌ | ✅ PASS |
| NQ-036 | Nested path (exists) | `"UI/HUD"` | `true` | ❌ | ❌ | ⚠️ node_query_validation.ferris | ❌ | ⚠️ PARTIAL |
| NQ-037 | Empty string | `""` | Callback error | ✅ test_has_node_empty_string | ❌ | ❌ | ❌ | ✅ PASS |

### find_child(name: String) → Node

| Test ID | Scenario | Input | Expected Output | Unit Test (Runtime) | Unit Test (Compiler) | Integration Test | Headless Test | Status |
|---------|----------|-------|-----------------|---------------------|----------------------|------------------|---------------|--------|
| NQ-040 | Basic search | `"Enemy"` | Node object | ✅ test_call_find_child_function | ✅ test_find_child_valid | ✅ node_query_search.ferris | ✅ Implicit | ✅ PASS |
| NQ-041 | Wrong arg count (0) | `find_child()` | Error E610 | ❌ | ✅ test_find_child_wrong_arg_count | ❌ | ❌ | ⚠️ PARTIAL |
| NQ-042 | Wrong arg count (2+) | `find_child("A", "B")` | Error E610 | ❌ | ✅ test_find_child_wrong_arg_count | ❌ | ❌ | ⚠️ PARTIAL |
| NQ-043 | Wrong arg type | `find_child(123)` | Error E611 | ❌ | ✅ test_find_child_wrong_arg_type | ❌ | ❌ | ⚠️ PARTIAL |
| NQ-044 | No callback set | `"Enemy"` | SelfObject | ❌ | ❌ | ❌ | ❌ | ❌ TODO |
| NQ-045 | Not found | `"NonExistent"` | Null/error? | ❌ | ❌ | ❌ | ❌ | ❌ TODO |
| NQ-046 | Multiple matches | `"Item"` (2+ exist) | First match | ❌ | ❌ | ❌ | ❌ | ❌ TODO |
| NQ-047 | Deep nesting | Name 10+ levels deep | Node object | ❌ | ❌ | ⚠️ node_query_search.ferris | ❌ | ⚠️ PARTIAL |
| NQ-048 | Case sensitivity | `"enemy"` vs `"Enemy"` | Match or not? | ❌ | ❌ | ❌ | ❌ | ❌ TODO |
| NQ-049 | Empty string | `""` | Error E611 | ❌ | ❌ | ❌ | ❌ | ❌ TODO |

---

## Signal Tests

### signal Declaration

| Test ID | Scenario | Input | Expected Output | Unit Test (Runtime) | Unit Test (Compiler) | Integration Test | Headless Test | Status |
|---------|----------|-------|-----------------|---------------------|----------------------|------------------|---------------|--------|
| SIG-001 | No parameters | `signal player_died;` | Success | ✅ test_signal_declaration_in_program | ✅ test_signal_no_params | ✅ signals.ferris | ❌ | ✅ PASS |
| SIG-002 | With parameters | `signal health_changed(i32, i32);` | Success | ✅ test_register_signal | ✅ test_signal_declaration_valid | ✅ signals.ferris | ❌ | ✅ PASS |
| SIG-003 | Multiple types | `signal item(String, i32, f32);` | Success | ❌ | ❌ | ✅ signals.ferris | ❌ | ⚠️ PARTIAL |
| SIG-004 | Duplicate signal | `signal x; signal x;` | Error E401 | ❌ | ✅ test_signal_duplicate_name_error | ❌ | ❌ | ⚠️ PARTIAL |
| SIG-005 | Undefined type | `signal x(Unknown);` | Error E402 | ❌ | ✅ test_signal_undefined_type_error | ❌ | ❌ | ⚠️ PARTIAL |
| SIG-006 | Missing semicolon | `signal x()` | Parse error | ❌ | ✅ test_parse_signal_missing_semicolon | ❌ | ❌ | ⚠️ PARTIAL |
| SIG-007 | Missing parens | `signal x;` (should have ()) | Parse error? | ❌ | ✅ test_parse_signal_missing_parens | ❌ | ❌ | ⚠️ PARTIAL |
| SIG-008 | Many signals | 50+ signals | Success | ❌ | ❌ | ❌ | ❌ | ❌ TODO |
| SIG-009 | Very long name | 100+ chars | Success or error | ❌ | ❌ | ❌ | ❌ | ❌ TODO |
| SIG-010 | Unicode name | `signal 信号;` | Success or error | ❌ | ❌ | ❌ | ❌ | ❌ TODO |
| SIG-011 | Special chars in name | `signal player_died!;` | Parse error | ❌ | ❌ | ❌ | ❌ | ❌ TODO |
| SIG-012 | Many parameters | 10+ params | Success or error | ❌ | ❌ | ❌ | ❌ | ❌ TODO |

### emit_signal()

| Test ID | Scenario | Input | Expected Output | Unit Test (Runtime) | Unit Test (Compiler) | Integration Test | Headless Test | Status |
|---------|----------|-------|-----------------|---------------------|----------------------|------------------|---------------|--------|
| SIG-020 | Basic emission | `emit_signal("player_died");` | Callback invoked | ✅ test_emit_signal_in_function | ✅ test_emit_signal_valid | ✅ signals.ferris | ❌ | ✅ PASS |
| SIG-021 | With parameters | `emit_signal("health_changed", 100, 80);` | Callback with args | ✅ test_signal_emitter_callback_invoked | ❌ | ✅ signals.ferris | ❌ | ✅ PASS |
| SIG-022 | No parameters | `emit_signal("player_died");` | Callback invoked | ✅ test_emit_signal_with_no_params | ⚠️ Implied | ✅ signals.ferris | ❌ | ✅ PASS |
| SIG-023 | All types | i32, f32, bool, String, Vector2 | Correct types passed | ✅ test_signal_emitter_callback_all_types | ❌ | ⚠️ signals.ferris | ❌ | ✅ PASS |
| SIG-024 | No callback set | `emit_signal("x");` | Returns Nil | ✅ test_signal_emitter_without_callback | ❌ | ❌ | ❌ | ✅ PASS |
| SIG-025 | Callback error | Callback panics | Error handled | ✅ test_signal_emitter_error_handling | ❌ | ❌ | ❌ | ✅ PASS |
| SIG-026 | No signal name | `emit_signal();` | Error E501 | ✅ test_emit_signal_error_no_signal_name | ❌ | ❌ | ❌ | ✅ PASS |
| SIG-027 | Invalid name type | `emit_signal(123);` | Error E502 | ✅ test_emit_signal_error_invalid_signal_name_type | ❌ | ❌ | ❌ | ✅ PASS |
| SIG-028 | Undefined signal | `emit_signal("unknown");` | Error E403 | ❌ | ✅ test_emit_signal_undefined_error | ❌ | ❌ | ⚠️ PARTIAL |
| SIG-029 | Param count mismatch | `emit_signal("x", 1)` (expects 0) | Error E404 | ❌ | ✅ test_emit_signal_param_count_mismatch | ❌ | ❌ | ⚠️ PARTIAL |
| SIG-030 | Param type mismatch | `emit_signal("x", "str")` (expects i32) | Error E405 | ❌ | ✅ test_emit_signal_param_type_mismatch | ❌ | ❌ | ⚠️ PARTIAL |
| SIG-031 | Type coercion | `emit_signal("x", 42)` (expects f32) | Coerced to f32 | ❌ | ✅ test_emit_signal_type_coercion | ❌ | ❌ | ⚠️ PARTIAL |
| SIG-032 | In _ready | `emit_signal("x");` in _ready | Success | ❌ | ❌ | ⚠️ signals.ferris | ❌ | ⚠️ PARTIAL |
| SIG-033 | In _process | `emit_signal("x");` in _process | Success | ❌ | ❌ | ⚠️ signals.ferris | ❌ | ⚠️ PARTIAL |
| SIG-034 | In conditional | `if (x) emit_signal("y");` | Success | ❌ | ❌ | ⚠️ signals.ferris | ❌ | ⚠️ PARTIAL |
| SIG-035 | In loop | `for (...) emit_signal("x");` | Success | ❌ | ❌ | ❌ | ❌ | ❌ TODO |
| SIG-036 | Multiple emissions | Sequential calls | All invoked | ❌ | ❌ | ⚠️ signals.ferris | ❌ | ⚠️ PARTIAL |
| SIG-037 | Signal name variable | `let s = "x"; emit_signal(s);` | Error E205 (NOT SUPPORTED) | ✅ test_emit_signal_name_as_variable | ❌ | ❌ | ❌ | ✅ PASS |
| SIG-038 | Nested emission | Signal handler emits signal | Success or error | ❌ | ❌ | ❌ | ❌ | ❌ TODO |
| SIG-039 | Recursive emission | Signal A→B→A | Stack overflow? | ❌ | ❌ | ❌ | ❌ | ❌ TODO |

---

## Summary Statistics

**Node Queries**:

- Total Scenarios: 33
- ✅ PASS: 10 (30%) ⬆️ +4 from baseline
- ⚠️ PARTIAL: 14 (42%)
- ❌ TODO: 9 (27%) ⬇️ -4 from baseline
- 💥 FAIL: 0 (0%)

**Signals**:

- Total Scenarios: 31
- ✅ PASS: 10 (32%) ⬆️ +1 from baseline
- ⚠️ PARTIAL: 14 (45%)
- ❌ TODO: 7 (23%) ⬇️ -1 from baseline
- 💥 FAIL: 0 (0%)

**Overall**:

- Total Scenarios: 64
- ✅ PASS: 20 (31%) ⬆️ +5 from baseline (23%)
- ⚠️ PARTIAL: 28 (44%)
- ❌ TODO: 16 (25%) ⬇️ -5 from baseline (33%)
- 💥 FAIL: 0 (0%)

---

## Priority TODO List

### ✅ Completed (October 10, 2025)

1. ~~**NQ-008**: Empty string path test~~ - ✅ DONE (test_get_node_empty_string)
2. ~~**NQ-022**: get_parent() without callback~~ - ✅ DONE (test_get_parent_without_callback)
3. ~~**NQ-035**: has_node() without callback~~ - ✅ DONE (test_has_node_without_callback)
4. ~~**NQ-037**: has_node() with empty string~~ - ✅ DONE (test_has_node_empty_string)
5. ~~**SIG-037**: Signal name as variable~~ - ✅ DONE (documented as NOT SUPPORTED)

### High Priority (Blocking Production)

1. **NQ-045**: find_child() not found behavior
2. **NQ-044**: find_child() without callback
3. **NQ-010**: Path with special characters
4. **NQ-046**: find_child() with multiple matches
5. **NQ-023**: get_parent() at root node

### Medium Priority (Important Edge Cases)

6. **NQ-048**: find_child() case sensitivity
7. **NQ-049**: find_child() empty string
8. **SIG-035**: emit_signal() in loop
9. **SIG-038**: Nested signal emissions
10. **NQ-009**: Path with spaces

### Low Priority (Nice to Have)

11. **NQ-011**: Relative path support
12. **NQ-013**: Very long path handling
13. **NQ-014**: Unicode path support
14. **SIG-038**: Nested signal emissions
15. **SIG-039**: Recursive signal emissions

---

## Notes

- **PARTIAL** status indicates test exists but doesn't cover all aspects of the scenario
- **TODO** status indicates no test exists for this scenario
- Test IDs are referenced in test code comments where applicable
- Update this matrix when adding/modifying tests
- Review this matrix during PR reviews to ensure coverage doesn't regress
