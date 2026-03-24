// Demonstrates float division behavior in Rust.
//
// Rust's f32/f64 follow IEEE 754 — the same standard Python uses.
// Understanding float quirks is essential for numeric correctness.
//
// Run: cargo run --example s03_float_division

fn main() {
    // --- 1. Basic float division ---
    let result = 2.0_f64 / 3.0;
    println!("2.0 / 3.0 = {result}");
    // => 0.6666666666666666

    // --- 2. Float representation is approximate ---
    // 0.1 + 0.2 != 0.3 in IEEE 754 (same gotcha as Python)
    let sum = 0.1_f64 + 0.2;
    println!("0.1 + 0.2 = {sum}");
    println!("0.1 + 0.2 == 0.3? {}", sum == 0.3); // false!
    println!("difference: {:.20}", sum - 0.3);

    // --- 3. Comparing floats correctly ---
    // Use an epsilon (tolerance) for approximate equality.
    let eps = f64::EPSILON;
    let close_enough = (sum - 0.3).abs() < eps * 4.0;
    println!("close enough? {close_enough}");

    // --- 4. Special values ---
    println!();
    println!("Special float values:");
    println!("  INFINITY:     {}", f64::INFINITY);
    println!("  NEG_INFINITY: {}", f64::NEG_INFINITY);
    println!("  NAN:          {}", f64::NAN);
    println!("  EPSILON:      {eps}");
    println!("  MIN:          {}", f64::MIN);
    println!("  MAX:          {}", f64::MAX);

    // --- 5. NaN is weird ---
    let nan = f64::NAN;
    println!();
    println!("NaN comparisons:");
    println!("  NaN == NaN: {}", nan == nan); // false!
    println!("  NaN != NaN: {}", nan != nan); // true!
    println!("  NaN < 0.0:  {}", nan < 0.0); // false
    println!("  NaN > 0.0:  {}", nan > 0.0); // false
    println!("  NaN.is_nan(): {}", nan.is_nan()); // true
    // Gotcha: NaN != NaN. Always use `.is_nan()` to check.

    // --- 6. Float methods ---
    println!();
    let x = 2.0_f64 / 3.0;
    println!("floor:  {}", x.floor()); // 0.0
    println!("ceil:   {}", x.ceil()); // 1.0
    println!("round:  {}", x.round()); // 1.0
    println!("trunc:  {}", x.trunc()); // 0.0 (toward zero)
    println!("fract:  {:.6}", x.fract()); // 0.666667

    // --- 7. f32 vs f64 precision ---
    let f32_val: f32 = 1.0 / 3.0;
    let f64_val: f64 = 1.0 / 3.0;
    println!();
    println!("f32: 1/3 = {f32_val:.10}"); // ~7 digits accurate
    println!("f64: 1/3 = {f64_val:.18}"); // ~15 digits accurate

    // --- 8. Floats are NOT Eq or Ord ---
    // Due to NaN, floats only implement `PartialEq` and `PartialOrd`,
    // not `Eq` or `Ord`. This means you can't directly use them as
    // HashMap keys or sort them without a custom comparator.
    // Use `f64::total_cmp` for a total ordering that handles NaN.

    // --- Gotcha: integer division vs float division ---
    // `2 / 3` is integer division = 0
    // `2.0 / 3.0` is float division = 0.666...
    // Mixing: `2.0 / 3` won't compile — no implicit conversion.
    println!();
    println!("2 / 3 (integer) = {}", 2_i32 / 3);
    println!("2.0 / 3.0 (float) = {}", 2.0_f64 / 3.0);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_float_division() {
        let result = 2.0_f64 / 3.0;
        assert!((result - 0.6666666666666666).abs() < f64::EPSILON);
    }

    #[test]
    fn test_point_one_plus_point_two() {
        // Demonstrates IEEE 754 imprecision.
        assert_ne!(0.1_f64 + 0.2, 0.3);
    }

    #[test]
    fn test_nan_not_equal_to_itself() {
        let nan = f64::NAN;
        assert_ne!(nan, nan);
        assert!(nan.is_nan());
    }

    #[test]
    fn test_infinity() {
        assert!((1.0_f64 / 0.0).is_infinite());
        assert!((1.0_f64 / 0.0).is_sign_positive());
        assert!((-1.0_f64 / 0.0).is_sign_negative());
    }

    #[test]
    fn test_total_cmp_handles_nan() {
        // total_cmp provides a total ordering including NaN.
        use std::cmp::Ordering;
        assert_eq!(f64::NAN.total_cmp(&f64::NAN), Ordering::Equal);
    }
}
