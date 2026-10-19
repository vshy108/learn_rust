//! # Rustfinity: Generics
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/generics
//!
//! Generics in Rust allow you to write code that works with different types without duplicating code for each specific type. They are commonly used in functions, structs, enums, and methods to make your code more reusable and flexible.
//!
//! For this challenge, you will implement a generic struct `ItemContainer` that can store any type of item and provide methods to retrieve and manipulate the item.
//!
//! ## Your Task
//!
//! 1. Define a generic struct `ItemContainer<T>` with a single field `item: T`.
//! 2. Implement a method `get_item` that returns a reference to the item.
//!
//! ## Hints
//!
//! If you're stuck, here are some hints to help you solve the challenge:
//!
//! <details>
//!   <summary>Click here to reveal hints</summary>
//!
//! - Define the struct as `ItemContainer<T>`. e.g.
//!
//!   ```rust
//!   struct ItemContainer<T> {
//!     item: T,
//!   }
//!   ```
//!
//! - Implement the method in the `impl<T>` block. The method should take `&self` and return `&T`.
//!
//! </details>
//!
//! Run: `cargo run --example rf_generics`
//! Test: `cargo test --example rf_generics`
//! Filter: `cargo test --example rf_generics <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

// TODO: Define a generic struct with a single field `item: T`.
pub struct ItemContainer<T> {
    pub item: T,
}

impl<T> ItemContainer<T> {
    pub fn get_item(&self) -> &T {
        todo!("return the stored item")
    }
}

// Example usage
pub fn main() {
    let item_1 = ItemContainer { item: 42 };
    assert_eq!(*item_1.get_item(), 42);

    let item_2 = ItemContainer {
        item: String::from("Hello"),
    };

    assert_eq!(item_2.get_item(), "Hello");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_integer_item() {
        let container = ItemContainer { item: 42 };
        assert_eq!(*container.get_item(), 42);
    }

    #[test]
    fn test_string_item() {
        let container = ItemContainer {
            item: String::from("Hello"),
        };
        assert_eq!(container.get_item(), "Hello");
    }

    #[test]
    fn test_custom_struct_item() {
        struct Point {
            x: i32,
            y: i32,
        }

        let container = ItemContainer {
            item: Point { x: 1, y: 2 },
        };
        assert_eq!(container.get_item().x, 1);
        assert_eq!(container.get_item().y, 2);
    }

    #[test]
    fn test_nested_container() {
        let inner_container = ItemContainer { item: 42 };
        let outer_container = ItemContainer {
            item: inner_container,
        };

        assert_eq!(*outer_container.get_item().get_item(), 42);
    }

    #[test]
    fn test_large_data() {
        let large_vector: Vec<u8> = vec![1, 2, 3, 4, 5];
        let container = ItemContainer {
            item: large_vector.clone(),
        };

        assert_eq!(container.get_item(), &large_vector);
    }

    #[test]
    fn test_item_lifetime() {
        let original_string = String::from("Owned");
        let container = ItemContainer {
            item: &original_string,
        };

        assert_eq!(*container.get_item(), "Owned");
    }

    #[test]
    fn test_empty_container() {
        let container: ItemContainer<Option<i32>> = ItemContainer { item: None };

        assert_eq!(*container.get_item(), None);
    }
}
