//! # Rustfinity: Slice Manipulation
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/slice-manipulation
//!
//! Now that you have an overview of slices, let's make it a little bit more challenging. In this problem, you will implement a function that updates specific elements of a mutable slice.
//!
//! ## Your Task
//!
//! Write a function `update_slice(slice: &mut [i32], indices: &[usize], value: i32)` that updates specific elements of a mutable slice. The function should:
//!
//! 1. Take a mutable slice of integers (`slice`) as the first argument.
//! 2. Take a slice of indices (`indices`) that specify which elements of the mutable slice to update.
//! 3. Update each specified index in the slice to the given `value`.
//!
//! The function should handle the following:
//!
//! - If an index in `indices` is out of bounds for the `slice`, the function should skip it without causing a panic.
//! - Modify only the elements specified by valid indices.
//! - Ensure that out-of-bound indices in the `indices` slice do not cause runtime errors.
//! - Remember that slices are views into arrays or vectors; they cannot be resized, but their contents can be modified.
//!
//! ## How it Works
//!
//! ```rust
//! let mut data = vec![1, 2, 3, 4, 5];
//! update_slice(&mut data, &[1, 3, 4], 7);
//! assert_eq!(data, vec![1, 7, 3, 7, 7]);
//!
//! let mut data = vec![10, 20, 30];
//! // Index 5 is out of bounds
//! update_slice(&mut data, &[2, 5], 100);
//! assert_eq!(data, vec![10, 20, 100]);
//! ```
//!
//! ## Hints
//!
//! If you're having trouble, consider these hints:
//!
//! <details>
//!   <summary>Click here to reveal hints</summary>
//!
//! - Use the `.get_mut(index)` method to safely access a mutable reference to an element at a given index. This avoids panics for out-of-bound accesses.
//! - A `for` loop is useful for iterating through the `indices` slice.
//!
//! </details>
//!
//! Run: `cargo run --example rf_slice_manipulation`
//! Test: `cargo test --example rf_slice_manipulation`
//! Filter: `cargo test --example rf_slice_manipulation <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub fn update_slice(slice: &mut [i32], indices: &[usize], value: i32) {
    // Implement your logic here
}

fn main() {
    println!("Implement the Slice Manipulation challenge.");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_update_valid_indices() {
        let mut data = vec![1, 2, 3, 4, 5];
        update_slice(&mut data, &[1, 3, 4], 7);
        assert_eq!(data, vec![1, 7, 3, 7, 7]);
    }

    #[test]
    fn test_update_with_out_of_bounds_indices() {
        let mut data = vec![10, 20, 30];
        update_slice(&mut data, &[2, 5], 100); // Index 5 is out of bounds
        assert_eq!(data, vec![10, 20, 100]);
    }

    #[test]
    fn test_update_empty_indices() {
        let mut data = vec![5, 15, 25];
        update_slice(&mut data, &[], 50); // No indices provided
        assert_eq!(data, vec![5, 15, 25]); // No changes made
    }

    #[test]
    fn test_update_all_indices() {
        let mut data = vec![0, 0, 0];
        update_slice(&mut data, &[0, 1, 2], 42);
        assert_eq!(data, vec![42, 42, 42]);
    }

    #[test]
    fn test_update_no_valid_indices() {
        let mut data = vec![8, 9, 10];
        update_slice(&mut data, &[3, 4, 5], 99); // All indices are out of bounds
        assert_eq!(data, vec![8, 9, 10]); // No changes made
    }
}
