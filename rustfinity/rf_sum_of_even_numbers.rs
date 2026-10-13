//! # Rustfinity: Sum Of Even Numbers
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/sum-of-even-numbers
//!
//! In this challenge, you will practice using the `for` loop in Rust to iterate over a range of numbers. The goal is to sum all the even numbers within a given range.
//!
//! The `for` loop in Rust is a powerful construct that allows you to iterate over collections, ranges, and iterators. It is commonly used for tasks such as summing numbers, collecting items, and more. By utilizing the `for` loop, you can write concise and readable code to perform repetitive tasks.
//!
//! ## Your task
//!
//! Your task is to implement a function `sum_of_evens` that takes two arguments, `start` and `end`, both of which are `i32`. The function should return the sum of all even numbers within the range `[start, end]` inclusive. If `start` is greater than `end`, the function should return `0`.
//!
//! ## Requirements
//!
//! - Use a `for` loop to iterate over the range from `start` to `end`.
//! - Sum only the even numbers within the specified range.
//! - If `start` is greater than `end`, return `0`.
//!
//! ## Example
//!
//! ```rust
//! let result = sum_of_evens(1, 10);
//! assert_eq!(result, 30); // 2 + 4 + 6 + 8 + 10 = 30
//!
//! let result = sum_of_evens(5, 5);
//! assert_eq!(result, 0); // 5 is not even
//!
//! let result = sum_of_evens(10, 1);
//! assert_eq!(result, 0); // start is greater than end
//! ```
//!
//! Run: `cargo run --example rf_sum_of_even_numbers`
//! Test: `cargo test --example rf_sum_of_even_numbers`
//! Filter: `cargo test --example rf_sum_of_even_numbers <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

pub fn sum_of_evens(start: i32, end: i32) -> i32 {
    // Your code here...
    unimplemented!()
}

fn main() {
    println!("Implement the Sum Of Even Numbers challenge.");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_sum_of_evens_range() {
        assert_eq!(sum_of_evens(1, 10), 30);
    }

    #[test]
    fn test_sum_of_evens_single_value() {
        assert_eq!(sum_of_evens(5, 5), 0);
    }

    #[test]
    fn test_sum_of_evens_reverse_range() {
        assert_eq!(sum_of_evens(10, 1), 0);
    }

    #[test]
    fn test_sum_of_evens_all_even() {
        assert_eq!(sum_of_evens(2, 6), 12); // 2 + 4 + 6 = 12
    }

    #[test]
    fn test_sum_of_evens_negative_range() {
        assert_eq!(sum_of_evens(-5, 5), 0); // -4 + -2 + 0 + 2 + 4 = 0
    }
}
