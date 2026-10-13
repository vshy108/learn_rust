//! # Rustfinity: Printing Hello World
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/printing-hello-world
//!
//! It's a convention to start learning a new programming language by writing a program that prints `"Hello, World!"` to the console.
//!
//! In Rust, we can do this using the `println!` macro. The `!` indicates that this is a **macro** rather than a **function**.
//!
//! ## What is a macro?
//!
//! **A macro is a way to write code that writes other code.** **Macros** are a powerful feature of Rust that allow you to write code that writes code. This is useful for **metaprogramming**, which is the process of writing code that writes code.
//!
//! ## println!
//!
//! The `println!` macro is used to print text to the console. It is similar to the `println` function in other programming languages, but it is a **macro** in Rust.
//!
//! Here's how to print `Hello` to the console using the `println!` macro:
//!
//! ```rust
//! fn main() {
//!     println!("Hello");
//! }
//! ```
//!
//! ## Your task
//!
//! Write a program that prints `"Hello, World!"` to the console using the `println!` macro.
//!
//! ## Expected Output
//!
//! ```rust
//! Hello, World!
//! ```
//!
//! Run: `cargo run --example rf_printing_hello_world`
//! Test: `cargo test --example rf_printing_hello_world`
//! Filter: `cargo test --example rf_printing_hello_world <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

pub fn hello_world() {
    // Use the println! macro to print "Hello, world!" to the console
}

fn main() {
    println!("Implement the Printing Hello World challenge.");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_compiles() {
        hello_world();
    }

}
