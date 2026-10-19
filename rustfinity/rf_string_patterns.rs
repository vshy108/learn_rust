//! # Rustfinity: String Patterns
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/string-patterns
//!
//! Searching for patterns within strings is a fundamental text processing task. Rust's standard library provides several powerful methods for finding substrings, checking prefixes and suffixes, and locating specific patterns within text.
//!
//! ## Pattern Matching Methods
//!
//! Rust's `str` type offers these key pattern-matching methods:
//!
//! 1. **`.starts_with(pattern)`** - Check if a string begins with a pattern
//! 2. **`.ends_with(pattern)`** - Check if a string ends with a pattern
//! 3. **`.contains(pattern)`** - Check if a pattern exists anywhere in the string
//! 4. **`.find(pattern)`** - Find the first occurrence, returns `Option<usize>` (byte index)
//! 5. **`.rfind(pattern)`** - Find the last occurrence, returns `Option<usize>` (byte index)
//! 6. **`.matches(pattern)`** - Iterate over all matches of a pattern
//! 7. **`.match_indices(pattern)`** - Iterate over matches with their byte positions
//!
//! These methods accept various pattern types: `&str`, `char`, or closures that take a `char` and return `bool`.
//!
//! ```rust
//! let text = "hello world, hello universe";
//!
//! // Prefix/suffix checks
//! assert!(text.starts_with("hello"));
//! assert!(text.ends_with("universe"));
//!
//! // Finding patterns
//! assert!(text.contains("world"));
//! assert_eq!(text.find("hello"), Some(0));
//! assert_eq!(text.rfind("hello"), Some(13));
//!
//! // Counting matches
//! assert_eq!(text.matches("hello").count(), 2);
//! ```
//!
//! ## Working with Indices
//!
//! The `.find()` and `.rfind()` methods return byte indices, not character indices. For ASCII strings these are the same, but for Unicode strings you need to be careful:
//!
//! ```rust
//! let text = "café";
//! // byte index, not char index
//! assert_eq!(
//!     text.find('é'),
//!     Some(3)
//! );
//! ```
//!
//! ## Your Task
//!
//! Implement the following functions for string pattern matching:
//!
//! 1. `has_prefix(s: &str, prefix: &str) -> bool` - Check if string starts with the given prefix
//! 2. `has_suffix(s: &str, suffix: &str) -> bool` - Check if string ends with the given suffix
//! 3. `find_first(s: &str, pattern: &str) -> Option<usize>` - Find first occurrence of pattern (byte index)
//! 4. `find_last(s: &str, pattern: &str) -> Option<usize>` - Find last occurrence of pattern (byte index)
//! 5. `count_occurrences(s: &str, pattern: &str) -> usize` - Count how many times pattern appears
//! 6. `find_all_indices(s: &str, pattern: &str) -> Vec<usize>` - Find all byte indices where pattern occurs
//! 7. `extract_between(s: &str, start: &str, end: &str) -> Option<String>` - Extract text between first occurrence of start and end markers
//!
//! ## Examples
//!
//! ```rust
//! // has_prefix
//! assert!(has_prefix("hello world", "hello"));
//! assert!(!has_prefix("hello world", "world"));
//! // empty prefix matches empty string
//! assert!(has_prefix("", ""));
//!
//! // has_suffix
//! assert!(has_suffix("hello world", "world"));
//! assert!(!has_suffix("hello world", "hello"));
//! assert!(has_suffix("test.rs", ".rs"));
//!
//! // find_first
//! assert_eq!(find_first("hello hello", "hello"), Some(0));
//! assert_eq!(find_first("hello world", "xyz"), None);
//! assert_eq!(find_first("abcabc", "bc"), Some(1));
//!
//! // find_last
//! assert_eq!(find_last("hello hello", "hello"), Some(6));
//! assert_eq!(find_last("hello world", "o"), Some(7));
//! assert_eq!(find_last("test", "xyz"), None);
//!
//! // count_occurrences
//! assert_eq!(count_occurrences("abababab", "ab"), 4);
//! assert_eq!(count_occurrences("hello world", "o"), 2);
//! assert_eq!(count_occurrences("test", "xyz"), 0);
//!
//! // find_all_indices
//! assert_eq!(
//!     find_all_indices("abcabc", "abc"),
//!     vec![0, 3]
//! );
//! assert_eq!(
//!     find_all_indices("hello", "l"),
//!     vec![2, 3]
//! );
//! assert_eq!(
//!     find_all_indices("test", "xyz"),
//!     vec![]
//! );
//!
//! // extract_between
//! assert_eq!(
//!     extract_between(
//!         "<tag>content</tag>",
//!         "<tag>",
//!         "</tag>"
//!     ),
//!     Some("content".to_string())
//! );
//! assert_eq!(
//!     extract_between("hello [world] test", "[", "]"),
//!     Some("world".to_string())
//! );
//! assert_eq!(
//!     extract_between("no markers here", "[", "]"),
//!     None
//! );
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - For `has_prefix` and `has_suffix`, use `.starts_with()` and `.ends_with()` directly
//! - For `find_first` and `find_last`, use `.find()` and `.rfind()`
//! - For `count_occurrences`, use `.matches(pattern).count()`
//! - For `find_all_indices`, use `.match_indices(pattern)`
//!   and collect the indices
//! - For `extract_between`:
//!   1. Use `.find()` to locate the start marker
//!   2. Calculate where the content begins (after the start marker)
//!   3. Use slicing and `.find()` again to locate the end marker
//!   4. Extract the substring between them
//!
//! </details>
//!
//! Run: `cargo run --example rf_string_patterns`
//! Test: `cargo test --example rf_string_patterns`
//! Filter: `cargo test --example rf_string_patterns <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

/// Check if a string starts with the given prefix.
pub fn has_prefix(s: &str, prefix: &str) -> bool {
    // TODO: Check for a prefix
    unimplemented!()
}

/// Check if a string ends with the given suffix.
pub fn has_suffix(s: &str, suffix: &str) -> bool {
    // TODO: Check for a suffix
    unimplemented!()
}

/// Find the first occurrence of a pattern in the string.
/// Returns the byte index of the match, or None if not found.
pub fn find_first(s: &str, pattern: &str) -> Option<usize> {
    // TODO: Find the first occurrence
    unimplemented!()
}

/// Find the last occurrence of a pattern in the string.
/// Returns the byte index of the match, or None if not found.
pub fn find_last(s: &str, pattern: &str) -> Option<usize> {
    // TODO: Find the last occurrence
    unimplemented!()
}

/// Count how many times a pattern appears in the string.
pub fn count_occurrences(s: &str, pattern: &str) -> usize {
    // TODO: Count pattern matches
    unimplemented!()
}

/// Find all byte indices where the pattern occurs.
pub fn find_all_indices(s: &str, pattern: &str) -> Vec<usize> {
    // TODO: Find all positions
    unimplemented!()
}

/// Extract text between the first occurrence of start and end markers.
/// Returns None if either marker is not found or if end comes before start.
pub fn extract_between(s: &str, start: &str, end: &str) -> Option<String> {
    // TODO: Find the start and end markers, return text between them
    unimplemented!()
}

pub fn main() {
    let text = "hello world, hello universe";

    println!("Text: '{}'", text);
    println!();

    // Test your implementations
    println!("has_prefix('hello'): {}", has_prefix(text, "hello"));
    println!("has_suffix('universe'): {}", has_suffix(text, "universe"));
    println!("find_first('hello'): {:?}", find_first(text, "hello"));
    println!("find_last('hello'): {:?}", find_last(text, "hello"));
    println!(
        "count_occurrences('hello'): {}",
        count_occurrences(text, "hello")
    );
    println!(
        "find_all_indices('hello'): {:?}",
        find_all_indices(text, "hello")
    );

    let html = "<title>My Page</title>";
    println!();
    println!("HTML: '{}'", html);
    println!(
        "extract_between('<title>', '</title>'): {:?}",
        extract_between(html, "<title>", "</title>")
    );
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    // ============================================================================
    // has_prefix tests
    // ============================================================================

    #[test]
    fn test_has_prefix_basic() {
        assert!(has_prefix("hello world", "hello"));
        assert!(has_prefix("hello world", "hello "));
        assert!(has_prefix("hello world", "hello world"));
    }

    #[test]
    fn test_has_prefix_negative() {
        assert!(!has_prefix("hello world", "world"));
        assert!(!has_prefix("hello world", "ello"));
        assert!(!has_prefix("hello", "hello world"));
    }

    #[test]
    fn test_has_prefix_empty() {
        assert!(has_prefix("hello", ""));
        assert!(has_prefix("", ""));
        assert!(!has_prefix("", "hello"));
    }

    #[test]
    fn test_has_prefix_single_char() {
        assert!(has_prefix("hello", "h"));
        assert!(!has_prefix("hello", "e"));
    }

    #[test]
    fn test_has_prefix_unicode() {
        assert!(has_prefix("café", "caf"));
        assert!(has_prefix("日本語", "日本"));
        assert!(!has_prefix("日本語", "語"));
    }

    // ============================================================================
    // has_suffix tests
    // ============================================================================

    #[test]
    fn test_has_suffix_basic() {
        assert!(has_suffix("hello world", "world"));
        assert!(has_suffix("hello world", " world"));
        assert!(has_suffix("hello world", "hello world"));
    }

    #[test]
    fn test_has_suffix_negative() {
        assert!(!has_suffix("hello world", "hello"));
        assert!(!has_suffix("hello world", "worl"));
        assert!(!has_suffix("world", "hello world"));
    }

    #[test]
    fn test_has_suffix_empty() {
        assert!(has_suffix("hello", ""));
        assert!(has_suffix("", ""));
        assert!(!has_suffix("", "hello"));
    }

    #[test]
    fn test_has_suffix_file_extension() {
        assert!(has_suffix("main.rs", ".rs"));
        assert!(has_suffix("image.png", ".png"));
        assert!(!has_suffix("main.rs", ".txt"));
    }

    #[test]
    fn test_has_suffix_unicode() {
        assert!(has_suffix("café", "fé"));
        assert!(has_suffix("日本語", "本語"));
        assert!(!has_suffix("日本語", "日"));
    }

    // ============================================================================
    // find_first tests
    // ============================================================================

    #[test]
    fn test_find_first_basic() {
        assert_eq!(find_first("hello world", "hello"), Some(0));
        assert_eq!(find_first("hello world", "world"), Some(6));
        assert_eq!(find_first("hello world", " "), Some(5));
    }

    #[test]
    fn test_find_first_multiple_occurrences() {
        assert_eq!(find_first("hello hello", "hello"), Some(0));
        assert_eq!(find_first("abcabc", "abc"), Some(0));
        assert_eq!(find_first("abcabc", "bc"), Some(1));
    }

    #[test]
    fn test_find_first_not_found() {
        assert_eq!(find_first("hello world", "xyz"), None);
        assert_eq!(find_first("hello", "hello world"), None);
        assert_eq!(find_first("", "hello"), None);
    }

    #[test]
    fn test_find_first_empty_pattern() {
        assert_eq!(find_first("hello", ""), Some(0));
        assert_eq!(find_first("", ""), Some(0));
    }

    #[test]
    fn test_find_first_unicode() {
        assert_eq!(find_first("café", "é"), Some(3)); // byte index
        assert_eq!(find_first("日本語", "本"), Some(3)); // byte index
    }

    #[test]
    fn test_find_first_single_char() {
        assert_eq!(find_first("hello", "l"), Some(2));
        assert_eq!(find_first("hello", "o"), Some(4));
    }

    // ============================================================================
    // find_last tests
    // ============================================================================

    #[test]
    fn test_find_last_basic() {
        assert_eq!(find_last("hello world", "o"), Some(7));
        assert_eq!(find_last("hello world", "l"), Some(9));
    }

    #[test]
    fn test_find_last_multiple_occurrences() {
        assert_eq!(find_last("hello hello", "hello"), Some(6));
        assert_eq!(find_last("abcabc", "abc"), Some(3));
        assert_eq!(find_last("abcabc", "bc"), Some(4));
    }

    #[test]
    fn test_find_last_single_occurrence() {
        assert_eq!(find_last("hello world", "world"), Some(6));
        assert_eq!(find_last("unique", "que"), Some(3));
    }

    #[test]
    fn test_find_last_not_found() {
        assert_eq!(find_last("hello world", "xyz"), None);
        assert_eq!(find_last("", "hello"), None);
    }

    #[test]
    fn test_find_last_empty_pattern() {
        assert_eq!(find_last("hello", ""), Some(5)); // at end
        assert_eq!(find_last("", ""), Some(0));
    }

    #[test]
    fn test_find_last_unicode() {
        assert_eq!(find_last("caféé", "é"), Some(5)); // last é
        assert_eq!(find_last("日本語日本", "本"), Some(12)); // byte index (each char is 3 bytes)
    }

    // ============================================================================
    // count_occurrences tests
    // ============================================================================

    #[test]
    fn test_count_occurrences_basic() {
        assert_eq!(count_occurrences("hello world", "o"), 2);
        assert_eq!(count_occurrences("hello world", "l"), 3);
        assert_eq!(count_occurrences("hello world", "hello"), 1);
    }

    #[test]
    fn test_count_occurrences_multiple() {
        assert_eq!(count_occurrences("abababab", "ab"), 4);
        assert_eq!(count_occurrences("aaaa", "a"), 4);
        assert_eq!(count_occurrences("aaa", "aa"), 1); // non-overlapping
    }

    #[test]
    fn test_count_occurrences_none() {
        assert_eq!(count_occurrences("hello world", "xyz"), 0);
        assert_eq!(count_occurrences("", "hello"), 0);
    }

    #[test]
    fn test_count_occurrences_empty_pattern() {
        // Empty pattern matches at every position including start and end
        assert_eq!(count_occurrences("hi", ""), 3); // before h, between h and i, after i
        assert_eq!(count_occurrences("", ""), 1);
    }

    #[test]
    fn test_count_occurrences_unicode() {
        assert_eq!(count_occurrences("café café café", "café"), 3);
        assert_eq!(count_occurrences("日本語日本語", "日本"), 2);
    }

    #[test]
    fn test_count_occurrences_whole_string() {
        assert_eq!(count_occurrences("hello", "hello"), 1);
        assert_eq!(count_occurrences("", ""), 1);
    }

    // ============================================================================
    // find_all_indices tests
    // ============================================================================

    #[test]
    fn test_find_all_indices_basic() {
        assert_eq!(find_all_indices("abcabc", "abc"), vec![0, 3]);
        assert_eq!(find_all_indices("hello", "l"), vec![2, 3]);
    }

    #[test]
    fn test_find_all_indices_multiple() {
        assert_eq!(find_all_indices("aaaa", "a"), vec![0, 1, 2, 3]);
        assert_eq!(find_all_indices("abababab", "ab"), vec![0, 2, 4, 6]);
    }

    #[test]
    fn test_find_all_indices_none() {
        assert_eq!(find_all_indices("hello", "xyz"), vec![]);
        assert_eq!(find_all_indices("", "hello"), vec![]);
    }

    #[test]
    fn test_find_all_indices_single_match() {
        assert_eq!(find_all_indices("hello world", "world"), vec![6]);
        assert_eq!(find_all_indices("unique", "que"), vec![3]);
    }

    #[test]
    fn test_find_all_indices_unicode() {
        assert_eq!(find_all_indices("café café", "café"), vec![0, 6]);
        assert_eq!(find_all_indices("日本語", "本"), vec![3]); // byte index
    }

    #[test]
    fn test_find_all_indices_overlapping_potential() {
        // .match_indices returns non-overlapping matches
        assert_eq!(find_all_indices("aaa", "aa"), vec![0]); // only first match
    }

    // ============================================================================
    // extract_between tests
    // ============================================================================

    #[test]
    fn test_extract_between_basic() {
        assert_eq!(
            extract_between("<tag>content</tag>", "<tag>", "</tag>"),
            Some("content".to_string())
        );
        assert_eq!(
            extract_between("hello [world] test", "[", "]"),
            Some("world".to_string())
        );
    }

    #[test]
    fn test_extract_between_html() {
        assert_eq!(
            extract_between("<title>My Page</title>", "<title>", "</title>"),
            Some("My Page".to_string())
        );
        assert_eq!(
            extract_between("<div class='foo'>bar</div>", ">", "</"),
            Some("bar".to_string())
        );
    }

    #[test]
    fn test_extract_between_not_found() {
        assert_eq!(extract_between("no markers here", "[", "]"), None);
        assert_eq!(extract_between("only start [", "[", "]"), None);
        assert_eq!(extract_between("only end ]", "[", "]"), None);
    }

    #[test]
    fn test_extract_between_empty_content() {
        assert_eq!(extract_between("[]", "[", "]"), Some("".to_string()));
        assert_eq!(extract_between("<></>", "<>", "</>"), Some("".to_string()));
    }

    #[test]
    fn test_extract_between_multiple_markers() {
        // Should return first match
        assert_eq!(
            extract_between("[first][second]", "[", "]"),
            Some("first".to_string())
        );
    }

    #[test]
    fn test_extract_between_nested() {
        // Inner content may contain start marker (greedy behavior)
        assert_eq!(
            extract_between("[[nested]]", "[", "]"),
            Some("[nested".to_string())
        );
    }

    #[test]
    fn test_extract_between_same_marker() {
        assert_eq!(
            extract_between("|content|rest", "|", "|"),
            Some("content".to_string())
        );
    }

    #[test]
    fn test_extract_between_unicode() {
        assert_eq!(
            extract_between("「日本語」", "「", "」"),
            Some("日本語".to_string())
        );
        assert_eq!(
            extract_between("<<café>>", "<<", ">>"),
            Some("café".to_string())
        );
    }

    #[test]
    fn test_extract_between_multichar_markers() {
        assert_eq!(
            extract_between("START>>>content<<<END", "START>>>", "<<<END"),
            Some("content".to_string())
        );
        assert_eq!(
            extract_between("BEGIN text FINISH", "BEGIN ", " FINISH"),
            Some("text".to_string())
        );
    }
}
