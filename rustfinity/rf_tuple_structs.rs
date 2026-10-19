//! # Rustfinity: Tuple Structs
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/tuple-structs
//!
//! Rust provides a special kind of struct known as a _tuple struct_. Unlike a regular struct with named fields, a tuple struct groups fields together without names, making it useful for lightweight structures where naming each field isn't necessary.
//!
//! A tuple struct looks like this:
//!
//! ```rust
//! struct Point(i32, i32, i32);
//! ```
//!
//! You can create an instance of a tuple struct just like you would with a tuple and access its fields using dot notation.
//!
//! ## Your Task
//!
//! Your task is to define a tuple struct called `Rectangle` with two fields:
//!
//! - The width of the rectangle as a `f32`.
//! - The height of the rectangle as a `f32`.
//!
//! Then, implement a function `area` that calculates and returns the area of the rectangle by multiplying its width and height.
//!
//! ### Requirements
//!
//! - The fields of the tuple struct must be public. Here's an example:
//!   ```rust
//!   pub struct Point(pub i32, pub i32, pub i32);
//!   ```
//!
//! ---
//!
//! ## Example Test
//!
//! Here's how your code will be tested:
//!
//! ```rust
//! let rect = Rectangle(4.0, 5.0);
//! assert_eq!(area(&rect), 20.0);
//!
//! let square = Rectangle(10.0, 10.0);
//! assert_eq!(area(&square), 100.0);
//! ```
//!
//! ---
//!
//! ## Hints
//!
//! If you get stuck, consider these hints:
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - Use dot notation to access the fields of the tuple struct (e.g., `rect.0` for width and `rect.1` for height).
//!
//! </details>
//!
//! Run: `cargo run --example rf_tuple_structs`
//! Test: `cargo test --example rf_tuple_structs`
//! Filter: `cargo test --example rf_tuple_structs <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub struct Rectangle(pub f32, pub f32);

pub fn area(rect: &Rectangle) -> f32 {
    todo!(
        "calculate the area for dimensions {:?} and {:?}",
        rect.0,
        rect.1
    )
}

fn main() {
    println!("Implement the Tuple Structs challenge.");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_area() {
        let rect = Rectangle(4.0, 5.0);
        assert_eq!(area(&rect), 20.0);

        let square = Rectangle(10.0, 10.0);
        assert_eq!(area(&square), 100.0);
    }

    #[test]
    fn test_area_zero() {
        let flat_rect = Rectangle(0.0, 5.0);
        assert_eq!(area(&flat_rect), 0.0);

        let empty_rect = Rectangle(0.0, 0.0);
        assert_eq!(area(&empty_rect), 0.0);
    }

    #[test]
    fn test_large_values() {
        let large_rect = Rectangle(1_000.0, 2_000.0);
        assert_eq!(area(&large_rect), 2_000_000.0);
    }

    #[test]
    fn test_edge_cases() {
        let tall_skinny = Rectangle(1.0, 100.0);
        assert_eq!(area(&tall_skinny), 100.0);

        let wide_flat = Rectangle(100.0, 1.0);
        assert_eq!(area(&wide_flat), 100.0);
    }
}
