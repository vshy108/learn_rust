//! # Rustfinity: Option Propagation
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/option-propagation
//!
//! In the previous challenge, you learned how to propagate errors using the `Result<T, E>` type for file handling. Rust also lets you use the `?` operator to propagate `Option<T>` values, but instead of propagating the error it will convert the `Option<T>` into a `None` value if it is `None`.
//!
//! This challenge focuses on propagating `Option` values and handling scenarios where optional values might be missing.
//!
//! ## Your Task
//!
//! Implement the function `find_and_multiply`:
//!
//! 1. It takes three parameters:
//!    - A vector of integers (`Vec<i32>`).
//!    - Two indices (`usize` values).
//! 2. The function retrieves the integers at the two specified indices from the vector.
//! 3. If both indices are valid, it returns the product of the two integers as an `Option<i32>`.
//! 4. If any index is out of bounds, the function returns `None`.
//! 5. Use the `?` operator to propagate `None` values.
//!
//! Run: `cargo run --example rf_option_propagation`
//! Test: `cargo test --example rf_option_propagation`
//! Filter: `cargo test --example rf_option_propagation <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

pub fn find_and_multiply(numbers: Vec<i32>, index1: usize, index2: usize) -> Option<i32> {
    // TODO: Instead of using `unwrap`, use the `?` operator to propagate the option
    // HINT: `numbers.get` returns a Option<i32> value

    let num1 = numbers.get(index1).unwrap();
    let num2 = numbers.get(index2).unwrap();
    Some(num1 * num2)
}

// Example usage
pub fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Example 1: Both indices are valid
    let result = find_and_multiply(numbers.clone(), 1, 3);
    println!("{:?}", result); // Should print Some(8)

    // Example 2: One index is out of bounds
    let result = find_and_multiply(numbers.clone(), 1, 10);
    println!("{:?}", result); // Should print None
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_find_and_multiply_valid_indices() {
        let numbers = vec![2, 4, 6, 8, 10];
        assert_eq!(find_and_multiply(numbers, 1, 3), Some(32));
    }

    #[test]
    fn test_find_and_multiply_first_index_out_of_bounds() {
        let numbers = vec![2, 4, 6, 8, 10];
        assert_eq!(find_and_multiply(numbers, 10, 3), None);
    }

    #[test]
    fn test_find_and_multiply_second_index_out_of_bounds() {
        let numbers = vec![2, 4, 6, 8, 10];
        assert_eq!(find_and_multiply(numbers, 1, 10), None);
    }

    #[test]
    fn test_find_and_multiply_both_indices_out_of_bounds() {
        let numbers = vec![2, 4, 6, 8, 10];
        assert_eq!(find_and_multiply(numbers, 10, 20), None);
    }

    #[test]
    fn test_find_and_multiply_empty_vector() {
        let numbers: Vec<i32> = vec![];
        assert_eq!(find_and_multiply(numbers, 0, 1), None);
    }

    #[test]
    fn test_find_and_multiply_same_index() {
        let numbers = vec![3, 5, 7, 9];
        assert_eq!(find_and_multiply(numbers, 2, 2), Some(49)); // 7 * 7
    }
}
