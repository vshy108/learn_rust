//! # Rustfinity: Lifetimes
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/lifetimes
//!
//! Lifetimes in Rust ensure that references are valid for as long as they are needed. The Rust compiler uses lifetimes to prevent dangling references, ensuring memory safety at compile time without the need for a garbage collector.
//!
//! A common scenario where lifetimes come into play is when
//! working with functions that accept or return references.
//! To ensure the correct association between input references
//! and output references, lifetimes are explicitly annotated.
//!
//! In this challenge, you'll implement a function that finds the longest string slice from two provided references. The function will use lifetimes to ensure that the returned reference is valid for as long as both input references are valid.
//!
//! ## Your Task
//!
//! Write a function `longest` that takes two string slices and returns the longest one. If both slices are of the same length, return the first one.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - Use explicit lifetime annotations in the function signature, such as `'a`. e.g
//!   ```rust
//!   fn longest<'a>(
//!       first: &'a str,
//!       second: &'a str
//!   ) -> &'a str {
//!       // implementation goes here
//!   }
//!   ```
//! - Rust allows you to compare the lengths of strings using the `.chars().count()` method.
//!
//! </details>
//!
//! Run: `cargo run --example rf_lifetimes`
//! Test: `cargo test --example rf_lifetimes`
//! Filter: `cargo test --example rf_lifetimes <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    todo!("return the longer string")
}

// Example usage
pub fn main() {
    let s1 = "short";
    let s2 = "longer string";

    let result = longest(s1, s2);
    println!("The longest string is: {}", result);
    assert_eq!(result, "longer string");

    let s3 = "equal";
    let s4 = "equal";
    let result = longest(s3, s4);
    println!("The longest string is: {}", result);
    assert_eq!(result, "equal");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use crate::longest;

    #[test]
    fn test_longest_different_lengths() {
        let s1 = "short";
        let s2 = "longer";
        assert_eq!(longest(s1, s2), s2);
    }

    #[test]
    fn test_longest_equal_lengths() {
        let s1 = "same";
        let s2 = "same";
        assert_eq!(longest(s1, s2), s1);
    }

    #[test]
    fn test_longest_static_str() {
        let s1: &'static str = "static";
        let s2: &'static str = "longer static";
        assert_eq!(longest(s1, s2), s2);
    }

    #[test]
    fn test_longest_edge_case_empty() {
        let s1 = "";
        let s2 = "non-empty";
        assert_eq!(longest(s1, s2), s2);
    }

    #[test]
    fn test_longest_empty_both() {
        let s1 = "";
        let s2 = "";
        assert_eq!(longest(s1, s2), s1);
    }

    #[test]
    fn test_longest_with_substring() {
        let s1 = "substring";
        let s2 = "sub";
        assert_eq!(longest(s1, s2), s1);
    }

    #[test]
    fn test_longest_dynamically_allocated_strings() {
        let s1 = String::from("dynamic one");
        let s2 = String::from("dynamic two longer");
        assert_eq!(longest(&s1, &s2), &s2);
    }

    #[test]
    fn test_longest_mixed_static_and_dynamic() {
        let s1 = "static string";
        let s2 = String::from("dynamic and longer string");
        assert_eq!(longest(s1, &s2), &s2);
    }

    #[test]
    fn test_longest_numbers_as_strings() {
        let s1 = "12345";
        let s2 = "67890";
        assert_eq!(longest(s1, s2), s1); // They are equal in length, so s1 should be returned.
    }

    #[test]
    fn test_longest_with_whitespace_strings() {
        let s1 = "   ";
        let s2 = " ";
        assert_eq!(longest(s1, s2), s1);
    }

    #[test]
    fn test_longest_special_characters() {
        let s1 = "@@!!";
        let s2 = "@@@";
        assert_eq!(longest(s1, s2), s1);
    }

    #[test]
    fn test_longest_unicode_strings() {
        let s1 = "こんにちは"; // Japanese for "Hello"
        let s2 = "안녕하세요"; // Korean for "Hello"
        assert_eq!(longest(s1, s2), s1); // Both are the same length, return s1.
    }

    #[test]
    fn test_with_emoji() {
        let s1 = "👋🏼🏼🏼🏼";
        let s2 = "longer";

        assert_eq!(longest(s1, s2), s2);
    }
}
