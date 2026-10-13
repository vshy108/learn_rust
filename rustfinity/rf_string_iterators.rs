//! # Rustfinity: String Iterators
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/string-iterators
//!
//! Rust strings provide several powerful iterator methods that let you process text character by character, word by word, or line by line. Understanding these iteration patterns is fundamental to text processing in Rust.
//!
//! ## String Iterators Overview
//!
//! Rust's `str` type provides several methods that return iterators:
//!
//! 1. **`.chars()`** - Iterates over Unicode scalar values (characters)
//! 2. **`.split_whitespace()`** - Splits on Unicode whitespace, skipping empty segments
//! 3. **`.lines()`** - Splits on line endings (`\n` or `\r\n`)
//! 4. **`.split(pattern)`** - Splits on a custom delimiter
//!
//! These iterators are lazy - they don't do any work until you consume them with methods like `.collect()`, `.count()`, or a `for` loop.
//!
//! ```rust
//! let text = "hello world";
//!
//! // Iterate over characters
//! for ch in text.chars() {
//!     println!("{}", ch);
//! }
//!
//! // Iterate over words
//! for word in text.split_whitespace() {
//!     println!("{}", word);
//! }
//!
//! // Collect into a Vec
//! let chars: Vec<char> = text.chars().collect();
//! let words: Vec<&str> = text.split_whitespace().collect();
//! ```
//!
//! ## Transforming While Iterating
//!
//! You can chain iterator methods to transform data as you iterate:
//!
//! ```rust
//! let text = "  hello  world  ";
//!
//! // Get uppercase words
//! let upper_words: Vec<String> = text
//!     .split_whitespace()
//!     .map(|w| w.to_uppercase())
//!     .collect();
//! assert_eq!(upper_words, vec!["HELLO", "WORLD"]);
//! ```
//!
//! ## Your Task
//!
//! Implement the following functions that demonstrate string iteration patterns:
//!
//! 1. `chars_to_vec(s: &str) -> Vec<char>` - Convert a string into a vector of its characters
//! 2. `words_to_vec(s: &str) -> Vec<String>` - Split a string on whitespace and collect into owned strings
//! 3. `lines_to_vec(s: &str) -> Vec<String>` - Split a string on line endings and collect into owned strings
//! 4. `count_words(s: &str) -> usize` - Count the number of whitespace-separated words
//! 5. `reverse_words(s: &str) -> String` - Reverse the order of words in a string (keep words themselves intact)
//! 6. `capitalize_words(s: &str) -> String` - Capitalize the first letter of each word
//!
//! ## Examples
//!
//! ```rust
//! // chars_to_vec
//! assert_eq!(chars_to_vec("hi"), vec!['h', 'i']);
//! assert_eq!(chars_to_vec(""), Vec::<char>::new());
//!
//! // words_to_vec
//! assert_eq!(
//!     words_to_vec("hello world"),
//!     vec!["hello", "world"]
//! );
//! assert_eq!(
//!     words_to_vec("  spaces  everywhere  "),
//!     vec!["spaces", "everywhere"]
//! );
//!
//! // lines_to_vec
//! assert_eq!(
//!     lines_to_vec("line1\nline2\nline3"),
//!     vec!["line1", "line2", "line3"]
//! );
//! assert_eq!(lines_to_vec("single"), vec!["single"]);
//!
//! // count_words
//! assert_eq!(count_words("one two three"), 3);
//! assert_eq!(count_words(""), 0);
//! assert_eq!(count_words("   "), 0);
//!
//! // reverse_words
//! assert_eq!(
//!     reverse_words("hello world"),
//!     "world hello"
//! );
//! assert_eq!(
//!     reverse_words("one two three"),
//!     "three two one"
//! );
//!
//! // capitalize_words
//! assert_eq!(
//!     capitalize_words("hello world"),
//!     "Hello World"
//! );
//! assert_eq!(
//!     capitalize_words("rust is great"),
//!     "Rust Is Great"
//! );
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - For `chars_to_vec`, use `.chars().collect()`
//! - For `words_to_vec`, use `.split_whitespace()` and `.map(|s| s.to_string())` before `.collect()`
//! - For `lines_to_vec`, use `.lines()` and convert each `&str` to `String`
//! - For `count_words`, use `.split_whitespace().count()`
//! - For `reverse_words`, collect words into a Vec, reverse it, then join with spaces
//! - For `capitalize_words`, for each word, make the first character uppercase and the rest lowercase, then join with spaces
//!
//! </details>
//!
//! Run: `cargo run --example rf_string_iterators`
//! Test: `cargo test --example rf_string_iterators`
//! Filter: `cargo test --example rf_string_iterators <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

/// Convert a string into a vector of its characters.
pub fn chars_to_vec(s: &str) -> Vec<char> {
    // TODO: Use .chars() and .collect()
    unimplemented!()
}

/// Split a string on whitespace and collect into owned strings.
pub fn words_to_vec(s: &str) -> Vec<String> {
    // TODO: Use .split_whitespace() and collect
    unimplemented!()
}

/// Split a string on line endings and collect into owned strings.
pub fn lines_to_vec(s: &str) -> Vec<String> {
    // TODO: Use .lines() and collect
    unimplemented!()
}

/// Count the number of whitespace-separated words.
pub fn count_words(s: &str) -> usize {
    // TODO: Use .split_whitespace() and .count()
    unimplemented!()
}

/// Reverse the order of words in a string (keep words themselves intact).
pub fn reverse_words(s: &str) -> String {
    // TODO: Split into words, reverse the order, then join with spaces
    unimplemented!()
}

/// Capitalize the first letter of each word.
pub fn capitalize_words(s: &str) -> String {
    // TODO: Make the first character uppercase and rest lowercase
    unimplemented!()
}

pub fn main() {
    let text = "hello world";
    println!("Original: '{}'", text);
    println!("chars_to_vec: {:?}", chars_to_vec(text));
    println!("words_to_vec: {:?}", words_to_vec(text));
    println!("count_words: {}", count_words(text));
    println!("reverse_words: '{}'", reverse_words(text));
    println!("capitalize_words: '{}'", capitalize_words(text));

    let multiline = "line one\nline two\nline three";
    println!("\nMultiline text:");
    println!("{}", multiline);
    println!("lines_to_vec: {:?}", lines_to_vec(multiline));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    // ============== chars_to_vec tests ==============

    #[test]
    fn chars_to_vec_simple() {
        assert_eq!(chars_to_vec("hi"), vec!['h', 'i']);
    }

    #[test]
    fn chars_to_vec_empty() {
        assert_eq!(chars_to_vec(""), Vec::<char>::new());
    }

    #[test]
    fn chars_to_vec_with_spaces() {
        assert_eq!(chars_to_vec("a b"), vec!['a', ' ', 'b']);
    }

    #[test]
    fn chars_to_vec_unicode() {
        assert_eq!(chars_to_vec("日本"), vec!['日', '本']);
    }

    #[test]
    fn chars_to_vec_emoji() {
        assert_eq!(chars_to_vec("Hi!"), vec!['H', 'i', '!']);
    }

    // ============== words_to_vec tests ==============

    #[test]
    fn words_to_vec_simple() {
        assert_eq!(words_to_vec("hello world"), vec!["hello", "world"]);
    }

    #[test]
    fn words_to_vec_extra_spaces() {
        assert_eq!(
            words_to_vec("  spaces  everywhere  "),
            vec!["spaces", "everywhere"]
        );
    }

    #[test]
    fn words_to_vec_single_word() {
        assert_eq!(words_to_vec("word"), vec!["word"]);
    }

    #[test]
    fn words_to_vec_empty() {
        assert_eq!(words_to_vec(""), Vec::<String>::new());
    }

    #[test]
    fn words_to_vec_only_spaces() {
        assert_eq!(words_to_vec("   "), Vec::<String>::new());
    }

    #[test]
    fn words_to_vec_tabs_and_newlines() {
        assert_eq!(words_to_vec("a\tb\nc"), vec!["a", "b", "c"]);
    }

    // ============== lines_to_vec tests ==============

    #[test]
    fn lines_to_vec_multiple() {
        assert_eq!(
            lines_to_vec("line1\nline2\nline3"),
            vec!["line1", "line2", "line3"]
        );
    }

    #[test]
    fn lines_to_vec_single() {
        assert_eq!(lines_to_vec("single"), vec!["single"]);
    }

    #[test]
    fn lines_to_vec_empty() {
        assert_eq!(lines_to_vec(""), Vec::<String>::new());
    }

    #[test]
    fn lines_to_vec_windows_line_endings() {
        assert_eq!(lines_to_vec("line1\r\nline2"), vec!["line1", "line2"]);
    }

    #[test]
    fn lines_to_vec_empty_lines() {
        assert_eq!(lines_to_vec("a\n\nb"), vec!["a", "", "b"]);
    }

    #[test]
    fn lines_to_vec_trailing_newline() {
        // Note: .lines() does not include empty trailing line
        assert_eq!(lines_to_vec("a\nb\n"), vec!["a", "b"]);
    }

    // ============== count_words tests ==============

    #[test]
    fn count_words_three() {
        assert_eq!(count_words("one two three"), 3);
    }

    #[test]
    fn count_words_empty() {
        assert_eq!(count_words(""), 0);
    }

    #[test]
    fn count_words_only_whitespace() {
        assert_eq!(count_words("   "), 0);
    }

    #[test]
    fn count_words_single() {
        assert_eq!(count_words("word"), 1);
    }

    #[test]
    fn count_words_with_tabs() {
        assert_eq!(count_words("a\tb\tc"), 3);
    }

    #[test]
    fn count_words_mixed_whitespace() {
        assert_eq!(count_words("  hello \t world \n test  "), 3);
    }

    // ============== reverse_words tests ==============

    #[test]
    fn reverse_words_two() {
        assert_eq!(reverse_words("hello world"), "world hello");
    }

    #[test]
    fn reverse_words_three() {
        assert_eq!(reverse_words("one two three"), "three two one");
    }

    #[test]
    fn reverse_words_single() {
        assert_eq!(reverse_words("single"), "single");
    }

    #[test]
    fn reverse_words_empty() {
        assert_eq!(reverse_words(""), "");
    }

    #[test]
    fn reverse_words_with_extra_spaces() {
        // Extra spaces are normalized
        assert_eq!(reverse_words("  hello   world  "), "world hello");
    }

    #[test]
    fn reverse_words_sentence() {
        assert_eq!(reverse_words("I love Rust"), "Rust love I");
    }

    // ============== capitalize_words tests ==============

    #[test]
    fn capitalize_words_simple() {
        assert_eq!(capitalize_words("hello world"), "Hello World");
    }

    #[test]
    fn capitalize_words_already_caps() {
        assert_eq!(capitalize_words("HELLO WORLD"), "Hello World");
    }

    #[test]
    fn capitalize_words_mixed_case() {
        assert_eq!(capitalize_words("hElLo WoRlD"), "Hello World");
    }

    #[test]
    fn capitalize_words_single() {
        assert_eq!(capitalize_words("rust"), "Rust");
    }

    #[test]
    fn capitalize_words_empty() {
        assert_eq!(capitalize_words(""), "");
    }

    #[test]
    fn capitalize_words_with_numbers() {
        assert_eq!(capitalize_words("hello 123 world"), "Hello 123 World");
    }

    #[test]
    fn capitalize_words_extra_spaces() {
        // Extra spaces are normalized
        assert_eq!(capitalize_words("  hello   world  "), "Hello World");
    }

    #[test]
    fn capitalize_words_single_letters() {
        assert_eq!(capitalize_words("a b c"), "A B C");
    }
}
