//! # Rustfinity: Error Propagation
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/error-propagation
//!
//! Error propagation is a core concept in Rust that allows you to handle errors in a clean and structured way. Instead of having to handle each error manually on every step, you can easily use the `?` operator to propagate errors to a higher level so that they can be handled in a single place.
//!
//! In this challenge, you’ll use `io::Error` to represent potential issues when working with file I/O. This approach leverages Rust’s standard library for concise and idiomatic error handling.
//!
//! ## Your Task
//!
//! Your task is to implement a function that reads integers from a file, computes their sum, and gracefully propagates any errors using the `?` operator.
//!
//! Implement the function `sum_integers_from_file`:
//!
//! - Takes the file path as a parameter.
//! - Reads the file line by line, assuming each line contains
//!   a single integer or invalid data.
//! - Computes and returns the sum of all integers as a `Result<i32, io::Error>`.
//! - Handles the following:
//!   - If the file cannot be opened, propagate the `io::Error`.
//!   - If a line cannot be parsed as an integer, propagate
//!     a custom `io::Error` with a meaningful message.
//!
//! ### Requirements
//!
//! 1. Handle errors cleanly and propagate them using the `?` operator.
//! 2. For invalid lines, create an `io::Error` with `io::ErrorKind::InvalidData`.
//!
//! ## Hints
//!
//! If you're stuck, here are some hints to help you solve the challenge:
//!
//! <details>
//! <summary>Click here to reveal hints</summary>
//!
//! - Use `std::fs::File::open` to open a file.
//! - Use `io::BufReader::new` to read lines from the file.
//! - Convert strings to integers with the `str::parse` method.
//! - The `io::Error::new` function can create custom errors.
//! - To create the error type we want for invalid numbers, you can do the following:
//!   ```rust
//!   let error = io::Error::new(
//!       io::ErrorKind::InvalidData,
//!       "Invalid number"
//!   );
//!   ```
//! - To propagate an error, use the `?` operator. e.g.
//!   ```rust
//!   let file = File::open(file_path)?;
//!   ```
//! - You can transform an error type to another error type by using the `map_err` method. e.g.
//!   ```rust
//!   let num = num_str
//!       .parse::<i32>()
//!       .map_err(|_| {
//!           io::Error::new(
//!               io::ErrorKind::InvalidData,
//!               "Invalid number"
//!           )
//!       })?;
//!   ```
//!
//! </details>
//!
//! Run: `cargo run --example rf_error_propagation`
//! Test: `cargo test --example rf_error_propagation`
//! Filter: `cargo test --example rf_error_propagation <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::io;

pub fn sum_integers_from_file(file_path: &str) -> Result<i32, io::Error> {
    todo!("sum integers from file {file_path}")
    // TODO: Implement this function
    // Hint: Use `File::open`, `BufReader::new`, and `.lines()` to process the file.
    // Use `?` to propagate errors and `io::Error::new` for custom errors.
}

// Example usage
pub fn main() {
    let file_path = "numbers.txt";

    match sum_integers_from_file(file_path) {
        Ok(sum) => println!("The sum is: {}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::fs::{remove_file, write};
    use std::io;

    #[test]
    fn test_sum_integers_from_file_valid() {
        let file_path = "test_valid.txt";
        write(file_path, "10\n20\n30\n").unwrap();
        assert_eq!(sum_integers_from_file(file_path).unwrap(), 60);
        remove_file(file_path).unwrap();
    }

    #[test]
    fn test_sum_integers_from_file_empty_file() {
        let file_path = "test_empty.txt";
        write(file_path, "").unwrap();
        assert_eq!(sum_integers_from_file(file_path).unwrap(), 0);
        remove_file(file_path).unwrap();
    }

    #[test]
    fn test_sum_integers_from_file_invalid_data() {
        let file_path = "test_invalid.txt";
        write(file_path, "10\nabc\n30\n").unwrap();
        let result = sum_integers_from_file(file_path);
        assert!(matches!(
            result,
            Err(ref e) if e.kind() == io::ErrorKind::InvalidData
        ));
        remove_file(file_path).unwrap();
    }

    #[test]
    fn test_sum_integers_from_file_file_error() {
        let file_path = "non_existent_file.txt";
        let result = sum_integers_from_file(file_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_sum_integers_from_file_negative_numbers() {
        let file_path = "test_negative.txt";
        write(file_path, "-10\n-20\n30\n").unwrap();
        assert_eq!(sum_integers_from_file(file_path).unwrap(), 0);
        remove_file(file_path).unwrap();
    }

    #[test]
    fn test_sum_integers_from_file_large_numbers() {
        let file_path = "test_large.txt";
        write(file_path, "1000000\n2000000\n").unwrap();
        assert_eq!(sum_integers_from_file(file_path).unwrap(), 3000000);
        remove_file(file_path).unwrap();
    }

    #[test]
    fn test_sum_integers_from_file_single_number() {
        let file_path = "test_single_number.txt";
        write(file_path, "42\n").unwrap();
        assert_eq!(sum_integers_from_file(file_path).unwrap(), 42);
        remove_file(file_path).unwrap();
    }
}
