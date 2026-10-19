//! # Rustfinity: Slices
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/slices
//!
//! Slices are one of Rust's most versatile and powerful tools for working with collections of data. A slice is a view into a contiguous sequence of elements in a collection, allowing you to operate on a subset of the data without needing to copy it.
//!
//! In this challenge, you will work with slices to find the largest number in a given range of a slice. Slices allow you to efficiently access and process subsets of arrays and vectors. Your task is to write a function that handles edge cases like empty slices and returns the largest number where possible.
//!
//! ## Your task
//!
//! Implement the function `find_largest_in_slice(slice: &[i32]) -> Option<i32>` that takes an immutable slice of integers and returns the largest integer in the slice. If the slice is empty, return `None`.
//!
//! Slices provide various methods that can help you work efficiently, such as iterators and bounds checking.
//!
//! ### Requirements
//!
//! - The function should return `Some(i32)` with the largest element in the slice if the slice is not empty.
//! - If the slice is empty, the function should return `None`.
//! - The function can not modify the input slice.
//!
//! ## Hints
//!
//! If you're stuck, here are some hints to help you solve the challenge:
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - Use the `.iter()` method on slices to create an iterator over the elements.
//! - Consider using the `.max()` method provided by iterators to find the largest value.
//! - If using `.max()`, remember to handle the `Option` it returns, as an empty iterator will yield `None`.
//!
//! </details>
//!
//! Run: `cargo run --example rf_slices`
//! Test: `cargo test --example rf_slices`
//! Filter: `cargo test --example rf_slices <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub fn find_largest_in_slice(slice: &[i32]) -> Option<i32> {
    // Your code here...
    unimplemented!()
}

// Example Usage
pub fn main() {
    let numbers = [1, 3, 7, 2, 5];
    assert_eq!(find_largest_in_slice(&numbers), Some(7));

    let empty: [i32; 0] = [];
    assert_eq!(find_largest_in_slice(&empty), None);

    let single_element = [42];
    assert_eq!(find_largest_in_slice(&single_element), Some(42));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use crate::find_largest_in_slice;

    #[test]
    fn test_largest_in_non_empty_slice() {
        let numbers = [10, 3, 7, 2, 99];
        assert_eq!(find_largest_in_slice(&numbers), Some(99));
    }

    #[test]
    fn test_empty_slice() {
        let empty: [i32; 0] = [];
        assert_eq!(find_largest_in_slice(&empty), None);
    }

    #[test]
    fn test_single_element_slice() {
        let single = [42];
        assert_eq!(find_largest_in_slice(&single), Some(42));
    }

    #[test]
    fn test_negative_numbers() {
        let negatives = [-10, -3, -7, -2, -99];
        assert_eq!(find_largest_in_slice(&negatives), Some(-2));
    }

    #[test]
    fn test_mixed_positive_and_negative() {
        let mixed = [-10, 20, -30, 40, -50];
        assert_eq!(find_largest_in_slice(&mixed), Some(40));
    }

    #[test]
    fn test_all_equal_elements() {
        let equal = [5, 5, 5, 5];
        assert_eq!(find_largest_in_slice(&equal), Some(5));
    }

    #[test]
    fn test_large_numbers() {
        let large_numbers = [1_000_000, 2_000_000, 3_000_000];
        assert_eq!(find_largest_in_slice(&large_numbers), Some(3_000_000));
    }

    #[test]
    fn test_edge_case_with_min_and_max_i32() {
        let edge_cases = [i32::MIN, 0, i32::MAX];
        assert_eq!(find_largest_in_slice(&edge_cases), Some(i32::MAX));
    }

    #[test]
    fn test_repeated_pattern_slice() {
        let pattern = [1, 3, 2, 1, 3, 2, 1, 3, 2];
        assert_eq!(find_largest_in_slice(&pattern), Some(3));
    }

    #[test]
    fn test_slice_with_zero() {
        let with_zero = [-1, 0, 1];
        assert_eq!(find_largest_in_slice(&with_zero), Some(1));
    }

    #[test]
    fn test_largest_at_end() {
        let end_large = [10, 20, 30, 40, 50];
        assert_eq!(find_largest_in_slice(&end_large), Some(50));
    }

    #[test]
    fn test_largest_at_start() {
        let start_large = [99, 10, 20, 30];
        assert_eq!(find_largest_in_slice(&start_large), Some(99));
    }
}
