//! # Rustfinity: Box T
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/box-t
//!
//! The `Box<T>` type in Rust is a smart pointer that allows heap allocation for values, enabling efficient management of large data or data with unknown sizes at compile time.
//!
//! In this challenge, you'll work with a custom struct `Animal` and use `Box<T>` to manage its memory. You will also implement two functions to access the struct's fields by dereferencing the boxed instance.
//!
//! ## Your Task
//!
//! 1. Implement the `create_animal` function to return a `Box<Animal>` containing a new `Animal` instance.
//!
//! 2. Define another function, `access_animal`, that takes a `Box<Animal>` and returns a tuple `(String, u8)` representing the animal's name and age. Use dereferencing to access the fields.
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here to reveal hints</summary>
//!
//! - To create a boxed struct, use `Box::new(struct_instance)`.
//! - Use the `*` operator to dereference the box and access its value.
//!
//! </details>
//!
//! Run: `cargo run --example rf_box_t`
//! Test: `cargo test --example rf_box_t`
//! Filter: `cargo test --example rf_box_t <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

pub struct Animal {
    pub name: String,
    pub age: u8,
}

pub fn create_animal(name: &str, age: u8) -> Box<Animal> {
    // Your code here
    unimplemented!()
}

pub fn access_animal(animal: Box<Animal>) -> (String, u8) {
    // Your code here
    unimplemented!()
}

// Example usage
pub fn main() {
    let animal = create_animal("Leo", 5);
    let (name, age) = access_animal(animal);
    println!("Animal's name: {}, age: {}", name, age);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use crate::{access_animal, create_animal};

    #[test]
    fn test_create_animal() {
        let animal = create_animal("Bella", 3);
        assert_eq!(animal.name, "Bella");
        assert_eq!(animal.age, 3);
    }

    #[test]
    fn test_access_animal() {
        let animal = create_animal("Max", 4);
        let (name, age) = access_animal(animal);
        assert_eq!(name, "Max");
        assert_eq!(age, 4);
    }

    #[test]
    fn test_create_animal_empty_name() {
        let animal = create_animal("", 7);
        assert_eq!(animal.name, "");
        assert_eq!(animal.age, 7);
    }

    #[test]
    fn test_access_animal_empty_name() {
        let animal = create_animal("", 7);
        let (name, age) = access_animal(animal);
        assert_eq!(name, "");
        assert_eq!(age, 7);
    }

    #[test]
    fn test_create_animal_long_name() {
        let long_name = "L".repeat(100);
        let animal = create_animal(&long_name, 2);
        assert_eq!(animal.name, long_name);
        assert_eq!(animal.age, 2);
    }

    #[test]
    fn test_access_animal_long_name() {
        let long_name = "L".repeat(100);
        let animal = create_animal(&long_name, 2);
        let (name, age) = access_animal(animal);
        assert_eq!(name, long_name);
        assert_eq!(age, 2);
    }

    #[test]
    fn test_create_animal_boundary_age() {
        let animal = create_animal("Tiny", 0);
        assert_eq!(animal.name, "Tiny");
        assert_eq!(animal.age, 0);

        let animal = create_animal("Giant", u8::MAX);
        assert_eq!(animal.name, "Giant");
        assert_eq!(animal.age, u8::MAX);
    }

    #[test]
    fn test_access_animal_boundary_age() {
        let animal = create_animal("Tiny", 0);
        let (name, age) = access_animal(animal);
        assert_eq!(name, "Tiny");
        assert_eq!(age, 0);

        let animal = create_animal("Giant", u8::MAX);
        let (name, age) = access_animal(animal);
        assert_eq!(name, "Giant");
        assert_eq!(age, u8::MAX);
    }
}
