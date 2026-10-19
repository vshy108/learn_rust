//! # Rustfinity: String Manipulation
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/string-manipulation
//!
//! Rust provides a rich set of methods for manipulating strings. These methods allow you to search, transform, split, and clean string data efficiently.
//!
//! ## Common String Methods
//!
//! ### Trimming Whitespace
//!
//! The `.trim()` method removes leading and trailing whitespace from a string:
//!
//! ```rust
//! let messy = "  hello world  ";
//! assert_eq!(messy.trim(), "hello world");
//! ```
//!
//! You can also trim only from one side with `.trim_start()` and `.trim_end()`.
//!
//! ### Case Conversion
//!
//! Convert strings to lowercase or uppercase:
//!
//! ```rust
//! let mixed = "Hello World";
//! assert_eq!(mixed.to_lowercase(), "hello world");
//! assert_eq!(mixed.to_uppercase(), "HELLO WORLD");
//! ```
//!
//! ### Searching
//!
//! Check if a string contains a substring:
//!
//! ```rust
//! let text = "Rust is awesome";
//! assert!(text.contains("awesome"));
//! assert!(!text.contains("boring"));
//! ```
//!
//! ### Replacing
//!
//! Replace occurrences of a pattern with another string:
//!
//! ```rust
//! let original = "hello world";
//! assert_eq!(original.replace("world", "Rust"), "hello Rust");
//! ```
//!
//! ### Splitting
//!
//! Split a string into parts based on a delimiter:
//!
//! ```rust
//! let csv = "apple,banana,cherry";
//! let fruits: Vec<&str> = csv.split(',').collect();
//! assert_eq!(fruits, vec!["apple", "banana", "cherry"]);
//! ```
//!
//! ## Your Task
//!
//! Implement the following functions:
//!
//! 1. `clean_string(s: &str) -> String` - Remove leading and trailing whitespace, then convert to lowercase
//! 2. `contains_word(text: &str, word: &str) -> bool` - Check if the text contains the given word (case-insensitive)
//! 3. `replace_word(text: &str, from: &str, to: &str) -> String` - Replace all occurrences of `from` with `to`
//! 4. `split_and_trim(s: &str, delimiter: char) -> Vec<String>` - Split the string by the delimiter and trim each part
//! 5. `normalize_whitespace(s: &str) -> String` - Replace all sequences of whitespace with a single space, and trim
//!
//! ## Examples
//!
//! ```rust
//! // clean_string: trim and lowercase
//! assert_eq!(
//!     clean_string("  Hello World  "),
//!     "hello world"
//! );
//!
//! // contains_word: case-insensitive search
//! assert_eq!(
//!     contains_word("Rust is awesome", "AWESOME"),
//!     true
//! );
//! assert_eq!(
//!     contains_word("Rust is awesome", "boring"),
//!     false
//! );
//!
//! // replace_word: simple replacement
//! assert_eq!(
//!     replace_word("hello world", "world", "Rust"),
//!     "hello Rust"
//! );
//!
//! // split_and_trim: split and clean each part
//! assert_eq!(
//!     split_and_trim("apple , banana , cherry", ','),
//!     vec!["apple", "banana", "cherry"]
//! );
//!
//! // normalize_whitespace: clean up messy whitespace
//! assert_eq!(
//!     normalize_whitespace("  hello    world  "),
//!     "hello world"
//! );
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - For `clean_string`, chain `.trim()` and `.to_lowercase()`
//! - For `contains_word`, convert both strings to the same case before checking
//! - For `replace_word`, use the `.replace()` method
//! - For `split_and_trim`, use `.split()` then `.map()` with `.trim()`, and `.collect()`
//! - For `normalize_whitespace`, use `.split_whitespace()` to automatically handle multiple spaces, then `.collect::<Vec<_>>().join(" ")`
//!
//! </details>
//!
//! Run: `cargo run --example rf_string_manipulation`
//! Test: `cargo test --example rf_string_manipulation`
//! Filter: `cargo test --example rf_string_manipulation <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

/// Remove leading and trailing whitespace, then convert to lowercase.
pub fn clean_string(s: &str) -> String {
    // TODO: Trim the string and convert to lowercase
    unimplemented!()
}

/// Check if the text contains the given word (case-insensitive).
pub fn contains_word(text: &str, word: &str) -> bool {
    // TODO: Check if text contains word, ignoring case
    unimplemented!()
}

/// Replace all occurrences of `from` with `to`.
pub fn replace_word(text: &str, from: &str, to: &str) -> String {
    // TODO: Replace all occurrences of `from` with `to`
    unimplemented!()
}

/// Split the string by the delimiter and trim each part.
pub fn split_and_trim(s: &str, delimiter: char) -> Vec<String> {
    // TODO: Split by delimiter, trim each part, and collect into Vec<String>
    unimplemented!()
}

/// Replace all sequences of whitespace with a single space, and trim.
pub fn normalize_whitespace(s: &str) -> String {
    // TODO: Normalize whitespace to single spaces and trim
    unimplemented!()
}

pub fn main() {
    let messy = "  Hello, World!  ";
    println!("Original: '{}'", messy);
    println!("Cleaned: '{}'", clean_string(messy));

    let text = "Rust is a systems programming language";
    println!("\nText: '{}'", text);
    println!("Contains 'SYSTEMS': {}", contains_word(text, "SYSTEMS"));
    println!("Contains 'Java': {}", contains_word(text, "Java"));

    let original = "hello world world";
    println!("\nOriginal: '{}'", original);
    println!(
        "Replace 'world' with 'Rust': '{}'",
        replace_word(original, "world", "Rust")
    );

    let csv = "  apple ,  banana  , cherry ";
    println!("\nCSV: '{}'", csv);
    println!("Split and trim: {:?}", split_and_trim(csv, ','));

    let spaced = "  too   many    spaces   here  ";
    println!("\nSpaced: '{}'", spaced);
    println!("Normalized: '{}'", normalize_whitespace(spaced));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    // Tests for clean_string

    #[test]
    fn test_clean_string_basic() {
        assert_eq!(clean_string("  Hello World  "), "hello world");
    }

    #[test]
    fn test_clean_string_already_clean() {
        assert_eq!(clean_string("hello"), "hello");
    }

    #[test]
    fn test_clean_string_uppercase() {
        assert_eq!(clean_string("RUST IS AWESOME"), "rust is awesome");
    }

    #[test]
    fn test_clean_string_mixed_whitespace() {
        assert_eq!(clean_string("\t  Hello \n"), "hello");
    }

    #[test]
    fn test_clean_string_empty() {
        assert_eq!(clean_string(""), "");
        assert_eq!(clean_string("   "), "");
    }

    // Tests for contains_word

    #[test]
    fn test_contains_word_exact() {
        assert!(contains_word("hello world", "world"));
    }

    #[test]
    fn test_contains_word_case_insensitive() {
        assert!(contains_word("Rust is awesome", "AWESOME"));
        assert!(contains_word("RUST IS AWESOME", "rust"));
        assert!(contains_word("rust is awesome", "Is"));
    }

    #[test]
    fn test_contains_word_not_found() {
        assert!(!contains_word("Rust is awesome", "boring"));
        assert!(!contains_word("hello", "world"));
    }

    #[test]
    fn test_contains_word_empty() {
        assert!(contains_word("hello", "")); // empty string is contained in any string
        assert!(!contains_word("", "hello"));
    }

    #[test]
    fn test_contains_word_partial() {
        assert!(contains_word("hello world", "ello"));
    }

    // Tests for replace_word

    #[test]
    fn test_replace_word_single() {
        assert_eq!(replace_word("hello world", "world", "Rust"), "hello Rust");
    }

    #[test]
    fn test_replace_word_multiple() {
        assert_eq!(
            replace_word("world world world", "world", "Rust"),
            "Rust Rust Rust"
        );
    }

    #[test]
    fn test_replace_word_not_found() {
        assert_eq!(replace_word("hello world", "foo", "bar"), "hello world");
    }

    #[test]
    fn test_replace_word_empty_replacement() {
        assert_eq!(replace_word("hello world", "world", ""), "hello ");
    }

    #[test]
    fn test_replace_word_longer_replacement() {
        assert_eq!(replace_word("hi", "hi", "hello there"), "hello there");
    }

    // Tests for split_and_trim

    #[test]
    fn test_split_and_trim_basic() {
        assert_eq!(
            split_and_trim("apple,banana,cherry", ','),
            vec!["apple", "banana", "cherry"]
        );
    }

    #[test]
    fn test_split_and_trim_with_spaces() {
        assert_eq!(
            split_and_trim("apple , banana , cherry", ','),
            vec!["apple", "banana", "cherry"]
        );
    }

    #[test]
    fn test_split_and_trim_extra_whitespace() {
        assert_eq!(
            split_and_trim("  apple  ,  banana  ,  cherry  ", ','),
            vec!["apple", "banana", "cherry"]
        );
    }

    #[test]
    fn test_split_and_trim_single_element() {
        assert_eq!(split_and_trim("apple", ','), vec!["apple"]);
    }

    #[test]
    fn test_split_and_trim_empty_parts() {
        assert_eq!(split_and_trim("a,,b", ','), vec!["a", "", "b"]);
    }

    #[test]
    fn test_split_and_trim_different_delimiter() {
        assert_eq!(
            split_and_trim("one:two:three", ':'),
            vec!["one", "two", "three"]
        );
    }

    // Tests for normalize_whitespace

    #[test]
    fn test_normalize_whitespace_multiple_spaces() {
        assert_eq!(normalize_whitespace("hello    world"), "hello world");
    }

    #[test]
    fn test_normalize_whitespace_leading_trailing() {
        assert_eq!(normalize_whitespace("  hello world  "), "hello world");
    }

    #[test]
    fn test_normalize_whitespace_mixed() {
        assert_eq!(normalize_whitespace("  hello    world  "), "hello world");
    }

    #[test]
    fn test_normalize_whitespace_tabs_newlines() {
        assert_eq!(
            normalize_whitespace("hello\t\tworld\n\nfoo"),
            "hello world foo"
        );
    }

    #[test]
    fn test_normalize_whitespace_already_normal() {
        assert_eq!(normalize_whitespace("hello world"), "hello world");
    }

    #[test]
    fn test_normalize_whitespace_empty() {
        assert_eq!(normalize_whitespace(""), "");
        assert_eq!(normalize_whitespace("   "), "");
    }

    #[test]
    fn test_normalize_whitespace_single_word() {
        assert_eq!(normalize_whitespace("  hello  "), "hello");
    }
}
