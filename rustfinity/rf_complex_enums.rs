//! # Rustfinity: Complex Enums
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/complex-enums
//!
//! Now that you have an overview of enums and it's possible variants, let's have a more complex example.
//!
//! In this challenge, you will create an `Animal` enum that demonstrates all three types of variants.
//!
//! ## Your Task
//!
//! Create an enum `Animal` with the following variants:
//!
//! 1. **Unit Struct Variant**:
//!    - `Dog` — represents a generic dog.
//!
//! 2. **Tuple Struct Variant**:
//!    - `Cat(String)` — represents a cat, where the `String` contains the cat's name.
//!
//! 3. **Named Field Struct Variant**:
//!    - `Bird { species: String, can_fly: bool }` — represents a bird with its species and whether it can fly.
//!
//! Write a function `describe_animal` that takes a reference to an `Animal` and returns a `String` description based on the variant:
//!
//! - For `Dog`, return `"A friendly dog."`.
//! - For `Cat(name)`, return `"A cat named {name}."`.
//! - For `Bird { species, can_fly }`, return:
//!   - `"A {species} that can fly."` if `can_fly` is true.
//!   - `"A {species} that cannot fly."` if `can_fly` is false.
//!
//! ## Hints
//!
//! <details>
//! <summary>Click here to reveal hints</summary>
//!
//! - Use the `match` statement to destructure tuple and named field variants.
//! - String formatting with `format!` makes it easy to include dynamic values in your description.
//!
//! </details>
//!
//! Run: `cargo run --example rf_complex_enums`
//! Test: `cargo test --example rf_complex_enums`
//! Filter: `cargo test --example rf_complex_enums <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

#[derive(Debug)]
pub enum Animal {
    Dog,
    Cat(String),
    Bird { species: String, can_fly: bool },
}

pub fn describe_animal(animal: &Animal) -> String {
    unimplemented!("describe animal {animal:?}")
}

// Example use case
pub fn main() {
    let dog = Animal::Dog;
    assert_eq!(describe_animal(&dog), "A friendly dog.");

    let cat = Animal::Cat("Whiskers".to_string());
    assert_eq!(describe_animal(&cat), "A cat named Whiskers.");

    let bird = Animal::Bird {
        species: "Penguin".to_string(),
        can_fly: false,
    };
    assert_eq!(describe_animal(&bird), "A Penguin that cannot fly.");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_describe_dog() {
        let dog = Animal::Dog;
        assert_eq!(describe_animal(&dog), "A friendly dog.");
    }

    #[test]
    fn test_describe_cat() {
        let cat = Animal::Cat("Whiskers".to_string());
        assert_eq!(describe_animal(&cat), "A cat named Whiskers.");

        let cat = Animal::Cat("Shadow".to_string());
        assert_eq!(describe_animal(&cat), "A cat named Shadow.");
    }

    #[test]
    fn test_describe_bird_can_fly() {
        let bird = Animal::Bird {
            species: "Eagle".to_string(),
            can_fly: true,
        };
        assert_eq!(describe_animal(&bird), "A Eagle that can fly.");
    }

    #[test]
    fn test_describe_bird_cannot_fly() {
        let bird = Animal::Bird {
            species: "Penguin".to_string(),
            can_fly: false,
        };
        assert_eq!(describe_animal(&bird), "A Penguin that cannot fly.");
    }

    #[test]
    fn test_mixed_animals() {
        let dog = Animal::Dog;
        assert_eq!(describe_animal(&dog), "A friendly dog.");

        let cat = Animal::Cat("Luna".to_string());
        assert_eq!(describe_animal(&cat), "A cat named Luna.");

        let bird = Animal::Bird {
            species: "Sparrow".to_string(),
            can_fly: true,
        };
        assert_eq!(describe_animal(&bird), "A Sparrow that can fly.");
    }
}
