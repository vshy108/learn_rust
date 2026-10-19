//! # Rustfinity: Derive Debug For Structs
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/derive-debug-for-structs
//!
//! In Rust, the `Debug` trait allows you to format your types using the `{:?}` formatter, making it easier to inspect and debug your code. Instead of manually implementing the `Debug` trait for every struct, Rust provides the convenient `#[derive(Debug)]` attribute to automatically generate a `Debug` implementation.
//!
//! For this challenge, you will work with structs that represent different types of data. Your task is to implement the `Debug` trait using the `derive` macro and demonstrate its functionality with sample structs.
//!
//! ## Your Task
//!
//! 1. Define several structs representing various entities (e.g., `Person`, `Point`, `Rectangle`).
//! 2. Derive the `Debug` trait for each struct using `#[derive(Debug)]`.
//!
//! ### Requirements
//!
//! 1. Use the `#[derive(Debug)]` attribute for all structs.
//! 2. Define at least three structs, each with different fields and types.
//!
//! Run: `cargo run --example rf_derive_debug_for_structs`
//! Test: `cargo test --example rf_derive_debug_for_structs`
//! Filter: `cargo test --example rf_derive_debug_for_structs <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

// 1. Complete the definitions of the structs Person, Point, and Rectangle.
// have a look at the function below `debug_example` to get the fields of the structs.
//
// 2. Make sure you implement the Debug trait for each struct. using the derive attribute.
#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

// Example function
pub fn debug_example() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    println!("{:?}", person);

    let point = Point { x: 5.0, y: -3.2 };
    println!("{:?}", point);

    let rectangle = Rectangle {
        width: 10,
        height: 20,
    };
    println!("{:?}", rectangle);
}

fn main() {
    println!("Implement the Derive Debug For Structs challenge.");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_debug_person() {
        let person = Person {
            name: "Alice".to_string(),
            age: 30,
        };
        assert_eq!(
            format!("{:?}", person),
            "Person { name: \"Alice\", age: 30 }"
        );
    }

    #[test]
    fn test_debug_point() {
        let point = Point { x: 5.0, y: -3.2 };
        assert_eq!(format!("{:?}", point), "Point { x: 5.0, y: -3.2 }");
    }

    #[test]
    fn test_debug_rectangle() {
        let rectangle = Rectangle {
            width: 10,
            height: 20,
        };
        assert_eq!(
            format!("{:?}", rectangle),
            "Rectangle { width: 10, height: 20 }"
        );
    }
}
