//! # Rustfinity: Control Flow
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/control-flow
//!
//! Control flow is a fundamental concept in programming that allows you to dictate how your code executes based on certain conditions. In Rust, one of the ways to control the flow of your program is by using `if-else` statements.
//!
//! In this challenge, you will implement a function that checks whether a number is **positive**, **negative**, or **zero**. Depending on the value, your function should return a corresponding string message.
//!
//! ## Your task
//!
//! Your task is to complete the function `check_number_sign` that takes an **integer `i32`** as input and returns a `String` indicating whether the number is `"positive"`, `"negative"`, or `"zero"`.
//!
//! ## Requirements
//!
//! - If the number is greater than zero, return `"positive"`.
//! - If the number is less than zero, return `"negative"`.
//! - If the number is equal to zero, return `"zero"`.
//!
//! ## Example
//!
//! ```rust
//! let result = check_number_sign(10);
//! assert_eq!(result, "positive");
//!
//! let result = check_number_sign(-5);
//! assert_eq!(result, "negative");
//!
//! let result = check_number_sign(0);
//! assert_eq!(result, "zero");
//! ```
//!
//! ## Hints
//!
//! - You can use the `if`, `else if`, and `else` keywords to implement the control flow.
//! - Remember to return the result as a `String`.
//!
//! Run: `cargo run --example rf_control_flow`
//! Test: `cargo test --example rf_control_flow`
//! Filter: `cargo test --example rf_control_flow <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub fn check_number_sign(number: i32) -> String {
    // Return `"positive"` if the number is positive.
    // Return `"negative"` if the number is negative.
    // Return `"zero"` if the number is zero.

    // Step 1:
    // Check if the number is positive.

    // Step 2:
    // Check if the number is negative.

    // Step 3:
    // Handle the case where it's neither positive nor negative.
    unimplemented!()
}

fn main() {
    println!("Implement the Control Flow challenge.");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_positive_number() {
        assert_eq!(check_number_sign(10), "positive");
    }

    #[test]
    fn test_negative_number() {
        assert_eq!(check_number_sign(-5), "negative");
    }

    #[test]
    fn test_zero() {
        assert_eq!(check_number_sign(0), "zero");
    }

    #[test]
    fn test_large_positive_number() {
        assert_eq!(check_number_sign(1000000), "positive");
    }

    #[test]
    fn test_large_negative_number() {
        assert_eq!(check_number_sign(-1000000), "negative");
    }
}
