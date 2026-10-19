//! # Rustfinity: Optional Values
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/optional-values
//!
//! Rust's `Option<T>` is a powerful enum type that represents a value that might or might not be present. It is often used to safely handle cases where a value could be missing or invalid.
//!
//! The `Option<T>` enum has two variants:
//!
//! - `Some(T)` which contains a value of type `T`.
//! - `None` which signifies the absence of a value.
//!
//! ## Your Task
//!
//! In this challenge, you will implement a function `find_first_even` that takes a list of integers and returns the first even number in the list wrapped in `Some`. If no even number is present, the function should return `None`.
//!
//! For example:
//!
//! - If the input list is `[1, 3, 5, 8, 10]`, the function should return `Some(8)`.
//! - If the input list is `[1, 3, 5]`, the function should return `None`.
//!
//! Your task is to implement the function so it correctly handles any list of integers.
//!
//! Run: `cargo run --example rf_optional_values`
//! Test: `cargo test --example rf_optional_values`
//! Filter: `cargo test --example rf_optional_values <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub fn find_first_even(numbers: &[i32]) -> Option<i32> {
    // Your code here...
    unimplemented!()
}

// Example usage
pub fn main() {
    let nums1 = vec![1, 3, 5, 8];
    let nums2 = vec![1, 3, 5];

    println!("{:?}", find_first_even(&nums1)); // Output: Some(8)
    println!("{:?}", find_first_even(&nums2)); // Output: None
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_find_first_even_with_even_numbers() {
        let nums = vec![1, 3, 5, 8];
        assert_eq!(find_first_even(&nums), Some(8));
    }

    #[test]
    fn test_find_first_even_no_even_numbers() {
        let nums = vec![1, 3, 5];
        assert_eq!(find_first_even(&nums), None);
    }

    #[test]
    fn test_find_first_even_empty_list() {
        let nums: Vec<i32> = vec![];
        assert_eq!(find_first_even(&nums), None);
    }

    #[test]
    fn test_find_first_even_all_even_numbers() {
        let nums = vec![2, 4, 6];
        assert_eq!(find_first_even(&nums), Some(2));
    }

    #[test]
    fn test_find_first_even_single_even_number() {
        let nums = vec![7, 8, 9];
        assert_eq!(find_first_even(&nums), Some(8));
    }

    #[test]
    fn test_find_first_even_single_element_even() {
        let nums = vec![4];
        assert_eq!(find_first_even(&nums), Some(4));
    }

    #[test]
    fn test_find_first_even_single_element_odd() {
        let nums = vec![3];
        assert_eq!(find_first_even(&nums), None);
    }

    #[test]
    fn test_find_first_even_mixed_with_negatives() {
        let nums = vec![-1, -2, -3, -4];
        assert_eq!(find_first_even(&nums), Some(-2));
    }

    #[test]
    fn test_find_first_even_large_numbers() {
        let nums = vec![1_000_001, 1_000_002, 1_000_003];
        assert_eq!(find_first_even(&nums), Some(1_000_002));
    }

    #[test]
    fn test_find_first_even_zeros() {
        let nums = vec![0, 1, 2, 3];
        assert_eq!(find_first_even(&nums), Some(0));
    }

    #[test]
    fn test_find_first_even_multiple_of_ten() {
        let nums = vec![10, 20, 30];
        assert_eq!(find_first_even(&nums), Some(10));
    }
}
