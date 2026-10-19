//! # Rustfinity: Btreemap Basics
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/btreemap-basics
//!
//! `BTreeMap<K, V>` is Rust's implementation of an ordered map based on a B-Tree data structure. Unlike `HashMap`, which stores entries in arbitrary order for O(1) average lookups, `BTreeMap` keeps its keys sorted and provides O(log n) operations. This ordering comes with a powerful advantage: you can efficiently iterate over keys in sorted order and perform range queries.
//!
//! The `BTreeMap` type is found in `std::collections` and requires that keys implement the `Ord` trait (total ordering). This means you can use it with types that have a natural ordering like integers, strings, and any custom types that implement `Ord`.
//!
//! One of the most useful features of `BTreeMap` is the `.range()` method, which allows you to iterate over a subset of entries within a specified key range. This is particularly useful for time-series data, finding entries between two values, or implementing features like autocomplete with prefix matching.
//!
//! ```rust
//! use std::collections::BTreeMap;
//!
//! let mut scores: BTreeMap<&str, i32> = BTreeMap::new();
//! scores.insert("Alice", 95);
//! scores.insert("Bob", 87);
//! scores.insert("Charlie", 92);
//!
//! // Keys are always sorted when iterating
//! for (name, score) in &scores {
//!     println!("{}: {}", name, score);
//! }
//! // Prints in alphabetical order:
//! // Alice: 95, Bob: 87, Charlie: 92
//!
//! // Range queries
//! use std::ops::Bound::{Included, Excluded};
//! for (name, score) in scores.range("B".."D") {
//!     println!("{}: {}", name, score);
//! }
//! // Prints: Bob: 87, Charlie: 92
//! ```
//!
//! ## Your Task
//!
//! Implement the following functions for working with `BTreeMap`:
//!
//! 1. `create_sorted_map(
//!  pairs: &[(String, i32)]
//! ) -> BTreeMap<String, i32>`
//!    - Create a BTreeMap from a slice of key-value
//!      pairs
//!
//! 2. `get_value(
//!  map: &BTreeMap<String, i32>,
//!  key: &str
//! ) -> Option<i32>`
//!    - Get a value by key, returning `None` if the
//!      key doesn't exist
//!
//! 3. `get_keys_in_order(
//!  map: &BTreeMap<String, i32>
//! ) -> Vec<String>`
//!    - Return all keys in sorted order
//!
//! 4. `get_values_in_key_order(
//!  map: &BTreeMap<String, i32>
//! ) -> Vec<i32>`
//!    - Return all values in the order of their
//!      sorted keys
//!
//! 5. `get_range(
//!  map: &BTreeMap<String, i32>,
//!  start: &str,
//!  end: &str
//! ) -> Vec<(String, i32)>`
//!    - Return all key-value pairs where the key is
//!      > = start and < end (half-open range)
//!
//! 6. `get_first(
//!  map: &BTreeMap<String, i32>
//! ) -> Option<(String, i32)>`
//!    - Return the first (smallest key) entry
//!
//! 7. `get_last(
//!  map: &BTreeMap<String, i32>
//! ) -> Option<(String, i32)>`
//!    - Return the last (largest key) entry
//!
//! ## Examples
//!
//! ```rust
//! use std::collections::BTreeMap;
//!
//! // create_sorted_map
//! let pairs = vec![
//!     ("cherry".to_string(), 3),
//!     ("apple".to_string(), 1),
//!     ("banana".to_string(), 2),
//! ];
//! let map = create_sorted_map(&pairs);
//! assert_eq!(map.len(), 3);
//!
//! // get_value
//! assert_eq!(get_value(&map, "apple"), Some(1));
//! assert_eq!(get_value(&map, "grape"), None);
//!
//! // get_keys_in_order
//! // Returns keys sorted alphabetically
//! let keys = get_keys_in_order(&map);
//! assert_eq!(
//!     keys,
//!     vec!["apple", "banana", "cherry"]
//! );
//!
//! // get_values_in_key_order
//! // Returns values in order of sorted keys
//! let values = get_values_in_key_order(&map);
//! assert_eq!(
//!     values,
//!     vec![1, 2, 3]
//! );
//!
//! // get_range - half-open range [start, end)
//! let range = get_range(&map, "apple", "cherry");
//! assert_eq!(
//!     range,
//!     vec![
//!         ("apple".to_string(), 1),
//!         ("banana".to_string(), 2),
//!     ]
//! );
//!
//! // get_first - smallest key
//! assert_eq!(
//!     get_first(&map),
//!     Some(("apple".to_string(), 1))
//! );
//!
//! // get_last - largest key
//! assert_eq!(
//!     get_last(&map),
//!     Some(("cherry".to_string(), 3))
//! );
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - For `create_sorted_map`, you can iterate over
//!   the slice and insert each pair, or use
//!   `.iter().cloned().collect()`
//! - For `get_value`, use the `.get()` method which
//!   returns an `Option<&V>`, then use `.copied()`
//!   to get `Option<V>`
//! - For `get_keys_in_order`, iterate over `.keys()`
//!   and collect into a Vec
//! - For `get_values_in_key_order`, iterate over
//!   `.values()` and collect into a Vec
//! - For `get_range`, use the `.range()` method
//!   with a range pattern like `start..end`
//! - For `get_first` and `get_last`, use
//!   `.first_key_value()` and `.last_key_value()`
//!   methods, which return `Option<(&K, &V)>`
//! - Remember that `BTreeMap` iteration methods
//!   return references, so you may need to clone
//!   values when collecting
//!
//! </details>
//!
//! Run: `cargo run --example rf_btreemap_basics`
//! Test: `cargo test --example rf_btreemap_basics`
//! Filter: `cargo test --example rf_btreemap_basics <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::collections::BTreeMap;

/// Creates a BTreeMap from a slice of key-value pairs.
///
/// The BTreeMap will automatically sort entries by key.
pub fn create_sorted_map(pairs: &[(String, i32)]) -> BTreeMap<String, i32> {
    // TODO: Collect pairs into a BTreeMap
    unimplemented!()
}

/// Gets a value from the map by key.
///
/// Returns `Some(value)` if the key exists, `None` otherwise.
pub fn get_value(map: &BTreeMap<String, i32>, key: &str) -> Option<i32> {
    // TODO: Look up the key
    unimplemented!()
}

/// Returns all keys from the map in sorted order.
pub fn get_keys_in_order(map: &BTreeMap<String, i32>) -> Vec<String> {
    // TODO: Get all keys
    unimplemented!()
}

/// Returns all values from the map in the order of their sorted keys.
pub fn get_values_in_key_order(map: &BTreeMap<String, i32>) -> Vec<i32> {
    // TODO: Get all values
    unimplemented!()
}

/// Returns all key-value pairs where the key is in the range [start, end).
///
/// This is a half-open range: includes start, excludes end.
pub fn get_range(map: &BTreeMap<String, i32>, start: &str, end: &str) -> Vec<(String, i32)> {
    // TODO: Get entries in the range
    unimplemented!()
}

/// Returns the first (smallest key) entry in the map.
pub fn get_first(map: &BTreeMap<String, i32>) -> Option<(String, i32)> {
    // TODO: Get the first entry
    unimplemented!()
}

/// Returns the last (largest key) entry in the map.
pub fn get_last(map: &BTreeMap<String, i32>) -> Option<(String, i32)> {
    // TODO: Get the last entry
    unimplemented!()
}

// Example usage
pub fn main() {
    let pairs = vec![
        ("cherry".to_string(), 3),
        ("apple".to_string(), 1),
        ("banana".to_string(), 2),
    ];

    let map = create_sorted_map(&pairs);
    println!("Map: {:?}", map);

    println!("Value for 'apple': {:?}", get_value(&map, "apple"));
    println!("Keys in order: {:?}", get_keys_in_order(&map));
    println!("Values in key order: {:?}", get_values_in_key_order(&map));

    let range = get_range(&map, "apple", "cherry");
    println!("Range [apple, cherry): {:?}", range);

    println!("First entry: {:?}", get_first(&map));
    println!("Last entry: {:?}", get_last(&map));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::collections::BTreeMap;

    // ============================================================================
    // Tests for create_sorted_map
    // ============================================================================

    #[test]
    fn test_create_sorted_map_basic() {
        let pairs = vec![
            ("cherry".to_string(), 3),
            ("apple".to_string(), 1),
            ("banana".to_string(), 2),
        ];
        let map = create_sorted_map(&pairs);
        assert_eq!(map.len(), 3);
        assert_eq!(map.get("apple"), Some(&1));
        assert_eq!(map.get("banana"), Some(&2));
        assert_eq!(map.get("cherry"), Some(&3));
    }

    #[test]
    fn test_create_sorted_map_empty() {
        let pairs: Vec<(String, i32)> = vec![];
        let map = create_sorted_map(&pairs);
        assert!(map.is_empty());
    }

    #[test]
    fn test_create_sorted_map_single() {
        let pairs = vec![("only".to_string(), 42)];
        let map = create_sorted_map(&pairs);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get("only"), Some(&42));
    }

    #[test]
    fn test_create_sorted_map_duplicate_keys() {
        // Later duplicate overwrites earlier
        let pairs = vec![("key".to_string(), 1), ("key".to_string(), 2)];
        let map = create_sorted_map(&pairs);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get("key"), Some(&2));
    }

    #[test]
    fn test_create_sorted_map_maintains_order() {
        let pairs = vec![
            ("zebra".to_string(), 4),
            ("alpha".to_string(), 1),
            ("mango".to_string(), 3),
            ("beta".to_string(), 2),
        ];
        let map = create_sorted_map(&pairs);
        let keys: Vec<_> = map.keys().collect();
        assert_eq!(keys, vec!["alpha", "beta", "mango", "zebra"]);
    }

    // ============================================================================
    // Tests for get_value
    // ============================================================================

    #[test]
    fn test_get_value_exists() {
        let pairs = vec![("apple".to_string(), 100), ("banana".to_string(), 200)];
        let map = create_sorted_map(&pairs);
        assert_eq!(get_value(&map, "apple"), Some(100));
        assert_eq!(get_value(&map, "banana"), Some(200));
    }

    #[test]
    fn test_get_value_not_exists() {
        let pairs = vec![("apple".to_string(), 100)];
        let map = create_sorted_map(&pairs);
        assert_eq!(get_value(&map, "grape"), None);
        assert_eq!(get_value(&map, ""), None);
    }

    #[test]
    fn test_get_value_empty_map() {
        let map: BTreeMap<String, i32> = BTreeMap::new();
        assert_eq!(get_value(&map, "anything"), None);
    }

    #[test]
    fn test_get_value_negative_values() {
        let pairs = vec![("negative".to_string(), -42), ("zero".to_string(), 0)];
        let map = create_sorted_map(&pairs);
        assert_eq!(get_value(&map, "negative"), Some(-42));
        assert_eq!(get_value(&map, "zero"), Some(0));
    }

    // ============================================================================
    // Tests for get_keys_in_order
    // ============================================================================

    #[test]
    fn test_get_keys_in_order_basic() {
        let pairs = vec![
            ("cherry".to_string(), 3),
            ("apple".to_string(), 1),
            ("banana".to_string(), 2),
        ];
        let map = create_sorted_map(&pairs);
        assert_eq!(get_keys_in_order(&map), vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn test_get_keys_in_order_empty() {
        let map: BTreeMap<String, i32> = BTreeMap::new();
        let keys: Vec<String> = get_keys_in_order(&map);
        assert!(keys.is_empty());
    }

    #[test]
    fn test_get_keys_in_order_single() {
        let pairs = vec![("single".to_string(), 1)];
        let map = create_sorted_map(&pairs);
        assert_eq!(get_keys_in_order(&map), vec!["single"]);
    }

    #[test]
    fn test_get_keys_in_order_numbers_as_strings() {
        let pairs = vec![
            ("10".to_string(), 10),
            ("2".to_string(), 2),
            ("1".to_string(), 1),
        ];
        let map = create_sorted_map(&pairs);
        // String comparison: "1" < "10" < "2" (lexicographic)
        assert_eq!(get_keys_in_order(&map), vec!["1", "10", "2"]);
    }

    // ============================================================================
    // Tests for get_values_in_key_order
    // ============================================================================

    #[test]
    fn test_get_values_in_key_order_basic() {
        let pairs = vec![
            ("cherry".to_string(), 3),
            ("apple".to_string(), 1),
            ("banana".to_string(), 2),
        ];
        let map = create_sorted_map(&pairs);
        // Values in order of sorted keys: apple(1), banana(2), cherry(3)
        assert_eq!(get_values_in_key_order(&map), vec![1, 2, 3]);
    }

    #[test]
    fn test_get_values_in_key_order_empty() {
        let map: BTreeMap<String, i32> = BTreeMap::new();
        let values: Vec<i32> = get_values_in_key_order(&map);
        assert!(values.is_empty());
    }

    #[test]
    fn test_get_values_in_key_order_same_values() {
        let pairs = vec![
            ("c".to_string(), 5),
            ("a".to_string(), 5),
            ("b".to_string(), 5),
        ];
        let map = create_sorted_map(&pairs);
        assert_eq!(get_values_in_key_order(&map), vec![5, 5, 5]);
    }

    #[test]
    fn test_get_values_in_key_order_mixed() {
        let pairs = vec![
            ("z".to_string(), -10),
            ("a".to_string(), 100),
            ("m".to_string(), 0),
        ];
        let map = create_sorted_map(&pairs);
        // Order: a(100), m(0), z(-10)
        assert_eq!(get_values_in_key_order(&map), vec![100, 0, -10]);
    }

    // ============================================================================
    // Tests for get_range
    // ============================================================================

    #[test]
    fn test_get_range_basic() {
        let pairs = vec![
            ("apple".to_string(), 1),
            ("banana".to_string(), 2),
            ("cherry".to_string(), 3),
            ("date".to_string(), 4),
        ];
        let map = create_sorted_map(&pairs);
        let range = get_range(&map, "banana", "date");
        assert_eq!(
            range,
            vec![("banana".to_string(), 2), ("cherry".to_string(), 3),]
        );
    }

    #[test]
    fn test_get_range_full() {
        let pairs = vec![
            ("a".to_string(), 1),
            ("b".to_string(), 2),
            ("c".to_string(), 3),
        ];
        let map = create_sorted_map(&pairs);
        let range = get_range(&map, "a", "d");
        assert_eq!(
            range,
            vec![
                ("a".to_string(), 1),
                ("b".to_string(), 2),
                ("c".to_string(), 3),
            ]
        );
    }

    #[test]
    fn test_get_range_empty_result() {
        let pairs = vec![("apple".to_string(), 1), ("banana".to_string(), 2)];
        let map = create_sorted_map(&pairs);
        // Range that doesn't include any keys
        let range = get_range(&map, "cherry", "date");
        assert!(range.is_empty());
    }

    #[test]
    fn test_get_range_single_element() {
        let pairs = vec![
            ("a".to_string(), 1),
            ("b".to_string(), 2),
            ("c".to_string(), 3),
        ];
        let map = create_sorted_map(&pairs);
        let range = get_range(&map, "b", "c");
        assert_eq!(range, vec![("b".to_string(), 2)]);
    }

    #[test]
    fn test_get_range_start_equals_end() {
        let pairs = vec![("a".to_string(), 1), ("b".to_string(), 2)];
        let map = create_sorted_map(&pairs);
        // Empty range when start == end
        let range = get_range(&map, "a", "a");
        assert!(range.is_empty());
    }

    #[test]
    fn test_get_range_empty_map() {
        let map: BTreeMap<String, i32> = BTreeMap::new();
        let range = get_range(&map, "a", "z");
        assert!(range.is_empty());
    }

    #[test]
    fn test_get_range_partial_match() {
        let pairs = vec![
            ("cat".to_string(), 1),
            ("dog".to_string(), 2),
            ("elephant".to_string(), 3),
        ];
        let map = create_sorted_map(&pairs);
        // Range includes some existing keys
        let range = get_range(&map, "cow", "duck");
        assert_eq!(range, vec![("dog".to_string(), 2)]);
    }

    // ============================================================================
    // Tests for get_first
    // ============================================================================

    #[test]
    fn test_get_first_basic() {
        let pairs = vec![
            ("cherry".to_string(), 3),
            ("apple".to_string(), 1),
            ("banana".to_string(), 2),
        ];
        let map = create_sorted_map(&pairs);
        assert_eq!(get_first(&map), Some(("apple".to_string(), 1)));
    }

    #[test]
    fn test_get_first_empty() {
        let map: BTreeMap<String, i32> = BTreeMap::new();
        assert_eq!(get_first(&map), None);
    }

    #[test]
    fn test_get_first_single() {
        let pairs = vec![("only".to_string(), 99)];
        let map = create_sorted_map(&pairs);
        assert_eq!(get_first(&map), Some(("only".to_string(), 99)));
    }

    #[test]
    fn test_get_first_special_characters() {
        let pairs = vec![
            ("beta".to_string(), 2),
            ("!special".to_string(), 0),
            ("alpha".to_string(), 1),
        ];
        let map = create_sorted_map(&pairs);
        // '!' comes before letters in ASCII
        assert_eq!(get_first(&map), Some(("!special".to_string(), 0)));
    }

    // ============================================================================
    // Tests for get_last
    // ============================================================================

    #[test]
    fn test_get_last_basic() {
        let pairs = vec![
            ("cherry".to_string(), 3),
            ("apple".to_string(), 1),
            ("banana".to_string(), 2),
        ];
        let map = create_sorted_map(&pairs);
        assert_eq!(get_last(&map), Some(("cherry".to_string(), 3)));
    }

    #[test]
    fn test_get_last_empty() {
        let map: BTreeMap<String, i32> = BTreeMap::new();
        assert_eq!(get_last(&map), None);
    }

    #[test]
    fn test_get_last_single() {
        let pairs = vec![("only".to_string(), 99)];
        let map = create_sorted_map(&pairs);
        assert_eq!(get_last(&map), Some(("only".to_string(), 99)));
    }

    #[test]
    fn test_get_last_unicode() {
        let pairs = vec![
            ("alpha".to_string(), 1),
            ("beta".to_string(), 2),
            ("zeta".to_string(), 3),
        ];
        let map = create_sorted_map(&pairs);
        assert_eq!(get_last(&map), Some(("zeta".to_string(), 3)));
    }

    // ============================================================================
    // Tests for first and last on same map
    // ============================================================================

    #[test]
    fn test_first_and_last_same_element() {
        let pairs = vec![("only".to_string(), 42)];
        let map = create_sorted_map(&pairs);
        assert_eq!(get_first(&map), get_last(&map));
        assert_eq!(get_first(&map), Some(("only".to_string(), 42)));
    }

    #[test]
    fn test_first_and_last_different() {
        let pairs = vec![("a".to_string(), 1), ("z".to_string(), 26)];
        let map = create_sorted_map(&pairs);
        assert_eq!(get_first(&map), Some(("a".to_string(), 1)));
        assert_eq!(get_last(&map), Some(("z".to_string(), 26)));
    }

    // ============================================================================
    // Integration tests
    // ============================================================================

    #[test]
    fn test_workflow_example() {
        // Simulate a score tracking system
        let scores = vec![
            ("player_c".to_string(), 150),
            ("player_a".to_string(), 200),
            ("player_b".to_string(), 175),
            ("player_d".to_string(), 125),
        ];
        let map = create_sorted_map(&scores);

        // Keys are sorted
        assert_eq!(
            get_keys_in_order(&map),
            vec!["player_a", "player_b", "player_c", "player_d"]
        );

        // Get a specific player's score
        assert_eq!(get_value(&map, "player_b"), Some(175));

        // Get range of players
        let range = get_range(&map, "player_a", "player_c");
        assert_eq!(range.len(), 2);

        // First player alphabetically
        assert_eq!(get_first(&map), Some(("player_a".to_string(), 200)));

        // Last player alphabetically
        assert_eq!(get_last(&map), Some(("player_d".to_string(), 125)));
    }

    #[test]
    fn test_large_map() {
        let pairs: Vec<(String, i32)> = (0..100).map(|i| (format!("key_{:03}", i), i)).collect();
        let map = create_sorted_map(&pairs);

        assert_eq!(map.len(), 100);
        assert_eq!(get_first(&map), Some(("key_000".to_string(), 0)));
        assert_eq!(get_last(&map), Some(("key_099".to_string(), 99)));

        let range = get_range(&map, "key_050", "key_055");
        assert_eq!(range.len(), 5);
        assert_eq!(range[0], ("key_050".to_string(), 50));
        assert_eq!(range[4], ("key_054".to_string(), 54));
    }
}
