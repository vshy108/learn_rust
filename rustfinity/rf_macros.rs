//! # Rustfinity: Macros
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/macros
//!
//! Macros in Rust are one of its most powerful features, enabling meta-programming and reducing repetitive code. A macro allows you to generate and include code fragments during compilation, making it an excellent tool for tasks like boilerplate elimination.
//!
//! In this challenge, you'll create a macro that formats mathematical operations as human-readable strings. The macro should perform the calculation and present it in a clear, formatted way.
//!
//! ## Your Task
//!
//! In this challenge, you will implement a macro named `math_operations!` that:
//!
//! 1. Takes three arguments: two integers and an operator
//! 2. Performs the mathematical operation
//! 3. Returns a formatted string showing the operation and its result
//!
//! ### Requirements
//!
//! 1. The macro should format the output as: `"{number} {operator} {number} = {result}"`
//!
//! 2. Input Types:
//!    - The macro expects integer operands only (no floating-point numbers)
//!    - Both signed and unsigned integers are supported
//!    - Results will also be **integers** (division rounds towards zero)
//!
//! 3. Supported Operations:
//!    - Addition (`+`)
//!    - Subtraction (`-`)
//!    - Multiplication (`*`)
//!    - Division (`/`)
//!
//! 4. Error Handling:
//!    - Division by zero should panic with message: `"Division by zero"`
//!    - Invalid operators should panic with message: `"Unsupported operator: {operator}"`
//!
//! ## Hints
//!
//! If you're stuck, check out the hints below.
//!
//! <details>
//!   <summary>Click here to reveal hints</summary>
//!
//! - Use pattern matching to handle different operators
//! - Remember to check for division by zero before performing division
//! - The `format!` macro is useful for creating the output string
//! - Use `expr` matchers in your macro for maximum flexibility, e.g.
//!
//!   ```rust
//!   macro_rules! math_operations {
//!     ($a:expr, $op:expr, $b:expr) => {{
//!         // Your code here
//!     }};
//!   }
//!   ```
//!
//! </details>
//!
//! Run: `cargo run --example rf_macros`
//! Test: `cargo test --example rf_macros`
//! Filter: `cargo test --example rf_macros <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

#[macro_export]
macro_rules! math_operations {
    ($left:expr, $operator:tt, $right:expr) => {
        String::new()
    };
}

// Example usage
pub fn main() {
    assert_eq!(math_operations!(4, "+", 2), "4 + 2 = 6");
    assert_eq!(math_operations!(10, "-", 3), "10 - 3 = 7");
    assert_eq!(math_operations!(6, "*", 4), "6 * 4 = 24");
    assert_eq!(math_operations!(15, "/", 3), "15 / 3 = 5");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    // Basic operations
    #[test]
    fn test_addition_format() {
        assert_eq!(
            math_operations!(4, "+", 2),
            "4 + 2 = 6",
            "Addition operation failed to format correctly"
        );
    }

    #[test]
    fn test_subtraction_format() {
        assert_eq!(
            math_operations!(4, "-", 2),
            "4 - 2 = 2",
            "Subtraction operation failed to format correctly"
        );
    }

    #[test]
    fn test_multiplication_format() {
        assert_eq!(
            math_operations!(4, "*", 2),
            "4 * 2 = 8",
            "Multiplication operation failed to format correctly"
        );
    }

    #[test]
    fn test_division_format() {
        assert_eq!(
            math_operations!(4, "/", 2),
            "4 / 2 = 2",
            "Division operation failed to format correctly"
        );
    }

    // Zero operations
    #[test]
    fn test_add_zero() {
        assert_eq!(
            math_operations!(5, "+", 0),
            "5 + 0 = 5",
            "Adding zero failed to format correctly"
        );
    }

    #[test]
    fn test_subtract_zero() {
        assert_eq!(
            math_operations!(5, "-", 0),
            "5 - 0 = 5",
            "Subtracting zero failed to format correctly"
        );
    }

    #[test]
    fn test_multiply_by_zero() {
        assert_eq!(
            math_operations!(5, "*", 0),
            "5 * 0 = 0",
            "Multiplication by zero failed to format correctly"
        );
    }

    // Negative numbers
    #[test]
    fn test_negative_addition() {
        assert_eq!(
            math_operations!(-4, "+", 2),
            "-4 + 2 = -2",
            "Addition with negative number failed to format correctly"
        );
    }

    #[test]
    fn test_negative_multiplication() {
        assert_eq!(
            math_operations!(-3, "*", -2),
            "-3 * -2 = 6",
            "Multiplication with negative numbers failed to format correctly"
        );
    }

    // Larger numbers
    #[test]
    fn test_large_numbers() {
        assert_eq!(
            math_operations!(1000, "*", 2000),
            "1000 * 2000 = 2000000",
            "Large number multiplication failed to format correctly"
        );
    }

    // Same number operations
    #[test]
    fn test_same_number_operations() {
        assert_eq!(
            math_operations!(5, "+", 5),
            "5 + 5 = 10",
            "Addition with same numbers failed to format correctly"
        );
        assert_eq!(
            math_operations!(5, "-", 5),
            "5 - 5 = 0",
            "Subtraction with same numbers failed to format correctly"
        );
        assert_eq!(
            math_operations!(5, "*", 5),
            "5 * 5 = 25",
            "Multiplication with same numbers failed to format correctly"
        );
        assert_eq!(
            math_operations!(5, "/", 5),
            "5 / 5 = 1",
            "Division with same numbers failed to format correctly"
        );
    }

    // Error cases
    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_division_by_zero() {
        let _ = math_operations!(4, "/", 0);
    }

    #[test]
    #[should_panic(expected = "Unsupported operator")]
    fn test_invalid_operator() {
        let _ = math_operations!(4, "%", 2);
    }
}
