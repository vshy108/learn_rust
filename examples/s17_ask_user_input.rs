// Demonstrates asking the user for input and handling errors.
//
// Combines stdin reading with error handling patterns.
//
// Run: echo "42" | cargo run --example s17_ask_user_input

use std::io;

/// Reads a line and parses it as a number.
///
/// # Errors
///
/// Returns a string error if reading or parsing fails
fn read_number_from_stdin() -> Result<f64, String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {e}"))?;

    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("Empty input".to_string());
    }

    trimmed
        .parse::<f64>()
        .map_err(|e| format!("Invalid number \"{trimmed}\": {e}"))
}

/// Validates that a number is positive.
///
/// # Errors
///
/// Returns error string if number is not positive
fn validate_positive(n: f64) -> Result<f64, String> {
    if n > 0.0 {
        Ok(n)
    } else {
        Err(format!("{n} is not positive"))
    }
}

fn main() {
    println!("=== Asking User for Input ===");
    println!();

    // --- 1. Simple read with error handling ---
    println!("Reading a number from stdin...");
    match read_number_from_stdin() {
        Ok(n) => {
            println!("Got number: {n}");
            // --- 2. Chain validation ---
            match validate_positive(n) {
                Ok(n) => println!("Valid positive number: {n}"),
                Err(e) => eprintln!("Validation error: {e}"),
            }
        }
        Err(e) => {
            eprintln!("Input error: {e}");
            // In a real app, you'd loop and ask again.
        }
    }

    // --- 3. Pattern: retry loop ---
    println!();
    println!("Retry loop pattern:");
    println!("  loop {{");
    println!("      println!(\"Enter a positive number:\");");
    println!("      match read_number_from_stdin() {{");
    println!("          Ok(n) if n > 0.0 => {{ use(n); break; }}");
    println!("          Ok(_) => eprintln!(\"Must be positive\"),");
    println!("          Err(e) => eprintln!(\"{{e}}\"),");
    println!("      }}");
    println!("  }}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_positive() {
        assert!(validate_positive(5.0).is_ok());
        assert!(validate_positive(-1.0).is_err());
        assert!(validate_positive(0.0).is_err());
    }
}
