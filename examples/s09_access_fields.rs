// Demonstrates accessing struct fields.
//
// Use dot notation: `instance.field_name`.
// For tuple structs: `instance.0`, `instance.1`, etc.
//
// You can also destructure structs with pattern matching.
//
// Run: cargo run --example s09_access_fields

#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
}

#[derive(Debug)]
struct Rgb(u8, u8, u8);

fn main() {
    let config = Config {
        host: String::from("localhost"),
        port: 8080,
        debug: true,
    };

    // --- 1. Dot notation ---
    println!("Host: {}", config.host);
    println!("Port: {}", config.port);
    println!("Debug: {}", config.debug);

    // --- 2. Tuple struct access ---
    let color = Rgb(255, 128, 0);
    println!("R={}, G={}, B={}", color.0, color.1, color.2);

    // --- 3. Destructuring in let ---
    let Config { host, port, debug } = &config;
    println!("Destructured: {host}:{port} (debug={debug})");

    // --- 4. Partial destructuring with `..` ---
    let Config { port, .. } = &config;
    println!("Just the port: {port}");

    // --- 5. Destructuring with rename ---
    let Config {
        host: h, port: p, ..
    } = &config;
    println!("Renamed: {h}:{p}");

    // --- 6. Nested struct access ---
    #[derive(Debug)]
    struct Server {
        config: Config,
        name: String,
    }
    let server = Server {
        config: Config {
            host: String::from("0.0.0.0"),
            port: 443,
            debug: false,
        },
        name: String::from("prod"),
    };
    println!(
        "{}: {}:{}",
        server.name, server.config.host, server.config.port
    );

    // --- 7. Access via reference ---
    let r = &config;
    // Rust auto-dereferences: r.host is the same as (*r).host.
    println!("Via ref: {}", r.host);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_access() {
        let c = Config {
            host: String::from("test"),
            port: 3000,
            debug: false,
        };
        assert_eq!(c.host, "test");
        assert_eq!(c.port, 3000);
        assert!(!c.debug);
    }

    #[test]
    fn test_tuple_struct_access() {
        let c = Rgb(10, 20, 30);
        assert_eq!(c.0, 10);
        assert_eq!(c.1, 20);
        assert_eq!(c.2, 30);
    }

    #[test]
    fn test_destructuring() {
        let c = Config {
            host: String::from("h"),
            port: 80,
            debug: true,
        };
        let Config { port, debug, .. } = &c;
        assert_eq!(*port, 80);
        assert!(debug);
    }
}
