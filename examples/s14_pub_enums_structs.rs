// Demonstrates visibility rules for pub enums, structs, and fields.
//
// Key difference:
// - **Enum**: Making an enum `pub` makes ALL its variants public.
// - **Struct**: Making a struct `pub` does NOT make its fields public.
//              Each field must be individually marked `pub`.
//
// This is by design:
// - Enums: you need to match all variants, so they must all be visible.
// - Structs: encapsulation — you might want private internals.
//
// Run: cargo run --example s14_pub_enums_structs

mod shapes {
    /// Public enum — ALL variants are public.
    #[derive(Debug)]
    pub enum Shape {
        Circle(f64),
        Rectangle(f64, f64),
        Triangle(f64, f64, f64),
    }

    /// Public struct — fields are private by default.
    #[derive(Debug)]
    pub struct Point {
        pub x: f64,    // public field
        pub y: f64,    // public field
        label: String, // PRIVATE field
    }

    impl Point {
        /// Constructor — needed because `label` is private.
        pub fn new(x: f64, y: f64, label: &str) -> Self {
            Self {
                x,
                y,
                label: label.to_string(),
            }
        }

        /// Getter for the private field.
        pub fn label(&self) -> &str {
            &self.label
        }
    }

    /// Tuple struct — fields follow the same rules.
    pub struct Color(pub u8, pub u8, pub u8); // all public
    pub struct Secret(u8); // field is private

    impl Secret {
        pub fn new(val: u8) -> Self {
            Self(val)
        }
        pub fn value(&self) -> u8 {
            self.0
        }
    }
}

fn main() {
    // --- 1. Enum: all variants accessible ---
    let s = shapes::Shape::Circle(5.0);
    println!("shape: {s:?}");

    // Match works because all variants are visible:
    match s {
        shapes::Shape::Circle(r) => println!("Circle with radius {r}"),
        shapes::Shape::Rectangle(w, h) => println!("Rect {w}x{h}"),
        shapes::Shape::Triangle(a, b, c) => println!("Tri {a},{b},{c}"),
    }

    // --- 2. Struct: only pub fields accessible ---
    let p = shapes::Point::new(1.0, 2.0, "origin");
    println!("point: ({}, {})", p.x, p.y); // public fields
    println!("label: {}", p.label()); // via getter
    // println!("{}", p.label); // ERROR: field `label` is private

    // Can't construct directly if any field is private:
    // let p = shapes::Point { x: 0.0, y: 0.0, label: "...".into() }; // ERROR

    // --- 3. Tuple struct field visibility ---
    let color = shapes::Color(255, 128, 0);
    println!("color: ({}, {}, {})", color.0, color.1, color.2);

    let secret = shapes::Secret::new(42);
    println!("secret: {}", secret.value());
    // println!("{}", secret.0); // ERROR: private field
}

#[cfg(test)]
mod tests {
    use super::shapes::*;

    #[test]
    fn test_enum_variants_accessible() {
        let _c = Shape::Circle(1.0);
        let _r = Shape::Rectangle(2.0, 3.0);
        let _t = Shape::Triangle(3.0, 4.0, 5.0);
    }

    #[test]
    fn test_point_pub_fields() {
        let p = Point::new(3.0, 4.0, "test");
        assert!((p.x - 3.0).abs() < f64::EPSILON);
        assert!((p.y - 4.0).abs() < f64::EPSILON);
        assert_eq!(p.label(), "test");
    }

    #[test]
    fn test_color_tuple_struct() {
        let c = Color(255, 0, 128);
        assert_eq!(c.0, 255);
        assert_eq!(c.1, 0);
        assert_eq!(c.2, 128);
    }
}
