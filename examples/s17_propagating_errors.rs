// Demonstrates propagating errors with the ? operator.
//
// `?` automatically returns the error from the current function
// if the Result is Err (or Option is None).
//
// Before `?`, you'd write:
//   let val = match result {
//       Ok(v) => v,
//       Err(e) => return Err(e),
//   };
//
// With `?`: `let val = result?;`
//
// Run: cargo run --example s17_propagating_errors

use std::fs;
use std::io;

/// Reads a file and returns its line count.
///
/// # Errors
///
/// Returns `io::Error` if the file cannot be read
fn count_lines(path: &str) -> Result<usize, io::Error> {
    let contents = fs::read_to_string(path)?; // ? propagates error
    Ok(contents.lines().count())
}

/// Reads a file, parses numbers, and sums them.
///
/// # Errors
///
/// Returns error string if file reading or parsing fails
fn sum_numbers_in_file(path: &str) -> Result<f64, String> {
    let contents = fs::read_to_string(path).map_err(|e| format!("Failed to read {path}: {e}"))?;

    let mut sum = 0.0_f64;
    for (i, line) in contents.lines().enumerate() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            let n: f64 = trimmed
                .parse()
                .map_err(|e| format!("Line {}: invalid number \"{trimmed}\": {e}", i + 1))?;
            sum += n;
        }
    }
    Ok(sum)
}

fn main() {
    // --- 1. Propagated error — success case ---
    match count_lines("Cargo.toml") {
        Ok(n) => println!("Cargo.toml: {n} lines"),
        Err(e) => eprintln!("Error: {e}"),
    }

    // --- 2. Propagated error — failure case ---
    match count_lines("nonexistent.txt") {
        Ok(n) => println!("lines: {n}"),
        Err(e) => eprintln!("Expected error: {e}"),
    }

    // --- 3. Chained propagation ---
    match sum_numbers_in_file("nonexistent.txt") {
        Ok(sum) => println!("Sum: {sum}"),
        Err(e) => eprintln!("Error: {e}"),
    }

    // --- 4. ? simplifies deeply nested error handling ---
    println!();
    println!("Without ?: nested match/if-let chains");
    println!("With ?:    clean linear code flow");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_lines_existing() {
        let count = count_lines("Cargo.toml").unwrap();
        assert!(count > 0);
    }

    #[test]
    fn test_count_lines_missing() {
        assert!(count_lines("no_such_file.txt").is_err());
    }

    #[test]
    fn test_sum_missing_file() {
        assert!(sum_numbers_in_file("no_file.txt").is_err());
    }
}
