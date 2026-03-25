#![allow(dead_code, unused_imports, unused_parens)]
// Demonstrates custom error types with thiserror.
//
// `thiserror` provides derive macros to easily create custom error types
// that implement `std::error::Error`, `Display`, and `From`.
//
// Run: cargo run --example s26_custom_error_types

use thiserror::Error;

/// Errors that can occur when processing user data.
#[derive(Debug, Error)]
enum UserError {
    /// Name cannot be empty.
    #[error("name cannot be empty")]
    EmptyName,

    /// Age must be between 0 and 150.
    #[error("invalid age: {0} (must be 0-150)")]
    InvalidAge(u32),

    /// Email format is invalid.
    #[error("invalid email: {0}")]
    InvalidEmail(String),

    /// Wraps a parse error.
    #[error("parse error: {0}")]
    ParseError(#[from] std::num::ParseIntError),

    /// Wraps an I/O error.
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}

/// A validated user.
#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u32,
    email: String,
}

/// Validates and creates a user.
///
/// # Errors
///
/// Returns `UserError` if any field is invalid.
fn create_user(name: &str, age_str: &str, email: &str) -> Result<User, UserError> {
    if name.is_empty() {
        return Err(UserError::EmptyName);
    }

    // The ? operator auto-converts ParseIntError -> UserError::ParseError
    let age: u32 = age_str.parse()?;

    if age > 150 {
        return Err(UserError::InvalidAge(age));
    }

    if !email.contains('@') {
        return Err(UserError::InvalidEmail(email.to_string()));
    }

    Ok(User {
        name: name.to_string(),
        age,
        email: email.to_string(),
    })
}

fn main() {
    println!("=== Custom Error Types ===");

    let test_cases = [
        ("Alice", "30", "alice@example.com"),
        ("", "25", "bob@example.com"),
        ("Charlie", "200", "charlie@example.com"),
        ("Dave", "abc", "dave@example.com"),
        ("Eve", "25", "eve-no-at"),
    ];

    for (name, age, email) in &test_cases {
        match create_user(name, age, email) {
            Ok(user) => println!("  OK: {user:?}"),
            Err(e) => println!("  ERR: {e}"),
        }
    }

    // --- Error source chain ---
    let err = create_user("Test", "abc", "test@test.com").unwrap_err();
    println!("error: {err}");
    if let Some(source) = std::error::Error::source(&err) {
        println!("source: {source}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_user() {
        let user = create_user("Alice", "30", "alice@example.com").unwrap();
        assert_eq!(user.name, "Alice");
        assert_eq!(user.age, 30);
    }

    #[test]
    fn test_empty_name() {
        let err = create_user("", "30", "a@b.com").unwrap_err();
        assert!(matches!(err, UserError::EmptyName));
    }

    #[test]
    fn test_invalid_age() {
        let err = create_user("Bob", "200", "b@b.com").unwrap_err();
        assert!(matches!(err, UserError::InvalidAge(200)));
    }

    #[test]
    fn test_parse_error() {
        let err = create_user("C", "abc", "c@c.com").unwrap_err();
        assert!(matches!(err, UserError::ParseError(_)));
    }

    #[test]
    fn test_invalid_email() {
        let err = create_user("D", "25", "no-at").unwrap_err();
        assert!(matches!(err, UserError::InvalidEmail(_)));
    }

    #[test]
    fn test_error_display() {
        let err = UserError::EmptyName;
        assert_eq!(err.to_string(), "name cannot be empty");
    }
}
