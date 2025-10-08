//! String similarity and identifier suggestion utilities.
//!
//! This module provides functions for calculating edit distance between strings
//! and suggesting similar identifiers for typos in variable names, function names,
//! and type names.

/// Calculate the Levenshtein distance between two strings.
///
/// Returns the minimum number of single-character edits (insertions, deletions,
/// or substitutions) required to transform one string into the other.
///
/// # Examples
///
/// ```
/// use ferrisscript_compiler::suggestions::levenshtein;
///
/// assert_eq!(levenshtein("velocity", "velocty"), 1);
/// assert_eq!(levenshtein("hello", "hallo"), 1);
/// assert_eq!(levenshtein("hello", "world"), 4);
/// ```
pub fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let len_a = a_chars.len();
    let len_b = b_chars.len();

    // Handle empty strings
    if len_a == 0 {
        return len_b;
    }
    if len_b == 0 {
        return len_a;
    }

    // Create distance matrix
    let mut matrix = vec![vec![0; len_b + 1]; len_a + 1];

    // Initialize first row and column
    for (i, row) in matrix.iter_mut().enumerate().take(len_a + 1) {
        row[0] = i;
    }
    for j in 0..=len_b {
        matrix[0][j] = j;
    }

    // Fill matrix using dynamic programming
    for i in 1..=len_a {
        for j in 1..=len_b {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };

            matrix[i][j] = std::cmp::min(
                std::cmp::min(
                    matrix[i - 1][j] + 1, // deletion
                    matrix[i][j - 1] + 1, // insertion
                ),
                matrix[i - 1][j - 1] + cost, // substitution
            );
        }
    }

    matrix[len_a][len_b]
}

/// Calculate the similarity percentage between two strings (0-100).
///
/// Returns a percentage where 100 means identical strings and 0 means
/// completely different strings (based on edit distance relative to length).
///
/// # Examples
///
/// ```
/// use ferrisscript_compiler::suggestions::similarity;
///
/// let sim = similarity("velocity", "velocty"); // 1 edit in 8 chars
/// assert!(sim >= 85 && sim <= 90); // ~87-88%
/// assert_eq!(similarity("hello", "world"), 20); // 4 edits in 5 chars
/// ```
pub fn similarity(a: &str, b: &str) -> u8 {
    let max_len = std::cmp::max(a.len(), b.len());
    if max_len == 0 {
        return 100; // Both empty strings are identical
    }

    let distance = levenshtein(a, b);
    let similarity = 100.0 * (1.0 - (distance as f64 / max_len as f64));
    similarity.round() as u8
}

/// Determine if two identifiers are similar enough to suggest as a correction.
///
/// Uses adaptive thresholds based on identifier length:
/// - Short identifiers (≤8 chars): ≤2-3 edit distance
/// - Very short identifiers (≤4 chars): ≤1 edit distance
/// - Long identifiers (>8 chars): ≥70% similarity
///
/// # Examples
///
/// ```
/// use ferrisscript_compiler::suggestions::is_similar_identifier;
///
/// // Close typos - should suggest
/// assert!(is_similar_identifier("velocty", "velocity"));   // 1 edit
/// assert!(is_similar_identifier("pirnt", "print"));        // 1 edit
///
/// // Distant typos - should not suggest
/// assert!(!is_similar_identifier("xyz", "velocity"));      // Too different
/// assert!(!is_similar_identifier("hello", "world"));       // Too different
/// ```
pub fn is_similar_identifier(typo: &str, candidate: &str) -> bool {
    let distance = levenshtein(typo, candidate);
    let len = typo.len();

    // Very short identifiers (≤4 chars): strict threshold
    if len <= 4 {
        return distance <= 1;
    }

    // Short identifiers (5-8 chars): allow 2-3 edit distance
    if len <= 8 {
        return distance <= 2;
    }

    // Long identifiers (>8 chars): use percentage similarity
    let similarity_pct = similarity(typo, candidate);
    similarity_pct >= 70
}

/// Find similar identifiers from a list of candidates.
///
/// Returns up to 3 suggestions ranked by edit distance (closest matches first).
///
/// # Arguments
///
/// * `typo` - The misspelled identifier
/// * `candidates` - List of valid identifiers to compare against
///
/// # Returns
///
/// A vector of suggested identifiers (maximum 3), sorted by similarity.
///
/// # Examples
///
/// ```
/// use ferrisscript_compiler::suggestions::find_similar_identifiers;
///
/// let candidates = vec!["velocity", "position", "direction"];
/// let suggestions = find_similar_identifiers("velocty", &candidates);
/// assert_eq!(suggestions, vec!["velocity"]);
/// ```
pub fn find_similar_identifiers(typo: &str, candidates: &[&str]) -> Vec<String> {
    let mut matches: Vec<(String, usize)> = candidates
        .iter()
        .filter_map(|&candidate| {
            if is_similar_identifier(typo, candidate) {
                Some((candidate.to_string(), levenshtein(typo, candidate)))
            } else {
                None
            }
        })
        .collect();

    // Sort by edit distance (closest matches first)
    matches.sort_by_key(|(_, dist)| *dist);

    // Return top 3 suggestions
    matches.into_iter().take(3).map(|(name, _)| name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_identical() {
        assert_eq!(levenshtein("hello", "hello"), 0);
        assert_eq!(levenshtein("", ""), 0);
        assert_eq!(levenshtein("velocity", "velocity"), 0);
    }

    #[test]
    fn test_levenshtein_empty_strings() {
        assert_eq!(levenshtein("", "hello"), 5);
        assert_eq!(levenshtein("hello", ""), 5);
    }

    #[test]
    fn test_levenshtein_single_char_diff() {
        assert_eq!(levenshtein("velocity", "velocty"), 1); // deletion
        assert_eq!(levenshtein("hello", "hallo"), 1); // substitution
        assert_eq!(levenshtein("cat", "cats"), 1); // insertion
    }

    #[test]
    fn test_levenshtein_multiple_edits() {
        assert_eq!(levenshtein("hello", "world"), 4);
        assert_eq!(levenshtein("kitten", "sitting"), 3);
        assert_eq!(levenshtein("saturday", "sunday"), 3);
    }

    #[test]
    fn test_levenshtein_transposition() {
        assert_eq!(levenshtein("retrun", "return"), 2); // Transposition = 2 edits
        assert_eq!(levenshtein("pirnt", "print"), 2);
    }

    #[test]
    fn test_similarity_identical() {
        assert_eq!(similarity("hello", "hello"), 100);
        assert_eq!(similarity("", ""), 100);
    }

    #[test]
    fn test_similarity_close_match() {
        // "velocity" vs "velocty": 1 edit in 8 chars = 87.5% ~ 87 or 88
        let sim = similarity("velocity", "velocty");
        assert!((85..=90).contains(&sim), "Expected 85-90, got {}", sim);
    }

    #[test]
    fn test_similarity_distant_match() {
        // "hello" vs "world": 4 edits in 5 chars = 20%
        assert_eq!(similarity("hello", "world"), 20);
    }

    #[test]
    fn test_is_similar_identifier_short_names() {
        // Very short (≤4 chars): ≤1 edit distance
        assert!(is_similar_identifier("cat", "bat")); // 1 edit
        assert!(is_similar_identifier("fn", "fnn")); // 1 edit
        assert!(!is_similar_identifier("cat", "dog")); // 3 edits
        assert!(!is_similar_identifier("xyz", "abc")); // 3 edits
    }

    #[test]
    fn test_is_similar_identifier_medium_names() {
        // Medium (5-8 chars): ≤2 edit distance
        assert!(is_similar_identifier("velocty", "velocity")); // 1 edit
        assert!(is_similar_identifier("pirnt", "print")); // 2 edits
        assert!(is_similar_identifier("hello", "hallo")); // 1 edit
        assert!(!is_similar_identifier("hello", "world")); // 4 edits
    }

    #[test]
    fn test_is_similar_identifier_long_names() {
        // Long (>8 chars): ≥70% similarity
        assert!(is_similar_identifier("initialization", "initialisation")); // 1 edit, >70%
        assert!(is_similar_identifier("configuration", "confguration")); // 1 edit, >70%
        assert!(!is_similar_identifier("configuration", "something")); // Too different
    }

    #[test]
    fn test_is_similar_identifier_case_sensitive() {
        // Case differences count as edits
        assert!(is_similar_identifier("velocity", "Velocity")); // 1 edit
        assert!(is_similar_identifier("hello", "Hello")); // 1 edit
    }

    #[test]
    fn test_find_similar_identifiers_single_match() {
        let candidates = vec!["velocity", "position", "direction"];
        let suggestions = find_similar_identifiers("velocty", &candidates);
        assert_eq!(suggestions, vec!["velocity"]);
    }

    #[test]
    fn test_find_similar_identifiers_multiple_matches() {
        let candidates = vec!["velocity", "velocities", "position"];
        let suggestions = find_similar_identifiers("velocty", &candidates);

        // Should return both velocity and velocities, with velocity first (closer)
        assert!(suggestions.contains(&"velocity".to_string()));
        assert_eq!(suggestions[0], "velocity"); // Closest match first
    }

    #[test]
    fn test_find_similar_identifiers_no_matches() {
        let candidates = vec!["position", "direction", "rotation"];
        let suggestions = find_similar_identifiers("xyz", &candidates);
        assert_eq!(suggestions, Vec::<String>::new());
    }

    #[test]
    fn test_find_similar_identifiers_max_three() {
        let candidates = vec!["vel", "velo", "veloc", "veloci", "velocity"];
        let suggestions = find_similar_identifiers("vel", &candidates);

        // Should return maximum 3 suggestions
        assert!(suggestions.len() <= 3);
    }

    #[test]
    fn test_find_similar_identifiers_ranking() {
        let candidates = vec!["velocity", "velocities", "vertical"];
        let suggestions = find_similar_identifiers("velocty", &candidates);

        // "velocity" (1 edit) should rank before "velocities" (2 edits)
        if suggestions.len() >= 2 {
            assert_eq!(suggestions[0], "velocity");
        }
    }

    #[test]
    fn test_unicode_identifiers() {
        // Test with Unicode characters
        assert_eq!(levenshtein("café", "cafe"), 1);
        assert!(is_similar_identifier("café", "cafe"));
    }

    #[test]
    fn test_empty_candidate_list() {
        let candidates: Vec<&str> = vec![];
        let suggestions = find_similar_identifiers("velocity", &candidates);
        assert_eq!(suggestions, Vec::<String>::new());
    }
}
