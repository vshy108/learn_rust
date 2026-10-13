//! # Rustfinity: Converting Numerical Types
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/converting-numerical-types
//!
//! Since we have so many different numerical types in Rust (e.g., `i32`, `u32`, `f32`, etc.), it is common to need to **convert** between them.
//!
//! In Rust, converting between numerical types is often done using the `as` keyword. This challenge focuses on using `as` to **convert** an `i32` to a `u32`. While this conversion is straightforward, it is crucial to understand the implications and usage of the `as` keyword for safe and efficient type casting in Rust.
//!
//! ## The `as` keyword
//!
//! The `as` keyword in Rust is used for casting between different types. It is commonly used to **convert** between numerical types. The `as` keyword is used to **convert** a value from one type to another, as long as the conversion is valid and does not result in data loss or overflow.
//!
//! ## Your task
//!
//! Implement a function called `numerical_type_conversion` that takes an `i32` as input and returns it as a `u32`. **You should use the `as` keyword to perform this conversion.**
//!
//! ## Requirements
//!
//! - The function should take an `i32` as input and return a `u32`.
//! - You must use the `as` keyword to perform the type conversion.
//!
//! ## Example
//!
//! ```rust
//! let result = numerical_type_conversion(42i32);
//! assert_eq!(result, 42u32);
//! ```
//!
//! Run: `cargo run --example rf_converting_numerical_types`
//! Test: `cargo test --example rf_converting_numerical_types`
//! Filter: `cargo test --example rf_converting_numerical_types <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

pub fn numerical_type_conversion(n: i32) -> u32 {
    // Implement here
    unimplemented!()
}

fn main() {
    println!("Implement the Converting Numerical Types challenge.");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_converts_numbers() {
        assert_eq!(numerical_type_conversion(42i32), 42u32);
        assert_eq!(numerical_type_conversion(0i32), 0u32);
        assert_eq!(numerical_type_conversion(-1i32), 4294967295u32);
        assert_eq!(numerical_type_conversion(1_000_000i32), 1_000_000u32);
    }
}
