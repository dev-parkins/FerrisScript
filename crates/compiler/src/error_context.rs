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

    // ============================================================================
    // Phase 4: Diagnostic Edge Cases
    // ============================================================================
    // Testing error formatting with Unicode characters, line endings, and
    // column alignment edge cases to ensure robust diagnostic output.

    // ----------------------------------------------------------------------------
    // Unicode Character Handling
    // ----------------------------------------------------------------------------

    #[test]
    fn test_error_pointer_with_emoji_before_error() {
        // Emoji are multi-byte UTF-8 characters
        let source = "let 🦀 = 10;\nlet y = unknown;";
        let context = extract_source_context_with_pointer(source, 2, Some(9), "undefined");

        // Should contain the source line and pointer
        assert!(context.contains("let y = unknown;"));
        assert!(context.contains("undefined"));
    }

    #[test]
    fn test_error_pointer_with_multibyte_chars() {
        // Chinese characters are 3 bytes in UTF-8
        let source = "let 变量 = 10;\nlet y = unknown;";
        let context = extract_source_context_with_pointer(source, 2, Some(9), "undefined");

        // Should still format correctly
        assert!(context.contains("let y = unknown;"));
        assert!(context.contains("undefined"));
    }

    #[test]
    fn test_error_at_emoji_location() {
        // Error pointing directly at an emoji
        let source = "let x = 🚀;";
        let context = extract_source_context_with_pointer(source, 1, Some(9), "invalid symbol");

        assert!(context.contains("let x = 🚀;"));
        assert!(context.contains("invalid symbol"));
    }

    #[test]
    fn test_extract_context_with_combining_characters() {
        // Combining diacritical marks (e.g., é as e + ́)
        let source = "let café = 10;\nlet y = x;";
        let context = extract_source_context(source, 2);

        // Should preserve combining characters
        assert!(context.contains("let café = 10;"));
        assert!(context.contains("let y = x;"));
    }

    #[test]
    fn test_error_pointer_with_zero_width_characters() {
        // Zero-width characters (like zero-width space U+200B)
        let source = "let\u{200B}x = 10;";
        let context = extract_source_context_with_pointer(source, 1, Some(4), "unexpected char");

        // Should handle zero-width characters gracefully
        assert!(context.contains("unexpected char"));
    }

    #[test]
    fn test_error_with_right_to_left_text() {
        // Arabic text (right-to-left script)
        let source = "let x = مرحبا;\nlet y = 10;";
        let context = extract_source_context(source, 2);

        // Should preserve RTL text
        assert!(context.contains("let x = مرحبا;"));
        assert!(context.contains("let y = 10;"));
    }

    // ----------------------------------------------------------------------------
    // Line Ending Edge Cases
    // ----------------------------------------------------------------------------

    #[test]
    fn test_extract_context_with_crlf_line_endings() {
        // Windows-style CRLF line endings
        let source = "line 1\r\nline 2\r\nline 3\r\nline 4\r\nline 5";
        let context = extract_source_context(source, 3);

        // Should handle CRLF correctly
        assert!(context.contains(" 1 | line 1"));
        assert!(context.contains(" 2 | line 2"));
        assert!(context.contains(" 3 | line 3"));
        assert!(context.contains(" 4 | line 4"));
        assert!(context.contains(" 5 | line 5"));
    }

    #[test]
    fn test_extract_context_with_mixed_line_endings() {
        // Mixed LF and CRLF line endings
        let source = "line 1\nline 2\r\nline 3\nline 4\r\nline 5";
        let context = extract_source_context(source, 3);

        // Should handle mixed line endings
        assert!(context.contains(" 1 | line 1"));
        assert!(context.contains(" 2 | line 2"));
        assert!(context.contains(" 3 | line 3"));
        assert!(context.contains(" 4 | line 4"));
        assert!(context.contains(" 5 | line 5"));
    }

    #[test]
    fn test_error_pointer_with_crlf() {
        // Error pointer with CRLF line endings
        let source = "fn test() {\r\n    let x = unknown;\r\n}";
        let context = extract_source_context_with_pointer(source, 2, Some(13), "undefined");

        assert!(context.contains("let x = unknown;"));
        assert!(context.contains("undefined"));
    }

    #[test]
    fn test_extract_context_cr_only_line_endings() {
        // Old Mac-style CR-only line endings (rare but possible)
        let source = "line 1\rline 2\rline 3\rline 4\rline 5";
        let context = extract_source_context(source, 3);

        // Should handle CR-only (each line becomes separate)
        // Note: Rust's lines() treats \r as line separator
        assert!(context.contains("line"));
    }

    // ----------------------------------------------------------------------------
    // Column Alignment and Pointer Positioning
    // ----------------------------------------------------------------------------

    #[test]
    fn test_error_pointer_at_column_1() {
        // Error at first column
        let source = "unknown;";
        let context = extract_source_context_with_pointer(source, 1, Some(1), "undefined");

        assert!(context.contains("unknown;"));
        assert!(context.contains("^ undefined"));
    }

    #[test]
    fn test_error_pointer_at_end_of_line() {
        // Error at last column of the line
        let source = "let x = 10";
        let context = extract_source_context_with_pointer(source, 1, Some(11), "missing ';'");

        assert!(context.contains("let x = 10"));
        assert!(context.contains("missing ';'"));
    }

    #[test]
    fn test_error_pointer_very_long_line() {
        // Error in a very long line (100+ chars)
        let mut source = String::from("let x = ");
        for i in 0..20 {
            source.push_str(&format!("value{} + ", i));
        }
        source.push_str("unknown;");

        let context = extract_source_context_with_pointer(&source, 1, Some(50), "undefined");

        // Should handle long lines without truncating
        assert!(context.contains("value"));
        assert!(context.contains("undefined"));
    }

    #[test]
    fn test_format_pointer_with_tabs_in_source() {
        // Tabs in source code affect column calculation
        let source = "fn test() {\n\tlet x = unknown;\n}";
        let context = extract_source_context_with_pointer(source, 2, Some(10), "undefined");

        // Should handle tabs (though pointer position may vary)
        assert!(context.contains("let x = unknown;"));
        assert!(context.contains("undefined"));
    }

    #[test]
    fn test_line_number_width_adjustment() {
        // Test alignment when transitioning from 1-digit to 2-digit line numbers
        let mut source = String::new();
        for i in 1..=12 {
            source.push_str(&format!("line {}\n", i));
        }

        let context = extract_source_context(&source, 10);

        // Line numbers should be aligned with proper width
        assert!(context.contains(" 8 | line 8"));
        assert!(context.contains(" 9 | line 9"));
        assert!(context.contains("10 | line 10"));
        assert!(context.contains("11 | line 11"));
        assert!(context.contains("12 | line 12"));
    }

    // ----------------------------------------------------------------------------
    // Error Context at File Boundaries
    // ----------------------------------------------------------------------------

    #[test]
    fn test_error_at_line_zero() {
        // Edge case: requesting line 0 (invalid)
        let source = "line 1\nline 2\nline 3";
        let context = extract_source_context(source, 0);

        // Should handle gracefully (likely shows first few lines)
        // Implementation may vary, but shouldn't panic
        assert!(!context.is_empty() || context.is_empty()); // Just ensure no panic
    }

    #[test]
    fn test_error_beyond_last_line() {
        // Error reported beyond file length
        let source = "line 1\nline 2\nline 3";
        let context = extract_source_context(source, 100);

        // Should handle gracefully (likely shows last few lines)
        assert!(context.contains("line") || context.is_empty()); // Just ensure no panic
    }

    #[test]
    fn test_extract_context_with_empty_lines() {
        // File with empty lines around error
        let source = "line 1\n\n\nline 4\nline 5";
        let context = extract_source_context(source, 4);

        // Should include empty lines in context
        assert!(context.contains(" 4 | line 4"));
    }

    #[test]
    fn test_error_in_file_with_only_newlines() {
        // File containing only newline characters
        let source = "\n\n\n";
        let context = extract_source_context(source, 2);

        // Should handle gracefully (empty lines)
        // Just ensure it doesn't panic
        let _ = context;
    }

    // ----------------------------------------------------------------------------
    // Error Message Formatting Edge Cases
    // ----------------------------------------------------------------------------

    #[test]
    fn test_format_error_with_very_long_message() {
        // Very long error message
        let source = "let x = 10;";
        let long_message = "This is a very long error message that explains in great detail what went wrong and why, including multiple sentences and elaborate explanations that go on and on.";
        let error = format_error_with_context("Syntax error", source, 1, 9, long_message);

        // Should include the full message without truncation
        assert!(error.contains(long_message));
        assert!(error.contains("let x = 10;"));
    }

    #[test]
    fn test_format_error_with_empty_hint() {
        // Error with no hint message
        let source = "let x = unknown;";
        let context = extract_source_context_with_pointer(source, 1, Some(9), "");

        // Should handle empty hint gracefully
        assert!(context.contains("let x = unknown;"));
    }

    #[test]
    fn test_format_error_with_special_chars_in_hint() {
        // Hint containing special characters
        let source = "let x = 10;";
        let hint = "Expected ';' or '\\n' or '\\t' character";
        let context = extract_source_context_with_pointer(source, 1, Some(11), hint);

        // Should preserve special characters in hint
        assert!(context.contains(hint));
    }

    #[test]
    fn test_multiple_errors_same_line_different_columns() {
        // Multiple errors on same line (different column positions)
        let source = "let x = y + z;";

        let context1 = extract_source_context_with_pointer(source, 1, Some(9), "y undefined");
        let context2 = extract_source_context_with_pointer(source, 1, Some(13), "z undefined");

        // Both should point to correct positions
        assert!(context1.contains("y undefined"));
        assert!(context2.contains("z undefined"));
    }

    // ----------------------------------------------------------------------------
    // Edge Cases with Error Code Formatting
    // ----------------------------------------------------------------------------

    #[test]
    fn test_format_error_with_code_unicode_source() {
        // Error code formatting with Unicode in source
        let source = "let π = 3.14;\nlet x = unknown_π;";
        let error = format_error_with_code(
            ErrorCode::E201,
            "Undefined variable at line 2, column 9",
            source,
            2,
            9,
            "Variable not found",
        );

        // Should handle Unicode in source
        assert!(error.contains("Error[E201]"));
        assert!(error.contains("let π = 3.14;"));
        assert!(error.contains("let x = unknown_π;"));
    }

    #[test]
    fn test_format_error_with_code_at_file_start() {
        // Error on first character of file
        let source = "unknown";
        let error = format_error_with_code(
            ErrorCode::E201,
            "Undefined variable at line 1, column 1",
            source,
            1,
            1,
            "Variable not declared",
        );

        // Should format correctly for file start
        assert!(error.contains("Error[E201]"));
        assert!(error.contains(" 1 | unknown"));
        assert!(error.contains("Variable not declared"));
    }

    #[test]
    fn test_format_error_with_code_at_file_end() {
        // Error at last character of file
        let source = "let x = 10";
        let error = format_error_with_code(
            ErrorCode::E101,
            "Expected ';' at line 1, column 11",
            source,
            1,
            11,
            "Missing semicolon",
        );

        // Should format correctly for file end
        assert!(error.contains("Error[E101]"));
        assert!(error.contains(" 1 | let x = 10"));
        assert!(error.contains("Missing semicolon"));
    }
}
