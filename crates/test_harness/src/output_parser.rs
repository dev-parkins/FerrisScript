use regex::Regex;
use serde::{Deserialize, Serialize};

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
                let test_name = captures.get(2).map(|m| m.as_str()).unwrap_or("").to_string();
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
                line.contains("ERROR") || 
                line.contains("Error") || 
                line.contains("FATAL") ||
                line.contains("panic")
            })
            .map(|s| s.to_string())
            .collect()
    }
    
    /// Count pass/fail results
    fn count_results(&self, markers: &[TestMarker]) -> (usize, usize) {
        let passed = markers.iter().filter(|m| m.kind == TestMarkerKind::Pass).count();
        let failed = markers.iter().filter(|m| m.kind == TestMarkerKind::Fail).count();
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
}
