// Demonstrates string interpolation using `{}` inside Rust format macros.
//
// Rust's formatting system (`println!`, `format!`, `write!`, etc.) uses
// curly braces `{}` as placeholders. This is different from C's `printf`
// (which uses `%d`, `%s`, etc.) and Python's f-strings (which use `{expr}`).
//
// Run: cargo run --example s01_interpolation_curly_braces

fn main() {
    // --- 1. Inline variable interpolation (Rust 1.58+) ---
    // You can place a variable name directly inside `{}`.
    // This is the preferred style when the expression is a simple identifier.
    let name = "Rust";
    println!("Hello, {name}!");
    // => Hello, Rust!

    // --- 2. Positional arguments ---
    // `{0}`, `{1}`, etc. refer to arguments by index.
    // Useful when you need to repeat an argument or reorder them.
    let lang = "Rust";
    let who = "Everyone";
    println!("{0} is fast. {0} is safe. {1} loves {0}.", lang, who);
    // => Rust is fast. Rust is safe. Everyone loves Rust.

    // --- 3. Named arguments ---
    // You can name parameters directly in the macro call.
    let language = "Rust";
    let feature = "zero-cost abstractions";
    println!("{language} has {feature}.");
    // => Rust has zero-cost abstractions.

    // --- 4. Display vs Debug formatting ---
    // `{}`  uses the `Display` trait (human-friendly output).
    // `{:?}` uses the `Debug` trait (developer-friendly, shows structure).
    // `{:#?}` is "pretty-print" Debug (indented, multi-line).
    let nums = vec![1, 2, 3];
    // Vectors don't implement Display, so `{}` won't compile for them.
    // Use Debug format instead:
    println!("Debug:        {:?}", nums);
    // => Debug:        [1, 2, 3]
    println!("Pretty debug: {:#?}", nums);
    // => Pretty debug: [
    //        1,
    //        2,
    //        3,
    //    ]

    // --- 5. Formatting traits summary ---
    // `{}`    -> Display        (human-readable)
    // `{:?}`  -> Debug          (developer-readable)
    // `{:#?}` -> Debug, pretty  (indented Debug)
    // `{:b}`  -> Binary
    // `{:o}`  -> Octal
    // `{:x}`  -> Hex lowercase
    // `{:X}`  -> Hex uppercase
    // `{:e}`  -> Scientific lowercase
    // `{:E}`  -> Scientific uppercase
    // `{:p}`  -> Pointer address
    let val = 255;
    println!("Decimal:     {val}");
    println!("Binary:      {val:b}");
    println!("Octal:       {val:o}");
    println!("Hex lower:   {val:x}");
    println!("Hex upper:   {val:X}");
    println!("Scientific:  {:e}", 123_456.789);
    // => Decimal: 255, Binary: 11111111, Octal: 377, Hex: ff / FF

    // --- 6. Width and alignment ---
    // `{:<width}`  left-align
    // `{:>width}`  right-align (default for numbers)
    // `{:^width}`  center-align
    // `{:fill^width}` center with custom fill character
    println!("|{:<10}|", "left");
    // => |left      |
    println!("|{:>10}|", "right");
    // => |     right|
    println!("|{:^10}|", "center");
    // => |  center  |
    println!("|{:-^10}|", "hi");
    // => |----hi----|

    // --- 7. Precision for floats ---
    // `{:.N}` limits decimal places.
    let pi = std::f64::consts::PI;
    println!("pi = {pi:.4}");
    // => pi = 3.1416

    // Precision also truncates strings:
    println!("{:.5}", "Hello, world!");
    // => Hello

    // --- 8. Zero-padding ---
    // `{:0>width}` pads with zeros. Commonly used for numbers.
    println!("Padded: {:05}", 42);
    // => Padded: 00042

    // --- 9. Literal curly braces ---
    // To print a literal `{` or `}`, double them: `{{` and `}}`.
    // Gotcha: forgetting to double them causes a compile error.
    println!("Set notation: {{1, 2, 3}}");
    // => Set notation: {1, 2, 3}

    // --- 10. Combining inline variables with format specs ---
    // You can use `{var:spec}` syntax for inline variables too.
    let x = 42;
    println!("Hex of x: {x:#06x}");
    // => Hex of x: 0x002a
    // `#` adds the `0x` prefix, `06` pads to 6 chars total, `x` is hex.

    // --- Gotcha: expressions are NOT allowed inline ---
    // `println!("{x + 1}")` does NOT work. Only simple identifiers are
    // allowed inside `{}`. For expressions, use positional arguments:
    println!("x + 1 = {}", x + 1);
    // => x + 1 = 43

    // --- Gotcha: argument count mismatch is a compile error ---
    // `println!("{} {}", 1)` won't compile -- Rust checks at compile time
    // that the number of `{}` placeholders matches the argument count.
    // This is a major safety advantage over C's printf.
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_format_inline_variable() {
        let name = "world";
        assert_eq!(format!("Hello, {name}!"), "Hello, world!");
    }

    #[test]
    fn test_format_positional() {
        let s = format!("{0} and {1} and {0}", "a", "b");
        assert_eq!(s, "a and b and a");
    }

    #[test]
    fn test_format_named() {
        let s = format!("{x} + {y}", x = 1, y = 2);
        assert_eq!(s, "1 + 2");
    }

    #[test]
    fn test_format_debug() {
        let v = vec![1, 2, 3];
        assert_eq!(format!("{v:?}"), "[1, 2, 3]");
    }

    #[test]
    fn test_format_width_alignment() {
        assert_eq!(format!("|{:<10}|", "left"), "|left      |");
        assert_eq!(format!("|{:>10}|", "right"), "|     right|");
        assert_eq!(format!("|{:^10}|", "mid"), "|   mid    |");
        assert_eq!(format!("|{:-^10}|", "hi"), "|----hi----|");
    }

    #[test]
    fn test_format_precision() {
        assert_eq!(format!("{:.2}", std::f64::consts::PI), "3.14");
        // Precision truncates strings too.
        assert_eq!(format!("{:.5}", "Hello, world!"), "Hello");
    }

    #[test]
    fn test_format_zero_padding() {
        assert_eq!(format!("{:05}", 42), "00042");
    }

    #[test]
    fn test_format_literal_braces() {
        assert_eq!(format!("{{}}"), "{}");
    }

    #[test]
    fn test_format_hex_with_prefix() {
        assert_eq!(format!("{:#06x}", 42), "0x002a");
    }
}
