//! # Rustfinity: The From Trait
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/the-from-trait
//!
//! The `From` trait is one of the most commonly used trait in the Rust programming language, **it is used for converting a value from one type to another.**
//!
//! After you implement the `From` trait for a type, the `Into` trait is implemented for you automatically. This means that you can then use `into()` on the source type to convert this type to the target type, and of course, you can use `From::from()` to convert from the source type to the target type.
//!
//! Here's an example on a use case for the `From` trait:
//!
//! ```rust
//! fn main() {
//!     let f = Fahrenheit(32.0);
//!     let c: Celsius = f.into();
//!     println!("{}°F is {}°C", f.0, c.0);
//! }
//! ```
//!
//! This code works because the `Fahrenheit` struct implements the `From` trait for the `Celsius` struct. This allows us to convert a `Fahrenheit` instance to a `Celsius` instance using the `into()` method. The explicit type annotation `: Celsius` tells the compiler to turn it "into" a `Celsius` instance.
//!
//! ## Your Task
//!
//! Your task is to implement the `From` trait for the following struct types:
//!
//! ```rust
//! struct Minutes(u32);
//! struct Hours(u32);
//! struct Days(u32);
//! ```
//!
//! You need to implement the `From` trait for the following conversions:
//!
//! - `Minutes` to `Hours`
//! - `Hours` to `Days`
//! - `Minutes` to `Days`
//! - `Days` to `Hours`
//!
//! Run: `cargo run --example rf_the_from_trait`
//! Test: `cargo test --example rf_the_from_trait`
//! Filter: `cargo test --example rf_the_from_trait <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

pub struct Minutes(pub i32);
pub struct Hours(pub i32);
pub struct Days(pub i32);

impl From<Minutes> for Hours {
    fn from(minutes: Minutes) -> Hours {
        // Implement the minute to hour conversion here
        unimplemented!()
    }
}

impl From<Hours> for Days {
    fn from(hours: Hours) -> Days {
        todo!("convert {} hours to days", hours.0)
    }
}

impl From<Minutes> for Days {
    fn from(minutes: Minutes) -> Days {
        todo!("convert {} minutes to days", minutes.0)
    }
}

impl From<Days> for Hours {
    fn from(days: Days) -> Hours {
        todo!("convert {} days to hours", days.0)
    }
}

// Example usage
pub fn main() {
    let minutes = Minutes(60);
    let hours: Hours = minutes.into();
    assert_eq!(hours.0, 1);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn should_convert_minutes_to_hours() {
        let minutes = Minutes(120);
        let hours: Hours = minutes.into();
        assert_eq!(hours.0, 2);
    }

    #[test]
    fn should_convert_hours_to_days() {
        let hours = Hours(48);
        let days: Days = hours.into();
        assert_eq!(days.0, 2);
    }

    #[test]
    fn should_convert_minutes_to_days() {
        let minutes = Minutes(2880);
        let days: Days = minutes.into();
        assert_eq!(days.0, 2);
    }

    #[test]
    fn should_convert_days_to_hours() {
        let days = Days(2);
        let hours: Hours = days.into();
        assert_eq!(hours.0, 48);
    }
}
