// Demonstrates division operators and edge cases in Rust.
//
// Rust has different division behavior for integers vs floats:
//   - Integer division truncates toward zero (like Python's `//` but NOT exactly)
//   - Float division follows IEEE 754
//
// Run: cargo run --example s03_division

fn main() {
    // --- 1. Integer division (truncates toward zero) ---
    println!("Integer division:");
    println!("  7 / 2   = {}", 7_i32 / 2); // 3  (not 3.5)
    println!("  -7 / 2  = {}", -7_i32 / 2); // -3 (truncates toward zero)
    println!("  7 / -2  = {}", 7_i32 / -2); // -3

    // Gotcha: Python's `//` rounds toward negative infinity:
    //   Python: -7 // 2 = -4
    //   Rust:   -7 / 2  = -3
    // This difference catches many Python developers!

    // --- 2. Integer remainder (%) ---
    // The sign of the result matches the DIVIDEND (left operand).
    println!();
    println!("Integer remainder:");
    println!("  7 % 2   = {}", 7_i32 % 2); // 1
    println!("  -7 % 2  = {}", -7_i32 % 2); // -1  (sign follows -7)
    println!("  7 % -2  = {}", 7_i32 % -2); // 1   (sign follows 7)

    // Python's `%` sign follows the DIVISOR:
    //   Python: -7 % 2 = 1
    //   Rust:   -7 % 2 = -1

    // --- 3. Division by zero ---
    // Integer division by zero PANICS at runtime (no undefined behavior).
    // let _boom = 1 / 0;  // thread 'main' panicked at 'attempt to divide by zero'
    println!();
    println!("Integer /0: panics at runtime (no UB like C)");

    // --- 4. Euclidean division and remainder ---
    // For Python-like behavior, use `.div_euclid()` and `.rem_euclid()`:
    println!();
    println!("Euclidean (Python-like):");
    println!("  (-7).div_euclid(2)  = {}", (-7_i32).div_euclid(2)); // -4
    println!("  (-7).rem_euclid(2)  = {}", (-7_i32).rem_euclid(2)); // 1
    // rem_euclid always returns a non-negative result.

    // --- 5. Float division --- (covered more in s03_float_division)
    println!();
    println!("Float division:");
    println!("  7.0 / 2.0  = {}", 7.0_f64 / 2.0); // 3.5
    println!("  -7.0 / 2.0 = {}", -7.0_f64 / 2.0); // -3.5

    // Float /0 does NOT panic — returns infinity or NaN
    println!("  1.0 / 0.0  = {}", 1.0_f64 / 0.0); // inf
    println!("  -1.0 / 0.0 = {}", -1.0_f64 / 0.0); // -inf
    println!("  0.0 / 0.0  = {}", 0.0_f64 / 0.0); // NaN

    // --- 6. Overflow in division ---
    // Only one case overflows: MIN / -1 for signed integers.
    // i32::MIN / -1 would be i32::MAX + 1, which doesn't fit.
    // In debug mode this panics; in release it wraps.
    println!();
    println!("Overflow edge case:");
    let (result, overflowed) = i32::MIN.overflowing_div(-1);
    println!("  i32::MIN.overflowing_div(-1) = ({result}, {overflowed})");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_integer_division_truncates_toward_zero() {
        assert_eq!(7 / 2, 3);
        assert_eq!(-7 / 2, -3); // NOT -4 like Python
    }

    #[test]
    fn test_remainder_sign_follows_dividend() {
        assert_eq!(7 % 2, 1);
        assert_eq!(-7 % 2, -1);
        assert_eq!(7 % -2, 1);
    }

    #[test]
    fn test_euclidean_matches_python() {
        assert_eq!((-7_i32).div_euclid(2), -4);
        assert_eq!((-7_i32).rem_euclid(2), 1);
    }

    #[test]
    fn test_rem_euclid_always_non_negative() {
        for dividend in -10_i32..=10 {
            let rem = dividend.rem_euclid(3);
            assert!(rem >= 0, "rem_euclid({dividend}, 3) = {rem}");
        }
    }

    #[test]
    fn test_float_division_infinity() {
        assert!(f64::INFINITY == 1.0 / 0.0);
        assert!(f64::NEG_INFINITY == -1.0 / 0.0);
        assert!((0.0_f64 / 0.0).is_nan());
    }

    #[test]
    fn test_overflow_div() {
        let (_, overflowed) = i32::MIN.overflowing_div(-1);
        assert!(overflowed);
    }
}
