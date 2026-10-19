//! # Rustfinity: Weekday From Number
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/weekday-from-number
//!
//! In this challenge, you will use Rust's pattern matching capabilities to determine the name of a weekday given its corresponding number. **Each day of the week will be represented by a number from 1 to 7, where 1 corresponds to Monday and 7 corresponds to Sunday.**
//!
//! This is a common task that can be very useful in various applications, such as scheduling systems or calendar utilities. You will practice working with Rust's `match` statement to map numbers to their respective weekdays.
//!
//! ## Your task
//!
//! Your task is to implement a function, `weekday_from_number(day: u8) -> &'static str`, that takes a number as input and returns the corresponding weekday as a string. If the input number is not within the range of 1 to 7, the function should return "Invalid day number".
//!
//! ## Requirements
//!
//! - If the input number is 1, the function should return "Monday".
//! - If the input number is 2, the function should return "Tuesday".
//! - If the input number is 3, the function should return "Wednesday".
//! - If the input number is 4, the function should return "Thursday".
//! - If the input number is 5, the function should return "Friday".
//! - If the input number is 6, the function should return "Saturday".
//! - If the input number is 7, the function should return "Sunday".
//! - If the input number is not between 1 and 7, the function should return "Invalid day number".
//!
//! ## Example
//!
//! ```rust
//! let result = weekday_from_number(1);
//! assert_eq!(result, "Monday");
//!
//! let result = weekday_from_number(5);
//! assert_eq!(result, "Friday");
//!
//! let result = weekday_from_number(8);
//! assert_eq!(result, "Invalid day number");
//! ```
//!
//! ## Hints
//!
//! - Use the `match` statement to map each number to its corresponding weekday.
//! - Remember that Rust strings are UTF-8 encoded and `&'static str` denotes a string slice that lives for the entire duration of the program
//!
//! Run: `cargo run --example rf_weekday_from_number`
//! Test: `cargo test --example rf_weekday_from_number`
//! Filter: `cargo test --example rf_weekday_from_number <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub fn weekday_from_number(day: u8) -> &'static str {
    // TODO: Implement the function here
    unimplemented!()
}

fn main() {
    println!("Implement the Weekday From Number challenge.");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_weekday_from_number_valid() {
        assert_eq!(weekday_from_number(1), "Monday");
        assert_eq!(weekday_from_number(2), "Tuesday");
        assert_eq!(weekday_from_number(3), "Wednesday");
        assert_eq!(weekday_from_number(4), "Thursday");
        assert_eq!(weekday_from_number(5), "Friday");
        assert_eq!(weekday_from_number(6), "Saturday");
        assert_eq!(weekday_from_number(7), "Sunday");
    }

    #[test]
    fn test_weekday_from_number_invalid() {
        assert_eq!(weekday_from_number(0), "Invalid day number");
        assert_eq!(weekday_from_number(8), "Invalid day number");
    }
}
