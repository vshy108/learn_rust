//! # Rustfinity: Unwrap And Expect
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/unwrap-and-expect
//!
//! Rust provides two methods for handling unrecoverable errors `unwrap` and `expect`, they can be used on both `Result` and `Option` types, they will trigger a `panic!` if the value is an `Err(E)` variant for `Result<T, E>` or a `None` variant for `Option<T>`
//!
//! These methods extract the inner value but terminate the program if the operation fails. They are particularly useful for quick prototyping or situations where an error is truly unrecoverable.
//!
//! In this challenge, you will interact with file operations to demonstrate the use of `unwrap` and `expect`. Instead of directly working with `Result` or `Option`, you will use standard library functions that return these types and handle their results using `unwrap` and `expect`.
//!
//! ## Your Task
//!
//! Implement the following functions:
//!
//! 1. **`read_file_to_string`**: This function takes a file path as input, attempts to read its contents, and returns the contents as a `String`. Use `expect` to handle any file I/O errors with a custom error message of exactly `Failed to read file: <path>`.
//! 2. **`get_env_variable`**: This function retrieves the value of an environment variable by name. Use `unwrap` to panic if the variable is not set.
//!
//! ### Notes
//!
//! - `expect` provides a way to add context to your panics, which can help with debugging.
//! - `unwrap` is more concise but less descriptive when errors occur.
//!
//! ## Hints
//!
//! If you're stuck, here are some hints to help you solve the challenge:
//!
//! <details>
//!   <summary>Click here to reveal hints</summary>
//!
//! - Use `std::fs::read_to_string(path)` to read the contents of a file. It returns a `Result<String, std::io::Error>`.
//! - Use `std::env::var(key)` to retrieve an environment variable. It returns a `Result<String, std::env::VarError>`.
//! - Use `.expect()` to add a custom error message when handling a `Result`.
//! - Use `.unwrap()` for a quick, less descriptive error handling method.
//!
//! </details>
//!
//! Run: `cargo run --example rf_unwrap_and_expect`
//! Test: `cargo test --example rf_unwrap_and_expect`
//! Filter: `cargo test --example rf_unwrap_and_expect <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub fn read_file_to_string(path: &str) -> String {
    // 1. Implement the function
    unimplemented!()
}

pub fn get_env_variable(key: &str) -> String {
    // 2. Implement the function
    unimplemented!()
}

/// Example usage
pub fn main() {
    // Example 1: Using read_file_to_string
    let file_content = read_file_to_string("example.txt");
    println!("File content: {}", file_content);

    // Example 2: Using get_env_variable
    // SAFETY: this standalone example has not started any other threads.
    unsafe { std::env::set_var("EXAMPLE_KEY", "example_value") };
    let value = get_env_variable("EXAMPLE_KEY");
    println!("Environment variable value: {}", value);

    // Must panic
    read_file_to_string("nonexistent.txt");
    get_env_variable("MISSING_KEY");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::env;
    use std::fs;

    static ENVIRONMENT_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn lock_environment() -> std::sync::MutexGuard<'static, ()> {
        ENVIRONMENT_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn set_test_variable(key: &str, value: &str) {
        // SAFETY: callers hold ENVIRONMENT_LOCK while accessing environment variables.
        unsafe { env::set_var(key, value) };
    }

    fn remove_test_variable(key: &str) {
        // SAFETY: callers hold ENVIRONMENT_LOCK while accessing environment variables.
        unsafe { env::remove_var(key) };
    }

    #[test]
    fn test_read_file_to_string_success() {
        let path = "test_file.txt";
        fs::write(path, "Test content").expect("Failed to create test file");
        let content = read_file_to_string(path);
        assert_eq!(content, "Test content");
        fs::remove_file(path).expect("Failed to remove test file");
    }

    #[test]
    #[should_panic(expected = "Failed to read file: nonexistent.txt")]
    fn test_read_file_to_string_panic() {
        read_file_to_string("nonexistent.txt");
    }

    #[test]
    fn test_read_file_to_string_empty_file() {
        let path = "empty_file.txt";
        fs::write(path, "").expect("Failed to create test file");
        let content = read_file_to_string(path);
        assert_eq!(content, "");
        fs::remove_file(path).expect("Failed to remove test file");
    }

    #[test]
    fn test_read_file_to_string_special_characters() {
        let path = "special_chars_file.txt";
        let content = "Line1\nLine2\tSpecial!@#$%^&*()";
        fs::write(path, content).expect("Failed to create test file");
        let result = read_file_to_string(path);
        assert_eq!(result, content);
        fs::remove_file(path).expect("Failed to remove test file");
    }

    #[test]
    fn test_get_env_variable_success() {
        let _environment_guard = lock_environment();
        set_test_variable("TEST_ENV_KEY", "TestValue");
        let value = get_env_variable("TEST_ENV_KEY");
        assert_eq!(value, "TestValue");
        remove_test_variable("TEST_ENV_KEY");
    }

    #[test]
    #[should_panic]
    fn test_get_env_variable_panic() {
        let _environment_guard = lock_environment();
        get_env_variable("MISSING_ENV_KEY");
    }

    #[test]
    fn test_get_env_variable_empty_value() {
        let _environment_guard = lock_environment();
        set_test_variable("EMPTY_ENV_KEY", "");
        let value = get_env_variable("EMPTY_ENV_KEY");
        assert_eq!(value, "");
        remove_test_variable("EMPTY_ENV_KEY");
    }

    #[test]
    fn test_get_env_variable_special_characters() {
        let _environment_guard = lock_environment();
        let key = "SPECIAL_KEY";
        let value = "Special!@#$%^&*()";
        set_test_variable(key, value);
        let result = get_env_variable(key);
        assert_eq!(result, value);
        remove_test_variable(key);
    }

    #[test]
    #[should_panic]
    fn test_read_file_to_string_after_deletion() {
        let path = "delete_test.txt";
        fs::write(path, "Temporary content").expect("Failed to create test file");
        fs::remove_file(path).expect("Failed to remove test file");
        read_file_to_string(path); // This should panic
    }

    #[test]
    #[should_panic]
    fn test_get_env_variable_removed_after_set() {
        let _environment_guard = lock_environment();
        let key = "TEMP_ENV_KEY";
        set_test_variable(key, "TemporaryValue");
        remove_test_variable(key);
        get_env_variable(key); // This should panic
    }
}
