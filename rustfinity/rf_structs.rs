//! # Rustfinity: Structs
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/structs
//!
//! Structs are one of Rust's fundamental building blocks for creating custom types. They allow you to group together related data under one type, making your programs easier to understand and maintain. In this challenge, you'll work with Rust structs by creating a simple structure to represent a `Person`.
//!
//! A struct in Rust can have named fields with different data types. For example:
//!
//! ```rust
//! struct Book {
//!     name: String,
//!     author: String,
//! }
//! ```
//!
//! Once defined, you can create instances of the struct and access or modify its fields using dot notation.
//!
//! ---
//!
//! ## Your Task
//!
//! Your task is to define a struct called `Person` with the following fields:
//!
//! - `name`: A `String` representing the person's name.
//! - `age`: A `u8` representing the person's age.
//!
//! You must also implement a function `is_adult` that takes a reference to a `Person` and returns `true` if the person's age is 18 or older, and `false` otherwise.
//!
//! ### Requirements
//!
//! 1. Define a struct called `Person` with the fields `name` and `age`.
//! 2. Implement the function `is_adult` to determine if a person is an adult (18 or older).
//! 3. Make sure the struct and it's fields are public.
//!    ```rust
//!    pub struct Book {
//!        pub name: String,
//!        pub author: String,
//!    }
//!    ```
//!    This will allow the tests to access the struct and its fields. Without this, the tests will fail.
//!
//! ---
//!
//! ## Example Test
//!
//! Here's how your code will be tested:
//!
//! ```rust
//! let john = Person { name: "John".to_string(), age: 20 };
//! assert_eq!(is_adult(&john), true);
//!
//! let emily = Person { name: "Emily".to_string(), age: 15 };
//! assert_eq!(is_adult(&emily), false);
//! ```
//!
//! ---
//!
//! ## Hints
//!
//! If you're having trouble, consider these hints:
//!
//! <details>
//!   <summary>Click here to reveal hints</summary>
//!
//! - Use `&` to pass references to avoid unnecessary copies.
//! - Use the `age` field directly in a comparison for the `is_adult` function.
//!
//! </details>
//!
//! Run: `cargo run --example rf_structs`
//! Test: `cargo test --example rf_structs`
//! Filter: `cargo test --example rf_structs <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub struct Person {
    pub name: String,
    pub age: u8,
}

pub fn is_adult(person: &Person) -> bool {
    todo!("check whether {} is an adult", person.name)
}

fn main() {
    println!("Implement the Structs challenge.");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_is_adult() {
        let john = Person {
            name: "John".to_string(),
            age: 20,
        };
        assert_eq!(is_adult(&john), true);

        let emily = Person {
            name: "Emily".to_string(),
            age: 15,
        };
        assert_eq!(is_adult(&emily), false);

        let mike = Person {
            name: "Mike".to_string(),
            age: 18,
        };
        assert_eq!(is_adult(&mike), true);
    }

    #[test]
    fn test_edge_cases() {
        let baby = Person {
            name: "Baby".to_string(),
            age: 0,
        };
        assert_eq!(is_adult(&baby), false);

        let elder = Person {
            name: "Elder".to_string(),
            age: 99,
        };
        assert_eq!(is_adult(&elder), true);
    }

    #[test]
    fn test_exact_age_boundary() {
        let just_barely_adult = Person {
            name: "Alex".to_string(),
            age: 18,
        };
        assert_eq!(is_adult(&just_barely_adult), true);

        let just_under_adult = Person {
            name: "Taylor".to_string(),
            age: 17,
        };
        assert_eq!(is_adult(&just_under_adult), false);
    }

    #[test]
    fn test_large_age_values() {
        let super_elder = Person {
            name: "Methuselah".to_string(),
            age: 255,
        };
        assert_eq!(is_adult(&super_elder), true);

        let extreme_child = Person {
            name: "Time Traveler".to_string(),
            age: 10,
        };
        assert_eq!(is_adult(&extreme_child), false);
    }

    #[test]
    fn test_non_ascii_names() {
        let non_ascii_name = Person {
            name: "Åsa".to_string(),
            age: 22,
        };
        assert_eq!(is_adult(&non_ascii_name), true);

        let young_non_ascii = Person {
            name: "小明".to_string(),
            age: 12,
        };
        assert_eq!(is_adult(&young_non_ascii), false);
    }

    #[test]
    fn test_name_edge_cases() {
        let empty_name = Person {
            name: "".to_string(),
            age: 30,
        };
        assert_eq!(is_adult(&empty_name), true);

        let whitespace_name = Person {
            name: "   ".to_string(),
            age: 25,
        };
        assert_eq!(is_adult(&whitespace_name), true);
    }
}
