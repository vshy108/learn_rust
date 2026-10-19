//! # Rustfinity: Result To Option
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/result-to-option
//!
//! Sometimes you run operations that might fail, and you don't care about the specific error details. You just want to know whether the operation succeeded or failed. Rust’s `Result<T, E>` can be converted into an `Option<T>` to represent success (`Some`) or failure (`None`), discarding the error details.
//!
//! This challenge builds on the concept of handling `Result` and converting it to `Option`. You will write a function that reads the entire content of a file and returns it as an `Option<String>`.
//!
//! ## Your Task
//!
//! Implement the function `read_file`:
//!
//! 1. It takes a file path (`&str`) as input.
//! 2. Attempts to open the file and read its entire content as a `String`.
//! 3. If the operation is successful, return `Some(String)` containing the file content.
//! 4. If any error occurs (e.g., file not found, permission issues, etc.), return `None`.
//! 5. Use the `.ok()` method to convert `Result` into `Option` and use the `?` operator to easily propagate errors if required.
//!
//! Run: `cargo run --example rf_result_to_option`
//! Test: `cargo test --example rf_result_to_option`
//! Filter: `cargo test --example rf_result_to_option <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub fn read_file(file_path: &str) -> Option<String> {
    // TODO: Implement this function
    // Hint: Use `File::open` and `.read_to_string()` with `?` to propagate errors.
    unimplemented!()
}

// Example usage
pub fn main() {
    let file_path = "example.txt";

    match read_file(file_path) {
        Some(contents) => println!("File contents:\n{}", contents),
        None => println!("Failed to read the file."),
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::fs::{remove_file, write};

    #[test]
    fn test_read_file_valid_file() {
        let file_path = "test_valid.txt";
        write(file_path, "Hello, world!\nSecond line.").unwrap();
        assert_eq!(
            read_file(file_path),
            Some("Hello, world!\nSecond line.".to_string())
        );
        remove_file(file_path).unwrap();
    }

    #[test]
    fn test_read_file_empty_file() {
        let file_path = "test_empty.txt";
        write(file_path, "").unwrap();
        assert_eq!(read_file(file_path), Some("".to_string()));
        remove_file(file_path).unwrap();
    }

    #[test]
    fn test_read_file_nonexistent_file() {
        let file_path = "nonexistent.txt";
        assert_eq!(read_file(file_path), None);
    }

    #[test]
    fn test_read_file_permission_denied() {
        #[cfg(unix)]
        {
            use std::fs::set_permissions;
            use std::os::unix::fs::PermissionsExt;
            use tempfile::NamedTempFile;

            let tmp_file = NamedTempFile::new().unwrap();
            write(&tmp_file, "Cannot read this file.").unwrap();

            let permissions = set_permissions(&tmp_file, PermissionsExt::from_mode(0o000));
            assert!(permissions.is_ok());
            assert_eq!(read_file(tmp_file.path().to_str().unwrap()), None);
        }
    }

    #[test]
    fn test_read_file_large_file() {
        let file_path = "test_large.txt";
        let content = "a".repeat(10_000); // 10 KB of data
        write(file_path, &content).unwrap();
        assert_eq!(read_file(file_path), Some(content));
        remove_file(file_path).unwrap();
    }
}
