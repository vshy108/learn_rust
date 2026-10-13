//! # Rustfinity: Rc
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/rc
//!
//! Reference-counted pointers, `Rc<T>`, are a type of smart pointer in Rust that allow multiple ownership of the same data. This means you can have multiple references to a value, and the value will only be dropped when the last reference is out of scope. This is particularly useful in scenarios where you want to share data between multiple parts of your program without needing to copy it.
//!
//! In this challenge, you'll use `Rc<T>` to share data between functions.
//!
//! ## Your Task
//!
//! Implement the functions `use_shared_data` and `share_data_to_other_functions` to work with `Rc<T>`.
//!
//! - `use_shared_data`:
//!   - Take an `Rc<Vec<T>>` as argument.
//!   - Loop over each item in the vector and print each element using `println!`.
//!
//! - `share_data_to_other_functions`:
//!   - Share the input as a reference-counted pointer 3 times with the given closure.
//!   - Do cheap clones only and avoid deep copying the data.
//!
//! ## Hints
//!
//! If you're stuck, here are some hints to help you solve the challenge:
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - Use `Rc::new` to wrap your vector in an `Rc` smart pointer.
//! - Use `Rc::clone` to create new references to the shared data.
//! - For `use_shared_data`, make sure to use `T: Display` to allow printing elements with `{}` formatting.
//!
//! </details>
//!
//! Run: `cargo run --example rf_rc`
//! Test: `cargo test --example rf_rc`
//! Filter: `cargo test --example rf_rc <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::rc::Rc;

pub fn use_shared_data<T>(data: Rc<Vec<T>>) {
    // 1. Loop over each item in the vector and print it using `println!`
}

pub fn share_data_to_other_functions<F>(mut take_item: F, items: Vec<String>)
where
    F: FnMut(Rc<Vec<String>>),
{
    // 2. Implement the function
    // Share the data as a reference-counted pointer 3 times with the closure
}

// Example usage
pub fn main() {
    // Example usage of `use_shared_data`
    let shared_numbers = Rc::new(vec![1, 2, 3, 4, 5]);
    println!("Using shared data:");
    use_shared_data(Rc::clone(&shared_numbers));

    // Example usage of `share_data_to_other_functions`
    let strings = vec!["Rust".to_string(), "is".to_string(), "awesome!".to_string()];

    let print_data = |data: Rc<Vec<String>>| {
        println!("Printing shared data:");
        for item in data.iter() {
            println!("{}", item);
        }
    };

    println!("\nSharing data with other functions:");
    share_data_to_other_functions(print_data, strings);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn test_use_shared_data_with_numbers() {
        let data = Rc::new(vec![10, 20, 30]);
        use_shared_data(Rc::clone(&data)); // Expect output: 10, 20, 30
                                           // Since this function prints to stdout, manual verification is required for correctness.
    }

    #[test]
    fn test_use_shared_data_empty_vector() {
        let data: Rc<Vec<i32>> = Rc::new(vec![]);
        use_shared_data(Rc::clone(&data)); // Expect no output
                                           // Since this function prints to stdout, manual verification is required for correctness.
    }

    #[test]
    fn test_share_data_to_other_functions_with_custom_closure() {
        let mut captured_data = Vec::new();
        let capture_closure = |data: Rc<Vec<String>>| {
            captured_data.push(data.clone());
        };

        let items = vec!["Rust".to_string(), "Rc".to_string(), "Test".to_string()];
        share_data_to_other_functions(capture_closure, items.clone());

        assert_eq!(
            captured_data.len(),
            3,
            "The closure should have been called 3 times"
        );
        for rc_data in captured_data {
            assert_eq!(
                rc_data.as_ref(),
                &items,
                "All Rc references should contain the same vector data"
            );
        }
    }

    #[test]
    fn test_share_data_to_other_functions_with_different_closures() {
        let mut call_count = 0;
        let count_closure = |_: Rc<Vec<String>>| {
            call_count += 1;
        };

        let items = vec!["One".to_string(), "Two".to_string()];
        share_data_to_other_functions(count_closure, items);

        assert_eq!(
            call_count, 3,
            "The closure should be invoked exactly 3 times"
        );
    }

    #[test]
    fn test_reference_count_increase() {
        let items = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let rc_data = Rc::new(items);

        let initial_count = Rc::strong_count(&rc_data);

        share_data_to_other_functions(
            |data: Rc<Vec<String>>| {
                let _ = Rc::clone(&data);
            },
            rc_data.as_ref().clone(),
        );

        assert_eq!(
            Rc::strong_count(&rc_data),
            initial_count,
            "The reference count should return to the initial count after function execution"
        );
    }

    #[test]
    fn test_reference_count_after_function_execution() {
        let items = vec!["Shared".to_string()];
        let rc_data = Rc::new(items);

        let initial_count = Rc::strong_count(&rc_data);

        let capture_closure = |data: Rc<Vec<String>>| {
            assert_eq!(
                Rc::strong_count(&data),
                initial_count + 1,
                "The reference count should increase while the closure holds a clone"
            );
        };

        share_data_to_other_functions(capture_closure, rc_data.as_ref().clone());

        assert_eq!(
            Rc::strong_count(&rc_data),
            initial_count,
            "The reference count should return to the initial count after the closure releases its clone"
        );
    }
}
