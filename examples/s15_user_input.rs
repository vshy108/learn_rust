#![allow(dead_code, unused_imports, unused_parens)]
// Demonstrates collecting user input with read_line.
//
// `std::io::stdin().read_line(&mut buffer)` reads from stdin.
// It appends to the buffer and includes the trailing newline.
//
// NOTE: This example demonstrates the API but doesn't actually
// prompt for input (so it can be run in automated contexts).
//
// Run: echo "Alice" | cargo run --example s15_user_input
// Or:  cargo run --example s15_user_input  (then type input)

use std::io;

/// Reads a line from stdin and returns it trimmed.
///
/// # Errors
///
/// Returns an `io::Error` if reading from stdin fails
fn read_trimmed_line() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

/// Reads a number from stdin.
///
/// # Errors
///
/// Returns a string error if reading or parsing fails
fn read_number() -> Result<f64, String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;
    input
        .trim()
        .parse::<f64>()
        .map_err(|e| format!("Not a number: {e}"))
}

fn main() {
    println!("=== User Input with read_line ===");
    println!();

    // --- 1. Basic API ---
    println!("Basic usage:");
    println!("  let mut input = String::new();");
    println!("  std::io::stdin().read_line(&mut input).unwrap();");
    println!("  // input now contains the typed text + \\n");
    println!("  let trimmed = input.trim();");
    println!();

    // --- 2. Key points ---
    println!("Key points:");
    println!("  - read_line APPENDS to the String (doesn't clear it)");
    println!("  - The trailing newline is included (use .trim())");
    println!("  - Returns Result<usize> — the number of bytes read");
    println!("  - For parsing: input.trim().parse::<i32>()");
    println!();

    // --- 3. Interactive example (uncomment to try) ---
    // println!("Enter your name:");
    // match read_trimmed_line() {
    //     Ok(name) => println!("Hello, {name}!"),
    //     Err(e) => println!("Error: {e}"),
    // }

    // --- 4. Pattern: loop until valid input ---
    println!("Common pattern - loop until valid:");
    println!("  loop {{");
    println!("      println!(\"Enter a number:\");");
    println!("      match read_number() {{");
    println!("          Ok(n) => {{ println!(\"Got: {{n}}\"); break; }}");
    println!("          Err(e) => println!(\"{{e}}, try again\"),");
    println!("      }}");
    println!("  }}");

    // Demonstrate functions exist and compile:
    println!();
    println!("Functions read_trimmed_line() and read_number() are defined.");
    println!("Pipe input: echo \"42\" | cargo run --example s15_user_input");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_trim_pattern() {
        // Simulates what read_line gives you:
        let input = "  hello world  \n";
        assert_eq!(input.trim(), "hello world");
    }

    #[test]
    fn test_parse_pattern() {
        let input = "42\n";
        let n: i32 = input.trim().parse().unwrap();
        assert_eq!(n, 42);
    }

    #[test]
    fn test_parse_float() {
        let input = "3.14\n";
        let f: f64 = input.trim().parse().unwrap();
        assert!((f - 3.14).abs() < 0.001);
    }
}
