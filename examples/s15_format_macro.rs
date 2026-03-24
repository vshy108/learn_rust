// Demonstrates the format! macro for string formatting.
//
// `format!` works exactly like `println!` but returns a String
// instead of printing to stdout.
//
// Run: cargo run --example s15_format_macro

fn main() {
    // --- 1. Basic formatting ---
    let s = format!("Hello, {}!", "Rust");
    println!("{s}");

    // --- 2. Multiple arguments ---
    let name = "Alice";
    let age = 30;
    let s = format!("{name} is {age} years old");
    println!("{s}");

    // --- 3. Positional arguments ---
    let s = format!("{0} {1} {0}", "tick", "tock");
    println!("{s}");

    // --- 4. Padding and alignment ---
    println!("Right: '{:>10}'", "right"); // right-align in 10 chars
    println!("Left:  '{:<10}'", "left"); // left-align
    println!("Center:'{:^10}'", "mid"); // center
    println!("Fill:  '{:*>10}'", "star"); // fill with *

    // --- 5. Number formatting ---
    println!("Decimal:   {}", 42);
    println!("Binary:    {:b}", 42);
    println!("Octal:     {:o}", 42);
    println!("Hex lower: {:x}", 255);
    println!("Hex upper: {:X}", 255);

    // --- 6. Float precision ---
    let pi = std::f64::consts::PI;
    println!("Default:   {pi}");
    println!("2 decimal: {pi:.2}");
    println!("6 decimal: {pi:.6}");
    println!("Width+prec:{pi:>10.3}");

    // --- 7. Zero-padding ---
    println!("Zero pad:  {:05}", 42); // "00042"
    println!("Zero pad:  {:08b}", 42); // "00101010"

    // --- 8. Debug formatting ---
    let v = vec![1, 2, 3];
    println!("Debug:  {:?}", v);
    println!("Pretty: {:#?}", v);

    // --- 9. Named arguments ---
    let s = format!("{name} scored {score}%", name = "Bob", score = 95);
    println!("{s}");

    // --- 10. Escape braces ---
    println!("Literal braces: {{}}");
    println!("Format + literal: {} {{}}", "value");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_format() {
        assert_eq!(format!("{} + {} = {}", 1, 2, 3), "1 + 2 = 3");
    }

    #[test]
    fn test_padding() {
        assert_eq!(format!("{:>5}", "hi"), "   hi");
        assert_eq!(format!("{:<5}", "hi"), "hi   ");
    }

    #[test]
    fn test_precision() {
        assert_eq!(format!("{:.2}", 3.14159), "3.14");
    }

    #[test]
    fn test_hex() {
        assert_eq!(format!("{:x}", 255), "ff");
        assert_eq!(format!("{:X}", 255), "FF");
    }

    #[test]
    fn test_zero_pad() {
        assert_eq!(format!("{:05}", 42), "00042");
    }
}
