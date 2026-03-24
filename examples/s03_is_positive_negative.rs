// Demonstrates `is_positive()` and `is_negative()` methods on integers.
//
// These methods are available on all signed integer types (i8..i128, isize).
// They return `true` if the value is > 0 or < 0 respectively.
// Zero is neither positive nor negative.
//
// Run: cargo run --example s03_is_positive_negative

fn main() {
    // --- 1. Basic usage ---
    let pos = 42_i32;
    let neg = -42_i32;
    let zero = 0_i32;

    println!("{pos}.is_positive() = {}", pos.is_positive()); // true
    println!("{pos}.is_negative() = {}", pos.is_negative()); // false
    println!("{neg}.is_positive() = {}", neg.is_positive()); // false
    println!("{neg}.is_negative() = {}", neg.is_negative()); // true
    println!("{zero}.is_positive() = {}", zero.is_positive()); // false
    println!("{zero}.is_negative() = {}", zero.is_negative()); // false

    // --- 2. Zero is neither ---
    // Gotcha for Python developers: Python's math.copysign distinguishes
    // +0.0 and -0.0 for floats, but for integers, 0 is simply neutral.

    // --- 3. signum() method ---
    // Returns -1, 0, or 1 indicating the sign.
    println!();
    println!("{pos}.signum() = {}", pos.signum()); // 1
    println!("{neg}.signum() = {}", neg.signum()); // -1
    println!("{zero}.signum() = {}", zero.signum()); // 0

    // --- 4. abs() method ---
    // Returns the absolute value. Panics on i32::MIN in debug mode!
    println!();
    println!("(-42).abs() = {}", (-42_i32).abs());
    // Gotcha: i32::MIN.abs() overflows because |i32::MIN| = i32::MAX + 1
    // Use `checked_abs()` or `unsigned_abs()` for safety:
    println!("i32::MIN.unsigned_abs() = {}", i32::MIN.unsigned_abs());
    println!("i32::MIN.checked_abs() = {:?}", i32::MIN.checked_abs()); // None

    // --- 5. Float equivalents ---
    // Floats have `is_sign_positive()` and `is_sign_negative()` instead.
    // These distinguish +0.0 from -0.0!
    let pz = 0.0_f64;
    let nz = -0.0_f64;
    println!();
    println!("Float sign methods:");
    println!("  0.0.is_sign_positive() = {}", pz.is_sign_positive()); // true
    println!("  -0.0.is_sign_positive() = {}", nz.is_sign_positive()); // false
    println!("  -0.0.is_sign_negative() = {}", nz.is_sign_negative()); // true

    // --- 6. Not available on unsigned types ---
    // `u32::is_positive()` doesn't exist — unsigned values are always >= 0.
    // The compiler will error if you try.
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_positive() {
        assert!(42_i32.is_positive());
        assert!(!42_i32.is_negative());
    }

    #[test]
    fn test_negative() {
        assert!((-42_i32).is_negative());
        assert!(!(-42_i32).is_positive());
    }

    #[test]
    fn test_zero_is_neither() {
        assert!(!0_i32.is_positive());
        assert!(!0_i32.is_negative());
    }

    #[test]
    fn test_signum() {
        assert_eq!(42_i32.signum(), 1);
        assert_eq!((-42_i32).signum(), -1);
        assert_eq!(0_i32.signum(), 0);
    }

    #[test]
    fn test_unsigned_abs_handles_min() {
        // i32::MIN.abs() would panic, but unsigned_abs() is safe.
        assert_eq!(i32::MIN.unsigned_abs(), 2_147_483_648);
    }

    #[test]
    fn test_float_negative_zero() {
        assert!((-0.0_f64).is_sign_negative());
        assert!((0.0_f64).is_sign_positive());
    }
}
