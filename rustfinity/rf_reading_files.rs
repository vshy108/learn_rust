//! # Rustfinity: Reading Files
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/reading-files
//!
//! Reading files is a fundamental operation in any programming language. In Rust, the standard library provides several ways to read files, each suited for different use cases.
//!
//! ## Opening and Reading Files
//!
//! The most basic way to read a file is using `std::fs::read_to_string`:
//!
//! ```rust
//! use std::fs;
//!
//! let contents = fs::read_to_string("file.txt")?;
//! println!("{}", contents);
//! ```
//!
//! For more control, you can use `File::open` with `Read` trait methods:
//!
//! ```rust
//! use std::fs::File;
//! use std::io::Read;
//!
//! let mut file = File::open("file.txt")?;
//! let mut contents = String::new();
//! file.read_to_string(&mut contents)?;
//! ```
//!
//! ## Buffered Reading with BufReader
//!
//! For line-by-line reading, `BufReader` provides efficient buffered access:
//!
//! ```rust
//! use std::fs::File;
//! use std::io::{BufRead, BufReader};
//!
//! let file = File::open("file.txt")?;
//! let reader = BufReader::new(file);
//!
//! for line in reader.lines() {
//!     let line = line?;
//!     println!("{}", line);
//! }
//! ```
//!
//! The `BufReader` buffers reads internally, which is more efficient than reading one byte at a time, especially for large files.
//!
//! ## Your Task
//!
//! Implement the following functions for reading file contents:
//!
//! 1. `read_entire_file(path: &str) -> Result<String, io::Error>` - Read the entire contents of a file into a String
//! 2. `count_lines(path: &str) -> Result<usize, io::Error>` - Count the number of lines in a file
//! 3. `count_words(path: &str) -> Result<usize, io::Error>` - Count the number of words in a file (words are whitespace-separated)
//! 4. `read_lines(path: &str) -> Result<Vec<String>, io::Error>` - Read all lines into a Vec, with trailing newlines removed
//! 5. `first_n_lines(path: &str, n: usize) -> Result<Vec<String>, io::Error>` - Read only the first n lines from a file
//!
//! ## Examples
//!
//! ```rust
//! // Assuming a file "example.txt" contains:
//! // Hello World
//! // This is a test
//! // Rust is great
//!
//! assert_eq!(
//!     read_entire_file("example.txt")?,
//!     "Hello World\nThis is a test\nRust is great\n"
//! );
//! assert_eq!(count_lines("example.txt")?, 3);
//! assert_eq!(count_words("example.txt")?, 9);
//! assert_eq!(
//!     read_lines("example.txt")?,
//!     vec!["Hello World", "This is a test", "Rust is great"]
//! );
//! assert_eq!(
//!     first_n_lines("example.txt", 2)?,
//!     vec!["Hello World", "This is a test"]
//! );
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - Use `std::fs::read_to_string` for the simplest way to read a whole file
//! - Use `BufReader::new(File::open(path)?)` to create a buffered reader
//! - The `lines()` iterator from `BufRead` returns `Result<String, io::Error>` for each line
//! - Use `split_whitespace()` to split a string into words
//! - Use `.take(n)` on an iterator to limit how many items you process
//! - Remember to handle the `Result` type returned by `File::open` and I/O operations
//!
//! </details>
//!
//! Run: `cargo run --example rf_reading_files`
//! Test: `cargo test --example rf_reading_files`
//! Filter: `cargo test --example rf_reading_files <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

/// Read the entire contents of a file into a String.
pub fn read_entire_file(path: &str) -> Result<String, io::Error> {
    // TODO: Read the entire file contents into a String
    unimplemented!()
}

/// Count the number of lines in a file.
pub fn count_lines(path: &str) -> Result<usize, io::Error> {
    // TODO: Open the file, create a BufReader, and count the lines
    unimplemented!()
}

/// Count the number of words in a file.
/// Words are defined as whitespace-separated sequences of characters.
pub fn count_words(path: &str) -> Result<usize, io::Error> {
    // TODO: Read the file and count whitespace-separated words
    unimplemented!()
}

/// Read all lines from a file into a Vec, with trailing newlines removed.
pub fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    // TODO: Open the file, create a BufReader, and collect all lines
    unimplemented!()
}

/// Read only the first n lines from a file.
pub fn first_n_lines(path: &str, n: usize) -> Result<Vec<String>, io::Error> {
    // TODO: Read only the first n lines from the file
    unimplemented!()
}

pub fn main() {
    // Example: Create a sample file and read it
    let sample_content = "Hello World\nThis is a test\nRust is great\n";
    let path = "sample.txt";

    // Write sample file
    if fs::write(path, sample_content).is_ok() {
        println!("Sample file created.");

        // Read entire file
        if let Ok(contents) = read_entire_file(path) {
            println!("File contents:\n{}", contents);
        }

        // Count lines
        if let Ok(count) = count_lines(path) {
            println!("Line count: {}", count);
        }

        // Count words
        if let Ok(count) = count_words(path) {
            println!("Word count: {}", count);
        }

        // Read lines into Vec
        if let Ok(lines) = read_lines(path) {
            println!("Lines: {:?}", lines);
        }

        // Read first 2 lines
        if let Ok(lines) = first_n_lines(path, 2) {
            println!("First 2 lines: {:?}", lines);
        }

        // Clean up
        let _ = fs::remove_file(path);
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::fs::{remove_file, write};
    use std::io;

    // Helper to create a test file and ensure cleanup
    struct TestFile {
        path: String,
    }

    impl TestFile {
        fn new(name: &str, content: &str) -> Self {
            let path = format!("test_{}.txt", name);
            write(&path, content).expect("Failed to create test file");
            TestFile { path }
        }
    }

    impl Drop for TestFile {
        fn drop(&mut self) {
            let _ = remove_file(&self.path);
        }
    }

    // ==================== read_entire_file tests ====================

    #[test]
    fn test_read_entire_file_simple() {
        let file = TestFile::new("read_simple", "Hello, World!");
        let result = read_entire_file(&file.path).unwrap();
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_read_entire_file_multiline() {
        let file = TestFile::new("read_multiline", "Line 1\nLine 2\nLine 3\n");
        let result = read_entire_file(&file.path).unwrap();
        assert_eq!(result, "Line 1\nLine 2\nLine 3\n");
    }

    #[test]
    fn test_read_entire_file_empty() {
        let file = TestFile::new("read_empty", "");
        let result = read_entire_file(&file.path).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_read_entire_file_unicode() {
        let file = TestFile::new("read_unicode", "Hello, \u{4E16}\u{754C}! \u{1F600}");
        let result = read_entire_file(&file.path).unwrap();
        assert_eq!(result, "Hello, \u{4E16}\u{754C}! \u{1F600}");
    }

    #[test]
    fn test_read_entire_file_not_found() {
        let result = read_entire_file("nonexistent_file_12345.txt");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::NotFound);
    }

    // ==================== count_lines tests ====================

    #[test]
    fn test_count_lines_simple() {
        let file = TestFile::new("lines_simple", "Line 1\nLine 2\nLine 3\n");
        let result = count_lines(&file.path).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_count_lines_no_trailing_newline() {
        let file = TestFile::new("lines_no_trailing", "Line 1\nLine 2\nLine 3");
        let result = count_lines(&file.path).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_count_lines_empty() {
        let file = TestFile::new("lines_empty", "");
        let result = count_lines(&file.path).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_count_lines_single_line() {
        let file = TestFile::new("lines_single", "Single line");
        let result = count_lines(&file.path).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_count_lines_blank_lines() {
        let file = TestFile::new("lines_blank", "Line 1\n\nLine 3\n\n");
        let result = count_lines(&file.path).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_count_lines_not_found() {
        let result = count_lines("nonexistent_file_12345.txt");
        assert!(result.is_err());
    }

    // ==================== count_words tests ====================

    #[test]
    fn test_count_words_simple() {
        let file = TestFile::new("words_simple", "Hello World");
        let result = count_words(&file.path).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_count_words_multiline() {
        let file = TestFile::new("words_multiline", "Hello World\nThis is a test\n");
        let result = count_words(&file.path).unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_count_words_empty() {
        let file = TestFile::new("words_empty", "");
        let result = count_words(&file.path).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_count_words_whitespace_only() {
        let file = TestFile::new("words_whitespace", "   \n\t\n   ");
        let result = count_words(&file.path).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_count_words_extra_whitespace() {
        let file = TestFile::new("words_extra", "  Hello   World  \n  Test  ");
        let result = count_words(&file.path).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_count_words_single_word() {
        let file = TestFile::new("words_single", "Hello");
        let result = count_words(&file.path).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_count_words_not_found() {
        let result = count_words("nonexistent_file_12345.txt");
        assert!(result.is_err());
    }

    // ==================== read_lines tests ====================

    #[test]
    fn test_read_lines_simple() {
        let file = TestFile::new("readlines_simple", "Line 1\nLine 2\nLine 3\n");
        let result = read_lines(&file.path).unwrap();
        assert_eq!(result, vec!["Line 1", "Line 2", "Line 3"]);
    }

    #[test]
    fn test_read_lines_no_trailing_newline() {
        let file = TestFile::new("readlines_no_trailing", "Line 1\nLine 2\nLine 3");
        let result = read_lines(&file.path).unwrap();
        assert_eq!(result, vec!["Line 1", "Line 2", "Line 3"]);
    }

    #[test]
    fn test_read_lines_empty() {
        let file = TestFile::new("readlines_empty", "");
        let result = read_lines(&file.path).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_read_lines_single_line() {
        let file = TestFile::new("readlines_single", "Single line");
        let result = read_lines(&file.path).unwrap();
        assert_eq!(result, vec!["Single line"]);
    }

    #[test]
    fn test_read_lines_preserves_whitespace() {
        let file = TestFile::new("readlines_whitespace", "  Indented\n\tTabbed\n");
        let result = read_lines(&file.path).unwrap();
        assert_eq!(result, vec!["  Indented", "\tTabbed"]);
    }

    #[test]
    fn test_read_lines_blank_lines() {
        let file = TestFile::new("readlines_blank", "Line 1\n\nLine 3\n");
        let result = read_lines(&file.path).unwrap();
        assert_eq!(result, vec!["Line 1", "", "Line 3"]);
    }

    #[test]
    fn test_read_lines_not_found() {
        let result = read_lines("nonexistent_file_12345.txt");
        assert!(result.is_err());
    }

    // ==================== first_n_lines tests ====================

    #[test]
    fn test_first_n_lines_simple() {
        let file = TestFile::new("firstn_simple", "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\n");
        let result = first_n_lines(&file.path, 3).unwrap();
        assert_eq!(result, vec!["Line 1", "Line 2", "Line 3"]);
    }

    #[test]
    fn test_first_n_lines_more_than_file() {
        let file = TestFile::new("firstn_more", "Line 1\nLine 2\n");
        let result = first_n_lines(&file.path, 10).unwrap();
        assert_eq!(result, vec!["Line 1", "Line 2"]);
    }

    #[test]
    fn test_first_n_lines_zero() {
        let file = TestFile::new("firstn_zero", "Line 1\nLine 2\nLine 3\n");
        let result = first_n_lines(&file.path, 0).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_first_n_lines_one() {
        let file = TestFile::new("firstn_one", "First line\nSecond line\n");
        let result = first_n_lines(&file.path, 1).unwrap();
        assert_eq!(result, vec!["First line"]);
    }

    #[test]
    fn test_first_n_lines_empty_file() {
        let file = TestFile::new("firstn_empty", "");
        let result = first_n_lines(&file.path, 5).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_first_n_lines_exact_count() {
        let file = TestFile::new("firstn_exact", "Line 1\nLine 2\nLine 3\n");
        let result = first_n_lines(&file.path, 3).unwrap();
        assert_eq!(result, vec!["Line 1", "Line 2", "Line 3"]);
    }

    #[test]
    fn test_first_n_lines_not_found() {
        let result = first_n_lines("nonexistent_file_12345.txt", 5);
        assert!(result.is_err());
    }

    // ==================== Integration tests ====================

    #[test]
    fn test_integration_read_and_count() {
        let content = "The quick brown fox\njumps over the lazy dog\n";
        let file = TestFile::new("integration", content);

        // All functions should work on the same file
        assert_eq!(read_entire_file(&file.path).unwrap(), content);
        assert_eq!(count_lines(&file.path).unwrap(), 2);
        assert_eq!(count_words(&file.path).unwrap(), 9);
        assert_eq!(
            read_lines(&file.path).unwrap(),
            vec!["The quick brown fox", "jumps over the lazy dog"]
        );
        assert_eq!(
            first_n_lines(&file.path, 1).unwrap(),
            vec!["The quick brown fox"]
        );
    }

    #[test]
    fn test_integration_large_content() {
        // Create content with many lines
        let lines: Vec<String> = (1..=100).map(|i| format!("Line number {}", i)).collect();
        let content = lines.join("\n") + "\n";
        let file = TestFile::new("integration_large", &content);

        assert_eq!(count_lines(&file.path).unwrap(), 100);
        assert_eq!(read_lines(&file.path).unwrap().len(), 100);
        assert_eq!(first_n_lines(&file.path, 10).unwrap().len(), 10);
    }
}
