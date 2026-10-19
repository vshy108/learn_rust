//! # Rustfinity: Collecting Thread Results
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/collecting-thread-results
//!
//! Spawning a single thread is easy. Spawning several and getting their answers back in a predictable order is where people usually slip.
//!
//! The pattern is two passes. First spawn every thread and collect the handles into a `Vec<JoinHandle<T>>`. Only then join them, one after another, in that same order. Because the vector remembers which handle came from which input, the results line up with the input regardless of which thread actually finished first.
//!
//! The order of the two passes matters a lot. If you join inside the loop that spawns, each thread is finished before the next one starts and you have written a sequential loop with extra steps. Spawn first, join second, and the work really does overlap.
//!
//! ## Your Task
//!
//! Implement two functions.
//!
//! ### Implement `sum_chunks`
//!
//! ```rust
//! pub fn sum_chunks(chunks: Vec<Vec<u64>>) -> Vec<u64>
//! ```
//!
//! Give each chunk its own thread. Return one sum per chunk, in the same order the chunks arrived. An empty chunk sums to 0.
//!
//! ```rust
//! let chunks = vec![
//!     vec![1, 2],
//!     vec![10],
//!     vec![],
//! ];
//! assert_eq!(sum_chunks(chunks), vec![3, 10, 0]);
//! ```
//!
//! ### Implement `sum_all`
//!
//! ```rust
//! pub fn sum_all(chunks: Vec<Vec<u64>>) -> u64
//! ```
//!
//! Sum the chunks in parallel, then add the partial sums into one grand total.
//!
//! ```rust
//! let chunks = vec![vec![1, 2], vec![3]];
//! assert_eq!(sum_all(chunks), 6);
//! ```
//!
//! ### Notes
//!
//! - The output length always equals the input length. No chunk is skipped.
//! - The tests deliberately feed you chunks of very different sizes, so a solution that relies on threads finishing in spawn order will fail.
//! - Never assert on which thread finished first. Order comes from the handle vector.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - Build the handles with `.map(...).collect::<Vec<_>>()`, which forces every thread to start before you touch a single result.
//! - Then `handles.into_iter().map(|h| h.join().unwrap()).collect()`.
//! - `sum_all` can just call `sum_chunks` and sum the answer. Reusing it is better than writing the spawn loop twice.
//! - `chunk.iter().sum::<u64>()` needs the turbofish because the closure's return type is otherwise ambiguous.
//!
//! </details>
//!
//! Run: `cargo run --example rf_collecting_thread_results`
//! Test: `cargo test --example rf_collecting_thread_results`
//! Filter: `cargo test --example rf_collecting_thread_results <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::thread;

/// Sums each chunk on its own thread, keeping the input order.
pub fn sum_chunks(chunks: Vec<Vec<u64>>) -> Vec<u64> {
    // TODO: spawn every thread first, collect the handles, then join
    unimplemented!()
}

/// Sums every chunk in parallel and adds the partial sums together.
pub fn sum_all(chunks: Vec<Vec<u64>>) -> u64 {
    // TODO: reuse sum_chunks
    unimplemented!()
}

// Example usage
pub fn main() {
    let chunks = vec![vec![1, 2], vec![10], vec![]];

    println!("{:?}", sum_chunks(chunks.clone()));
    println!("{}", sum_all(chunks));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    mod sum_chunks_tests {
        use super::*;

        #[test]
        fn sums_three_chunks() {
            let chunks = vec![vec![1, 2], vec![10], vec![]];
            assert_eq!(sum_chunks(chunks), vec![3, 10, 0]);
        }

        #[test]
        fn no_chunks_gives_no_sums() {
            assert_eq!(sum_chunks(vec![]), Vec::<u64>::new());
        }

        #[test]
        fn one_result_per_chunk() {
            let chunks: Vec<Vec<u64>> = (0..8).map(|i| vec![i; 3]).collect();
            assert_eq!(sum_chunks(chunks).len(), 8);
        }

        #[test]
        fn order_follows_the_input_not_the_finish_time() {
            // Descending sizes: the first chunk is the slowest, yet it must
            // still come first in the output.
            let chunks: Vec<Vec<u64>> = (1..=6).rev().map(|n| vec![1; n * 200]).collect();

            let expected: Vec<u64> = (1..=6).rev().map(|n| (n * 200) as u64).collect();
            assert_eq!(sum_chunks(chunks), expected);
        }

        #[test]
        fn empty_chunks_are_kept_as_zero() {
            let chunks = vec![vec![], vec![5], vec![], vec![7]];
            assert_eq!(sum_chunks(chunks), vec![0, 5, 0, 7]);
        }

        #[test]
        fn handles_large_values() {
            let big = u32::MAX as u64;
            assert_eq!(sum_chunks(vec![vec![big, big]]), vec![big * 2]);
        }
    }

    mod sum_all_tests {
        use super::*;

        #[test]
        fn adds_the_partial_sums() {
            assert_eq!(sum_all(vec![vec![1, 2], vec![3]]), 6);
        }

        #[test]
        fn no_chunks_is_zero() {
            assert_eq!(sum_all(vec![]), 0);
        }

        #[test]
        fn only_empty_chunks_is_zero() {
            assert_eq!(sum_all(vec![vec![], vec![]]), 0);
        }

        #[test]
        fn matches_a_flat_sum() {
            let chunks: Vec<Vec<u64>> = (0..10)
                .map(|i| (0..10).map(|j| i * 10 + j).collect())
                .collect();

            let flat: u64 = (0..100).sum();
            assert_eq!(sum_all(chunks), flat);
        }
    }
}
