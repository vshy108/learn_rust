#![allow(dead_code, unused_imports, unused_parens)]
// Demonstrates struct basics in Rust.
//
// A struct groups related data into a named type. Think of it like
// a Python dataclass or a named tuple with typed fields.
//
// Three kinds of structs:
// 1. Regular (named fields) — `struct Point { x: f64, y: f64 }`
// 2. Tuple structs — `struct Color(u8, u8, u8)`
// 3. Unit-like structs — `struct Marker` (covered separately)
//
// Run: cargo run --example s09_struct

/// A 2D point in Cartesian coordinates.
///
/// # Fields
///
/// * `x` - Horizontal position
/// * `y` - Vertical position
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

/// An RGB color represented as a tuple struct.
///
/// Fields are accessed by index: `.0`, `.1`, `.2`.
#[derive(Debug, Clone, PartialEq)]
struct Color(u8, u8, u8);

/// A rectangle defined by its top-left corner and dimensions.
#[derive(Debug, Clone)]
struct Rectangle {
    origin: Point,
    width: f64,
    height: f64,
}

fn main() {
    // --- 1. Creating a struct instance ---
    let p = Point { x: 3.0, y: 4.0 };
    println!("Point: {p:?}");

    // --- 2. Accessing fields ---
    println!("x = {}, y = {}", p.x, p.y);

    // --- 3. Structs are immutable by default ---
    // p.x = 5.0; // error: cannot assign to field of immutable binding

    // --- 4. Mutable struct ---
    let mut q = Point { x: 0.0, y: 0.0 };
    q.x = 1.0; // OK because q is mut
    q.y = 2.0;
    println!("Moved point: {q:?}");
    // NOTE: You can't make individual fields mutable — the whole struct
    // is either mutable or immutable.

    // --- 5. Tuple struct ---
    let red = Color(255, 0, 0);
    println!("Red: ({}, {}, {})", red.0, red.1, red.2);

    // --- 6. Nested structs ---
    let rect = Rectangle {
        origin: Point { x: 0.0, y: 0.0 },
        width: 10.0,
        height: 5.0,
    };
    println!("Rectangle: {rect:?}");
    println!("origin.x = {}", rect.origin.x);

    // --- 7. Struct ownership ---
    // Structs own their data. When a struct goes out of scope,
    // all its fields are dropped.
    let r = Rectangle {
        origin: Point { x: 1.0, y: 2.0 },
        width: 3.0,
        height: 4.0,
    };
    let r2 = r; // MOVE (Rectangle doesn't derive Copy because Point has f64 fields
    // and while f64 IS Copy, we only derived Clone, not Copy)
    println!("{r2:?}");
    // println!("{r:?}"); // error: r was moved
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let p = Point { x: 1.0, y: 2.0 };
        assert!((p.x - 1.0).abs() < f64::EPSILON);
        assert!((p.y - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_color_tuple_struct() {
        let c = Color(255, 128, 0);
        assert_eq!(c.0, 255);
        assert_eq!(c.1, 128);
        assert_eq!(c.2, 0);
    }

    #[test]
    fn test_point_equality() {
        let a = Point { x: 1.0, y: 2.0 };
        let b = Point { x: 1.0, y: 2.0 };
        assert_eq!(a, b);
    }

    #[test]
    fn test_mutable_struct() {
        let mut p = Point { x: 0.0, y: 0.0 };
        p.x = 5.0;
        assert!((p.x - 5.0).abs() < f64::EPSILON);
    }
}
