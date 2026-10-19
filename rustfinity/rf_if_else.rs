//! # Rustfinity: If Else
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/if-else
//!
//! Control flow is a fundamental concept in any programming language, including Rust. The `if-else` statement is a basic control flow construct that allows you to execute different code blocks based on certain conditions.
//!
//! ## Your task
//!
//! In this challenge, you will write a function `is_even(n: i32) -> bool` that checks if a given number is even or odd using an `if-else` statement. The function should return `true` if the number is even, and `false` if it is odd.
//!
//! ## Requirements
//!
//! - The function should take an integer as input.
//! - Use an `if-else` statement to determine if the number is even.
//! - Return `true` if the number is even.
//! - Return `false` if the number is odd.
//!
//! ## Example
//!
//! ```rust
//! assert_eq!(is_even(4), true);
//! assert_eq!(is_even(7), false);
//! assert_eq!(is_even(0), true);
//! assert_eq!(is_even(-2), true);
//! assert_eq!(is_even(-3), false);
//! ```
//!
//! Run: `cargo run --example rf_if_else`
//! Test: `cargo test --example rf_if_else`
//! Filter: `cargo test --example rf_if_else <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub fn is_even(n: i32) -> bool {
    // Your code here...
    unimplemented!()
}

fn main() {
    println!("Implement the If Else challenge.");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(4), true);
        assert_eq!(is_even(7), false);
        assert_eq!(is_even(0), true);
        assert_eq!(is_even(-2), true);
        assert_eq!(is_even(-3), false);
    }
}
