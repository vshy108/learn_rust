//! # Rustfinity: Constants
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/constants
//!
//! In Rust, **constants** are values that are bound to a name and are **not allowed to change**. Constants are always immutable and are declared using the `const` keyword. They must be annotated with their type, and they can be set only to a constant expression.
//!
//! In this challenge, you will define and use a constant in Rust. This will help you understand how to declare constants and how they can be utilized in your programs.
//!
//! ## Your task
//!
//! - Define a constant named `MAX_SIZE` with a value of `100`.
//! - Ensure that `MAX_SIZE` has the type `i32`.
//! - Return the value of `MAX_SIZE` from the `main` function.
//! - Make sure you make the constant **public** using the `pub` keyword (important for the tests to pass).
//! - Remember that constants can only be defined at the global scope. You can not define them inside a function.
//!
//! ## Hints
//!
//! - Use the `const` keyword to define the constant.
//! - Ensure you **specify the type** of the constant explicitly.
//!
//! Run: `cargo run --example rf_constants`
//! Test: `cargo test --example rf_constants`
//! Filter: `cargo test --example rf_constants <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

// Define a constant MAX_SIZE with a value of 100.
// NOTE: Define the constant outside the main function
pub const MAX_SIZE: i32 = 0; // TODO: Set the required constant value.

pub fn main_value() -> i32 {
    MAX_SIZE
}

fn main() {
    let _ = main_value();
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn compiles() {
        assert_eq!(main_value(), 100, "Expected main_value() to return 100");
    }

    #[test]
    fn test_constants() {
        assert_eq!(MAX_SIZE, 100, "Expected MAX_SIZE to be 100");
    }
}
