//! # Rustfinity: Scoped Threads
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/scoped-threads
//!
//! `thread::spawn` demands `'static`, which means a spawned thread can never borrow a local variable. The usual workaround is to clone the data or wrap it in an `Arc`, and for a short parallel scan over a slice you already own that feels like a lot of ceremony.
//!
//! `std::thread::scope` removes the need for it. You hand it a closure, and inside that closure you spawn threads with `s.spawn(...)`. The scope will not return until every thread it spawned has been joined. Since the compiler knows that, it can prove the borrows end before the borrowed data does, and ordinary references are allowed straight through.
//!
//! That also means mutable borrows work, as long as they follow the usual rules. Two scoped threads can each hold a `&mut` to a **different** local at the same time.
//!
//! ## Your Task
//!
//! Implement two functions. Both must use `thread::scope`, and neither may clone the input or reach for `Arc`.
//!
//! ### Implement `parallel_max`
//!
//! ```rust
//! pub fn parallel_max(data: &[i32]) -> Option<i32>
//! ```
//!
//! Split `data` in half, find the maximum of each half concurrently, and return the larger. An empty slice has no maximum.
//!
//! ```rust
//! assert_eq!(parallel_max(&[3, 9, 1, 4]), Some(9));
//! assert_eq!(parallel_max(&[]), None);
//! ```
//!
//! ### Implement `split_evens_odds`
//!
//! ```rust
//! pub fn split_evens_odds(
//!     data: &[i32],
//! ) -> (Vec<i32>, Vec<i32>)
//! ```
//!
//! Use one scoped thread to collect the even values and another to collect the odd ones, then return `(evens, odds)`. Each vector keeps the relative order of the input.
//!
//! ```rust
//! let (e, o) = split_evens_odds(&[1, 2, 3, 4]);
//! assert_eq!(e, vec![2, 4]);
//! assert_eq!(o, vec![1, 3]);
//! ```
//!
//! ### Notes
//!
//! - 0 is even, and negative numbers follow the same rule: `-2` is even, `-1` is odd.
//! - The scope joins for you when the closure ends, so you only call `join()` when you actually want a returned value back.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - `slice::split_at(mid)` gives you two subslices without copying anything.
//! - `thread::scope(|s| { ... })` returns whatever its closure returns, so you can compute the answer inside and let it fall out.
//! - `s.spawn(...)` gives a scoped handle. Call `join()` on it when you need the value, or ignore it and let the scope clean up.
//! - Combining two `Option<i32>` values: match on the pair, and `a.or(b)` covers the cases where at most one is `Some`.
//! - For the split, borrow `evens` in one closure and `odds` in the other. Borrowing the same vector in both will not compile, and that is the point.
//!
//! </details>
//!
//! Run: `cargo run --example rf_scoped_threads`
//! Test: `cargo test --example rf_scoped_threads`
//! Filter: `cargo test --example rf_scoped_threads <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::thread;

/// Finds the largest value by scanning both halves in parallel.
pub fn parallel_max(data: &[i32]) -> Option<i32> {
    // TODO: split `data` in half and compare both halves inside a
    // thread::scope, without cloning or using Arc
    unimplemented!()
}

/// Splits values into evens and odds using two scoped threads.
pub fn split_evens_odds(data: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut evens = Vec::new();
    let mut odds = Vec::new();

    // TODO: fill both vectors from inside a thread::scope

    (evens, odds)
}

// Example usage
pub fn main() {
    println!("{:?}", parallel_max(&[3, 9, 1, 4]));
    println!("{:?}", split_evens_odds(&[1, 2, 3, 4]));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    mod parallel_max_tests {
        use super::*;

        #[test]
        fn finds_the_largest() {
            assert_eq!(parallel_max(&[3, 9, 1, 4]), Some(9));
        }

        #[test]
        fn empty_slice_is_none() {
            assert_eq!(parallel_max(&[]), None);
        }

        #[test]
        fn single_element() {
            assert_eq!(parallel_max(&[7]), Some(7));
        }

        #[test]
        fn largest_in_the_first_half() {
            assert_eq!(parallel_max(&[100, 2, 3, 4]), Some(100));
        }

        #[test]
        fn largest_in_the_second_half() {
            assert_eq!(parallel_max(&[1, 2, 3, 100]), Some(100));
        }

        #[test]
        fn handles_negatives() {
            assert_eq!(parallel_max(&[-9, -3, -20, -1]), Some(-1));
        }

        #[test]
        fn odd_length_slice() {
            assert_eq!(parallel_max(&[5, 8, 2]), Some(8));
        }

        #[test]
        fn borrows_a_local_without_cloning_it() {
            let data: Vec<i32> = (0..500).collect();
            assert_eq!(parallel_max(&data), Some(499));
        }
    }

    mod split_evens_odds_tests {
        use super::*;

        #[test]
        fn splits_a_short_slice() {
            let (evens, odds) = split_evens_odds(&[1, 2, 3, 4]);
            assert_eq!(evens, vec![2, 4]);
            assert_eq!(odds, vec![1, 3]);
        }

        #[test]
        fn empty_slice_gives_two_empty_vectors() {
            let (evens, odds) = split_evens_odds(&[]);
            assert!(evens.is_empty());
            assert!(odds.is_empty());
        }

        #[test]
        fn all_even() {
            let (evens, odds) = split_evens_odds(&[2, 4, 6]);
            assert_eq!(evens, vec![2, 4, 6]);
            assert!(odds.is_empty());
        }

        #[test]
        fn all_odd() {
            let (evens, odds) = split_evens_odds(&[1, 3, 5]);
            assert!(evens.is_empty());
            assert_eq!(odds, vec![1, 3, 5]);
        }

        #[test]
        fn keeps_relative_order() {
            let (evens, odds) = split_evens_odds(&[9, 8, 7, 6, 5, 4]);
            assert_eq!(evens, vec![8, 6, 4]);
            assert_eq!(odds, vec![9, 7, 5]);
        }

        #[test]
        fn negative_numbers_land_correctly() {
            let (evens, odds) = split_evens_odds(&[-2, -1, 0, 1]);
            assert_eq!(evens, vec![-2, 0]);
            assert_eq!(odds, vec![-1, 1]);
        }

        #[test]
        fn every_value_appears_exactly_once() {
            let data: Vec<i32> = (0..200).collect();
            let (evens, odds) = split_evens_odds(&data);

            assert_eq!(evens.len() + odds.len(), 200);
            assert_eq!(evens.len(), 100);
        }
    }
}
