//! # Rustfinity: Env Variables
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/env-variables
//!
//! The `std::env` module provides functions for inspecting and manipulating the process's environment. This includes reading and setting environment variables, accessing command-line arguments, and querying information about the current execution context like the current directory and executable path.
//!
//! Environment variables are key-value pairs that configure how programs behave. They're commonly used for configuration that varies between environments (development vs production), storing sensitive information (API keys, database URLs), and communicating between parent and child processes. Rust provides safe, cross-platform APIs for working with these variables.
//!
//! ## Key Functions
//!
//! ```rust
//! use std::env;
//!
//! // Reading environment variables
//! let path = env::var("PATH");
//! // Result<String, VarError>
//! let home = env::var("HOME").ok();
//! // Option<String>
//!
//! // Check if a variable exists (returns OsString)
//! let exists = env::var_os("MY_VAR");
//! // Option<OsString>
//!
//! // Setting environment variables
//! // (affects only current process)
//! env::set_var("MY_VAR", "my_value");
//! env::remove_var("MY_VAR");
//!
//! // Command-line arguments
//! let args: Vec<String> = env::args().collect();
//! let program_name = &args[0];
//!
//! // Current directory
//! let cwd = env::current_dir();
//! // Result<PathBuf, Error>
//!
//! // Executable path
//! let exe = env::current_exe();
//! // Result<PathBuf, Error>
//! ```
//!
//! ## Error Handling
//!
//! `env::var()` returns a `Result` because the variable might not exist or might contain invalid Unicode:
//!
//! ```rust
//! use std::env;
//!
//! match env::var("DATABASE_URL") {
//!     Ok(url) => println!("Database URL: {}", url),
//!     Err(env::VarError::NotPresent) => {
//!         println!("DATABASE_URL not set")
//!     }
//!     Err(env::VarError::NotUnicode(_)) => {
//!         println!("DATABASE_URL contains invalid UTF-8")
//!     }
//! }
//!
//! // For optional variables, use .ok() or .unwrap_or()
//! let debug = env::var("DEBUG")
//!     .unwrap_or_else(|_| String::from("false"));
//! ```
//!
//! ## Your Task
//!
//! Implement the following functions to work with environment variables and process information:
//!
//! **1. `get_env_var(key: &str) -> Option<String>`**
//!
//! Get an environment variable by key. Return `None` if it
//! doesn't exist or contains invalid Unicode.
//!
//! **2. `get_env_var_or_default(key: &str, default: &str) -> String`**
//!
//! Get an environment variable, returning a default value
//! if it doesn't exist.
//!
//! **3. `get_multiple_env_vars(keys: &[&str]) -> Vec<(String, Option<String>)>`**
//!
//! Get multiple environment variables at once. Return a
//! vector of (key, value) pairs where value is `None` if
//! the variable doesn't exist.
//!
//! **4. `parse_env_var<T: std::str::FromStr>(key: &str) -> Option<T>`**
//!
//! Get an environment variable and parse it to type T.
//! Return `None` if the variable doesn't exist or can't
//! be parsed.
//!
//! **5. `get_args() -> Vec<String>`**
//!
//! Get all command-line arguments as a vector of strings.
//!
//! **6. `get_current_dir() -> Option<String>`**
//!
//! Get the current working directory as a string. Return
//! `None` if it can't be determined or converted to UTF-8.
//!
//! **7. `get_current_exe_name() -> Option<String>`**
//!
//! Get the name of the current executable (just the file
//! name, not the full path). Return `None` if it can't
//! be determined.
//!
//! **8. `env_var_is_set(key: &str) -> bool`**
//!
//! Check if an environment variable is set (even if it's
//! an empty string).
//!
//! ## Examples
//!
//! ```rust
//! use std::env;
//!
//! // Set up some test variables
//! env::set_var("MY_APP_PORT", "8080");
//! env::set_var("MY_APP_DEBUG", "true");
//! env::set_var("MY_APP_NAME", "test-app");
//!
//! // Basic retrieval
//! assert_eq!(
//!     get_env_var("MY_APP_NAME"),
//!     Some("test-app".to_string())
//! );
//! assert_eq!(get_env_var("NONEXISTENT"), None);
//!
//! // With default
//! assert_eq!(
//!     get_env_var_or_default("MY_APP_NAME", "default"),
//!     "test-app".to_string()
//! );
//! assert_eq!(
//!     get_env_var_or_default("NONEXISTENT", "default"),
//!     "default".to_string()
//! );
//!
//! // Multiple variables
//! let vars = get_multiple_env_vars(&[
//!     "MY_APP_PORT",
//!     "MY_APP_DEBUG",
//!     "NONEXISTENT",
//! ]);
//! assert_eq!(vars.len(), 3);
//! assert_eq!(
//!     vars[0],
//!     ("MY_APP_PORT".to_string(), Some("8080".to_string()))
//! );
//! assert_eq!(vars[2], ("NONEXISTENT".to_string(), None));
//!
//! // Parsing
//! let port: Option<u16> = parse_env_var("MY_APP_PORT");
//! assert_eq!(port, Some(8080));
//!
//! let debug: Option<bool> = parse_env_var("MY_APP_DEBUG");
//! assert_eq!(debug, Some(true));
//!
//! let missing: Option<i32> = parse_env_var("NONEXISTENT");
//! assert_eq!(missing, None);
//!
//! // Arguments (first is typically the program name)
//! let args = get_args();
//! assert!(!args.is_empty());
//!
//! // Current directory
//! let cwd = get_current_dir();
//! assert!(cwd.is_some());
//!
//! // Check if set
//! assert!(env_var_is_set("MY_APP_PORT"));
//! assert!(!env_var_is_set("DEFINITELY_NOT_SET_12345"));
//!
//! // Clean up
//! env::remove_var("MY_APP_PORT");
//! env::remove_var("MY_APP_DEBUG");
//! env::remove_var("MY_APP_NAME");
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - Use `std::env::var()` which returns `Result<String, VarError>`
//! - Convert `Result` to `Option` using `.ok()`
//! - For parsing, chain `.ok()?.parse().ok()` to handle both missing and parse errors
//! - `std::env::args()` returns an iterator, use `.collect()` to get a `Vec<String>`
//! - `std::env::current_dir()` returns `Result<PathBuf, Error>`
//! - Use `.to_str()` or `.to_string_lossy()` to convert `PathBuf` to string
//! - `std::env::current_exe()` gives the full path; use `.file_name()` to get just the name
//! - `std::env::var_os()` returns `Option<OsString>` and can
//!   detect if a variable is set regardless of content
//!
//! </details>
//!
//! Run: `cargo run --example rf_env_variables`
//! Test: `cargo test --example rf_env_variables`
//! Filter: `cargo test --example rf_env_variables <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::str::FromStr;

/// Gets an environment variable by key.
///
/// Returns `None` if the variable doesn't exist or contains invalid Unicode.
pub fn get_env_var(key: &str) -> Option<String> {
    // TODO: Use env::var() and convert the Result to Option
    unimplemented!()
}

/// Gets an environment variable, returning a default value if it doesn't exist.
pub fn get_env_var_or_default(key: &str, default: &str) -> String {
    // TODO: Use env::var() with unwrap_or_else to provide a default
    unimplemented!()
}

/// Gets multiple environment variables at once.
///
/// Returns a vector of (key, value) pairs where value is `None` if the variable doesn't exist.
pub fn get_multiple_env_vars(keys: &[&str]) -> Vec<(String, Option<String>)> {
    // TODO: Iterate over keys and collect (key, value) pairs
    unimplemented!()
}

/// Gets an environment variable and parses it to type T.
///
/// Returns `None` if the variable doesn't exist or can't be parsed.
pub fn parse_env_var<T: FromStr>(key: &str) -> Option<T> {
    // TODO: Get the env var, then parse it
    unimplemented!()
}

/// Gets all command-line arguments as a vector of strings.
pub fn get_args() -> Vec<String> {
    // TODO: Use env::args() and collect into a Vec
    unimplemented!()
}

/// Gets the current working directory as a string.
///
/// Returns `None` if the current directory can't be determined or converted to UTF-8.
pub fn get_current_dir() -> Option<String> {
    // TODO: Use env::current_dir() and convert PathBuf to String
    unimplemented!()
}

/// Gets the name of the current executable (just the file name, not the full path).
///
/// Returns `None` if the executable path can't be determined.
pub fn get_current_exe_name() -> Option<String> {
    // TODO: Use env::current_exe() to get the path
    unimplemented!()
}

/// Checks if an environment variable is set.
///
/// Returns `true` if the variable is set, even if it's an empty string.
pub fn env_var_is_set(key: &str) -> bool {
    // TODO: Use env::var_os() which returns Option<OsString>
    unimplemented!()
}

fn set_main_env_var(key: &str, value: &str) {
    // SAFETY: the standalone example does not create threads while mutating its environment.
    unsafe { std::env::set_var(key, value) };
}

fn remove_main_env_var(key: &str) {
    // SAFETY: the standalone example does not create threads while mutating its environment.
    unsafe { std::env::remove_var(key) };
}

// Example usage
pub fn main() {
    // Set some test environment variables
    set_main_env_var("MY_APP_NAME", "test-app");
    set_main_env_var("MY_APP_PORT", "8080");
    set_main_env_var("MY_APP_DEBUG", "true");

    // Test basic retrieval
    println!("App name: {:?}", get_env_var("MY_APP_NAME"));
    println!("Missing var: {:?}", get_env_var("NONEXISTENT"));

    // Test with default
    println!(
        "With default: {}",
        get_env_var_or_default("MY_APP_NAME", "default")
    );
    println!(
        "Missing with default: {}",
        get_env_var_or_default("NONEXISTENT", "default")
    );

    // Test multiple variables
    let vars = get_multiple_env_vars(&["MY_APP_NAME", "MY_APP_PORT", "NONEXISTENT"]);
    println!("Multiple vars: {:?}", vars);

    // Test parsing
    let port: Option<u16> = parse_env_var("MY_APP_PORT");
    println!("Parsed port: {:?}", port);

    let debug: Option<bool> = parse_env_var("MY_APP_DEBUG");
    println!("Parsed debug: {:?}", debug);

    // Test arguments
    let args = get_args();
    println!("Args: {:?}", args);

    // Test current directory
    println!("Current dir: {:?}", get_current_dir());

    // Test executable name
    println!("Exe name: {:?}", get_current_exe_name());

    // Test is_set
    println!("MY_APP_NAME is set: {}", env_var_is_set("MY_APP_NAME"));
    println!("NONEXISTENT is set: {}", env_var_is_set("NONEXISTENT"));

    // Clean up
    remove_main_env_var("MY_APP_NAME");
    remove_main_env_var("MY_APP_PORT");
    remove_main_env_var("MY_APP_DEBUG");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::env;

    static ENVIRONMENT_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn lock_environment() -> std::sync::MutexGuard<'static, ()> {
        ENVIRONMENT_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn set_test_env_var(key: &str, value: &str) {
        // SAFETY: each test holds ENVIRONMENT_LOCK for its entire environment interaction.
        unsafe { env::set_var(key, value) };
    }

    fn remove_test_env_var(key: &str) {
        // SAFETY: each test holds ENVIRONMENT_LOCK for its entire environment interaction.
        unsafe { env::remove_var(key) };
    }

    // Helper to generate unique test variable names to avoid conflicts between tests
    fn test_var(name: &str) -> String {
        format!("ENV_VAR_TEST_{}", name)
    }

    // ==================== get_env_var tests ====================

    #[test]
    fn test_get_env_var_exists() {
        let _environment_guard = lock_environment();
        let key = test_var("GET_EXISTS");
        set_test_env_var(&key, "test_value");
        assert_eq!(get_env_var(&key), Some("test_value".to_string()));
        remove_test_env_var(&key);
    }

    #[test]
    fn test_get_env_var_not_exists() {
        let _environment_guard = lock_environment();
        let key = test_var("NONEXISTENT_12345");
        remove_test_env_var(&key); // Ensure it doesn't exist
        assert_eq!(get_env_var(&key), None);
    }

    #[test]
    fn test_get_env_var_empty_value() {
        let _environment_guard = lock_environment();
        let key = test_var("EMPTY_VALUE");
        set_test_env_var(&key, "");
        assert_eq!(get_env_var(&key), Some("".to_string()));
        remove_test_env_var(&key);
    }

    #[test]
    fn test_get_env_var_with_spaces() {
        let _environment_guard = lock_environment();
        let key = test_var("WITH_SPACES");
        set_test_env_var(&key, "  hello world  ");
        assert_eq!(get_env_var(&key), Some("  hello world  ".to_string()));
        remove_test_env_var(&key);
    }

    #[test]
    fn test_get_env_var_with_special_chars() {
        let _environment_guard = lock_environment();
        let key = test_var("SPECIAL_CHARS");
        set_test_env_var(&key, "hello=world&foo=bar");
        assert_eq!(get_env_var(&key), Some("hello=world&foo=bar".to_string()));
        remove_test_env_var(&key);
    }

    #[test]
    fn test_get_env_var_unicode() {
        let _environment_guard = lock_environment();
        let key = test_var("UNICODE");
        set_test_env_var(&key, "Hello, \u{4e16}\u{754c}"); // Hello, 世界
        assert_eq!(get_env_var(&key), Some("Hello, 世界".to_string()));
        remove_test_env_var(&key);
    }

    // ==================== get_env_var_or_default tests ====================

    #[test]
    fn test_get_env_var_or_default_exists() {
        let _environment_guard = lock_environment();
        let key = test_var("DEFAULT_EXISTS");
        set_test_env_var(&key, "actual_value");
        assert_eq!(
            get_env_var_or_default(&key, "default"),
            "actual_value".to_string()
        );
        remove_test_env_var(&key);
    }

    #[test]
    fn test_get_env_var_or_default_not_exists() {
        let _environment_guard = lock_environment();
        let key = test_var("DEFAULT_NOT_EXISTS");
        remove_test_env_var(&key);
        assert_eq!(
            get_env_var_or_default(&key, "default_value"),
            "default_value".to_string()
        );
    }

    #[test]
    fn test_get_env_var_or_default_empty_value() {
        let _environment_guard = lock_environment();
        let key = test_var("DEFAULT_EMPTY");
        set_test_env_var(&key, "");
        // Empty string is a valid value, not the default
        assert_eq!(get_env_var_or_default(&key, "default"), "".to_string());
        remove_test_env_var(&key);
    }

    #[test]
    fn test_get_env_var_or_default_empty_default() {
        let _environment_guard = lock_environment();
        let key = test_var("EMPTY_DEFAULT");
        remove_test_env_var(&key);
        assert_eq!(get_env_var_or_default(&key, ""), "".to_string());
    }

    #[test]
    fn test_get_env_var_or_default_complex_default() {
        let _environment_guard = lock_environment();
        let key = test_var("COMPLEX_DEFAULT");
        remove_test_env_var(&key);
        assert_eq!(
            get_env_var_or_default(&key, "postgresql://localhost:5432/db"),
            "postgresql://localhost:5432/db".to_string()
        );
    }

    // ==================== get_multiple_env_vars tests ====================

    #[test]
    fn test_get_multiple_env_vars_all_exist() {
        let _environment_guard = lock_environment();
        let key_a = test_var("MULTI_A");
        let key_b = test_var("MULTI_B");
        set_test_env_var(&key_a, "value_a");
        set_test_env_var(&key_b, "value_b");

        let vars = get_multiple_env_vars(&[&key_a, &key_b]);
        assert_eq!(vars.len(), 2);
        assert_eq!(vars[0], (key_a.clone(), Some("value_a".to_string())));
        assert_eq!(vars[1], (key_b.clone(), Some("value_b".to_string())));

        remove_test_env_var(&key_a);
        remove_test_env_var(&key_b);
    }

    #[test]
    fn test_get_multiple_env_vars_some_missing() {
        let _environment_guard = lock_environment();
        let key_a = test_var("MULTI_SOME_A");
        let key_b = test_var("MULTI_SOME_B");
        set_test_env_var(&key_a, "value_a");
        remove_test_env_var(&key_b);

        let vars = get_multiple_env_vars(&[&key_a, &key_b]);
        assert_eq!(vars.len(), 2);
        assert_eq!(vars[0], (key_a.clone(), Some("value_a".to_string())));
        assert_eq!(vars[1], (key_b.clone(), None));

        remove_test_env_var(&key_a);
    }

    #[test]
    fn test_get_multiple_env_vars_all_missing() {
        let _environment_guard = lock_environment();
        let key_a = test_var("MULTI_MISSING_A");
        let key_b = test_var("MULTI_MISSING_B");
        remove_test_env_var(&key_a);
        remove_test_env_var(&key_b);

        let vars = get_multiple_env_vars(&[&key_a, &key_b]);
        assert_eq!(vars.len(), 2);
        assert_eq!(vars[0], (key_a.clone(), None));
        assert_eq!(vars[1], (key_b.clone(), None));
    }

    #[test]
    fn test_get_multiple_env_vars_empty_slice() {
        let _environment_guard = lock_environment();
        let vars = get_multiple_env_vars(&[]);
        assert!(vars.is_empty());
    }

    #[test]
    fn test_get_multiple_env_vars_single() {
        let _environment_guard = lock_environment();
        let key = test_var("MULTI_SINGLE");
        set_test_env_var(&key, "single_value");

        let vars = get_multiple_env_vars(&[&key]);
        assert_eq!(vars.len(), 1);
        assert_eq!(vars[0], (key.clone(), Some("single_value".to_string())));

        remove_test_env_var(&key);
    }

    #[test]
    fn test_get_multiple_env_vars_preserves_order() {
        let _environment_guard = lock_environment();
        let key_c = test_var("MULTI_ORDER_C");
        let key_a = test_var("MULTI_ORDER_A");
        let key_b = test_var("MULTI_ORDER_B");
        set_test_env_var(&key_c, "c");
        set_test_env_var(&key_a, "a");
        set_test_env_var(&key_b, "b");

        // Order should match input order, not alphabetical
        let vars = get_multiple_env_vars(&[&key_c, &key_a, &key_b]);
        assert_eq!(vars[0].0, key_c);
        assert_eq!(vars[1].0, key_a);
        assert_eq!(vars[2].0, key_b);

        remove_test_env_var(&key_c);
        remove_test_env_var(&key_a);
        remove_test_env_var(&key_b);
    }

    // ==================== parse_env_var tests ====================

    #[test]
    fn test_parse_env_var_u16() {
        let _environment_guard = lock_environment();
        let key = test_var("PARSE_U16");
        set_test_env_var(&key, "8080");
        let port: Option<u16> = parse_env_var(&key);
        assert_eq!(port, Some(8080));
        remove_test_env_var(&key);
    }

    #[test]
    fn test_parse_env_var_bool_true() {
        let _environment_guard = lock_environment();
        let key = test_var("PARSE_BOOL_TRUE");
        set_test_env_var(&key, "true");
        let val: Option<bool> = parse_env_var(&key);
        assert_eq!(val, Some(true));
        remove_test_env_var(&key);
    }

    #[test]
    fn test_parse_env_var_bool_false() {
        let _environment_guard = lock_environment();
        let key = test_var("PARSE_BOOL_FALSE");
        set_test_env_var(&key, "false");
        let val: Option<bool> = parse_env_var(&key);
        assert_eq!(val, Some(false));
        remove_test_env_var(&key);
    }

    #[test]
    fn test_parse_env_var_i32_negative() {
        let _environment_guard = lock_environment();
        let key = test_var("PARSE_I32_NEG");
        set_test_env_var(&key, "-42");
        let val: Option<i32> = parse_env_var(&key);
        assert_eq!(val, Some(-42));
        remove_test_env_var(&key);
    }

    #[test]
    fn test_parse_env_var_f64() {
        let _environment_guard = lock_environment();
        let key = test_var("PARSE_F64");
        set_test_env_var(&key, "3.14159");
        let val: Option<f64> = parse_env_var(&key);
        assert!((val.unwrap() - 3.14159).abs() < 0.00001);
        remove_test_env_var(&key);
    }

    #[test]
    fn test_parse_env_var_missing() {
        let _environment_guard = lock_environment();
        let key = test_var("PARSE_MISSING");
        remove_test_env_var(&key);
        let val: Option<i32> = parse_env_var(&key);
        assert_eq!(val, None);
    }

    #[test]
    fn test_parse_env_var_invalid_format() {
        let _environment_guard = lock_environment();
        let key = test_var("PARSE_INVALID");
        set_test_env_var(&key, "not_a_number");
        let val: Option<i32> = parse_env_var(&key);
        assert_eq!(val, None);
        remove_test_env_var(&key);
    }

    #[test]
    fn test_parse_env_var_overflow() {
        let _environment_guard = lock_environment();
        let key = test_var("PARSE_OVERFLOW");
        set_test_env_var(&key, "999999999999999999999");
        let val: Option<u32> = parse_env_var(&key);
        assert_eq!(val, None);
        remove_test_env_var(&key);
    }

    #[test]
    fn test_parse_env_var_string() {
        let _environment_guard = lock_environment();
        let key = test_var("PARSE_STRING");
        set_test_env_var(&key, "hello");
        let val: Option<String> = parse_env_var(&key);
        assert_eq!(val, Some("hello".to_string()));
        remove_test_env_var(&key);
    }

    #[test]
    fn test_parse_env_var_char() {
        let _environment_guard = lock_environment();
        let key = test_var("PARSE_CHAR");
        set_test_env_var(&key, "x");
        let val: Option<char> = parse_env_var(&key);
        assert_eq!(val, Some('x'));
        remove_test_env_var(&key);
    }

    #[test]
    fn test_parse_env_var_char_multiple_fails() {
        let _environment_guard = lock_environment();
        let key = test_var("PARSE_CHAR_MULTI");
        set_test_env_var(&key, "xyz");
        let val: Option<char> = parse_env_var(&key);
        assert_eq!(val, None); // Can't parse multiple chars as single char
        remove_test_env_var(&key);
    }

    // ==================== get_args tests ====================

    #[test]
    fn test_get_args_not_empty() {
        let _environment_guard = lock_environment();
        let args = get_args();
        // There should always be at least the program name
        assert!(!args.is_empty());
    }

    #[test]
    fn test_get_args_first_is_string() {
        let _environment_guard = lock_environment();
        let args = get_args();
        // First argument should be the executable path/name
        assert!(!args[0].is_empty());
    }

    // ==================== get_current_dir tests ====================

    #[test]
    fn test_get_current_dir_exists() {
        let _environment_guard = lock_environment();
        let cwd = get_current_dir();
        assert!(cwd.is_some());
    }

    #[test]
    fn test_get_current_dir_not_empty() {
        let _environment_guard = lock_environment();
        let cwd = get_current_dir();
        assert!(!cwd.unwrap().is_empty());
    }

    #[test]
    fn test_get_current_dir_is_valid_path() {
        let _environment_guard = lock_environment();
        let cwd = get_current_dir().unwrap();
        // Should be an absolute path (starts with / on Unix or drive letter on Windows)
        #[cfg(unix)]
        assert!(cwd.starts_with('/'));
        #[cfg(windows)]
        assert!(cwd.chars().nth(1) == Some(':'));
    }

    // ==================== get_current_exe_name tests ====================

    #[test]
    fn test_get_current_exe_name_exists() {
        let _environment_guard = lock_environment();
        let exe_name = get_current_exe_name();
        assert!(exe_name.is_some());
    }

    #[test]
    fn test_get_current_exe_name_not_empty() {
        let _environment_guard = lock_environment();
        let exe_name = get_current_exe_name();
        assert!(!exe_name.unwrap().is_empty());
    }

    #[test]
    fn test_get_current_exe_name_no_path_separator() {
        let _environment_guard = lock_environment();
        let exe_name = get_current_exe_name().unwrap();
        // Should be just the file name, no path separators
        assert!(!exe_name.contains('/'));
        #[cfg(windows)]
        assert!(!exe_name.contains('\\'));
    }

    // ==================== env_var_is_set tests ====================

    #[test]
    fn test_env_var_is_set_true() {
        let _environment_guard = lock_environment();
        let key = test_var("IS_SET_TRUE");
        set_test_env_var(&key, "some_value");
        assert!(env_var_is_set(&key));
        remove_test_env_var(&key);
    }

    #[test]
    fn test_env_var_is_set_false() {
        let _environment_guard = lock_environment();
        let key = test_var("IS_SET_FALSE");
        remove_test_env_var(&key);
        assert!(!env_var_is_set(&key));
    }

    #[test]
    fn test_env_var_is_set_empty_string() {
        let _environment_guard = lock_environment();
        let key = test_var("IS_SET_EMPTY");
        set_test_env_var(&key, "");
        // Empty string still counts as set
        assert!(env_var_is_set(&key));
        remove_test_env_var(&key);
    }

    #[test]
    fn test_env_var_is_set_after_remove() {
        let _environment_guard = lock_environment();
        let key = test_var("IS_SET_REMOVE");
        set_test_env_var(&key, "temp");
        assert!(env_var_is_set(&key));
        remove_test_env_var(&key);
        assert!(!env_var_is_set(&key));
    }

    // ==================== Integration tests ====================

    #[test]
    fn test_integration_set_get_parse() {
        let _environment_guard = lock_environment();
        let key = test_var("INTEGRATION_1");
        set_test_env_var(&key, "42");

        // Should be set
        assert!(env_var_is_set(&key));

        // Should get the value
        assert_eq!(get_env_var(&key), Some("42".to_string()));

        // Should parse correctly
        let val: Option<i32> = parse_env_var(&key);
        assert_eq!(val, Some(42));

        // Default should return actual value
        assert_eq!(get_env_var_or_default(&key, "0"), "42".to_string());

        remove_test_env_var(&key);
    }

    #[test]
    fn test_integration_multiple_vars_workflow() {
        let _environment_guard = lock_environment();
        let host_key = test_var("INT_HOST");
        let port_key = test_var("INT_PORT");
        let debug_key = test_var("INT_DEBUG");

        set_test_env_var(&host_key, "localhost");
        set_test_env_var(&port_key, "3000");
        // debug not set

        let vars = get_multiple_env_vars(&[&host_key, &port_key, &debug_key]);

        let host = vars.iter().find(|(k, _)| k == &host_key).unwrap().1.clone();
        let port = vars.iter().find(|(k, _)| k == &port_key).unwrap().1.clone();
        let debug = vars
            .iter()
            .find(|(k, _)| k == &debug_key)
            .unwrap()
            .1
            .clone();

        assert_eq!(host, Some("localhost".to_string()));
        assert_eq!(port, Some("3000".to_string()));
        assert_eq!(debug, None);

        remove_test_env_var(&host_key);
        remove_test_env_var(&port_key);
    }

    #[test]
    fn test_integration_config_pattern() {
        let _environment_guard = lock_environment();
        // Simulate a typical configuration pattern
        let app_name = test_var("APP_NAME");
        let app_port = test_var("APP_PORT");
        let app_debug = test_var("APP_DEBUG");
        let app_timeout = test_var("APP_TIMEOUT");

        set_test_env_var(&app_name, "my_app");
        set_test_env_var(&app_port, "8080");
        set_test_env_var(&app_debug, "true");
        // timeout not set, will use default

        // Build config from env vars
        let name = get_env_var_or_default(&app_name, "unnamed");
        let port: u16 = parse_env_var(&app_port).unwrap_or(3000);
        let debug: bool = parse_env_var(&app_debug).unwrap_or(false);
        let timeout: u64 = parse_env_var(&app_timeout).unwrap_or(30);

        assert_eq!(name, "my_app");
        assert_eq!(port, 8080);
        assert!(debug);
        assert_eq!(timeout, 30); // default

        remove_test_env_var(&app_name);
        remove_test_env_var(&app_port);
        remove_test_env_var(&app_debug);
    }

    #[test]
    fn test_integration_environment_info() {
        let _environment_guard = lock_environment();
        // Test getting process/environment info
        let cwd = get_current_dir();
        let exe = get_current_exe_name();
        let args = get_args();

        // All should succeed
        assert!(cwd.is_some());
        assert!(exe.is_some());
        assert!(!args.is_empty());

        // cwd should be a directory that exists (we're running in it)
        let cwd_path = std::path::Path::new(cwd.as_ref().unwrap());
        assert!(cwd_path.is_absolute());
    }

    #[test]
    fn test_integration_var_lifecycle() {
        let _environment_guard = lock_environment();
        let key = test_var("LIFECYCLE");

        // Initially not set
        assert!(!env_var_is_set(&key));
        assert_eq!(get_env_var(&key), None);

        // Set it
        set_test_env_var(&key, "first");
        assert!(env_var_is_set(&key));
        assert_eq!(get_env_var(&key), Some("first".to_string()));

        // Update it
        set_test_env_var(&key, "second");
        assert!(env_var_is_set(&key));
        assert_eq!(get_env_var(&key), Some("second".to_string()));

        // Remove it
        remove_test_env_var(&key);
        assert!(!env_var_is_set(&key));
        assert_eq!(get_env_var(&key), None);
    }

    #[test]
    fn test_integration_path_like_value() {
        let _environment_guard = lock_environment();
        let key = test_var("PATH_VALUE");
        let path_value = "/usr/local/bin:/usr/bin:/bin";
        set_test_env_var(&key, path_value);

        assert_eq!(get_env_var(&key), Some(path_value.to_string()));

        // Can still get it with multiple vars
        let vars = get_multiple_env_vars(&[&key]);
        assert_eq!(vars[0].1, Some(path_value.to_string()));

        remove_test_env_var(&key);
    }

    #[test]
    fn test_integration_url_value() {
        let _environment_guard = lock_environment();
        let key = test_var("URL_VALUE");
        let url = "https://user:pass@example.com:8080/path?query=value#fragment";
        set_test_env_var(&key, url);

        assert_eq!(get_env_var(&key), Some(url.to_string()));
        assert_eq!(get_env_var_or_default(&key, ""), url.to_string());

        remove_test_env_var(&key);
    }
}
