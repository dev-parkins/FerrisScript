/// Helper functions for formatting compiler errors with source context
///
/// This module provides utilities to display error messages with surrounding
/// source code context (±2 lines) and visual indicators pointing to the error location.
use crate::error_code::ErrorCode;
/// Extract source code context around an error location
///
/// Returns a formatted string with line numbers showing ±2 lines around the error,
/// handling edge cases like errors on line 1, last line, or files with <3 lines.
///
/// # Arguments
/// * `source` - The complete source code
/// * `error_line` - The 1-based line number where the error occurred
///
/// # Returns
/// A string with formatted lines including line numbers (e.g., "  3 | fn add() {")
pub fn extract_source_context(source: &str, error_line: usize) -> String {
    extract_source_context_with_pointer(source, error_line, None, "")
}

/// Extract source context with optional error pointer
///
/// Shows lines around the error location with proper formatting and line numbers.
/// If column and hint are provided, inserts a caret pointer after the error line.
///
/// # Arguments
/// * `source` - The complete source code
/// * `error_line` - The 1-based line number where the error occurred
/// * `error_column` - Optional 1-based column number for the caret pointer
/// * `hint` - Hint message to show with the pointer
///
/// # Returns
/// A string with formatted lines, including the pointer if column is provided
pub fn extract_source_context_with_pointer(
    source: &str,
    error_line: usize,
    error_column: Option<usize>,
    hint: &str,
) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let total_lines = lines.len();

    if total_lines == 0 {
        return String::new();
    }

    // Calculate line range (±2 lines, clamped to valid range)
    // Lines are 1-based, but vec is 0-based
    let start_line = error_line.saturating_sub(2).max(1);
    let end_line = (error_line + 2).min(total_lines);

    // Calculate line number width for alignment
    let line_num_width = end_line.to_string().len().max(2);

    let mut result = String::new();
    for line_num in start_line..=end_line {
        let line_content = lines[line_num - 1]; // Convert 1-based to 0-based
        result.push_str(&format!(
            "{:>width$} | {}\n",
            line_num,
            line_content,
            width = line_num_width
        ));

        // Insert pointer right after the error line
        if line_num == error_line {
            if let Some(column) = error_column {
                let pointer = format_error_pointer(column, line_num_width, hint);
                result.push_str(&pointer);
            }
        }
    }

    result
}

/// Generate a pointer line indicating the error column
///
/// Creates a line with proper spacing and a caret (^) pointing to the error column,
/// followed by the hint message.
///
/// # Arguments
/// * `column` - The 1-based column number where the error occurred
/// * `line_num_width` - Width of the line number column for alignment
/// * `hint` - The error hint message to display after the caret
///
/// # Returns
/// A formatted pointer line (e.g., "   |     ^ Expected ';'")
pub fn format_error_pointer(column: usize, line_num_width: usize, hint: &str) -> String {
    // Format: "   | " + (column-1 spaces) + "^ " + hint
    let spaces = " ".repeat(column.saturating_sub(1));
    format!(
        "{:>width$} | {}^ {}\n",
        "",
        spaces,
        hint,
        width = line_num_width
    )
}

/// Format a complete error message with context and pointer
///
/// Combines the base error message with source context and a visual pointer
/// to create a user-friendly error display.
///
/// # Arguments
/// * `base_message` - The main error message (e.g., "Expected ';', found '}'")
/// * `source` - The complete source code
/// * `line` - The 1-based line number where the error occurred
/// * `column` - The 1-based column number where the error occurred
/// * `hint` - A helpful hint message for the pointer line
///
/// # Returns
/// A fully formatted error message with context and pointer
///
/// # Example
/// ```
/// use ferrisscript_compiler::error_context::format_error_with_context;
///
/// let source = "fn test() {\n    let x = 10\n}\n";
/// let error = format_error_with_context(
///     "Expected ';', found '}'",
///     source,
///     2,
///     15,
///     "Expected ';' before end of statement"
/// );
/// // Output:
/// // Expected ';', found '}' at line 2, column 15
/// //
/// //  1 | fn test() {
/// //  2 |     let x = 10
/// //    |               ^ Expected ';' before end of statement
/// //  3 | }
/// ```
pub fn format_error_with_context(
    base_message: &str,
    source: &str,
    line: usize,
    column: usize,
    hint: &str,
) -> String {
    let context = extract_source_context(source, line);

    // Calculate line number width from the context
    let lines: Vec<&str> = source.lines().collect();
    let end_line = (line + 2).min(lines.len());
    let line_num_width = end_line.to_string().len().max(2);

    let pointer = format_error_pointer(column, line_num_width, hint);

    format!("{}\n\n{}{}", base_message, context, pointer)
}

/// Format a complete error message with error code, context, and pointer
///
/// Similar to [`format_error_with_context`] but includes a structured error code
/// prefix (e.g., "Error[E201]:") for better error tracking and documentation.
///
/// # Arguments
/// * `code` - The structured error code (e.g., ErrorCode::E201)
/// * `base_message` - The main error message (e.g., "Undefined variable 'x'")
/// * `source` - The complete source code
/// * `line` - The 1-based line number where the error occurred
/// * `column` - The 1-based column number where the error occurred
/// * `hint` - A helpful hint message for the pointer line
///
/// # Returns
/// A fully formatted error message with error code, context, and pointer
///
/// # Example
/// ```
/// use ferrisscript_compiler::error_context::format_error_with_code;
/// use ferrisscript_compiler::error_code::ErrorCode;
///
/// let source = "fn test() {\n    let x = unknown_var;\n}\n";
/// let error = format_error_with_code(
///     ErrorCode::E201,
///     "Undefined variable 'unknown_var'",
///     source,
///     2,
///     13,
///     "Variable must be declared before use"
/// );
/// // Output:
/// // Error[E201]: Undefined variable
/// // Undefined variable 'unknown_var' at line 2, column 13
/// //
/// //  1 | fn test() {
/// //  2 |     let x = unknown_var;
/// //    |             ^ Variable must be declared before use
/// //  3 | }
/// ```
pub fn format_error_with_code(
    code: ErrorCode,
    base_message: &str,
    source: &str,
    line: usize,
    column: usize,
    hint: &str,
) -> String {
    // Extract context with pointer included at the right position
    let context = extract_source_context_with_pointer(source, line, Some(column), hint);

    // Add documentation link
    let docs_url = code.get_docs_url();
    let docs_note = format!("   = note: see {} for more information\n", docs_url);

    format!(
        "Error[{}]: {}\n{}\n\n{}{}",
        code.as_str(),
        code.description(),
        base_message,
        context,
        docs_note
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error_code::ErrorCode;

    #[test]
    fn test_extract_context_normal() {
        let source = "line 1\nline 2\nline 3\nline 4\nline 5\nline 6\nline 7";
        let context = extract_source_context(source, 4);

        assert!(context.contains(" 2 | line 2"));
        assert!(context.contains(" 3 | line 3"));
        assert!(context.contains(" 4 | line 4"));
        assert!(context.contains(" 5 | line 5"));
        assert!(context.contains(" 6 | line 6"));
        assert!(!context.contains(" 1 | line 1"));
        assert!(!context.contains(" 7 | line 7"));
    }

    #[test]
    fn test_extract_context_first_line() {
        let source = "line 1\nline 2\nline 3\nline 4\nline 5";
        let context = extract_source_context(source, 1);

        assert!(context.contains(" 1 | line 1"));
        assert!(context.contains(" 2 | line 2"));
        assert!(context.contains(" 3 | line 3"));
        assert!(!context.contains(" 4 | line 4"));
    }

    #[test]
    fn test_extract_context_last_line() {
        let source = "line 1\nline 2\nline 3\nline 4\nline 5";
        let context = extract_source_context(source, 5);

        assert!(context.contains(" 3 | line 3"));
        assert!(context.contains(" 4 | line 4"));
        assert!(context.contains(" 5 | line 5"));
        assert!(!context.contains(" 1 | line 1"));
        assert!(!context.contains(" 2 | line 2"));
    }

    #[test]
    fn test_extract_context_short_file() {
        let source = "line 1\nline 2";
        let context = extract_source_context(source, 1);

        assert!(context.contains(" 1 | line 1"));
        assert!(context.contains(" 2 | line 2"));
    }

    #[test]
    fn test_extract_context_single_line() {
        let source = "only line";
        let context = extract_source_context(source, 1);

        assert!(context.contains(" 1 | only line"));
    }

    #[test]
    fn test_extract_context_empty_file() {
        let source = "";
        let context = extract_source_context(source, 1);

        assert_eq!(context, "");
    }

    #[test]
    fn test_format_pointer_column_1() {
        let pointer = format_error_pointer(1, 2, "Test message");
        assert!(pointer.contains("   | ^ Test message"));
    }

    #[test]
    fn test_format_pointer_column_10() {
        let pointer = format_error_pointer(10, 2, "Test message");
        assert!(pointer.contains("         ^ Test message"));
    }

    #[test]
    fn test_format_pointer_alignment() {
        let pointer = format_error_pointer(5, 3, "Error here");
        // Should have 3-char line number width + " | " + 4 spaces + "^ Error here"
        assert!(pointer.starts_with("    | "));
        assert!(pointer.contains("^ Error here"));
    }

    #[test]
    fn test_full_error_formatting() {
        let source = "fn test() {\n    let x = 10\n}\n";
        let error = format_error_with_context(
            "Expected ';', found '}' at line 2, column 15",
            source,
            2,
            15,
            "Expected ';' before end of statement",
        );

        // Check all components are present
        assert!(error.contains("Expected ';', found '}'"));
        assert!(error.contains(" 1 | fn test() {"));
        assert!(error.contains(" 2 |     let x = 10"));
        assert!(error.contains(" 3 | }"));
        assert!(error.contains("^ Expected ';' before end of statement"));
    }

    #[test]
    fn test_line_number_alignment_large_files() {
        let mut source = String::new();
        for i in 1..=150 {
            source.push_str(&format!("line {}\n", i));
        }

        let context = extract_source_context(&source, 100);

        // Line numbers should be right-aligned with 3-char width (for 100)
        assert!(context.contains(" 98 | line 98"));
        assert!(context.contains(" 99 | line 99"));
        assert!(context.contains("100 | line 100"));
        assert!(context.contains("101 | line 101"));
        assert!(context.contains("102 | line 102"));
    }

    #[test]
    fn test_format_error_with_code() {
        let source = "fn test() {\n    let x = unknown_var;\n}\n";
        let error = format_error_with_code(
            ErrorCode::E201,
            "Undefined variable 'unknown_var' at line 2, column 13",
            source,
            2,
            13,
            "Variable must be declared before use",
        );

        // Check all components are present
        assert!(error.contains("Error[E201]"));
        assert!(error.contains("Undefined variable"));
        assert!(error.contains("at line 2, column 13"));
        assert!(error.contains(" 1 | fn test() {"));
        assert!(error.contains(" 2 |     let x = unknown_var;"));
        assert!(error.contains(" 3 | }"));
        assert!(error.contains("^ Variable must be declared before use"));
    }

    #[test]
    fn test_error_code_format() {
        let source = "let x = \"unterminated\n";
        let error = format_error_with_code(
            ErrorCode::E002,
            "Unterminated string at line 1, column 9",
            source,
            1,
            9,
            "String must be closed with \"",
        );

        assert!(error.contains("Error[E002]"));
        assert!(error.contains("Unterminated string literal"));
    }
}
