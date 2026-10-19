//! # Rustfinity: Mathematical Operations
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/mathematical-operations
//!
//! This challenge is about basic mathematical operations. You will be given 2 numbers `a` and `b`. You need to perform the following operations:
//!
//! 1. Sum of `a` and `b`
//! 2. Difference of `a` and `b`
//! 3. Multiplication of `a` and `b`
//! 4. Division of `a` and `b`
//!
//! You need to return a tuple containing the results of the above operations in the same order. `(sum, difference, multiply, divide)`
//!
//! > Note that every value in the tuple must be of type `i32`.
//!
//! ### Example
//!
//! ```rust
//! let a = 10;
//! let b = 5;
//! let result = math_operations(a, b);
//! assert_eq!(result, (15, 5, 50, 2));
//! ```
//!
//! ## Hints
//!
//! In Rust you can use the following operators for the above operations:
//!
//! 1. Sum: `+`
//! 2. Difference: `-`
//! 3. Multiplication: `*`
//! 4. Division: `/`
//!
//! **Good luck!**
//!
//! Run: `cargo run --example rf_mathematical_operations`
//! Test: `cargo test --example rf_mathematical_operations`
//! Filter: `cargo test --example rf_mathematical_operations <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub fn math_operations(a: i32, b: i32) -> (i32, i32, i32, i32) {
    // TODO: Return a tuple of 4 values: (sum, difference, multiply, divide)
    unimplemented!()
}

fn main() {
    println!("Implement the Mathematical Operations challenge.");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn math_tests() {
        let (sum, difference, multiply, divide) = math_operations(10, 5);

        assert_eq!(sum, 15);
        assert_eq!(difference, 5);
        assert_eq!(multiply, 50);
        assert_eq!(divide, 2);
    }

    #[test]
    fn with_large_numbers() {
        let (sum, difference, multiply, divide) = math_operations(1000, 5000);

        assert_eq!(sum, 6000);
        assert_eq!(difference, -4000);
        assert_eq!(multiply, 5000000);
        assert_eq!(divide, 0);
    }

    #[test]
    fn with_negative_numbers() {
        let (sum, difference, multiply, divide) = math_operations(-10, 5);

        assert_eq!(sum, -5);
        assert_eq!(difference, -15);
        assert_eq!(multiply, -50);
        assert_eq!(divide, -2);
    }

    #[test]
    fn with_mixed_numbers() {
        let (sum, difference, multiply, divide) = math_operations(-10, 5);

        assert_eq!(sum, -5);
        assert_eq!(difference, -15);
        assert_eq!(multiply, -50);
        assert_eq!(divide, -2);
    }
}
