//! # Rustfinity: Advanced Traits
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/advanced-traits
//!
//! You're already familiar with implementing traits in Rust, let's explore some special traits from the standard library - specifically those that allow operator overloading through the `std::ops` module.
//!
//! One such trait is `Add`, which allows you to implement custom behavior for the `+` operator. This is particularly useful when working with types that have a natural addition operation, like measurements or mathematical types.
//!
//! ## Your Task
//!
//! Let's implement custom addition between different units of measurement. You'll create a system where you can add meters to millimeters, with automatic unit conversion.
//!
//! Your job is to implement a `Millimeters` struct and a `Meters` struct, then implement the `Add` trait to allow adding a `Meters` value to a `Millimeters` value. The result should be a new `Millimeters` value.
//!
//! ### Requirements
//!
//! - Define two structs: `Millimeters` and `Meters`, each holding a single `u32` value
//! - Implement the `Add<Meters>` trait for `Millimeters` to handle addition with `Meters`
//! - Conversion rules:
//!   - 1 meter = 1000 millimeters
//!   - When adding, convert meters to millimeters first
//!   - Return the result as `Millimeters`
//!
//! ## Hints
//!
//! If you're stuck, here are some hints to help you solve the challenge!
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - The `Add` trait requires you to specify:
//!
//!   ```rust
//!   type Output = Millimeters;
//!   fn add(self, other: Meters) -> Self::Output;
//!   ```
//!
//! - Remember that meters must be **multiplied** by `1000` to convert to millimeters
//! - Access tuple struct fields with `.0`
//!
//! </details>
//!
//! Run: `cargo run --example rf_advanced_traits`
//! Test: `cargo test --example rf_advanced_traits`
//! Filter: `cargo test --example rf_advanced_traits <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub struct Millimeters(pub u32);
pub struct Meters(pub u32);

impl std::ops::Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Self::Output {
        todo!("add {:?} millimeters and {:?} meters", self.0, other.0)
    }
}

// Example usage
pub fn main() {
    let length1 = Millimeters(1500);
    let length2 = Meters(3);

    let result = length1 + length2;
    assert_eq!(result.0, 4500, "Expected 1500mm + 3000mm to equal 4500mm");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_add_millimeters_and_meters() {
        let mm = Millimeters(500);
        let m = Meters(2);

        let result = mm + m;
        assert_eq!(result.0, 2500, "Expected 500mm + 2000mm to equal 2500mm");
    }

    #[test]
    fn test_zero_addition() {
        let mm = Millimeters(0);
        let m = Meters(5);

        let result = mm + m;
        assert_eq!(result.0, 5000, "Expected 0mm + 5000mm to equal 5000mm");
    }

    #[test]
    fn test_large_numbers() {
        let mm = Millimeters(1_000_000);
        let m = Meters(2_000);

        let result = mm + m;
        assert_eq!(
            result.0, 3_000_000,
            "Expected 1_000_000mm + 2_000_000mm to equal 3_000_000mm"
        );
    }

    #[test]
    fn test_addition_with_zero_meters() {
        let mm = Millimeters(1000);
        let m = Meters(0);

        let result = mm + m;
        assert_eq!(result.0, 1000, "Expected 1000mm + 0mm to equal 1000mm");
    }

    #[test]
    fn test_addition_with_zero_millimeters() {
        let mm = Millimeters(0);
        let m = Meters(10);

        let result = mm + m;
        assert_eq!(result.0, 10000, "Expected 0mm + 10000mm to equal 10000mm");
    }

    #[test]
    fn test_addition_with_both_zero() {
        let mm = Millimeters(0);
        let m = Meters(0);

        let result = mm + m;
        assert_eq!(result.0, 0, "Expected 0mm + 0mm to equal 0mm");
    }

    #[test]
    fn test_addition_with_large_meters() {
        let mm = Millimeters(500);
        let m = Meters(1_000_000);

        let result = mm + m;
        assert_eq!(
            result.0, 1_000_000_500,
            "Expected 500mm + 1_000_000_000mm to equal 1_000_000_500mm"
        );
    }

    #[test]
    fn test_multiple_additions() {
        let mm = Millimeters(1000);
        let m1 = Meters(1);
        let m2 = Meters(2);

        let result = (mm + m1) + m2;
        assert_eq!(
            result.0, 4000,
            "Expected 1000mm + 1000mm + 2000mm to equal 4000mm"
        );
    }
}
