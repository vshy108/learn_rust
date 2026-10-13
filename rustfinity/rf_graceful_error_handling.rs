//! # Rustfinity: Graceful Error Handling
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/graceful-error-handling
//!
//! One of the most loved features of Rust is the way it lets you handle errors. The `Result` type is a powerful tool that allows you to handle errors in a way that is both safe and expressive. In this challenge, you will be working with the `Result<T, E>` type to handle errors in a graceful way.
//!
//! The `Result<T, E>` itself is an enum that has two variants: `Ok(T)` and `Err(E)`. The `Ok` variant is used to represent a successful computation that returns a value of type `T`. The `Err` variant is used to represent an error that returns a value of type `E`.
//!
//! When you have a function that can fail, you can use the `Result` type to return the result of the computation. If the computation is successful, you can return the success variant of `Result` with the value of the computation. If the computation fails, you can return the error variant of `Result` with an error message that explains what went wrong.
//!
//! ## Your task
//!
//! In this challenge, you're given a function, `parse_percentage` that takes a string as input and returns a `Result` type. The function should parse the input string as a percentage and return the percentage as a `u8` if the input is valid. If the input is invalid, the function should return an error message as a `String`.
//!
//! Parsing from a string to a number can fail for many reasons. For example, the input string may not be a valid number, or it may be a valid number but not a valid percentage. Your task is to handle these errors gracefully and return an error message that explains what went wrong.
//!
//! Complete the function, if the parsing was successful return a success variant of the `Result`, if there was an error in parsing, return an error variant of the `Result` with an error message.
//!
//! ### Requirements
//!
//! - If the parse was successful, the function should return the success variant of `Result` with the percentage as a `Ok(u8)`.
//! - If the number was out of range (not between 0 and 100), the function should return the error with `String` `Err("Percentage out of range")`.
//! - If the string was not a valid number, the function should return the error with `String` `Err("Invalid input")`.
//!
//! Run: `cargo run --example rf_graceful_error_handling`
//! Test: `cargo test --example rf_graceful_error_handling`
//! Filter: `cargo test --example rf_graceful_error_handling <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

pub fn parse_percentage(input: &str) -> Result<u8, String> {
    // TODO: Implement the function here
    unimplemented!()
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    assert_eq!(result, Ok(50));

    let result = parse_percentage("101");
    assert_eq!(result, Err("Percentage out of range".to_string()));

    let result = parse_percentage("abc");
    assert_eq!(result, Err("Invalid input".to_string()));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse_percentage_valid() {
        assert_eq!(parse_percentage("75"), Ok(75));
    }

    #[test]
    fn test_parse_percentage_out_of_range() {
        assert_eq!(
            parse_percentage("150"),
            Err(String::from("Percentage out of range"))
        );
    }

    #[test]
    fn test_parse_percentage_non_numeric() {
        assert_eq!(parse_percentage("abc"), Err(String::from("Invalid input")));
    }

    #[test]
    fn test_parse_percentage_edge_cases() {
        assert_eq!(parse_percentage("0"), Ok(0));
        assert_eq!(parse_percentage("100"), Ok(100));
    }
}
