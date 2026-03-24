#![allow(dead_code, unused_imports, unused_parens)]
// Demonstrates the builder pattern for structs.
//
// The builder pattern is used when a struct has many fields, some optional.
// Instead of a constructor with many parameters, you chain method calls:
//
//   let config = ConfigBuilder::new("app")
//       .port(8080)
//       .debug(true)
//       .build();
//
// Each builder method takes `self` (ownership) and returns `Self`,
// enabling method chaining. The final `.build()` produces the target struct.
//
// Run: cargo run --example s09_builder_pattern

#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    max_connections: u32,
    timeout_secs: u64,
    debug: bool,
}

/// Builder for ServerConfig with sensible defaults.
#[derive(Debug)]
struct ServerConfigBuilder {
    host: String,
    port: u16,
    max_connections: u32,
    timeout_secs: u64,
    debug: bool,
}

impl ServerConfigBuilder {
    /// Starts building with required field `host` and defaults for the rest.
    fn new(host: &str) -> Self {
        ServerConfigBuilder {
            host: host.to_string(),
            port: 8080,
            max_connections: 100,
            timeout_secs: 30,
            debug: false,
        }
    }

    /// Sets the port.
    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Sets max connections.
    fn max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    /// Sets the timeout in seconds.
    fn timeout_secs(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// Enables or disables debug mode.
    fn debug(mut self, enabled: bool) -> Self {
        self.debug = enabled;
        self
    }

    /// Consumes the builder and produces a ServerConfig.
    ///
    /// # Returns
    ///
    /// The final ServerConfig
    fn build(self) -> ServerConfig {
        ServerConfig {
            host: self.host,
            port: self.port,
            max_connections: self.max_connections,
            timeout_secs: self.timeout_secs,
            debug: self.debug,
        }
    }
}

fn main() {
    // --- 1. Builder with all defaults ---
    let default_config = ServerConfigBuilder::new("localhost").build();
    println!("Default: {default_config:#?}");

    // --- 2. Builder with some overrides ---
    let prod_config = ServerConfigBuilder::new("0.0.0.0")
        .port(443)
        .max_connections(10_000)
        .timeout_secs(60)
        .build();
    println!("Prod: {prod_config:#?}");

    // --- 3. Builder with method chaining ---
    let dev_config = ServerConfigBuilder::new("127.0.0.1")
        .debug(true)
        .port(3000)
        .build();
    println!("Dev: {dev_config:#?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let config = ServerConfigBuilder::new("test").build();
        assert_eq!(config.host, "test");
        assert_eq!(config.port, 8080);
        assert_eq!(config.max_connections, 100);
        assert_eq!(config.timeout_secs, 30);
        assert!(!config.debug);
    }

    #[test]
    fn test_overrides() {
        let config = ServerConfigBuilder::new("host")
            .port(9090)
            .debug(true)
            .build();
        assert_eq!(config.port, 9090);
        assert!(config.debug);
        // Non-overridden fields keep defaults:
        assert_eq!(config.max_connections, 100);
    }

    #[test]
    fn test_all_fields() {
        let config = ServerConfigBuilder::new("h")
            .port(1)
            .max_connections(2)
            .timeout_secs(3)
            .debug(true)
            .build();
        assert_eq!(config.host, "h");
        assert_eq!(config.port, 1);
        assert_eq!(config.max_connections, 2);
        assert_eq!(config.timeout_secs, 3);
        assert!(config.debug);
    }
}
