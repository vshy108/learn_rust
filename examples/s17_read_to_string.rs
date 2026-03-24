// Demonstrates the fs::read_to_string associated function.
//
// `std::fs::read_to_string` is a convenience function that:
// 1. Opens the file
// 2. Reads entire contents into a String
// 3. Closes the file
// All in one call.
//
// Run: cargo run --example s17_read_to_string

use std::fs;

/// Reads a file and returns word count.
///
/// # Errors
///
/// Returns `io::Error` if the file cannot be read
fn word_count(path: &str) -> Result<usize, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents.split_whitespace().count())
}

/// Reads first N lines of a file.
///
/// # Errors
///
/// Returns `io::Error` if the file cannot be read
fn head(path: &str, n: usize) -> Result<Vec<String>, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents.lines().take(n).map(String::from).collect())
}

fn main() {
    // --- 1. Simple file read ---
    match fs::read_to_string("Cargo.toml") {
        Ok(contents) => {
            println!("Read {} bytes", contents.len());
            println!("First line: {:?}", contents.lines().next());
        }
        Err(e) => eprintln!("Error: {e}"),
    }

    // --- 2. Word count ---
    match word_count("Cargo.toml") {
        Ok(n) => println!("Word count: {n}"),
        Err(e) => eprintln!("Error: {e}"),
    }

    // --- 3. Head (first N lines) ---
    match head("Cargo.toml", 5) {
        Ok(lines) => {
            println!("First 5 lines:");
            for line in &lines {
                println!("  {line}");
            }
        }
        Err(e) => eprintln!("Error: {e}"),
    }

    // --- 4. vs manual approach ---
    println!();
    println!("read_to_string is equivalent to:");
    println!("  let mut file = File::open(path)?;");
    println!("  let mut contents = String::new();");
    println!("  file.read_to_string(&mut contents)?;");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        let count = word_count("Cargo.toml").unwrap();
        assert!(count > 0);
    }

    #[test]
    fn test_head() {
        let lines = head("Cargo.toml", 2).unwrap();
        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn test_missing_file() {
        assert!(fs::read_to_string("no_file_xyz.txt").is_err());
    }
}
