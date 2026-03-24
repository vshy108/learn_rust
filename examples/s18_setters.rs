// Demonstrates setters in traits.
//
// Setters provide write access through &mut self methods.
// Combine with getters for a full property interface.
//
// Run: cargo run --example s18_setters

/// A trait with both getters and setters.
trait Config {
    fn host(&self) -> &str;
    fn port(&self) -> u16;
    fn set_host(&mut self, host: &str);
    fn set_port(&mut self, port: u16);

    /// Default setter that parses "host:port".
    fn set_address(&mut self, address: &str) {
        if let Some((host, port_str)) = address.split_once(':') {
            self.set_host(host);
            if let Ok(port) = port_str.parse::<u16>() {
                self.set_port(port);
            }
        }
    }
}

struct ServerConfig {
    host: String,
    port: u16,
}

impl ServerConfig {
    fn new() -> Self {
        Self {
            host: "localhost".into(),
            port: 8080,
        }
    }
}

impl Config for ServerConfig {
    fn host(&self) -> &str {
        &self.host
    }
    fn port(&self) -> u16 {
        self.port
    }
    fn set_host(&mut self, host: &str) {
        self.host = host.to_string();
    }
    fn set_port(&mut self, port: u16) {
        self.port = port;
    }
}

/// Builder pattern using setter trait (fluent interface).
trait Configurable: Config + Sized {
    fn with_host(mut self, host: &str) -> Self {
        self.set_host(host);
        self
    }

    fn with_port(mut self, port: u16) -> Self {
        self.set_port(port);
        self
    }
}

// Blanket implementation: every Config gets Configurable for free.
impl<T: Config> Configurable for T {}

fn main() {
    // Traditional setters:
    let mut cfg = ServerConfig::new();
    println!("default: {}:{}", cfg.host(), cfg.port());

    cfg.set_host("0.0.0.0");
    cfg.set_port(3000);
    println!("after set: {}:{}", cfg.host(), cfg.port());

    // Default setter method:
    cfg.set_address("example.com:443");
    println!("after address: {}:{}", cfg.host(), cfg.port());

    // Fluent builder pattern:
    let cfg2 = ServerConfig::new().with_host("prod.server").with_port(9090);
    println!("builder: {}:{}", cfg2.host(), cfg2.port());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setters() {
        let mut c = ServerConfig::new();
        c.set_host("test");
        c.set_port(1234);
        assert_eq!(c.host(), "test");
        assert_eq!(c.port(), 1234);
    }

    #[test]
    fn test_set_address() {
        let mut c = ServerConfig::new();
        c.set_address("host:5555");
        assert_eq!(c.host(), "host");
        assert_eq!(c.port(), 5555);
    }

    #[test]
    fn test_builder() {
        let c = ServerConfig::new().with_host("h").with_port(99);
        assert_eq!(c.host(), "h");
        assert_eq!(c.port(), 99);
    }
}
