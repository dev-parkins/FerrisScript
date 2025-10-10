/// Metadata parser for structured test definitions
///
/// Parses test metadata directives from FerrisScript source code comments.
/// Supports directives like:
/// - // TEST: test_name
/// - // CATEGORY: unit|integration|error_demo
/// - // DESCRIPTION: description text
/// - // EXPECT: success|error
/// - // EXPECT_ERROR: error substring
/// - // ASSERT: expected output
/// - // ASSERT_OPTIONAL: optional output
use std::fmt;
use std::str::FromStr;

/// Test category for organizing test results
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TestCategory {
    Unit,
    Integration,
    ErrorDemo,
}

impl TestCategory {
    pub fn as_str(&self) -> &str {
        match self {
            TestCategory::Unit => "unit",
            TestCategory::Integration => "integration",
            TestCategory::ErrorDemo => "error_demo",
        }
    }
}

impl FromStr for TestCategory {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "unit" => Ok(TestCategory::Unit),
            "integration" => Ok(TestCategory::Integration),
            "error_demo" | "error-demo" => Ok(TestCategory::ErrorDemo),
            _ => Err(ParseError::InvalidCategory(s.to_string())),
        }
    }
}

impl fmt::Display for TestCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Expected test outcome
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestExpectation {
    Success,
    Error,
}

impl FromStr for TestExpectation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "success" => Ok(TestExpectation::Success),
            "error" => Ok(TestExpectation::Error),
            _ => Err(ParseError::InvalidExpectation(s.to_string())),
        }
    }
}

/// Assertion type (required or optional)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssertionKind {
    Required,
    Optional,
}

/// Single assertion to validate against output
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assertion {
    pub kind: AssertionKind,
    pub expected: String,
}

impl Assertion {
    pub fn required(expected: String) -> Self {
        Self {
            kind: AssertionKind::Required,
            expected,
        }
    }

    pub fn optional(expected: String) -> Self {
        Self {
            kind: AssertionKind::Optional,
            expected,
        }
    }
}

/// Complete test metadata block
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestMetadata {
    pub name: String,
    pub category: TestCategory,
    pub description: Option<String>,
    pub expect: TestExpectation,
    pub expect_error: Option<String>,
    pub assertions: Vec<Assertion>,
}

impl TestMetadata {
    pub fn new(name: String) -> Self {
        Self {
            name,
            category: TestCategory::Unit, // Default
            description: None,
            expect: TestExpectation::Success, // Default
            expect_error: None,
            assertions: Vec::new(),
        }
    }
}

/// Parse errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidCategory(String),
    InvalidExpectation(String),
    MissingTestName,
    DuplicateTestName(String),
    InvalidDirective(String),
    ExpectErrorWithoutExpectError,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidCategory(cat) => write!(f, "Invalid category: {}", cat),
            ParseError::InvalidExpectation(exp) => write!(f, "Invalid expectation: {}", exp),
            ParseError::MissingTestName => write!(f, "Missing TEST directive"),
            ParseError::DuplicateTestName(name) => write!(f, "Duplicate test name: {}", name),
            ParseError::InvalidDirective(dir) => write!(f, "Invalid directive: {}", dir),
            ParseError::ExpectErrorWithoutExpectError => {
                write!(f, "EXPECT_ERROR requires EXPECT: error")
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// Metadata parser
pub struct MetadataParser;

impl MetadataParser {
    /// Parse all test metadata blocks from source code
    pub fn parse_metadata(source: &str) -> Result<Vec<TestMetadata>, ParseError> {
        let mut tests = Vec::new();
        let mut seen_names = std::collections::HashSet::new();
        let lines: Vec<&str> = source.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            if let Some((metadata, consumed)) = Self::extract_test_block(&lines[i..])? {
                // Check for duplicate names
                if seen_names.contains(&metadata.name) {
                    return Err(ParseError::DuplicateTestName(metadata.name.clone()));
                }
                seen_names.insert(metadata.name.clone());

                // Validate metadata
                Self::validate_metadata(&metadata)?;

                tests.push(metadata);
                i += consumed;
            } else {
                i += 1;
            }
        }

        Ok(tests)
    }

    /// Extract a single test metadata block starting from the given lines
    /// Returns (metadata, lines_consumed) or None if no test block found
    fn extract_test_block(lines: &[&str]) -> Result<Option<(TestMetadata, usize)>, ParseError> {
        // Look for TEST directive
        let mut i = 0;
        while i < lines.len() {
            let line = lines[i].trim();
            if line.starts_with("// TEST:") {
                break;
            }
            i += 1;
        }

        if i >= lines.len() {
            return Ok(None); // No test block found
        }

        // Parse TEST directive
        let test_line = lines[i].trim();
        let test_name = Self::parse_test_name(test_line)?;
        let mut metadata = TestMetadata::new(test_name);
        i += 1;

        // Parse subsequent directives
        while i < lines.len() {
            let line = lines[i].trim();

            // Stop at empty line or non-directive line
            if line.is_empty() || !line.starts_with("//") {
                break;
            }

            // Stop at next TEST directive
            if line.starts_with("// TEST:") {
                break;
            }

            Self::parse_directive(line, &mut metadata)?;
            i += 1;
        }

        Ok(Some((metadata, i)))
    }

    /// Parse TEST directive and extract test name
    fn parse_test_name(line: &str) -> Result<String, ParseError> {
        let line = line.trim();
        if !line.starts_with("// TEST:") {
            return Err(ParseError::MissingTestName);
        }

        let name = line["// TEST:".len()..].trim();
        if name.is_empty() {
            return Err(ParseError::MissingTestName);
        }

        Ok(name.to_string())
    }

    /// Parse a single directive and update metadata
    fn parse_directive(line: &str, metadata: &mut TestMetadata) -> Result<(), ParseError> {
        let line = line.trim();
        if !line.starts_with("//") {
            return Ok(());
        }

        let content = line[2..].trim();

        if let Some(rest) = content.strip_prefix("CATEGORY:") {
            metadata.category = rest.trim().parse()?;
        } else if let Some(rest) = content.strip_prefix("DESCRIPTION:") {
            metadata.description = Some(rest.trim().to_string());
        } else if let Some(rest) = content.strip_prefix("EXPECT:") {
            metadata.expect = rest.trim().parse()?;
        } else if let Some(rest) = content.strip_prefix("EXPECT_ERROR:") {
            metadata.expect_error = Some(rest.trim().to_string());
        } else if let Some(rest) = content.strip_prefix("ASSERT:") {
            metadata
                .assertions
                .push(Assertion::required(rest.trim().to_string()));
        } else if let Some(rest) = content.strip_prefix("ASSERT_OPTIONAL:") {
            metadata
                .assertions
                .push(Assertion::optional(rest.trim().to_string()));
        }
        // Ignore other comment lines

        Ok(())
    }

    /// Validate metadata consistency
    fn validate_metadata(metadata: &TestMetadata) -> Result<(), ParseError> {
        // If EXPECT_ERROR is set, EXPECT must be Error
        if metadata.expect_error.is_some() && metadata.expect != TestExpectation::Error {
            return Err(ParseError::ExpectErrorWithoutExpectError);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_test() {
        let source = r#"
// TEST: simple_test
// CATEGORY: unit
// DESCRIPTION: A simple test
// EXPECT: success
// ASSERT: Expected output

fn _ready() {
    print("Expected output");
}
"#;

        let result = MetadataParser::parse_metadata(source).unwrap();
        assert_eq!(result.len(), 1);

        let test = &result[0];
        assert_eq!(test.name, "simple_test");
        assert_eq!(test.category, TestCategory::Unit);
        assert_eq!(test.description, Some("A simple test".to_string()));
        assert_eq!(test.expect, TestExpectation::Success);
        assert_eq!(test.assertions.len(), 1);
        assert_eq!(test.assertions[0].expected, "Expected output");
        assert_eq!(test.assertions[0].kind, AssertionKind::Required);
    }

    #[test]
    fn test_parse_error_demo() {
        let source = r#"
// TEST: error_test
// CATEGORY: error_demo
// EXPECT: error
// EXPECT_ERROR: Node not found

fn _ready() {
    let invalid = get_node("Invalid");
}
"#;

        let result = MetadataParser::parse_metadata(source).unwrap();
        assert_eq!(result.len(), 1);

        let test = &result[0];
        assert_eq!(test.name, "error_test");
        assert_eq!(test.category, TestCategory::ErrorDemo);
        assert_eq!(test.expect, TestExpectation::Error);
        assert_eq!(test.expect_error, Some("Node not found".to_string()));
    }

    #[test]
    fn test_parse_multiple_assertions() {
        let source = r#"
// TEST: multi_assert
// ASSERT: First output
// ASSERT: Second output
// ASSERT_OPTIONAL: Optional output

fn _ready() {}
"#;

        let result = MetadataParser::parse_metadata(source).unwrap();
        let test = &result[0];

        assert_eq!(test.assertions.len(), 3);
        assert_eq!(test.assertions[0].kind, AssertionKind::Required);
        assert_eq!(test.assertions[1].kind, AssertionKind::Required);
        assert_eq!(test.assertions[2].kind, AssertionKind::Optional);
    }

    #[test]
    fn test_parse_multiple_tests() {
        let source = r#"
// TEST: test1
// ASSERT: Output 1

fn test1() {}

// TEST: test2
// ASSERT: Output 2

fn test2() {}
"#;

        let result = MetadataParser::parse_metadata(source).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "test1");
        assert_eq!(result[1].name, "test2");
    }

    #[test]
    fn test_duplicate_test_name_error() {
        let source = r#"
// TEST: duplicate
// TEST: duplicate
"#;

        let result = MetadataParser::parse_metadata(source);
        assert!(matches!(
            result,
            Err(ParseError::DuplicateTestName(name)) if name == "duplicate"
        ));
    }

    #[test]
    fn test_expect_error_without_error_expectation() {
        let source = r#"
// TEST: invalid
// EXPECT: success
// EXPECT_ERROR: Some error
"#;

        let result = MetadataParser::parse_metadata(source);
        assert!(matches!(
            result,
            Err(ParseError::ExpectErrorWithoutExpectError)
        ));
    }

    #[test]
    fn test_default_values() {
        let source = r#"
// TEST: minimal
"#;

        let result = MetadataParser::parse_metadata(source).unwrap();
        let test = &result[0];

        assert_eq!(test.category, TestCategory::Unit); // Default
        assert_eq!(test.expect, TestExpectation::Success); // Default
        assert!(test.description.is_none());
        assert!(test.expect_error.is_none());
        assert!(test.assertions.is_empty());
    }

    #[test]
    fn test_invalid_category() {
        let source = r#"
// TEST: test
// CATEGORY: invalid_category
"#;

        let result = MetadataParser::parse_metadata(source);
        assert!(matches!(result, Err(ParseError::InvalidCategory(_))));
    }

    #[test]
    fn test_invalid_expectation() {
        let source = r#"
// TEST: test
// EXPECT: maybe
"#;

        let result = MetadataParser::parse_metadata(source);
        assert!(matches!(result, Err(ParseError::InvalidExpectation(_))));
    }
}
