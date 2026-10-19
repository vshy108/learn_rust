//! # Rustfinity: Vectors
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/vectors
//!
//! Vectors are one of Rust's most commonly used collections. They are growable arrays that can hold multiple elements of the same type. They are defined in Rust using the `Vec<T>` type, where `T` is the type of elements the vector contains.
//!
//! In this challenge, you will perform basic operations on a vector, such as adding, removing, and accessing elements. Understanding how to work with vectors is essential for building efficient and idiomatic Rust applications.
//!
//! ## Your Task
//!
//! You are required to implement the following functions:
//!
//! 1. `add_elements`: This function takes a mutable reference to a vector of integers and a slice of integers. It appends all elements from the slice to the vector.
//! 2. `remove_element`: This function takes a mutable reference to a vector of integers and an index. It removes the element at the given index if it exists.
//! 3. `get_element`: This function takes a reference to a vector of integers and an index. It returns the element at the given index as an `Option<i32>`.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - You can use `vec.extend_from_slice(elements)` to add all elements from a slice to a vector.
//! - Use `Vec::remove` to remove an element by index.
//! - To safely access an element by index, use the `get` method on vectors.
//! - Keep in mind that `Vec::remove` will panic if the index is out of bounds, so handle it carefully in `remove_element`.
//!
//! </details>
//!
//! Run: `cargo run --example rf_vectors`
//! Test: `cargo test --example rf_vectors`
//! Filter: `cargo test --example rf_vectors <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub fn add_elements(vec: &mut Vec<i32>, elements: &[i32]) {
    // Your code here
    unimplemented!()
}

pub fn remove_element(vec: &mut Vec<i32>, index: usize) {
    // Your code here
    unimplemented!()
}

pub fn get_element(vec: &Vec<i32>, index: usize) -> Option<i32> {
    // Your code here
    unimplemented!()
}

// Example usage
pub fn main() {
    let mut vec = vec![1, 2, 3];
    add_elements(&mut vec, &[4, 5]);
    assert_eq!(vec, vec![1, 2, 3, 4, 5]);

    remove_element(&mut vec, 2);
    assert_eq!(vec, vec![1, 2, 4, 5]);

    assert_eq!(get_element(&vec, 1), Some(2));
    assert_eq!(get_element(&vec, 10), None);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_add_elements_empty_vector() {
        let mut vec = Vec::new();
        add_elements(&mut vec, &[1, 2, 3]);
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_add_elements_empty_slice() {
        let mut vec = vec![1, 2, 3];
        add_elements(&mut vec, &[]);
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_add_elements_large_input() {
        let mut vec = vec![1];
        let large_input: Vec<i32> = (2..=1000).collect();
        add_elements(&mut vec, &large_input);
        assert_eq!(vec.len(), 1000);
        assert_eq!(vec[999], 1000);
    }

    #[test]
    fn test_remove_element_first() {
        let mut vec = vec![1, 2, 3];
        remove_element(&mut vec, 0);
        assert_eq!(vec, vec![2, 3]);
    }

    #[test]
    fn test_remove_element_last() {
        let mut vec = vec![1, 2, 3];
        remove_element(&mut vec, 2);
        assert_eq!(vec, vec![1, 2]);
    }

    #[test]
    fn test_remove_element_out_of_bounds_high() {
        let mut vec = vec![1, 2, 3];
        remove_element(&mut vec, 10); // Should do nothing
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_remove_element_out_of_bounds_negative() {
        let mut vec = vec![1, 2, 3];
        remove_element(&mut vec, usize::MAX); // Should do nothing
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_get_element_empty_vector() {
        let vec: Vec<i32> = Vec::new();
        assert_eq!(get_element(&vec, 0), None);
    }

    #[test]
    fn test_get_element_boundary_cases() {
        let vec = vec![1, 2, 3];
        assert_eq!(get_element(&vec, 0), Some(1)); // First element
        assert_eq!(get_element(&vec, 2), Some(3)); // Last element
        assert_eq!(get_element(&vec, 3), None); // Out of bounds
    }

    #[test]
    fn test_combined_operations() {
        let mut vec = vec![10];
        add_elements(&mut vec, &[20, 30, 40]);
        assert_eq!(vec, vec![10, 20, 30, 40]);

        remove_element(&mut vec, 1);
        assert_eq!(vec, vec![10, 30, 40]);

        assert_eq!(get_element(&vec, 1), Some(30));
        assert_eq!(get_element(&vec, 3), None);
    }
}
