//! Error code system for FerrisScript compiler errors.
//!
//! This module provides structured error codes (E001-E499) for all compiler errors,
//! organized by category for better error tracking and documentation.
//!
//! # Error Code Categories
//!
//! - **E001-E099**: Lexical errors (tokenization)
//! - **E100-E199**: Syntax errors (parsing)
//! - **E200-E299**: Type errors (type checking)
//! - **E300-E399**: Semantic errors (semantic analysis) - Reserved for future use
//! - **E400-E499**: Runtime errors (execution)
//!
//! # Example
//!
//! ```no_run
//! use ferrisscript_compiler::error_code::{ErrorCode, ErrorCategory};
//!
//! let code = ErrorCode::E201;
//! assert_eq!(code.as_str(), "E201");
//! assert_eq!(code.description(), "Undefined variable");
//! assert_eq!(code.category(), ErrorCategory::Type);
//! ```

/// Error code categories for organizing compiler errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    /// Lexical errors (E001-E099) - tokenization issues
    Lexical,
    /// Syntax errors (E100-E199) - parsing issues
    Syntax,
    /// Type errors (E200-E299) - type checking issues
    Type,
    /// Semantic errors (E300-E399) - semantic analysis issues (reserved for future)
    Semantic,
    /// Runtime errors (E400-E499) - execution issues
    Runtime,
}

impl ErrorCategory {
    /// Returns the human-readable name of the category
    pub fn name(&self) -> &'static str {
        match self {
            ErrorCategory::Lexical => "Lexical Error",
            ErrorCategory::Syntax => "Syntax Error",
            ErrorCategory::Type => "Type Error",
            ErrorCategory::Semantic => "Semantic Error",
            ErrorCategory::Runtime => "Runtime Error",
        }
    }
}

/// Structured error codes for FerrisScript compiler errors
///
/// Each error code corresponds to a specific type of error that can occur
/// during compilation or execution. Error codes are organized by category
/// and include human-readable descriptions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum ErrorCode {
    // Lexical Errors (E001-E099)
    /// Invalid character in source code
    E001,
    /// Unterminated string literal
    E002,
    /// Invalid escape sequence in string
    E003,
    /// Invalid number format
    E004,
    /// Invalid identifier
    E005,
    /// Unexpected end of file during lexing
    E006,

    // Syntax Errors (E100-E199)
    /// Expected a specific token but found another
    E100,
    /// Unexpected token encountered
    E101,
    /// Missing closing delimiter (parenthesis, bracket, or brace)
    E102,
    /// Invalid expression syntax
    E103,
    /// Invalid statement syntax
    E104,
    /// Invalid function declaration syntax
    E105,
    /// Invalid type annotation syntax
    E106,
    /// Invalid pattern syntax
    E107,
    /// Unexpected end of file during parsing
    E108,
    /// Missing identifier where one was expected
    E109,
    /// Missing type annotation where one was expected
    E110,
    /// Invalid function parameter syntax
    E111,
    /// Invalid return type syntax
    E112,
    /// Invalid operator in expression context
    E113,

    // Type Errors (E200-E299)
    /// Type mismatch between expected and actual types
    E200,
    /// Reference to undefined variable
    E201,
    /// Call to undefined function
    E202,
    /// Reference to undefined type
    E203,
    /// Wrong number of arguments in function call
    E204,
    /// Incorrect argument type in function call
    E205,
    /// Return type doesn't match function signature
    E206,
    /// Attempt to assign to immutable variable
    E207,
    /// Duplicate definition of variable or function
    E208,
    /// Invalid field access on type
    E209,
    /// Invalid method call on type
    E210,
    /// Condition expression must be boolean
    E211,
    /// Binary operation not supported for given types
    E212,
    /// Unary operation not supported for given type
    E213,
    /// Cannot assign to field of non-object type
    E214,
    /// Field not found on type
    E215,
    /// Cannot perform compound assignment on immutable variable
    E216,
    /// Invalid assignment target
    E217,
    /// Type annotation required but not provided
    E218,
    /// Incompatible types in assignment
    E219,

    // Semantic Errors (E300-E399) - Reserved for future semantic analysis
    // These will be implemented when semantic analyzer is added
    // E300: Unreachable code
    // E301: Unused variable (warning)
    // E302: Unused function (warning)
    // E303: Dead code (warning)
    // E304: Invalid break/continue (not in loop)
    // E305: Invalid return (not in function)

    // Runtime Errors (E400-E499)
    /// Division by zero
    E400,
    /// Array index out of bounds
    E401,
    /// Null pointer or None value access
    E402,
    /// Stack overflow
    E403,
    /// Memory exhaustion
    E404,
    /// Godot API error
    E405,
    /// Undefined variable at runtime
    E406,
    /// Unknown built-in function
    E407,
    /// Invalid assignment target at runtime
    E408,
    /// Cannot assign value to immutable variable at runtime
    E409,
    /// Invalid field assignment
    E410,
    /// Field not found at runtime
    /// Complex field assignment not implemented
    E411,
    E412,
    /// Function not found at runtime
    E413,
    /// Invalid function call at runtime
    E414,
}

impl ErrorCode {
    /// Returns the error code as a string (e.g., "E001")
    pub fn as_str(&self) -> &'static str {
        match self {
            // Lexical Errors
            ErrorCode::E001 => "E001",
            ErrorCode::E002 => "E002",
            ErrorCode::E003 => "E003",
            ErrorCode::E004 => "E004",
            ErrorCode::E005 => "E005",
            ErrorCode::E006 => "E006",

            // Syntax Errors
            ErrorCode::E100 => "E100",
            ErrorCode::E101 => "E101",
            ErrorCode::E102 => "E102",
            ErrorCode::E103 => "E103",
            ErrorCode::E104 => "E104",
            ErrorCode::E105 => "E105",
            ErrorCode::E106 => "E106",
            ErrorCode::E107 => "E107",
            ErrorCode::E108 => "E108",
            ErrorCode::E109 => "E109",
            ErrorCode::E110 => "E110",
            ErrorCode::E111 => "E111",
            ErrorCode::E112 => "E112",
            ErrorCode::E113 => "E113",

            // Type Errors
            ErrorCode::E200 => "E200",
            ErrorCode::E201 => "E201",
            ErrorCode::E202 => "E202",
            ErrorCode::E203 => "E203",
            ErrorCode::E204 => "E204",
            ErrorCode::E205 => "E205",
            ErrorCode::E206 => "E206",
            ErrorCode::E207 => "E207",
            ErrorCode::E208 => "E208",
            ErrorCode::E209 => "E209",
            ErrorCode::E210 => "E210",
            ErrorCode::E211 => "E211",
            ErrorCode::E212 => "E212",
            ErrorCode::E213 => "E213",
            ErrorCode::E214 => "E214",
            ErrorCode::E215 => "E215",
            ErrorCode::E216 => "E216",
            ErrorCode::E217 => "E217",
            ErrorCode::E218 => "E218",
            ErrorCode::E219 => "E219",

            // Runtime Errors
            ErrorCode::E400 => "E400",
            ErrorCode::E401 => "E401",
            ErrorCode::E402 => "E402",
            ErrorCode::E403 => "E403",
            ErrorCode::E404 => "E404",
            ErrorCode::E405 => "E405",
            ErrorCode::E406 => "E406",
            ErrorCode::E407 => "E407",
            ErrorCode::E408 => "E408",
            ErrorCode::E409 => "E409",
            ErrorCode::E410 => "E410",
            ErrorCode::E411 => "E411",
            ErrorCode::E412 => "E412",
            ErrorCode::E413 => "E413",
            ErrorCode::E414 => "E414",
        }
    }

    /// Get documentation URL for this error code
    ///
    /// By default, links to GitHub Pages documentation. Set `FERRIS_DOCS_BASE` environment
    /// variable to use a custom documentation site (e.g., when docs.ferrisscript.dev launches).
    ///
    /// # Examples
    ///
    /// Default (GitHub Pages):
    /// ```
    /// use ferrisscript_compiler::error_code::ErrorCode;
    /// let url = ErrorCode::E201.get_docs_url();
    /// assert!(url.contains("github.io"));
    /// assert!(url.contains("#e201"));
    /// ```
    ///
    /// With custom docs site:
    /// ```bash
    /// export FERRIS_DOCS_BASE=https://docs.ferrisscript.dev
    /// ```
    /// ```ignore
    /// use ferrisscript_compiler::error_code::ErrorCode;
    /// let url = ErrorCode::E201.get_docs_url();
    /// // Returns: "https://docs.ferrisscript.dev/errors/E201"
    /// ```
    pub fn get_docs_url(&self) -> String {
        let code = self.as_str(); // "E201"

        // Check for custom docs base URL
        if let Ok(base) = std::env::var("FERRIS_DOCS_BASE") {
            format!("{}/errors/{}", base.trim_end_matches('/'), code)
        } else {
            // Default: GitHub Pages documentation site
            // Generate proper GitHub Pages anchor from error code and description
            // Example: E201 "Undefined variable" → #e201-undefined-variable
            let description = self.description();
            let slug = description
                .to_lowercase()
                .replace(' ', "-")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-')
                .collect::<String>();
            let anchor = format!("{}-{}", code.to_lowercase(), slug);

            format!(
                "https://dev-parkins.github.io/FerrisScript/ERROR_CODES/#{}",
                anchor
            )
        }
    }

    /// Returns a human-readable description of the error
    pub fn description(&self) -> &'static str {
        match self {
            // Lexical Errors
            ErrorCode::E001 => "Invalid character",
            ErrorCode::E002 => "Unterminated string literal",
            ErrorCode::E003 => "Invalid escape sequence",
            ErrorCode::E004 => "Invalid number format",
            ErrorCode::E005 => "Invalid identifier",
            ErrorCode::E006 => "Unexpected end of file",

            // Syntax Errors
            ErrorCode::E100 => "Expected token",
            ErrorCode::E101 => "Unexpected token",
            ErrorCode::E102 => "Missing closing delimiter",
            ErrorCode::E103 => "Invalid expression",
            ErrorCode::E104 => "Invalid statement",
            ErrorCode::E105 => "Invalid function declaration",
            ErrorCode::E106 => "Invalid type annotation",
            ErrorCode::E107 => "Invalid pattern",
            ErrorCode::E108 => "Unexpected end of file",
            ErrorCode::E109 => "Missing identifier",
            ErrorCode::E110 => "Missing type annotation",
            ErrorCode::E111 => "Invalid function parameter",
            ErrorCode::E112 => "Invalid return type",
            ErrorCode::E113 => "Invalid operator",

            // Type Errors
            ErrorCode::E200 => "Type mismatch",
            ErrorCode::E201 => "Undefined variable",
            ErrorCode::E202 => "Undefined function",
            ErrorCode::E203 => "Undefined type",
            ErrorCode::E204 => "Wrong number of arguments",
            ErrorCode::E205 => "Incorrect argument type",
            ErrorCode::E206 => "Return type mismatch",
            ErrorCode::E207 => "Cannot assign to immutable variable",
            ErrorCode::E208 => "Duplicate definition",
            ErrorCode::E209 => "Invalid field access",
            ErrorCode::E210 => "Invalid method call",
            ErrorCode::E211 => "Condition must be boolean",
            ErrorCode::E212 => "Binary operation type error",
            ErrorCode::E213 => "Unary operation type error",
            ErrorCode::E214 => "Cannot assign to field",
            ErrorCode::E215 => "Field not found",
            ErrorCode::E216 => "Cannot perform compound assignment on immutable variable",
            ErrorCode::E217 => "Invalid assignment target",
            ErrorCode::E218 => "Type annotation required",
            ErrorCode::E219 => "Incompatible types in assignment",

            // Runtime Errors
            ErrorCode::E400 => "Division by zero",
            ErrorCode::E401 => "Index out of bounds",
            ErrorCode::E402 => "Null pointer access",
            ErrorCode::E403 => "Stack overflow",
            ErrorCode::E404 => "Memory exhaustion",
            ErrorCode::E405 => "Godot API error",
            ErrorCode::E406 => "Undefined variable",
            ErrorCode::E407 => "Unknown built-in function",
            ErrorCode::E408 => "Invalid assignment target",
            ErrorCode::E409 => "Cannot assign to immutable variable",
            ErrorCode::E410 => "Invalid field assignment",
            ErrorCode::E411 => "Field not found",
            ErrorCode::E412 => "Complex field assignment not implemented",
            ErrorCode::E413 => "Function not found",
            ErrorCode::E414 => "Invalid function call",
        }
    }

    /// Returns the category this error code belongs to
    pub fn category(&self) -> ErrorCategory {
        match self {
            // Lexical Errors
            ErrorCode::E001
            | ErrorCode::E002
            | ErrorCode::E003
            | ErrorCode::E004
            | ErrorCode::E005
            | ErrorCode::E006 => ErrorCategory::Lexical,

            // Syntax Errors
            ErrorCode::E100
            | ErrorCode::E101
            | ErrorCode::E102
            | ErrorCode::E103
            | ErrorCode::E104
            | ErrorCode::E105
            | ErrorCode::E106
            | ErrorCode::E107
            | ErrorCode::E108
            | ErrorCode::E109
            | ErrorCode::E110
            | ErrorCode::E111
            | ErrorCode::E112
            | ErrorCode::E113 => ErrorCategory::Syntax,

            // Type Errors
            ErrorCode::E200
            | ErrorCode::E201
            | ErrorCode::E202
            | ErrorCode::E203
            | ErrorCode::E204
            | ErrorCode::E205
            | ErrorCode::E206
            | ErrorCode::E207
            | ErrorCode::E208
            | ErrorCode::E209
            | ErrorCode::E210
            | ErrorCode::E211
            | ErrorCode::E212
            | ErrorCode::E213
            | ErrorCode::E214
            | ErrorCode::E215
            | ErrorCode::E216
            | ErrorCode::E217
            | ErrorCode::E218
            | ErrorCode::E219 => ErrorCategory::Type,

            // Runtime Errors
            ErrorCode::E400
            | ErrorCode::E401
            | ErrorCode::E402
            | ErrorCode::E403
            | ErrorCode::E404
            | ErrorCode::E405
            | ErrorCode::E406
            | ErrorCode::E407
            | ErrorCode::E408
            | ErrorCode::E409
            | ErrorCode::E410
            | ErrorCode::E411
            | ErrorCode::E412
            | ErrorCode::E413
            | ErrorCode::E414 => ErrorCategory::Runtime,
        }
    }

    /// Returns a documentation URL for this error code
    pub fn doc_url(&self) -> String {
        format!("https://ferrisscript.dev/errors/{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_as_str() {
        assert_eq!(ErrorCode::E001.as_str(), "E001");
        assert_eq!(ErrorCode::E100.as_str(), "E100");
        assert_eq!(ErrorCode::E201.as_str(), "E201");
        assert_eq!(ErrorCode::E400.as_str(), "E400");
    }

    #[test]
    fn test_error_code_description() {
        assert_eq!(ErrorCode::E002.description(), "Unterminated string literal");
        assert_eq!(ErrorCode::E201.description(), "Undefined variable");
    }

    #[test]
    fn test_error_code_category() {
        assert_eq!(ErrorCode::E001.category(), ErrorCategory::Lexical);
        assert_eq!(ErrorCode::E100.category(), ErrorCategory::Syntax);
        assert_eq!(ErrorCode::E201.category(), ErrorCategory::Type);
        assert_eq!(ErrorCode::E400.category(), ErrorCategory::Runtime);
    }

    #[test]
    fn test_error_category_name() {
        assert_eq!(ErrorCategory::Lexical.name(), "Lexical Error");
        assert_eq!(ErrorCategory::Syntax.name(), "Syntax Error");
        assert_eq!(ErrorCategory::Type.name(), "Type Error");
        assert_eq!(ErrorCategory::Runtime.name(), "Runtime Error");
    }

    #[test]
    fn test_doc_url() {
        assert_eq!(
            ErrorCode::E201.doc_url(),
            "https://ferrisscript.dev/errors/E201"
        );
    }

    #[test]
    fn test_all_lexical_errors() {
        let codes = vec![
            ErrorCode::E001,
            ErrorCode::E002,
            ErrorCode::E003,
            ErrorCode::E004,
            ErrorCode::E005,
            ErrorCode::E006,
        ];
        for code in codes {
            assert_eq!(code.category(), ErrorCategory::Lexical);
            assert!(!code.as_str().is_empty());
            assert!(!code.description().is_empty());
        }
    }

    #[test]
    fn test_all_syntax_errors() {
        let codes = vec![
            ErrorCode::E100,
            ErrorCode::E101,
            ErrorCode::E102,
            ErrorCode::E103,
            ErrorCode::E104,
            ErrorCode::E105,
            ErrorCode::E106,
            ErrorCode::E107,
            ErrorCode::E108,
            ErrorCode::E109,
            ErrorCode::E110,
            ErrorCode::E111,
            ErrorCode::E112,
            ErrorCode::E113,
        ];
        for code in codes {
            assert_eq!(code.category(), ErrorCategory::Syntax);
            assert!(!code.as_str().is_empty());
            assert!(!code.description().is_empty());
        }
    }

    #[test]
    fn test_all_type_errors() {
        let codes = vec![
            ErrorCode::E200,
            ErrorCode::E201,
            ErrorCode::E202,
            ErrorCode::E203,
            ErrorCode::E204,
            ErrorCode::E205,
            ErrorCode::E206,
            ErrorCode::E207,
            ErrorCode::E208,
            ErrorCode::E209,
            ErrorCode::E210,
            ErrorCode::E211,
            ErrorCode::E212,
            ErrorCode::E213,
            ErrorCode::E214,
            ErrorCode::E215,
            ErrorCode::E216,
            ErrorCode::E217,
            ErrorCode::E218,
            ErrorCode::E219,
        ];
        for code in codes {
            assert_eq!(code.category(), ErrorCategory::Type);
            assert!(!code.as_str().is_empty());
            assert!(!code.description().is_empty());
        }
    }

    #[test]
    fn test_all_runtime_errors() {
        let codes = vec![
            ErrorCode::E400,
            ErrorCode::E401,
            ErrorCode::E402,
            ErrorCode::E403,
            ErrorCode::E404,
            ErrorCode::E405,
            ErrorCode::E406,
            ErrorCode::E407,
            ErrorCode::E408,
            ErrorCode::E409,
            ErrorCode::E410,
            ErrorCode::E411,
            ErrorCode::E412,
            ErrorCode::E413,
            ErrorCode::E414,
        ];
        for code in codes {
            assert_eq!(code.category(), ErrorCategory::Runtime);
            assert!(!code.as_str().is_empty());
            assert!(!code.description().is_empty());
        }
    }

    #[test]
    fn test_get_docs_url_default() {
        // Without FERRIS_DOCS_BASE env var, should return GitHub Pages URL with proper anchors
        std::env::remove_var("FERRIS_DOCS_BASE");

        let url = ErrorCode::E001.get_docs_url();
        assert!(url.contains("github.io"));
        assert!(url.contains("#e001-invalid-character"));

        let url = ErrorCode::E201.get_docs_url();
        assert!(url.contains("github.io"));
        assert!(url.contains("#e201-undefined-variable"));
    }

    #[test]
    fn test_get_docs_url_format() {
        // Test URL format structure with proper GitHub Pages slugification
        std::env::remove_var("FERRIS_DOCS_BASE");

        // Test that URLs are well-formed
        let url = ErrorCode::E001.get_docs_url();
        assert!(!url.is_empty());
        assert!(url.starts_with("http"));
        assert!(url.contains("ERROR_CODES/#"));
        assert!(url.contains("e001-")); // Lowercase code with hyphen

        // Test a few specific codes to ensure slugification works
        let url = ErrorCode::E001.get_docs_url();
        println!("E001 URL: {}", url);
        assert!(url.contains("#e001-invalid-character"));

        let url = ErrorCode::E201.get_docs_url();
        println!("E201 URL: {}", url);
        assert!(url.contains("#e201-undefined-variable"));

        let url = ErrorCode::E200.get_docs_url();
        println!("E200 URL: {}", url);
        assert!(url.contains("#e200-type-mismatch"));
    }
}
