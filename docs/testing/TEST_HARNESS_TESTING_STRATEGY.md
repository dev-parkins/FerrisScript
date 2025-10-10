# Test Harness Testing Strategy

## Overview

This document outlines a comprehensive strategy for testing and stress-testing the FerrisScript test harness itself. The goal is to ensure robustness, identify edge cases, and find opportunities for improvement.

## Executive Summary

**Purpose**: Validate the test harness can handle:
- Invalid inputs and malformed data
- Edge cases in metadata parsing and output validation
- Performance under load (large test suites)
- Platform-specific issues
- Error recovery and graceful degradation

**Approach**: Multi-layered testing strategy combining unit tests, integration tests, stress tests, and property-based testing.

---

## 1. Metadata Parser Testing

### 1.1 Invalid Metadata Formats

**Test Cases**:
```ferrisscript
// TEST: missing_category
// No CATEGORY directive - should default to "unit"

// TEST: invalid_category_value
// CATEGORY: not_a_valid_category
// Should error with InvalidCategory

// TEST: duplicate_test_names
// TEST: duplicate_test_names
// Should error with DuplicateTestName

// TEST: expect_error_without_error_expectation
// EXPECT: success
// EXPECT_ERROR: Some error
// Should error - can't have EXPECT_ERROR with EXPECT: success

// TEST: malformed_directive
// BADDIR ECTIVE: value
// Should be ignored or handled gracefully

// TEST: empty_test_name
// TEST:
// Should error with MissingTestName

// TEST: very_long_test_name_that_exceeds_reasonable_length_limits_and_might_cause_buffer_issues_or_display_problems_in_reports
// Should handle gracefully

// TEST: unicode_test_name_🚀_emoji
// DESCRIPTION: Test with emoji and unicode characters 日本語
// Should handle UTF-8 correctly

// TEST: special_chars_in_metadata
// DESCRIPTION: Contains "quotes" and 'apostrophes' and \backslashes\
// ASSERT: Output with "nested quotes"
// Should escape properly
```

**Implementation**:
- Add unit tests to `metadata_parser.rs::tests`
- Test parsing errors return appropriate ParseError variants
- Test validation catches inconsistencies

### 1.2 Edge Cases in Metadata Blocks

**Test Cases**:
- Multiple metadata blocks in single file (should work)
- Metadata block at end of file without code
- Metadata block with no assertions (valid for basic tests)
- Hundreds of assertions in single test (performance)
- Assertions with very long expected strings (1000+ chars)
- Mixed line endings (CRLF vs LF) in metadata
- Metadata with trailing whitespace
- Comments within metadata block structure

**Implementation**:
```rust
#[test]
fn test_parse_metadata_with_many_assertions() {
    let source = generate_test_with_n_assertions(1000);
    let result = MetadataParser::parse_metadata(&source);
    assert!(result.is_ok());
    assert_eq!(result.unwrap()[0].assertions.len(), 1000);
}

#[test]
fn test_parse_metadata_mixed_line_endings() {
    let source = "// TEST: mixed\r\n// CATEGORY: unit\n// EXPECT: success\r\n";
    let result = MetadataParser::parse_metadata(&source);
    assert!(result.is_ok());
}
```

---

## 2. Output Parser Testing

### 2.1 Malformed Godot Output

**Test Cases**:
```
// Empty output
stdout: ""
stderr: ""

// Only whitespace
stdout: "   \n\n   \t  \n"
stderr: ""

// Truncated output (interrupted execution)
stdout: "=== Test Started ===\n✓ Step 1\n✓ Step "
stderr: ""

// Binary/garbage data
stdout: "\x00\x01\xFF\xFE"
stderr: "Binary data"

// Extremely long lines (10K+ characters)
stdout: "A very long line..." + ("." * 10000)

// Missing markers entirely
stdout: "Test ran but forgot to print markers"

// Markers in unexpected format
stdout: "PASS test_name" (instead of ✓)
stdout: "[FAIL] test_name" (instead of ✗)

// Interleaved stdout/stderr
stdout: "Line 1\n"
stderr: "Error 1\n"
stdout: "Line 2\n"
stderr: "Error 2\n"

// UTF-8 encoding issues
stdout: "Invalid UTF-8: \xC3\x28"

// ANSI color codes in output
stdout: "\x1b[32m✓\x1b[0m Test passed"

// Output exceeds buffer limits (1MB+)
stdout: "Logging spam..." * 100000
```

**Implementation**:
- Add unit tests to `output_parser.rs::tests`
- Test error extraction handles empty/malformed input
- Test marker extraction with missing/corrupted markers
- Test assertion validation with incomplete output

### 2.2 Assertion Validation Edge Cases

**Test Cases**:
- Assertion text appears multiple times in output (should match first)
- Assertion text is substring of another (e.g., "Error" vs "ErrorCode")
- Case sensitivity in assertions
- Assertions with regex special characters
- Optional assertions that are present (should pass)
- Required assertions that appear but with wrong context
- Assertions with newlines or special formatting

**Implementation**:
```rust
#[test]
fn test_assertion_substring_ambiguity() {
    let parser = OutputParser::new();
    let assertions = vec![
        Assertion { kind: Required, expected: "Error".into() },
        Assertion { kind: Required, expected: "ErrorCode: 404".into() },
    ];
    let output = "ErrorCode: 404 occurred";
    let results = parser.validate_assertions(&assertions, output);
    assert!(results[0].found); // "Error" found (substring of ErrorCode)
    assert!(results[1].found); // "ErrorCode: 404" found
}
```

---

## 3. Scene Builder Testing

### 3.1 Invalid Scene Configurations

**Test Cases**:
- Missing script file
- Script file with syntax errors
- Invalid node names (special characters, spaces)
- Deeply nested node hierarchies (100+ levels)
- Very wide node hierarchies (1000+ siblings)
- Node paths with invalid characters
- Circular dependencies in scene structure
- Missing parent nodes
- Script paths with spaces, unicode, special chars

**Implementation**:
- Test scene generation with invalid inputs
- Test error messages are informative
- Test cleanup happens even on failures

### 3.2 File System Edge Cases

**Test Cases**:
- Read-only directories (can't write scene files)
- Insufficient disk space
- File name collisions
- Path length limits (Windows MAX_PATH = 260)
- Permissions issues
- Concurrent access to same scene file
- Files locked by other processes

---

## 4. Report Generator Testing

### 4.1 Formatting Edge Cases

**Test Cases**:
- Empty test suite (no tests)
- All tests passing (100% success)
- All tests failing (0% success)
- Mixed results across categories
- Very long test names in reports (wrapping)
- Very long assertion messages
- Unicode in test names and descriptions
- Test names with special characters
- Hundreds of tests (report readability)
- Colorization on/off
- Terminal width variations (80 vs 120 vs 200 chars)

**Implementation**:
```rust
#[test]
fn test_report_with_no_tests() {
    let suite = TestSuiteResult::new("empty.ferris".to_string());
    let generator = ReportGenerator::new();
    let report = generator.generate_report(&suite);
    assert!(report.contains("Total:  0 tests"));
}

#[test]
fn test_report_with_very_long_test_names() {
    let long_name = "a".repeat(500);
    let result = create_test_result(&long_name, true);
    // Test formatting handles this gracefully
}
```

### 4.2 Statistics Accuracy

**Test Cases**:
- Verify counts are correct across categories
- Verify percentages are accurate
- Verify timing information is reasonable
- Edge case: 0 duration tests
- Edge case: very long-running tests (hours)

---

## 5. Integration Testing

### 5.1 End-to-End Test Scenarios

**Test Cases**:

**Scenario 1: Happy Path**
```ferrisscript
// TEST: integration_happy_path
// CATEGORY: integration
// DESCRIPTION: Complete end-to-end test
// EXPECT: success
// ASSERT: Step 1 complete
// ASSERT: Step 2 complete
// ASSERT: Step 3 complete

fn _ready() {
    print("Step 1 complete");
    print("Step 2 complete");
    print("Step 3 complete");
}
```
Expected: Parse metadata → Build scene → Run test → Validate → Generate report → All pass

**Scenario 2: Partial Failure**
```ferrisscript
// TEST: integration_partial_fail
// CATEGORY: unit
// EXPECT: success
// ASSERT: Found node A
// ASSERT: Found node B
// ASSERT: Found node C

fn _ready() {
    print("Found node A");
    print("Found node B");
    // Missing "Found node C"
}
```
Expected: Report shows 2/3 assertions passed, test fails

**Scenario 3: Error Demo Success**
```ferrisscript
// TEST: integration_error_demo
// CATEGORY: error_demo
// EXPECT: error
// EXPECT_ERROR: Node not found

fn _ready() {
    let node = get_node("MissingNode"); // Intentional error
}
```
Expected: Parse metadata → Run test → Extract error → Match expected → Pass

**Scenario 4: Multiple Tests Per File**
```ferrisscript
// TEST: test_a
// CATEGORY: unit
// EXPECT: success
// ASSERT: Test A ran

// TEST: test_b  
// CATEGORY: unit
// EXPECT: success
// ASSERT: Test B ran

fn _ready() {
    print("Test A ran");
    print("Test B ran");
}
```
Expected: Both tests tracked separately, both pass

### 5.2 TestRunner Integration

**Test Cases**:
- Run single test file
- Run all tests in directory
- Run tests with filtering (category, name pattern)
- Run tests with different output formats
- Handle test timeouts
- Handle crashed tests (segfault, panic)
- Handle tests that never complete
- Parallel test execution (if implemented)

---

## 6. Stress Testing

### 6.1 Large Test Suites

**Performance Targets**:
- 100 test files: < 10 seconds total
- 1000 test files: < 2 minutes total
- Single file with 100 tests: < 5 seconds

**Test Cases**:
```bash
# Generate test suite
for i in {1..100}; do
  echo "// TEST: stress_test_$i" > "test_$i.ferris"
  echo "// CATEGORY: unit" >> "test_$i.ferris"
  echo "// EXPECT: success" >> "test_$i.ferris"
  echo "// ASSERT: Test $i passed" >> "test_$i.ferris"
  echo "fn _ready() { print(\"Test $i passed\"); }" >> "test_$i.ferris"
done

# Run stress test
time ferris-test --all
```

**Metrics to Track**:
- Total execution time
- Memory usage (peak and average)
- File I/O operations
- Godot startup overhead per test
- Scene generation time
- Report generation time

### 6.2 Memory Stress

**Test Cases**:
- Very large assertion lists (10,000 assertions)
- Very large output buffers (100MB+ of output)
- Many tests in single run (memory accumulation)
- Recursive test execution
- Memory leaks in test harness components

**Implementation**:
```rust
#[test]
fn test_memory_with_large_assertions() {
    let large_assertion_list = (0..10000)
        .map(|i| Assertion {
            kind: AssertionKind::Required,
            expected: format!("Assertion {}", i),
        })
        .collect::<Vec<_>>();
    
    // Verify parsing doesn't OOM
    // Verify validation completes
    // Verify memory is released
}
```

---

## 7. Platform-Specific Testing

### 7.1 Operating System Variations

**Platforms to Test**:
- Windows 11 (primary dev platform)
- Windows 10
- Ubuntu 22.04 LTS
- macOS (Monterey+)

**Platform-Specific Issues**:
- Path separators (\ vs /)
- Line endings (CRLF vs LF)
- Case sensitivity in file names
- Executable extensions (.exe on Windows)
- Permission models
- Maximum path length
- Symlink support
- File locking behavior

### 7.2 PowerShell vs Bash

**Test Cases**:
- Script runner works in both shells
- Path handling (Windows paths in PowerShell, Unix in Bash)
- Exit codes propagate correctly
- ANSI color codes work or are disabled appropriately
- Command quoting and escaping

---

## 8. Error Recovery Testing

### 8.1 Graceful Degradation

**Test Cases**:

**Godot Not Found**:
- Test harness detects Godot is missing
- Provides helpful error message
- Suggests installation steps
- Exits cleanly

**Invalid Configuration**:
- ferris-test.toml is malformed
- Provides helpful error message pointing to line
- Falls back to defaults where possible

**Scene Generation Fails**:
- Continue with other tests
- Report which tests couldn't run
- Don't crash entire test suite

**Timeout Handling**:
- Test runs forever (infinite loop)
- Test harness kills it after timeout
- Reports timeout error
- Continues with other tests

### 8.2 Error Message Quality

**Quality Criteria**:
- Error messages are clear and actionable
- Include file names and line numbers
- Suggest fixes where possible
- Don't expose internal implementation details
- Provide context (what was being attempted)

**Test Cases**:
```rust
#[test]
fn test_error_message_quality() {
    let result = parse_metadata("// TEST:");
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("TEST"));
    assert!(err_msg.contains("name"));
    // Error should explain what's missing
}
```

---

## 9. Regression Testing

### 9.1 Test Suite for Test Harness

**Strategy**: Maintain a comprehensive test suite that runs on every commit.

**Components**:
1. **Unit Tests** (current: 38 tests)
   - Expand to 100+ tests covering all edge cases
   
2. **Integration Tests**
   - Full end-to-end scenarios
   - Real Godot execution (in CI)
   - Example files as test fixtures
   
3. **Snapshot Tests**
   - Capture report output for known test suites
   - Detect unintended formatting changes
   - Store in `tests/snapshots/`

4. **Performance Benchmarks**
   - Track execution time trends
   - Alert on regressions > 20%
   - Use Criterion for Rust benchmarks

### 9.2 Continuous Integration

**CI Pipeline**:
```yaml
test_harness_tests:
  - cargo test -p ferrisscript_test_harness
  - cargo clippy -p ferrisscript_test_harness
  - cargo bench -p ferrisscript_test_harness (baseline)
  
integration_tests:
  - ./run-tests.ps1 --all (Windows)
  - ./run-tests.sh --all (Linux/macOS)
  
stress_tests:
  - ./generate_stress_suite.sh 100
  - time ./run-tests.sh --all
  - assert time < 10s
```

---

## 10. Property-Based Testing

### 10.1 QuickCheck/PropTest Integration

**Properties to Test**:

**Property 1: Parsing Idempotency**
```rust
#[test]
fn prop_metadata_parse_serialize_roundtrip() {
    // Given any valid TestMetadata
    // When serialized to string and parsed back
    // Then should equal original
}
```

**Property 2: Output Validation Consistency**
```rust
#[test]
fn prop_assertion_validation_is_consistent() {
    // Given any assertion and output string
    // If assertion.expected is substring of output
    // Then validation should find it
}
```

**Property 3: Report Generation Determinism**
```rust
#[test]
fn prop_report_generation_is_deterministic() {
    // Given same TestSuiteResult
    // When generate_report() called multiple times
    // Then output should be identical
}
```

---

## 11. Testing Priorities

### Phase 1 (Immediate - Current Sprint)
- [x] Basic unit tests for all modules (38 tests)
- [ ] Edge case tests for metadata parser (10+ tests)
- [ ] Edge case tests for output parser (10+ tests)
- [ ] Integration test for happy path
- [ ] Integration test for error demo

### Phase 2 (Near-term)
- [ ] Report generator edge cases
- [ ] Scene builder error handling
- [ ] Large test suite stress test (100 files)
- [ ] Platform testing (Windows + Linux)
- [ ] Error message quality audit

### Phase 3 (Future)
- [ ] Memory stress testing
- [ ] Performance benchmarking suite
- [ ] Property-based testing implementation
- [ ] Snapshot testing for reports
- [ ] Concurrent execution testing

---

## 12. Test Implementation Checklist

### Current Coverage (38/500+ target tests)
- ✅ metadata_parser: 9 tests
- ✅ output_parser: 11 tests
- ✅ report_generator: 15 tests
- ✅ scene_builder: 2 tests
- ✅ godot_cli: 1 test
- ❌ test_runner: 0 tests (needs coverage)
- ❌ test_config: 0 tests (needs coverage)
- ❌ Integration tests: 0 tests (needs coverage)
- ❌ Stress tests: 0 tests (needs implementation)

### Recommended Next Tests (Priority Order)

1. **Metadata Parser Edge Cases** (metadata_parser.rs)
   ```rust
   test_parse_metadata_with_1000_assertions()
   test_parse_metadata_mixed_line_endings()
   test_parse_unicode_in_metadata()
   test_parse_empty_metadata_block()
   test_parse_malformed_directives()
   test_parse_duplicate_directives()
   test_parse_very_long_test_names()
   test_parse_special_chars_in_assertions()
   ```

2. **Output Parser Robustness** (output_parser.rs)
   ```rust
   test_extract_error_from_empty_output()
   test_extract_error_from_truncated_output()
   test_extract_markers_with_ansi_codes()
   test_validate_assertions_with_partial_matches()
   test_validate_with_very_large_output()
   test_extract_error_with_multiple_errors()
   ```

3. **Report Generator Formatting** (report_generator.rs)
   ```rust
   test_report_with_1000_tests()
   test_report_with_very_long_names()
   test_report_with_unicode_everywhere()
   test_report_colorization_edge_cases()
   test_summary_statistics_accuracy()
   ```

4. **Integration Tests** (new file: tests/integration_tests.rs)
   ```rust
   test_end_to_end_happy_path()
   test_end_to_end_partial_failure()
   test_end_to_end_error_demo()
   test_end_to_end_multiple_tests_per_file()
   test_end_to_end_with_real_godot()
   ```

---

## 13. Fuzzing Strategy

### 13.1 AFL/cargo-fuzz Integration

**Targets for Fuzzing**:
1. Metadata parser (parse arbitrary strings)
2. Output parser (parse arbitrary Godot output)
3. Scene generator (fuzz scene structures)

**Implementation**:
```toml
[dependencies]
cargo-fuzz = "0.11"

# fuzz/fuzz_targets/metadata_parser.rs
#![no_main]
use libfuzzer_sys::fuzz_target;
use ferrisscript_test_harness::MetadataParser;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = MetadataParser::parse_metadata(s);
    }
});
```

---

## 14. Documentation Testing

### 14.1 Example Code Validation

**Strategy**: All code examples in documentation should:
- Actually compile
- Actually run
- Produce expected output

**Implementation**:
- Use `#[doc]` tests in Rust
- Add examples to test suite
- Validate README examples in CI

---

## 15. Monitoring and Metrics

### 15.1 Test Health Metrics

**Metrics to Track**:
- Total test count (target: 500+)
- Test coverage (target: 90%+)
- Average test execution time (target: <100ms)
- Flaky test rate (target: <1%)
- Test failure rate over time
- Time to fix broken tests

### 15.2 Performance Metrics

**Metrics to Track**:
- Test harness startup time
- Metadata parsing time per file
- Scene generation time per test
- Godot startup overhead
- Report generation time
- Total test suite execution time

---

## Conclusion

This comprehensive testing strategy provides a roadmap for ensuring the test harness is robust, reliable, and performant. Implementation should be prioritized based on risk and impact, starting with edge cases in core components (metadata parser, output parser) and expanding to integration and stress testing.

**Next Steps**:
1. Implement Phase 1 tests (metadata and output parser edge cases)
2. Create integration test framework
3. Set up stress testing infrastructure
4. Monitor test health metrics
5. Iterate based on findings
