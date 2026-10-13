//! # Rustfinity: Stdio Operations
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/stdio-operations
//!
//! Standard I/O (stdio) operations are fundamental for any program that interacts with users or other processes. Rust provides safe, efficient access to standard input, output, and error streams through the `std::io` module.
//!
//! ## Understanding Standard Streams
//!
//! Every process has three standard streams:
//!
//! - **stdin** (standard input): For reading input, typically from the keyboard or piped data
//! - **stdout** (standard output): For normal program output
//! - **stderr** (standard error): For error messages and diagnostics (not buffered by default)
//!
//! ## Reading from stdin
//!
//! Use `std::io::stdin()` to get a handle to standard input:
//!
//! ```rust
//! use std::io::{self, BufRead};
//!
//! let stdin = io::stdin();
//! let mut line = String::new();
//! stdin.read_line(&mut line)?;
//! // line now contains the input including newline
//! ```
//!
//! For reading multiple lines, use the `lines()` iterator from `BufRead`:
//!
//! ```rust
//! use std::io::{self, BufRead};
//!
//! let stdin = io::stdin();
//! for line in stdin.lock().lines() {
//!     let line = line?;
//!     println!("Read: {}", line);
//! }
//! ```
//!
//! ## Writing to stdout and stderr
//!
//! Use `std::io::stdout()` and `std::io::stderr()` for output handles:
//!
//! ```rust
//! use std::io::{self, Write};
//!
//! let mut stdout = io::stdout();
//! writeln!(stdout, "Normal output")?;
//! stdout.flush()?;  // Ensure output is written immediately
//!
//! let mut stderr = io::stderr();
//! writeln!(stderr, "Error message")?;
//! ```
//!
//! ## Buffering and Flushing
//!
//! stdout is line-buffered when connected to a terminal, meaning output is flushed after each newline. When writing to a file or pipe, it's fully buffered. Use `.flush()` to ensure output is written immediately:
//!
//! ```rust
//! use std::io::{self, Write};
//!
//! print!("Enter your name: ");
//! // Important! Without this, prompt might not appear
//! io::stdout().flush()?;
//! ```
//!
//! ## Your Task
//!
//! Implement the following functions for working with standard I/O:
//!
//! 1. `read_line_from_reader<R: BufRead>(reader: R)
//! -> io::Result<String>` - Read a single line from
//!    any reader implementing `BufRead`, trimming the
//!    trailing newline
//! 2. `read_all_lines_from_reader<R: BufRead>(reader: R)
//! -> io::Result<Vec<String>>` - Read all lines from
//!    a reader into a Vec
//! 3. `write_to_writer<W: Write>(writer: &mut W, message: &str) -> io::Result<()>` - Write a message to any writer implementing `Write`
//! 4. `writeln_to_writer<W: Write>(writer: &mut W, message: &str) -> io::Result<()>` - Write a message with a newline to any writer
//! 5. `write_and_flush<W: Write>(writer: &mut W, message: &str) -> io::Result<()>` - Write a message and immediately flush the buffer
//! 6. `write_error_to_writer<W: Write>(writer: &mut W, error: &str) -> io::Result<()>` - Write an error message in the format "[ERROR] message" with a newline
//!
//! Note: The functions are generic over `BufRead` and `Write` traits to make them testable without needing actual stdin/stdout.
//!
//! ## Examples
//!
//! ```rust
//! use std::io::Cursor;
//!
//! // Reading from a mock input
//! let input = Cursor::new("Hello, World!\n");
//! let line = read_line_from_reader(input)?;
//! assert_eq!(line, "Hello, World!");
//!
//! // Reading multiple lines
//! let input = Cursor::new("Line 1\nLine 2\nLine 3\n");
//! let lines = read_all_lines_from_reader(input)?;
//! assert_eq!(
//!     lines,
//!     vec!["Line 1", "Line 2", "Line 3"]
//! );
//!
//! // Writing to a buffer
//! let mut output = Vec::new();
//! write_to_writer(&mut output, "Hello")?;
//! assert_eq!(String::from_utf8(output).unwrap(), "Hello");
//!
//! // Writing with newline
//! let mut output = Vec::new();
//! writeln_to_writer(&mut output, "Hello")?;
//! assert_eq!(
//!     String::from_utf8(output).unwrap(),
//!     "Hello\n"
//! );
//!
//! // Writing an error message
//! let mut output = Vec::new();
//! write_error_to_writer(&mut output, "Something went wrong")?;
//! assert_eq!(
//!     String::from_utf8(output).unwrap(),
//!     "[ERROR] Something went wrong\n"
//! );
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - Use `reader.read_line(&mut buffer)` to read a single line
//! - Use `buffer.trim_end_matches('\n')
//! .trim_end_matches('\r')` to strip line endings
//!   for both Unix and Windows
//! - The `lines()` iterator from `BufRead` automatically strips newlines
//! - Use `write!(writer, "{}", message)` or `writer.write_all(message.as_bytes())` to write
//! - Use `writeln!(writer, "{}", message)` to write with a newline
//! - Call `writer.flush()` to ensure buffered output is written immediately
//! - Format error messages using `format!("[ERROR] {}", error)` or `write!(writer, "[ERROR] {}\n", error)`
//!
//! </details>
//!
//! Run: `cargo run --example rf_stdio_operations`
//! Test: `cargo test --example rf_stdio_operations`
//! Filter: `cargo test --example rf_stdio_operations <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::io::{self, BufRead, Write};

/// Read a single line from any reader implementing `BufRead`.
/// The trailing newline (both `\n` and `\r\n`) is trimmed from the result.
pub fn read_line_from_reader<R: BufRead>(mut reader: R) -> io::Result<String> {
    // TODO: Implement this function
    unimplemented!()
}

/// Read all lines from a reader into a Vec.
/// Each line has its trailing newline removed.
pub fn read_all_lines_from_reader<R: BufRead>(reader: R) -> io::Result<Vec<String>> {
    // TODO: Implement this function
    unimplemented!()
}

/// Write a message to any writer implementing `Write`.
/// Does not add a trailing newline.
pub fn write_to_writer<W: Write>(writer: &mut W, message: &str) -> io::Result<()> {
    // TODO: Implement this function
    unimplemented!()
}

/// Write a message with a trailing newline to any writer implementing `Write`.
pub fn writeln_to_writer<W: Write>(writer: &mut W, message: &str) -> io::Result<()> {
    // TODO: Implement this function
    unimplemented!()
}

/// Write a message and immediately flush the buffer.
/// Useful for prompts or real-time output.
pub fn write_and_flush<W: Write>(writer: &mut W, message: &str) -> io::Result<()> {
    // TODO: Implement this function
    unimplemented!()
}

/// Write an error message in the format "[ERROR] message" with a newline.
pub fn write_error_to_writer<W: Write>(writer: &mut W, error: &str) -> io::Result<()> {
    // TODO: Implement this function
    unimplemented!()
}

pub fn main() {
    use std::io::Cursor;

    println!("=== Stdio Operations Demo ===\n");

    // Demonstrate reading from a mock input
    println!("Reading a single line:");
    let input = Cursor::new("Hello from stdin!\n");
    match read_line_from_reader(input) {
        Ok(line) => println!("  Read: '{}'", line),
        Err(e) => eprintln!("  Error: {}", e),
    }

    // Demonstrate reading multiple lines
    println!("\nReading multiple lines:");
    let input = Cursor::new("First line\nSecond line\nThird line\n");
    match read_all_lines_from_reader(input) {
        Ok(lines) => {
            for (i, line) in lines.iter().enumerate() {
                println!("  Line {}: '{}'", i + 1, line);
            }
        }
        Err(e) => eprintln!("  Error: {}", e),
    }

    // Demonstrate writing to stdout
    println!("\nWriting to stdout:");
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let _ = write_to_writer(&mut handle, "  This is written without newline... ");
    let _ = write_and_flush(&mut handle, "and this is flushed immediately!\n");
    let _ = writeln_to_writer(&mut handle, "  This has an automatic newline");

    // Demonstrate writing to stderr
    println!("\nWriting to stderr:");
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    let _ = write_error_to_writer(&mut handle, "This is an error message");

    println!("\n=== Demo Complete ===");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::io::Cursor;
    // ==================== read_line_from_reader tests ====================

    #[test]
    fn test_read_line_simple() {
        let input = Cursor::new("Hello, World!\n");
        let result = read_line_from_reader(input).unwrap();
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_read_line_no_newline() {
        let input = Cursor::new("No newline at end");
        let result = read_line_from_reader(input).unwrap();
        assert_eq!(result, "No newline at end");
    }

    #[test]
    fn test_read_line_windows_newline() {
        let input = Cursor::new("Windows line\r\n");
        let result = read_line_from_reader(input).unwrap();
        assert_eq!(result, "Windows line");
    }

    #[test]
    fn test_read_line_empty() {
        let input = Cursor::new("");
        let result = read_line_from_reader(input).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_read_line_only_newline() {
        let input = Cursor::new("\n");
        let result = read_line_from_reader(input).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_read_line_with_spaces() {
        let input = Cursor::new("  spaces around  \n");
        let result = read_line_from_reader(input).unwrap();
        assert_eq!(result, "  spaces around  ");
    }

    #[test]
    fn test_read_line_unicode() {
        let input = Cursor::new("Hello, \u{4E16}\u{754C}!\n");
        let result = read_line_from_reader(input).unwrap();
        assert_eq!(result, "Hello, \u{4E16}\u{754C}!");
    }

    #[test]
    fn test_read_line_multiline_only_first() {
        let input = Cursor::new("First line\nSecond line\n");
        let result = read_line_from_reader(input).unwrap();
        assert_eq!(result, "First line");
    }

    // ==================== read_all_lines_from_reader tests ====================

    #[test]
    fn test_read_all_lines_simple() {
        let input = Cursor::new("Line 1\nLine 2\nLine 3\n");
        let result = read_all_lines_from_reader(input).unwrap();
        assert_eq!(result, vec!["Line 1", "Line 2", "Line 3"]);
    }

    #[test]
    fn test_read_all_lines_no_trailing_newline() {
        let input = Cursor::new("Line 1\nLine 2\nLine 3");
        let result = read_all_lines_from_reader(input).unwrap();
        assert_eq!(result, vec!["Line 1", "Line 2", "Line 3"]);
    }

    #[test]
    fn test_read_all_lines_empty() {
        let input = Cursor::new("");
        let result = read_all_lines_from_reader(input).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_read_all_lines_single_line() {
        let input = Cursor::new("Single line\n");
        let result = read_all_lines_from_reader(input).unwrap();
        assert_eq!(result, vec!["Single line"]);
    }

    #[test]
    fn test_read_all_lines_blank_lines() {
        let input = Cursor::new("Line 1\n\nLine 3\n");
        let result = read_all_lines_from_reader(input).unwrap();
        assert_eq!(result, vec!["Line 1", "", "Line 3"]);
    }

    #[test]
    fn test_read_all_lines_windows_newlines() {
        let input = Cursor::new("Line 1\r\nLine 2\r\n");
        let result = read_all_lines_from_reader(input).unwrap();
        assert_eq!(result, vec!["Line 1", "Line 2"]);
    }

    #[test]
    fn test_read_all_lines_unicode() {
        let input =
            Cursor::new("\u{4E2D}\u{6587}\n\u{65E5}\u{672C}\u{8A9E}\n\u{D55C}\u{AD6D}\u{C5B4}\n");
        let result = read_all_lines_from_reader(input).unwrap();
        assert_eq!(
            result,
            vec![
                "\u{4E2D}\u{6587}",
                "\u{65E5}\u{672C}\u{8A9E}",
                "\u{D55C}\u{AD6D}\u{C5B4}"
            ]
        );
    }

    // ==================== write_to_writer tests ====================

    #[test]
    fn test_write_simple() {
        let mut output = Vec::new();
        write_to_writer(&mut output, "Hello").unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "Hello");
    }

    #[test]
    fn test_write_empty() {
        let mut output = Vec::new();
        write_to_writer(&mut output, "").unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "");
    }

    #[test]
    fn test_write_with_newline_in_message() {
        let mut output = Vec::new();
        write_to_writer(&mut output, "Line 1\nLine 2").unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "Line 1\nLine 2");
    }

    #[test]
    fn test_write_unicode() {
        let mut output = Vec::new();
        write_to_writer(&mut output, "\u{1F600} Emoji!").unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "\u{1F600} Emoji!");
    }

    #[test]
    fn test_write_multiple_times() {
        let mut output = Vec::new();
        write_to_writer(&mut output, "Hello, ").unwrap();
        write_to_writer(&mut output, "World!").unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "Hello, World!");
    }

    // ==================== writeln_to_writer tests ====================

    #[test]
    fn test_writeln_simple() {
        let mut output = Vec::new();
        writeln_to_writer(&mut output, "Hello").unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "Hello\n");
    }

    #[test]
    fn test_writeln_empty() {
        let mut output = Vec::new();
        writeln_to_writer(&mut output, "").unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "\n");
    }

    #[test]
    fn test_writeln_multiple_times() {
        let mut output = Vec::new();
        writeln_to_writer(&mut output, "Line 1").unwrap();
        writeln_to_writer(&mut output, "Line 2").unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "Line 1\nLine 2\n");
    }

    #[test]
    fn test_writeln_unicode() {
        let mut output = Vec::new();
        writeln_to_writer(&mut output, "\u{4E16}\u{754C}").unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "\u{4E16}\u{754C}\n");
    }

    // ==================== write_and_flush tests ====================

    #[test]
    fn test_write_and_flush_simple() {
        let mut output = Vec::new();
        write_and_flush(&mut output, "Prompt: ").unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "Prompt: ");
    }

    #[test]
    fn test_write_and_flush_empty() {
        let mut output = Vec::new();
        write_and_flush(&mut output, "").unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "");
    }

    #[test]
    fn test_write_and_flush_multiple() {
        let mut output = Vec::new();
        write_and_flush(&mut output, "Enter name: ").unwrap();
        write_and_flush(&mut output, ">> ").unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "Enter name: >> ");
    }

    // ==================== write_error_to_writer tests ====================

    #[test]
    fn test_write_error_simple() {
        let mut output = Vec::new();
        write_error_to_writer(&mut output, "File not found").unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "[ERROR] File not found\n"
        );
    }

    #[test]
    fn test_write_error_empty() {
        let mut output = Vec::new();
        write_error_to_writer(&mut output, "").unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "[ERROR] \n");
    }

    #[test]
    fn test_write_error_with_details() {
        let mut output = Vec::new();
        write_error_to_writer(&mut output, "Connection refused: timeout after 30s").unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "[ERROR] Connection refused: timeout after 30s\n"
        );
    }

    #[test]
    fn test_write_error_unicode() {
        let mut output = Vec::new();
        write_error_to_writer(&mut output, "\u{9519}\u{8BEF}").unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "[ERROR] \u{9519}\u{8BEF}\n"
        );
    }

    #[test]
    fn test_write_error_multiple() {
        let mut output = Vec::new();
        write_error_to_writer(&mut output, "Error 1").unwrap();
        write_error_to_writer(&mut output, "Error 2").unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "[ERROR] Error 1\n[ERROR] Error 2\n"
        );
    }

    // ==================== Integration tests ====================

    #[test]
    fn test_integration_read_then_echo() {
        // Simulate reading input and echoing it back
        let input = Cursor::new("User input here\n");
        let line = read_line_from_reader(input).unwrap();

        let mut output = Vec::new();
        writeln_to_writer(&mut output, &format!("You entered: {}", line)).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "You entered: User input here\n"
        );
    }

    #[test]
    fn test_integration_prompt_pattern() {
        // Simulate a prompt-and-input pattern
        let mut output = Vec::new();

        // Write prompt without newline and flush
        write_and_flush(&mut output, "Enter your name: ").unwrap();
        assert_eq!(
            String::from_utf8(output.clone()).unwrap(),
            "Enter your name: "
        );

        // Simulate user input
        let input = Cursor::new("Alice\n");
        let name = read_line_from_reader(input).unwrap();

        // Write response
        writeln_to_writer(&mut output, &format!("Hello, {}!", name)).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "Enter your name: Hello, Alice!\n"
        );
    }

    #[test]
    fn test_integration_read_process_write() {
        // Read multiple lines, process them, and write results
        let input = Cursor::new("apple\nbanana\ncherry\n");
        let lines = read_all_lines_from_reader(input).unwrap();

        let mut output = Vec::new();
        for line in lines {
            writeln_to_writer(&mut output, &line.to_uppercase()).unwrap();
        }

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "APPLE\nBANANA\nCHERRY\n"
        );
    }

    #[test]
    fn test_integration_error_handling_pattern() {
        // Simulate a program that writes normal output and errors
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        writeln_to_writer(&mut stdout, "Processing file...").unwrap();
        write_error_to_writer(&mut stderr, "Warning: file is empty").unwrap();
        writeln_to_writer(&mut stdout, "Done.").unwrap();

        assert_eq!(
            String::from_utf8(stdout).unwrap(),
            "Processing file...\nDone.\n"
        );
        assert_eq!(
            String::from_utf8(stderr).unwrap(),
            "[ERROR] Warning: file is empty\n"
        );
    }

    #[test]
    fn test_integration_mixed_write_and_writeln() {
        let mut output = Vec::new();

        write_to_writer(&mut output, "Name: ").unwrap();
        writeln_to_writer(&mut output, "Alice").unwrap();
        write_to_writer(&mut output, "Age: ").unwrap();
        writeln_to_writer(&mut output, "30").unwrap();

        assert_eq!(String::from_utf8(output).unwrap(), "Name: Alice\nAge: 30\n");
    }

    #[test]
    fn test_integration_large_input() {
        // Create a large input with many lines
        let lines: Vec<String> = (1..=100).map(|i| format!("Line {}", i)).collect();
        let content = lines.join("\n") + "\n";
        let input = Cursor::new(content);

        let result = read_all_lines_from_reader(input).unwrap();
        assert_eq!(result.len(), 100);
        assert_eq!(result[0], "Line 1");
        assert_eq!(result[99], "Line 100");
    }
}
