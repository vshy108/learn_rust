#![allow(dead_code, unused_imports, unused_parens)]
// Demonstrates deriving the Debug trait for structs.
//
// `#[derive(Debug)]` auto-generates a `fmt::Debug` implementation,
// letting you print structs with `{:?}` (compact) or `{:#?}` (pretty).
//
// Without Debug, you can't print a struct with `println!`.
//
// You can also implement Debug manually for custom formatting.
//
// Run: cargo run --example s09_debug_trait

/// A simple point — derives Debug automatically.
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

/// A color with a custom Debug implementation.
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

// Manual Debug implementation for custom formatting.
impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

/// A nested struct — all fields must also implement Debug.
#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

fn main() {
    // --- 1. Derived Debug: {:?} ---
    let p = Point { x: 3.0, y: 4.0 };
    println!("Debug:  {p:?}");

    // --- 2. Pretty-print: {:#?} ---
    println!("Pretty: {p:#?}");

    // --- 3. Nested structs ---
    let line = Line {
        start: Point { x: 0.0, y: 0.0 },
        end: Point { x: 10.0, y: 5.0 },
    };
    println!("Line: {line:#?}");

    // --- 4. Custom Debug ---
    let red = Color { r: 255, g: 0, b: 0 };
    let teal = Color {
        r: 0,
        g: 128,
        b: 128,
    };
    println!("Red:  {red:?}");
    println!("Teal: {teal:?}");

    // --- 5. Debug in collections ---
    let points = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 1.0 },
        Point { x: 2.0, y: 4.0 },
    ];
    println!("Points: {points:?}");

    // --- 6. dbg! macro uses Debug ---
    let q = Point { x: 7.0, y: 8.0 };
    // dbg! prints to stderr with file:line info and returns the value.
    // NOTE: dbg! takes ownership, so use &q to keep q usable.
    let _ = dbg!(&q);
    println!("q is still usable: {q:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derived_debug() {
        let p = Point { x: 1.0, y: 2.0 };
        let s = format!("{p:?}");
        assert!(s.contains("Point"));
        assert!(s.contains("1.0"));
        assert!(s.contains("2.0"));
    }

    #[test]
    fn test_custom_debug() {
        let c = Color {
            r: 255,
            g: 0,
            b: 128,
        };
        assert_eq!(format!("{c:?}"), "#FF0080");
    }

    #[test]
    fn test_nested_debug() {
        let line = Line {
            start: Point { x: 0.0, y: 0.0 },
            end: Point { x: 1.0, y: 1.0 },
        };
        let s = format!("{line:?}");
        assert!(s.contains("Line"));
        assert!(s.contains("start"));
        assert!(s.contains("end"));
    }
}
