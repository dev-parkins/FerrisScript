# Phase 3 Implementation Plan: Structured Test Protocol

**Date**: October 10, 2025  
**Branch**: `feature/v0.0.4-phase3-node-queries`  
**Status**: 🚧 **IN PLANNING**

---

## Executive Summary

Phase 3 introduces a structured test protocol system that extends the headless testing infrastructure with metadata-driven test definitions, error demo detection, and categorized reporting. This phase transforms the test harness from a simple pass/fail system into a comprehensive testing framework with rich diagnostics and clear test organization.

### Key Objectives

1. **Test Metadata Parsing** - Parse structured comment directives (`// TEST:`, `// EXPECT:`, `// ASSERT:`)
2. **Error Demo Detection** - Distinguish intentional error examples from real test failures
3. **Structured Assertion System** - Move beyond simple print markers to rich assertion blocks
4. **Categorized Reporting** - Organize tests by type (unit, integration, error_demo) with detailed statistics

---

## Problem Statement

### Current Limitations (Phase 2)

**Simple Print-Based Assertions**:
```ferrisscript
print("✓ Found Player node");  // Manual marker - no programmatic validation
```

**No Test Metadata**:
- Can't distinguish error demos from real failures
- No way to specify expected behaviors
- No test categorization or organization
- No multi-step test scenarios

**Limited Reporting**:
- Binary pass/fail per script
- No test categories or grouping
- Minimal diagnostic information
- Hard to understand why tests fail

### Phase 3 Solutions

**Structured Metadata**:
```ferrisscript
// TEST: node_query_basic_get_node
// CATEGORY: unit
// DESCRIPTION: Verify get_node() retrieves child nodes correctly
// EXPECT: success
// ASSERT: Found Player node
// ASSERT: Found UI node
```

**Error Demo Support**:
```ferrisscript
// TEST: error_handling_invalid_node
// CATEGORY: error_demo
// EXPECT: error
// EXPECT_ERROR: Node not found
```

**Rich Reporting**:
```
========================================
Test Summary - node_query_basic.ferris
========================================

Unit Tests:        3/3 passed
Integration Tests: 2/2 passed
Error Demos:       1/1 passed (expected errors)

Total: 6/6 passed ✓
```

---

## Architecture Design

### Component Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    TestHarness (Core)                        │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐  ┌───────────────┐  ┌─────────────────┐  │
│  │  Metadata    │  │   Assertion   │  │   Test Result   │  │
│  │   Parser     │  │   Validator   │  │   Aggregator    │  │
│  └──────────────┘  └───────────────┘  └─────────────────┘  │
│         │                  │                    │           │
│         ▼                  ▼                    ▼           │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              Enhanced OutputParser                    │  │
│  └──────────────────────────────────────────────────────┘  │
│         │                                                   │
│         ▼                                                   │
│  ┌──────────────────────────────────────────────────────┐  │
│  │         Categorized Report Generator                  │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Module Responsibilities

**1. MetadataParser** (`metadata_parser.rs`)
- Parse `// TEST:`, `// CATEGORY:`, `// EXPECT:`, `// ASSERT:` directives
- Extract test definitions from FerrisScript files
- Support multi-line metadata blocks
- Validate metadata syntax

**2. AssertionValidator** (`assertion_validator.rs`)
- Match expected assertions against actual output
- Support multiple assertion types (exact, contains, regex)
- Handle optional assertions (○ markers)
- Track assertion pass/fail state

**3. TestResultAggregator** (`test_result.rs`)
- Aggregate results by category (unit, integration, error_demo)
- Calculate statistics (total, passed, failed, skipped)
- Build structured result objects for reporting
- Track test execution metadata (duration, errors)

**4. Enhanced OutputParser** (extend existing)
- Parse assertion blocks from Godot output
- Extract error messages for error demos
- Support structured output formats
- Handle multi-line assertions

**5. CategoryReportGenerator** (`report_generator.rs`)
- Generate categorized test reports
- Format output with sections and summaries
- Colorized output for CLI
- Optional JSON/XML export for CI integration

---

## Metadata Syntax Specification

### Core Directives

#### TEST Directive
Defines a unique test identifier.

**Syntax**: `// TEST: <test_name>`

**Example**:
```ferrisscript
// TEST: node_query_basic_get_node
```

**Rules**:
- Must be first directive in test block
- Test name must be unique within file
- Snake_case naming convention
- No spaces in test name

---

#### CATEGORY Directive
Classifies test type for reporting.

**Syntax**: `// CATEGORY: <category_type>`

**Valid Categories**:
- `unit` - Unit tests (single function/feature)
- `integration` - Integration tests (multiple components)
- `error_demo` - Error demonstration examples

**Example**:
```ferrisscript
// CATEGORY: unit
```

**Default**: `unit` if not specified

---

#### DESCRIPTION Directive
Human-readable test description.

**Syntax**: `// DESCRIPTION: <description_text>`

**Example**:
```ferrisscript
// DESCRIPTION: Verify get_node() retrieves child nodes correctly
```

**Rules**:
- Optional but recommended
- Single line only
- Used in test reports for context

---

#### EXPECT Directive
Defines expected test outcome.

**Syntax**: 
- `// EXPECT: success` - Test should pass
- `// EXPECT: error` - Test should fail (for error demos)

**Example**:
```ferrisscript
// EXPECT: success
```

**Default**: `success` if not specified

---

#### EXPECT_ERROR Directive
Specifies expected error message (for error demos only).

**Syntax**: `// EXPECT_ERROR: <error_substring>`

**Example**:
```ferrisscript
// CATEGORY: error_demo
// EXPECT: error
// EXPECT_ERROR: Node not found: InvalidNode
```

**Rules**:
- Only valid with `EXPECT: error`
- Uses substring matching (not exact match)
- Case-sensitive

---

#### ASSERT Directive
Defines expected output assertions.

**Syntax**: `// ASSERT: <expected_output>`

**Example**:
```ferrisscript
// ASSERT: Found Player node
// ASSERT: Found UI node
// ASSERT: Got parent node
```

**Rules**:
- Can have multiple assertions per test
- Uses substring matching against Godot output
- Order-independent by default
- Fails if assertion not found in output

---

#### ASSERT_OPTIONAL Directive
Defines optional assertions (won't fail if missing).

**Syntax**: `// ASSERT_OPTIONAL: <expected_output>`

**Example**:
```ferrisscript
// ASSERT_OPTIONAL: DebugUI node exists (optional)
```

**Rules**:
- Won't cause test failure if not found
- Reported as "○" in output
- Useful for conditional features

---

### Complete Example

```ferrisscript
// TEST: node_query_basic_get_node
// CATEGORY: unit
// DESCRIPTION: Verify get_node() retrieves child nodes correctly
// EXPECT: success
// ASSERT: Found Player node
// ASSERT: Found UI node
// ASSERT: Got parent node
// ASSERT: Found OtherChild node

fn _ready() {
    // Scene Hierarchy:
    // TestRunner (Node2D)
    // └─ Main (FerrisScriptNode) ← self
    //    ├─ Player (Node2D)
    //    ├─ UI (Node2D)
    //    ├─ Camera2D (Camera2D)
    //    └─ Enemy (Node2D)
    //       └─ OtherChild (Node2D)
    
    let player = get_node("Player");
    if player != nil {
        print("✓ Found Player node");
    }
    
    let ui = get_node("UI");
    if ui != nil {
        print("✓ Found UI node");
    }
    
    let parent = get_parent();
    if parent != nil {
        print("✓ Got parent node");
    }
    
    let sibling = get_node("Enemy/OtherChild");
    if sibling != nil {
        print("✓ Found OtherChild node");
    }
}
```

---

### Error Demo Example

```ferrisscript
// TEST: node_query_error_invalid_path
// CATEGORY: error_demo
// DESCRIPTION: Demonstrate error handling for invalid node paths
// EXPECT: error
// EXPECT_ERROR: Node not found: InvalidNode

fn _ready() {
    // This should produce an error (intentional)
    let invalid = get_node("InvalidNode");
    print("This should not be reached");
}
```

---

## Implementation Tasks

### Task 1: Create MetadataParser Module

**File**: `crates/test_harness/src/metadata_parser.rs`

**Structures**:
```rust
pub struct TestMetadata {
    pub name: String,
    pub category: TestCategory,
    pub description: Option<String>,
    pub expect: TestExpectation,
    pub expect_error: Option<String>,
    pub assertions: Vec<Assertion>,
}

pub enum TestCategory {
    Unit,
    Integration,
    ErrorDemo,
}

pub enum TestExpectation {
    Success,
    Error,
}

pub struct Assertion {
    pub kind: AssertionKind,
    pub expected: String,
    pub found: bool,
}

pub enum AssertionKind {
    Required,
    Optional,
}
```

**Functions**:
```rust
pub fn parse_metadata(source: &str) -> Result<Vec<TestMetadata>, ParseError>;
pub fn extract_test_block(lines: &[&str]) -> Option<TestMetadata>;
pub fn parse_directive(line: &str) -> Option<Directive>;
```

**Acceptance Criteria**:
- ✅ Parses all directive types
- ✅ Handles multiple test blocks per file
- ✅ Validates metadata syntax
- ✅ Returns descriptive errors for invalid metadata
- ✅ Supports empty lines between directives

---

### Task 2: Extend OutputParser

**File**: `crates/test_harness/src/output_parser.rs`

**New Functions**:
```rust
pub fn validate_assertions(
    output: &str, 
    metadata: &TestMetadata
) -> Vec<AssertionResult>;

pub fn extract_error_message(output: &str) -> Option<String>;

pub fn match_expected_error(
    actual_error: &str, 
    expected_error: &str
) -> bool;
```

**Structures**:
```rust
pub struct AssertionResult {
    pub assertion: Assertion,
    pub found: bool,
    pub message: String,
}
```

**Acceptance Criteria**:
- ✅ Matches assertions against output
- ✅ Extracts error messages from Godot output
- ✅ Supports substring matching
- ✅ Handles optional assertions
- ✅ Returns detailed mismatch information

---

### Task 3: Create AssertionValidator

**File**: `crates/test_harness/src/assertion_validator.rs`

**Functions**:
```rust
pub fn validate_test(
    metadata: &TestMetadata,
    output: &str
) -> TestValidationResult;

pub fn check_assertions(
    assertions: &[Assertion],
    output: &str
) -> Vec<AssertionResult>;

pub fn validate_error_demo(
    expected_error: &str,
    actual_output: &str
) -> bool;
```

**Structures**:
```rust
pub struct TestValidationResult {
    pub test_name: String,
    pub passed: bool,
    pub category: TestCategory,
    pub assertion_results: Vec<AssertionResult>,
    pub error_match: Option<bool>,
    pub duration_ms: u64,
}
```

**Acceptance Criteria**:
- ✅ Validates all assertions
- ✅ Handles error demo validation
- ✅ Returns detailed results
- ✅ Tracks test timing
- ✅ Supports partial matches

---

### Task 4: Create TestResultAggregator

**File**: `crates/test_harness/src/test_result.rs`

**Structures**:
```rust
pub struct TestSuiteResult {
    pub file_name: String,
    pub results_by_category: HashMap<TestCategory, CategoryResults>,
    pub total_duration_ms: u64,
}

pub struct CategoryResults {
    pub category: TestCategory,
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub tests: Vec<TestValidationResult>,
}
```

**Functions**:
```rust
pub fn aggregate_results(
    results: Vec<TestValidationResult>
) -> TestSuiteResult;

pub fn calculate_statistics(
    results: &[TestValidationResult]
) -> CategoryResults;
```

**Acceptance Criteria**:
- ✅ Groups results by category
- ✅ Calculates statistics per category
- ✅ Tracks overall suite metrics
- ✅ Handles empty result sets
- ✅ Supports result filtering

---

### Task 5: Create Report Generator

**File**: `crates/test_harness/src/report_generator.rs`

**Functions**:
```rust
pub fn generate_report(suite_result: &TestSuiteResult) -> String;
pub fn format_category_section(results: &CategoryResults) -> String;
pub fn format_assertion_details(result: &TestValidationResult) -> String;
pub fn generate_summary_table(suite_result: &TestSuiteResult) -> String;
```

**Report Format**:
```
========================================
Test Results: node_query_basic.ferris
========================================

Unit Tests
----------
✓ node_query_basic_get_node
  ✓ Found Player node
  ✓ Found UI node
  ✓ Got parent node
  ✓ Found OtherChild node

✓ node_query_basic_get_parent
  ✓ Got parent TestRunner node
  ✓ Parent is correct type

Unit Tests: 2/2 passed ✓

Integration Tests
-----------------
(none)

Error Demos
-----------
✓ error_handling_invalid_node (expected error)
  ✓ Error message: "Node not found: InvalidNode"

Error Demos: 1/1 passed ✓

========================================
Summary
========================================
Total:  3 tests
Passed: 3 ✓
Failed: 0 ✗
Skipped: 0 ○

Total Time: 498ms
========================================
```

**Acceptance Criteria**:
- ✅ Categorized output sections
- ✅ Colorized CLI output
- ✅ Detailed assertion breakdown
- ✅ Summary statistics
- ✅ Timing information

---

### Task 6: Update TestRunner Integration

**File**: `crates/test_harness/src/test_runner.rs`

**Changes**:
1. Parse metadata before running test
2. Pass metadata to output parser
3. Use assertion validator for results
4. Generate categorized report
5. Return structured results

**New Flow**:
```rust
pub fn run_test(script_path: &Path) -> Result<TestSuiteResult> {
    // 1. Parse metadata from script
    let source = read_to_string(script_path)?;
    let metadata_list = MetadataParser::parse_metadata(&source)?;
    
    // 2. Build scene and run Godot
    let scene_path = build_scene(script_path)?;
    let output = run_godot(scene_path)?;
    
    // 3. Validate against metadata
    let mut results = Vec::new();
    for metadata in metadata_list {
        let result = validate_test(&metadata, &output);
        results.push(result);
    }
    
    // 4. Aggregate results
    let suite_result = aggregate_results(results);
    
    // 5. Generate report
    let report = generate_report(&suite_result);
    println!("{}", report);
    
    Ok(suite_result)
}
```

**Acceptance Criteria**:
- ✅ Parses metadata before execution
- ✅ Validates all tests in file
- ✅ Returns structured results
- ✅ Generates categorized reports
- ✅ Handles metadata parsing errors gracefully

---

### Task 7: Update Examples with Metadata

**Files to Update**:
1. `examples/node_query_basic.ferris`
2. `examples/node_query_validation.ferris`
3. `examples/node_query_search.ferris`
4. `examples/node_query_error_handling.ferris` (create if needed)

**Example Update**:
```ferrisscript
// TEST: node_query_basic_get_node
// CATEGORY: unit
// DESCRIPTION: Verify get_node() retrieves child nodes correctly
// EXPECT: success
// ASSERT: Found Player node
// ASSERT: Found UI node
// ASSERT: Got parent node
// ASSERT: Found OtherChild node

fn _ready() {
    // ... existing code ...
}
```

**New Error Demo Example**:
```ferrisscript
// TEST: error_demo_invalid_node
// CATEGORY: error_demo
// DESCRIPTION: Demonstrate error handling for invalid node paths
// EXPECT: error
// EXPECT_ERROR: Node not found

fn _ready() {
    let invalid = get_node("NonExistentNode");
    print("This should not execute");
}
```

**Acceptance Criteria**:
- ✅ All examples have TEST metadata
- ✅ Categories assigned correctly
- ✅ Assertions match output
- ✅ At least one error demo exists
- ✅ Descriptions are clear

---

### Task 8: Add CLI Flags for Reporting

**File**: `crates/test_harness/src/main.rs`

**New Flags**:
```rust
#[derive(Parser)]
struct Cli {
    // ... existing flags ...
    
    /// Output format (text, json, xml)
    #[arg(long, default_value = "text")]
    format: OutputFormat,
    
    /// Show only failed tests
    #[arg(long)]
    failures_only: bool,
    
    /// Filter by category (unit, integration, error_demo)
    #[arg(long)]
    category: Option<TestCategory>,
    
    /// Show detailed assertion breakdown
    #[arg(long)]
    show_assertions: bool,
}

enum OutputFormat {
    Text,
    Json,
    Xml,
}
```

**Usage Examples**:
```bash
# Show only failures
cargo run --bin ferris-test -- --all --failures-only

# Filter by category
cargo run --bin ferris-test -- --all --category unit

# JSON output for CI
cargo run --bin ferris-test -- --all --format json > results.json

# Detailed assertions
cargo run --bin ferris-test -- --script examples/node_query_basic.ferris --show-assertions
```

**Acceptance Criteria**:
- ✅ All flags implemented
- ✅ JSON export works
- ✅ Category filtering works
- ✅ Failures-only mode works
- ✅ Help text is clear

---

## Testing Strategy

### Unit Tests

**MetadataParser Tests**:
- ✅ Parse valid metadata blocks
- ✅ Handle invalid syntax gracefully
- ✅ Parse multiple test blocks
- ✅ Default values for optional fields
- ✅ Reject duplicate test names

**AssertionValidator Tests**:
- ✅ Match exact assertions
- ✅ Handle optional assertions
- ✅ Validate error demos
- ✅ Substring matching
- ✅ Case sensitivity

**ReportGenerator Tests**:
- ✅ Format categorized reports
- ✅ Handle empty categories
- ✅ Colorize output correctly
- ✅ Generate JSON output
- ✅ Summary statistics

### Integration Tests

**End-to-End Tests**:
1. Run test with valid metadata → expect structured report
2. Run error demo → expect error validation
3. Run multiple tests → expect categorized results
4. Run with filtering → expect correct subset

**Regression Tests**:
- Ensure Phase 1/2 examples still work
- Backward compatibility with simple markers
- Performance benchmarks (< 200ms per test)

---

## Success Criteria

### Must Have (Phase 3.0)
- ✅ Parse all core directives (TEST, CATEGORY, EXPECT, ASSERT)
- ✅ Validate assertions against output
- ✅ Detect and validate error demos
- ✅ Generate categorized reports
- ✅ Update all examples with metadata

### Should Have (Phase 3.1)
- ✅ JSON/XML export for CI integration
- ✅ Detailed assertion breakdown
- ✅ Performance metrics per test
- ✅ Category filtering

### Nice to Have (Future)
- ⏸️ Regex assertion matching
- ⏸️ Multi-line assertions
- ⏸️ Test dependencies/ordering
- ⏸️ Parallel test execution

---

## Timeline

**Total Estimate**: 4-6 hours

| Task | Estimate | Priority |
|------|----------|----------|
| MetadataParser module | 1h | High |
| AssertionValidator module | 1h | High |
| Extend OutputParser | 0.5h | High |
| TestResultAggregator | 0.5h | Medium |
| Report Generator | 1h | High |
| TestRunner integration | 1h | High |
| Update examples | 0.5h | High |
| CLI flags | 0.5h | Medium |
| Testing & validation | 1h | High |

---

## Dependencies

**Existing Phase 1/2 Components**:
- GodotRunner - No changes needed
- SceneBuilder - No changes needed
- OutputParser - Extend with new functions
- TestHarness - Integrate new modules
- CLI - Add new flags

**New Dependencies**:
- None required (all std library)

---

## Risk Assessment

**Low Risk**:
- Adding new modules (no breaking changes)
- Backward compatible with Phase 2 tests

**Medium Risk**:
- Parsing complexity if metadata syntax is ambiguous
- Mitigation: Strict syntax validation, clear error messages

**High Risk**:
- None identified

---

## Next Steps

1. **Create MetadataParser module** - Start with core parsing logic
2. **Add unit tests for parser** - Validate parsing edge cases
3. **Extend OutputParser** - Add assertion validation
4. **Create simple example** - Test end-to-end flow
5. **Build report generator** - Start with text format
6. **Integrate into TestRunner** - Wire up all components
7. **Update existing examples** - Add metadata to all tests
8. **Create error demo example** - Validate error detection
9. **Add CLI flags** - Enable filtering and formatting
10. **Final testing** - Run all tests, create completion report

---

**Document Created**: October 10, 2025  
**Author**: GitHub Copilot Agent  
**Review Status**: Ready for implementation
