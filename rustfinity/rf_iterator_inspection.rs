//! # Rustfinity: Iterator Inspection
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/iterator-inspection
//!
//! When working with iterators, you often need to inspect or track the state of iteration without fundamentally changing the data. Rust provides several powerful methods for this: `enumerate()` adds indices to your elements, `peekable()` lets you look ahead without consuming, and `inspect()` allows side effects for debugging.
//!
//! ## Understanding Iterator Inspection Methods
//!
//! These methods are invaluable for debugging iterator pipelines, implementing parsers that need lookahead, or simply tracking positions within a sequence.
//!
//! ```rust
//! let words = vec!["hello", "world", "rust"];
//!
//! // enumerate() adds indices
//! for (index, word) in words.iter().enumerate() {
//!     println!("{}: {}", index, word);
//! }
//! // Output:
//! // 0: hello
//! // 1: world
//! // 2: rust
//!
//! // peekable() allows looking ahead
//! let mut iter = words.iter().peekable();
//! while let Some(word) = iter.next() {
//!     if let Some(&next_word) = iter.peek() {
//!         println!("{} is followed by {}", word, next_word);
//!     } else {
//!         println!("{} is the last word", word);
//!     }
//! }
//! ```
//!
//! ## Key Methods
//!
//! - **`enumerate()`** - Wraps each element with its index as `(index, element)` tuples
//! - **`peekable()`** - Creates an iterator that can look at the next element via `peek()` without consuming it
//! - **`inspect(closure)`** - Calls a closure on each element for side effects (e.g., debugging) without modifying the iterator
//!
//! ```rust
//! // inspect() for debugging pipelines
//! let sum: i32 = vec![1, 2, 3, 4, 5]
//!     .iter()
//!     .inspect(|x| {
//!         println!("Before filter: {}", x)
//!     })
//!     .filter(|x| *x % 2 == 0)
//!     .inspect(|x| {
//!         println!("After filter: {}", x)
//!     })
//!     .sum();
//! // Prints debugging info while computing sum = 6
//! ```
//!
//! ## Your Task
//!
//! Implement the following functions that use iterator inspection methods:
//!
//! 1. `indexed_elements<T: Clone>(
//!  items: &[T]
//! ) -> Vec<(usize, T)>`
//!    - Return elements with their indices using
//!      `enumerate()`
//! 2. `find_index<T: PartialEq>(
//!  items: &[T],
//!  target: &T
//! ) -> Option<usize>`
//!    - Find the index of the first occurrence of
//!      target
//! 3. `elements_with_next<T: Clone>(items: &[T]) -> Vec<(T, Option<T>)>`
//!    - Return each element paired with the next
//!      element (or None if last) using `peekable()`
//! 4. `group_consecutive_duplicates<T: Clone + PartialEq>(items: &[T]) -> Vec<(T, usize)>`
//!    - Group consecutive identical elements and
//!      count them using `peekable()`
//! 5. `find_first_repeated<T: Clone + PartialEq>(items: &[T]) -> Option<T>`
//!    - Find the first element that equals its next
//!      neighbor using `peekable()`
//! 6. `collect_with_trace<T: Clone + std::fmt::Debug>(items: &[T], trace: &mut Vec<String>) -> Vec<T>`
//!    - Collect elements while recording a trace of
//!      each element using `inspect()`
//! 7. `sum_with_running_total(
//!  numbers: &[i32],
//!  totals: &mut Vec<i32>
//! ) -> i32`
//!    - Sum numbers while recording running totals at
//!      each step using `inspect()`
//!
//! ## Examples
//!
//! ```rust
//! // indexed_elements
//! assert_eq!(
//!     indexed_elements(&["a", "b", "c"]),
//!     vec![(0, "a"), (1, "b"), (2, "c")]
//! );
//!
//! // find_index
//! assert_eq!(find_index(&[10, 20, 30, 40], &30), Some(2));
//! assert_eq!(find_index(&[10, 20, 30], &50), None);
//!
//! // elements_with_next
//! assert_eq!(
//!     elements_with_next(&[1, 2, 3]),
//!     vec![(1, Some(2)), (2, Some(3)), (3, None)]
//! );
//!
//! // group_consecutive_duplicates
//! assert_eq!(
//!     group_consecutive_duplicates(
//!         &[1, 1, 2, 3, 3, 3]
//!     ),
//!     vec![(1, 2), (2, 1), (3, 3)]
//! );
//!
//! // find_first_repeated
//! assert_eq!(find_first_repeated(&[1, 2, 2, 3]), Some(2));
//! assert_eq!(find_first_repeated(&[1, 2, 3]), None);
//!
//! // collect_with_trace
//! let mut trace = Vec::new();
//! let result = collect_with_trace(&[1, 2, 3], &mut trace);
//! assert_eq!(result, vec![1, 2, 3]);
//! assert_eq!(trace, vec!["1", "2", "3"]);
//!
//! // sum_with_running_total
//! let mut totals = Vec::new();
//! let sum = sum_with_running_total(
//!     &[10, 20, 30],
//!     &mut totals
//! );
//! assert_eq!(sum, 60);
//! assert_eq!(totals, vec![10, 30, 60]);
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - For `indexed_elements`, use
//!   `.iter().enumerate()` and clone the elements
//!   when collecting
//! - For `find_index`, use `enumerate()` with
//!   `find()` to locate the target, then extract
//!   the index
//! - For `elements_with_next`, use `peekable()`
//!   with `peek()` to look at the next element
//!   before calling `next()`
//! - For `group_consecutive_duplicates`, use
//!   `peekable()` to compare current with next and
//!   count runs
//! - For `find_first_repeated`, use `peekable()`
//!   to compare each element with its successor via
//!   `peek()`
//! - For `collect_with_trace`, use `inspect()` to
//!   push formatted elements into the trace vector
//! - For `sum_with_running_total`, use `inspect()`
//!   with a running total that gets updated and
//!   pushed
//! - Remember: `peek()` returns `Option<&T>`, so
//!   you may need to handle references carefully
//!
//! </details>
//!
//! Run: `cargo run --example rf_iterator_inspection`
//! Test: `cargo test --example rf_iterator_inspection`
//! Filter: `cargo test --example rf_iterator_inspection <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

/// Returns elements with their indices using enumerate().
///
/// # Arguments
/// * `items` - The slice to enumerate
///
/// # Returns
/// A Vec of (index, element) tuples
pub fn indexed_elements<T: Clone>(items: &[T]) -> Vec<(usize, T)> {
    // TODO: Implement this function
    unimplemented!()
}

/// Finds the index of the first occurrence of target.
///
/// # Arguments
/// * `items` - The slice to search
/// * `target` - The value to find
///
/// # Returns
/// Some(index) if found, None otherwise
pub fn find_index<T: PartialEq>(items: &[T], target: &T) -> Option<usize> {
    // TODO: Implement this function
    unimplemented!()
}

/// Returns each element paired with the next element (or None if last).
///
/// Uses peekable() to look ahead at the next element without consuming it.
///
/// # Arguments
/// * `items` - The slice to process
///
/// # Returns
/// A Vec of (element, Option<next_element>) tuples
pub fn elements_with_next<T: Clone>(items: &[T]) -> Vec<(T, Option<T>)> {
    // TODO: Implement this function
    unimplemented!()
}

/// Groups consecutive identical elements and counts them.
///
/// Uses peekable() to compare current element with next to detect group boundaries.
///
/// # Arguments
/// * `items` - The slice to group
///
/// # Returns
/// A Vec of (element, count) tuples for each consecutive group
pub fn group_consecutive_duplicates<T: Clone + PartialEq>(items: &[T]) -> Vec<(T, usize)> {
    // TODO: Implement this function
    unimplemented!()
}

/// Finds the first element that equals its next neighbor.
///
/// Uses peekable() to compare each element with its successor.
///
/// # Arguments
/// * `items` - The slice to search
///
/// # Returns
/// Some(element) for the first repeated element, None if no consecutive duplicates
pub fn find_first_repeated<T: Clone + PartialEq>(items: &[T]) -> Option<T> {
    // TODO: Implement this function
    unimplemented!()
}

/// Collects elements while recording a trace of each element.
///
/// Uses inspect() to push a Debug-formatted string for each element into the trace.
///
/// # Arguments
/// * `items` - The slice to collect
/// * `trace` - A mutable Vec to record the trace
///
/// # Returns
/// A Vec containing all elements
pub fn collect_with_trace<T: Clone + std::fmt::Debug>(
    items: &[T],
    trace: &mut Vec<String>,
) -> Vec<T> {
    // TODO: Implement this function
    unimplemented!()
}

/// Sums numbers while recording running totals at each step.
///
/// Uses inspect() to track the running total after each addition.
///
/// # Arguments
/// * `numbers` - The slice of numbers to sum
/// * `totals` - A mutable Vec to record running totals
///
/// # Returns
/// The final sum
pub fn sum_with_running_total(numbers: &[i32], totals: &mut Vec<i32>) -> i32 {
    // TODO: Implement this function
    unimplemented!()
}

pub fn main() {
    // indexed_elements example
    let indexed = indexed_elements(&["a", "b", "c"]);
    println!("indexed_elements([\"a\", \"b\", \"c\"]) = {:?}", indexed);

    // find_index example
    let index = find_index(&[10, 20, 30, 40], &30);
    println!("find_index([10, 20, 30, 40], &30) = {:?}", index);

    // elements_with_next example
    let with_next = elements_with_next(&[1, 2, 3]);
    println!("elements_with_next([1, 2, 3]) = {:?}", with_next);

    // group_consecutive_duplicates example
    let grouped = group_consecutive_duplicates(&[1, 1, 2, 3, 3, 3]);
    println!(
        "group_consecutive_duplicates([1, 1, 2, 3, 3, 3]) = {:?}",
        grouped
    );

    // find_first_repeated example
    let repeated = find_first_repeated(&[1, 2, 2, 3]);
    println!("find_first_repeated([1, 2, 2, 3]) = {:?}", repeated);

    // collect_with_trace example
    let mut trace = Vec::new();
    let collected = collect_with_trace(&[1, 2, 3], &mut trace);
    println!("collect_with_trace([1, 2, 3]) = {:?}", collected);
    println!("trace = {:?}", trace);

    // sum_with_running_total example
    let mut totals = Vec::new();
    let sum = sum_with_running_total(&[10, 20, 30], &mut totals);
    println!("sum_with_running_total([10, 20, 30]) = {}", sum);
    println!("totals = {:?}", totals);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    // ==================== indexed_elements tests ====================

    #[test]
    fn test_indexed_elements_basic() {
        assert_eq!(
            indexed_elements(&["a", "b", "c"]),
            vec![(0, "a"), (1, "b"), (2, "c")]
        );
    }

    #[test]
    fn test_indexed_elements_numbers() {
        assert_eq!(
            indexed_elements(&[10, 20, 30]),
            vec![(0, 10), (1, 20), (2, 30)]
        );
    }

    #[test]
    fn test_indexed_elements_empty() {
        let empty: &[i32] = &[];
        assert_eq!(indexed_elements(empty), Vec::<(usize, i32)>::new());
    }

    #[test]
    fn test_indexed_elements_single() {
        assert_eq!(indexed_elements(&[42]), vec![(0, 42)]);
    }

    #[test]
    fn test_indexed_elements_strings() {
        assert_eq!(
            indexed_elements(&["hello".to_string(), "world".to_string()]),
            vec![(0, "hello".to_string()), (1, "world".to_string())]
        );
    }

    // ==================== find_index tests ====================

    #[test]
    fn test_find_index_found() {
        assert_eq!(find_index(&[10, 20, 30, 40], &30), Some(2));
    }

    #[test]
    fn test_find_index_not_found() {
        assert_eq!(find_index(&[10, 20, 30], &50), None);
    }

    #[test]
    fn test_find_index_first_element() {
        assert_eq!(find_index(&[10, 20, 30], &10), Some(0));
    }

    #[test]
    fn test_find_index_last_element() {
        assert_eq!(find_index(&[10, 20, 30], &30), Some(2));
    }

    #[test]
    fn test_find_index_duplicate_returns_first() {
        assert_eq!(find_index(&[10, 20, 20, 30], &20), Some(1));
    }

    #[test]
    fn test_find_index_empty() {
        let empty: &[i32] = &[];
        assert_eq!(find_index(empty, &10), None);
    }

    #[test]
    fn test_find_index_strings() {
        assert_eq!(find_index(&["a", "b", "c"], &"b"), Some(1));
    }

    #[test]
    fn test_find_index_single_element_found() {
        assert_eq!(find_index(&[42], &42), Some(0));
    }

    #[test]
    fn test_find_index_single_element_not_found() {
        assert_eq!(find_index(&[42], &0), None);
    }

    // ==================== elements_with_next tests ====================

    #[test]
    fn test_elements_with_next_basic() {
        assert_eq!(
            elements_with_next(&[1, 2, 3]),
            vec![(1, Some(2)), (2, Some(3)), (3, None)]
        );
    }

    #[test]
    fn test_elements_with_next_single() {
        assert_eq!(elements_with_next(&[42]), vec![(42, None)]);
    }

    #[test]
    fn test_elements_with_next_empty() {
        let empty: &[i32] = &[];
        assert_eq!(elements_with_next(empty), Vec::<(i32, Option<i32>)>::new());
    }

    #[test]
    fn test_elements_with_next_two_elements() {
        assert_eq!(elements_with_next(&[1, 2]), vec![(1, Some(2)), (2, None)]);
    }

    #[test]
    fn test_elements_with_next_strings() {
        assert_eq!(
            elements_with_next(&["a", "b", "c"]),
            vec![("a", Some("b")), ("b", Some("c")), ("c", None)]
        );
    }

    #[test]
    fn test_elements_with_next_duplicates() {
        assert_eq!(
            elements_with_next(&[1, 1, 1]),
            vec![(1, Some(1)), (1, Some(1)), (1, None)]
        );
    }

    // ==================== group_consecutive_duplicates tests ====================

    #[test]
    fn test_group_consecutive_duplicates_basic() {
        assert_eq!(
            group_consecutive_duplicates(&[1, 1, 2, 3, 3, 3]),
            vec![(1, 2), (2, 1), (3, 3)]
        );
    }

    #[test]
    fn test_group_consecutive_duplicates_no_duplicates() {
        assert_eq!(
            group_consecutive_duplicates(&[1, 2, 3, 4]),
            vec![(1, 1), (2, 1), (3, 1), (4, 1)]
        );
    }

    #[test]
    fn test_group_consecutive_duplicates_all_same() {
        assert_eq!(group_consecutive_duplicates(&[5, 5, 5, 5]), vec![(5, 4)]);
    }

    #[test]
    fn test_group_consecutive_duplicates_empty() {
        let empty: &[i32] = &[];
        assert_eq!(
            group_consecutive_duplicates(empty),
            Vec::<(i32, usize)>::new()
        );
    }

    #[test]
    fn test_group_consecutive_duplicates_single() {
        assert_eq!(group_consecutive_duplicates(&[42]), vec![(42, 1)]);
    }

    #[test]
    fn test_group_consecutive_duplicates_alternating() {
        assert_eq!(
            group_consecutive_duplicates(&[1, 2, 1, 2, 1]),
            vec![(1, 1), (2, 1), (1, 1), (2, 1), (1, 1)]
        );
    }

    #[test]
    fn test_group_consecutive_duplicates_strings() {
        assert_eq!(
            group_consecutive_duplicates(&["a", "a", "b", "b", "b"]),
            vec![("a", 2), ("b", 3)]
        );
    }

    #[test]
    fn test_group_consecutive_duplicates_long_run() {
        assert_eq!(
            group_consecutive_duplicates(&[1, 1, 1, 1, 1, 2, 3, 3]),
            vec![(1, 5), (2, 1), (3, 2)]
        );
    }

    // ==================== find_first_repeated tests ====================

    #[test]
    fn test_find_first_repeated_basic() {
        assert_eq!(find_first_repeated(&[1, 2, 2, 3]), Some(2));
    }

    #[test]
    fn test_find_first_repeated_none() {
        assert_eq!(find_first_repeated(&[1, 2, 3, 4]), None);
    }

    #[test]
    fn test_find_first_repeated_at_start() {
        assert_eq!(find_first_repeated(&[1, 1, 2, 3]), Some(1));
    }

    #[test]
    fn test_find_first_repeated_at_end() {
        assert_eq!(find_first_repeated(&[1, 2, 3, 3]), Some(3));
    }

    #[test]
    fn test_find_first_repeated_multiple_duplicates() {
        // Returns first occurrence of consecutive duplicates
        assert_eq!(find_first_repeated(&[1, 1, 2, 2, 3, 3]), Some(1));
    }

    #[test]
    fn test_find_first_repeated_empty() {
        let empty: &[i32] = &[];
        assert_eq!(find_first_repeated(empty), None);
    }

    #[test]
    fn test_find_first_repeated_single() {
        assert_eq!(find_first_repeated(&[42]), None);
    }

    #[test]
    fn test_find_first_repeated_two_same() {
        assert_eq!(find_first_repeated(&[5, 5]), Some(5));
    }

    #[test]
    fn test_find_first_repeated_two_different() {
        assert_eq!(find_first_repeated(&[1, 2]), None);
    }

    #[test]
    fn test_find_first_repeated_strings() {
        assert_eq!(find_first_repeated(&["a", "b", "b", "c"]), Some("b"));
    }

    // ==================== collect_with_trace tests ====================

    #[test]
    fn test_collect_with_trace_basic() {
        let mut trace = Vec::new();
        let result = collect_with_trace(&[1, 2, 3], &mut trace);
        assert_eq!(result, vec![1, 2, 3]);
        assert_eq!(trace, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_collect_with_trace_empty() {
        let mut trace = Vec::new();
        let empty: &[i32] = &[];
        let result = collect_with_trace(empty, &mut trace);
        assert!(result.is_empty());
        assert!(trace.is_empty());
    }

    #[test]
    fn test_collect_with_trace_strings() {
        let mut trace = Vec::new();
        let result = collect_with_trace(&["a", "b"], &mut trace);
        assert_eq!(result, vec!["a", "b"]);
        assert_eq!(trace, vec!["\"a\"", "\"b\""]);
    }

    #[test]
    fn test_collect_with_trace_single() {
        let mut trace = Vec::new();
        let result = collect_with_trace(&[42], &mut trace);
        assert_eq!(result, vec![42]);
        assert_eq!(trace, vec!["42"]);
    }

    #[test]
    fn test_collect_with_trace_preserves_existing() {
        let mut trace = vec!["existing".to_string()];
        let result = collect_with_trace(&[1, 2], &mut trace);
        assert_eq!(result, vec![1, 2]);
        assert_eq!(trace, vec!["existing", "1", "2"]);
    }

    // ==================== sum_with_running_total tests ====================

    #[test]
    fn test_sum_with_running_total_basic() {
        let mut totals = Vec::new();
        let sum = sum_with_running_total(&[10, 20, 30], &mut totals);
        assert_eq!(sum, 60);
        assert_eq!(totals, vec![10, 30, 60]);
    }

    #[test]
    fn test_sum_with_running_total_empty() {
        let mut totals = Vec::new();
        let sum = sum_with_running_total(&[], &mut totals);
        assert_eq!(sum, 0);
        assert!(totals.is_empty());
    }

    #[test]
    fn test_sum_with_running_total_single() {
        let mut totals = Vec::new();
        let sum = sum_with_running_total(&[100], &mut totals);
        assert_eq!(sum, 100);
        assert_eq!(totals, vec![100]);
    }

    #[test]
    fn test_sum_with_running_total_with_negatives() {
        let mut totals = Vec::new();
        let sum = sum_with_running_total(&[10, -5, 20, -10], &mut totals);
        assert_eq!(sum, 15);
        assert_eq!(totals, vec![10, 5, 25, 15]);
    }

    #[test]
    fn test_sum_with_running_total_zeros() {
        let mut totals = Vec::new();
        let sum = sum_with_running_total(&[0, 0, 5, 0], &mut totals);
        assert_eq!(sum, 5);
        assert_eq!(totals, vec![0, 0, 5, 5]);
    }

    #[test]
    fn test_sum_with_running_total_preserves_existing() {
        let mut totals = vec![999];
        let sum = sum_with_running_total(&[1, 2, 3], &mut totals);
        assert_eq!(sum, 6);
        assert_eq!(totals, vec![999, 1, 3, 6]);
    }

    // ==================== Integration tests ====================

    #[test]
    fn test_indexed_find_pattern() {
        // Find all indices of a specific value
        let items = [1, 2, 3, 2, 4, 2];
        let indices: Vec<usize> = indexed_elements(&items)
            .into_iter()
            .filter(|(_, v)| *v == 2)
            .map(|(i, _)| i)
            .collect();
        assert_eq!(indices, vec![1, 3, 5]);
    }

    #[test]
    fn test_elements_with_next_for_differences() {
        // Calculate differences between consecutive elements
        let numbers = [10, 15, 12, 20];
        let differences: Vec<i32> = elements_with_next(&numbers)
            .into_iter()
            .filter_map(|(curr, next)| next.map(|n| n - curr))
            .collect();
        assert_eq!(differences, vec![5, -3, 8]);
    }

    #[test]
    fn test_group_and_find_longest_run() {
        // Find the longest consecutive run
        let data = [1, 2, 2, 3, 3, 3, 4, 4];
        let groups = group_consecutive_duplicates(&data);
        let longest = groups.iter().max_by_key(|(_, count)| count);
        assert_eq!(longest, Some(&(3, 3)));
    }

    #[test]
    fn test_trace_pipeline() {
        // Trace through a processing pipeline
        let mut trace1 = Vec::new();
        let mut trace2 = Vec::new();

        let data = [1, 2, 3, 4, 5];
        let step1 = collect_with_trace(&data, &mut trace1);
        let evens: Vec<i32> = step1.into_iter().filter(|x| x % 2 == 0).collect();
        let _ = collect_with_trace(&evens, &mut trace2);

        assert_eq!(trace1, vec!["1", "2", "3", "4", "5"]);
        assert_eq!(trace2, vec!["2", "4"]);
    }

    #[test]
    fn test_combined_inspection() {
        // Use multiple inspection techniques together
        let data = [1, 1, 2, 2, 2, 3];

        // Group consecutive duplicates
        let groups = group_consecutive_duplicates(&data);
        assert_eq!(groups, vec![(1, 2), (2, 3), (3, 1)]);

        // Find first repeated
        let first_repeated = find_first_repeated(&data);
        assert_eq!(first_repeated, Some(1));

        // Get indexed elements
        let indexed = indexed_elements(&data);
        assert_eq!(indexed.len(), 6);
        assert_eq!(indexed[0], (0, 1));
        assert_eq!(indexed[5], (5, 3));
    }

    #[test]
    fn test_running_total_with_trace() {
        // Combine running totals with trace
        let numbers = [5, 10, 15];
        let mut totals = Vec::new();
        let mut trace = Vec::new();

        let collected = collect_with_trace(&numbers, &mut trace);
        let sum = sum_with_running_total(&collected, &mut totals);

        assert_eq!(trace, vec!["5", "10", "15"]);
        assert_eq!(totals, vec![5, 15, 30]);
        assert_eq!(sum, 30);
    }
}
