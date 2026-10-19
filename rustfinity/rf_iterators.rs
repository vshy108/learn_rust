//! # Rustfinity: Iterators
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/iterators
//!
//! Iterators are a powerful abstraction in Rust, enabling developers to process sequences of data in a functional, efficient, and composable way. They are the foundation of many collection-related operations and provide a clean and expressive alternative to traditional loops.
//!
//! In this challenge, you will implement two functions that utilize iterators to perform specific operations:
//!
//! 1. Filtering out even numbers from a sequence of integers.
//! 2. Converting strings to uppercase from a sequence of string slices.
//!
//! Using Rust's iterator methods such as `filter`, `map`, and `collect`, you'll create concise, functional implementations that demonstrate the elegance and power of iterators.
//!
//! ## Your Task
//!
//! Write the following two functions:
//!
//! 1. **`filter_even_numbers`**:
//!    Takes an iterator of integers and filters out even numbers, returning a vector of the remaining odd numbers.
//!
//! 2. **`uppercase_strings`**:
//!    Takes an iterator of string slices and converts each string to uppercase, returning a vector of the transformed strings.
//!
//! Read the `main()` function carefully to understand how the functions will be used and what is expected from them.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - Use the `filter` method for filtering elements based on a condition.
//! - Make sure you dereference the values when using them in the closure. For example:
//!
//!   ```rust
//!   .filter(|&x| x % 2 != 0)
//!   ```
//!
//!   Or
//!
//!   ```rust
//!   .filter(|x| *x % 2 != 0)
//!   ```
//!
//! - Use the `collect` method to gather results from an iterator into a vector.
//! - Use the `map` method for transforming elements in an iterator.
//! - Use the `to_uppercase` method to convert a string to uppercase.
//! - Define lifetimes for the input and output types in the function signature.
//!
//! </details>
//!
//! Run: `cargo run --example rf_iterators`
//! Test: `cargo test --example rf_iterators`
//! Filter: `cargo test --example rf_iterators <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub fn filter_even_numbers(iter: impl Iterator<Item = i32>) -> Vec<i32> {
    todo!("filter odd values from the input iterator")
}

pub fn uppercase_strings<I, S>(items: I) -> Vec<String>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    todo!("uppercase each string from the iterator")
}

// 2. Finish the function here
// Signature scaffold above

// Example usage
pub fn main() {
    // Filtering even numbers
    let numbers = vec![1, 2, 3, 4, 5];
    let odd_numbers = filter_even_numbers(numbers.into_iter());
    println!("{:?}", odd_numbers); // Should print: [1, 3, 5]

    // Converting strings to uppercase
    let words = vec!["hello", "world"];
    let uppercase_words = uppercase_strings(words.into_iter());
    println!("{:?}", uppercase_words); // Should print: ["HELLO", "WORLD"]
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_filter_even_numbers() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7];
        let result = filter_even_numbers(numbers.into_iter());
        assert_eq!(result, vec![1, 3, 5, 7]);
    }

    #[test]
    fn test_filter_even_numbers_all_odd() {
        let numbers = vec![1, 3, 5, 7, 9];
        let result = filter_even_numbers(numbers.into_iter());
        assert_eq!(result, vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_filter_even_numbers_all_even() {
        let numbers = vec![2, 4, 6, 8, 10];
        let result = filter_even_numbers(numbers.into_iter());
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_filter_even_numbers_with_negatives() {
        let numbers = vec![-3, -2, -1, 0, 1, 2];
        let result = filter_even_numbers(numbers.into_iter());
        assert_eq!(result, vec![-3, -1, 1]);
    }

    #[test]
    fn test_filter_even_numbers_large_input() {
        let numbers: Vec<i32> = (1..=1000).collect();
        let result = filter_even_numbers(numbers.into_iter());
        assert_eq!(
            result,
            (1..=1000).filter(|x| x % 2 != 0).collect::<Vec<i32>>()
        );
    }

    #[test]
    fn test_uppercase_strings() {
        let words = vec!["rust", "iterator", "challenge"];
        let result = uppercase_strings(words.iter().map(|&s| s));
        assert_eq!(result, vec!["RUST", "ITERATOR", "CHALLENGE"]);
    }

    #[test]
    fn test_uppercase_strings_mixed_case() {
        let words = vec!["RuSt", "IterATOR", "ChAllEngE"];
        let result = uppercase_strings(words.iter().map(|&s| s));
        assert_eq!(result, vec!["RUST", "ITERATOR", "CHALLENGE"]);
    }

    #[test]
    fn test_uppercase_strings_special_characters() {
        let words = vec!["hello!", "rust@", "123"];
        let result = uppercase_strings(words.iter().map(|&s| s));
        assert_eq!(result, vec!["HELLO!", "RUST@", "123"]);
    }

    #[test]
    fn test_uppercase_strings_unicode() {
        let words = vec!["rust", "こんにちは", "добрый"];
        let result = uppercase_strings(words.iter().map(|&s| s));
        assert_eq!(result, vec!["RUST", "こんにちは", "ДОБРЫЙ"]);
    }

    #[test]
    fn test_uppercase_strings_with_empty_string() {
        let words = vec!["hello", ""];
        let result = uppercase_strings(words.iter().map(|&s| s));
        assert_eq!(result, vec!["HELLO", ""]);
    }

    #[test]
    fn test_empty_input() {
        let numbers: Vec<i32> = vec![];
        assert_eq!(filter_even_numbers(numbers.into_iter()), vec![]);

        let words: Vec<&str> = vec![];
        assert_eq!(uppercase_strings(words.into_iter()), Vec::<&str>::new());
    }

    #[test]
    fn test_edge_cases() {
        let single_number = vec![2];
        assert_eq!(filter_even_numbers(single_number.into_iter()), vec![]);

        let single_word = vec!["single"];
        assert_eq!(
            uppercase_strings(single_word.into_iter()),
            vec!["SINGLE".to_string()]
        );
    }
}
