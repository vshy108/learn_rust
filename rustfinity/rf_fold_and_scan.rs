//! # Rustfinity: Fold And Scan
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/fold-and-scan
//!
//! The `fold()` and `scan()` methods are powerful iterator adapters that allow you to maintain state across iteration. They're essential for tasks like computing running totals, building complex aggregations, or transforming sequences based on accumulated state.
//!
//! ## Understanding fold()
//!
//! `fold()` reduces an iterator to a single value by repeatedly applying an accumulator function. It takes an initial value and a closure that combines the accumulator with each element:
//!
//! ```rust
//! let numbers = vec![1, 2, 3, 4, 5];
//!
//! // Sum all numbers
//! let sum = numbers.iter().fold(0, |acc, &x| acc + x);
//! assert_eq!(sum, 15);
//!
//! // Product of all numbers
//! let product = numbers.iter().fold(1, |acc, &x| acc * x);
//! assert_eq!(product, 120);
//!
//! // Build a string
//! let words = vec!["Hello", "World"];
//! let sentence = words
//!     .iter()
//!     .fold(String::new(), |acc, &word| {
//!     if acc.is_empty() {
//!         word.to_string()
//!     } else {
//!         format!("{} {}", acc, word)
//!     }
//! });
//! assert_eq!(sentence, "Hello World");
//! ```
//!
//! ## Understanding scan()
//!
//! `scan()` is similar to `fold()` but yields intermediate values at each step, producing a new iterator. It maintains mutable state and returns `Option<T>` values:
//!
//! ```rust
//! let numbers = vec![1, 2, 3, 4, 5];
//!
//! // Running sum - yields cumulative totals
//! let running_sums: Vec<i32> = numbers
//!     .iter()
//!     .scan(0, |state, &x| {
//!         *state += x;
//!         Some(*state)
//!     })
//!     .collect();
//! assert_eq!(running_sums, vec![1, 3, 6, 10, 15]);
//!
//! // Running product
//! let running_products: Vec<i32> = numbers
//!     .iter()
//!     .scan(1, |state, &x| {
//!         *state *= x;
//!         Some(*state)
//!     })
//!     .collect();
//! assert_eq!(running_products, vec![1, 2, 6, 24, 120]);
//! ```
//!
//! ## Key Differences
//!
//! - **`fold()`**: Consumes the entire iterator and returns a single final value
//! - **`scan()`**: Returns an iterator of intermediate results; can stop early by returning `None`
//!
//! ```rust
//! // scan can stop early
//! let numbers = vec![1, 2, 3, 4, 5];
//! let until_exceeds_5: Vec<i32> = numbers
//!     .iter()
//!     .scan(0, |state, &x| {
//!         *state += x;
//!         if *state <= 5 {
//!             Some(*state)
//!         } else {
//!             None  // Stop iteration
//!         }
//!     })
//!     .collect();
//! // Stops before exceeding 5
//! assert_eq!(until_exceeds_5, vec![1, 3]);
//! ```
//!
//! ## Your Task
//!
//! Implement the following functions that demonstrate `fold()` and `scan()` patterns:
//!
//! 1. `sum_with_fold(numbers: &[i32]) -> i32` - Sum all numbers using `fold()`
//! 2. `product_with_fold(numbers: &[i32]) -> i64` - Multiply all numbers using `fold()` (use i64 to avoid overflow)
//! 3. `concat_strings(strings: &[&str], separator: &str) -> String`
//!    - Join strings with a separator using `fold()`
//! 4. `running_sum(numbers: &[i32]) -> Vec<i32>`
//!    - Return cumulative sums using `scan()`
//! 5. `running_max(numbers: &[i32]) -> Vec<i32>`
//!    - Return running maximum values using `scan()`
//! 6. `take_while_sum_under(numbers: &[i32], limit: i32) -> Vec<i32>` - Take numbers while running sum stays under limit using `scan()`
//! 7. `count_occurrences<T: PartialEq>(items: &[T], target: &T) -> usize`
//!    - Count occurrences using `fold()`
//! 8. `running_average(numbers: &[f64]) -> Vec<f64>`
//!    - Return running average at each position using `scan()`
//!
//! ## Examples
//!
//! ```rust
//! // sum_with_fold
//! assert_eq!(sum_with_fold(&[1, 2, 3, 4, 5]), 15);
//!
//! // product_with_fold
//! assert_eq!(product_with_fold(&[1, 2, 3, 4]), 24);
//!
//! // concat_strings
//! assert_eq!(concat_strings(&["a", "b", "c"], "-"), "a-b-c");
//!
//! // running_sum
//! assert_eq!(running_sum(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
//!
//! // running_max
//! assert_eq!(
//!     running_max(&[3, 1, 4, 1, 5]),
//!     vec![3, 3, 4, 4, 5]
//! );
//!
//! // take_while_sum_under
//! // Takes [1, 2, 3] but stops before adding 4
//! // (sum would be 10 >= 10)
//! assert_eq!(
//!     take_while_sum_under(&[1, 2, 3, 4, 5], 10),
//!     vec![1, 2, 3]
//! );
//!
//! // count_occurrences
//! assert_eq!(count_occurrences(&[1, 2, 1, 3, 1], &1), 3);
//!
//! // running_average
//! assert_eq!(
//!     running_average(&[2.0, 4.0, 6.0]),
//!     vec![2.0, 3.0, 4.0]
//! );
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - For `fold()`, the closure signature is `|accumulator, element| -> new_accumulator`
//! - For `scan()`, the closure signature is `|&mut state, element| -> Option<output>`
//! - In `scan()`, you modify `state` in place using `*state = ...`
//! - Use `scan()` returning `None` to stop iteration early
//! - For `running_average`, track both the sum and count in the state (use a tuple)
//! - `product_with_fold` uses i64 to handle larger products safely
//! - Empty slices should return sensible defaults (0 for sum, 1 for product, empty string, etc.)
//!
//! </details>
//!
//! Run: `cargo run --example rf_fold_and_scan`
//! Test: `cargo test --example rf_fold_and_scan`
//! Filter: `cargo test --example rf_fold_and_scan <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

/// Sums all numbers using fold().
///
/// # Arguments
/// * `numbers` - The slice of numbers to sum
///
/// # Returns
/// The sum of all numbers (0 for empty slice)
pub fn sum_with_fold(numbers: &[i32]) -> i32 {
    // TODO: Implement this function
    unimplemented!()
}

/// Multiplies all numbers using fold().
///
/// Uses i64 to handle larger products safely.
///
/// # Arguments
/// * `numbers` - The slice of numbers to multiply
///
/// # Returns
/// The product of all numbers (1 for empty slice)
pub fn product_with_fold(numbers: &[i32]) -> i64 {
    // TODO: Implement this function
    unimplemented!()
}

/// Joins strings with a separator using fold().
///
/// # Arguments
/// * `strings` - The slice of string slices to join
/// * `separator` - The separator to place between strings
///
/// # Returns
/// A single string with all inputs joined by the separator
pub fn concat_strings(strings: &[&str], separator: &str) -> String {
    // TODO: Implement this function
    unimplemented!()
}

/// Returns cumulative sums using scan().
///
/// # Arguments
/// * `numbers` - The slice of numbers
///
/// # Returns
/// A Vec where each element is the sum of all elements up to that position
pub fn running_sum(numbers: &[i32]) -> Vec<i32> {
    // TODO: Implement this function
    unimplemented!()
}

/// Returns running maximum values using scan().
///
/// # Arguments
/// * `numbers` - The slice of numbers
///
/// # Returns
/// A Vec where each element is the maximum of all elements up to that position
pub fn running_max(numbers: &[i32]) -> Vec<i32> {
    // TODO: Implement this function
    unimplemented!()
}

/// Takes numbers while running sum stays strictly under limit.
///
/// Stops before adding a number that would make the sum >= limit.
///
/// # Arguments
/// * `numbers` - The slice of numbers
/// * `limit` - The exclusive upper limit for the running sum
///
/// # Returns
/// A Vec of numbers taken while sum stays under limit
pub fn take_while_sum_under(numbers: &[i32], limit: i32) -> Vec<i32> {
    // TODO: Implement this function
    unimplemented!()
}

/// Counts occurrences of target in items using fold().
///
/// # Arguments
/// * `items` - The slice to search
/// * `target` - The value to count
///
/// # Returns
/// The number of times target appears in items
pub fn count_occurrences<T: PartialEq>(items: &[T], target: &T) -> usize {
    // TODO: Implement this function
    unimplemented!()
}

/// Returns running average at each position using scan().
///
/// # Arguments
/// * `numbers` - The slice of numbers
///
/// # Returns
/// A Vec where each element is the average of all elements up to that position
pub fn running_average(numbers: &[f64]) -> Vec<f64> {
    // TODO: Implement this function
    unimplemented!()
}

// Example usage
pub fn main() {
    // sum_with_fold
    let sum = sum_with_fold(&[1, 2, 3, 4, 5]);
    println!("Sum: {}", sum); // Expected: 15

    // product_with_fold
    let product = product_with_fold(&[1, 2, 3, 4]);
    println!("Product: {}", product); // Expected: 24

    // concat_strings
    let joined = concat_strings(&["Hello", "World"], " ");
    println!("Joined: {}", joined); // Expected: "Hello World"

    // running_sum
    let sums = running_sum(&[1, 2, 3, 4]);
    println!("Running sums: {:?}", sums); // Expected: [1, 3, 6, 10]

    // running_max
    let maxes = running_max(&[3, 1, 4, 1, 5]);
    println!("Running max: {:?}", maxes); // Expected: [3, 3, 4, 4, 5]

    // take_while_sum_under
    let taken = take_while_sum_under(&[1, 2, 3, 4, 5], 10);
    println!("Taken while sum < 10: {:?}", taken); // Expected: [1, 2, 3]

    // count_occurrences
    let count = count_occurrences(&[1, 2, 1, 3, 1], &1);
    println!("Count of 1: {}", count); // Expected: 3

    // running_average
    let averages = running_average(&[2.0, 4.0, 6.0]);
    println!("Running averages: {:?}", averages); // Expected: [2.0, 3.0, 4.0]
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    // ============================================================================
    // sum_with_fold tests
    // ============================================================================

    #[test]
    fn test_sum_with_fold_basic() {
        assert_eq!(sum_with_fold(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_sum_with_fold_empty() {
        assert_eq!(sum_with_fold(&[]), 0);
    }

    #[test]
    fn test_sum_with_fold_single() {
        assert_eq!(sum_with_fold(&[42]), 42);
    }

    #[test]
    fn test_sum_with_fold_negative() {
        assert_eq!(sum_with_fold(&[-1, -2, -3]), -6);
    }

    #[test]
    fn test_sum_with_fold_mixed() {
        assert_eq!(sum_with_fold(&[-5, 10, -3, 8]), 10);
    }

    #[test]
    fn test_sum_with_fold_zeros() {
        assert_eq!(sum_with_fold(&[0, 0, 0]), 0);
    }

    // ============================================================================
    // product_with_fold tests
    // ============================================================================

    #[test]
    fn test_product_with_fold_basic() {
        assert_eq!(product_with_fold(&[1, 2, 3, 4]), 24);
    }

    #[test]
    fn test_product_with_fold_empty() {
        assert_eq!(product_with_fold(&[]), 1);
    }

    #[test]
    fn test_product_with_fold_single() {
        assert_eq!(product_with_fold(&[7]), 7);
    }

    #[test]
    fn test_product_with_fold_with_zero() {
        assert_eq!(product_with_fold(&[1, 2, 0, 4]), 0);
    }

    #[test]
    fn test_product_with_fold_negative() {
        assert_eq!(product_with_fold(&[-2, 3, -4]), 24);
    }

    #[test]
    fn test_product_with_fold_large_numbers() {
        // Test that i64 handles larger products
        assert_eq!(product_with_fold(&[1000, 1000, 1000]), 1_000_000_000);
    }

    #[test]
    fn test_product_with_fold_ones() {
        assert_eq!(product_with_fold(&[1, 1, 1, 1]), 1);
    }

    // ============================================================================
    // concat_strings tests
    // ============================================================================

    #[test]
    fn test_concat_strings_basic() {
        assert_eq!(concat_strings(&["a", "b", "c"], "-"), "a-b-c");
    }

    #[test]
    fn test_concat_strings_empty() {
        assert_eq!(concat_strings(&[], "-"), "");
    }

    #[test]
    fn test_concat_strings_single() {
        assert_eq!(concat_strings(&["hello"], ", "), "hello");
    }

    #[test]
    fn test_concat_strings_spaces() {
        assert_eq!(concat_strings(&["Hello", "World"], " "), "Hello World");
    }

    #[test]
    fn test_concat_strings_empty_separator() {
        assert_eq!(concat_strings(&["a", "b", "c"], ""), "abc");
    }

    #[test]
    fn test_concat_strings_long_separator() {
        assert_eq!(concat_strings(&["x", "y"], " -> "), "x -> y");
    }

    #[test]
    fn test_concat_strings_with_empty_strings() {
        assert_eq!(concat_strings(&["", "a", ""], "-"), "-a-");
    }

    // ============================================================================
    // running_sum tests
    // ============================================================================

    #[test]
    fn test_running_sum_basic() {
        assert_eq!(running_sum(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
    }

    #[test]
    fn test_running_sum_empty() {
        assert_eq!(running_sum(&[]), Vec::<i32>::new());
    }

    #[test]
    fn test_running_sum_single() {
        assert_eq!(running_sum(&[5]), vec![5]);
    }

    #[test]
    fn test_running_sum_negative() {
        assert_eq!(running_sum(&[-1, -2, -3]), vec![-1, -3, -6]);
    }

    #[test]
    fn test_running_sum_mixed() {
        assert_eq!(running_sum(&[10, -5, 3, -2]), vec![10, 5, 8, 6]);
    }

    #[test]
    fn test_running_sum_zeros() {
        assert_eq!(running_sum(&[0, 0, 5, 0]), vec![0, 0, 5, 5]);
    }

    // ============================================================================
    // running_max tests
    // ============================================================================

    #[test]
    fn test_running_max_basic() {
        assert_eq!(running_max(&[3, 1, 4, 1, 5]), vec![3, 3, 4, 4, 5]);
    }

    #[test]
    fn test_running_max_empty() {
        assert_eq!(running_max(&[]), Vec::<i32>::new());
    }

    #[test]
    fn test_running_max_single() {
        assert_eq!(running_max(&[42]), vec![42]);
    }

    #[test]
    fn test_running_max_ascending() {
        assert_eq!(running_max(&[1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_running_max_descending() {
        assert_eq!(running_max(&[5, 4, 3, 2, 1]), vec![5, 5, 5, 5, 5]);
    }

    #[test]
    fn test_running_max_negative() {
        assert_eq!(running_max(&[-5, -3, -4, -1, -2]), vec![-5, -3, -3, -1, -1]);
    }

    #[test]
    fn test_running_max_same_values() {
        assert_eq!(running_max(&[7, 7, 7]), vec![7, 7, 7]);
    }

    // ============================================================================
    // take_while_sum_under tests
    // ============================================================================

    #[test]
    fn test_take_while_sum_under_basic() {
        assert_eq!(take_while_sum_under(&[1, 2, 3, 4, 5], 10), vec![1, 2, 3]);
    }

    #[test]
    fn test_take_while_sum_under_empty() {
        assert_eq!(take_while_sum_under(&[], 10), Vec::<i32>::new());
    }

    #[test]
    fn test_take_while_sum_under_all_pass() {
        assert_eq!(take_while_sum_under(&[1, 2, 3], 100), vec![1, 2, 3]);
    }

    #[test]
    fn test_take_while_sum_under_none_pass() {
        assert_eq!(take_while_sum_under(&[10, 20], 5), Vec::<i32>::new());
    }

    #[test]
    fn test_take_while_sum_under_first_only() {
        assert_eq!(take_while_sum_under(&[5, 5, 5], 10), vec![5]);
    }

    #[test]
    fn test_take_while_sum_under_exact_limit() {
        // Sum of [2, 3] = 5, next would be 5 + 5 = 10, which is NOT < 10
        assert_eq!(take_while_sum_under(&[2, 3, 5], 10), vec![2, 3]);
    }

    #[test]
    fn test_take_while_sum_under_single_element() {
        assert_eq!(take_while_sum_under(&[5], 10), vec![5]);
        assert_eq!(take_while_sum_under(&[15], 10), Vec::<i32>::new());
    }

    #[test]
    fn test_take_while_sum_under_zero_limit() {
        assert_eq!(take_while_sum_under(&[1, 2, 3], 0), Vec::<i32>::new());
    }

    // ============================================================================
    // count_occurrences tests
    // ============================================================================

    #[test]
    fn test_count_occurrences_basic() {
        assert_eq!(count_occurrences(&[1, 2, 1, 3, 1], &1), 3);
    }

    #[test]
    fn test_count_occurrences_empty() {
        assert_eq!(count_occurrences(&[] as &[i32], &1), 0);
    }

    #[test]
    fn test_count_occurrences_not_found() {
        assert_eq!(count_occurrences(&[1, 2, 3], &4), 0);
    }

    #[test]
    fn test_count_occurrences_all_match() {
        assert_eq!(count_occurrences(&[5, 5, 5, 5], &5), 4);
    }

    #[test]
    fn test_count_occurrences_strings() {
        assert_eq!(count_occurrences(&["a", "b", "a", "c", "a"], &"a"), 3);
    }

    #[test]
    fn test_count_occurrences_single() {
        assert_eq!(count_occurrences(&[42], &42), 1);
        assert_eq!(count_occurrences(&[42], &0), 0);
    }

    // ============================================================================
    // running_average tests
    // ============================================================================

    #[test]
    fn test_running_average_basic() {
        assert_eq!(running_average(&[2.0, 4.0, 6.0]), vec![2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_running_average_empty() {
        assert_eq!(running_average(&[]), Vec::<f64>::new());
    }

    #[test]
    fn test_running_average_single() {
        assert_eq!(running_average(&[5.0]), vec![5.0]);
    }

    #[test]
    fn test_running_average_same_values() {
        assert_eq!(running_average(&[3.0, 3.0, 3.0]), vec![3.0, 3.0, 3.0]);
    }

    #[test]
    fn test_running_average_ascending() {
        let result = running_average(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(result, vec![1.0, 1.5, 2.0, 2.5, 3.0]);
    }

    #[test]
    fn test_running_average_with_zero() {
        assert_eq!(running_average(&[0.0, 10.0, 5.0]), vec![0.0, 5.0, 5.0]);
    }

    #[test]
    fn test_running_average_negative() {
        assert_eq!(running_average(&[-2.0, -4.0, -6.0]), vec![-2.0, -3.0, -4.0]);
    }

    // ============================================================================
    // Integration tests
    // ============================================================================

    #[test]
    fn test_fold_sum_equals_running_sum_last() {
        let numbers = vec![1, 2, 3, 4, 5];
        let sum = sum_with_fold(&numbers);
        let running = running_sum(&numbers);
        assert_eq!(running.last().copied(), Some(sum));
    }

    #[test]
    fn test_fold_product_equals_scan_product_last() {
        let numbers = vec![1, 2, 3, 4];
        let product = product_with_fold(&numbers);

        // Compute running product using scan
        let running_product: Vec<i64> = numbers
            .iter()
            .scan(1i64, |state, &x| {
                *state *= x as i64;
                Some(*state)
            })
            .collect();

        assert_eq!(running_product.last().copied(), Some(product));
    }

    #[test]
    fn test_running_max_monotonically_non_decreasing() {
        let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let maxes = running_max(&numbers);

        for i in 1..maxes.len() {
            assert!(
                maxes[i] >= maxes[i - 1],
                "Running max should be non-decreasing"
            );
        }
    }

    #[test]
    fn test_take_while_and_count() {
        let numbers = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]; // 10 ones
        let taken = take_while_sum_under(&numbers, 5);
        assert_eq!(taken.len(), 4); // Can take 4 ones (sum = 4 < 5)
    }

    #[test]
    fn test_concat_then_count() {
        let words = vec!["hello", "world", "hello"];
        let joined = concat_strings(&words, " ");
        assert!(joined.contains("hello"));
        assert_eq!(count_occurrences(&words, &"hello"), 2);
    }

    #[test]
    fn test_running_average_precision() {
        let numbers = vec![1.0, 2.0, 3.0];
        let averages = running_average(&numbers);

        // Check precision
        assert!((averages[0] - 1.0).abs() < f64::EPSILON);
        assert!((averages[1] - 1.5).abs() < f64::EPSILON);
        assert!((averages[2] - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_complex_pipeline() {
        // Take numbers while sum < 15, then get running max
        let numbers = vec![5, 3, 2, 8, 1];
        let taken = take_while_sum_under(&numbers, 15); // [5, 3, 2] (sum = 10)
        assert_eq!(taken, vec![5, 3, 2]);

        let maxes = running_max(&taken);
        assert_eq!(maxes, vec![5, 5, 5]);
    }

    #[test]
    fn test_count_with_custom_type() {
        #[derive(PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        let points = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 0, y: 0 },
        ];

        let origin = Point { x: 0, y: 0 };
        assert_eq!(count_occurrences(&points, &origin), 2);
    }
}
