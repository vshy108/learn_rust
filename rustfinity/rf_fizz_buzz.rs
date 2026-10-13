//! # Rustfinity: Fizz Buzz
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/fizz-buzz
//!
//! Your task is to write a Rust program that implements the classic programming challenge known as **FizzBuzz**.
//!
//! You are given a function `fizz_buzz(num: u32) -> String` that takes a single parameter `num` of type `u32` and returns a `String`.
//!
//! Your job is to return a string based on the following rules:
//!
//! - Return "Fizz" for numbers that are multiples of 3.
//! - Return "Buzz" for numbers that are multiples of 5.
//! - Return "FizzBuzz" for numbers that are multiples of both 3 and 5.
//! - Return the number itself for all other numbers.
//!
//! ## Example
//!
//! Here's how the behavior of the function should look like:
//!
//! ```rust
//! assert_eq!(fizz_buzz(1), "1");
//! assert_eq!(fizz_buzz(2), "2");
//! assert_eq!(fizz_buzz(3), "Fizz");
//! assert_eq!(fizz_buzz(4), "4");
//! assert_eq!(fizz_buzz(5), "Buzz");
//! assert_eq!(fizz_buzz(6), "Fizz");
//! assert_eq!(fizz_buzz(7), "7");
//! assert_eq!(fizz_buzz(8), "8");
//! assert_eq!(fizz_buzz(9), "Fizz");
//! assert_eq!(fizz_buzz(10), "Buzz");
//! assert_eq!(fizz_buzz(11), "11");
//! assert_eq!(fizz_buzz(12), "Fizz");
//! assert_eq!(fizz_buzz(13), "13");
//! assert_eq!(fizz_buzz(14), "14");
//! assert_eq!(fizz_buzz(15), "FizzBuzz");
//! ```
//!
//! Good luck, and happy coding!
//!
//! Run: `cargo run --example rf_fizz_buzz`
//! Test: `cargo test --example rf_fizz_buzz`
//! Filter: `cargo test --example rf_fizz_buzz <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

pub fn fizz_buzz(num: u32) -> String {
    // TODO: Implement the FizzBuzz challenge
    unimplemented!()
}

fn main() {
    println!("Implement the Fizz Buzz challenge.");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn should_return_fizz() {
        assert_eq!(fizz_buzz(3), "Fizz");
        assert_eq!(fizz_buzz(6), "Fizz");
        assert_eq!(fizz_buzz(9), "Fizz");
        assert_eq!(fizz_buzz(12), "Fizz");
        assert_eq!(fizz_buzz(18), "Fizz");
    }

    #[test]
    fn should_return_buzz() {
        assert_eq!(fizz_buzz(5), "Buzz");
        assert_eq!(fizz_buzz(10), "Buzz");
        assert_eq!(fizz_buzz(20), "Buzz");
        assert_eq!(fizz_buzz(25), "Buzz");
        assert_eq!(fizz_buzz(35), "Buzz");
    }

    #[test]
    fn should_return_fizz_buzz() {
        assert_eq!(fizz_buzz(15), "FizzBuzz");
        assert_eq!(fizz_buzz(30), "FizzBuzz");
        assert_eq!(fizz_buzz(45), "FizzBuzz");
        assert_eq!(fizz_buzz(60), "FizzBuzz");
        assert_eq!(fizz_buzz(75), "FizzBuzz");
    }

    #[test]
    fn should_return_number() {
        assert_eq!(fizz_buzz(1), "1");
        assert_eq!(fizz_buzz(2), "2");
        assert_eq!(fizz_buzz(4), "4");
        assert_eq!(fizz_buzz(7), "7");
        assert_eq!(fizz_buzz(8), "8");
    }
}
