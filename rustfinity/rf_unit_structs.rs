//! # Rustfinity: Unit Structs
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/unit-structs
//!
//! In Rust, a **unit struct** is a type of struct that doesn't have any fields. It is essentially a "marker" or "tag" type and is represented by an empty pair of curly braces (`struct Name;`).
//!
//! Unit structs are particularly useful in scenarios where you need to define types without associated data. Here's an example:
//!
//! ```rust
//! pub struct Database;
//! ```
//!
//! ## Your Task
//!
//! In this challenge, you'll implement a unit struct named `Logger`. The struct will serve as a type marker that enables certain logging functionality via a method `log_message`. You'll also implement a `log_message` function that prints a message to the console using the unit struct `Logger` as a marker. The goal is to familiarize yourself with unit structs and their usage as markers.
//!
//! ### Requirements
//!
//! 1. Define a unit struct named `Logger`.
//! 2. Implement a `Logger::log_message` method that:
//!    - Takes a `&str` message as input.
//!    - Prints the message to the console in the format: `[LOG]: {message}`.
//!
//! 3. Use the unit struct `Logger` to call this method.
//!
//! ## Example Test
//!
//! ```rust
//! Logger::log_message("This is a test log.");
//! // Output: [LOG]: This is a test log.
//! ```
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - Define the unit struct as `pub struct Logger;`.
//! - Implement an associated function for the unit struct using the `impl Logger` block.
//! - Use the `println!` macro to format and print the message.
//! - Remember, unit structs do not require any fields and are defined with empty braces.
//!
//! </details>
//!
//! Run: `cargo run --example rf_unit_structs`
//! Test: `cargo test --example rf_unit_structs`
//! Filter: `cargo test --example rf_unit_structs <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

pub struct Logger;

impl Logger {
    pub fn log_message(message: &str) {
        todo!("log message {message}")
    }
}

// Example usage:
pub fn main() {
    Logger::log_message("Hello, World!");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use crate::Logger;
    use std::{fs, mem, process};

    fn create_bin_and_run(code: &str) -> process::Output {
        let bin = "src/main.rs";
        fs::write(bin, code).unwrap();
        let output = process::Command::new("cargo")
            .arg("run")
            .output()
            .expect("Failed to compile");
        fs::remove_file(bin).unwrap();
        output
    }

    #[test]
    fn test_compiles() {
        Logger::log_message("Test message");
    }

    #[test]
    fn test_log_message() {
        let code = r#"
        use crate::Logger;
        pub fn main() {
            Logger::log_message("Test message");
            Logger::log_message("Rust is great");
            Logger::log_message("I love Rust");
            Logger::log_message("Rust is the best");
        }
    "#;

        let output = create_bin_and_run(code);

        let stdout = String::from_utf8(output.stdout).unwrap();

        assert!(stdout.contains("Test message"));
        assert!(stdout.contains("Rust is great"));
        assert!(stdout.contains("I love Rust"));
        assert!(stdout.contains("Rust is the best"));
    }

    #[test]
    fn test_should_be_unit_struct() {
        assert_eq!(
            mem::size_of::<Logger>(),
            0,
            "Logger should be a unit struct"
        );
    }
}
