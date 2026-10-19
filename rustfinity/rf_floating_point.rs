//! # Rustfinity: Floating Point
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/floating-point
//!
//! # Floating Point Edge Cases
//!
//! Floating-point numbers in Rust (`f32` and `f64`) follow the IEEE 754 standard, which includes some surprising behaviors that can trip up even experienced programmers. Understanding these edge cases is crucial for writing robust numerical code.
//!
//! ## Special Values
//!
//! Floating-point types have special values that represent unusual mathematical concepts:
//!
//! - **NaN (Not a Number)**: Result of invalid operations like `0.0/0.0` or `(-1.0_f64).sqrt()`. NaN is unique because it's not equal to anything, including itself!
//! - **Infinity**: Result of overflow or dividing by zero (e.g., `1.0/0.0` gives positive infinity)
//! - **Negative Zero**: `-0.0` exists and equals `0.0`, but can be distinguished
//!
//! ```rust
//! let nan = f64::NAN;
//! assert!(nan != nan);  // NaN is never equal to itself!
//! assert!(nan.is_nan());
//!
//! let inf = f64::INFINITY;
//! assert!(inf.is_infinite());
//! assert!(inf > f64::MAX);
//!
//! // Negative zero equals positive zero
//! assert_eq!(0.0, -0.0);
//! ```
//!
//! ## Checking for Special Values
//!
//! The standard library provides methods to detect these special values:
//!
//! ```rust
//! let x = 1.0_f64;
//! assert!(x.is_finite());      // Not NaN, not infinite
//! assert!(!x.is_nan());
//! assert!(!x.is_infinite());
//! // Not zero, subnormal, infinite, or NaN
//! assert!(x.is_normal());
//!
//! let zero = 0.0_f64;
//! assert!(!zero.is_normal());  // Zero is not "normal"
//! assert!(zero.is_finite());   // But it is finite
//! ```
//!
//! ## Rounding and Truncation
//!
//! Rust provides several methods for converting floats to integers or rounding:
//!
//! ```rust
//! let x = 3.7_f64;
//! // Round toward negative infinity
//! assert_eq!(x.floor(), 3.0);
//! // Round toward positive infinity
//! assert_eq!(x.ceil(), 4.0);
//! // Round to nearest, ties away from zero
//! assert_eq!(x.round(), 4.0);
//! assert_eq!(x.trunc(), 3.0);   // Round toward zero
//!
//! let y = -3.7_f64;
//! assert_eq!(y.floor(), -4.0);  // -4 is "more negative"
//! assert_eq!(y.trunc(), -3.0);  // Toward zero is -3
//! ```
//!
//! ## Approximate Comparisons
//!
//! Due to floating-point precision limits, direct equality comparison is often unreliable:
//!
//! ```rust
//! let a = 0.1 + 0.2;
//! let b = 0.3;
//! assert!(a != b);  // Surprise! They're not exactly equal
//! assert!((a - b).abs() < 1e-10);  // But they're very close
//! ```
//!
//! ## Your Task
//!
//! Implement the following functions to handle floating-point edge cases:
//!
//! 1. **`is_valid_number(x: f64) -> bool`**
//!    - Returns `true` if `x` is a finite, non-NaN number
//!      (i.e., a "normal" number you can do math with).
//!
//! 2. **`classify_float(x: f64) -> &'static str`**
//!    - Returns a string describing the float: `"nan"`,
//!      `"infinite"`, `"zero"`, `"normal"`, or `"subnormal"`.
//!
//! 3. **`safe_divide(a: f64, b: f64) -> Option<f64>`**
//!    - Divides `a` by `b`, returning `None` if `b` is zero
//!      or if either input is NaN/infinite.
//!
//! 4. **`round_to_places(x: f64, places: u32) -> f64`**
//!    - Rounds `x` to the specified number of decimal places.
//!
//! 5. **`approx_equal(a: f64, b: f64, epsilon: f64) -> bool`**
//!    - Returns `true` if `a` and `b` are within `epsilon`
//!      of each other. Returns `false` if either is NaN.
//!
//! 6. **`clamp_to_range(x: f64, min: f64, max: f64) -> Option<f64>`**
//!    - Clamps `x` to the range `[min, max]`. Returns `None`
//!      if any input is NaN or if `min > max`.
//!
//! 7. **`safe_sqrt(x: f64) -> Option<f64>`** - Returns the square root of `x`, or `None` if `x` is negative or NaN.
//!
//! 8. **`sum_finite(numbers: &[f64]) -> f64`** - Sums all finite numbers in the slice, skipping any NaN or infinite values.
//!
//! ## Examples
//!
//! ```rust
//! // Validation
//! assert!(is_valid_number(42.0));
//! assert!(!is_valid_number(f64::NAN));
//! assert!(!is_valid_number(f64::INFINITY));
//!
//! // Classification
//! assert_eq!(classify_float(f64::NAN), "nan");
//! assert_eq!(classify_float(0.0), "zero");
//! assert_eq!(classify_float(1.5), "normal");
//!
//! // Safe division
//! assert_eq!(safe_divide(10.0, 2.0), Some(5.0));
//! assert_eq!(safe_divide(10.0, 0.0), None);
//! assert_eq!(safe_divide(f64::NAN, 2.0), None);
//!
//! // Rounding
//! assert_eq!(round_to_places(3.14159, 2), 3.14);
//! assert_eq!(round_to_places(2.5, 0), 3.0);
//!
//! // Approximate equality
//! assert!(approx_equal(0.1 + 0.2, 0.3, 1e-10));
//! assert!(!approx_equal(1.0, 2.0, 0.5));
//!
//! // Clamping
//! assert_eq!(clamp_to_range(5.0, 0.0, 10.0), Some(5.0));
//! assert_eq!(clamp_to_range(-5.0, 0.0, 10.0), Some(0.0));
//! assert_eq!(clamp_to_range(f64::NAN, 0.0, 10.0), None);
//!
//! // Safe sqrt
//! assert_eq!(safe_sqrt(4.0), Some(2.0));
//! assert_eq!(safe_sqrt(-1.0), None);
//!
//! // Sum finite values
//! assert_eq!(
//!     sum_finite(&[1.0, f64::NAN, 2.0, f64::INFINITY, 3.0]),
//!     6.0
//! );
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here to reveal hints</summary>
//!
//! - Use `x.is_finite()` to check if a number is neither NaN nor infinite.
//! - Use `x.is_nan()` specifically to check for NaN values.
//! - The `x.classify()` method returns an `FpCategory` enum with variants `Nan`, `Infinite`, `Zero`, `Subnormal`, and `Normal`.
//! - For `round_to_places`, multiply by 10^places, round, then divide by 10^places.
//! - For `approx_equal`, compute `(a - b).abs()` and compare to epsilon. Remember that any comparison with NaN returns false.
//! - The `x.clamp(min, max)` method exists but doesn't handle NaN the way we want - you'll need custom logic.
//! - For `safe_sqrt`, check both that `x >= 0.0` and that `x` is not NaN.
//! - Use `filter` with `is_finite()` before summing in `sum_finite`.
//!
//! </details>
//!
//! Run: `cargo run --example rf_floating_point`
//! Test: `cargo test --example rf_floating_point`
//! Filter: `cargo test --example rf_floating_point <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

/// Checks if a number is a valid, finite number (not NaN or infinite).
///
/// Returns `true` if `x` is a number you can safely do math with.
pub fn is_valid_number(x: f64) -> bool {
    // TODO
    unimplemented!()
}

/// Classifies a floating-point number into categories.
///
/// Returns one of: `"nan"`, `"infinite"`, `"zero"`, `"normal"`, or `"subnormal"`.
pub fn classify_float(x: f64) -> &'static str {
    // TODO
    unimplemented!()
}

/// Safely divides two numbers, handling edge cases.
///
/// Returns `None` if:
/// - `b` is zero
/// - Either `a` or `b` is NaN
/// - Either `a` or `b` is infinite
pub fn safe_divide(a: f64, b: f64) -> Option<f64> {
    // TODO
    unimplemented!()
}

/// Rounds a number to the specified number of decimal places.
pub fn round_to_places(x: f64, places: u32) -> f64 {
    // TODO
    unimplemented!()
}

/// Checks if two numbers are approximately equal within a tolerance.
///
/// Returns `false` if either number is NaN (since NaN comparisons are always false).
pub fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
    // TODO
    unimplemented!()
}

/// Clamps a number to a range, returning `None` for invalid inputs.
///
/// Returns `None` if:
/// - Any of the inputs is NaN
/// - `min > max`
pub fn clamp_to_range(x: f64, min: f64, max: f64) -> Option<f64> {
    // TODO
    unimplemented!()
}

/// Safely computes the square root, returning `None` for invalid inputs.
///
/// Returns `None` if:
/// - `x` is negative
/// - `x` is NaN
pub fn safe_sqrt(x: f64) -> Option<f64> {
    // TODO
    unimplemented!()
}

/// Sums all finite numbers in a slice, skipping NaN and infinite values.
///
/// Returns 0.0 for an empty slice.
pub fn sum_finite(numbers: &[f64]) -> f64 {
    // TODO
    unimplemented!()
}

// Example usage
pub fn main() {
    // Test is_valid_number
    println!("42.0 is valid: {}", is_valid_number(42.0));
    println!("NaN is valid: {}", is_valid_number(f64::NAN));

    // Test classify_float
    println!("Classify 1.0: {}", classify_float(1.0));
    println!("Classify NaN: {}", classify_float(f64::NAN));

    // Test safe_divide
    println!("10 / 2 = {:?}", safe_divide(10.0, 2.0));
    println!("10 / 0 = {:?}", safe_divide(10.0, 0.0));

    // Test round_to_places
    println!(
        "3.14159 rounded to 2 places: {}",
        round_to_places(3.14159, 2)
    );

    // Test approx_equal
    println!("0.1 + 0.2 ≈ 0.3: {}", approx_equal(0.1 + 0.2, 0.3, 1e-10));

    // Test clamp_to_range
    println!(
        "15.0 clamped to [0, 10]: {:?}",
        clamp_to_range(15.0, 0.0, 10.0)
    );

    // Test safe_sqrt
    println!("sqrt(4) = {:?}", safe_sqrt(4.0));
    println!("sqrt(-1) = {:?}", safe_sqrt(-1.0));

    // Test sum_finite
    let numbers = vec![1.0, f64::NAN, 2.0, f64::INFINITY, 3.0];
    println!("sum_finite({:?}) = {}", numbers, sum_finite(&numbers));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    // ==================== is_valid_number tests ====================

    #[test]
    fn is_valid_number_positive() {
        assert!(is_valid_number(42.0));
        assert!(is_valid_number(3.14159));
        assert!(is_valid_number(1e10));
    }

    #[test]
    fn is_valid_number_negative() {
        assert!(is_valid_number(-42.0));
        assert!(is_valid_number(-3.14159));
        assert!(is_valid_number(-1e10));
    }

    #[test]
    fn is_valid_number_zero() {
        assert!(is_valid_number(0.0));
        assert!(is_valid_number(-0.0));
    }

    #[test]
    fn is_valid_number_nan() {
        assert!(!is_valid_number(f64::NAN));
        assert!(!is_valid_number(0.0 / 0.0));
    }

    #[test]
    fn is_valid_number_infinity() {
        assert!(!is_valid_number(f64::INFINITY));
        assert!(!is_valid_number(f64::NEG_INFINITY));
        assert!(!is_valid_number(1.0 / 0.0));
    }

    #[test]
    fn is_valid_number_subnormal() {
        // Subnormal numbers are still valid finite numbers
        assert!(is_valid_number(f64::MIN_POSITIVE / 2.0));
    }

    // ==================== classify_float tests ====================

    #[test]
    fn classify_float_nan() {
        assert_eq!(classify_float(f64::NAN), "nan");
        assert_eq!(classify_float(0.0 / 0.0), "nan");
    }

    #[test]
    fn classify_float_infinite() {
        assert_eq!(classify_float(f64::INFINITY), "infinite");
        assert_eq!(classify_float(f64::NEG_INFINITY), "infinite");
        assert_eq!(classify_float(1.0 / 0.0), "infinite");
    }

    #[test]
    fn classify_float_zero() {
        assert_eq!(classify_float(0.0), "zero");
        assert_eq!(classify_float(-0.0), "zero");
    }

    #[test]
    fn classify_float_normal() {
        assert_eq!(classify_float(1.0), "normal");
        assert_eq!(classify_float(-42.5), "normal");
        assert_eq!(classify_float(f64::MAX), "normal");
        assert_eq!(classify_float(f64::MIN), "normal");
    }

    #[test]
    fn classify_float_subnormal() {
        // Subnormal (denormalized) numbers are very small
        let subnormal = f64::MIN_POSITIVE / 2.0;
        assert_eq!(classify_float(subnormal), "subnormal");
    }

    // ==================== safe_divide tests ====================

    #[test]
    fn safe_divide_basic() {
        assert_eq!(safe_divide(10.0, 2.0), Some(5.0));
        assert_eq!(safe_divide(7.0, 2.0), Some(3.5));
        assert_eq!(safe_divide(-15.0, 3.0), Some(-5.0));
    }

    #[test]
    fn safe_divide_by_zero() {
        assert_eq!(safe_divide(10.0, 0.0), None);
        assert_eq!(safe_divide(-10.0, 0.0), None);
        assert_eq!(safe_divide(0.0, 0.0), None);
    }

    #[test]
    fn safe_divide_with_nan() {
        assert_eq!(safe_divide(f64::NAN, 2.0), None);
        assert_eq!(safe_divide(10.0, f64::NAN), None);
        assert_eq!(safe_divide(f64::NAN, f64::NAN), None);
    }

    #[test]
    fn safe_divide_with_infinity() {
        assert_eq!(safe_divide(f64::INFINITY, 2.0), None);
        assert_eq!(safe_divide(10.0, f64::INFINITY), None);
        assert_eq!(safe_divide(f64::NEG_INFINITY, 2.0), None);
    }

    #[test]
    fn safe_divide_zero_numerator() {
        assert_eq!(safe_divide(0.0, 5.0), Some(0.0));
        assert_eq!(safe_divide(0.0, -5.0), Some(-0.0));
    }

    #[test]
    fn safe_divide_small_numbers() {
        let result = safe_divide(1e-300, 1e-10);
        assert!(result.is_some());
        assert!(result.unwrap() < 1e-289);
    }

    // ==================== round_to_places tests ====================

    #[test]
    fn round_to_places_basic() {
        assert_eq!(round_to_places(3.14159, 2), 3.14);
        assert_eq!(round_to_places(3.14159, 4), 3.1416);
        assert_eq!(round_to_places(3.14159, 0), 3.0);
    }

    #[test]
    fn round_to_places_negative() {
        assert_eq!(round_to_places(-3.14159, 2), -3.14);
        assert_eq!(round_to_places(-3.14159, 0), -3.0);
    }

    #[test]
    fn round_to_places_half_up() {
        assert_eq!(round_to_places(2.5, 0), 3.0);
        assert_eq!(round_to_places(2.55, 1), 2.6);
        assert_eq!(round_to_places(-2.5, 0), -3.0);
    }

    #[test]
    fn round_to_places_zero() {
        assert_eq!(round_to_places(0.0, 2), 0.0);
        assert_eq!(round_to_places(0.001, 2), 0.0);
    }

    #[test]
    fn round_to_places_whole_numbers() {
        assert_eq!(round_to_places(42.0, 2), 42.0);
        assert_eq!(round_to_places(100.0, 0), 100.0);
    }

    #[test]
    fn round_to_places_many_places() {
        let result = round_to_places(1.123456789, 6);
        assert!((result - 1.123457).abs() < 1e-10);
    }

    // ==================== approx_equal tests ====================

    #[test]
    fn approx_equal_exact() {
        assert!(approx_equal(1.0, 1.0, 0.0));
        assert!(approx_equal(0.0, 0.0, 0.0));
    }

    #[test]
    fn approx_equal_within_epsilon() {
        assert!(approx_equal(1.0, 1.0001, 0.001));
        assert!(approx_equal(1.0, 1.1, 0.2));
        assert!(approx_equal(0.1 + 0.2, 0.3, 1e-10));
    }

    #[test]
    fn approx_equal_outside_epsilon() {
        assert!(!approx_equal(1.0, 2.0, 0.5));
        assert!(!approx_equal(1.0, 1.1, 0.05));
    }

    #[test]
    fn approx_equal_with_nan() {
        assert!(!approx_equal(f64::NAN, f64::NAN, 1.0));
        assert!(!approx_equal(f64::NAN, 1.0, 1.0));
        assert!(!approx_equal(1.0, f64::NAN, 1.0));
    }

    #[test]
    fn approx_equal_negative() {
        assert!(approx_equal(-1.0, -1.0001, 0.001));
        assert!(approx_equal(-5.0, -5.5, 1.0));
    }

    #[test]
    fn approx_equal_across_zero() {
        assert!(approx_equal(-0.5, 0.5, 1.0));
        assert!(!approx_equal(-0.5, 0.5, 0.5));
    }

    #[test]
    fn approx_equal_infinity() {
        // Infinity - infinity is NaN, so comparison should fail
        assert!(!approx_equal(f64::INFINITY, f64::INFINITY, 1.0));
    }

    // ==================== clamp_to_range tests ====================

    #[test]
    fn clamp_to_range_within() {
        assert_eq!(clamp_to_range(5.0, 0.0, 10.0), Some(5.0));
        assert_eq!(clamp_to_range(0.5, 0.0, 1.0), Some(0.5));
    }

    #[test]
    fn clamp_to_range_at_boundary() {
        assert_eq!(clamp_to_range(0.0, 0.0, 10.0), Some(0.0));
        assert_eq!(clamp_to_range(10.0, 0.0, 10.0), Some(10.0));
    }

    #[test]
    fn clamp_to_range_below() {
        assert_eq!(clamp_to_range(-5.0, 0.0, 10.0), Some(0.0));
        assert_eq!(clamp_to_range(-100.0, -50.0, 50.0), Some(-50.0));
    }

    #[test]
    fn clamp_to_range_above() {
        assert_eq!(clamp_to_range(15.0, 0.0, 10.0), Some(10.0));
        assert_eq!(clamp_to_range(100.0, -50.0, 50.0), Some(50.0));
    }

    #[test]
    fn clamp_to_range_nan_input() {
        assert_eq!(clamp_to_range(f64::NAN, 0.0, 10.0), None);
    }

    #[test]
    fn clamp_to_range_nan_min() {
        assert_eq!(clamp_to_range(5.0, f64::NAN, 10.0), None);
    }

    #[test]
    fn clamp_to_range_nan_max() {
        assert_eq!(clamp_to_range(5.0, 0.0, f64::NAN), None);
    }

    #[test]
    fn clamp_to_range_inverted() {
        assert_eq!(clamp_to_range(5.0, 10.0, 0.0), None);
    }

    #[test]
    fn clamp_to_range_equal_bounds() {
        assert_eq!(clamp_to_range(5.0, 5.0, 5.0), Some(5.0));
        assert_eq!(clamp_to_range(3.0, 5.0, 5.0), Some(5.0));
        assert_eq!(clamp_to_range(7.0, 5.0, 5.0), Some(5.0));
    }

    #[test]
    fn clamp_to_range_negative_range() {
        assert_eq!(clamp_to_range(-5.0, -10.0, -1.0), Some(-5.0));
        assert_eq!(clamp_to_range(0.0, -10.0, -1.0), Some(-1.0));
    }

    // ==================== safe_sqrt tests ====================

    #[test]
    fn safe_sqrt_positive() {
        assert_eq!(safe_sqrt(4.0), Some(2.0));
        assert_eq!(safe_sqrt(9.0), Some(3.0));
        assert_eq!(safe_sqrt(2.0), Some(2.0_f64.sqrt()));
    }

    #[test]
    fn safe_sqrt_zero() {
        assert_eq!(safe_sqrt(0.0), Some(0.0));
    }

    #[test]
    fn safe_sqrt_negative() {
        assert_eq!(safe_sqrt(-1.0), None);
        assert_eq!(safe_sqrt(-100.0), None);
        assert_eq!(safe_sqrt(-0.001), None);
    }

    #[test]
    fn safe_sqrt_nan() {
        assert_eq!(safe_sqrt(f64::NAN), None);
    }

    #[test]
    fn safe_sqrt_infinity() {
        // Sqrt of infinity is infinity, which is valid
        let result = safe_sqrt(f64::INFINITY);
        assert!(result.is_some());
        assert!(result.unwrap().is_infinite());
    }

    #[test]
    fn safe_sqrt_small_positive() {
        let result = safe_sqrt(1e-100);
        assert!(result.is_some());
        assert!((result.unwrap() - 1e-50).abs() < 1e-60);
    }

    #[test]
    fn safe_sqrt_perfect_squares() {
        assert_eq!(safe_sqrt(1.0), Some(1.0));
        assert_eq!(safe_sqrt(16.0), Some(4.0));
        assert_eq!(safe_sqrt(100.0), Some(10.0));
    }

    // ==================== sum_finite tests ====================

    #[test]
    fn sum_finite_all_valid() {
        assert_eq!(sum_finite(&[1.0, 2.0, 3.0]), 6.0);
        assert_eq!(sum_finite(&[10.0, 20.0, 30.0, 40.0]), 100.0);
    }

    #[test]
    fn sum_finite_with_nan() {
        assert_eq!(sum_finite(&[1.0, f64::NAN, 2.0]), 3.0);
        assert_eq!(sum_finite(&[f64::NAN, f64::NAN, f64::NAN]), 0.0);
    }

    #[test]
    fn sum_finite_with_infinity() {
        assert_eq!(sum_finite(&[1.0, f64::INFINITY, 2.0]), 3.0);
        assert_eq!(sum_finite(&[1.0, f64::NEG_INFINITY, 2.0]), 3.0);
    }

    #[test]
    fn sum_finite_mixed_special() {
        assert_eq!(sum_finite(&[1.0, f64::NAN, 2.0, f64::INFINITY, 3.0]), 6.0);
    }

    #[test]
    fn sum_finite_empty() {
        assert_eq!(sum_finite(&[]), 0.0);
    }

    #[test]
    fn sum_finite_all_special() {
        assert_eq!(
            sum_finite(&[f64::NAN, f64::INFINITY, f64::NEG_INFINITY]),
            0.0
        );
    }

    #[test]
    fn sum_finite_negative() {
        assert_eq!(sum_finite(&[-1.0, -2.0, -3.0]), -6.0);
        assert_eq!(sum_finite(&[1.0, -2.0, 3.0, -4.0]), -2.0);
    }

    #[test]
    fn sum_finite_with_zero() {
        assert_eq!(sum_finite(&[0.0, 1.0, 2.0]), 3.0);
        assert_eq!(sum_finite(&[0.0, 0.0, 0.0]), 0.0);
    }

    // ==================== Integration tests ====================

    #[test]
    fn integration_validate_then_operate() {
        let values = vec![1.0, 2.0, f64::NAN, 4.0, f64::INFINITY];

        // Filter valid numbers and compute their sum
        let sum: f64 = values.iter().filter(|&&x| is_valid_number(x)).sum();
        assert_eq!(sum, 7.0);

        // Same result as sum_finite
        assert_eq!(sum_finite(&values), sum);
    }

    #[test]
    fn integration_classify_and_count() {
        let values = vec![
            0.0,
            1.0,
            -1.0,
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::MIN_POSITIVE / 2.0,
        ];

        let nan_count = values
            .iter()
            .filter(|&&x| classify_float(x) == "nan")
            .count();
        let infinite_count = values
            .iter()
            .filter(|&&x| classify_float(x) == "infinite")
            .count();
        let zero_count = values
            .iter()
            .filter(|&&x| classify_float(x) == "zero")
            .count();

        assert_eq!(nan_count, 1);
        assert_eq!(infinite_count, 2);
        assert_eq!(zero_count, 1);
    }

    #[test]
    fn integration_safe_sqrt_and_round() {
        // Compute sqrt and round to 2 places
        let values = vec![4.0, 2.0, 9.0, -1.0];
        let results: Vec<Option<f64>> = values
            .iter()
            .map(|&x| safe_sqrt(x).map(|r| round_to_places(r, 2)))
            .collect();

        assert_eq!(results[0], Some(2.0));
        assert_eq!(results[1], Some(1.41));
        assert_eq!(results[2], Some(3.0));
        assert_eq!(results[3], None);
    }

    #[test]
    fn integration_clamp_and_approx() {
        // Clamp a value and check if it's approximately equal to target
        let clamped = clamp_to_range(15.0, 0.0, 10.0).unwrap();
        assert!(approx_equal(clamped, 10.0, 0.001));

        let clamped2 = clamp_to_range(-5.0, 0.0, 10.0).unwrap();
        assert!(approx_equal(clamped2, 0.0, 0.001));
    }

    #[test]
    fn integration_safe_divide_chain() {
        // Chain of divisions
        let a = 100.0;
        let b = 4.0;
        let c = 5.0;

        let result = safe_divide(a, b).and_then(|r| safe_divide(r, c));

        assert_eq!(result, Some(5.0));

        // Chain with zero
        let result2 = safe_divide(a, b).and_then(|r| safe_divide(r, 0.0));

        assert_eq!(result2, None);
    }

    #[test]
    fn integration_real_world_average() {
        // Calculate average of valid numbers
        let data = vec![10.0, 20.0, f64::NAN, 30.0, f64::INFINITY, 40.0];
        let valid_data: Vec<f64> = data
            .iter()
            .filter(|&&x| is_valid_number(x))
            .copied()
            .collect();

        if !valid_data.is_empty() {
            let sum = sum_finite(&valid_data);
            let avg = sum / valid_data.len() as f64;
            assert!(approx_equal(avg, 25.0, 0.001));
        }
    }

    #[test]
    fn integration_floating_point_comparison_pitfall() {
        // The classic 0.1 + 0.2 != 0.3 problem
        let a = 0.1 + 0.2;
        let b = 0.3;

        // Direct comparison fails
        assert!(a != b);

        // But they're approximately equal
        assert!(approx_equal(a, b, 1e-15));
    }

    #[test]
    fn integration_normalize_and_sum() {
        // Normalize values to [0, 1] range and sum
        let values = vec![25.0, 50.0, 75.0, 100.0];
        let min = 0.0;
        let max = 100.0;

        let normalized: Vec<f64> = values
            .iter()
            .filter_map(|&x| clamp_to_range(x, min, max))
            .map(|x| (x - min) / (max - min))
            .collect();

        let sum = sum_finite(&normalized);
        assert!(approx_equal(sum, 2.5, 0.001));
    }

    #[test]
    fn integration_error_propagation() {
        // Test that errors propagate correctly through Option chain
        let compute = |x: f64| -> Option<f64> {
            if !is_valid_number(x) {
                return None;
            }
            let clamped = clamp_to_range(x, 0.0, 100.0)?;
            let sqrt = safe_sqrt(clamped)?;
            Some(round_to_places(sqrt, 2))
        };

        assert_eq!(compute(49.0), Some(7.0));
        assert_eq!(compute(f64::NAN), None);
        assert_eq!(compute(-10.0), Some(0.0)); // Clamped to 0, sqrt(0) = 0
    }
}
