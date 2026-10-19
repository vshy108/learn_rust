//! # Rustfinity: Countdown
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/countdown
//!
//! In this challenge, you will implement a simple countdown timer using a **while loop**. The goal is to create a function that takes an integer `n` and returns a vector containing the countdown from `n` to 0.
//!
//! ## Your task
//!
//! Write a function `countdown(n: u32) -> Vec<u32>` that takes a non-negative integer `n` and returns a vector with the numbers from `n` to 0.
//!
//! ## Requirements
//!
//! - Use a `while` loop to implement the countdown.
//! - Ensure the function handles the case where `n` is 0 correctly.
//! - The function should return a vector with each number in the countdown sequence.
//!
//! ## Example
//!
//! ```rust
//! let result = countdown(3);
//! assert_eq!(result, vec![3, 2, 1, 0]);
//!
//! let result = countdown(0);
//! assert_eq!(result, vec![0]);
//! ```
//!
//! Run: `cargo run --example rf_countdown`
//! Test: `cargo test --example rf_countdown`
//! Filter: `cargo test --example rf_countdown <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub fn countdown(n: u32) -> Vec<u32> {
    // TODO: Implement the countdown using a while loop
    unimplemented!()
}

fn main() {
    println!("Implement the Countdown challenge.");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_countdown() {
        let result = countdown(3);
        assert_eq!(result, vec![3, 2, 1, 0]);
    }

    #[test]
    fn test_countdown_zero() {
        let result = countdown(0);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_countdown_large() {
        let result = countdown(10000);
        assert_eq!(result, (0..=10000).rev().collect::<Vec<u32>>());
    }
}
