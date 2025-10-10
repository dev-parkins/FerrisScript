//! Report generation for structured test results.
//!
//! This module provides functionality to generate categorized test reports
//! with detailed assertion breakdowns, error demo results, and summary statistics.

use crate::metadata_parser::TestCategory;
use crate::output_parser::TestValidationResult;
use std::time::Duration;

/// Results grouped by test category.
#[derive(Debug)]
pub struct CategoryResults {
    pub unit: Vec<TestValidationResult>,
    pub integration: Vec<TestValidationResult>,
    pub error_demo: Vec<TestValidationResult>,
}

impl CategoryResults {
    /// Create empty category results.
    pub fn new() -> Self {
        Self {
            unit: Vec::new(),
            integration: Vec::new(),
            error_demo: Vec::new(),
        }
    }

    /// Add a test result to the appropriate category.
    pub fn add(&mut self, result: TestValidationResult, category: TestCategory) {
        match category {
            TestCategory::Unit => self.unit.push(result),
            TestCategory::Integration => self.integration.push(result),
            TestCategory::ErrorDemo => self.error_demo.push(result),
        }
    }

    /// Get total test count across all categories.
    pub fn total_count(&self) -> usize {
        self.unit.len() + self.integration.len() + self.error_demo.len()
    }

    /// Get total passed count across all categories.
    pub fn passed_count(&self) -> usize {
        self.unit.iter().filter(|r| r.passed).count()
            + self.integration.iter().filter(|r| r.passed).count()
            + self.error_demo.iter().filter(|r| r.passed).count()
    }

    /// Get total failed count across all categories.
    pub fn failed_count(&self) -> usize {
        self.total_count() - self.passed_count()
    }
}

impl Default for CategoryResults {
    fn default() -> Self {
        Self::new()
    }
}

/// Complete test suite results with timing information.
#[derive(Debug)]
pub struct TestSuiteResult {
    pub file_name: String,
    pub results: CategoryResults,
    pub duration: Option<Duration>,
}

impl TestSuiteResult {
    /// Create a new test suite result.
    pub fn new(file_name: String) -> Self {
        Self {
            file_name,
            results: CategoryResults::new(),
            duration: None,
        }
    }

    /// Set the execution duration.
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Check if all tests passed.
    pub fn all_passed(&self) -> bool {
        self.results.failed_count() == 0
    }
}

/// Report generator for test results.
pub struct ReportGenerator {
    show_assertions: bool,
    colorized: bool,
}

impl ReportGenerator {
    /// Create a new report generator with default settings.
    pub fn new() -> Self {
        Self {
            show_assertions: true,
            colorized: true,
        }
    }

    /// Set whether to show detailed assertion breakdown.
    pub fn with_assertions(mut self, show: bool) -> Self {
        self.show_assertions = show;
        self
    }

    /// Set whether to use colorized output.
    pub fn with_colors(mut self, colorized: bool) -> Self {
        self.colorized = colorized;
        self
    }

    /// Generate a complete test report.
    pub fn generate_report(&self, suite: &TestSuiteResult) -> String {
        let mut output = String::new();

        // Header
        output.push_str(&self.format_header(&suite.file_name));
        output.push('\n');

        // Unit tests section
        if !suite.results.unit.is_empty() {
            output.push_str(&self.format_category_section("Unit Tests", &suite.results.unit));
            output.push('\n');
        }

        // Integration tests section
        if !suite.results.integration.is_empty() {
            output.push_str(
                &self.format_category_section("Integration Tests", &suite.results.integration),
            );
            output.push('\n');
        }

        // Error demos section
        if !suite.results.error_demo.is_empty() {
            output.push_str(&self.format_error_demo_section(&suite.results.error_demo));
            output.push('\n');
        }

        // Summary
        output.push_str(&self.format_summary(suite));

        output
    }

    /// Format the report header.
    fn format_header(&self, file_name: &str) -> String {
        let separator = "=".repeat(60);
        format!("{}\nTest Results: {}\n{}", separator, file_name, separator)
    }

    /// Format a category section (unit or integration tests).
    fn format_category_section(&self, title: &str, results: &[TestValidationResult]) -> String {
        let mut output = String::new();

        output.push_str(&format!("\n{}\n", title));
        output.push_str(&"-".repeat(title.len()));
        output.push('\n');

        for result in results {
            output.push_str(&self.format_test_result(result));
        }

        let passed = results.iter().filter(|r| r.passed).count();
        let total = results.len();
        let status = if passed == total {
            self.colorize("✓", Color::Green)
        } else {
            self.colorize("✗", Color::Red)
        };

        output.push_str(&format!(
            "\n{}: {}/{} passed {}\n",
            title, passed, total, status
        ));

        output
    }

    /// Format the error demo section.
    fn format_error_demo_section(&self, results: &[TestValidationResult]) -> String {
        let mut output = String::new();

        output.push_str("\nError Demos\n");
        output.push_str("-----------\n");

        for result in results {
            let symbol = if result.passed {
                self.colorize("✓", Color::Green)
            } else {
                self.colorize("✗", Color::Red)
            };

            output.push_str(&format!(
                "{} {} (expected error)\n",
                symbol, result.test_name
            ));

            if self.show_assertions {
                if let Some(matched) = result.expected_error_matched {
                    if matched {
                        if let Some(error) = &result.actual_error {
                            output.push_str(&format!(
                                "  {} Error message: \"{}\"\n",
                                self.colorize("✓", Color::Green),
                                error
                            ));
                        }
                    } else {
                        output.push_str(&format!(
                            "  {} Expected error not found\n",
                            self.colorize("✗", Color::Red)
                        ));
                        if let Some(error) = &result.actual_error {
                            output.push_str(&format!("  Actual error: \"{}\"\n", error));
                        }
                    }
                }
            }
        }

        let passed = results.iter().filter(|r| r.passed).count();
        let total = results.len();
        let status = if passed == total {
            self.colorize("✓", Color::Green)
        } else {
            self.colorize("✗", Color::Red)
        };

        output.push_str(&format!(
            "\nError Demos: {}/{} passed {}\n",
            passed, total, status
        ));

        output
    }

    /// Format a single test result with optional assertion details.
    fn format_test_result(&self, result: &TestValidationResult) -> String {
        let mut output = String::new();

        let symbol = if result.passed {
            self.colorize("✓", Color::Green)
        } else {
            self.colorize("✗", Color::Red)
        };

        output.push_str(&format!("{} {}\n", symbol, result.test_name));

        if self.show_assertions && !result.assertion_results.is_empty() {
            for assertion in &result.assertion_results {
                let ass_symbol = if assertion.passed() {
                    self.colorize("✓", Color::Green)
                } else {
                    self.colorize("✗", Color::Red)
                };
                output.push_str(&format!("  {} {}\n", ass_symbol, assertion.message));
            }
        }

        output
    }

    /// Format the summary section.
    fn format_summary(&self, suite: &TestSuiteResult) -> String {
        let mut output = String::new();

        let separator = "=".repeat(60);
        output.push_str(&format!("\n{}\n", separator));
        output.push_str("Summary\n");
        output.push_str(&format!("{}\n", separator));

        let total = suite.results.total_count();
        let passed = suite.results.passed_count();
        let failed = suite.results.failed_count();

        output.push_str(&format!("Total:  {} tests\n", total));
        output.push_str(&format!(
            "Passed: {} {}\n",
            passed,
            self.colorize("✓", Color::Green)
        ));
        output.push_str(&format!(
            "Failed: {} {}\n",
            failed,
            if failed > 0 {
                self.colorize("✗", Color::Red)
            } else {
                "".to_string()
            }
        ));

        if let Some(duration) = suite.duration {
            output.push_str(&format!("Time:   {:.2}s\n", duration.as_secs_f64()));
        }

        output.push_str(&format!("{}\n", separator));

        output
    }

    /// Apply color to text if colorization is enabled.
    fn colorize(&self, text: &str, color: Color) -> String {
        if self.colorized {
            match color {
                Color::Green => format!("\x1b[32m{}\x1b[0m", text),
                Color::Red => format!("\x1b[31m{}\x1b[0m", text),
                Color::Yellow => format!("\x1b[33m{}\x1b[0m", text),
            }
        } else {
            text.to_string()
        }
    }
}

impl Default for ReportGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Color options for terminal output.
enum Color {
    Green,
    Red,
    #[allow(dead_code)]
    Yellow,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata_parser::AssertionKind;
    use crate::output_parser::AssertionResult;

    fn create_test_result(name: &str, passed: bool) -> TestValidationResult {
        TestValidationResult {
            test_name: name.to_string(),
            passed,
            assertion_results: vec![],
            expected_error_matched: None,
            actual_error: None,
        }
    }

    fn create_test_result_with_assertions(
        name: &str,
        passed: bool,
        assertions: Vec<(&str, bool)>,
    ) -> TestValidationResult {
        let assertion_results = assertions
            .into_iter()
            .map(|(text, found)| AssertionResult {
                expected: text.to_string(),
                kind: AssertionKind::Required,
                found,
                message: if found {
                    format!("✓ {}", text)
                } else {
                    format!("✗ {} (not found)", text)
                },
            })
            .collect();

        TestValidationResult {
            test_name: name.to_string(),
            passed,
            assertion_results,
            expected_error_matched: None,
            actual_error: None,
        }
    }

    #[test]
    fn test_category_results_new() {
        let results = CategoryResults::new();
        assert_eq!(results.total_count(), 0);
        assert_eq!(results.passed_count(), 0);
        assert_eq!(results.failed_count(), 0);
    }

    #[test]
    fn test_category_results_add() {
        let mut results = CategoryResults::new();

        results.add(create_test_result("test1", true), TestCategory::Unit);
        results.add(
            create_test_result("test2", false),
            TestCategory::Integration,
        );
        results.add(create_test_result("test3", true), TestCategory::ErrorDemo);

        assert_eq!(results.unit.len(), 1);
        assert_eq!(results.integration.len(), 1);
        assert_eq!(results.error_demo.len(), 1);
        assert_eq!(results.total_count(), 3);
        assert_eq!(results.passed_count(), 2);
        assert_eq!(results.failed_count(), 1);
    }

    #[test]
    fn test_test_suite_result_new() {
        let suite = TestSuiteResult::new("test.ferris".to_string());
        assert_eq!(suite.file_name, "test.ferris");
        assert_eq!(suite.results.total_count(), 0);
        assert!(suite.duration.is_none());
        assert!(suite.all_passed());
    }

    #[test]
    fn test_test_suite_result_with_duration() {
        let suite =
            TestSuiteResult::new("test.ferris".to_string()).with_duration(Duration::from_secs(2));
        assert_eq!(suite.duration, Some(Duration::from_secs(2)));
    }

    #[test]
    fn test_report_generator_new() {
        let generator = ReportGenerator::new();
        assert!(generator.show_assertions);
        assert!(generator.colorized);
    }

    #[test]
    fn test_report_generator_with_options() {
        let generator = ReportGenerator::new()
            .with_assertions(false)
            .with_colors(false);
        assert!(!generator.show_assertions);
        assert!(!generator.colorized);
    }

    #[test]
    fn test_format_header() {
        let generator = ReportGenerator::new().with_colors(false);
        let header = generator.format_header("test.ferris");
        assert!(header.contains("Test Results: test.ferris"));
        assert!(header.contains("====="));
    }

    #[test]
    fn test_format_category_section_all_passed() {
        let generator = ReportGenerator::new().with_colors(false);
        let results = vec![
            create_test_result("test1", true),
            create_test_result("test2", true),
        ];
        let section = generator.format_category_section("Unit Tests", &results);
        assert!(section.contains("Unit Tests"));
        assert!(section.contains("✓ test1"));
        assert!(section.contains("✓ test2"));
        assert!(section.contains("2/2 passed"));
    }

    #[test]
    fn test_format_category_section_some_failed() {
        let generator = ReportGenerator::new().with_colors(false);
        let results = vec![
            create_test_result("test1", true),
            create_test_result("test2", false),
        ];
        let section = generator.format_category_section("Integration Tests", &results);
        assert!(section.contains("✓ test1"));
        assert!(section.contains("✗ test2"));
        assert!(section.contains("1/2 passed"));
    }

    #[test]
    fn test_format_test_result_with_assertions() {
        let generator = ReportGenerator::new().with_colors(false);
        let result = create_test_result_with_assertions(
            "test_assertions",
            true,
            vec![("Output 1", true), ("Output 2", true)],
        );
        let formatted = generator.format_test_result(&result);
        assert!(formatted.contains("✓ test_assertions"));
        assert!(formatted.contains("✓ Output 1"));
        assert!(formatted.contains("✓ Output 2"));
    }

    #[test]
    fn test_format_test_result_without_assertions() {
        let generator = ReportGenerator::new()
            .with_colors(false)
            .with_assertions(false);
        let result = create_test_result_with_assertions(
            "test_no_assertions",
            true,
            vec![("Output 1", true)],
        );
        let formatted = generator.format_test_result(&result);
        assert!(formatted.contains("✓ test_no_assertions"));
        assert!(!formatted.contains("Output 1"));
    }

    #[test]
    fn test_format_error_demo_section() {
        let generator = ReportGenerator::new().with_colors(false);
        let results = vec![TestValidationResult {
            test_name: "error_test".to_string(),
            passed: true,
            assertion_results: vec![],
            expected_error_matched: Some(true),
            actual_error: Some("Expected error message".to_string()),
        }];
        let section = generator.format_error_demo_section(&results);
        assert!(section.contains("Error Demos"));
        assert!(section.contains("✓ error_test"));
        assert!(section.contains("Expected error message"));
        assert!(section.contains("1/1 passed"));
    }

    #[test]
    fn test_format_summary() {
        let generator = ReportGenerator::new().with_colors(false);
        let mut suite = TestSuiteResult::new("test.ferris".to_string())
            .with_duration(Duration::from_millis(1500));
        suite
            .results
            .add(create_test_result("test1", true), TestCategory::Unit);
        suite.results.add(
            create_test_result("test2", false),
            TestCategory::Integration,
        );

        let summary = generator.format_summary(&suite);
        assert!(summary.contains("Summary"));
        assert!(summary.contains("Total:  2 tests"));
        assert!(summary.contains("Passed: 1"));
        assert!(summary.contains("Failed: 1"));
        assert!(summary.contains("Time:   1.50s"));
    }

    #[test]
    fn test_generate_full_report() {
        let generator = ReportGenerator::new().with_colors(false);
        let mut suite = TestSuiteResult::new("test.ferris".to_string());

        suite
            .results
            .add(create_test_result("unit_test", true), TestCategory::Unit);
        suite.results.add(
            create_test_result("integration_test", true),
            TestCategory::Integration,
        );
        suite.results.add(
            TestValidationResult {
                test_name: "error_demo".to_string(),
                passed: true,
                assertion_results: vec![],
                expected_error_matched: Some(true),
                actual_error: Some("Error occurred".to_string()),
            },
            TestCategory::ErrorDemo,
        );

        let report = generator.generate_report(&suite);
        assert!(report.contains("Test Results: test.ferris"));
        assert!(report.contains("Unit Tests"));
        assert!(report.contains("Integration Tests"));
        assert!(report.contains("Error Demos"));
        assert!(report.contains("Summary"));
        assert!(report.contains("Total:  3 tests"));
        assert!(report.contains("Passed: 3"));
    }

    #[test]
    fn test_colorize() {
        let generator = ReportGenerator::new().with_colors(true);
        let green = generator.colorize("✓", Color::Green);
        assert!(green.contains("\x1b[32m"));
        assert!(green.contains("\x1b[0m"));

        let generator = ReportGenerator::new().with_colors(false);
        let plain = generator.colorize("✓", Color::Green);
        assert_eq!(plain, "✓");
    }
}
