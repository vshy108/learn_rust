// Demonstrates Rust format specifiers in detail.
//
// Format specifiers go inside `{}` in `println!`, `format!`, `write!`, etc.
// Syntax: `{[arg][:[[fill]align][sign][#][0][width][.precision][type]]}`
//
// This builds on s01_interpolation_curly_braces with more advanced specifiers.
//
// Run: cargo run --example s03_format_specifiers

fn main() {
    // --- 1. Sign specifiers ---
    // `{:+}` forces a sign (+ or -) on numbers
    let pos = 42;
    let neg = -42;
    println!("default:    {pos}, {neg}");
    println!("with sign:  {pos:+}, {neg:+}");

    // --- 2. # (alternate form) ---
    // Adds prefix: 0b for binary, 0o for octal, 0x for hex
    let n = 255;
    println!("binary:  {n:#b}"); // 0b11111111
    println!("octal:   {n:#o}"); // 0o377
    println!("hex:     {n:#x}"); // 0xff
    println!("hex:     {n:#X}"); // 0xFF
    // For Debug: `{:#?}` gives pretty-printed output (covered before)

    // --- 3. Zero-fill with width ---
    // `{:0width}` pads with zeros (right-aligned by default for numbers)
    println!("zero-fill: {:08b}", 42u8); // 00101010
    println!("zero-fill: {:06x}", 255); // 0000ff
    println!("zero-fill: {:05}", 42); // 00042

    // --- 4. Width with variables ---
    // Use `{:width$}` to take width from a variable
    let width = 15;
    println!("|{:>width$}|", "dynamic"); // |        dynamic|

    // --- 5. Precision with variables ---
    // Use `{:.prec$}` to take precision from a variable
    let prec = 3;
    println!("pi = {:.prec$}", std::f64::consts::PI); // 3.142

    // --- 6. Fill and align for non-numeric types ---
    // `{:.<width}` left-align with dot fill
    // `{:.>width}` right-align with dot fill
    // `{:.^width}` center-align with dot fill
    println!("|{:.<20}|", "left"); // |left................|
    println!("|{:.>20}|", "right"); // |...............right|
    println!("|{:.^20}|", "center"); // |.......center.......|

    // --- 7. Named + specifiers combined ---
    let val = 42;
    println!("combined: {val:+#010x}"); // +0x000002a

    // --- 8. Pointer formatting ---
    let x = 42;
    let ptr = &x;
    println!("pointer: {ptr:p}"); // 0x7fff... (platform-dependent)

    // --- 9. Escaping in format strings ---
    // `{{` and `}}` produce literal braces
    println!("literal braces: {{ and }}");

    // --- 10. Debug with precision ---
    // `{:.N?}` limits float precision inside Debug output
    let v = vec![1.23456789, 2.34567890, 3.45678901];
    println!("debug with precision: {v:.2?}");
    // => [1.23, 2.35, 3.46]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sign_format() {
        assert_eq!(format!("{:+}", 42), "+42");
        assert_eq!(format!("{:+}", -42), "-42");
    }

    #[test]
    fn test_alternate_hex() {
        assert_eq!(format!("{:#x}", 255), "0xff");
        assert_eq!(format!("{:#X}", 255), "0xFF");
    }

    #[test]
    fn test_zero_fill_binary() {
        assert_eq!(format!("{:08b}", 42), "00101010");
    }

    #[test]
    fn test_dynamic_width() {
        let w = 10;
        assert_eq!(format!("|{:>w$}|", "hi"), "|        hi|");
    }

    #[test]
    fn test_dynamic_precision() {
        let p = 2;
        assert_eq!(format!("{:.p$}", 3.14159), "3.14");
    }

    #[test]
    fn test_fill_align() {
        assert_eq!(format!("|{:.<10}|", "hi"), "|hi........|");
        assert_eq!(format!("|{:.>10}|", "hi"), "|........hi|");
    }
}
