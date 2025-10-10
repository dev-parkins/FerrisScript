use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::metadata_parser::{Assertion, AssertionKind, TestExpectation, TestMetadata};

/// Parses structured output from Godot test runs
pub struct OutputParser {
    marker_regex: Regex,
}

/// Type of test marker
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TestMarkerKind {
    Start,
    Pass,
    Fail,
    End,
    Info,
}

/// A structured test marker found in output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestMarker {
    pub kind: TestMarkerKind,
    pub test_name: String,
    pub message: Option<String>,
}

/// Complete test results parsed from output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResults {
    pub script_name: String,
    pub passed: usize,
    pub failed: usize,
    pub markers: Vec<TestMarker>,
    pub errors: Vec<String>,
    pub stdout: String,
    pub stderr: String,
}

/// Result of validating a single assertion
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssertionResult {
    pub expected: String,
    pub kind: AssertionKind,
    pub found: bool,
    pub message: String,
}

impl AssertionResult {
    pub fn passed(&self) -> bool {
        self.found || self.kind == AssertionKind::Optional
    }
}

/// Validation result for a test with metadata
#[derive(Debug, Clone)]
pub struct TestValidationResult {
    pub test_name: String,
    pub passed: bool,
    pub assertion_results: Vec<AssertionResult>,
    pub expected_error_matched: Option<bool>,
    pub actual_error: Option<String>,
}

impl OutputParser {
    pub fn new() -> Self {
        Self {
            // Match: [FS_TEST] START|PASS|FAIL|END test_name optional_message
            marker_regex: Regex::new(r"\[FS_TEST\]\s+(\w+)\s+(\w+)(?:\s+(.+))?").unwrap(),
        }
    }

    /// Parse test output and extract structured results
    pub fn parse(&self, script_name: &str, stdout: &str, stderr: &str) -> TestResults {
        let markers = self.extract_markers(stdout);
        let errors = self.extract_errors(stderr);

        let (passed, failed) = self.count_results(&markers);

        TestResults {
            script_name: script_name.to_string(),
            passed,
            failed,
            markers,
            errors,
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
        }
    }

    /// Extract structured test markers
    fn extract_markers(&self, stdout: &str) -> Vec<TestMarker> {
        let mut markers = Vec::new();

        for line in stdout.lines() {
            if let Some(captures) = self.marker_regex.captures(line) {
                let kind_str = captures.get(1).map(|m| m.as_str()).unwrap_or("");
                let test_name = captures
                    .get(2)
                    .map(|m| m.as_str())
                    .unwrap_or("")
                    .to_string();
                let message = captures.get(3).map(|m| m.as_str().to_string());

                let kind = match kind_str {
                    "START" => TestMarkerKind::Start,
                    "PASS" => TestMarkerKind::Pass,
                    "FAIL" => TestMarkerKind::Fail,
                    "END" => TestMarkerKind::End,
                    "INFO" => TestMarkerKind::Info,
                    _ => continue,
                };

                markers.push(TestMarker {
                    kind,
                    test_name,
                    message,
                });
            }
        }

        // Also detect implicit pass/fail from ✓ and ✗ markers
        for line in stdout.lines() {
            if line.contains("✓") || line.contains("Found") {
                // Implicit pass
                if let Some(test_name) = self.extract_test_name_from_message(line) {
                    markers.push(TestMarker {
                        kind: TestMarkerKind::Pass,
                        test_name,
                        message: Some(line.trim().to_string()),
                    });
                }
            } else if line.contains("✗") || line.contains("ERROR:") {
                // Implicit fail
                if let Some(test_name) = self.extract_test_name_from_message(line) {
                    markers.push(TestMarker {
                        kind: TestMarkerKind::Fail,
                        test_name,
                        message: Some(line.trim().to_string()),
                    });
                }
            }
        }

        markers
    }

    /// Extract error messages from stderr
    fn extract_errors(&self, stderr: &str) -> Vec<String> {
        stderr
            .lines()
            .filter(|line| {
                line.contains("ERROR")
                    || line.contains("Error")
                    || line.contains("FATAL")
                    || line.contains("panic")
            })
            .map(|s| s.to_string())
            .collect()
    }

    /// Count pass/fail results
    fn count_results(&self, markers: &[TestMarker]) -> (usize, usize) {
        let passed = markers
            .iter()
            .filter(|m| m.kind == TestMarkerKind::Pass)
            .count();
        let failed = markers
            .iter()
            .filter(|m| m.kind == TestMarkerKind::Fail)
            .count();
        (passed, failed)
    }

    /// Extract test name from implicit marker message
    fn extract_test_name_from_message(&self, line: &str) -> Option<String> {
        // Try to extract meaningful test name from message
        if line.contains("Found") {
            // "✓ Found Player node" -> "found_player_node"
            let name = line
                .replace("✓", "")
                .replace("Found", "")
                .replace("node", "")
                .trim()
                .to_lowercase()
                .replace(' ', "_");
            Some(format!("found_{}", name))
        } else if line.contains("not found") {
            let name = line
                .split("not found")
                .next()
                .unwrap_or("unknown")
                .replace("✗", "")
                .trim()
                .to_lowercase()
                .replace(' ', "_");
            Some(format!("missing_{}", name))
        } else {
            Some("unnamed_test".to_string())
        }
    }

    /// Check if script loaded successfully
    pub fn script_loaded_successfully(&self, stdout: &str) -> bool {
        stdout.contains("Successfully loaded FerrisScript:")
    }

    /// Check if script encountered compilation errors
    pub fn has_compilation_errors(&self, stdout: &str, stderr: &str) -> bool {
        stdout.contains("Failed to compile") || stderr.contains("Error")
    }

    /// Validate test metadata against actual output
    pub fn validate_test(
        &self,
        metadata: &TestMetadata,
        stdout: &str,
        stderr: &str,
    ) -> TestValidationResult {
        let assertion_results = self.validate_assertions(&metadata.assertions, stdout);

        // Check if any required assertions failed
        let all_required_passed = assertion_results
            .iter()
            .filter(|r| r.kind == AssertionKind::Required)
            .all(|r| r.found);

        // Check error expectation
        let (expected_error_matched, actual_error) = if metadata.expect == TestExpectation::Error {
            let error = self.extract_error_message(stdout, stderr);
            let matched = if let Some(ref expected) = metadata.expect_error {
                error
                    .as_ref()
                    .map(|e| e.contains(expected))
                    .unwrap_or(false)
            } else {
                // Just check that an error occurred
                error.is_some()
            };
            (Some(matched), error)
        } else {
            (None, None)
        };

        let passed = if metadata.expect == TestExpectation::Error {
            // For error demos, pass if error occurred and matched (or no specific error expected)
            expected_error_matched.unwrap_or(false)
        } else {
            // For success tests, pass if all required assertions passed and no unexpected errors
            all_required_passed && actual_error.is_none()
        };

        TestValidationResult {
            test_name: metadata.name.clone(),
            passed,
            assertion_results,
            expected_error_matched,
            actual_error,
        }
    }

    /// Validate all assertions against output
    pub fn validate_assertions(
        &self,
        assertions: &[Assertion],
        output: &str,
    ) -> Vec<AssertionResult> {
        assertions
            .iter()
            .map(|assertion| self.validate_single_assertion(assertion, output))
            .collect()
    }

    /// Validate a single assertion
    fn validate_single_assertion(&self, assertion: &Assertion, output: &str) -> AssertionResult {
        let found = output.contains(&assertion.expected);

        let message = if found {
            format!("✓ {}", assertion.expected)
        } else if assertion.kind == AssertionKind::Optional {
            format!("○ {} (optional - not found)", assertion.expected)
        } else {
            format!("✗ {} (not found)", assertion.expected)
        };

        AssertionResult {
            expected: assertion.expected.clone(),
            kind: assertion.kind.clone(),
            found,
            message,
        }
    }

    /// Extract error message from output
    pub fn extract_error_message(&self, stdout: &str, stderr: &str) -> Option<String> {
        // Check stderr first
        if !stderr.is_empty() {
            for line in stderr.lines() {
                if line.contains("ERROR") || line.contains("Error") || line.contains("error") {
                    return Some(line.trim().to_string());
                }
            }
        }

        // Check stdout for error patterns
        for line in stdout.lines() {
            if line.contains("ERROR:") || line.contains("Error:") || line.contains("FATAL") {
                return Some(line.trim().to_string());
            }
            // FerrisScript specific errors
            if line.contains("Node not found") || line.contains("Failed to") {
                return Some(line.trim().to_string());
            }
        }

        None
    }

    /// Check if expected error matches actual error (substring match)
    pub fn match_expected_error(actual_error: &str, expected_error: &str) -> bool {
        actual_error.contains(expected_error)
    }
}

impl Default for OutputParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_extraction() {
        let stdout = "[FS_TEST] PASS test_one\n[FS_TEST] FAIL test_two Expected Player\n";
        let parser = OutputParser::new();
        let markers = parser.extract_markers(stdout);

        assert_eq!(markers.len(), 2);
        assert_eq!(markers[0].kind, TestMarkerKind::Pass);
        assert_eq!(markers[1].kind, TestMarkerKind::Fail);
    }

    #[test]
    fn test_implicit_markers() {
        let stdout = "✓ Found Player node\n✗ Player node not found!";
        let parser = OutputParser::new();
        let markers = parser.extract_markers(stdout);

        assert!(markers.iter().any(|m| m.kind == TestMarkerKind::Pass));
        assert!(markers.iter().any(|m| m.kind == TestMarkerKind::Fail));
    }

    #[test]
    fn test_validate_assertions_all_found() {
        let parser = OutputParser::new();
        let assertions = vec![
            Assertion::required("Found Player node".to_string()),
            Assertion::required("Found UI node".to_string()),
        ];
        let output = "✓ Found Player node\n✓ Found UI node";

        let results = parser.validate_assertions(&assertions, output);

        assert_eq!(results.len(), 2);
        assert!(results[0].found);
        assert!(results[1].found);
        assert!(results[0].passed());
        assert!(results[1].passed());
    }

    #[test]
    fn test_validate_assertions_some_missing() {
        let parser = OutputParser::new();
        let assertions = vec![
            Assertion::required("Found Player node".to_string()),
            Assertion::required("Found Enemy node".to_string()),
        ];
        let output = "✓ Found Player node";

        let results = parser.validate_assertions(&assertions, output);

        assert_eq!(results.len(), 2);
        assert!(results[0].found);
        assert!(!results[1].found);
        assert!(results[0].passed());
        assert!(!results[1].passed());
    }

    #[test]
    fn test_validate_optional_assertions() {
        let parser = OutputParser::new();
        let assertions = vec![
            Assertion::required("Found Player node".to_string()),
            Assertion::optional("Found DebugUI node".to_string()),
        ];
        let output = "✓ Found Player node";

        let results = parser.validate_assertions(&assertions, output);

        assert_eq!(results.len(), 2);
        assert!(results[0].found);
        assert!(!results[1].found);
        assert!(results[0].passed()); // Required and found
        assert!(results[1].passed()); // Optional and not found - still passes
    }

    #[test]
    fn test_extract_error_message_from_stderr() {
        let parser = OutputParser::new();
        let stderr = "ERROR: Node not found: InvalidNode";
        let error = parser.extract_error_message("", stderr);

        assert!(error.is_some());
        assert!(error.unwrap().contains("Node not found"));
    }

    #[test]
    fn test_extract_error_message_from_stdout() {
        let parser = OutputParser::new();
        let stdout = "Some output\nERROR: Node not found: InvalidNode\nMore output";
        let error = parser.extract_error_message(stdout, "");

        assert!(error.is_some());
        assert!(error.unwrap().contains("Node not found"));
    }

    #[test]
    fn test_match_expected_error() {
        assert!(OutputParser::match_expected_error(
            "ERROR: Node not found: InvalidNode",
            "Node not found"
        ));
        assert!(OutputParser::match_expected_error(
            "ERROR: Node not found: InvalidNode",
            "InvalidNode"
        ));
        assert!(!OutputParser::match_expected_error(
            "ERROR: Node not found: InvalidNode",
            "Player"
        ));
    }

    #[test]
    fn test_validate_success_test() {
        use crate::metadata_parser::TestCategory;

        let parser = OutputParser::new();
        let mut metadata = TestMetadata::new("test_success".to_string());
        metadata.category = TestCategory::Unit;
        metadata.expect = TestExpectation::Success;
        metadata
            .assertions
            .push(Assertion::required("Found Player node".to_string()));
        metadata
            .assertions
            .push(Assertion::required("Found UI node".to_string()));

        let stdout = "✓ Found Player node\n✓ Found UI node";
        let result = parser.validate_test(&metadata, stdout, "");

        assert!(result.passed);
        assert_eq!(result.assertion_results.len(), 2);
        assert!(result.assertion_results.iter().all(|r| r.found));
    }

    #[test]
    fn test_validate_error_demo() {
        use crate::metadata_parser::TestCategory;

        let parser = OutputParser::new();
        let mut metadata = TestMetadata::new("test_error".to_string());
        metadata.category = TestCategory::ErrorDemo;
        metadata.expect = TestExpectation::Error;
        metadata.expect_error = Some("Node not found".to_string());

        let stdout = "ERROR: Node not found: InvalidNode";
        let result = parser.validate_test(&metadata, stdout, "");

        assert!(result.passed);
        assert!(result.expected_error_matched.unwrap());
        assert!(result.actual_error.is_some());
    }

    #[test]
    fn test_validate_error_demo_mismatch() {
        use crate::metadata_parser::TestCategory;

        let parser = OutputParser::new();
        let mut metadata = TestMetadata::new("test_error".to_string());
        metadata.category = TestCategory::ErrorDemo;
        metadata.expect = TestExpectation::Error;
        metadata.expect_error = Some("Type error".to_string());

        let stdout = "ERROR: Node not found: InvalidNode";
        let result = parser.validate_test(&metadata, stdout, "");

        assert!(!result.passed); // Expected "Type error" but got "Node not found"
        assert!(!result.expected_error_matched.unwrap());
    }
}
