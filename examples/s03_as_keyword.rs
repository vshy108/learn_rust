// Demonstrates the `as` keyword for type casting in Rust.
//
// `as` performs explicit type conversions between primitive types.
// Rust requires explicit type conversion.
//
// `as` is a *truncating/wrapping* cast — it can silently lose data.
// For safer conversions, prefer `From`/`Into` (infallible) or
// `TryFrom`/`TryInto` (returns Result on failure).
//
// Run: cargo run --example s03_as_keyword

fn main() {
    // --- 1. Integer widening (safe) ---
    // Smaller to larger: no data loss, always safe.
    let small: i8 = 42;
    let big: i64 = small as i64;
    println!("i8 {small} as i64 = {big}");

    // Prefer `From`/`Into` for safe widening (compile-time guarantee):
    let big2: i64 = i64::from(small);
    println!("i64::from(i8) = {big2}");

    // --- 2. Integer narrowing (lossy!) ---
    // Larger to smaller: truncates to fit the target type.
    let big_val: i32 = 300;
    let small_val: u8 = big_val as u8; // 300 % 256 = 44
    println!("i32 {big_val} as u8 = {small_val}");
    // Gotcha: no warning or error — data silently lost!

    // Safer: use TryFrom
    let safe_result = u8::try_from(big_val);
    println!("u8::try_from({big_val}) = {safe_result:?}"); // Err(...)

    // --- 3. Signed to unsigned ---
    let negative: i32 = -1;
    let as_unsigned: u32 = negative as u32; // wraps to u32::MAX
    println!("i32 {negative} as u32 = {as_unsigned}");

    // --- 4. Float to integer (truncates toward zero) ---
    let f: f64 = 3.99;
    let i: i32 = f as i32;
    println!("f64 {f} as i32 = {i}"); // 3 (truncates, doesn't round)

    let neg_f: f64 = -3.99;
    let neg_i: i32 = neg_f as i32;
    println!("f64 {neg_f} as i32 = {neg_i}"); // -3

    // --- 5. Integer to float ---
    let n: i32 = 42;
    let f: f64 = n as f64;
    println!("i32 {n} as f64 = {f}");

    // Gotcha: large integers may lose precision in f32
    let large: i64 = 16_777_217; // 2^24 + 1
    let as_f32: f32 = large as f32;
    println!("i64 {large} as f32 = {as_f32}"); // 16777216.0 (precision lost!)

    // --- 6. bool/char casts ---
    let t: bool = true;
    let f_b: bool = false;
    println!("true as u8 = {}", t as u8); // 1
    println!("false as u8 = {}", f_b as u8); // 0

    // char -> u32 gives the Unicode code point
    let ch = 'A';
    println!("'A' as u32 = {}", ch as u32); // 65

    // u8 -> char (only u8 is allowed, not larger integers)
    let byte: u8 = 65;
    let c = byte as char;
    println!("65u8 as char = '{c}'"); // 'A'

    // --- 7. `as` for pointer casts (advanced) ---
    // `as` can cast between raw pointer types and usize.
    // This is mainly used in unsafe code / FFI.
    let x = 42;
    let ptr = &x as *const i32;
    let addr = ptr as usize;
    println!("pointer address as usize: {addr:#x}");

    // --- Summary of safer alternatives ---
    println!();
    println!("Prefer these over `as`:");
    println!("  From/Into:       safe widening (i8 -> i64)");
    println!("  TryFrom/TryInto: checked narrowing (i32 -> u8)");
    println!("  as:              only for primitive casts where you");
    println!("                   accept truncation/wrapping");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_widening() {
        let x: i8 = 42;
        assert_eq!(x as i64, 42i64);
    }

    #[test]
    fn test_narrowing_truncates() {
        assert_eq!(300i32 as u8, 44); // 300 % 256 = 44
    }

    #[test]
    fn test_negative_to_unsigned() {
        assert_eq!(-1i32 as u32, u32::MAX);
    }

    #[test]
    fn test_float_to_int_truncates() {
        assert_eq!(3.99f64 as i32, 3);
        assert_eq!(-3.99f64 as i32, -3);
    }

    #[test]
    fn test_bool_to_int() {
        assert_eq!(true as u8, 1);
        assert_eq!(false as u8, 0);
    }

    #[test]
    fn test_char_to_u32() {
        assert_eq!('A' as u32, 65);
        assert_eq!(65u8 as char, 'A');
    }

    #[test]
    fn test_try_from_out_of_range() {
        assert!(u8::try_from(300i32).is_err());
        assert!(u8::try_from(255i32).is_ok());
    }
}
