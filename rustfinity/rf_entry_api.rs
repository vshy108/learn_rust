//! # Rustfinity: Entry Api
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/entry-api
//!
//! The Entry API is one of Rust's most elegant patterns for working with maps (`HashMap` and `BTreeMap`). It solves a common problem: when you want to insert a value into a map, but only if the key doesn't already exist, or when you want to modify an existing value, the naive approach requires two separate lookups - one to check if the key exists, and another to insert or modify. The Entry API combines these operations into a single, efficient lookup.
//!
//! When you call `.entry(key)` on a map, you get an `Entry` enum that represents either a vacant slot (the key doesn't exist) or an occupied slot (the key exists with a value). This enum provides methods like `.or_insert()`, `.or_insert_with()`, `.or_default()`, and `.and_modify()` that let you handle both cases elegantly without redundant lookups.
//!
//! The Entry API is particularly useful for building frequency counters, caches with computed values, and any scenario where you need to accumulate or aggregate values by key. It's a fundamental pattern that every Rust developer should master.
//!
//! ```rust
//! use std::collections::HashMap;
//!
//! let mut word_counts: HashMap<&str, u32> = HashMap::new();
//! let words = [
//!     "apple", "banana", "apple", "cherry", "banana", "apple"
//! ];
//!
//! // Without Entry API (inefficient - two lookups per word):
//! for word in &words {
//!     if word_counts.contains_key(word) {
//!         *word_counts.get_mut(word).unwrap() += 1;
//!     } else {
//!         word_counts.insert(word, 1);
//!     }
//! }
//!
//! // With Entry API (efficient - single lookup per word):
//! let mut word_counts: HashMap<&str, u32> = HashMap::new();
//! for word in &words {
//!     *word_counts.entry(word).or_insert(0) += 1;
//! }
//!
//! assert_eq!(word_counts["apple"], 3);
//! assert_eq!(word_counts["banana"], 2);
//! assert_eq!(word_counts["cherry"], 1);
//! ```
//!
//! ## Your Task
//!
//! Implement the following functions that demonstrate various Entry API patterns:
//!
//! 1. `count_words(text: &str) -> HashMap<String, u32>` - Count the frequency of each word in the text. Words should be lowercased. Split on whitespace.
//!
//! 2. `group_by_length(words: &[&str]) -> HashMap<usize, Vec<String>>` - Group words by their length. Each key is a length, and the value is a vector of words with that length.
//!
//! 3. `get_or_compute<F>(cache: &mut HashMap<String, i32>, key: &str, compute: F) -> i32` - Get a cached value or compute and cache it. The `compute` function should only be called if the key doesn't exist. Return the value (either cached or newly computed).
//!
//! 4. `increment_or_init(map: &mut HashMap<String, i32>, key: &str, init: i32)` - If the key exists, increment its value by 1. If it doesn't exist, insert the initial value.
//!
//! 5. `merge_maps(map1: HashMap<String, i32>, map2: HashMap<String, i32>) -> HashMap<String, i32>` - Merge two maps. If a key exists in both, sum the values.
//!
//! 6. `first_occurrence(items: &[&str]) -> HashMap<String, usize>` - Create a map where each key is an item and the value is the index of its first occurrence. Don't overwrite if the item was already seen.
//!
//! 7. `update_or_default(map: &mut HashMap<String, Vec<i32>>, key: &str, value: i32)` - Add a value to the vector at the given key. If the key doesn't exist, create a new vector with that value.
//!
//! ## Examples
//!
//! ```rust
//! use std::collections::HashMap;
//!
//! // count_words
//! let counts = count_words(
//!     "the quick brown fox jumps over the lazy dog"
//! );
//! assert_eq!(counts["the"], 2);
//! assert_eq!(counts["quick"], 1);
//!
//! // group_by_length
//! let grouped = group_by_length(
//!     &["hi", "hello", "hey", "world"]
//! );
//! assert_eq!(grouped[&2], vec!["hi"]);
//! assert_eq!(grouped[&5], vec!["hello", "world"]);
//! assert_eq!(grouped[&3], vec!["hey"]);
//!
//! // get_or_compute
//! let mut cache: HashMap<String, i32> = HashMap::new();
//! let value = get_or_compute(
//!     &mut cache,
//!     "answer",
//!     || 42
//! );
//! assert_eq!(value, 42);
//! assert_eq!(cache["answer"], 42);
//!
//! // increment_or_init
//! let mut scores: HashMap<String, i32> = HashMap::new();
//! // Inserts 100
//! increment_or_init(&mut scores, "alice", 100);
//! // Increments to 101
//! increment_or_init(&mut scores, "alice", 100);
//! assert_eq!(scores["alice"], 101);
//!
//! // merge_maps
//! let map1: HashMap<String, i32> = [
//!     ("a".to_string(), 1),
//!     ("b".to_string(), 2)
//! ].into();
//! let map2: HashMap<String, i32> = [
//!     ("b".to_string(), 3),
//!     ("c".to_string(), 4)
//! ].into();
//! let merged = merge_maps(map1, map2);
//! assert_eq!(merged["a"], 1);
//! assert_eq!(merged["b"], 5);  // 2 + 3
//! assert_eq!(merged["c"], 4);
//!
//! // first_occurrence
//! let firsts = first_occurrence(
//!     &["a", "b", "a", "c", "b"]
//! );
//! assert_eq!(firsts["a"], 0);
//! assert_eq!(firsts["b"], 1);
//! assert_eq!(firsts["c"], 3);
//!
//! // update_or_default
//! let mut groups: HashMap<String, Vec<i32>> = HashMap::new();
//! update_or_default(&mut groups, "evens", 2);
//! update_or_default(&mut groups, "evens", 4);
//! assert_eq!(groups["evens"], vec![2, 4]);
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - For `count_words`, use `.entry(word).or_insert(0)` and then dereference and increment
//! - For `group_by_length`, use `.entry(len).or_default()` (Vec has a Default implementation) and then push
//! - For `get_or_compute`, use `.or_insert_with(compute)` to lazily evaluate the closure only when needed
//! - For `increment_or_init`, use `.and_modify(|v| *v += 1).or_insert(init)` to handle both cases
//! - For `merge_maps`, iterate over one map and use the Entry API to merge into the result
//! - For `first_occurrence`, use `.or_insert(index)` which only inserts if the key is vacant
//! - For `update_or_default`, use `.or_default()` to get an empty Vec, then push the value
//! - The `Entry::or_insert` method returns a mutable reference to the value, which you can then modify
//!
//! </details>
//!
//! Run: `cargo run --example rf_entry_api`
//! Test: `cargo test --example rf_entry_api`
//! Filter: `cargo test --example rf_entry_api <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::collections::HashMap;

/// Counts the frequency of each word in the text.
/// Words are converted to lowercase and split on whitespace.
pub fn count_words(text: &str) -> HashMap<String, u32> {
    // TODO: Use the Entry API to count word frequencies
    unimplemented!()
}

/// Groups words by their length.
pub fn group_by_length(words: &[&str]) -> HashMap<usize, Vec<String>> {
    // TODO: Use the Entry API to group words by their length
    unimplemented!()
}

/// Gets a cached value or computes and caches it if not present.
/// The compute function is only called if the key doesn't exist.
pub fn get_or_compute<F>(cache: &mut HashMap<String, i32>, key: &str, compute: F) -> i32
where
    F: FnOnce() -> i32,
{
    // TODO: Use .or_insert_with() for lazy evaluation
    unimplemented!()
}

/// Increments the value at the key by 1 if it exists, otherwise inserts the initial value.
pub fn increment_or_init(map: &mut HashMap<String, i32>, key: &str, init: i32) {
    // TODO: Use .and_modify() combined with .or_insert()
    unimplemented!()
}

/// Merges two HashMaps. If a key exists in both, the values are summed.
pub fn merge_maps(map1: HashMap<String, i32>, map2: HashMap<String, i32>) -> HashMap<String, i32> {
    // TODO: Use the Entry API to merge maps
    unimplemented!()
}

/// Creates a map where each key is an item and the value is the index of its first occurrence.
pub fn first_occurrence(items: &[&str]) -> HashMap<String, usize> {
    // TODO: Use .or_insert() to only insert the first occurrence index
    unimplemented!()
}

/// Adds a value to the vector at the given key.
/// If the key doesn't exist, creates a new vector with that value.
pub fn update_or_default(map: &mut HashMap<String, Vec<i32>>, key: &str, value: i32) {
    // TODO: Use .or_default()
    unimplemented!()
}

// Example usage
pub fn main() {
    // count_words example
    let counts = count_words("the quick brown fox jumps over the lazy dog");
    println!("Word counts: {:?}", counts);

    // group_by_length example
    let grouped = group_by_length(&["hi", "hello", "hey", "world"]);
    println!("Grouped by length: {:?}", grouped);

    // get_or_compute example
    let mut cache: HashMap<String, i32> = HashMap::new();
    let value = get_or_compute(&mut cache, "answer", || {
        println!("Computing expensive value...");
        42
    });
    println!("Cached value: {}", value);

    // increment_or_init example
    let mut scores: HashMap<String, i32> = HashMap::new();
    increment_or_init(&mut scores, "alice", 100);
    increment_or_init(&mut scores, "alice", 100);
    println!("Alice's score: {}", scores["alice"]);

    // merge_maps example
    let map1: HashMap<String, i32> = [("a".to_string(), 1), ("b".to_string(), 2)].into();
    let map2: HashMap<String, i32> = [("b".to_string(), 3), ("c".to_string(), 4)].into();
    let merged = merge_maps(map1, map2);
    println!("Merged: {:?}", merged);

    // first_occurrence example
    let firsts = first_occurrence(&["a", "b", "a", "c", "b"]);
    println!("First occurrences: {:?}", firsts);

    // update_or_default example
    let mut groups: HashMap<String, Vec<i32>> = HashMap::new();
    update_or_default(&mut groups, "evens", 2);
    update_or_default(&mut groups, "evens", 4);
    println!("Groups: {:?}", groups);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::collections::HashMap;

    // ==================== count_words tests ====================

    #[test]
    fn count_words_single_word() {
        let counts = count_words("hello");
        assert_eq!(counts.len(), 1);
        assert_eq!(counts["hello"], 1);
    }

    #[test]
    fn count_words_multiple_different_words() {
        let counts = count_words("hello world");
        assert_eq!(counts.len(), 2);
        assert_eq!(counts["hello"], 1);
        assert_eq!(counts["world"], 1);
    }

    #[test]
    fn count_words_with_duplicates() {
        let counts = count_words("the quick brown fox jumps over the lazy dog");
        assert_eq!(counts["the"], 2);
        assert_eq!(counts["quick"], 1);
        assert_eq!(counts["fox"], 1);
    }

    #[test]
    fn count_words_case_insensitive() {
        let counts = count_words("Hello HELLO hello");
        assert_eq!(counts.len(), 1);
        assert_eq!(counts["hello"], 3);
    }

    #[test]
    fn count_words_empty_string() {
        let counts = count_words("");
        assert!(counts.is_empty());
    }

    #[test]
    fn count_words_whitespace_only() {
        let counts = count_words("   \t\n  ");
        assert!(counts.is_empty());
    }

    #[test]
    fn count_words_many_duplicates() {
        let counts = count_words("a a a a a b b b c");
        assert_eq!(counts["a"], 5);
        assert_eq!(counts["b"], 3);
        assert_eq!(counts["c"], 1);
    }

    // ==================== group_by_length tests ====================

    #[test]
    fn group_by_length_single_word() {
        let grouped = group_by_length(&["hello"]);
        assert_eq!(grouped.len(), 1);
        assert_eq!(grouped[&5], vec!["hello"]);
    }

    #[test]
    fn group_by_length_same_length() {
        let grouped = group_by_length(&["hello", "world", "again"]);
        assert_eq!(grouped.len(), 1);
        assert_eq!(grouped[&5], vec!["hello", "world", "again"]);
    }

    #[test]
    fn group_by_length_different_lengths() {
        let grouped = group_by_length(&["hi", "hello", "hey", "world"]);
        assert_eq!(grouped[&2], vec!["hi"]);
        assert_eq!(grouped[&5], vec!["hello", "world"]);
        assert_eq!(grouped[&3], vec!["hey"]);
    }

    #[test]
    fn group_by_length_empty_slice() {
        let grouped = group_by_length(&[]);
        assert!(grouped.is_empty());
    }

    #[test]
    fn group_by_length_with_empty_string() {
        let grouped = group_by_length(&["", "a", "ab"]);
        assert_eq!(grouped[&0], vec![""]);
        assert_eq!(grouped[&1], vec!["a"]);
        assert_eq!(grouped[&2], vec!["ab"]);
    }

    #[test]
    fn group_by_length_preserves_order() {
        let grouped = group_by_length(&["cat", "dog", "bat"]);
        assert_eq!(grouped[&3], vec!["cat", "dog", "bat"]);
    }

    // ==================== get_or_compute tests ====================

    #[test]
    fn get_or_compute_inserts_new_value() {
        let mut cache: HashMap<String, i32> = HashMap::new();
        let value = get_or_compute(&mut cache, "key", || 42);
        assert_eq!(value, 42);
        assert_eq!(cache["key"], 42);
    }

    #[test]
    fn get_or_compute_returns_existing() {
        let mut cache: HashMap<String, i32> = HashMap::new();
        cache.insert("key".to_string(), 100);
        let value = get_or_compute(&mut cache, "key", || 42);
        assert_eq!(value, 100);
    }

    #[test]
    fn get_or_compute_lazy_evaluation() {
        let mut cache: HashMap<String, i32> = HashMap::new();
        cache.insert("key".to_string(), 100);
        let mut called = false;
        let value = get_or_compute(&mut cache, "key", || {
            called = true;
            42
        });
        assert_eq!(value, 100);
        assert!(
            !called,
            "Compute function should not be called for existing key"
        );
    }

    #[test]
    fn get_or_compute_calls_closure_for_missing() {
        let mut cache: HashMap<String, i32> = HashMap::new();
        let mut call_count = 0;
        let _ = get_or_compute(&mut cache, "key", || {
            call_count += 1;
            42
        });
        assert_eq!(call_count, 1);
    }

    #[test]
    fn get_or_compute_multiple_keys() {
        let mut cache: HashMap<String, i32> = HashMap::new();
        let v1 = get_or_compute(&mut cache, "a", || 1);
        let v2 = get_or_compute(&mut cache, "b", || 2);
        let v3 = get_or_compute(&mut cache, "a", || 100);
        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
        assert_eq!(v3, 1); // Should return cached value, not 100
    }

    // ==================== increment_or_init tests ====================

    #[test]
    fn increment_or_init_inserts_initial() {
        let mut map: HashMap<String, i32> = HashMap::new();
        increment_or_init(&mut map, "key", 100);
        assert_eq!(map["key"], 100);
    }

    #[test]
    fn increment_or_init_increments_existing() {
        let mut map: HashMap<String, i32> = HashMap::new();
        map.insert("key".to_string(), 50);
        increment_or_init(&mut map, "key", 100);
        assert_eq!(map["key"], 51);
    }

    #[test]
    fn increment_or_init_multiple_increments() {
        let mut map: HashMap<String, i32> = HashMap::new();
        increment_or_init(&mut map, "key", 0);
        increment_or_init(&mut map, "key", 0);
        increment_or_init(&mut map, "key", 0);
        assert_eq!(map["key"], 2); // Init + 2 increments
    }

    #[test]
    fn increment_or_init_different_keys() {
        let mut map: HashMap<String, i32> = HashMap::new();
        increment_or_init(&mut map, "a", 10);
        increment_or_init(&mut map, "b", 20);
        increment_or_init(&mut map, "a", 10);
        assert_eq!(map["a"], 11);
        assert_eq!(map["b"], 20);
    }

    #[test]
    fn increment_or_init_negative_init() {
        let mut map: HashMap<String, i32> = HashMap::new();
        increment_or_init(&mut map, "key", -5);
        assert_eq!(map["key"], -5);
        increment_or_init(&mut map, "key", -5);
        assert_eq!(map["key"], -4);
    }

    // ==================== merge_maps tests ====================

    #[test]
    fn merge_maps_disjoint() {
        let map1: HashMap<String, i32> = [("a".to_string(), 1)].into();
        let map2: HashMap<String, i32> = [("b".to_string(), 2)].into();
        let merged = merge_maps(map1, map2);
        assert_eq!(merged["a"], 1);
        assert_eq!(merged["b"], 2);
    }

    #[test]
    fn merge_maps_overlapping() {
        let map1: HashMap<String, i32> = [("a".to_string(), 1), ("b".to_string(), 2)].into();
        let map2: HashMap<String, i32> = [("b".to_string(), 3), ("c".to_string(), 4)].into();
        let merged = merge_maps(map1, map2);
        assert_eq!(merged["a"], 1);
        assert_eq!(merged["b"], 5);
        assert_eq!(merged["c"], 4);
    }

    #[test]
    fn merge_maps_empty_first() {
        let map1: HashMap<String, i32> = HashMap::new();
        let map2: HashMap<String, i32> = [("a".to_string(), 1)].into();
        let merged = merge_maps(map1, map2);
        assert_eq!(merged["a"], 1);
    }

    #[test]
    fn merge_maps_empty_second() {
        let map1: HashMap<String, i32> = [("a".to_string(), 1)].into();
        let map2: HashMap<String, i32> = HashMap::new();
        let merged = merge_maps(map1, map2);
        assert_eq!(merged["a"], 1);
    }

    #[test]
    fn merge_maps_both_empty() {
        let map1: HashMap<String, i32> = HashMap::new();
        let map2: HashMap<String, i32> = HashMap::new();
        let merged = merge_maps(map1, map2);
        assert!(merged.is_empty());
    }

    #[test]
    fn merge_maps_negative_values() {
        let map1: HashMap<String, i32> = [("a".to_string(), -5)].into();
        let map2: HashMap<String, i32> = [("a".to_string(), 3)].into();
        let merged = merge_maps(map1, map2);
        assert_eq!(merged["a"], -2);
    }

    #[test]
    fn merge_maps_same_keys() {
        let map1: HashMap<String, i32> = [("x".to_string(), 10), ("y".to_string(), 20)].into();
        let map2: HashMap<String, i32> = [("x".to_string(), 5), ("y".to_string(), 15)].into();
        let merged = merge_maps(map1, map2);
        assert_eq!(merged["x"], 15);
        assert_eq!(merged["y"], 35);
    }

    // ==================== first_occurrence tests ====================

    #[test]
    fn first_occurrence_all_unique() {
        let firsts = first_occurrence(&["a", "b", "c"]);
        assert_eq!(firsts["a"], 0);
        assert_eq!(firsts["b"], 1);
        assert_eq!(firsts["c"], 2);
    }

    #[test]
    fn first_occurrence_with_duplicates() {
        let firsts = first_occurrence(&["a", "b", "a", "c", "b"]);
        assert_eq!(firsts["a"], 0);
        assert_eq!(firsts["b"], 1);
        assert_eq!(firsts["c"], 3);
    }

    #[test]
    fn first_occurrence_all_same() {
        let firsts = first_occurrence(&["x", "x", "x"]);
        assert_eq!(firsts.len(), 1);
        assert_eq!(firsts["x"], 0);
    }

    #[test]
    fn first_occurrence_empty() {
        let firsts = first_occurrence(&[]);
        assert!(firsts.is_empty());
    }

    #[test]
    fn first_occurrence_single() {
        let firsts = first_occurrence(&["only"]);
        assert_eq!(firsts["only"], 0);
    }

    #[test]
    fn first_occurrence_late_duplicate() {
        let firsts = first_occurrence(&["a", "b", "c", "d", "e", "a"]);
        assert_eq!(firsts["a"], 0);
        assert_eq!(firsts["e"], 4);
    }

    // ==================== update_or_default tests ====================

    #[test]
    fn update_or_default_new_key() {
        let mut map: HashMap<String, Vec<i32>> = HashMap::new();
        update_or_default(&mut map, "key", 42);
        assert_eq!(map["key"], vec![42]);
    }

    #[test]
    fn update_or_default_existing_key() {
        let mut map: HashMap<String, Vec<i32>> = HashMap::new();
        map.insert("key".to_string(), vec![1, 2]);
        update_or_default(&mut map, "key", 3);
        assert_eq!(map["key"], vec![1, 2, 3]);
    }

    #[test]
    fn update_or_default_multiple_adds() {
        let mut map: HashMap<String, Vec<i32>> = HashMap::new();
        update_or_default(&mut map, "nums", 1);
        update_or_default(&mut map, "nums", 2);
        update_or_default(&mut map, "nums", 3);
        assert_eq!(map["nums"], vec![1, 2, 3]);
    }

    #[test]
    fn update_or_default_different_keys() {
        let mut map: HashMap<String, Vec<i32>> = HashMap::new();
        update_or_default(&mut map, "a", 1);
        update_or_default(&mut map, "b", 2);
        update_or_default(&mut map, "a", 3);
        assert_eq!(map["a"], vec![1, 3]);
        assert_eq!(map["b"], vec![2]);
    }

    #[test]
    fn update_or_default_negative_values() {
        let mut map: HashMap<String, Vec<i32>> = HashMap::new();
        update_or_default(&mut map, "key", -1);
        update_or_default(&mut map, "key", -2);
        assert_eq!(map["key"], vec![-1, -2]);
    }

    // ==================== Integration tests ====================

    #[test]
    fn integration_word_frequency_analysis() {
        let text = "rust is great rust is fast rust is safe";
        let counts = count_words(text);

        assert_eq!(counts["rust"], 3);
        assert_eq!(counts["is"], 3);
        assert_eq!(counts["great"], 1);
        assert_eq!(counts["fast"], 1);
        assert_eq!(counts["safe"], 1);
    }

    #[test]
    fn integration_cache_pattern() {
        let mut cache: HashMap<String, i32> = HashMap::new();

        // First access computes
        let v1 = get_or_compute(&mut cache, "expensive", || 1000);
        assert_eq!(v1, 1000);

        // Second access returns cached
        let v2 = get_or_compute(&mut cache, "expensive", || 9999);
        assert_eq!(v2, 1000);

        // Different key computes new value
        let v3 = get_or_compute(&mut cache, "another", || 2000);
        assert_eq!(v3, 2000);
    }

    #[test]
    fn integration_score_tracking() {
        let mut scores: HashMap<String, i32> = HashMap::new();

        // Initialize players
        increment_or_init(&mut scores, "alice", 100);
        increment_or_init(&mut scores, "bob", 100);

        // Alice scores multiple times
        increment_or_init(&mut scores, "alice", 0);
        increment_or_init(&mut scores, "alice", 0);

        // Bob scores once
        increment_or_init(&mut scores, "bob", 0);

        assert_eq!(scores["alice"], 102);
        assert_eq!(scores["bob"], 101);
    }

    #[test]
    fn integration_event_grouping() {
        let mut events: HashMap<String, Vec<i32>> = HashMap::new();

        update_or_default(&mut events, "errors", 404);
        update_or_default(&mut events, "errors", 500);
        update_or_default(&mut events, "success", 200);
        update_or_default(&mut events, "errors", 403);

        assert_eq!(events["errors"], vec![404, 500, 403]);
        assert_eq!(events["success"], vec![200]);
    }

    #[test]
    fn integration_complex_merge() {
        let daily: HashMap<String, i32> =
            [("views".to_string(), 100), ("clicks".to_string(), 10)].into();

        let hourly: HashMap<String, i32> =
            [("views".to_string(), 50), ("shares".to_string(), 5)].into();

        let total = merge_maps(daily, hourly);
        assert_eq!(total["views"], 150);
        assert_eq!(total["clicks"], 10);
        assert_eq!(total["shares"], 5);
    }
}
