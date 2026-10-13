//! # Rustfinity: String Building
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/string-building
//!
//! Building strings efficiently is a common task in Rust. Whether you're generating reports, creating log messages, or formatting output for display, Rust provides several powerful tools for constructing strings.
//!
//! ## String Building Techniques
//!
//! ### The `format!` Macro
//!
//! The `format!` macro is the most straightforward way to build formatted strings. It works like `println!` but returns a `String` instead of printing:
//!
//! ```rust
//! let name = "Alice";
//! let age = 30;
//! let greeting = format!(
//!     "Hello, {}! You are {} years old.",
//!     name,
//!     age
//! );
//! assert_eq!(greeting, "Hello, Alice! You are 30 years old.");
//! ```
//!
//! ### The `write!` Macro
//!
//! For building strings incrementally, `write!` is more efficient than repeated concatenation. It writes directly into a `String` (or any type implementing `std::fmt::Write`):
//!
//! ```rust
//! use std::fmt::Write;
//!
//! let mut output = String::new();
//! write!(output, "Name: {}", "Bob").unwrap();
//! write!(output, ", Age: {}", 25).unwrap();
//! assert_eq!(output, "Name: Bob, Age: 25");
//! ```
//!
//! ### The `Display` Trait
//!
//! Implementing `Display` for your types lets them work with `format!`, `write!`, and `println!`:
//!
//! ```rust
//! use std::fmt::{Display, Formatter, Result};
//!
//! struct Point { x: i32, y: i32 }
//!
//! impl Display for Point {
//!     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//!         write!(f, "({}, {})", self.x, self.y)
//!     }
//! }
//!
//! let p = Point { x: 3, y: 4 };
//! assert_eq!(format!("{}", p), "(3, 4)");
//! ```
//!
//! ### String Concatenation
//!
//! For simple cases, you can use `+` operator or `push_str`:
//!
//! ```rust
//! let mut s = String::from("Hello");
//! s.push_str(", World!");
//! assert_eq!(s, "Hello, World!");
//!
//! // Or using +
//! let s1 = String::from("Hello, ");
//! let s2 = "World!";
//! let s3 = s1 + s2;  // Note: s1 is moved here
//! assert_eq!(s3, "Hello, World!");
//! ```
//!
//! ## Your Task
//!
//! Implement the following:
//!
//! 1. `build_greeting(name: &str, age: u32) -> String` - Build a greeting message using `format!`
//! 2. `build_list(items: &[&str]) -> String` - Build a numbered list (1. first\n2. second\n...) using `write!`
//! 3. `Person` struct with `name: String` and `age: u32` - Implement `Display` to format as "Name (Age years old)"
//! 4. `build_table(headers: &[&str], rows: &[Vec<String>]) -> String` - Build a simple text table with headers and data rows
//! 5. `concat_with_separator(parts: &[&str], sep: &str) -> String` - Join strings with a separator (implement without using `.join()`)
//!
//! ## Examples
//!
//! ```rust
//! // build_greeting
//! assert_eq!(
//!     build_greeting("Alice", 30),
//!     "Hello, Alice! You are 30 years old."
//! );
//!
//! // build_list
//! assert_eq!(
//!     build_list(&["apple", "banana", "cherry"]),
//!     "1. apple\n2. banana\n3. cherry"
//! );
//!
//! // Person Display
//! let person = Person {
//!     name: "Bob".to_string(),
//!     age: 25
//! };
//! assert_eq!(format!("{}", person), "Bob (25 years old)");
//!
//! // build_table
//! let headers = vec!["Name", "Age"];
//! let rows = vec![
//!     vec!["Alice".to_string(), "30".to_string()],
//!     vec!["Bob".to_string(), "25".to_string()],
//! ];
//! let table = build_table(&headers, &rows);
//! // Returns:
//! // | Name  | Age |
//! // |-------|-----|
//! // | Alice | 30  |
//! // | Bob   | 25  |
//!
//! // concat_with_separator
//! assert_eq!(
//!     concat_with_separator(&["a", "b", "c"], ", "),
//!     "a, b, c"
//! );
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - For `build_greeting`, use `format!("{} ... {}", name, age)`
//! - For `build_list`, use `std::fmt::Write` and iterate with `.enumerate()`
//! - For `Person`'s `Display`, implement `fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`
//! - For `build_table`, calculate column widths first, then use `write!` with padding specifiers like `{:<width$}`
//! - For `concat_with_separator`, iterate through items and conditionally add the separator
//!
//! </details>
//!
//! Run: `cargo run --example rf_string_building`
//! Test: `cargo test --example rf_string_building`
//! Filter: `cargo test --example rf_string_building <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::fmt::{self, Display, Formatter, Write};

/// Build a greeting message using format!
pub fn build_greeting(name: &str, age: u32) -> String {
    // TODO: Use format!() to create a greeting
    // Format: "Hello, {name}! You are {age} years old."
    unimplemented!()
}

/// Build a numbered list from items using write!
pub fn build_list(items: &[&str]) -> String {
    // TODO: Create a numbered list using std::fmt::Write
    // Format: "1. item1\n2. item2\n3. item3"
    unimplemented!()
}

/// A person with a name and age.
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

// TODO: Implement Display for Person
// Format: "Name (Age years old)"
impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // TODO: Write the person's info in the format
        unimplemented!()
    }
}

/// Build a simple text table with headers and data rows.
///
/// The table format uses pipes and dashes:
/// ```text
/// | Name  | Age |
/// |-------|-----|
/// | Alice | 30  |
/// | Bob   | 25  |
/// ```
pub fn build_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    // TODO: Build a formatted table
    // Steps:
    // 1. Calculate column widths (max of header and cell values)
    // 2. Build header row with pipes: | Header1 | Header2 |
    // 3. Build separator row: |---------|---------|
    // 4. Build each data row: | Value1  | Value2  |
    unimplemented!()
}

/// Concatenate strings with a separator without using .join()
pub fn concat_with_separator(parts: &[&str], sep: &str) -> String {
    // TODO: Join the parts with the separator
    // Don't use the .join() method - implement it manually
    unimplemented!()
}

pub fn main() {
    // Demonstrate build_greeting
    println!("=== build_greeting ===");
    println!("{}", build_greeting("Alice", 30));

    // Demonstrate build_list
    println!("\n=== build_list ===");
    println!("{}", build_list(&["apple", "banana", "cherry"]));

    // Demonstrate Person Display
    println!("\n=== Person Display ===");
    let person = Person {
        name: "Bob".to_string(),
        age: 25,
    };
    println!("{}", person);

    // Demonstrate build_table
    println!("\n=== build_table ===");
    let headers = &["Name", "Age"];
    let rows = vec![
        vec!["Alice".to_string(), "30".to_string()],
        vec!["Bob".to_string(), "25".to_string()],
    ];
    println!("{}", build_table(headers, &rows));

    // Demonstrate concat_with_separator
    println!("\n=== concat_with_separator ===");
    println!("{}", concat_with_separator(&["a", "b", "c"], ", "));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    // ============ build_greeting tests ============

    #[test]
    fn greeting_basic() {
        assert_eq!(
            build_greeting("Alice", 30),
            "Hello, Alice! You are 30 years old."
        );
    }

    #[test]
    fn greeting_different_name() {
        assert_eq!(
            build_greeting("Bob", 25),
            "Hello, Bob! You are 25 years old."
        );
    }

    #[test]
    fn greeting_zero_age() {
        assert_eq!(
            build_greeting("Baby", 0),
            "Hello, Baby! You are 0 years old."
        );
    }

    #[test]
    fn greeting_large_age() {
        assert_eq!(
            build_greeting("Elder", 100),
            "Hello, Elder! You are 100 years old."
        );
    }

    #[test]
    fn greeting_empty_name() {
        assert_eq!(build_greeting("", 42), "Hello, ! You are 42 years old.");
    }

    // ============ build_list tests ============

    #[test]
    fn list_three_items() {
        assert_eq!(
            build_list(&["apple", "banana", "cherry"]),
            "1. apple\n2. banana\n3. cherry"
        );
    }

    #[test]
    fn list_single_item() {
        assert_eq!(build_list(&["only"]), "1. only");
    }

    #[test]
    fn list_empty() {
        assert_eq!(build_list(&[]), "");
    }

    #[test]
    fn list_two_items() {
        assert_eq!(build_list(&["first", "second"]), "1. first\n2. second");
    }

    #[test]
    fn list_many_items() {
        let items = &["a", "b", "c", "d", "e"];
        let expected = "1. a\n2. b\n3. c\n4. d\n5. e";
        assert_eq!(build_list(items), expected);
    }

    #[test]
    fn list_with_spaces() {
        assert_eq!(
            build_list(&["hello world", "foo bar"]),
            "1. hello world\n2. foo bar"
        );
    }

    // ============ Person Display tests ============

    #[test]
    fn person_display_basic() {
        let person = Person {
            name: "Bob".to_string(),
            age: 25,
        };
        assert_eq!(format!("{}", person), "Bob (25 years old)");
    }

    #[test]
    fn person_display_zero_age() {
        let person = Person {
            name: "Baby".to_string(),
            age: 0,
        };
        assert_eq!(format!("{}", person), "Baby (0 years old)");
    }

    #[test]
    fn person_display_long_name() {
        let person = Person {
            name: "Alexander Hamilton".to_string(),
            age: 47,
        };
        assert_eq!(format!("{}", person), "Alexander Hamilton (47 years old)");
    }

    #[test]
    fn person_in_format_string() {
        let person = Person {
            name: "Alice".to_string(),
            age: 30,
        };
        let message = format!("Welcome, {}!", person);
        assert_eq!(message, "Welcome, Alice (30 years old)!");
    }

    #[test]
    fn person_to_string() {
        let person = Person {
            name: "Charlie".to_string(),
            age: 35,
        };
        let s = person.to_string();
        assert_eq!(s, "Charlie (35 years old)");
    }

    // ============ build_table tests ============

    #[test]
    fn table_basic() {
        let headers = &["Name", "Age"];
        let rows = vec![
            vec!["Alice".to_string(), "30".to_string()],
            vec!["Bob".to_string(), "25".to_string()],
        ];
        let table = build_table(headers, &rows);

        assert!(table.contains("| Name"));
        assert!(table.contains("| Age"));
        assert!(table.contains("| Alice"));
        assert!(table.contains("| 30"));
        assert!(table.contains("| Bob"));
        assert!(table.contains("| 25"));
        assert!(table.contains("|---"));
    }

    #[test]
    fn table_single_row() {
        let headers = &["Item"];
        let rows = vec![vec!["Test".to_string()]];
        let table = build_table(headers, &rows);

        assert!(table.contains("| Item"));
        assert!(table.contains("| Test"));
    }

    #[test]
    fn table_empty_rows() {
        let headers = &["A", "B"];
        let rows: Vec<Vec<String>> = vec![];
        let table = build_table(headers, &rows);

        assert!(table.contains("| A"));
        assert!(table.contains("| B"));
        assert!(table.contains("|---"));
    }

    #[test]
    fn table_empty_headers() {
        let headers: &[&str] = &[];
        let rows: Vec<Vec<String>> = vec![];
        let table = build_table(headers, &rows);
        assert_eq!(table, "");
    }

    #[test]
    fn table_column_width_from_data() {
        let headers = &["N", "Name"];
        let rows = vec![vec!["1".to_string(), "Christopher".to_string()]];
        let table = build_table(headers, &rows);

        // Christopher is longer than "Name", so column should accommodate it
        assert!(table.contains("Christopher"));
    }

    #[test]
    fn table_three_columns() {
        let headers = &["ID", "Name", "Score"];
        let rows = vec![
            vec!["1".to_string(), "Alice".to_string(), "95".to_string()],
            vec!["2".to_string(), "Bob".to_string(), "87".to_string()],
        ];
        let table = build_table(headers, &rows);

        assert!(table.contains("| ID"));
        assert!(table.contains("| Name"));
        assert!(table.contains("| Score"));
        assert!(table.contains("| 1"));
        assert!(table.contains("| Alice"));
        assert!(table.contains("| 95"));
    }

    #[test]
    fn table_alignment_check() {
        let headers = &["A", "BB"];
        let rows = vec![
            vec!["x".to_string(), "yy".to_string()],
            vec!["xx".to_string(), "y".to_string()],
        ];
        let table = build_table(headers, &rows);

        // Table should have consistent formatting with pipes
        let lines: Vec<&str> = table.lines().collect();
        assert!(lines.len() >= 3); // Header, separator, at least one row
        for line in lines {
            assert!(line.starts_with('|'));
            assert!(line.ends_with('|'));
        }
    }

    // ============ concat_with_separator tests ============

    #[test]
    fn concat_basic() {
        assert_eq!(concat_with_separator(&["a", "b", "c"], ", "), "a, b, c");
    }

    #[test]
    fn concat_single_item() {
        assert_eq!(concat_with_separator(&["hello"], "-"), "hello");
    }

    #[test]
    fn concat_empty() {
        assert_eq!(concat_with_separator(&[], ", "), "");
    }

    #[test]
    fn concat_two_items() {
        assert_eq!(concat_with_separator(&["foo", "bar"], " - "), "foo - bar");
    }

    #[test]
    fn concat_empty_separator() {
        assert_eq!(concat_with_separator(&["a", "b", "c"], ""), "abc");
    }

    #[test]
    fn concat_longer_separator() {
        assert_eq!(concat_with_separator(&["x", "y"], " <=> "), "x <=> y");
    }

    #[test]
    fn concat_with_newlines() {
        assert_eq!(
            concat_with_separator(&["line1", "line2", "line3"], "\n"),
            "line1\nline2\nline3"
        );
    }

    #[test]
    fn concat_with_empty_strings() {
        assert_eq!(concat_with_separator(&["", "", ""], ","), ",,");
    }

    #[test]
    fn concat_words() {
        assert_eq!(
            concat_with_separator(&["hello", "world", "rust"], " "),
            "hello world rust"
        );
    }
}
