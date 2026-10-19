//! # Rustfinity: Is Prime
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/is-prime
//!
//! ## Checking Prime Numbers
//!
//! Prime numbers are **natural numbers greater than 1 that have no positive divisors other than 1 and themselves**. Determining whether a number is prime is a fundamental problem in number theory and has applications in various fields, including cryptography and computer science.
//!
//! In this challenge, you will implement a function to check if a given number is prime. You will use logical operators and conditional statements to handle the multiple conditions required to identify a prime number efficiently.
//!
//! ## Your task
//!
//! Your task is to complete the function `is_prime(n: u32) -> bool` that takes an unsigned integer `n` and returns a boolean value indicating whether `n` is a prime number.
//!
//! ## Requirements
//!
//! - The function should return `true` if `n` is a prime number and `false` otherwise.
//! - A prime number is a natural number greater than 1 that is not divisible by any number other than 1 and itself.
//! - Use logical operators and conditional statements to check the conditions for a prime number.
//! - Optimize the function to minimize unnecessary checks.
//!
//! ## Example
//!
//! ```rust
//! let result = is_prime(5);
//! assert_eq!(result, true);
//!
//! let result = is_prime(4);
//! assert_eq!(result, false);
//! ```
//!
//! ## Hints
//!
//! - Any number less than 2 is not prime.
//! - The number 2 is the only even prime number.
//! - For any other even number greater than 2, return false.
//! - Check divisibility starting from 3 up to the square root of the number.
//!
//! Run: `cargo run --example rf_is_prime`
//! Test: `cargo test --example rf_is_prime`
//! Filter: `cargo test --example rf_is_prime <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub fn is_prime(n: u32) -> bool {
    // Implement your code here
    unimplemented!()
}

fn main() {
    println!("Implement the Is Prime challenge.");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_is_prime_small_numbers() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
    }

    #[test]
    fn test_is_prime_large_numbers() {
        assert_eq!(is_prime(29), true);
        assert_eq!(is_prime(49), false);
        assert_eq!(is_prime(97), true);
    }

    #[test]
    fn test_is_prime_even_numbers() {
        assert_eq!(is_prime(10), false);
        assert_eq!(is_prime(12), false);
        assert_eq!(is_prime(14), false);
    }

    #[test]
    fn test_is_prime_odd_numbers() {
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(13), true);
    }
}
