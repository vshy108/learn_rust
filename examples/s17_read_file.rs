// Demonstrates reading file contents.
//
// Several approaches:
// 1. `fs::read_to_string` — reads entire file to String
// 2. `BufReader` — buffered line-by-line reading
// 3. `fs::read` — reads bytes
//
// Run: cargo run --example s17_read_file

use std::fs;
use std::io::{self, BufRead};

fn main() {
    // --- 1. fs::read_to_string — simplest approach ---
    match fs::read_to_string("Cargo.toml") {
        Ok(contents) => {
            let lines: Vec<&str> = contents.lines().collect();
            println!("Cargo.toml has {} lines", lines.len());
            // Show first 3 lines:
            for line in lines.iter().take(3) {
                println!("  {line}");
            }
        }
        Err(e) => eprintln!("Error reading file: {e}"),
    }

    println!();

    // --- 2. BufReader for line-by-line reading ---
    // Better for large files — doesn't load everything at once.
    match fs::File::open("Cargo.toml") {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            println!("Reading line by line:");
            for (i, line) in reader.lines().enumerate() {
                match line {
                    Ok(text) => {
                        if i < 3 {
                            println!("  line {}: {text}", i + 1);
                        }
                    }
                    Err(e) => eprintln!("  Error reading line: {e}"),
                }
            }
        }
        Err(e) => eprintln!("Error opening file: {e}"),
    }

    println!();

    // --- 3. fs::read — read as bytes ---
    match fs::read("Cargo.toml") {
        Ok(bytes) => println!("Read {} bytes", bytes.len()),
        Err(e) => eprintln!("Error: {e}"),
    }

    // --- 4. Reading a missing file ---
    match fs::read_to_string("nosuchfile.txt") {
        Ok(_) => println!("Unexpected success"),
        Err(e) => eprintln!("Expected error: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_read_cargo_toml() {
        let contents = fs::read_to_string("Cargo.toml").unwrap();
        assert!(contents.contains("[package]"));
    }

    #[test]
    fn test_read_missing_file() {
        let result = fs::read_to_string("nonexistent_file_xyz.txt");
        assert!(result.is_err());
    }
}
