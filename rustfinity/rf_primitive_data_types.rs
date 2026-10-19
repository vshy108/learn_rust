//! # Rustfinity: Primitive Data Types
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/primitive-data-types
//!
//! Rust is a **statically-typed** language, which means that every variable must have a specific type. Rust's type system is designed to be safe and to prevent many common errors that occur in other languages. In this challenge, you will learn about some of the basic primitive data types in Rust, such as integers, floating-point numbers, booleans, and characters.
//!
//! Understanding how to declare and use these basic data types is fundamental to writing effective Rust code. This challenge will guide you through defining variables with specific types and initializing them.
//!
//! ## Your task
//!
//! 1. Define a variable with type `u8` and value `42`
//! 2. Define variable with type `f64` and value `3.14`
//! 3. Define variable with type `bool` and value `false`
//! 4. Define variable with type `char` and value `a`
//! 5. In the end, return the tuple `(u8, f64, bool, char)` with the variables you defined.
//!
//! ## Explanation of Data Types
//!
//! - **Integer** `(u8)`: Represents an `8-bit` unsigned integer.
//! - **Floating-point number** `(f64)`: Represents a `64-bit` **floating-point number**.
//! - **Boolean** `(bool)`: Represents a `boolean` value, which can be either `true` or `false`.
//! - **Character** `(char)`: Represents a single Unicode **scalar** value.
//!
//! ## Hints
//!
//! Here are some hints for you if you're stuck:
//!
//! <details>
//!   <summary>Click to show hints</summary>
//!
//! - To define a variable of type `u8` you can use the syntax `let variable_name: u8 = 10;`
//! - To define a variable of type `f64` you can use the syntax `let variable_name = 3.14;`
//! - To define a variable of type `bool` you can use the syntax `let variable_name = false;`
//! - To define a variable of type `char` you can use single quotes like `let variable_name = 'a';`
//!
//! </details>
//!
//! Run: `cargo run --example rf_primitive_data_types`
//! Test: `cargo test --example rf_primitive_data_types`
//! Filter: `cargo test --example rf_primitive_data_types <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub fn data_types() -> (u8, f64, bool, char) {
    // 1. Define variable of type `u8` and value `42`

    // 2. Define variable of type `f64` and value `3.14`

    // 3. Define variable of type `bool` and value `false`

    // 4. Define variable of type `char` and value `a`

    // 5. Return a tuple with the variables in the order they were defined
    unimplemented!()
}

fn main() {
    println!("Implement the Primitive Data Types challenge.");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_compiles() {
        let (x, y, z, a) = data_types();

        assert_eq!(x, 42, "Expected x to be 42");
        assert_eq!(y, 3.14, "Expected y to be 3.14");
        assert_eq!(z, false, "Expected z to be false");
        assert_eq!(a, 'a', "Expected a to be 'a'");
    }
}
