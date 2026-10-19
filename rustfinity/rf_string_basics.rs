//! # Rustfinity: String Basics
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/string-basics
//!
//! In Rust, there are two main string types: `String` and `&str`. Understanding the difference between them is fundamental to working with text in Rust.
//!
//! - **`String`** is an owned, heap-allocated, growable string type. You have full control over it and can modify its contents.
//! - **`&str`** (string slice) is a borrowed reference to a string. It's typically used for string literals and when you want to view string data without owning it.
//!
//! ## Converting Between String Types
//!
//! You can create a `String` from a `&str` in several ways:
//!
//! ```rust
//! // Using .to_string()
//! let s1 = "hello".to_string();
//! // Using String::from()
//! let s2 = String::from("hello");
//! ```
//!
//! ## Iterating Over Strings
//!
//! Rust strings are UTF-8 encoded, which means characters can be 1-4 bytes. You can iterate over:
//!
//! - **Characters** using `.chars()` - gives you each Unicode scalar value
//! - **Bytes** using `.bytes()` - gives you each raw byte
//!
//! ```rust
//! let s = "hello";
//! for c in s.chars() {
//!     println!("{}", c); // prints: h, e, l, l, o
//! }
//! for b in s.bytes() {
//!     println!("{}", b); // prints: 104, 101, 108, 108, 111
//! }
//! ```
//!
//! ## Checking ASCII
//!
//! You can check if a string contains only ASCII characters using `.is_ascii()`:
//!
//! ```rust
//! let ascii_str = "Hello";
//! let unicode_str = "Hello, world!";
//!
//! assert!(ascii_str.is_ascii());     // true
//! // true - common punctuation is ASCII
//! assert!(unicode_str.is_ascii());
//! ```
//!
//! ## Your Task
//!
//! Implement the following functions:
//!
//! 1. `to_owned_string(s: &str) -> String` - Convert a string slice to an owned `String`
//! 2. `count_chars(s: &str) -> usize` - Count the number of characters in a string
//! 3. `count_bytes(s: &str) -> usize` - Count the number of bytes in a string
//! 4. `is_ascii_only(s: &str) -> bool` - Check if a string contains only ASCII characters
//! 5. `first_char(s: &str) -> Option<char>` - Return the first character of a string, or `None` if empty
//!
//! ## Examples
//!
//! ```rust
//! assert_eq!(
//!     to_owned_string("hello"),
//!     String::from("hello")
//! );
//! assert_eq!(count_chars("hello"), 5);
//! // 5 Unicode characters (cafe + combining accent)
//! assert_eq!(count_chars("cafe\u{0301}"), 5);
//! assert_eq!(count_bytes("hello"), 5);
//! // 5 bytes: 'c', 'a', 'f' = 1 byte each
//! // 'e with accent' = 2 bytes
//! assert_eq!(count_bytes("caf\u{00E9}"), 5);
//! assert_eq!(is_ascii_only("hello"), true);
//! // accented 'e' is not ASCII
//! assert_eq!(is_ascii_only("caf\u{00E9}"), false);
//! assert_eq!(first_char("hello"), Some('h'));
//! assert_eq!(first_char(""), None);
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - Use `.to_string()` or `String::from()` to convert `&str` to `String`
//! - Use `.chars().count()` to count Unicode characters
//! - Use `.bytes().count()` or `.len()` to count bytes
//! - Use `.is_ascii()` method on `&str` to check ASCII
//! - Use `.chars().next()` to get the first character as an `Option<char>`
//!
//! </details>
//!
//! Run: `cargo run --example rf_string_basics`
//! Test: `cargo test --example rf_string_basics`
//! Filter: `cargo test --example rf_string_basics <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

/// Convert a string slice to an owned String.
pub fn to_owned_string(s: &str) -> String {
    // TODO: Convert the string slice to an owned String
    unimplemented!()
}

/// Count the number of Unicode characters in a string.
pub fn count_chars(s: &str) -> usize {
    // TODO: Count the number of characters
    unimplemented!()
}

/// Count the number of bytes in a string.
pub fn count_bytes(s: &str) -> usize {
    // TODO: Count the number of bytes
    unimplemented!()
}

/// Check if a string contains only ASCII characters.
pub fn is_ascii_only(s: &str) -> bool {
    // TODO: Check if the string contains only ASCII characters
    unimplemented!()
}

/// Return the first character of a string, or None if the string is empty.
pub fn first_char(s: &str) -> Option<char> {
    // TODO: Return the first character or None if empty
    unimplemented!()
}

pub fn main() {
    let greeting = "Hello, world!";

    println!("Original: {}", greeting);
    println!("As owned String: {}", to_owned_string(greeting));
    println!("Character count: {}", count_chars(greeting));
    println!("Byte count: {}", count_bytes(greeting));
    println!("Is ASCII only: {}", is_ascii_only(greeting));
    println!("First character: {:?}", first_char(greeting));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_to_owned_string_simple() {
        assert_eq!(to_owned_string("hello"), String::from("hello"));
    }

    #[test]
    fn test_to_owned_string_empty() {
        assert_eq!(to_owned_string(""), String::new());
    }

    #[test]
    fn test_to_owned_string_unicode() {
        assert_eq!(to_owned_string("caf\u{00E9}"), String::from("caf\u{00E9}"));
    }

    #[test]
    fn test_count_chars_ascii() {
        assert_eq!(count_chars("hello"), 5);
    }

    #[test]
    fn test_count_chars_empty() {
        assert_eq!(count_chars(""), 0);
    }

    #[test]
    fn test_count_chars_unicode() {
        // "\u{1F600}" is a smiley face emoji - 1 Unicode character
        assert_eq!(count_chars("h\u{1F600}i\u{1F600}"), 4);
    }

    #[test]
    fn test_count_chars_accented() {
        // "caf\u{00E9}" has 4 characters (e with accent is one character)
        assert_eq!(count_chars("caf\u{00E9}"), 4);
    }

    #[test]
    fn test_count_bytes_ascii() {
        assert_eq!(count_bytes("hello"), 5);
    }

    #[test]
    fn test_count_bytes_empty() {
        assert_eq!(count_bytes(""), 0);
    }

    #[test]
    fn test_count_bytes_unicode() {
        // Each emoji takes 4 bytes in UTF-8
        assert_eq!(count_bytes("h\u{1F600}i\u{1F600}"), 10); // 'h' (1) + emoji (4) + 'i' (1) + emoji (4)
    }

    #[test]
    fn test_count_bytes_accented() {
        // "caf\u{00E9}" - 'c', 'a', 'f' are 1 byte each, '\u{00E9}' (e with accent) is 2 bytes
        assert_eq!(count_bytes("caf\u{00E9}"), 5);
    }

    #[test]
    fn test_is_ascii_only_true() {
        assert!(is_ascii_only("hello world"));
        assert!(is_ascii_only("Hello, World! 123"));
        assert!(is_ascii_only(""));
    }

    #[test]
    fn test_is_ascii_only_false() {
        assert!(!is_ascii_only("caf\u{00E9}"));
        assert!(!is_ascii_only("hello \u{1F600}"));
        assert!(!is_ascii_only("\u{4E2D}\u{6587}")); // Chinese characters
    }

    #[test]
    fn test_first_char_ascii() {
        assert_eq!(first_char("hello"), Some('h'));
        assert_eq!(first_char("world"), Some('w'));
    }

    #[test]
    fn test_first_char_empty() {
        assert_eq!(first_char(""), None);
    }

    #[test]
    fn test_first_char_unicode() {
        assert_eq!(first_char("caf\u{00E9}"), Some('c'));
        assert_eq!(first_char("\u{1F600}world"), Some('\u{1F600}'));
    }

    #[test]
    fn test_first_char_single() {
        assert_eq!(first_char("a"), Some('a'));
        assert_eq!(first_char("\u{1F600}"), Some('\u{1F600}'));
    }
}
