//! # Rustfinity: Animal Sanctuary Registry
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/animal-sanctuary-registry
//!
//! ## Hashmaps
//!
//! Hashmaps are a powerful data structure that allow you to store **key-value pairs**. In Rust, the `HashMap` type is provided that uses the a **hashing algorithm** called **SipHash** to store keys and values in a way that allows for **fast and secure** lookups. Think of them as a dictionary in Python or an object in JavaScript.
//!
//! In this challenge, we want to build a sanctuary registry that allows us to manage animals in different sections of the sanctuary. We'll use a `HashMap` to store the sections as keys and a `Vec` to store the animals in each section. Each key is a section name `String` and each value is a list of animals in that section `Vec<String>`.
//!
//! ## Task
//!
//! You are given a type of `Collection` which is a `HashMap<String, Vec<String>>`. The key is the section name and the value is a list of animals in that section.
//!
//! Your task is to implement the following functions:
//!
//! 1. `add_animal_to_section`: This function should add an animal to a section in the registry. If the section does not exist, it should be created. If the animal is already in the section, it should not be added again.
//!
//! 2. `get_animals_in_section`: This function should return a list of animals **sorted alphabetically** in a given section. If the section does not exist, it should return an empty list.
//!
//! 3. `get_all_animals_sorted`: This function should return a copy of the entire registry with all animals **sorted alphabetically** in each section.
//!
//! ## Example
//!
//! ```rust
//! let mut registry = Collection::new();
//!
//! add_animal_to_section("Eagle", "Birds", &mut registry);
//! assert_eq!(
//!     get_animals_in_section("Birds", &registry),
//!     vec!["Eagle"]
//! );
//! ```
//!
//! ## Hints
//!
//! - Use the `insert` method on a `HashMap` to add a new section or update an existing section.
//! - Use the `entry` method on a `HashMap` to get a mutable reference to a section.
//! - Use the `iter` method on a `HashMap` to iterate over the key-value pairs.
//! - Use the `sort` method on a `Vec` to sort the animals alphabetically.
//!
//! Run: `cargo run --example rf_animal_sanctuary_registry`
//! Test: `cargo test --example rf_animal_sanctuary_registry`
//! Filter: `cargo test --example rf_animal_sanctuary_registry <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::collections::HashMap;

type Collection = HashMap<String, Vec<String>>;

pub fn add_animal_to_section(animal: &str, section: &str, registry: &mut Collection) {
    // TODO: implement this function
    unimplemented!()
}

pub fn get_animals_in_section(section: &str, registry: &Collection) -> Vec<String> {
    // TODO: implement this function
    unimplemented!()
}

pub fn get_all_animals_sorted(registry: &Collection) -> Vec<String> {
    // TODO: implement this function
    unimplemented!()
}

fn main() {
    println!("Implement the Animal Sanctuary Registry challenge.");
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::collections::HashMap;

    #[test]
    fn it_should_add_animal_to_section() {
        let mut registry = HashMap::new();

        add_animal_to_section("Lion", "Savannah", &mut registry);
        assert_eq!(registry["Savannah"], vec!["Lion"]);

        add_animal_to_section("Elephant", "Savannah", &mut registry);
        assert_eq!(registry["Savannah"], vec!["Lion", "Elephant"]);

        add_animal_to_section("Penguin", "Arctic", &mut registry);
        assert_eq!(registry["Arctic"], vec!["Penguin"]);

        add_animal_to_section("Polar Bear", "Arctic", &mut registry);
        assert_eq!(registry["Arctic"], vec!["Penguin", "Polar Bear"]);

        add_animal_to_section("Monkey", "Jungle", &mut registry);
        assert_eq!(registry["Jungle"], vec!["Monkey"]);

        add_animal_to_section("Tiger", "Jungle", &mut registry);
        assert_eq!(registry["Jungle"], vec!["Monkey", "Tiger"]);

        add_animal_to_section("Shark", "Ocean", &mut registry);
        assert_eq!(registry["Ocean"], vec!["Shark"]);
    }

    #[test]
    fn it_should_append_if_section_already_exists() {
        let mut registry = HashMap::new();

        add_animal_to_section("Penguin", "Arctic", &mut registry);
        add_animal_to_section("Polar Bear", "Arctic", &mut registry);
        add_animal_to_section("Another Animal", "Arctic", &mut registry);

        assert_eq!(
            registry["Arctic"],
            vec!["Penguin", "Polar Bear", "Another Animal"]
        );

        add_animal_to_section("Monkey", "Jungle", &mut registry);
        add_animal_to_section("Tiger", "Jungle", &mut registry);
        add_animal_to_section("Another Animal", "Jungle", &mut registry);

        assert_eq!(
            registry["Jungle"],
            vec!["Monkey", "Tiger", "Another Animal"]
        );
    }

    #[test]
    fn it_should_get_animals_in_section() {
        let mut registry = HashMap::new();
        registry.insert(
            "Arctic".to_string(),
            vec!["Penguin".to_string(), "Polar Bear".to_string()],
        );
        let animals = get_animals_in_section("Arctic", &registry);
        assert_eq!(animals, vec!["Penguin", "Polar Bear"]);
    }

    #[test]
    fn it_should_not_add_duplicates() {
        let mut registry = HashMap::new();
        add_animal_to_section("Penguin", "Arctic", &mut registry);
        add_animal_to_section("Penguin", "Arctic", &mut registry);
        assert_eq!(registry["Arctic"], vec!["Penguin"]);
    }

    #[test]
    fn it_should_return_empty_vector_if_section_does_not_exist() {
        let registry = HashMap::new();
        let animals = get_animals_in_section("Arctic", &registry);
        assert_eq!(animals, Vec::<String>::new());
    }

    #[test]
    fn it_should_sort_alphabetically() {
        let mut registry = HashMap::new();
        let animals = vec![
            "Zebra".to_string(),
            "Leopard".to_string(),
            "Gorilla".to_string(),
            "Snake".to_string(),
            "Rhino".to_string(),
            "Armadillo".to_string(),
            "Anteater".to_string(),
        ];
        registry.insert("Jungle".to_string(), animals.clone());

        let sorted = vec![
            "Anteater".to_string(),
            "Armadillo".to_string(),
            "Gorilla".to_string(),
            "Leopard".to_string(),
            "Rhino".to_string(),
            "Snake".to_string(),
            "Zebra".to_string(),
        ];
        let animals_sorted = get_animals_in_section("Jungle", &registry);

        assert_eq!(animals_sorted, sorted);
    }

    #[test]
    fn it_should_sort_alphabetically_for_get_all() {
        let mut registry = HashMap::new();
        let jungle_animals = vec![
            "Leopard".to_string(),
            "Gorilla".to_string(),
            "Snake".to_string(),
            "Rhino".to_string(),
            "Hippo".to_string(),
            "Crocodile".to_string(),
            "Jaguar".to_string(),
            "Chimpanzee".to_string(),
            "Kangaroo".to_string(),
            "Panther".to_string(),
        ];

        let arctic_animals = vec![
            "Penguin".to_string(),
            "Polar Bear".to_string(),
            "Seal".to_string(),
            "Walrus".to_string(),
            "Arctic Fox".to_string(),
            "Arctic Hare".to_string(),
            "Arctic Wolf".to_string(),
            "Beluga Whale".to_string(),
            "Musk Ox".to_string(),
            "Snowy Owl".to_string(),
        ];

        let ocean_animals = vec![
            "Shark".to_string(),
            "Dolphin".to_string(),
            "Whale".to_string(),
            "Octopus".to_string(),
            "Jellyfish".to_string(),
            "Sea Turtle".to_string(),
            "Stingray".to_string(),
            "Squid".to_string(),
            "Crab".to_string(),
            "Lobster".to_string(),
        ];

        registry.insert("Jungle".to_string(), jungle_animals.clone());
        registry.insert("Arctic".to_string(), arctic_animals.clone());
        registry.insert("Ocean".to_string(), ocean_animals.clone());

        let mut all_animals = Vec::new();

        all_animals.extend(jungle_animals);
        all_animals.extend(arctic_animals);
        all_animals.extend(ocean_animals);
        all_animals.sort();

        let all_animals_sorted = get_all_animals_sorted(&registry);

        assert_eq!(all_animals_sorted, all_animals);
    }
}
