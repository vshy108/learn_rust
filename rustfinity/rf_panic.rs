//! # Rustfinity: Panic
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/panic
//!
//! What we covered in the previous challenges were **recoverable errors**, meaning they can be handled during runtime. However, in Rust we have **unrecoverable errors** that are so severe that the program cannot safely continue and it will shut down completely.
//!
//! You can trigger an unrecoverable error using the `panic!` macro, for example: `panic!("This is an unrecoverable error!")`.
//!
//! In this challenge, you will write a function that retrieves the value of an environment variable named `DATABASE_URL` if the environment variable is not set, you will exit the program using `panic!`, it makes sense to use `panic!` instead of `Result<T, E>` because the program will not be able to run database queries if this value isn't available.
//!
//! Just to make it more interesting, there is another requirement, your function must validate that the value starts with `postgresql://`, this is not a full proof validation for real life applications, but we'll keep it simple for this challenge.
//!
//! ## Your Task
//!
//! Write a function, `get_database_url`, that retrieves the value of the `DATABASE_URL` environment variable and validates it. The function should behave as follows:
//!
//! 1. If the `DATABASE_URL` variable is set and starts with `postgresql://`, return its value as a `String`.
//! 2. If the `DATABASE_URL` variable is not set, the function should terminate the program by calling `panic!` with the exact message `"DATABASE_URL environment variable is not set."`.
//! 3. If the `DATABASE_URL` variable is set but does not start with `postgresql://`, the function should terminate the program by calling `panic!` with the message `"DATABASE_URL must start with 'postgresql://'"`.
//!
//! ## Hints
//!
//! If you're stuck, here are some hints to help you solve the challenge:
//!
//! <details>
//!   <summary>Click here to reveal hints</summary>
//!
//! - Use the `std::env::var()` function to retrieve the value of an environment variable. e.g. `std::env::var("DATABASE_URL")`.
//! - Use the `starts_with` method to check if a string starts with a specific prefix. e.g. `my_string.starts_with("prefix")`.
//!
//! </details>
//!
//! Run: `cargo run --example rf_panic`
//! Test: `cargo test --example rf_panic`
//! Filter: `cargo test --example rf_panic <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub fn get_database_url() -> String {
    // Your code here...
    unimplemented!()
}

fn set_database_url(value: &str) {
    // SAFETY: main is single-threaded, and tests hold ENV_MUTEX while calling this helper.
    unsafe { std::env::set_var("DATABASE_URL", value) };
}

fn remove_database_url() {
    // SAFETY: main is single-threaded, and tests hold ENV_MUTEX while calling this helper.
    unsafe { std::env::remove_var("DATABASE_URL") };
}

/// Example usage
pub fn main() {
    set_database_url("postgresql://localhost");

    let db_url = get_database_url();
    println!("Database URL: {}", db_url);

    remove_database_url(); // Missing variable scenario
    get_database_url();

    set_database_url("mysql://localhost"); // Invalid prefix scenario
    get_database_url();
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::sync::Mutex;

    // All tests mutate the DATABASE_URL env var, which is global process state.
    // Rust runs tests in parallel by default, causing race conditions.
    // This mutex serializes access to prevent flakiness.
    // We use lock_env() to handle mutex poisoning from #[should_panic] tests.
    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    fn lock_env() -> std::sync::MutexGuard<'static, ()> {
        ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
    }

    #[test]
    fn test_get_database_url_valid() {
        let _lock = lock_env();
        set_database_url("postgresql://localhost");
        let db_url = get_database_url();
        assert_eq!(db_url, "postgresql://localhost");
    }

    #[test]
    #[should_panic(expected = "DATABASE_URL environment variable is not set.")]
    fn test_get_database_url_missing() {
        let _lock = lock_env();
        remove_database_url();
        get_database_url();
    }

    #[test]
    #[should_panic(expected = "DATABASE_URL must start with 'postgresql://'")]
    fn test_get_database_url_invalid_prefix() {
        let _lock = lock_env();
        set_database_url("mysql://localhost");
        get_database_url();
    }

    #[test]
    fn test_get_database_url_empty_value() {
        let _lock = lock_env();
        set_database_url("postgresql://");
        let db_url = get_database_url();
        assert_eq!(db_url, "postgresql://");
    }

    #[test]
    fn test_get_database_url_with_long_valid_url() {
        let _lock = lock_env();
        let long_url = "postgresql://user:password@host:5432/database";
        set_database_url(long_url);
        let db_url = get_database_url();
        assert_eq!(db_url, long_url);
    }

    #[test]
    #[should_panic(expected = "DATABASE_URL must start with 'postgresql://'")]
    fn test_get_database_url_prefix_case_sensitive() {
        let _lock = lock_env();
        set_database_url("PostgreSQL://localhost");
        get_database_url();
    }

    #[test]
    fn test_get_database_url_with_special_characters() {
        let _lock = lock_env();
        let special_url = "postgresql://user:pa$$word@host:5432/dbname";
        set_database_url(special_url);
        let db_url = get_database_url();
        assert_eq!(db_url, special_url);
    }

    #[test]
    #[should_panic(expected = "DATABASE_URL must start with 'postgresql://'")]
    fn test_get_database_url_numeric_prefix() {
        let _lock = lock_env();
        set_database_url("123postgresql://localhost");
        get_database_url();
    }

    #[test]
    fn test_get_database_url_edge_case_minimal_valid() {
        let _lock = lock_env();
        set_database_url("postgresql://h");
        let db_url = get_database_url();
        assert_eq!(db_url, "postgresql://h");
    }

    #[test]
    fn test_get_database_url_with_port_only() {
        let _lock = lock_env();
        let port_only_url = "postgresql://:5432";
        set_database_url(port_only_url);
        let db_url = get_database_url();
        assert_eq!(db_url, port_only_url);
    }
}
