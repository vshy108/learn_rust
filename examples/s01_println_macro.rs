// Section 1, Topic 4: println! macro deep dive
//
// println!() is a **macro** (the `!` suffix means macro, not function).
//
// Why a macro instead of a function?
//   - The compiler parses the format string at compile time.
//   - It can accept a variable number of arguments (variadic — not possible
//     with regular Rust functions).
//   - Type mismatches in format strings are caught at compile time, not runtime.
//
// Related macros: print!() (no newline), eprintln!() (stderr), format!() (returns String).

/// Format an integer in multiple representations.
///
/// # Arguments
///
/// * `value` - The integer to format.
///
/// # Returns
///
/// A vector of (label, formatted_string) pairs.
fn format_integer(value: i32) -> Vec<(&'static str, String)> {
    vec![
        ("Display (default)", format!("{value}")),
        ("Debug", format!("{value:?}")),
        ("Binary", format!("{value:b}")),
        ("Octal", format!("{value:o}")),
        ("Hex (lower)", format!("{value:x}")),
        ("Hex (upper)", format!("{value:X}")),
        ("Padded (width 10)", format!("{value:>10}")),
        ("Zero-padded (width 5)", format!("{value:05}")),
    ]
}

/// Format a float with various precision levels.
///
/// # Arguments
///
/// * `value` - The float to format.
///
/// # Returns
///
/// A vector of (label, formatted_string) pairs.
fn format_float(value: f64) -> Vec<(&'static str, String)> {
    vec![
        ("Default", format!("{value}")),
        ("2 decimal places", format!("{value:.2}")),
        ("Scientific (lower)", format!("{value:e}")),
        ("Scientific (upper)", format!("{value:E}")),
    ]
}

/// Demonstrate positional and named arguments in format strings.
///
/// # Returns
///
/// A formatted string using positional and named arguments.
fn positional_and_named() -> String {
    // Positional: {0} and {1} refer to argument indices.
    let positional = format!("{0} is {1} and {1} is {0}", "left", "right");

    // Named: use variable names directly inside braces.
    let language = "Rust";
    let year = 2015;
    let named = format!("{language} was released in {year}");

    format!("{positional}\n{named}")
}

fn main() {
    println!("=== println! Macro Deep Dive ===\n");

    // --- Integer formatting ---
    println!("-- Integer formatting (value = 42) --");
    for (label, formatted) in format_integer(42) {
        println!("  {label:25}: {formatted}");
    }

    // --- Float formatting ---
    println!("\n-- Float formatting (value = 3.14159) --");
    for (label, formatted) in format_float(3.14159) {
        println!("  {label:25}: {formatted}");
    }

    // --- Positional and named ---
    println!("\n-- Positional and named arguments --");
    println!("{}", positional_and_named());

    // --- Debug formatting ---
    // {:?} prints the Debug representation.
    // {:#?} is "pretty-print Debug" — adds indentation for nested structures.
    println!("\n-- Debug formatting --");
    let numbers = vec![1, 2, 3];
    println!("  Debug:        {numbers:?}");
    println!("  Pretty debug: {numbers:#?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_integer_display() {
        let formats = format_integer(42);
        // First entry is the default Display representation.
        assert_eq!(formats[0].1, "42");
    }

    #[test]
    fn test_format_integer_binary() {
        let formats = format_integer(42);
        assert_eq!(formats[2].1, "101010");
    }

    #[test]
    fn test_format_integer_hex_lower() {
        let formats = format_integer(255);
        assert_eq!(formats[4].1, "ff");
    }

    #[test]
    fn test_format_integer_hex_upper() {
        let formats = format_integer(255);
        assert_eq!(formats[5].1, "FF");
    }

    #[test]
    fn test_format_integer_zero_padded() {
        let formats = format_integer(42);
        assert_eq!(formats[7].1, "00042");
    }

    #[test]
    fn test_format_float_two_decimals() {
        let formats = format_float(3.14159);
        assert_eq!(formats[1].1, "3.14");
    }

    #[test]
    fn test_positional_and_named_contains_both() {
        let result = positional_and_named();
        assert!(result.contains("left is right"));
        assert!(result.contains("Rust was released in 2015"));
    }
}
