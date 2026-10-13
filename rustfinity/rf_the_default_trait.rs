//! # Rustfinity: The Default Trait
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/the-default-trait
//!
//! The `Default` trait is a powerful tool for reducing code repetition and managing complex configurations. When dealing with structs that have many fields, manually specifying every value can be tedious and error-prone. The `Default` trait, combined with the `..Default::default()` syntax, provides an elegant solution to this problem.
//!
//! ## Why Use Default?
//!
//! Consider a scenario where you have a configuration struct with 8 fields, but you only want to customize 1 or 2 of them. Without `Default`, you would need to write:
//!
//! ```rust
//! let config = AppConfig {
//!     theme: String::from("Dark"),
//!     notifications_enabled: true,
//!     max_users: 100,
//!     auto_save: true,
//!     cache_size_mb: 512,
//!     log_level: String::from("INFO"),
//!     retry_attempts: 3,
//!     timeout_seconds: 30,
//! };
//! ```
//!
//! With `Default`, you can simply write:
//!
//! ```rust
//! let config = AppConfig {
//!     theme: String::from("Dark"),
//!     ..Default::default()
//! };
//! ```
//!
//! This not only reduces code repetition but also makes your code more maintainable and less prone to errors.
//!
//! ## Your Task
//!
//! In this challenge, you will implement the `Default` trait for an `AppConfig` struct that has several configuration options. The struct should have the following fields and default values:
//!
//! - `theme` (String): Default value `"Light"`
//! - `notifications_enabled` (bool): Default value `true`
//! - `max_users` (u32): Default value `100`
//! - `auto_save` (bool): Default value `true`
//! - `cache_size_mb` (u32): Default value `512`
//! - `log_level` (String): Default value `"INFO"`
//! - `retry_attempts` (u32): Default value `3`
//! - `timeout_seconds` (u32): Default value `30`
//!
//! Your task is to:
//!
//! 1. Define the `AppConfig` struct with all specified fields
//! 2. Implement the `Default` trait manually
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - Use `String::from()` for string values in the default implementation
//! - Remember that `..Default::default()` must come last in struct initialization
//! - Consider how this pattern would scale with even more configuration options
//! - Think about maintainability when default values need to change
//!
//! </details>
//!
//! Run: `cargo run --example rf_the_default_trait`
//! Test: `cargo test --example rf_the_default_trait`
//! Filter: `cargo test --example rf_the_default_trait <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

#[derive(Debug)]
pub struct AppConfig {
    pub theme: String,
    pub notifications_enabled: bool,
    pub max_users: u32,
    pub auto_save: bool,
    pub cache_size_mb: u32,
    pub log_level: String,
    pub retry_attempts: u32,
    pub timeout_seconds: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        todo!("create a default AppConfig")
    }
}

// Example usage
pub fn main() {
    // Create a default configuration
    let default_config = AppConfig::default();
    println!("Default Config: {:?}", default_config);

    // Create a custom configuration using ..Default::default()
    let custom_config = AppConfig {
        theme: String::from("Dark"),
        ..Default::default()
    };
    println!("Custom Config: {:?}", custom_config);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.theme, "Light");
        assert!(config.notifications_enabled);
        assert_eq!(config.max_users, 100);
    }

    #[test]
    fn test_custom_config() {
        let config = AppConfig {
            theme: String::from("Dark"),
            ..Default::default()
        };
        assert_eq!(config.theme, "Dark");
        assert!(config.notifications_enabled);
        assert_eq!(config.max_users, 100);
    }

    #[test]
    fn test_notifications_only() {
        let config = AppConfig {
            notifications_enabled: false,
            ..Default::default()
        };
        assert_eq!(config.theme, "Light");
        assert!(!config.notifications_enabled);
        assert_eq!(config.max_users, 100);
    }

    #[test]
    fn test_max_users_only() {
        let config = AppConfig {
            max_users: 500,
            ..Default::default()
        };
        assert_eq!(config.theme, "Light");
        assert!(config.notifications_enabled);
        assert_eq!(config.max_users, 500);
    }

    #[test]
    fn test_multiple_custom_fields() {
        let config = AppConfig {
            theme: String::from("Dark"),
            max_users: 250,
            ..Default::default()
        };
        assert_eq!(config.theme, "Dark");
        assert!(config.notifications_enabled);
        assert_eq!(config.max_users, 250);
    }

    #[test]
    fn test_all_custom_fields() {
        let config = AppConfig {
            theme: String::from("Blue"),
            notifications_enabled: false,
            max_users: 1000,
            auto_save: false,
            cache_size_mb: 256,
            log_level: String::from("WARN"),
            retry_attempts: 5,
            timeout_seconds: 45,
        };
        assert_eq!(config.theme, "Blue");
        assert!(!config.notifications_enabled);
        assert_eq!(config.max_users, 1000);
        assert!(!config.auto_save);
        assert_eq!(config.cache_size_mb, 256);
        assert_eq!(config.log_level, "WARN");
        assert_eq!(config.retry_attempts, 5);
        assert_eq!(config.timeout_seconds, 45);
    }

    #[test]
    fn test_minimal_changes() {
        let config = AppConfig {
            cache_size_mb: 1024,
            timeout_seconds: 60,
            ..Default::default()
        };
        assert_eq!(config.cache_size_mb, 1024);
        assert_eq!(config.timeout_seconds, 60);
        assert_eq!(config.theme, "Light");
        assert_eq!(config.log_level, "INFO");
        assert_eq!(config.retry_attempts, 3);
        assert!(config.auto_save);
    }

    #[test]
    fn test_production_settings() {
        let config = AppConfig {
            log_level: String::from("ERROR"),
            cache_size_mb: 2048,
            retry_attempts: 5,
            timeout_seconds: 120,
            ..Default::default()
        };
        assert_eq!(config.log_level, "ERROR");
        assert_eq!(config.cache_size_mb, 2048);
        assert_eq!(config.retry_attempts, 5);
        assert_eq!(config.timeout_seconds, 120);
        assert!(config.notifications_enabled);
    }

    #[test]
    fn test_debug_settings() {
        let config = AppConfig {
            log_level: String::from("DEBUG"),
            cache_size_mb: 256,
            timeout_seconds: 10,
            auto_save: false,
            ..Default::default()
        };
        assert_eq!(config.log_level, "DEBUG");
        assert_eq!(config.cache_size_mb, 256);
        assert_eq!(config.timeout_seconds, 10);
        assert!(!config.auto_save);
        assert_eq!(config.retry_attempts, 3);
    }
}
