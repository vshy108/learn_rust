// Demonstrates implementing Display on a struct.
//
// `Display` controls how a type appears with `{}` formatting.
// Unlike `Debug` (which can be derived), `Display` must be
// implemented manually — it defines the "user-facing" representation.
//
// Run: cargo run --example s18_display_struct

use std::fmt;

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Matrix {
    data: [[f64; 2]; 2],
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Multi-line display:
        writeln!(f, "| {:.1} {:.1} |", self.data[0][0], self.data[0][1])?;
        write!(f, "| {:.1} {:.1} |", self.data[1][0], self.data[1][1])
    }
}

fn main() {
    let red = Color { r: 255, g: 0, b: 0 };
    let p = Point { x: 3.14, y: 2.72 };
    let m = Matrix {
        data: [[1.0, 2.0], [3.0, 4.0]],
    };

    // {} uses Display:
    println!("Color: {red}");
    println!("Point: {p}");
    println!("Matrix:\n{m}");

    // to_string() also uses Display:
    let s = red.to_string();
    println!("As string: {s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_display() {
        let c = Color {
            r: 255,
            g: 128,
            b: 0,
        };
        assert_eq!(c.to_string(), "#FF8000");
    }

    #[test]
    fn test_point_display() {
        let p = Point { x: 1.0, y: 2.0 };
        assert_eq!(p.to_string(), "(1, 2)");
    }

    #[test]
    fn test_matrix_display() {
        let m = Matrix {
            data: [[1.0, 0.0], [0.0, 1.0]],
        };
        let s = m.to_string();
        assert!(s.contains("1.0"));
    }
}
