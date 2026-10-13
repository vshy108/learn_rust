//! # Rustfinity: Macros 2
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/macros-2
//!
//! Now that we have an overview of how macros work, let's dive into another macro challenge. When implementing traits for multiple types, you often end up writing very similar code over and over.
//!
//! In this challenge, you'll create a macro that generates implementations of a trait for multiple types. The goal is to reduce the amount of boilerplate code you need to write.
//!
//! ## Your Task
//!
//! 1. Write a macro named `config_default_impl` that:
//!    - Accepts a type and a default value as arguments.
//!    - Generates the implementation of `ConfigDefault` for that type.
//!
//! 2. Use the macro to implement the `ConfigDefault` trait for the following types:
//!    - `ConnectionTimeout` with a value of `30`.
//!    - `MaxConnections` with a value of `100`.
//!    - `RetryAttempts` with a value of `3`.
//!    - `PostgresPort` with a value of `5432`.
//!    - `MySQLPort` with a value of `3306`.
//!    - `MongoPort` with a value of `27017`.
//!    - `RedisPort` with a value of `6379`.
//!
//! ## Hints
//!
//! If you're stuck, feel free to check out the hints below:
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - The syntax for a macro that accepts a type and a value is as follows:
//!
//!   ```rust
//!   macro_rules! config_default_impl {
//!     ($type:ty, $value:expr) => {
//!         // Implementation here
//!     };
//!   }
//!   ```
//!
//! </details>
//!
//! Run: `cargo run --example rf_macros_2`
//! Test: `cargo test --example rf_macros_2`
//! Filter: `cargo test --example rf_macros_2 <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

pub trait ConfigDefault {
    fn get_default() -> Self;
}

// These types need default values
pub struct ConnectionTimeout(pub u64);
pub struct MaxConnections(pub u32);
pub struct RetryAttempts(pub u8);
pub struct PostgresPort(pub u16);
pub struct MySQLPort(pub u16);
pub struct MongoPort(pub u16);
pub struct RedisPort(pub u16);

#[macro_export]
macro_rules! config_default_impl {
    ($type:ty, $value:expr) => {
        impl ConfigDefault for $type {
            fn get_default() -> Self {
                todo!("return the configured default value")
            }
        }
    };
}

config_default_impl!(ConnectionTimeout, 30);
config_default_impl!(MaxConnections, 100);
config_default_impl!(RetryAttempts, 3);
config_default_impl!(PostgresPort, 5432);
config_default_impl!(MySQLPort, 3306);
config_default_impl!(MongoPort, 27017);
config_default_impl!(RedisPort, 6379);

// Example usage
pub fn main() {
    // let's say we have a new struct
    struct CustomPort(pub u16);

    // we implement the ConfigDefault trait for CustomPort
    config_default_impl!(CustomPort, 8080);

    // when running the `get_default` method, it should return the default value
    assert_eq!(<CustomPort as ConfigDefault>::get_default().0, 8080);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_connection_timeout() {
        assert_eq!(
            <ConnectionTimeout as ConfigDefault>::get_default().0,
            30,
            "Connection timeout should default to 30 seconds"
        );
    }

    #[test]
    fn test_max_connections() {
        assert_eq!(
            <MaxConnections as ConfigDefault>::get_default().0,
            100,
            "Max connections should default to 100"
        );
    }

    #[test]
    fn test_retry_attempts() {
        assert_eq!(
            <RetryAttempts as ConfigDefault>::get_default().0,
            3,
            "Retry attempts should default to 3"
        );
    }

    #[test]
    fn test_database_ports() {
        assert_eq!(
            <PostgresPort as ConfigDefault>::get_default().0,
            5432,
            "Postgres should use default port 5432"
        );
        assert_eq!(
            <MySQLPort as ConfigDefault>::get_default().0,
            3306,
            "MySQL should use default port 3306"
        );
        assert_eq!(
            <MongoPort as ConfigDefault>::get_default().0,
            27017,
            "MongoDB should use default port 27017"
        );
        assert_eq!(
            <RedisPort as ConfigDefault>::get_default().0,
            6379,
            "Redis should use default port 6379"
        );
    }

    #[test]
    fn test_timeout_conversion() {
        let timeout = <ConnectionTimeout as ConfigDefault>::get_default();
        assert_eq!(
            timeout.0 * 1000,
            30000,
            "30 seconds should convert to 30000 milliseconds"
        );
    }

    #[test]
    fn test_ports_are_valid_range() {
        let ports = [
            <PostgresPort as ConfigDefault>::get_default().0,
            <MySQLPort as ConfigDefault>::get_default().0,
            <MongoPort as ConfigDefault>::get_default().0,
            <RedisPort as ConfigDefault>::get_default().0,
        ];

        for port in ports {
            assert!(port > 0, "Port number must be positive");
            assert!(port < 65535, "Port number exceeds valid range");
        }
    }

    #[test]
    fn test_max_connections_reasonable() {
        let max_conn = <MaxConnections as ConfigDefault>::get_default().0;
        assert!(max_conn >= 50, "Max connections should be at least 50");
        assert!(max_conn <= 1000, "Max connections shouldn't exceed 1000");
    }

    #[test]
    fn test_retry_attempts_bounds() {
        let retries = <RetryAttempts as ConfigDefault>::get_default().0;
        assert!(retries > 0, "Should have at least 1 retry");
        assert!(retries <= 10, "Shouldn't have excessive retries");
    }

    // Test configuration composition
    struct DatabaseConfig {
        timeout: ConnectionTimeout,
        max_conn: MaxConnections,
        retries: RetryAttempts,
        port: PostgresPort,
    }

    impl DatabaseConfig {
        fn new_with_defaults() -> Self {
            Self {
                timeout: ConfigDefault::get_default(),
                max_conn: ConfigDefault::get_default(),
                retries: ConfigDefault::get_default(),
                port: ConfigDefault::get_default(),
            }
        }
    }

    #[test]
    fn test_config_composition() {
        let config = DatabaseConfig::new_with_defaults();
        assert_eq!(
            config.timeout.0, 30,
            "Composed config should have default timeout"
        );
        assert_eq!(
            config.max_conn.0, 100,
            "Composed config should have default max connections"
        );
        assert_eq!(
            config.retries.0, 3,
            "Composed config should have default retry attempts"
        );
        assert_eq!(
            config.port.0, 5432,
            "Composed config should have default Postgres port"
        );
    }

    #[test]
    fn test_all_ports_different() {
        let ports = [
            <PostgresPort as ConfigDefault>::get_default().0,
            <MySQLPort as ConfigDefault>::get_default().0,
            <MongoPort as ConfigDefault>::get_default().0,
            <RedisPort as ConfigDefault>::get_default().0,
        ];

        // Check that all ports are unique
        for i in 0..ports.len() {
            for j in i + 1..ports.len() {
                assert_ne!(
                    ports[i], ports[j],
                    "Port values must be unique - found duplicate: {}",
                    ports[i]
                );
            }
        }
    }

    // Use the macro to do more impl
    struct CustomPort(pub u16);

    config_default_impl!(CustomPort, 8080);

    #[test]
    fn test_custom_port() {
        assert_eq!(
            <CustomPort as ConfigDefault>::get_default().0,
            8080,
            "Custom port should default to 8080"
        );
    }
}
