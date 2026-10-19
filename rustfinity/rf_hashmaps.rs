//! # Rustfinity: Hashmaps
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/hashmaps
//!
//! Hashmaps are one of the most versatile data structures in Rust, providing a way to store key-value pairs. They are part of the standard library and offer a convenient way to associate values with unique keys. In this challenge, you will implement a simple key-value store using Rust's `HashMap`.
//!
//! The `HashMap` type is part of the `std::collections` module and allows you to perform operations such as inserting, retrieving, and removing elements efficiently.
//!
//! ## Your Task
//!
//! You are tasked with implementing two functions for a key-value store:
//!
//! 1. `insert_or_update`: Inserts a new key-value pair into the `HashMap`, or updates the value if the key already exists.
//! 2. `get_value`: Retrieves a value by its key. If the key is not present, return `None`.
//!
//! You will write both functions to work with a `HashMap<String, String>`.
//!
//! ### Requirements
//!
//! #### insert_or_update
//!
//! - Accepts a mutable reference to a `HashMap<String, String>`, a key of type `String`, and a value of type `String`.
//! - If the key exists in the map, updates its value.
//! - If the key does not exist, inserts the new key-value pair.
//!
//! #### get_value
//!
//! - Accepts an immutable reference to a `HashMap<String, String>` and a key of type `String`.
//! - Returns an `Option<String>`:
//!   - `Some(value)` if the key is present.
//!   - `None` if the key is absent.
//!
//! ## Hints
//!
//! <details>
//! <summary>Click here to reveal hints</summary>
//!
//! - Use the `HashMap` type from `std::collections`.
//! - To insert or update a value, use the `.insert()` method of `HashMap`.
//! - To retrieve a value by its key, use the `.get()` method of `HashMap`.
//!
//! </details>
//!
//! Run: `cargo run --example rf_hashmaps`
//! Test: `cargo test --example rf_hashmaps`
//! Filter: `cargo test --example rf_hashmaps <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::collections::HashMap;

/// Inserts a key-value pair into the hashmap or updates the value if the key exists.
pub fn insert_or_update(map: &mut HashMap<String, String>, key: String, value: String) {
    // Your code here...
    unimplemented!()
}

/// Retrieves the value associated with a key from the hashmap.
pub fn get_value(map: &HashMap<String, String>, key: String) -> Option<String> {
    // Your code here...
    unimplemented!()
}

// Example usage
pub fn main() {
    let mut store = HashMap::new();

    // Insert a new key-value pair
    insert_or_update(&mut store, "name".to_string(), "Alice".to_string());

    // Update an existing key
    insert_or_update(&mut store, "name".to_string(), "Bob".to_string());

    // Retrieve the value by key
    let value = get_value(&store, "name".to_string());
    assert_eq!(value, Some("Bob".to_string()));

    // Try to get a non-existent key
    let missing = get_value(&store, "age".to_string());
    assert_eq!(missing, None);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::collections::HashMap;

    #[test]
    fn test_insert_or_update() {
        let mut store = HashMap::new();

        // Insert a new key-value pair
        insert_or_update(&mut store, "name".to_string(), "Alice".to_string());
        assert_eq!(store.get("name"), Some(&"Alice".to_string()));

        // Update an existing key
        insert_or_update(&mut store, "name".to_string(), "Bob".to_string());
        assert_eq!(store.get("name"), Some(&"Bob".to_string()));

        // Insert another key-value pair
        insert_or_update(&mut store, "age".to_string(), "25".to_string());
        assert_eq!(store.get("age"), Some(&"25".to_string()));

        // Update another key
        insert_or_update(&mut store, "age".to_string(), "30".to_string());
        assert_eq!(store.get("age"), Some(&"30".to_string()));
    }

    #[test]
    fn test_get_value() {
        let mut store = HashMap::new();
        store.insert("city".to_string(), "Paris".to_string());
        store.insert("country".to_string(), "France".to_string());

        // Retrieve existing key
        let value = get_value(&store, "city".to_string());
        assert_eq!(value, Some("Paris".to_string()));

        // Attempt to retrieve a non-existent key
        let missing = get_value(&store, "population".to_string());
        assert_eq!(missing, None);

        // Retrieve another existing key
        let value = get_value(&store, "country".to_string());
        assert_eq!(value, Some("France".to_string()));
    }

    #[test]
    fn test_empty_map() {
        let store: HashMap<String, String> = HashMap::new();

        // Attempt to retrieve a value from an empty map
        let value = get_value(&store, "key".to_string());
        assert_eq!(value, None);
    }

    #[test]
    fn test_overwrite_behavior() {
        let mut store = HashMap::new();

        // Insert a value and overwrite it multiple times
        insert_or_update(&mut store, "key".to_string(), "Value1".to_string());
        assert_eq!(store.get("key"), Some(&"Value1".to_string()));

        insert_or_update(&mut store, "key".to_string(), "Value2".to_string());
        assert_eq!(store.get("key"), Some(&"Value2".to_string()));

        insert_or_update(&mut store, "key".to_string(), "Value3".to_string());
        assert_eq!(store.get("key"), Some(&"Value3".to_string()));
    }

    #[test]
    fn test_large_map() {
        let mut store = HashMap::new();

        // Insert a large number of key-value pairs
        for i in 0..1000 {
            let key = format!("key{}", i);
            let value = format!("value{}", i);
            insert_or_update(&mut store, key.clone(), value.clone());
            assert_eq!(store.get(&key), Some(&value));
        }

        // Verify all keys are present
        for i in 0..1000 {
            let key = format!("key{}", i);
            let value = format!("value{}", i);
            assert_eq!(get_value(&store, key), Some(value));
        }
    }

    #[test]
    fn test_unicode_keys_and_values() {
        let mut store = HashMap::new();

        // Insert and retrieve keys and values with Unicode characters
        insert_or_update(&mut store, "ключ".to_string(), "значение".to_string());
        assert_eq!(
            get_value(&store, "ключ".to_string()),
            Some("значение".to_string())
        );

        insert_or_update(&mut store, "emoji".to_string(), "😊".to_string());
        assert_eq!(
            get_value(&store, "emoji".to_string()),
            Some("😊".to_string())
        );
    }
}
