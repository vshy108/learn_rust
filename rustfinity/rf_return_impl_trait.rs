//! # Rustfinity: Return Impl Trait
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/return-impl-trait
//!
//! Rust's `impl Trait` syntax allows for concise and flexible code, especially when returning iterators or other complex types. By using `impl Trait`, you can abstract away concrete types while ensuring optimal performance through Rust's zero-cost abstractions.
//!
//! In this challenge, you will implement a function that filters strings from a slice, returning only those that start with a given keyword. The function will return an iterator over the filtered results. This approach demonstrates how to combine Rust's iterator combinators with the `impl Trait` syntax.
//!
//! ## Your Task
//!
//! Write a function named `filter_starts_with` that:
//!
//! 1. Accepts two arguments:
//!    - A slice of `String`
//!    - A `&str` keyword
//! 2. Returns an iterator that filters and yields only the strings starting with the given keyword.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - Use `.iter()` to iterate over references to the strings in the slice.
//! - The `filter` method takes a closure to apply a filtering condition.
//! - Use the `starts_with` method to check if a string starts with a keyword.
//! - Use `move` in the closure to capture the keyword.
//!
//! </details>
//!
//! Run: `cargo run --example rf_return_impl_trait`
//! Test: `cargo test --example rf_return_impl_trait`
//! Filter: `cargo test --example rf_return_impl_trait <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub fn filter_starts_with<'a>(
    items: &'a [String],
    prefix: &'a str,
) -> impl Iterator<Item = &'a String> + 'a {
    let _ = (items, prefix);
    std::iter::empty()
}

// Example usage
pub fn main() {
    let input = vec![
        String::from("apple"),
        String::from("apricot"),
        String::from("banana"),
        String::from("cherry"),
    ];
    let filtered: Vec<&String> = filter_starts_with(&input, "ap").collect();
    println!("{:?}", filtered); // Expected output: ["apple", "apricot"]
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_filter_starts_with() {
        let input = vec![
            String::from("apple"),
            String::from("apricot"),
            String::from("banana"),
        ];
        let result: Vec<&String> = filter_starts_with(&input, "ap").collect();
        assert_eq!(
            result,
            vec![&String::from("apple"), &String::from("apricot")]
        );
    }

    #[test]
    fn test_filter_starts_with_empty_slice() {
        let input: Vec<String> = vec![];
        let result: Vec<&String> = filter_starts_with(&input, "ap").collect();
        assert!(result.is_empty());
    }

    #[test]
    fn test_filter_starts_with_no_matches() {
        let input = vec![
            String::from("banana"),
            String::from("cherry"),
            String::from("date"),
        ];
        let result: Vec<&String> = filter_starts_with(&input, "ap").collect();
        assert!(result.is_empty());
    }

    #[test]
    fn test_filter_starts_with_partial_matches() {
        let input = vec![
            String::from("apple"),
            String::from("banana"),
            String::from("apricot"),
            String::from("cherry"),
        ];
        let result: Vec<&String> = filter_starts_with(&input, "ap").collect();
        assert_eq!(
            result,
            vec![&String::from("apple"), &String::from("apricot")]
        );
    }

    #[test]
    fn test_filter_starts_with_keyword_empty() {
        let input = vec![
            String::from("apple"),
            String::from("banana"),
            String::from("cherry"),
        ];
        let result: Vec<&String> = filter_starts_with(&input, "").collect();
        assert_eq!(
            result,
            vec![
                &String::from("apple"),
                &String::from("banana"),
                &String::from("cherry")
            ]
        );
    }

    #[test]
    fn test_filter_starts_with_case_sensitivity() {
        let input = vec![
            String::from("Apple"),
            String::from("apricot"),
            String::from("banana"),
        ];
        let result: Vec<&String> = filter_starts_with(&input, "Ap").collect();
        assert_eq!(result, vec![&String::from("Apple")]);
    }
}
