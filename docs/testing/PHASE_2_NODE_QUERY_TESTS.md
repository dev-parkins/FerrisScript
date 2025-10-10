# Phase 2: Node Query Test Coverage

## Objectives

Phase 2 extends the headless testing infrastructure to comprehensively test all FerrisScript node query functions across multiple scenarios.

## Goals

1. **Test All 4 Node Query Examples**
   - `examples/node_query_basic.ferris` - Basic get_node/get_parent usage
   - `examples/node_query_validation.ferris` - has_node validation patterns
   - `examples/node_query_search.ferris` - find_child recursive search
   - `examples/node_query_error_handling.ferris` - Safe access patterns

2. **Fix Example Compatibility Issues**
   - Address method chaining limitations (E100 errors)
   - Update examples to work within current language constraints
   - Document workarounds for unsupported patterns

3. **Validate Scene Generation**
   - Ensure `parse_scene_requirements()` handles all hierarchy patterns
   - Verify complex nested structures (UI/HUD/HealthBar)
   - Test edge cases (optional nodes, deeply nested paths)

4. **Improve Test Detection**
   - Distinguish between intentional error-handling tests and real failures
   - Better pass/fail logic for defensive programming examples
   - Support test annotations (e.g., `// EXPECT_ERROR`)

## Known Issues from Phase 1

### 1. Method Chaining Not Supported (E100)

**Problem**: `get_parent().get_node("Child")` causes syntax error

**Example**: `node_query_basic.ferris` line 41
```ferrisscript
let sibling = get_parent().get_node("OtherChild");  // ❌ E100: Expected ;
```

**Workaround**: Use intermediate variables
```ferrisscript
let parent = get_parent();
let sibling = parent.get_node("OtherChild");  // ✅ Works
```

**Action**: Update example files to avoid chaining

### 2. Error Handling Tests Report as Failures

**Problem**: Tests designed to demonstrate error handling (✗ markers) count as failures

**Example**: `node_query_error_handling.ferris`
- Intentional: `✗ RequiredSystem node not found!` (testing missing node behavior)
- Actual output: Test marked as FAILED

**Solution**: Add test metadata system
- `// TEST: error_handling` - Mark as error demo
- `// EXPECT: fail` - Failure is expected
- Update pass/fail logic to check annotations

### 3. Scene Requirements Parser Limitations

**Current**: Only parses tree diagrams with specific format
```
// └─ Main
//   ├─ Player
//   ├─ UI
//   │  ├─ HUD
//   │  └─ HealthBar
```

**Missing**:
- Indentation-based hierarchy
- Node type specifications (Node2D, Control, etc.)
- Optional node markers handling

**Action**: Enhance parser in Phase 2

## Implementation Plan

### Task 1: Fix Method Chaining Issues

**Files to Update**:
- `examples/node_query_basic.ferris` (line 41)
- Any other examples using chained calls

**Changes**:
```ferrisscript
// OLD (broken):
let sibling = get_parent().get_node("OtherChild");

// NEW (working):
let parent = get_parent();
let sibling = parent.get_node("OtherChild");
```

### Task 2: Add Test Metadata System

**New Feature**: Test annotations in comments

**Syntax**:
```ferrisscript
// TEST: error_handling
// EXPECT: pass=5, fail=2, info=3
```

**Parser Changes**:
- `output_parser.rs`: Add `parse_test_metadata()` function
- Extract TEST and EXPECT directives from script comments
- Compare actual results against expectations

**Updated `TestResult`**:
```rust
pub struct TestResult {
    // ... existing fields ...
    pub expected_pass: Option<usize>,
    pub expected_fail: Option<usize>,
    pub is_error_demo: bool,
}
```

### Task 3: Run All Node Query Tests

**Test Matrix**:

| Script | Nodes | Complexity | Expected Outcome |
|--------|-------|-----------|------------------|
| `node_query_basic.ferris` | 5 (Main + 4 children) | Low | All pass after chain fix |
| `node_query_validation.ferris` | 6-8 | Medium | Mix of pass/fail (validation demo) |
| `node_query_search.ferris` | 8-10 (nested) | High | All pass (find_child tests) |
| `node_query_error_handling.ferris` | 5 + optional | Medium | Mixed (error demo) |

**Execution**:
```powershell
# Run individually
cargo run --release --bin ferris-test -- --script examples/node_query_basic.ferris
cargo run --release --bin ferris-test -- --script examples/node_query_validation.ferris
cargo run --release --bin ferris-test -- --script examples/node_query_search.ferris
cargo run --release --bin ferris-test -- --script examples/node_query_error_handling.ferris

# Run all node_query tests
cargo run --release --bin ferris-test -- --all --filter "node_query"
```

### Task 4: Enhance Scene Builder

**Improvements**:

1. **Better Node Type Detection**
   ```rust
   // Detect from comments:
   // - Control named "HUD"  → type = "Control"
   // - Node2D named "Player" → type = "Node2D"
   ```

2. **Optional Node Support**
   ```rust
   // (optional) or [optional] marker → don't fail if missing
   ```

3. **Indentation-Based Parsing**
   ```rust
   // Support both:
   // └─ Main        (tree format)
   //    - Player    (bullet format)
   ```

### Task 5: Improve Output Detection

**Current Limitation**: Only detects ✓ ✗ ○ symbols

**Enhancement**: Structured markers
```ferrisscript
fn _ready() {
    // [TEST_START: node_access]
    let player = get_node("Player");
    if has_node("Player") {
        print("[TEST_PASS: node_access] Player found");
    }
    // [TEST_END: node_access]
}
```

**Benefits**:
- Clear test boundaries
- Named test cases
- Better error isolation

## Success Criteria

- [ ] All 4 node_query examples run without compilation errors
- [ ] Scene generation handles nested hierarchies (3+ levels)
- [ ] Test harness distinguishes error demos from real failures
- [ ] Pass/fail counts match expectations for each script
- [ ] Documentation updated with test annotations

## Metrics

**Target Coverage**:
- 4 scripts tested ✅
- 40+ node query operations validated
- 3 hierarchy patterns verified (flat, nested, mixed)
- 2 error handling patterns demonstrated

**Performance**:
- All tests complete in <5 seconds
- Scene generation <50ms per script
- Output parsing <10ms per script

## Next Steps After Phase 2

**Phase 3**: Structured Test Protocol
- Implement `[FS_TEST]` marker blocks
- Test isolation and parallel execution
- Snapshot comparison

**Phase 4**: CI/CD Integration
- GitHub Actions workflow
- Automated testing on PR
- Coverage reporting

**Phase 5**: Advanced Features
- Benchmarking
- Watch mode
- Interactive runner
