//! # Rustfinity: Option To Result
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/option-to-result
//!
//! In the previous challenge we converted a `Result<T, E>` to an `Option<T>` by using the `.ok()` method, Rust provides us other mechanisms to convert an `Option<T>` to a `Result<T, E>` as well. In this challenge, you will learn how to use the `.ok_or()` method to convert an `Option<T>` to a `Result<T, E>`.
//!
//! ## Your Task
//!
//! Implement the function `get_first_element`:
//!
//! 1. Takes two parameters:
//!    - A vector of integers (`Vec<i32>`).
//!    - A minimum allowed value (`i32`).
//! 2. Use the `.first()` method to retrieve the first element of the vector, this returns `Option<&T>`.
//! 3. If the value is `None`, convert it to a `Result<T, E>` using the `.ok_or()` method with the error message `"Vector is empty"`.
//! 4. Then run a validation check to ensure the first element is greater than or equal to the minimum allowed value provided. If not, return an error with message `"First element is below the minimum allowed value"`.
//! 5. If everything is ok, return a `Ok(T)` with the first element.
//!
//! ## Hints
//!
//! If you're stuck, here are some hints to help you solve the challenge:
//!
//! <details>
//! <summary>Click here to reveal hints</summary>
//!
//! - You can use the `.ok_or()` method and propagate the
//!   error cleanly. e.g.
//!   ```rust
//!   let first_element = numbers
//!       .first()
//!       .ok_or("Vector is empty".to_string())?;
//!   ```
//!
//! </details>
//!
//! Run: `cargo run --example rf_option_to_result`
//! Test: `cargo test --example rf_option_to_result`
//! Filter: `cargo test --example rf_option_to_result <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub fn get_first_element(numbers: Vec<i32>, min_value: i32) -> Result<i32, String> {
    // Finish the function
    let first_element = numbers.first(); // <- Returns an Option<&i32>
    todo!("convert the first value at least {min_value} to Result")
}

// Example usage
pub fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    match get_first_element(numbers.clone(), 15) {
        Ok(value) => println!("First valid value: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    let empty_numbers: Vec<i32> = vec![];
    match get_first_element(empty_numbers, 15) {
        Ok(value) => println!("First valid value: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_get_first_element_valid() {
        let numbers = vec![20, 30, 40, 50];
        assert_eq!(get_first_element(numbers, 15), Ok(20));
    }

    #[test]
    fn test_get_first_element_empty_vector() {
        let numbers: Vec<i32> = vec![];
        assert_eq!(
            get_first_element(numbers, 15),
            Err("Vector is empty".to_string())
        );
    }

    #[test]
    fn test_get_first_element_below_min_value() {
        let numbers = vec![10, 20, 30, 40];
        assert_eq!(
            get_first_element(numbers, 15),
            Err("First element is below the minimum allowed value".to_string())
        );
    }

    #[test]
    fn test_get_first_element_edge_case_valid() {
        let numbers = vec![15, 20, 30];
        assert_eq!(get_first_element(numbers, 15), Ok(15));
    }

    #[test]
    fn test_get_first_element_negative_values() {
        let numbers = vec![-5, -10, 0];
        assert_eq!(get_first_element(numbers, -10), Ok(-5));
    }

    #[test]
    fn test_get_first_element_large_min_value() {
        let numbers = vec![1000, 2000, 3000];
        assert_eq!(
            get_first_element(numbers, 5000),
            Err("First element is below the minimum allowed value".to_string())
        );
    }
}
