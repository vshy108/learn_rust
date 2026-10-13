//! # Rustfinity: Iterator Filtering
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/iterator-filtering
//!
//! Rust's iterator system provides powerful filtering methods that allow you to selectively process elements from a sequence. These methods are essential for data processing, validation, and transformation pipelines.
//!
//! ## Core Filtering Methods
//!
//! ### filter()
//!
//! The `filter()` method retains only elements that satisfy a predicate:
//!
//! ```rust
//! let numbers = vec![1, 2, 3, 4, 5, 6];
//!
//! // Keep only even numbers
//! let evens: Vec<i32> = numbers
//!     .iter()
//!     .filter(|&x| x % 2 == 0)
//!     .cloned()
//!     .collect();
//! assert_eq!(evens, vec![2, 4, 6]);
//!
//! // Keep only numbers greater than 3
//! let large: Vec<i32> = numbers
//!     .iter()
//!     .filter(|&x| *x > 3)
//!     .cloned()
//!     .collect();
//! assert_eq!(large, vec![4, 5, 6]);
//! ```
//!
//! ### filter_map()
//!
//! The `filter_map()` method combines filtering and mapping in one step. It applies a function that returns `Option<T>` and keeps only the `Some` values:
//!
//! ```rust
//! let strings = vec!["1", "two", "3", "four", "5"];
//!
//! // Parse to integers, keeping only successful parses
//! let numbers: Vec<i32> = strings
//!     .iter()
//!     .filter_map(|s| s.parse::<i32>().ok())
//!     .collect();
//! assert_eq!(numbers, vec![1, 3, 5]);
//! ```
//!
//! ### take_while() and skip_while()
//!
//! These methods process elements based on a condition at the beginning of the sequence:
//!
//! ```rust
//! let numbers = vec![1, 2, 3, 4, 5, 1, 2];
//!
//! // Take elements while they're less than 4
//! // (stops at first failure)
//! let taken: Vec<i32> = numbers
//!     .iter()
//!     .take_while(|&&x| x < 4)
//!     .cloned()
//!     .collect();
//! assert_eq!(taken, vec![1, 2, 3]);
//!
//! // Skip elements while they're less than 4
//! // (takes the rest after first failure)
//! let skipped: Vec<i32> = numbers
//!     .iter()
//!     .skip_while(|&&x| x < 4)
//!     .cloned()
//!     .collect();
//! assert_eq!(skipped, vec![4, 5, 1, 2]);
//! ```
//!
//! Note that `take_while` and `skip_while` only check the condition at the beginning - once the condition fails (for `take_while`) or succeeds (for `skip_while`), the behavior changes for all remaining elements.
//!
//! ## Your Task
//!
//! Implement the following functions that demonstrate iterator filtering patterns:
//!
//! 1. `filter_even(numbers: &[i32]) -> Vec<i32>` - Return only even numbers
//! 2. `filter_by_predicate<F>(numbers: &[i32], predicate: F) -> Vec<i32>`
//!    - Return numbers matching a custom predicate
//!      (where `F: Fn(&i32) -> bool`)
//! 3. `parse_valid_numbers(strings: &[&str]) -> Vec<i32>`
//!    - Parse strings to integers, keeping only valid parses
//! 4. `filter_map_with<T, U, F>(items: &[T], f: F) -> Vec<U>`
//!    - Generic filter_map (where `T: Clone`,
//!      `F: Fn(T) -> Option<U>`)
//! 5. `take_while_positive(numbers: &[i32]) -> Vec<i32>`
//!    - Take numbers while they're positive (> 0)
//! 6. `skip_while_negative(numbers: &[i32]) -> Vec<i32>`
//!    - Skip numbers while they're negative (< 0)
//! 7. `filter_in_range(numbers: &[i32], min: i32, max: i32) -> Vec<i32>` - Keep numbers within range [min, max] inclusive
//! 8. `first_matching<T: Clone, F>(items: &[T], predicate: F) -> Option<T>`
//!    - Find first element matching predicate
//!      (where `F: Fn(&T) -> bool`)
//!
//! ## Examples
//!
//! ```rust
//! // filter_even
//! assert_eq!(filter_even(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
//!
//! // filter_by_predicate
//! assert_eq!(
//!     filter_by_predicate(&[1, 2, 3, 4, 5], |&x| x > 3),
//!     vec![4, 5]
//! );
//!
//! // parse_valid_numbers
//! assert_eq!(
//!     parse_valid_numbers(&["1", "hello", "3", "world"]),
//!     vec![1, 3]
//! );
//!
//! // filter_map_with
//! let doubled: Vec<i32> = filter_map_with(
//!     &[1, 2, 3],
//!     |x| if x % 2 == 0 { Some(x * 2) } else { None }
//! );
//! // Only 2 is even, doubled to 4
//! assert_eq!(doubled, vec![4]);
//!
//! // take_while_positive
//! assert_eq!(
//!     take_while_positive(&[3, 5, -1, 2, 4]),
//!     vec![3, 5]
//! );
//!
//! // skip_while_negative
//! assert_eq!(
//!     skip_while_negative(&[-3, -1, 2, -4, 5]),
//!     vec![2, -4, 5]
//! );
//!
//! // filter_in_range
//! assert_eq!(
//!     filter_in_range(&[1, 5, 10, 15, 20], 5, 15),
//!     vec![5, 10, 15]
//! );
//!
//! // first_matching
//! assert_eq!(
//!     first_matching(&[1, 2, 3, 4, 5], |&x| x > 3),
//!     Some(4)
//! );
//! assert_eq!(
//!     first_matching(&[1, 2, 3], |&x| x > 10),
//!     None
//! );
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - `filter()` takes a closure that receives `&&T` when iterating over `&[T]`, so use `|&&x|` or `|&x|` patterns
//! - `filter_map()` combines filter and map - return `Some(value)` to keep, `None` to skip
//! - `take_while()` stops at the first element that doesn't match the predicate
//! - `skip_while()` skips until predicate fails, then takes all remaining elements
//! - Use `.cloned()` to convert from `&T` to `T` when collecting
//! - For `first_matching`, consider using `.find()` or `.filter().next()`
//! - Remember that `parse::<i32>()` returns a `Result`, use `.ok()` to convert to `Option`
//!
//! </details>
//!
//! Run: `cargo run --example rf_iterator_filtering`
//! Test: `cargo test --example rf_iterator_filtering`
//! Filter: `cargo test --example rf_iterator_filtering <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

/// Filters a slice to return only even numbers.
///
/// Use the `.filter()` method with a predicate that checks for evenness.
pub fn filter_even(numbers: &[i32]) -> Vec<i32> {
    // TODO: Implement this function
    unimplemented!()
}

/// Filters a slice using a custom predicate function.
///
/// The predicate takes a reference to an i32 and returns true for elements to keep.
pub fn filter_by_predicate<F>(numbers: &[i32], predicate: F) -> Vec<i32>
where
    F: Fn(&i32) -> bool,
{
    // TODO: Implement this function
    unimplemented!()
}

/// Parses a slice of strings to integers, keeping only valid parses.
///
/// Use filter_map to combine parsing and filtering in one step.
pub fn parse_valid_numbers(strings: &[&str]) -> Vec<i32> {
    // TODO: Implement this function
    unimplemented!()
}

/// Generic filter_map operation that applies a function returning Option<U>.
///
/// Items where f returns None are filtered out; Some values are collected.
pub fn filter_map_with<T, U, F>(items: &[T], f: F) -> Vec<U>
where
    T: Clone,
    F: Fn(T) -> Option<U>,
{
    // TODO: Implement this function
    unimplemented!()
}

/// Takes elements from the beginning while they are positive (> 0).
///
/// Stops at the first non-positive element.
pub fn take_while_positive(numbers: &[i32]) -> Vec<i32> {
    // TODO: Implement this function
    unimplemented!()
}

/// Skips elements from the beginning while they are negative (< 0).
///
/// After the first non-negative element, takes all remaining elements.
pub fn skip_while_negative(numbers: &[i32]) -> Vec<i32> {
    // TODO: Implement this function
    unimplemented!()
}

/// Filters numbers to keep only those within a range [min, max] inclusive.
pub fn filter_in_range(numbers: &[i32], min: i32, max: i32) -> Vec<i32> {
    // TODO: Implement this function
    unimplemented!()
}

/// Finds the first element that matches a predicate.
///
/// Returns Some(element) if found, None otherwise.
pub fn first_matching<T: Clone, F>(items: &[T], predicate: F) -> Option<T>
where
    F: Fn(&T) -> bool,
{
    // TODO: Implement this function
    unimplemented!()
}

// Example usage
pub fn main() {
    // filter_even
    let numbers = vec![1, 2, 3, 4, 5, 6];
    println!("Even numbers: {:?}", filter_even(&numbers));

    // filter_by_predicate
    println!(
        "Numbers > 3: {:?}",
        filter_by_predicate(&numbers, |&x| x > 3)
    );

    // parse_valid_numbers
    let strings = vec!["1", "hello", "3", "world"];
    println!("Valid numbers: {:?}", parse_valid_numbers(&strings));

    // filter_map_with
    let doubled: Vec<i32> =
        filter_map_with(
            &[1, 2, 3, 4],
            |x| {
                if x % 2 == 0 { Some(x * 2) } else { None }
            },
        );
    println!("Doubled evens: {:?}", doubled);

    // take_while_positive
    let mixed = vec![3, 5, -1, 2, 4];
    println!("Take while positive: {:?}", take_while_positive(&mixed));

    // skip_while_negative
    let negative_start = vec![-3, -1, 2, -4, 5];
    println!(
        "Skip while negative: {:?}",
        skip_while_negative(&negative_start)
    );

    // filter_in_range
    let range_nums = vec![1, 5, 10, 15, 20];
    println!(
        "In range [5, 15]: {:?}",
        filter_in_range(&range_nums, 5, 15)
    );

    // first_matching
    println!("First > 3: {:?}", first_matching(&numbers, |&x| x > 3));
    println!("First > 100: {:?}", first_matching(&numbers, |&x| x > 100));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    // ==================== filter_even tests ====================

    #[test]
    fn test_filter_even_mixed() {
        assert_eq!(filter_even(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
    }

    #[test]
    fn test_filter_even_all_even() {
        assert_eq!(filter_even(&[2, 4, 6, 8]), vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_filter_even_all_odd() {
        assert_eq!(filter_even(&[1, 3, 5, 7]), vec![]);
    }

    #[test]
    fn test_filter_even_empty() {
        assert_eq!(filter_even(&[]), vec![]);
    }

    #[test]
    fn test_filter_even_with_negatives() {
        assert_eq!(filter_even(&[-4, -3, -2, -1, 0, 1, 2]), vec![-4, -2, 0, 2]);
    }

    #[test]
    fn test_filter_even_single_even() {
        assert_eq!(filter_even(&[4]), vec![4]);
    }

    #[test]
    fn test_filter_even_single_odd() {
        assert_eq!(filter_even(&[5]), vec![]);
    }

    // ==================== filter_by_predicate tests ====================

    #[test]
    fn test_filter_by_predicate_greater_than() {
        assert_eq!(
            filter_by_predicate(&[1, 2, 3, 4, 5], |&x| x > 3),
            vec![4, 5]
        );
    }

    #[test]
    fn test_filter_by_predicate_less_than() {
        assert_eq!(
            filter_by_predicate(&[1, 2, 3, 4, 5], |&x| x < 3),
            vec![1, 2]
        );
    }

    #[test]
    fn test_filter_by_predicate_odd() {
        assert_eq!(
            filter_by_predicate(&[1, 2, 3, 4, 5], |&x| x % 2 == 1),
            vec![1, 3, 5]
        );
    }

    #[test]
    fn test_filter_by_predicate_none_match() {
        assert_eq!(filter_by_predicate(&[1, 2, 3], |&x| x > 10), vec![]);
    }

    #[test]
    fn test_filter_by_predicate_all_match() {
        assert_eq!(filter_by_predicate(&[1, 2, 3], |&x| x > 0), vec![1, 2, 3]);
    }

    #[test]
    fn test_filter_by_predicate_empty() {
        assert_eq!(filter_by_predicate(&[], |&x| x > 0), vec![]);
    }

    #[test]
    fn test_filter_by_predicate_divisible_by_3() {
        assert_eq!(
            filter_by_predicate(&[1, 3, 5, 6, 9, 10], |&x| x % 3 == 0),
            vec![3, 6, 9]
        );
    }

    // ==================== parse_valid_numbers tests ====================

    #[test]
    fn test_parse_valid_numbers_mixed() {
        assert_eq!(
            parse_valid_numbers(&["1", "hello", "3", "world"]),
            vec![1, 3]
        );
    }

    #[test]
    fn test_parse_valid_numbers_all_valid() {
        assert_eq!(
            parse_valid_numbers(&["42", "-5", "0", "100"]),
            vec![42, -5, 0, 100]
        );
    }

    #[test]
    fn test_parse_valid_numbers_all_invalid() {
        assert_eq!(parse_valid_numbers(&["abc", "def", "xyz"]), vec![]);
    }

    #[test]
    fn test_parse_valid_numbers_empty() {
        assert_eq!(parse_valid_numbers(&[]), vec![]);
    }

    #[test]
    fn test_parse_valid_numbers_with_whitespace() {
        // parse::<i32>() doesn't accept whitespace
        assert_eq!(parse_valid_numbers(&[" 1", "2 ", " 3 "]), vec![]);
    }

    #[test]
    fn test_parse_valid_numbers_floats_fail() {
        assert_eq!(parse_valid_numbers(&["1.5", "2.0", "3"]), vec![3]);
    }

    #[test]
    fn test_parse_valid_numbers_negative() {
        assert_eq!(parse_valid_numbers(&["-1", "-2", "-3"]), vec![-1, -2, -3]);
    }

    #[test]
    fn test_parse_valid_numbers_large_numbers() {
        assert_eq!(
            parse_valid_numbers(&["2147483647", "-2147483648"]),
            vec![2147483647, -2147483648]
        );
    }

    // ==================== filter_map_with tests ====================

    #[test]
    fn test_filter_map_with_double_even() {
        let result: Vec<i32> =
            filter_map_with(
                &[1, 2, 3, 4],
                |x| if x % 2 == 0 { Some(x * 2) } else { None },
            );
        assert_eq!(result, vec![4, 8]);
    }

    #[test]
    fn test_filter_map_with_square_positive() {
        let result: Vec<i32> =
            filter_map_with(
                &[-2, -1, 0, 1, 2],
                |x| if x > 0 { Some(x * x) } else { None },
            );
        assert_eq!(result, vec![1, 4]);
    }

    #[test]
    fn test_filter_map_with_empty() {
        let result: Vec<i32> = filter_map_with(&[], |x: i32| Some(x));
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_filter_map_with_all_none() {
        let result: Vec<i32> = filter_map_with(&[1, 2, 3], |_| None);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_filter_map_with_all_some() {
        let result: Vec<i32> = filter_map_with(&[1, 2, 3], |x| Some(x + 10));
        assert_eq!(result, vec![11, 12, 13]);
    }

    #[test]
    fn test_filter_map_with_string_lengths() {
        let result: Vec<usize> = filter_map_with(&["a", "bb", "ccc"], |s: &str| {
            if s.len() > 1 { Some(s.len()) } else { None }
        });
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_filter_map_with_type_conversion() {
        let result: Vec<String> = filter_map_with(&[1, 2, 3, 4, 5], |x| {
            if x % 2 == 0 {
                Some(format!("even:{}", x))
            } else {
                None
            }
        });
        assert_eq!(result, vec!["even:2", "even:4"]);
    }

    // ==================== take_while_positive tests ====================

    #[test]
    fn test_take_while_positive_basic() {
        assert_eq!(take_while_positive(&[3, 5, -1, 2, 4]), vec![3, 5]);
    }

    #[test]
    fn test_take_while_positive_starts_negative() {
        assert_eq!(take_while_positive(&[-1, 2, 3]), vec![]);
    }

    #[test]
    fn test_take_while_positive_all_positive() {
        assert_eq!(take_while_positive(&[1, 2, 3, 4]), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_take_while_positive_all_negative() {
        assert_eq!(take_while_positive(&[-1, -2, -3]), vec![]);
    }

    #[test]
    fn test_take_while_positive_empty() {
        assert_eq!(take_while_positive(&[]), vec![]);
    }

    #[test]
    fn test_take_while_positive_stops_at_zero() {
        assert_eq!(take_while_positive(&[1, 2, 0, 3, 4]), vec![1, 2]);
    }

    #[test]
    fn test_take_while_positive_single_positive() {
        assert_eq!(take_while_positive(&[5]), vec![5]);
    }

    #[test]
    fn test_take_while_positive_single_negative() {
        assert_eq!(take_while_positive(&[-5]), vec![]);
    }

    // ==================== skip_while_negative tests ====================

    #[test]
    fn test_skip_while_negative_basic() {
        assert_eq!(skip_while_negative(&[-3, -1, 2, -4, 5]), vec![2, -4, 5]);
    }

    #[test]
    fn test_skip_while_negative_starts_positive() {
        assert_eq!(skip_while_negative(&[1, -2, -3]), vec![1, -2, -3]);
    }

    #[test]
    fn test_skip_while_negative_all_negative() {
        assert_eq!(skip_while_negative(&[-1, -2, -3]), vec![]);
    }

    #[test]
    fn test_skip_while_negative_all_positive() {
        assert_eq!(skip_while_negative(&[1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_skip_while_negative_empty() {
        assert_eq!(skip_while_negative(&[]), vec![]);
    }

    #[test]
    fn test_skip_while_negative_stops_at_zero() {
        assert_eq!(skip_while_negative(&[-1, -2, 0, -3, 4]), vec![0, -3, 4]);
    }

    #[test]
    fn test_skip_while_negative_single_negative() {
        assert_eq!(skip_while_negative(&[-5]), vec![]);
    }

    #[test]
    fn test_skip_while_negative_single_positive() {
        assert_eq!(skip_while_negative(&[5]), vec![5]);
    }

    // ==================== filter_in_range tests ====================

    #[test]
    fn test_filter_in_range_basic() {
        assert_eq!(filter_in_range(&[1, 5, 10, 15, 20], 5, 15), vec![5, 10, 15]);
    }

    #[test]
    fn test_filter_in_range_none_in_range() {
        assert_eq!(filter_in_range(&[1, 2, 3], 10, 20), vec![]);
    }

    #[test]
    fn test_filter_in_range_all_in_range() {
        assert_eq!(filter_in_range(&[5, 6, 7], 1, 10), vec![5, 6, 7]);
    }

    #[test]
    fn test_filter_in_range_empty() {
        assert_eq!(filter_in_range(&[], 1, 10), vec![]);
    }

    #[test]
    fn test_filter_in_range_single_value_range() {
        assert_eq!(filter_in_range(&[1, 2, 3, 4, 5], 3, 3), vec![3]);
    }

    #[test]
    fn test_filter_in_range_negative_range() {
        assert_eq!(
            filter_in_range(&[-5, -3, -1, 0, 1, 3, 5], -3, 1),
            vec![-3, -1, 0, 1]
        );
    }

    #[test]
    fn test_filter_in_range_boundary_values() {
        assert_eq!(
            filter_in_range(&[0, 1, 2, 3, 4, 5], 0, 5),
            vec![0, 1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn test_filter_in_range_large_range() {
        assert_eq!(
            filter_in_range(&[100, 200, 300], 0, 1000),
            vec![100, 200, 300]
        );
    }

    // ==================== first_matching tests ====================

    #[test]
    fn test_first_matching_found() {
        assert_eq!(first_matching(&[1, 2, 3, 4, 5], |&x| x > 3), Some(4));
    }

    #[test]
    fn test_first_matching_not_found() {
        assert_eq!(first_matching(&[1, 2, 3], |&x| x > 10), None);
    }

    #[test]
    fn test_first_matching_empty() {
        assert_eq!(first_matching::<i32, _>(&[], |&x| x > 0), None);
    }

    #[test]
    fn test_first_matching_first_element() {
        assert_eq!(first_matching(&[5, 1, 2, 3], |&x| x > 4), Some(5));
    }

    #[test]
    fn test_first_matching_last_element() {
        assert_eq!(first_matching(&[1, 2, 3, 5], |&x| x > 4), Some(5));
    }

    #[test]
    fn test_first_matching_multiple_matches() {
        // Should return the first match
        assert_eq!(first_matching(&[1, 4, 5, 6], |&x| x > 3), Some(4));
    }

    #[test]
    fn test_first_matching_with_strings() {
        let strings = vec!["apple", "banana", "cherry"];
        assert_eq!(
            first_matching(&strings, |s| s.starts_with('b')),
            Some("banana")
        );
    }

    #[test]
    fn test_first_matching_with_strings_not_found() {
        let strings = vec!["apple", "banana", "cherry"];
        assert_eq!(first_matching(&strings, |s| s.starts_with('z')), None);
    }

    // ==================== Integration tests ====================

    #[test]
    fn test_chained_filtering() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // Filter even numbers, then filter those in range
        let evens = filter_even(&numbers);
        let result = filter_in_range(&evens, 4, 8);
        assert_eq!(result, vec![4, 6, 8]);
    }

    #[test]
    fn test_parse_then_filter() {
        let strings = vec!["1", "20", "hello", "5", "100", "world"];
        let numbers = parse_valid_numbers(&strings);
        let small_numbers = filter_by_predicate(&numbers, |&x| x < 50);
        assert_eq!(small_numbers, vec![1, 20, 5]);
    }

    #[test]
    fn test_take_and_skip_complement() {
        let numbers = vec![1, 2, 3, 4, 5];
        // take_while + skip_while should cover all elements for a boundary condition
        let taken = take_while_positive(&numbers); // [1, 2, 3, 4, 5]
        assert_eq!(taken.len(), 5);

        let numbers_with_negative = vec![-1, -2, 3, 4, 5];
        let skipped = skip_while_negative(&numbers_with_negative); // [3, 4, 5]
        assert_eq!(skipped, vec![3, 4, 5]);
    }

    #[test]
    fn test_filter_map_chain() {
        // Parse strings, filter positives, then double
        let strings = vec!["1", "-2", "3", "invalid", "-4", "5"];
        let numbers = parse_valid_numbers(&strings); // [1, -2, 3, -4, 5]
        let result: Vec<i32> =
            filter_map_with(&numbers, |x| if x > 0 { Some(x * 2) } else { None });
        assert_eq!(result, vec![2, 6, 10]);
    }

    #[test]
    fn test_first_matching_with_filter() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let evens = filter_even(&numbers);
        let first_greater_than_5 = first_matching(&evens, |&x| x > 5);
        assert_eq!(first_greater_than_5, Some(6));
    }

    #[test]
    fn test_complex_pipeline() {
        // Start with strings, parse, filter range, find first matching condition
        let data = vec!["5", "10", "abc", "15", "20", "xyz", "25", "30"];
        let numbers = parse_valid_numbers(&data); // [5, 10, 15, 20, 25, 30]
        let in_range = filter_in_range(&numbers, 10, 25); // [10, 15, 20, 25]
        let first_odd = first_matching(&in_range, |&x| x % 2 == 1);
        assert_eq!(first_odd, Some(15));
    }

    #[test]
    fn test_edge_case_all_filtered_out() {
        let numbers = vec![1, 3, 5, 7, 9];
        let evens = filter_even(&numbers);
        assert!(evens.is_empty());

        let first = first_matching(&evens, |&x| x > 0);
        assert_eq!(first, None);
    }

    #[test]
    fn test_filter_with_custom_struct() {
        #[derive(Clone, Debug, PartialEq)]
        struct Person {
            name: String,
            age: u32,
        }

        let people = vec![
            Person {
                name: "Alice".to_string(),
                age: 25,
            },
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
            Person {
                name: "Charlie".to_string(),
                age: 20,
            },
        ];

        let adult = first_matching(&people, |p| p.age >= 25);
        assert_eq!(
            adult,
            Some(Person {
                name: "Alice".to_string(),
                age: 25
            })
        );

        let elderly = first_matching(&people, |p| p.age >= 65);
        assert_eq!(elderly, None);
    }
}
