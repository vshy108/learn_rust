//! # Rustfinity: Graceful Error Handling 2
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/graceful-error-handling-2
//!
//! Let's improve previous example a little bit by returning a custom error type instead of a plain string. This will allow us to define specific error types and provide more structured error handling.
//!
//! ## Your Task
//!
//! The logic of the function remains the same as the previous challenge, but the returned error type is what you need to change.
//!
//! 1. Define an enum `ParsePercentageError` with the following variants:
//!    - `InvalidInput`: for inputs that cannot be parsed as numbers.
//!    - `OutOfRange`: for numbers that are not in the range 0-100.
//!
//! 2. Implement the `Error` trait for `ParsePercentageError`. Use the `std::error::Error` trait and provide human-readable descriptions for each error.
//!
//! 3. Update the `parse_percentage` function to:
//!    - Return `Ok(u8)` if the input is a valid percentage (between 0 and 100).
//!    - Return `Err(ParsePercentageError::OutOfRange)` if the number is out of range.
//!    - Return `Err(ParsePercentageError::InvalidInput)` if the input is not a valid number.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - The `std::error::Error` trait requires implementing the `Display` and `Debug` traits. You can derive `Debug` by `#[derive(Debug)]` and implement `Display` manually.
//! - Use the `std::fmt` module to implement `Display` for the error enum, which is required for the `Error` trait.
//!
//! </details>
//!
//! Run: `cargo run --example rf_graceful_error_handling_2`
//! Test: `cargo test --example rf_graceful_error_handling_2`
//! Filter: `cargo test --example rf_graceful_error_handling_2 <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

#[derive(Debug, PartialEq)]
pub enum ParsePercentageError {
    InvalidInput,
    OutOfRange,
}

impl std::fmt::Display for ParsePercentageError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("format percentage error {self:?}")
    }
}

impl std::error::Error for ParsePercentageError {}

pub fn parse_percentage(input: &str) -> Result<u8, ParsePercentageError> {
    todo!("parse percentage input {input}")
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    println!("{:?}", result); // Should print: Ok(50)

    let result = parse_percentage("101");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::OutOfRange)

    let result = parse_percentage("abc");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::InvalidInput)
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    fn implements_debug<T: std::fmt::Debug>(_: T) {}
    fn implements_display<T: std::fmt::Display>(_: T) {}
    fn implements_error<T: std::error::Error>(_: T) {}

    #[test]
    fn test_parse_percentage_valid() {
        assert_eq!(parse_percentage("75"), Ok(75));
    }

    #[test]
    fn test_parse_percentage_out_of_range() {
        assert_eq!(
            parse_percentage("150"),
            Err(ParsePercentageError::OutOfRange)
        );
    }

    #[test]
    fn test_parse_percentage_invalid_input() {
        assert_eq!(
            parse_percentage("abc"),
            Err(ParsePercentageError::InvalidInput)
        );
    }

    #[test]
    fn test_parse_percentage_edge_cases() {
        assert_eq!(parse_percentage("0"), Ok(0));
        assert_eq!(parse_percentage("100"), Ok(100));
    }

    #[test]
    fn test_implements_debug() {
        implements_debug(ParsePercentageError::InvalidInput);
        implements_debug(ParsePercentageError::OutOfRange);
    }

    #[test]
    fn test_implements_display() {
        implements_display(ParsePercentageError::InvalidInput);
        implements_display(ParsePercentageError::OutOfRange);
    }

    #[test]
    fn test_implements_error() {
        implements_error(ParsePercentageError::InvalidInput);
        implements_error(ParsePercentageError::OutOfRange);
    }
}
