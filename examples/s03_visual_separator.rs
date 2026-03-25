// Demonstrates `_` as a visual separator in numeric literals.
//
// Rust lets you insert underscores anywhere in numeric literals for readability.
// They're completely ignored by the compiler — purely cosmetic.
//
// Run: cargo run --example s03_visual_separator

fn main() {
    // --- 1. Thousands separator ---
    let million = 1_000_000;
    let precise = 3.141_592_653_589_793;
    println!("million = {million}");
    println!("pi = {precise}");

    // --- 2. Binary, octal, hex readability ---
    let binary = 0b1111_0000_1010_1010;
    let octal = 0o77_77;
    let hex = 0xFF_EC_D5;
    println!("binary = {binary} (0b1111_0000_1010_1010)");
    println!("octal  = {octal} (0o77_77)");
    println!("hex    = {hex} (0xFF_EC_D5)");

    // --- 3. Type suffix with separator ---
    let big = 1_000_000u64;
    let small = 255_u8;
    println!("big = {big}, small = {small}");

    // --- 4. Leading, trailing, and multiple underscores ---
    // All of these compile, though some are not idiomatic:
    let _leading = 1_000; // idiomatic
    let _double = 1__000; // compiles but odd-looking
    println!("leading = {_leading}, double = {_double}");

    // --- 5. Floats with separators ---
    let large_float = 123_456.789_012;
    println!("large_float = {large_float}");

    // --- Convention ---
    // Use `_` every 3 digits for decimal: 1_000_000
    // Use `_` every 4 digits for binary: 0b1111_0000
    // Use `_` every 2 digits for hex: 0xFF_EC_D5
    // No leading/trailing underscores in practice.
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_separator_has_no_effect() {
        assert_eq!(1_000_000, 1000000);
        assert_eq!(0xFF_FF, 0xFFFF);
        assert_eq!(0b1111_0000, 0b11110000);
    }

    #[test]
    fn test_float_separator() {
        let a: f64 = 1_234.567_8;
        let b: f64 = 1234.5678;
        assert!((a - b).abs() < f64::EPSILON);
    }
}
