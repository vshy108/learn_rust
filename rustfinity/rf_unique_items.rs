//! # Rustfinity: Unique Items
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/unique-items
//!
//! You now have a good understanding of iterators and closures and how they can be used to process collections of data.
//!
//! In this challenge, you will use iterators to filter out duplicate items from a collection of potentially redundant entries.
//!
//! ## Your Task
//!
//! Define a function that takes a collection of items (as an iterator) and returns a collection of unique items in sorted order. The items are represented as strings. You are expected to handle duplicates and ignore entries that are **empty** or consist solely of whitespace.
//!
//! Your function signature and return types must be determined by you, but the input must implement the `Iterator` trait, ensuring flexibility for different iterator sources.
//!
//! ### Requirements
//!
//! - **Filter duplicates:** Ensure that the resulting collection contains only unique items.
//! - **Ignore invalid entries:** Exclude entries that are empty or only consist of whitespace.
//! - **Sort the output:** Return the items sorted in ascending lexicographical order.
//! - **Flexibility:** The function should work with any iterator that has `String`, `&String`, or `&str` items.
//!
//! ## Hints
//!
//! <details>
//! <summary>Click here to reveal hints</summary>
//!
//! - Use trait bounds to ensure that the input iterator can provide items that can be converted to strings. e.g.
//!   ```rust
//!   fn unique_items<I, T>(items: I) -> Vec<String>
//!   where
//!         I: Iterator<Item = T>,
//!         T: AsRef<str>,
//!   {
//!       // Your code here
//!   }
//!   ```
//! - Use a `HashSet` to track unique items.
//! - Use the `filter_map` method to remove invalid entries (e.g., empty or whitespace-only strings).
//! - You can use the `trim` method on strings to handle whitespace effectively.
//! - `HashSet` provides a method `insert` that returns a `bool` indicating whether the item was already present.
//! - To sort, you can convert to a `Vec` and call the `sort` method.
//!
//! </details>
//!
//! Run: `cargo run --example rf_unique_items`
//! Test: `cargo test --example rf_unique_items`
//! Filter: `cargo test --example rf_unique_items <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

// 1. Finish the function
pub fn unique_items<I, T>(items: I) -> Vec<String>
where
    I: Iterator<Item = T>,
    T: AsRef<str>,
{
    todo!("collect unique items from the input iterator")
}

/// Example usage
pub fn main() {
    let product_ids = vec![
        "abc123".to_string(),
        "  ".to_string(),
        "def456".to_string(),
        "abc123".to_string(),
        "ghi789".to_string(),
        "ghi789".to_string(),
        "   def456".to_string(),
    ];

    let unique_ids = unique_items(product_ids.into_iter());
    assert_eq!(unique_ids, vec!["abc123", "def456", "ghi789"]);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_unique_items_basic() {
        let items = vec![
            "abc123".to_string(),
            "  ".to_string(),
            "def456".to_string(),
            "abc123".to_string(),
            "ghi789".to_string(),
            "ghi789".to_string(),
            "   def456".to_string(),
        ];

        let unique_items = unique_items(items.into_iter());
        assert_eq!(unique_items, vec!["abc123", "def456", "ghi789"]);
    }

    #[test]
    fn test_unique_items_with_empty_entries() {
        let items = vec![
            "abc123".to_string(),
            "".to_string(),
            " ".to_string(),
            "xyz987".to_string(),
            "abc123".to_string(),
        ];

        let unique_items = unique_items(items.into_iter());
        assert_eq!(unique_items, vec!["abc123", "xyz987"]);
    }

    #[test]
    fn test_unique_items_all_duplicates() {
        let items = vec![
            "dup123".to_string(),
            "dup123".to_string(),
            "dup123".to_string(),
        ];

        let unique_items = unique_items(items.into_iter());
        assert_eq!(unique_items, vec!["dup123"]);
    }

    #[test]
    fn test_unique_items_no_valid_entries() {
        let items = vec![" ".to_string(), "".to_string(), "\t".to_string()];

        let unique_items = unique_items(items.into_iter());
        assert_eq!(unique_items, Vec::<String>::new());
    }

    #[test]
    fn test_unique_items_case_sensitivity() {
        let items = vec![
            "ABC123".to_string(),
            "abc123".to_string(),
            "ABC123".to_string(),
            "def456".to_string(),
            "DEF456".to_string(),
        ];

        let unique_items = unique_items(items.into_iter());
        assert_eq!(unique_items, vec!["ABC123", "DEF456", "abc123", "def456"]);
    }

    #[test]
    fn test_unique_items_trim_and_sort() {
        let items = vec![
            "   xyz789".to_string(),
            "abc123   ".to_string(),
            "  def456  ".to_string(),
            "ghi789".to_string(),
        ];

        let unique_items = unique_items(items.into_iter());
        assert_eq!(unique_items, vec!["abc123", "def456", "ghi789", "xyz789"]);
    }

    #[test]
    fn test_unique_items_with_symbols() {
        let items = vec![
            "!@#$%^".to_string(),
            "!@#$%^".to_string(),
            "123".to_string(),
            "abc".to_string(),
            "abc".to_string(),
            "   xyz  ".to_string(),
        ];

        let unique_items = unique_items(items.into_iter());
        assert_eq!(unique_items, vec!["!@#$%^", "123", "abc", "xyz"]);
    }

    #[test]
    fn test_unique_items_mixed_whitespace() {
        let items = vec![
            "  ".to_string(),
            "\t".to_string(),
            "abc".to_string(),
            " abc ".to_string(),
            "\nabc\n".to_string(),
        ];

        let unique_items = unique_items(items.into_iter());
        assert_eq!(unique_items, vec!["abc"]);
    }

    #[test]
    fn test_unique_items_with_str() {
        let items = vec!["abc", "  ", "def", "abc", "ghi", "ghi", "   def"];
        let unique_items = unique_items(items.into_iter());
        assert_eq!(unique_items, vec!["abc", "def", "ghi"]);
    }
}
