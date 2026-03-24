#![allow(dead_code, unused_imports, unused_parens)]
// Demonstrates struct update syntax.
//
// Struct update syntax lets you create a new struct from an existing one,
// overriding specific fields while copying/moving the rest.
//
// Syntax: `NewStruct { changed_field: value, ..existing }`
//
// **Gotcha**: This MOVES non-Copy fields from `existing`. If the struct
// contains a String, the original is partially moved and may become unusable.
//
// Run: cargo run --example s09_update_syntax

#[derive(Debug, Clone)]
struct Config {
    host: String,
    port: u16,
    max_connections: u32,
    debug: bool,
}

impl Config {
    /// Creates a default development configuration.
    fn dev_default() -> Self {
        Config {
            host: String::from("localhost"),
            port: 3000,
            max_connections: 10,
            debug: true,
        }
    }
}

fn main() {
    // --- 1. Basic update syntax ---
    let dev = Config::dev_default();
    println!("Dev: {dev:?}");

    // Create a production config, changing only some fields:
    let prod = Config {
        host: String::from("0.0.0.0"),
        port: 443,
        debug: false,
        ..dev.clone() // copies max_connections from dev
                      // We clone dev because host is a String (non-Copy), and
                      // we've already provided a new host above — but the `..`
                      // syntax still moves/copies ALL remaining fields.
    };
    println!("Prod: {prod:?}");
    // dev is still valid because we cloned it.
    println!("Dev still usable: {dev:?}");

    // --- 2. Move semantics with update syntax ---
    let base = Config::dev_default();
    // Only override port — host is MOVED from base:
    let custom = Config {
        port: 8080,
        ..base // base.host is moved into custom
    };
    println!("Custom: {custom:?}");
    // base.host was moved, so base is partially moved:
    // println!("{:?}", base); // error: use of partially moved value
    // But Copy fields are still accessible:
    // println!("{}", base.port); // This would work!

    // --- 3. All-Copy struct: no move issues ---
    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: f64,
        y: f64,
        z: f64,
    }

    let origin = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let p = Point { x: 5.0, ..origin }; // y and z copied from origin
    println!("Point: {p:?}");
    println!("Origin still valid: {origin:?}"); // OK: Point is Copy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_syntax_overrides() {
        let base = Config::dev_default();
        let updated = Config {
            port: 9090,
            ..base.clone()
        };
        assert_eq!(updated.port, 9090);
        assert_eq!(updated.host, "localhost"); // from base
        assert_eq!(updated.max_connections, 10); // from base
    }

    #[test]
    fn test_update_copy_struct() {
        #[derive(Debug, Copy, Clone, PartialEq)]
        struct Vec2 {
            x: f64,
            y: f64,
        }
        let a = Vec2 { x: 1.0, y: 2.0 };
        let b = Vec2 { x: 10.0, ..a };
        assert!((b.x - 10.0).abs() < f64::EPSILON);
        assert!((b.y - 2.0).abs() < f64::EPSILON);
        assert!((a.x - 1.0).abs() < f64::EPSILON); // a still valid (Copy)
    }
}
