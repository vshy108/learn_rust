// Demonstrates error type redeclaration with type aliases.
//
// Long Result types are common in Rust. A type alias shortens them.
//
// std::io does this: `pub type Result<T> = std::result::Result<T, std::io::Error>;`
// So `io::Result<String>` is shorthand for `Result<String, io::Error>`.
//
// Run: cargo run --example s17_error_type_redecl

use std::io;

// --- 1. io::Result is already a type alias ---
// Under the hood: type Result<T> = std::result::Result<T, std::io::Error>;

/// Reads a file, returning io::Result (alias).
///
/// # Errors
///
/// Returns an `io::Error` if the file cannot be read
fn read_file_short(path: &str) -> io::Result<String> {
    // This is the same as Result<String, io::Error>
    std::fs::read_to_string(path)
}

// --- 2. Custom type alias for your errors ---
type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {e}"),
            AppError::Parse(msg) => write!(f, "Parse error: {msg}"),
        }
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}

/// Reads a config value from a file.
///
/// # Errors
///
/// Returns `AppError` on IO or parse failures
fn read_config(path: &str) -> AppResult<i32> {
    let contents = std::fs::read_to_string(path)?; // io::Error → AppError via From
    let first_line = contents
        .lines()
        .next()
        .ok_or_else(|| AppError::Parse("empty file".to_string()))?;
    first_line
        .trim()
        .parse::<i32>()
        .map_err(|e| AppError::Parse(format!("invalid integer: {e}")))
}

fn main() {
    // --- 1. io::Result alias ---
    match read_file_short("Cargo.toml") {
        Ok(s) => println!("Read {} bytes via io::Result", s.len()),
        Err(e) => eprintln!("Error: {e}"),
    }

    // --- 2. Custom AppResult alias ---
    match read_config("Cargo.toml") {
        Ok(n) => println!("Config value: {n}"),
        Err(e) => eprintln!("Expected: {e}"),
    }

    // --- 3. Summary ---
    println!();
    println!("Type aliases simplify Result types:");
    println!("  type io::Result<T> = Result<T, io::Error>");
    println!("  type AppResult<T> = Result<T, AppError>");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_result() {
        assert!(read_file_short("Cargo.toml").is_ok());
    }

    #[test]
    fn test_app_error_display() {
        let e = AppError::Parse("bad".to_string());
        assert!(format!("{e}").contains("Parse error"));
    }
}
