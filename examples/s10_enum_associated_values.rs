// Demonstrates enums with associated values (data attached to variants).
//
// Unlike C enums that are just integers, Rust enums can carry data:
//
//   enum Message {
//       Quit,                       // no data
//       Echo(String),               // one String
//       Move { x: i32, y: i32 },   // named fields (struct variant)
//       Color(u8, u8, u8),          // tuple variant
//   }
//
// Each variant is like a different struct, all under one enum type.
// Think of it as a tagged union — the tag identifies which variant it is.
//
// Run: cargo run --example s10_enum_associated_values

#[derive(Debug)]
enum Shape {
    Circle(f64),                           // radius
    Rectangle { width: f64, height: f64 }, // named fields
    Triangle(f64, f64, f64),               // three side lengths
}

impl Shape {
    /// Computes the area of the shape.
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle(a, b, c) => {
                // Heron's formula
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}

fn main() {
    // --- 1. Creating enum variants with data ---
    let circle = Shape::Circle(5.0);
    let rect = Shape::Rectangle {
        width: 10.0,
        height: 3.0,
    };
    let tri = Shape::Triangle(3.0, 4.0, 5.0);

    // --- 2. Printing with Debug ---
    println!("{circle:?}");
    println!("{rect:?}");
    println!("{tri:?}");

    // --- 3. Using methods on enum ---
    let shapes = [circle, rect, tri];
    for shape in &shapes {
        println!("{shape:?} → area = {:.2}", shape.area());
    }

    // --- 4. Pattern matching to access the data ---
    for shape in &shapes {
        match shape {
            Shape::Circle(r) => println!("Circle with radius {r}"),
            Shape::Rectangle { width, height } => {
                println!("Rectangle {width}x{height}");
            }
            Shape::Triangle(a, b, c) => {
                println!("Triangle with sides {a}, {b}, {c}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let c = Shape::Circle(1.0);
        assert!((c.area() - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_rectangle_area() {
        let r = Shape::Rectangle {
            width: 3.0,
            height: 4.0,
        };
        assert!((r.area() - 12.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_triangle_area() {
        // 3-4-5 right triangle has area 6.0
        let t = Shape::Triangle(3.0, 4.0, 5.0);
        assert!((t.area() - 6.0).abs() < 1e-10);
    }
}
