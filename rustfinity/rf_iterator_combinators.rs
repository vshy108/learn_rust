//! # Rustfinity: Iterator Combinators
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/iterator-combinators
//!
//! Iterator combinators are methods on iterators that allow you to compose and transform sequences of values in powerful ways. They are a cornerstone of functional programming in Rust, enabling you to write concise, expressive, and efficient code without explicit loops.
//!
//! ## Understanding Iterator Combinators
//!
//! Rust's `Iterator` trait provides dozens of combinator methods. These methods take an iterator and return a new iterator with modified behavior. Because they're lazy, combinators don't actually do any work until you consume the iterator (e.g., with `collect()`, `for_each()`, or `sum()`).
//!
//! ```rust
//! let numbers = vec![1, 2, 3, 4, 5];
//!
//! // chain() combines two iterators into one
//! let more_numbers = vec![6, 7, 8];
//! let combined: Vec<i32> = numbers.iter()
//!     .chain(more_numbers.iter())
//!     .copied()
//!     .collect();
//! // combined = [1, 2, 3, 4, 5, 6, 7, 8]
//!
//! // zip() pairs elements from two iterators
//! let names = vec!["Alice", "Bob", "Charlie"];
//! let ages = vec![30, 25, 35];
//! let people: Vec<_> = names
//!     .iter()
//!     .zip(ages.iter())
//!     .collect();
//! // people = [("Alice", 30), ("Bob", 25), ("Charlie", 35)]
//! ```
//!
//! ## Key Combinators
//!
//! - **`chain(other)`** - Concatenates two iterators, yielding all elements from the first, then all from the second
//! - **`zip(other)`** - Pairs up elements from two iterators; stops when either is exhausted
//! - **`take(n)`** - Yields only the first `n` elements
//! - **`skip(n)`** - Skips the first `n` elements, yielding the rest
//! - **`rev()`** - Reverses a double-ended iterator
//! - **`cycle()`** - Repeats an iterator infinitely
//!
//! ```rust
//! // take() and skip() for slicing
//! let nums: Vec<i32> = (1..=10).skip(2).take(5).collect();
//! // nums = [3, 4, 5, 6, 7]
//!
//! // rev() to reverse
//! let reversed: Vec<i32> = vec![1, 2, 3]
//!     .into_iter()
//!     .rev()
//!     .collect();
//! // reversed = [3, 2, 1]
//!
//! // cycle() for infinite repetition (use with take!)
//! let pattern: Vec<i32> = [1, 2]
//!     .iter()
//!     .copied()
//!     .cycle()
//!     .take(6)
//!     .collect();
//! // pattern = [1, 2, 1, 2, 1, 2]
//! ```
//!
//! ## Your Task
//!
//! Implement the following functions that use iterator combinators:
//!
//! 1. `chain_sequences<T: Clone>(first: &[T], second: &[T]) -> Vec<T>`
//!    - Combine two slices into a single Vec using `chain()`
//! 2. `zip_pairs<T: Clone, U: Clone>(first: &[T],
//!   second: &[U]) -> Vec<(T, U)>`
//!    - Pair elements from two slices using `zip()`
//! 3. `take_first<T: Clone>(items: &[T], n: usize) -> Vec<T>` - Return the first `n` elements using `take()`
//! 4. `skip_first<T: Clone>(items: &[T], n: usize) -> Vec<T>` - Skip the first `n` elements using `skip()`
//! 5. `reverse_sequence<T: Clone>(items: &[T]) -> Vec<T>` - Reverse a sequence using `rev()`
//! 6. `interleave<T: Clone>(first: &[T], second: &[T]) -> Vec<T>`
//!    - Interleave elements from two slices
//!      (alternating elements)
//! 7. `sliding_pairs<T: Clone>(items: &[T]) -> Vec<(T, T)>`
//!    - Create pairs of consecutive elements using `zip()`
//!      with a skipped version
//!
//! ## Examples
//!
//! ```rust
//! // chain_sequences
//! assert_eq!(
//!     chain_sequences(&[1, 2], &[3, 4]),
//!     vec![1, 2, 3, 4]
//! );
//!
//! // zip_pairs
//! assert_eq!(
//!     zip_pairs(&[1, 2, 3], &["a", "b"]),
//!     vec![(1, "a"), (2, "b")]
//! );
//!
//! // take_first
//! assert_eq!(take_first(&[1, 2, 3, 4, 5], 3), vec![1, 2, 3]);
//!
//! // skip_first
//! assert_eq!(skip_first(&[1, 2, 3, 4, 5], 2), vec![3, 4, 5]);
//!
//! // reverse_sequence
//! assert_eq!(reverse_sequence(&[1, 2, 3]), vec![3, 2, 1]);
//!
//! // interleave - alternates elements from both slices
//! assert_eq!(
//!     interleave(&[1, 3, 5], &[2, 4, 6]),
//!     vec![1, 2, 3, 4, 5, 6]
//! );
//!
//! // sliding_pairs - pairs consecutive elements
//! assert_eq!(
//!     sliding_pairs(&[1, 2, 3, 4]),
//!     vec![(1, 2), (2, 3), (3, 4)]
//! );
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - For `chain_sequences`, use `.iter().chain(other.iter())` and then `.cloned().collect()`
//! - For `zip_pairs`, use `.iter().zip(other.iter())` - remember zip stops at the shorter iterator
//! - For `interleave`, use `zip()` with `flat_map()` to flatten tuples into individual elements
//! - For `sliding_pairs`, zip the original iterator with a version that skips the first element
//! - Most combinators work with iterators, so start with `.iter()` on the slice
//! - Use `.cloned()` to convert `&T` to `T` when you need owned values
//!
//! </details>
//!
//! Run: `cargo run --example rf_iterator_combinators`
//! Test: `cargo test --example rf_iterator_combinators`
//! Filter: `cargo test --example rf_iterator_combinators <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

/// Combines two slices into a single Vec using chain().
///
/// # Arguments
/// * `first` - The first slice
/// * `second` - The second slice
///
/// # Returns
/// A Vec containing all elements from first, followed by all elements from second
pub fn chain_sequences<T: Clone>(first: &[T], second: &[T]) -> Vec<T> {
    // TODO: Implement this function
    unimplemented!()
}

/// Pairs elements from two slices using zip().
///
/// Stops when either slice is exhausted (uses the shorter length).
///
/// # Arguments
/// * `first` - The first slice
/// * `second` - The second slice
///
/// # Returns
/// A Vec of tuples pairing elements from both slices
pub fn zip_pairs<T: Clone, U: Clone>(first: &[T], second: &[U]) -> Vec<(T, U)> {
    // TODO: Implement this function
    unimplemented!()
}

/// Returns the first n elements of a slice using take().
///
/// If n is greater than the slice length, returns all elements.
///
/// # Arguments
/// * `items` - The slice to take from
/// * `n` - The number of elements to take
///
/// # Returns
/// A Vec containing at most n elements from the beginning
pub fn take_first<T: Clone>(items: &[T], n: usize) -> Vec<T> {
    // TODO: Implement this function
    unimplemented!()
}

/// Skips the first n elements and returns the rest using skip().
///
/// If n is greater than or equal to the slice length, returns an empty Vec.
///
/// # Arguments
/// * `items` - The slice to skip from
/// * `n` - The number of elements to skip
///
/// # Returns
/// A Vec containing all elements after skipping the first n
pub fn skip_first<T: Clone>(items: &[T], n: usize) -> Vec<T> {
    // TODO: Implement this function
    unimplemented!()
}

/// Reverses a sequence using rev().
///
/// # Arguments
/// * `items` - The slice to reverse
///
/// # Returns
/// A Vec with elements in reverse order
pub fn reverse_sequence<T: Clone>(items: &[T]) -> Vec<T> {
    // TODO: Implement this function
    unimplemented!()
}

/// Interleaves elements from two slices, alternating between them.
///
/// Takes one element from first, then one from second, repeating.
/// If slices have different lengths, remaining elements from the longer slice are appended.
///
/// # Arguments
/// * `first` - The first slice
/// * `second` - The second slice
///
/// # Returns
/// A Vec with elements interleaved from both slices
pub fn interleave<T: Clone>(first: &[T], second: &[T]) -> Vec<T> {
    // TODO: Implement this function
    unimplemented!()
}

/// Creates pairs of consecutive elements using zip() with a skipped version.
///
/// Returns pairs of (element[i], element[i+1]) for all valid i.
///
/// # Arguments
/// * `items` - The slice to create sliding pairs from
///
/// # Returns
/// A Vec of tuples containing consecutive element pairs
pub fn sliding_pairs<T: Clone>(items: &[T]) -> Vec<(T, T)> {
    // TODO: Implement this function
    unimplemented!()
}

pub fn main() {
    // chain_sequences example
    let result = chain_sequences(&[1, 2], &[3, 4]);
    println!("chain_sequences([1, 2], [3, 4]) = {:?}", result);

    // zip_pairs example
    let result = zip_pairs(&[1, 2, 3], &["a", "b"]);
    println!("zip_pairs([1, 2, 3], [\"a\", \"b\"]) = {:?}", result);

    // take_first example
    let result = take_first(&[1, 2, 3, 4, 5], 3);
    println!("take_first([1, 2, 3, 4, 5], 3) = {:?}", result);

    // skip_first example
    let result = skip_first(&[1, 2, 3, 4, 5], 2);
    println!("skip_first([1, 2, 3, 4, 5], 2) = {:?}", result);

    // reverse_sequence example
    let result = reverse_sequence(&[1, 2, 3]);
    println!("reverse_sequence([1, 2, 3]) = {:?}", result);

    // interleave example
    let result = interleave(&[1, 3, 5], &[2, 4, 6]);
    println!("interleave([1, 3, 5], [2, 4, 6]) = {:?}", result);

    // sliding_pairs example
    let result = sliding_pairs(&[1, 2, 3, 4]);
    println!("sliding_pairs([1, 2, 3, 4]) = {:?}", result);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    // ==================== chain_sequences tests ====================

    #[test]
    fn test_chain_sequences_basic() {
        assert_eq!(chain_sequences(&[1, 2], &[3, 4]), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_chain_sequences_empty_first() {
        let empty: &[i32] = &[];
        assert_eq!(chain_sequences(empty, &[1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_chain_sequences_empty_second() {
        let empty: &[i32] = &[];
        assert_eq!(chain_sequences(&[1, 2, 3], empty), vec![1, 2, 3]);
    }

    #[test]
    fn test_chain_sequences_both_empty() {
        let empty: &[i32] = &[];
        assert_eq!(chain_sequences(empty, empty), Vec::<i32>::new());
    }

    #[test]
    fn test_chain_sequences_strings() {
        assert_eq!(
            chain_sequences(&["a", "b"], &["c", "d"]),
            vec!["a", "b", "c", "d"]
        );
    }

    #[test]
    fn test_chain_sequences_single_elements() {
        assert_eq!(chain_sequences(&[1], &[2]), vec![1, 2]);
    }

    // ==================== zip_pairs tests ====================

    #[test]
    fn test_zip_pairs_equal_length() {
        assert_eq!(
            zip_pairs(&[1, 2, 3], &["a", "b", "c"]),
            vec![(1, "a"), (2, "b"), (3, "c")]
        );
    }

    #[test]
    fn test_zip_pairs_first_shorter() {
        assert_eq!(
            zip_pairs(&[1, 2], &["a", "b", "c", "d"]),
            vec![(1, "a"), (2, "b")]
        );
    }

    #[test]
    fn test_zip_pairs_second_shorter() {
        assert_eq!(
            zip_pairs(&[1, 2, 3, 4], &["a", "b"]),
            vec![(1, "a"), (2, "b")]
        );
    }

    #[test]
    fn test_zip_pairs_empty_first() {
        let empty: &[i32] = &[];
        let result: Vec<(i32, &str)> = zip_pairs(empty, &["a", "b"]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_zip_pairs_empty_second() {
        let empty: &[&str] = &[];
        let result: Vec<(i32, &str)> = zip_pairs(&[1, 2], empty);
        assert!(result.is_empty());
    }

    #[test]
    fn test_zip_pairs_single_elements() {
        assert_eq!(zip_pairs(&[42], &["only"]), vec![(42, "only")]);
    }

    #[test]
    fn test_zip_pairs_numbers() {
        assert_eq!(
            zip_pairs(&[1, 2, 3], &[10, 20, 30]),
            vec![(1, 10), (2, 20), (3, 30)]
        );
    }

    // ==================== take_first tests ====================

    #[test]
    fn test_take_first_basic() {
        assert_eq!(take_first(&[1, 2, 3, 4, 5], 3), vec![1, 2, 3]);
    }

    #[test]
    fn test_take_first_zero() {
        assert_eq!(take_first(&[1, 2, 3], 0), Vec::<i32>::new());
    }

    #[test]
    fn test_take_first_all() {
        assert_eq!(take_first(&[1, 2, 3], 3), vec![1, 2, 3]);
    }

    #[test]
    fn test_take_first_more_than_length() {
        assert_eq!(take_first(&[1, 2, 3], 10), vec![1, 2, 3]);
    }

    #[test]
    fn test_take_first_empty() {
        let empty: &[i32] = &[];
        assert_eq!(take_first(empty, 5), Vec::<i32>::new());
    }

    #[test]
    fn test_take_first_one() {
        assert_eq!(take_first(&[1, 2, 3, 4, 5], 1), vec![1]);
    }

    #[test]
    fn test_take_first_strings() {
        assert_eq!(take_first(&["a", "b", "c", "d"], 2), vec!["a", "b"]);
    }

    // ==================== skip_first tests ====================

    #[test]
    fn test_skip_first_basic() {
        assert_eq!(skip_first(&[1, 2, 3, 4, 5], 2), vec![3, 4, 5]);
    }

    #[test]
    fn test_skip_first_zero() {
        assert_eq!(skip_first(&[1, 2, 3], 0), vec![1, 2, 3]);
    }

    #[test]
    fn test_skip_first_all() {
        assert_eq!(skip_first(&[1, 2, 3], 3), Vec::<i32>::new());
    }

    #[test]
    fn test_skip_first_more_than_length() {
        assert_eq!(skip_first(&[1, 2, 3], 10), Vec::<i32>::new());
    }

    #[test]
    fn test_skip_first_empty() {
        let empty: &[i32] = &[];
        assert_eq!(skip_first(empty, 5), Vec::<i32>::new());
    }

    #[test]
    fn test_skip_first_one() {
        assert_eq!(skip_first(&[1, 2, 3, 4, 5], 1), vec![2, 3, 4, 5]);
    }

    #[test]
    fn test_skip_first_strings() {
        assert_eq!(skip_first(&["a", "b", "c", "d"], 2), vec!["c", "d"]);
    }

    // ==================== reverse_sequence tests ====================

    #[test]
    fn test_reverse_sequence_basic() {
        assert_eq!(reverse_sequence(&[1, 2, 3]), vec![3, 2, 1]);
    }

    #[test]
    fn test_reverse_sequence_single() {
        assert_eq!(reverse_sequence(&[42]), vec![42]);
    }

    #[test]
    fn test_reverse_sequence_empty() {
        let empty: &[i32] = &[];
        assert_eq!(reverse_sequence(empty), Vec::<i32>::new());
    }

    #[test]
    fn test_reverse_sequence_two_elements() {
        assert_eq!(reverse_sequence(&[1, 2]), vec![2, 1]);
    }

    #[test]
    fn test_reverse_sequence_strings() {
        assert_eq!(reverse_sequence(&["a", "b", "c"]), vec!["c", "b", "a"]);
    }

    #[test]
    fn test_reverse_sequence_long() {
        assert_eq!(
            reverse_sequence(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
            vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
        );
    }

    // ==================== interleave tests ====================

    #[test]
    fn test_interleave_equal_length() {
        assert_eq!(interleave(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_interleave_first_longer() {
        assert_eq!(interleave(&[1, 3, 5, 7], &[2, 4]), vec![1, 2, 3, 4, 5, 7]);
    }

    #[test]
    fn test_interleave_second_longer() {
        assert_eq!(interleave(&[1, 3], &[2, 4, 6, 8]), vec![1, 2, 3, 4, 6, 8]);
    }

    #[test]
    fn test_interleave_empty_first() {
        let empty: &[i32] = &[];
        assert_eq!(interleave(empty, &[1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_interleave_empty_second() {
        let empty: &[i32] = &[];
        assert_eq!(interleave(&[1, 2, 3], empty), vec![1, 2, 3]);
    }

    #[test]
    fn test_interleave_both_empty() {
        let empty: &[i32] = &[];
        assert_eq!(interleave(empty, empty), Vec::<i32>::new());
    }

    #[test]
    fn test_interleave_single_elements() {
        assert_eq!(interleave(&[1], &[2]), vec![1, 2]);
    }

    #[test]
    fn test_interleave_strings() {
        assert_eq!(
            interleave(&["a", "c", "e"], &["b", "d", "f"]),
            vec!["a", "b", "c", "d", "e", "f"]
        );
    }

    // ==================== sliding_pairs tests ====================

    #[test]
    fn test_sliding_pairs_basic() {
        assert_eq!(sliding_pairs(&[1, 2, 3, 4]), vec![(1, 2), (2, 3), (3, 4)]);
    }

    #[test]
    fn test_sliding_pairs_two_elements() {
        assert_eq!(sliding_pairs(&[1, 2]), vec![(1, 2)]);
    }

    #[test]
    fn test_sliding_pairs_single_element() {
        assert_eq!(sliding_pairs(&[1]), Vec::<(i32, i32)>::new());
    }

    #[test]
    fn test_sliding_pairs_empty() {
        let empty: &[i32] = &[];
        assert_eq!(sliding_pairs(empty), Vec::<(i32, i32)>::new());
    }

    #[test]
    fn test_sliding_pairs_five_elements() {
        assert_eq!(
            sliding_pairs(&[1, 2, 3, 4, 5]),
            vec![(1, 2), (2, 3), (3, 4), (4, 5)]
        );
    }

    #[test]
    fn test_sliding_pairs_strings() {
        assert_eq!(
            sliding_pairs(&["a", "b", "c"]),
            vec![("a", "b"), ("b", "c")]
        );
    }

    #[test]
    fn test_sliding_pairs_duplicates() {
        assert_eq!(sliding_pairs(&[1, 1, 2, 2]), vec![(1, 1), (1, 2), (2, 2)]);
    }

    // ==================== Integration tests ====================

    #[test]
    fn test_chain_then_take() {
        // Combine chain and take
        let combined = chain_sequences(&[1, 2, 3], &[4, 5, 6]);
        let result = take_first(&combined, 4);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_chain_then_skip() {
        // Combine chain and skip
        let combined = chain_sequences(&[1, 2], &[3, 4, 5]);
        let result = skip_first(&combined, 2);
        assert_eq!(result, vec![3, 4, 5]);
    }

    #[test]
    fn test_reverse_then_take() {
        // Reverse and take first elements (which were last)
        let reversed = reverse_sequence(&[1, 2, 3, 4, 5]);
        let result = take_first(&reversed, 2);
        assert_eq!(result, vec![5, 4]);
    }

    #[test]
    fn test_interleave_then_reverse() {
        // Interleave and then reverse
        let interleaved = interleave(&[1, 3], &[2, 4]);
        let result = reverse_sequence(&interleaved);
        assert_eq!(result, vec![4, 3, 2, 1]);
    }

    #[test]
    fn test_zip_indices() {
        // Common pattern: zip with indices
        let items = ["a", "b", "c"];
        let indices: Vec<usize> = (0..items.len()).collect();
        let result = zip_pairs(&indices, &items);
        assert_eq!(result, vec![(0, "a"), (1, "b"), (2, "c")]);
    }

    #[test]
    fn test_sliding_pairs_differences() {
        // Use sliding pairs to compute differences
        let numbers = [10, 15, 12, 20, 18];
        let pairs = sliding_pairs(&numbers);
        let differences: Vec<i32> = pairs.iter().map(|(a, b)| b - a).collect();
        assert_eq!(differences, vec![5, -3, 8, -2]);
    }

    #[test]
    fn test_complex_pipeline() {
        // Chain -> skip -> take -> reverse
        let first = [1, 2, 3];
        let second = [4, 5, 6, 7, 8];
        let chained = chain_sequences(&first, &second);
        let skipped = skip_first(&chained, 2);
        let taken = take_first(&skipped, 4);
        let result = reverse_sequence(&taken);
        assert_eq!(result, vec![6, 5, 4, 3]);
    }
}
