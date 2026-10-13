//! # Rustfinity: Mutable Slices
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/mutable-slices
//!
//! In this challenge, you'll explore how to work with **mutable slices** in Rust. A mutable slice, `&mut [T]`, allows you to modify the elements of a collection directly and efficiently without creating a new collection.
//!
//! Slices are a fundamental part of Rust, providing a view into contiguous sequences of elements such as arrays or vectors. When working with slices, you can iterate over and manipulate the elements dynamically.
//!
//! ## Your Task
//!
//! Write a function `transform_even_odd(slice: &mut [i32])` that modifies the elements of a mutable slice of integers in the following way:
//!
//! 1. **Double each even number** in the slice.
//! 2. **Subtract 1 from each odd number** in the slice.
//! 3. Modify the elements directly without creating a new collection.
//!
//! ## Example
//!
//! ```rust
//! let mut numbers = [1, 2, 3, 4, 5];
//! transform_even_odd(&mut numbers);
//!
//! // Odd numbers reduced by 1
//! // Even numbers doubled
//! assert_eq!(numbers, [0, 4, 2, 8, 4]);
//!
//! let mut numbers = [10, 15, 20];
//! transform_even_odd(&mut numbers);
//!
//! // 10 -> 20, 15 -> 14, 20 -> 40
//! assert_eq!(numbers, [20, 14, 40]);
//! ```
//!
//! ## Hints
//!
//! If you're stuck, here are some hints to help you:
//!
//! <details>
//!     <summary>Additional Hints</summary>
//!
//! - Use the `.iter_mut()` method to traverse the slice by mutable reference.
//! - Remember that you can check if a number is even using `num % 2 == 0`.
//! - Use dereferencing (`*`) to update the value pointed to by the mutable reference.
//! - You can use a `for` loop with mutable references for concise modification.
//! - The slice directly modifies the original collection it is derived from, so no need to return anything.
//!
//! </details>
//!
//! Run: `cargo run --example rf_mutable_slices`
//! Test: `cargo test --example rf_mutable_slices`
//! Filter: `cargo test --example rf_mutable_slices <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

pub fn transform_even_odd(slice: &mut [i32]) {
    // Your code here: iterate over the mutable slice and modify its elements.
}

fn main() {
    println!("Implement the Mutable Slices challenge.");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_transform_even_odd_mixed() {
        let mut numbers = [1, 2, 3, 4, 5];
        transform_even_odd(&mut numbers);
        assert_eq!(numbers, [0, 4, 2, 8, 4]);
    }

    #[test]
    fn test_transform_even_odd_even_only() {
        let mut numbers = [2, 4, 6];
        transform_even_odd(&mut numbers);
        assert_eq!(numbers, [4, 8, 12]);
    }

    #[test]
    fn test_transform_even_odd_odd_only() {
        let mut numbers = [1, 3, 5];
        transform_even_odd(&mut numbers);
        assert_eq!(numbers, [0, 2, 4]);
    }

    #[test]
    fn test_transform_even_odd_empty_slice() {
        let mut numbers: [i32; 0] = [];
        transform_even_odd(&mut numbers);
        assert_eq!(numbers, []);
    }

    #[test]
    fn test_transform_even_odd_large_numbers() {
        let mut numbers = [100, 101, 102];
        transform_even_odd(&mut numbers);
        assert_eq!(numbers, [200, 100, 204]);
    }

    #[test]
    fn test_transform_even_odd_negative_numbers() {
        let mut numbers = [-1, -2, -3, -4];
        transform_even_odd(&mut numbers);
        assert_eq!(numbers, [-2, -4, -4, -8]); // Odd numbers reduced by 1, even numbers doubled
    }

    #[test]
    fn test_transform_even_odd_single_even() {
        let mut numbers = [8];
        transform_even_odd(&mut numbers);
        assert_eq!(numbers, [16]); // 8 doubled
    }

    #[test]
    fn test_transform_even_odd_single_odd() {
        let mut numbers = [7];
        transform_even_odd(&mut numbers);
        assert_eq!(numbers, [6]); // 7 reduced by 1
    }

    #[test]
    fn test_transform_even_odd_with_zeros() {
        let mut numbers = [0, 1, 2, 0];
        transform_even_odd(&mut numbers);
        assert_eq!(numbers, [0, 0, 4, 0]); // 0 remains 0, odd numbers reduced by 1, even numbers doubled
    }

    #[test]
    fn test_transform_even_odd_large_array() {
        let mut numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        transform_even_odd(&mut numbers);
        assert_eq!(numbers, [0, 4, 2, 8, 4, 12, 6, 16, 8, 20]);
    }

    #[test]
    fn test_transform_even_odd_alternating_even_odd() {
        let mut numbers = [1, 2, 3, 4, 5, 6];
        transform_even_odd(&mut numbers);
        assert_eq!(numbers, [0, 4, 2, 8, 4, 12]);
    }

    #[test]
    fn test_transform_even_odd_repeated_values() {
        let mut numbers = [2, 2, 2, 3, 3, 3];
        transform_even_odd(&mut numbers);
        assert_eq!(numbers, [4, 4, 4, 2, 2, 2]); // All evens doubled, all odds reduced by 1
    }

    #[test]
    fn test_transform_even_odd_all_zeroes() {
        let mut numbers = [0, 0, 0];
        transform_even_odd(&mut numbers);
        assert_eq!(numbers, [0, 0, 0]); // Zeros remain unchanged
    }
}
