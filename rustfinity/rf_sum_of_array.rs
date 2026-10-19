//! # Rustfinity: Sum Of Array
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/sum-of-array
//!
//! Arrays are a fundamental data structure in Rust that allow you to store a **fixed-size** collection of elements of the same type. A common operation is to calculate the sum of all elements in an array.
//!
//! In this challenge, you will implement a function to calculate the **sum of elements in an array of integers `i32`**.
//!
//! ## Your task
//!
//! You need to implement the function `sum_array(arr: &[i32]) -> i32` that takes a slice of integers and returns the sum of all elements.
//!
//! ## Requirements
//!
//! - The `sum_array` function should return the sum of all elements in the array.
//!
//! ## Example
//!
//! ```rust
//! let arr = [1, 2, 3, 4, 5];
//!
//! let sum = sum_array(&arr);
//! assert_eq!(sum, 15); // 1 + 2 + 3 + 4 + 5 = 15
//! ```
//!
//! ## Hints
//!
//! - Use the `.iter()` method to iterate over the elements of the array.
//! - Use the `.sum()` method to calculate the sum of the elements.
//!
//! Run: `cargo run --example rf_sum_of_array`
//! Test: `cargo test --example rf_sum_of_array`
//! Filter: `cargo test --example rf_sum_of_array <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub fn sum_array(arr: &[i32]) -> i32 {
    // TODO: Implement the function here
    unimplemented!()
}

fn main() {
    println!("Implement the Sum Of Array challenge.");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_sum_array() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(sum_array(&arr), 15);
    }

    #[test]
    fn test_sum_array_with_negatives() {
        let arr = [-1, -2, -3, -4, -5];
        assert_eq!(sum_array(&arr), -15);
    }

    #[test]
    fn test_sum_array_with_mixed() {
        let arr = [1, -2, 3, -4, 5];
        assert_eq!(sum_array(&arr), 3);
    }

    #[test]
    fn test_sum_array_empty() {
        let arr: [i32; 0] = [];
        assert_eq!(sum_array(&arr), 0);
    }

    #[test]
    fn test_large_array() {
        let arr = [2; 1000];
        assert_eq!(sum_array(&arr), 2000);
    }
}
