//! # Rustfinity: Hashset Operations
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/hashset-operations
//!
//! `HashSet<T>` is Rust's implementation of a hash set - a collection of unique values with O(1) average-time complexity for insertions, lookups, and removals. Unlike `Vec`, a `HashSet` automatically ensures that no duplicate values exist, making it perfect for deduplication and membership testing.
//!
//! The `HashSet` type is found in `std::collections` and requires that elements implement both `Hash` and `Eq` traits. Most primitive types and many standard library types already implement these traits, so you can use them directly.
//!
//! Beyond basic operations like `insert()` and `contains()`, `HashSet` provides powerful set-theoretic operations: `union()` (elements in either set), `intersection()` (elements in both sets), `difference()` (elements in one set but not the other), and `symmetric_difference()` (elements in exactly one set).
//!
//! ```rust
//! use std::collections::HashSet;
//!
//! let mut set: HashSet<i32> = HashSet::new();
//! set.insert(1);
//! set.insert(2);
//! set.insert(2);  // Duplicate, won't be added
//! assert_eq!(set.len(), 2);
//! assert!(set.contains(&1));
//!
//! // Creating from an iterator
//! let set: HashSet<_> = vec![1, 2, 3, 2, 1]
//!     .into_iter()
//!     .collect();
//! assert_eq!(set.len(), 3);  // Only unique values
//! ```
//!
//! ## Your Task
//!
//! Implement the following functions for working with `HashSet`:
//!
//! 1. `unique_elements(items: &[i32]) -> HashSet<i32>`
//!    - Return a set containing only the unique elements from
//!      the slice
//!
//! 2. `count_unique(items: &[i32]) -> usize`
//!    - Return the count of unique elements in the slice
//!
//! 3. `find_common(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32>` - Return elements that appear in both sets (intersection)
//!
//! 4. `find_all(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32>` - Return elements that appear in either set (union)
//!
//! 5. `find_difference(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32>` - Return elements in `set1` that are not in `set2` (difference)
//!
//! 6. `find_symmetric_difference(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32>` - Return elements that are in exactly one of the sets (symmetric difference)
//!
//! 7. `is_subset(potential_subset: &HashSet<i32>,
//! potential_superset: &HashSet<i32>) -> bool`
//!    - Check if all elements of `potential_subset` are
//!      contained in `potential_superset`
//!
//! ## Examples
//!
//! ```rust
//! use std::collections::HashSet;
//!
//! // unique_elements
//! let items = vec![1, 2, 3, 2, 1, 4, 3];
//! let unique = unique_elements(&items);
//! assert!(unique.contains(&1));
//! assert!(unique.contains(&4));
//! assert_eq!(unique.len(), 4);
//!
//! // count_unique
//! assert_eq!(count_unique(&[1, 2, 2, 3, 3, 3]), 3);
//!
//! // find_common (intersection)
//! let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
//! let set2: HashSet<i32> = [2, 3, 4].into_iter().collect();
//! let common = find_common(&set1, &set2);
//! assert_eq!(
//!     common,
//!     [2, 3].into_iter().collect()
//! );
//!
//! // find_all (union)
//! let all = find_all(&set1, &set2);
//! assert_eq!(
//!     all,
//!     [1, 2, 3, 4].into_iter().collect()
//! );
//!
//! // find_difference
//! // In set1 but not set2
//! let diff = find_difference(&set1, &set2);
//! assert_eq!(
//!     diff,
//!     [1].into_iter().collect()
//! );
//!
//! // find_symmetric_difference
//! // In exactly one set
//! let sym_diff = find_symmetric_difference(&set1, &set2);
//! assert_eq!(
//!     sym_diff,
//!     [1, 4].into_iter().collect()
//! );
//!
//! // is_subset
//! let small: HashSet<i32> = [2, 3]
//!     .into_iter()
//!     .collect();
//! let large: HashSet<i32> = [1, 2, 3, 4]
//!     .into_iter()
//!     .collect();
//! assert!(is_subset(&small, &large));
//! assert!(!is_subset(&large, &small));
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - For `unique_elements`, you can iterate over the slice and collect into a `HashSet`, or use `.iter().cloned().collect()`
//! - For `count_unique`, you can create a set and return its length
//! - For `find_common`, use the `.intersection()` method, which returns an iterator
//! - For `find_all`, use the `.union()` method
//! - For `find_difference`, use the `.difference()` method
//! - For `find_symmetric_difference`, use the `.symmetric_difference()` method
//! - All set operations return iterators of references, so you'll need to clone or copy the values when collecting
//! - For `is_subset`, use the `.is_subset()` method
//!
//! </details>
//!
//! Run: `cargo run --example rf_hashset_operations`
//! Test: `cargo test --example rf_hashset_operations`
//! Filter: `cargo test --example rf_hashset_operations <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::collections::HashSet;

/// Returns a HashSet containing only the unique elements from the input slice.
pub fn unique_elements(items: &[i32]) -> HashSet<i32> {
    // TODO: Collect items into a HashSet
    unimplemented!()
}

/// Returns the count of unique elements in the input slice.
pub fn count_unique(items: &[i32]) -> usize {
    // TODO: Count unique elements
    unimplemented!()
}

/// Returns elements that appear in both sets (intersection).
pub fn find_common(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    // TODO: Find intersection
    unimplemented!()
}

/// Returns elements that appear in either set (union).
pub fn find_all(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    // TODO: Find union
    unimplemented!()
}

/// Returns elements in set1 that are not in set2 (difference).
pub fn find_difference(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    // TODO: Find difference
    unimplemented!()
}

/// Returns elements that are in exactly one of the sets (symmetric difference).
pub fn find_symmetric_difference(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    // TODO: Find symmetric difference
    unimplemented!()
}

/// Checks if all elements of potential_subset are contained in potential_superset.
pub fn is_subset(potential_subset: &HashSet<i32>, potential_superset: &HashSet<i32>) -> bool {
    // TODO: Check subset relationship
    unimplemented!()
}

pub fn main() {
    // Example: unique_elements
    let items = vec![1, 2, 3, 2, 1, 4, 3];
    let unique = unique_elements(&items);
    println!("Unique elements: {:?}", unique);

    // Example: count_unique
    let count = count_unique(&[1, 2, 2, 3, 3, 3]);
    println!("Count of unique elements: {}", count);

    // Example: set operations
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = [2, 3, 4].into_iter().collect();

    println!("Set 1: {:?}", set1);
    println!("Set 2: {:?}", set2);
    println!("Intersection: {:?}", find_common(&set1, &set2));
    println!("Union: {:?}", find_all(&set1, &set2));
    println!(
        "Difference (set1 - set2): {:?}",
        find_difference(&set1, &set2)
    );
    println!(
        "Symmetric difference: {:?}",
        find_symmetric_difference(&set1, &set2)
    );

    // Example: is_subset
    let small: HashSet<i32> = [2, 3].into_iter().collect();
    let large: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
    println!(
        "Is {:?} subset of {:?}? {}",
        small,
        large,
        is_subset(&small, &large)
    );
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::collections::HashSet;

    // ==================== unique_elements tests ====================

    #[test]
    fn test_unique_elements_with_duplicates() {
        let items = vec![1, 2, 3, 2, 1, 4, 3];
        let unique = unique_elements(&items);
        assert_eq!(unique.len(), 4);
        assert!(unique.contains(&1));
        assert!(unique.contains(&2));
        assert!(unique.contains(&3));
        assert!(unique.contains(&4));
    }

    #[test]
    fn test_unique_elements_all_unique() {
        let items = vec![1, 2, 3, 4, 5];
        let unique = unique_elements(&items);
        assert_eq!(unique.len(), 5);
    }

    #[test]
    fn test_unique_elements_all_same() {
        let items = vec![7, 7, 7, 7, 7];
        let unique = unique_elements(&items);
        assert_eq!(unique.len(), 1);
        assert!(unique.contains(&7));
    }

    #[test]
    fn test_unique_elements_empty() {
        let items: Vec<i32> = vec![];
        let unique = unique_elements(&items);
        assert!(unique.is_empty());
    }

    #[test]
    fn test_unique_elements_single() {
        let items = vec![42];
        let unique = unique_elements(&items);
        assert_eq!(unique.len(), 1);
        assert!(unique.contains(&42));
    }

    #[test]
    fn test_unique_elements_negative_numbers() {
        let items = vec![-1, -2, -1, 0, -2, 1];
        let unique = unique_elements(&items);
        assert_eq!(unique.len(), 4);
        assert!(unique.contains(&-1));
        assert!(unique.contains(&-2));
        assert!(unique.contains(&0));
        assert!(unique.contains(&1));
    }

    // ==================== count_unique tests ====================

    #[test]
    fn test_count_unique_with_duplicates() {
        assert_eq!(count_unique(&[1, 2, 2, 3, 3, 3]), 3);
    }

    #[test]
    fn test_count_unique_all_unique() {
        assert_eq!(count_unique(&[1, 2, 3, 4, 5]), 5);
    }

    #[test]
    fn test_count_unique_all_same() {
        assert_eq!(count_unique(&[1, 1, 1, 1]), 1);
    }

    #[test]
    fn test_count_unique_empty() {
        assert_eq!(count_unique(&[]), 0);
    }

    #[test]
    fn test_count_unique_single() {
        assert_eq!(count_unique(&[42]), 1);
    }

    // ==================== find_common tests ====================

    #[test]
    fn test_find_common_overlapping() {
        let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let set2: HashSet<i32> = [2, 3, 4].into_iter().collect();
        let common = find_common(&set1, &set2);
        let expected: HashSet<i32> = [2, 3].into_iter().collect();
        assert_eq!(common, expected);
    }

    #[test]
    fn test_find_common_disjoint() {
        let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let set2: HashSet<i32> = [4, 5, 6].into_iter().collect();
        let common = find_common(&set1, &set2);
        assert!(common.is_empty());
    }

    #[test]
    fn test_find_common_identical() {
        let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let set2: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let common = find_common(&set1, &set2);
        assert_eq!(common, set1);
    }

    #[test]
    fn test_find_common_one_empty() {
        let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let set2: HashSet<i32> = HashSet::new();
        let common = find_common(&set1, &set2);
        assert!(common.is_empty());
    }

    #[test]
    fn test_find_common_subset() {
        let set1: HashSet<i32> = [1, 2, 3, 4, 5].into_iter().collect();
        let set2: HashSet<i32> = [2, 3].into_iter().collect();
        let common = find_common(&set1, &set2);
        let expected: HashSet<i32> = [2, 3].into_iter().collect();
        assert_eq!(common, expected);
    }

    // ==================== find_all tests ====================

    #[test]
    fn test_find_all_overlapping() {
        let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let set2: HashSet<i32> = [2, 3, 4].into_iter().collect();
        let all = find_all(&set1, &set2);
        let expected: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
        assert_eq!(all, expected);
    }

    #[test]
    fn test_find_all_disjoint() {
        let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let set2: HashSet<i32> = [4, 5, 6].into_iter().collect();
        let all = find_all(&set1, &set2);
        let expected: HashSet<i32> = [1, 2, 3, 4, 5, 6].into_iter().collect();
        assert_eq!(all, expected);
    }

    #[test]
    fn test_find_all_identical() {
        let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let set2: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let all = find_all(&set1, &set2);
        assert_eq!(all, set1);
    }

    #[test]
    fn test_find_all_one_empty() {
        let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let set2: HashSet<i32> = HashSet::new();
        let all = find_all(&set1, &set2);
        assert_eq!(all, set1);
    }

    #[test]
    fn test_find_all_both_empty() {
        let set1: HashSet<i32> = HashSet::new();
        let set2: HashSet<i32> = HashSet::new();
        let all = find_all(&set1, &set2);
        assert!(all.is_empty());
    }

    // ==================== find_difference tests ====================

    #[test]
    fn test_find_difference_overlapping() {
        let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let set2: HashSet<i32> = [2, 3, 4].into_iter().collect();
        let diff = find_difference(&set1, &set2);
        let expected: HashSet<i32> = [1].into_iter().collect();
        assert_eq!(diff, expected);
    }

    #[test]
    fn test_find_difference_disjoint() {
        let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let set2: HashSet<i32> = [4, 5, 6].into_iter().collect();
        let diff = find_difference(&set1, &set2);
        assert_eq!(diff, set1);
    }

    #[test]
    fn test_find_difference_identical() {
        let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let set2: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let diff = find_difference(&set1, &set2);
        assert!(diff.is_empty());
    }

    #[test]
    fn test_find_difference_first_empty() {
        let set1: HashSet<i32> = HashSet::new();
        let set2: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let diff = find_difference(&set1, &set2);
        assert!(diff.is_empty());
    }

    #[test]
    fn test_find_difference_second_empty() {
        let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let set2: HashSet<i32> = HashSet::new();
        let diff = find_difference(&set1, &set2);
        assert_eq!(diff, set1);
    }

    #[test]
    fn test_find_difference_subset() {
        let set1: HashSet<i32> = [1, 2, 3, 4, 5].into_iter().collect();
        let set2: HashSet<i32> = [2, 3].into_iter().collect();
        let diff = find_difference(&set1, &set2);
        let expected: HashSet<i32> = [1, 4, 5].into_iter().collect();
        assert_eq!(diff, expected);
    }

    // ==================== find_symmetric_difference tests ====================

    #[test]
    fn test_find_symmetric_difference_overlapping() {
        let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let set2: HashSet<i32> = [2, 3, 4].into_iter().collect();
        let sym_diff = find_symmetric_difference(&set1, &set2);
        let expected: HashSet<i32> = [1, 4].into_iter().collect();
        assert_eq!(sym_diff, expected);
    }

    #[test]
    fn test_find_symmetric_difference_disjoint() {
        let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let set2: HashSet<i32> = [4, 5, 6].into_iter().collect();
        let sym_diff = find_symmetric_difference(&set1, &set2);
        let expected: HashSet<i32> = [1, 2, 3, 4, 5, 6].into_iter().collect();
        assert_eq!(sym_diff, expected);
    }

    #[test]
    fn test_find_symmetric_difference_identical() {
        let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let set2: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let sym_diff = find_symmetric_difference(&set1, &set2);
        assert!(sym_diff.is_empty());
    }

    #[test]
    fn test_find_symmetric_difference_one_empty() {
        let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let set2: HashSet<i32> = HashSet::new();
        let sym_diff = find_symmetric_difference(&set1, &set2);
        assert_eq!(sym_diff, set1);
    }

    #[test]
    fn test_find_symmetric_difference_both_empty() {
        let set1: HashSet<i32> = HashSet::new();
        let set2: HashSet<i32> = HashSet::new();
        let sym_diff = find_symmetric_difference(&set1, &set2);
        assert!(sym_diff.is_empty());
    }

    // ==================== is_subset tests ====================

    #[test]
    fn test_is_subset_true() {
        let small: HashSet<i32> = [2, 3].into_iter().collect();
        let large: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
        assert!(is_subset(&small, &large));
    }

    #[test]
    fn test_is_subset_false() {
        let small: HashSet<i32> = [2, 3, 5].into_iter().collect();
        let large: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
        assert!(!is_subset(&small, &large));
    }

    #[test]
    fn test_is_subset_equal_sets() {
        let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let set2: HashSet<i32> = [1, 2, 3].into_iter().collect();
        assert!(is_subset(&set1, &set2));
    }

    #[test]
    fn test_is_subset_empty_is_subset_of_any() {
        let empty: HashSet<i32> = HashSet::new();
        let non_empty: HashSet<i32> = [1, 2, 3].into_iter().collect();
        assert!(is_subset(&empty, &non_empty));
    }

    #[test]
    fn test_is_subset_empty_of_empty() {
        let empty1: HashSet<i32> = HashSet::new();
        let empty2: HashSet<i32> = HashSet::new();
        assert!(is_subset(&empty1, &empty2));
    }

    #[test]
    fn test_is_subset_reverse_not_subset() {
        let small: HashSet<i32> = [2, 3].into_iter().collect();
        let large: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
        assert!(!is_subset(&large, &small));
    }

    #[test]
    fn test_is_subset_disjoint() {
        let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let set2: HashSet<i32> = [4, 5, 6].into_iter().collect();
        assert!(!is_subset(&set1, &set2));
    }

    // ==================== Additional edge case tests ====================

    #[test]
    fn test_operations_with_large_numbers() {
        let items = vec![i32::MAX, i32::MIN, 0, i32::MAX, i32::MIN];
        let unique = unique_elements(&items);
        assert_eq!(unique.len(), 3);
        assert!(unique.contains(&i32::MAX));
        assert!(unique.contains(&i32::MIN));
        assert!(unique.contains(&0));
    }

    #[test]
    fn test_symmetric_difference_commutative() {
        let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let set2: HashSet<i32> = [2, 3, 4].into_iter().collect();
        let sym_diff1 = find_symmetric_difference(&set1, &set2);
        let sym_diff2 = find_symmetric_difference(&set2, &set1);
        assert_eq!(sym_diff1, sym_diff2);
    }

    #[test]
    fn test_union_intersection_relationship() {
        // |A ∪ B| = |A| + |B| - |A ∩ B|
        let set1: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
        let set2: HashSet<i32> = [3, 4, 5, 6].into_iter().collect();
        let union = find_all(&set1, &set2);
        let intersection = find_common(&set1, &set2);
        assert_eq!(union.len(), set1.len() + set2.len() - intersection.len());
    }

    #[test]
    fn test_difference_not_commutative() {
        let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let set2: HashSet<i32> = [2, 3, 4].into_iter().collect();
        let diff1 = find_difference(&set1, &set2);
        let diff2 = find_difference(&set2, &set1);
        assert_ne!(diff1, diff2);
        let expected1: HashSet<i32> = [1].into_iter().collect();
        let expected2: HashSet<i32> = [4].into_iter().collect();
        assert_eq!(diff1, expected1);
        assert_eq!(diff2, expected2);
    }
}
