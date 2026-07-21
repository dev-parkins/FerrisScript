//! Source code location tracking for precise error messages and LSP support.
//!
//! This module provides types for tracking the exact location of AST nodes in source code.
//! Every AST node has a [`Span`] that records where it appears in the original source,
//! enabling precise error messages and LSP features like go-to-definition.
//!
//! # Overview
//!
//! - [`Position`]: A single point in source code (line, column, byte offset)
//! - [`Span`]: A range in source code (start and end positions)
//!
//! # Examples
//!
//! ```
//! use ferrisscript_compiler::span::{Position, Span};
//!
//! // Create a position at line 5, column 10, byte offset 42
//! let pos = Position::new(5, 10, 42);
//!
//! // Create a span from two positions
//! let start = Position::new(5, 10, 42);
//! let end = Position::new(5, 15, 47);
//! let span = Span::new(start, end);
//!
//! // Merge two spans to get the encompassing range
//! let span1 = Span::new(Position::new(1, 0, 0), Position::new(1, 5, 5));
//! let span2 = Span::new(Position::new(1, 10, 10), Position::new(1, 15, 15));
//! let merged = span1.merge(span2);
//! assert_eq!(merged.start.column, 0);
//! assert_eq!(merged.end.column, 15);
//! ```

use std::fmt;

/// A position in source code (line, column, and byte offset).
///
/// Positions are 1-indexed for line and column (matching editor conventions),
/// and 0-indexed for byte offset (matching Rust string indexing).
///
/// # Examples
///
/// ```
/// use ferrisscript_compiler::span::Position;
///
/// let pos = Position::new(10, 5, 123);
/// assert_eq!(pos.line, 10);
/// assert_eq!(pos.column, 5);
/// assert_eq!(pos.offset, 123);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    /// Line number (1-indexed, first line is 1)
    pub line: usize,
    /// Column number (1-indexed, first column is 1)
    pub column: usize,
    /// Byte offset from start of file (0-indexed)
    pub offset: usize,
}

impl Position {
    /// Create a new position.
    ///
    /// # Arguments
    ///
    /// * `line` - Line number (1-indexed)
    /// * `column` - Column number (1-indexed)
    /// * `offset` - Byte offset from start of file (0-indexed)
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisscript_compiler::span::Position;
    ///
    /// let pos = Position::new(1, 1, 0); // First character of file
    /// ```
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Position {
            line,
            column,
            offset,
        }
    }

    /// Create an unknown position (used as placeholder).
    ///
    /// Returns position (0, 0, 0) which is invalid but recognizable.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisscript_compiler::span::Position;
    ///
    /// let pos = Position::unknown();
    /// assert_eq!(pos.line, 0);
    /// ```
    pub fn unknown() -> Self {
        Position {
            line: 0,
            column: 0,
            offset: 0,
        }
    }

    /// Check if this position is unknown (placeholder).
    pub fn is_unknown(&self) -> bool {
        self.line == 0 && self.column == 0 && self.offset == 0
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// A span representing a range in source code.
///
/// Spans track the start and end positions of AST nodes, enabling precise
/// error messages and LSP features. Every AST node should have an associated span.
///
/// # Invariants
///
/// - `start` should come before or equal to `end` in the source
/// - For single-token spans, `start` and `end` may be equal
///
/// # Examples
///
/// ```
/// use ferrisscript_compiler::span::{Position, Span};
///
/// // Span for "hello" at line 1, columns 5-9
/// let span = Span::new(
///     Position::new(1, 5, 4),
///     Position::new(1, 9, 8)
/// );
///
/// assert_eq!(span.len(), 4); // Offsets 4 to 8 = 4 bytes
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    /// Start position of the span (inclusive)
    pub start: Position,
    /// End position of the span (exclusive)
    pub end: Position,
}

impl Span {
    /// Create a new span from start and end positions.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisscript_compiler::span::{Position, Span};
    ///
    /// let span = Span::new(
    ///     Position::new(1, 1, 0),
    ///     Position::new(1, 6, 5)
    /// );
    /// ```
    pub fn new(start: Position, end: Position) -> Self {
        Span { start, end }
    }

    /// Create a span from a single position (zero-length span).
    ///
    /// Useful for punctuation tokens or error markers.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisscript_compiler::span::{Position, Span};
    ///
    /// let pos = Position::new(5, 10, 42);
    /// let span = Span::point(pos);
    /// assert_eq!(span.start, span.end);
    /// ```
    pub fn point(pos: Position) -> Self {
        Span {
            start: pos,
            end: pos,
        }
    }

    /// Create an unknown span (used as placeholder).
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisscript_compiler::span::Span;
    ///
    /// let span = Span::unknown();
    /// assert!(span.is_unknown());
    /// ```
    pub fn unknown() -> Self {
        Span {
            start: Position::unknown(),
            end: Position::unknown(),
        }
    }

    /// Check if this span is unknown (placeholder).
    pub fn is_unknown(&self) -> bool {
        self.start.is_unknown() && self.end.is_unknown()
    }

    /// Merge this span with another, creating a span that encompasses both.
    ///
    /// The resulting span starts at the earlier start position and ends at
    /// the later end position.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisscript_compiler::span::{Position, Span};
    ///
    /// let span1 = Span::new(Position::new(1, 5, 4), Position::new(1, 10, 9));
    /// let span2 = Span::new(Position::new(1, 15, 14), Position::new(1, 20, 19));
    /// let merged = span1.merge(span2);
    ///
    /// assert_eq!(merged.start.column, 5);
    /// assert_eq!(merged.end.column, 20);
    /// ```
    pub fn merge(self, other: Span) -> Span {
        use std::cmp::{max, min};

        Span {
            start: min(self.start, other.start),
            end: max(self.end, other.end),
        }
    }

    /// Get the length of this span in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisscript_compiler::span::{Position, Span};
    ///
    /// let span = Span::new(Position::new(1, 1, 0), Position::new(1, 6, 5));
    /// assert_eq!(span.len(), 5);
    /// ```
    pub fn len(&self) -> usize {
        self.end.offset.saturating_sub(self.start.offset)
    }

    /// Check if this span is empty (zero length).
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisscript_compiler::span::{Position, Span};
    ///
    /// let pos = Position::new(1, 1, 0);
    /// let span = Span::point(pos);
    /// assert!(span.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Check if this span contains a position.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisscript_compiler::span::{Position, Span};
    ///
    /// let span = Span::new(Position::new(1, 5, 4), Position::new(1, 10, 9));
    /// let pos = Position::new(1, 7, 6);
    /// assert!(span.contains(pos));
    /// ```
    pub fn contains(&self, pos: Position) -> bool {
        self.start <= pos && pos < self.end
    }

    /// Create a span from line and column numbers (for backward compatibility).
    ///
    /// Creates a zero-length span at the given line and column.
    /// Offset is set to 0 (unknown).
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisscript_compiler::span::Span;
    ///
    /// let span = Span::from_line_col(10, 5);
    /// assert_eq!(span.start.line, 10);
    /// assert_eq!(span.start.column, 5);
    /// ```
    #[deprecated(since = "0.0.5", note = "Use Span::new with Position instead")]
    pub fn from_line_col(line: usize, column: usize) -> Self {
        let pos = Position::new(line, column, 0);
        Span::point(pos)
    }

    /// Get the line number where this span starts (for backward compatibility).
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisscript_compiler::span::{Position, Span};
    ///
    /// let span = Span::new(Position::new(5, 1, 20), Position::new(6, 1, 30));
    /// assert_eq!(span.line(), 5);
    /// ```
    pub fn line(&self) -> usize {
        self.start.line
    }

    /// Get the column number where this span starts (for backward compatibility).
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisscript_compiler::span::{Position, Span};
    ///
    /// let span = Span::new(Position::new(5, 10, 42), Position::new(5, 15, 47));
    /// assert_eq!(span.column(), 10);
    /// ```
    pub fn column(&self) -> usize {
        self.start.column
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start.line == self.end.line {
            write!(
                f,
                "line {}, columns {}-{}",
                self.start.line, self.start.column, self.end.column
            )
        } else {
            write!(f, "{} to {}", self.start, self.end)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_new() {
        let pos = Position::new(10, 5, 42);
        assert_eq!(pos.line, 10);
        assert_eq!(pos.column, 5);
        assert_eq!(pos.offset, 42);
    }

    #[test]
    fn test_position_unknown() {
        let pos = Position::unknown();
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 0);
        assert_eq!(pos.offset, 0);
        assert!(pos.is_unknown());
    }

    #[test]
    fn test_position_display() {
        let pos = Position::new(10, 5, 42);
        assert_eq!(format!("{}", pos), "10:5");
    }

    #[test]
    fn test_position_ordering() {
        let pos1 = Position::new(1, 5, 4);
        let pos2 = Position::new(1, 10, 9);
        let pos3 = Position::new(2, 1, 15);

        assert!(pos1 < pos2);
        assert!(pos2 < pos3);
        assert!(pos1 < pos3);
    }

    #[test]
    fn test_span_new() {
        let start = Position::new(1, 5, 4);
        let end = Position::new(1, 10, 9);
        let span = Span::new(start, end);

        assert_eq!(span.start, start);
        assert_eq!(span.end, end);
    }

    #[test]
    fn test_span_point() {
        let pos = Position::new(5, 10, 42);
        let span = Span::point(pos);

        assert_eq!(span.start, pos);
        assert_eq!(span.end, pos);
        assert_eq!(span.len(), 0);
        assert!(span.is_empty());
    }

    #[test]
    fn test_span_unknown() {
        let span = Span::unknown();
        assert!(span.is_unknown());
        assert_eq!(span.start.line, 0);
        assert_eq!(span.end.line, 0);
    }

    #[test]
    fn test_span_merge() {
        let span1 = Span::new(Position::new(1, 5, 4), Position::new(1, 10, 9));
        let span2 = Span::new(Position::new(1, 15, 14), Position::new(1, 20, 19));
        let merged = span1.merge(span2);

        assert_eq!(merged.start.column, 5);
        assert_eq!(merged.end.column, 20);
        assert_eq!(merged.start.offset, 4);
        assert_eq!(merged.end.offset, 19);
    }

    #[test]
    fn test_span_merge_overlapping() {
        let span1 = Span::new(Position::new(1, 5, 4), Position::new(1, 15, 14));
        let span2 = Span::new(Position::new(1, 10, 9), Position::new(1, 20, 19));
        let merged = span1.merge(span2);

        assert_eq!(merged.start.column, 5);
        assert_eq!(merged.end.column, 20);
    }

    #[test]
    fn test_span_merge_reverse_order() {
        let span1 = Span::new(Position::new(1, 15, 14), Position::new(1, 20, 19));
        let span2 = Span::new(Position::new(1, 5, 4), Position::new(1, 10, 9));
        let merged = span1.merge(span2);

        // Should still produce same result regardless of order
        assert_eq!(merged.start.column, 5);
        assert_eq!(merged.end.column, 20);
    }

    #[test]
    fn test_span_len() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 6, 5));
        assert_eq!(span.len(), 5);
    }

    #[test]
    fn test_span_len_multiline() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(3, 1, 20));
        assert_eq!(span.len(), 20);
    }

    #[test]
    fn test_span_is_empty() {
        let pos = Position::new(1, 1, 0);
        let span = Span::point(pos);
        assert!(span.is_empty());

        let span2 = Span::new(Position::new(1, 1, 0), Position::new(1, 6, 5));
        assert!(!span2.is_empty());
    }

    #[test]
    fn test_span_contains() {
        let span = Span::new(Position::new(1, 5, 4), Position::new(1, 10, 9));

        // Inside span
        assert!(span.contains(Position::new(1, 7, 6)));

        // At start (inclusive)
        assert!(span.contains(Position::new(1, 5, 4)));

        // At end (exclusive)
        assert!(!span.contains(Position::new(1, 10, 9)));

        // Before span
        assert!(!span.contains(Position::new(1, 3, 2)));

        // After span
        assert!(!span.contains(Position::new(1, 12, 11)));
    }

    #[test]
    fn test_span_display_single_line() {
        let span = Span::new(Position::new(5, 10, 42), Position::new(5, 15, 47));
        assert_eq!(format!("{}", span), "line 5, columns 10-15");
    }

    #[test]
    fn test_span_display_multi_line() {
        let span = Span::new(Position::new(5, 10, 42), Position::new(7, 5, 67));
        assert_eq!(format!("{}", span), "5:10 to 7:5");
    }

    #[test]
    fn test_span_backward_compatibility() {
        #[allow(deprecated)]
        let span = Span::from_line_col(10, 5);
        assert_eq!(span.line(), 10);
        assert_eq!(span.column(), 5);
    }

    #[test]
    fn test_span_line_column_accessors() {
        let span = Span::new(Position::new(10, 5, 42), Position::new(10, 15, 52));
        assert_eq!(span.line(), 10);
        assert_eq!(span.column(), 5);
    }

    #[test]
    fn test_span_merge_multiline() {
        let span1 = Span::new(Position::new(1, 5, 4), Position::new(2, 10, 25));
        let span2 = Span::new(Position::new(3, 1, 30), Position::new(4, 5, 50));
        let merged = span1.merge(span2);

        assert_eq!(merged.start.line, 1);
        assert_eq!(merged.start.column, 5);
        assert_eq!(merged.end.line, 4);
        assert_eq!(merged.end.column, 5);
    }

    #[test]
    fn test_span_contains_multiline() {
        let span = Span::new(Position::new(5, 10, 50), Position::new(7, 5, 100));

        // Inside on first line
        assert!(span.contains(Position::new(5, 15, 55)));

        // Inside on middle line
        assert!(span.contains(Position::new(6, 1, 70)));

        // Inside on last line (before end)
        assert!(span.contains(Position::new(7, 3, 98)));

        // At end (exclusive)
        assert!(!span.contains(Position::new(7, 5, 100)));

        // After span
        assert!(!span.contains(Position::new(7, 10, 105)));
    }
}
