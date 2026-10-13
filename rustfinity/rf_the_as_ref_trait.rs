//! # Rustfinity: The As Ref Trait
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/the-as-ref-trait
//!
//! Working with strings is a common requirement in real-world applications. Sometimes, you want a function that can take either a `String` or a `&str` as input without needing separate implementations. Rust's `AsRef` trait provides a simple way to borrow data as a reference in a generic manner, enabling functions to work with various types that can be represented as a reference to a common type.
//!
//! The `AsRef` trait is a powerful standard library trait used for cheap conversions to a reference of another type. For example, it can be used to create a function that works with both owned (`String`) and borrowed (`&str`) string types. This approach is lightweight and avoids unnecessary allocations.
//!
//! ## Your Task
//!
//! Implement a function `print_message` that accepts any type that implements the `AsRef<str>` trait. This function should:
//!
//! - Accept a single argument of a generic type that implements `AsRef<str>`.
//! - Borrow the input as a string slice (`&str`) and print it to the console using `println!`.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - The `AsRef` trait provides a `.as_ref()` method to borrow a reference.
//! - Both `String` and `&str` implement `AsRef<str>`, so they can be used interchangeably with this trait.
//!
//! </details>
//!
//! Run: `cargo run --example rf_the_as_ref_trait`
//! Test: `cargo test --example rf_the_as_ref_trait`
//! Filter: `cargo test --example rf_the_as_ref_trait <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

pub fn print_message<T: AsRef<str>>(message: T) {
    todo!("print message {}", message.as_ref())
}

// Example usage
pub fn main() {
    // Example 1: Using a &str
    print_message("Hello, world!");

    // Example 2: Using a String
    let greeting = String::from("Welcome to Rust!");
    print_message(greeting);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use crate::print_message;
    use std::{fs, process};

    #[test]
    fn test_with_str() {
        print_message("This is a &str.");
    }

    #[test]
    fn test_with_string() {
        let message = String::from("This is a String.");
        print_message(message);
    }

    #[test]
    fn test_with_empty_string() {
        print_message("");
    }

    #[test]
    fn test_with_special_characters() {
        print_message("Special characters: !@#$%^&*()");
    }

    #[test]
    fn test_with_whitespace_only() {
        print_message("    ");
    }

    #[test]
    fn test_with_numeric_string() {
        print_message("1234567890");
    }

    #[test]
    fn test_with_unicode_characters() {
        print_message("こんにちは、世界! 🌏"); // "Hello, World!" in Japanese
    }

    #[test]
    fn test_with_long_string() {
        let long_string = "Rust".repeat(1000); // Creates a very long string
        print_message(long_string);
    }

    #[test]
    fn test_with_escape_sequences() {
        print_message("Line 1\nLine 2\tIndented");
    }

    #[test]
    #[ignore = "This test launches Cargo from its original package and may mutate the root src/main.rs."]
    fn test_with_multiline_string() {
        let multiline = r#"This is
a multi-line
string."#;
        print_message(multiline);
    }

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
    fn test_should_print_the_right_message() {
        let code = r#"
        use crate::print_message;

        fn main() {
            print_message("Hello, world!");
        }
    "#;

        let output = create_bin_and_run(code);

        let output_str = String::from_utf8(output.stdout).unwrap();

        assert_eq!(output_str, "Hello, world!\n");
    }
}
