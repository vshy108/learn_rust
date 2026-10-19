//! # Rustfinity: From Into Traits
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/from-into-traits
//!
//! The `From` and `Into` traits are Rust's standard way to express type conversions. They provide a consistent interface for transforming one type into another, making your code more flexible and easier to read. When you implement `From`, you automatically get `Into` for free thanks to a blanket implementation in the standard library.
//!
//! The `From` trait defines a `from()` method that takes ownership of a value and returns the new type. It's typically used when the conversion is infallible (cannot fail). For fallible conversions, Rust provides `TryFrom` and `TryInto` (covered in a separate challenge).
//!
//! ## The From and Into Traits
//!
//! ```rust
//! pub trait From<T> {
//!     fn from(value: T) -> Self;
//! }
//!
//! pub trait Into<T> {
//!     fn into(self) -> T;
//! }
//! ```
//!
//! When you implement `From<T> for U`, you automatically get `Into<U> for T`. This means you should always implement `From` rather than `Into` directly.
//!
//! ## Standard Library Examples
//!
//! ```rust
//! // String implements From<&str>
//! let s: String = String::from("hello");
//! let s: String = "hello".into();
//!
//! // Vec<u8> implements From<&str>
//! let bytes: Vec<u8> = Vec::from("hello");
//!
//! // i64 implements From<i32>
//! let big: i64 = i64::from(42i32);
//! let big: i64 = 42i32.into();
//! ```
//!
//! ## Your Task
//!
//! Implement the `From` trait for the following type conversions:
//!
//! ### 1. Celsius to Fahrenheit
//!
//! Create a `Celsius` struct and a `Fahrenheit` struct, both wrapping an `f64`. Implement conversions in both directions:
//!
//! - `Fahrenheit::from(Celsius)` - Formula: F = C × 9/5 + 32
//! - `Celsius::from(Fahrenheit)` - Formula: C = (F - 32) × 5/9
//!
//! ### 2. RGB to Hex Color
//!
//! Create an `Rgb` struct with `r`, `g`, `b` fields (all `u8`) and a `HexColor` struct wrapping a `String`. Implement:
//!
//! - `HexColor::from(Rgb)` - Convert RGB to hex string like "#FF5733"
//!
//! ### 3. Email Address Validation
//!
//! Create an `Email` struct wrapping a `String`. Implement:
//!
//! - `Email::from(&str)` - Create an email from a string slice
//! - `Email::from(String)` - Create an email from an owned string
//!
//! ### 4. Point2D to Point3D
//!
//! Create `Point2D` with `x` and `y` fields (`f64`) and `Point3D` with `x`, `y`, `z` fields (`f64`). Implement:
//!
//! - `Point3D::from(Point2D)` - Convert 2D point to 3D with z=0.0
//!
//! ### 5. Generic Wrapper
//!
//! Create a generic `Wrapper<T>` struct that wraps a value. Implement:
//!
//! - `Wrapper<T>::from(T)` - Wrap any value
//! - Implement a method `into_inner(self) -> T` to unwrap the value
//!
//! ## Examples
//!
//! ```rust
//! // Temperature conversion
//! let celsius = Celsius(100.0);
//! let fahrenheit: Fahrenheit = celsius.into();
//! assert!((fahrenheit.0 - 212.0).abs() < 0.001);
//!
//! let fahrenheit = Fahrenheit(32.0);
//! let celsius: Celsius = fahrenheit.into();
//! assert!((celsius.0 - 0.0).abs() < 0.001);
//!
//! // RGB to Hex
//! let rgb = Rgb { r: 255, g: 87, b: 51 };
//! let hex: HexColor = rgb.into();
//! assert_eq!(hex.0, "#FF5733");
//!
//! // Email
//! let email: Email = "user@example.com".into();
//! assert_eq!(email.0, "user@example.com");
//!
//! // Point conversion
//! let p2d = Point2D { x: 1.0, y: 2.0 };
//! let p3d: Point3D = p2d.into();
//! assert_eq!(p3d.z, 0.0);
//!
//! // Generic wrapper
//! let wrapped: Wrapper<i32> = 42.into();
//! assert_eq!(wrapped.into_inner(), 42);
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - Implement `From` rather than `Into` - you get `Into` automatically
//! - Use `format!("{:02X}{:02X}{:02X}", r, g, b)` for hex conversion (formats RGB values as 2-digit uppercase hex)
//! - For the generic `Wrapper<T>`, the `From` implementation should be generic over `T`
//! - Remember that `From` takes ownership of the input value
//! - The `.into()` method can require type annotations if Rust can't infer the target type
//! - You can use turbofish syntax `value.into::<TargetType>()` or let binding `let x: TargetType = value.into()`
//!
//! </details>
//!
//! Run: `cargo run --example rf_from_into_traits`
//! Test: `cargo test --example rf_from_into_traits`
//! Filter: `cargo test --example rf_from_into_traits <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

/// A temperature in Celsius.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Celsius(pub f64);

/// A temperature in Fahrenheit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fahrenheit(pub f64);

impl From<Celsius> for Fahrenheit {
    fn from(value: Celsius) -> Self {
        todo!("convert {} Celsius to Fahrenheit", value.0)
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(value: Fahrenheit) -> Self {
        todo!("convert {} Fahrenheit to Celsius", value.0)
    }
}

/// An RGB color with red, green, and blue components.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// A hex color string representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexColor(pub String);

impl From<Rgb> for HexColor {
    fn from(value: Rgb) -> Self {
        todo!("convert RGB({}, {}, {}) to hex", value.r, value.g, value.b)
    }
}

/// An email address wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(pub String);

impl From<&str> for Email {
    fn from(value: &str) -> Self {
        todo!("convert email {value}")
    }
}

impl From<String> for Email {
    fn from(value: String) -> Self {
        todo!("convert email {value}")
    }
}

/// A point in 2D space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

/// A point in 3D space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<Point2D> for Point3D {
    fn from(value: Point2D) -> Self {
        todo!("convert point ({}, {}) to 3D", value.x, value.y)
    }
}

/// A generic wrapper that can hold any value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wrapper<T>(pub T);

impl<T> Wrapper<T> {
    /// Unwraps the value, consuming the wrapper.
    pub fn into_inner(self) -> T {
        // TODO: Return the inner value
        unimplemented!()
    }
}

impl<T> From<T> for Wrapper<T> {
    fn from(value: T) -> Self {
        todo!("wrap value")
    }
}

pub fn main() {
    // Temperature conversion example
    let celsius = Celsius(100.0);
    let fahrenheit: Fahrenheit = celsius.into();
    println!("100°C = {}°F", fahrenheit.0);

    let fahrenheit = Fahrenheit(32.0);
    let celsius: Celsius = fahrenheit.into();
    println!("32°F = {}°C", celsius.0);

    // RGB to Hex example
    let rgb = Rgb {
        r: 255,
        g: 87,
        b: 51,
    };
    let hex: HexColor = rgb.into();
    println!("RGB({}, {}, {}) = {}", rgb.r, rgb.g, rgb.b, hex.0);

    // Email example
    let email: Email = "user@example.com".into();
    println!("Email: {}", email.0);

    // Point conversion example
    let p2d = Point2D { x: 3.0, y: 4.0 };
    let p3d: Point3D = p2d.into();
    println!(
        "2D({}, {}) -> 3D({}, {}, {})",
        p2d.x, p2d.y, p3d.x, p3d.y, p3d.z
    );

    // Wrapper example
    let wrapped: Wrapper<i32> = 42.into();
    println!("Wrapped value: {}", wrapped.into_inner());
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    // ============================================
    // Temperature Conversion Tests
    // ============================================

    mod celsius_to_fahrenheit {
        use super::*;

        #[test]
        fn freezing_point() {
            let c = Celsius(0.0);
            let f: Fahrenheit = c.into();
            assert!((f.0 - 32.0).abs() < 0.001);
        }

        #[test]
        fn boiling_point() {
            let c = Celsius(100.0);
            let f: Fahrenheit = c.into();
            assert!((f.0 - 212.0).abs() < 0.001);
        }

        #[test]
        fn body_temperature() {
            let c = Celsius(37.0);
            let f: Fahrenheit = c.into();
            assert!((f.0 - 98.6).abs() < 0.1);
        }

        #[test]
        fn negative_temperature() {
            let c = Celsius(-40.0);
            let f: Fahrenheit = c.into();
            // -40 is the same in both scales!
            assert!((f.0 - (-40.0)).abs() < 0.001);
        }

        #[test]
        fn using_from_syntax() {
            let c = Celsius(25.0);
            let f = Fahrenheit::from(c);
            assert!((f.0 - 77.0).abs() < 0.001);
        }
    }

    mod fahrenheit_to_celsius {
        use super::*;

        #[test]
        fn freezing_point() {
            let f = Fahrenheit(32.0);
            let c: Celsius = f.into();
            assert!((c.0 - 0.0).abs() < 0.001);
        }

        #[test]
        fn boiling_point() {
            let f = Fahrenheit(212.0);
            let c: Celsius = f.into();
            assert!((c.0 - 100.0).abs() < 0.001);
        }

        #[test]
        fn room_temperature() {
            let f = Fahrenheit(68.0);
            let c: Celsius = f.into();
            assert!((c.0 - 20.0).abs() < 0.001);
        }

        #[test]
        fn negative_temperature() {
            let f = Fahrenheit(-40.0);
            let c: Celsius = f.into();
            assert!((c.0 - (-40.0)).abs() < 0.001);
        }

        #[test]
        fn using_from_syntax() {
            let f = Fahrenheit(77.0);
            let c = Celsius::from(f);
            assert!((c.0 - 25.0).abs() < 0.001);
        }
    }

    mod temperature_round_trip {
        use super::*;

        #[test]
        fn celsius_round_trip() {
            let original = Celsius(42.5);
            let f: Fahrenheit = original.into();
            let back: Celsius = f.into();
            assert!((back.0 - original.0).abs() < 0.001);
        }

        #[test]
        fn fahrenheit_round_trip() {
            let original = Fahrenheit(100.0);
            let c: Celsius = original.into();
            let back: Fahrenheit = c.into();
            assert!((back.0 - original.0).abs() < 0.001);
        }
    }

    // ============================================
    // RGB to HexColor Tests
    // ============================================

    mod rgb_to_hex {
        use super::*;

        #[test]
        fn white() {
            let rgb = Rgb {
                r: 255,
                g: 255,
                b: 255,
            };
            let hex: HexColor = rgb.into();
            assert_eq!(hex.0, "#FFFFFF");
        }

        #[test]
        fn black() {
            let rgb = Rgb { r: 0, g: 0, b: 0 };
            let hex: HexColor = rgb.into();
            assert_eq!(hex.0, "#000000");
        }

        #[test]
        fn red() {
            let rgb = Rgb { r: 255, g: 0, b: 0 };
            let hex: HexColor = rgb.into();
            assert_eq!(hex.0, "#FF0000");
        }

        #[test]
        fn green() {
            let rgb = Rgb { r: 0, g: 255, b: 0 };
            let hex: HexColor = rgb.into();
            assert_eq!(hex.0, "#00FF00");
        }

        #[test]
        fn blue() {
            let rgb = Rgb { r: 0, g: 0, b: 255 };
            let hex: HexColor = rgb.into();
            assert_eq!(hex.0, "#0000FF");
        }

        #[test]
        fn coral() {
            let rgb = Rgb {
                r: 255,
                g: 87,
                b: 51,
            };
            let hex: HexColor = rgb.into();
            assert_eq!(hex.0, "#FF5733");
        }

        #[test]
        fn leading_zeros() {
            let rgb = Rgb { r: 1, g: 2, b: 3 };
            let hex: HexColor = rgb.into();
            assert_eq!(hex.0, "#010203");
        }

        #[test]
        fn using_from_syntax() {
            let rgb = Rgb {
                r: 128,
                g: 128,
                b: 128,
            };
            let hex = HexColor::from(rgb);
            assert_eq!(hex.0, "#808080");
        }
    }

    // ============================================
    // Email Tests
    // ============================================

    mod email_from_str {
        use super::*;

        #[test]
        fn from_str_slice() {
            let email: Email = "user@example.com".into();
            assert_eq!(email.0, "user@example.com");
        }

        #[test]
        fn from_string() {
            let s = String::from("admin@test.org");
            let email: Email = s.into();
            assert_eq!(email.0, "admin@test.org");
        }

        #[test]
        fn using_from_syntax_str() {
            let email = Email::from("hello@world.net");
            assert_eq!(email.0, "hello@world.net");
        }

        #[test]
        fn using_from_syntax_string() {
            let s = String::from("test@domain.com");
            let email = Email::from(s);
            assert_eq!(email.0, "test@domain.com");
        }

        #[test]
        fn empty_string() {
            let email: Email = "".into();
            assert_eq!(email.0, "");
        }

        #[test]
        fn unicode_email() {
            let email: Email = "user@例え.jp".into();
            assert_eq!(email.0, "user@例え.jp");
        }
    }

    // ============================================
    // Point2D to Point3D Tests
    // ============================================

    mod point_conversion {
        use super::*;

        #[test]
        fn origin() {
            let p2d = Point2D { x: 0.0, y: 0.0 };
            let p3d: Point3D = p2d.into();
            assert_eq!(p3d.x, 0.0);
            assert_eq!(p3d.y, 0.0);
            assert_eq!(p3d.z, 0.0);
        }

        #[test]
        fn positive_coordinates() {
            let p2d = Point2D { x: 1.0, y: 2.0 };
            let p3d: Point3D = p2d.into();
            assert_eq!(p3d.x, 1.0);
            assert_eq!(p3d.y, 2.0);
            assert_eq!(p3d.z, 0.0);
        }

        #[test]
        fn negative_coordinates() {
            let p2d = Point2D { x: -5.5, y: -10.25 };
            let p3d: Point3D = p2d.into();
            assert_eq!(p3d.x, -5.5);
            assert_eq!(p3d.y, -10.25);
            assert_eq!(p3d.z, 0.0);
        }

        #[test]
        fn using_from_syntax() {
            let p2d = Point2D { x: 3.14, y: 2.71 };
            let p3d = Point3D::from(p2d);
            assert!((p3d.x - 3.14).abs() < 0.001);
            assert!((p3d.y - 2.71).abs() < 0.001);
            assert_eq!(p3d.z, 0.0);
        }

        #[test]
        fn large_values() {
            let p2d = Point2D { x: 1e10, y: -1e10 };
            let p3d: Point3D = p2d.into();
            assert_eq!(p3d.x, 1e10);
            assert_eq!(p3d.y, -1e10);
            assert_eq!(p3d.z, 0.0);
        }
    }

    // ============================================
    // Generic Wrapper Tests
    // ============================================

    mod wrapper {
        use super::*;

        #[test]
        fn wrap_integer() {
            let wrapped: Wrapper<i32> = 42.into();
            assert_eq!(wrapped.into_inner(), 42);
        }

        #[test]
        fn wrap_string() {
            let wrapped: Wrapper<String> = String::from("hello").into();
            assert_eq!(wrapped.into_inner(), "hello");
        }

        #[test]
        fn wrap_float() {
            let wrapped: Wrapper<f64> = 3.14.into();
            assert!((wrapped.into_inner() - 3.14).abs() < 0.001);
        }

        #[test]
        fn wrap_bool() {
            let wrapped: Wrapper<bool> = true.into();
            assert!(wrapped.into_inner());
        }

        #[test]
        fn wrap_vec() {
            let wrapped: Wrapper<Vec<i32>> = vec![1, 2, 3].into();
            assert_eq!(wrapped.into_inner(), vec![1, 2, 3]);
        }

        #[test]
        fn wrap_tuple() {
            let wrapped: Wrapper<(i32, &str)> = (1, "hello").into();
            assert_eq!(wrapped.into_inner(), (1, "hello"));
        }

        #[test]
        fn using_from_syntax() {
            let wrapped = Wrapper::from(100u8);
            assert_eq!(wrapped.into_inner(), 100u8);
        }

        #[test]
        fn wrap_custom_struct() {
            #[derive(Debug, PartialEq)]
            struct Point {
                x: i32,
                y: i32,
            }

            let p = Point { x: 1, y: 2 };
            let wrapped: Wrapper<Point> = p.into();
            assert_eq!(wrapped.into_inner(), Point { x: 1, y: 2 });
        }

        #[test]
        fn wrap_option() {
            let wrapped: Wrapper<Option<i32>> = Some(42).into();
            assert_eq!(wrapped.into_inner(), Some(42));
        }

        #[test]
        fn wrap_result() {
            let wrapped: Wrapper<Result<i32, &str>> = Ok(42).into();
            assert_eq!(wrapped.into_inner(), Ok(42));
        }
    }

    // ============================================
    // Integration Tests
    // ============================================

    mod integration {
        use super::*;

        #[test]
        fn temperature_chain() {
            // Start with Celsius, convert through Fahrenheit, back to Celsius
            let temps: Vec<Celsius> = vec![Celsius(0.0), Celsius(25.0), Celsius(100.0)];
            let converted: Vec<Celsius> = temps
                .into_iter()
                .map(|c| -> Fahrenheit { c.into() })
                .map(|f| -> Celsius { f.into() })
                .collect();

            assert!((converted[0].0 - 0.0).abs() < 0.001);
            assert!((converted[1].0 - 25.0).abs() < 0.001);
            assert!((converted[2].0 - 100.0).abs() < 0.001);
        }

        #[test]
        fn rgb_collection_to_hex() {
            let colors = vec![
                Rgb { r: 255, g: 0, b: 0 },
                Rgb { r: 0, g: 255, b: 0 },
                Rgb { r: 0, g: 0, b: 255 },
            ];

            let hex_colors: Vec<HexColor> = colors.into_iter().map(|rgb| rgb.into()).collect();

            assert_eq!(hex_colors[0].0, "#FF0000");
            assert_eq!(hex_colors[1].0, "#00FF00");
            assert_eq!(hex_colors[2].0, "#0000FF");
        }

        #[test]
        fn points_to_3d() {
            let points_2d = vec![
                Point2D { x: 0.0, y: 0.0 },
                Point2D { x: 1.0, y: 1.0 },
                Point2D { x: 2.0, y: 2.0 },
            ];

            let points_3d: Vec<Point3D> = points_2d.into_iter().map(|p| p.into()).collect();

            for p in &points_3d {
                assert_eq!(p.z, 0.0);
            }
            assert_eq!(points_3d[0].x, 0.0);
            assert_eq!(points_3d[2].x, 2.0);
        }

        #[test]
        fn generic_function_with_into() {
            fn process_wrapped<T, U>(value: T) -> U
            where
                T: Into<Wrapper<U>>,
                Wrapper<U>: Into<Wrapper<U>>,
            {
                let wrapped: Wrapper<U> = value.into();
                wrapped.into_inner()
            }

            let result: i32 = process_wrapped(42);
            assert_eq!(result, 42);
        }

        #[test]
        fn email_from_various_sources() {
            let sources: Vec<Email> = vec![
                "static@example.com".into(),
                String::from("dynamic@example.com").into(),
            ];

            assert_eq!(sources[0].0, "static@example.com");
            assert_eq!(sources[1].0, "dynamic@example.com");
        }

        #[test]
        fn nested_wrapper() {
            let inner: Wrapper<i32> = 42.into();
            let outer: Wrapper<Wrapper<i32>> = inner.into();
            assert_eq!(outer.into_inner().into_inner(), 42);
        }
    }
}
