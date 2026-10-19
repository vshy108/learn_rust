//! # Rustfinity: Mpsc Fan In
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/mpsc-fan-in
//!
//! An `mpsc` channel is *multi producer, single consumer*. The `Sender` can be cloned as many times as you like; the `Receiver` cannot. That makes it the natural fit for fan-in: many workers producing, one thread collecting.
//!
//! ```rust
//! let (tx, rx) = mpsc::channel();
//!
//! for chunk in chunks {
//!     let tx = tx.clone();
//!     thread::spawn(move || {
//!         for value in chunk {
//!             tx.send(value).unwrap();
//!         }
//!     });
//! }
//! ```
//!
//! Now the part that catches everybody. Iterating a receiver:
//!
//! ```rust
//! for value in rx {
//!     // ...
//! }
//! ```
//!
//! ends when the channel is *disconnected*, which means when the last `Sender` has been dropped. The workers drop their clones as they finish, but the `tx` that `channel()` handed you is still alive in this function, so the receiver keeps waiting for a message that will never come. The loop hangs.
//!
//! The fix is one line:
//!
//! ```rust
//! drop(tx);
//! ```
//!
//! Drop your own sender once every clone has been handed out. Then the last worker to finish disconnects the channel and the loop ends on its own.
//!
//! The other thing fan-in costs you is identity. Every message lands in one stream, so "which worker sent this" is gone unless you put it in the message:
//!
//! ```rust
//! tx.send((index, value)).unwrap();
//! ```
//!
//! Arrival order is not something you control either. A worker with less to do finishes first regardless of when it started, so any result that has to be ordered must be sorted or indexed after the fact, never assumed.
//!
//! ## Your Task
//!
//! Implement two functions.
//!
//! ### Implement `collect_all`
//!
//! ```rust
//! pub fn collect_all(chunks: Vec<Vec<u64>>) -> Vec<u64>
//! ```
//!
//! Give each chunk its own thread, send every value through a single shared channel, and return everything that arrived in ascending order.
//!
//! ```rust
//! let chunks = vec![vec![3, 1], vec![2], vec![]];
//! assert_eq!(collect_all(chunks), vec![1, 2, 3]);
//! ```
//!
//! Duplicates are kept. An empty input, or input of only empty chunks, gives an empty vector.
//!
//! ### Implement `sum_by_worker`
//!
//! ```rust
//! pub fn sum_by_worker(chunks: Vec<Vec<u64>>) -> Vec<u64>
//! ```
//!
//! The same fan-in, but the result has one entry per chunk holding that chunk's sum, in input order. Empty chunks sum to 0.
//!
//! ```rust
//! let chunks = vec![vec![1, 2, 3], vec![], vec![10]];
//! assert_eq!(sum_by_worker(chunks), vec![6, 0, 10]);
//! ```
//!
//! Messages arrive in an order you do not choose, so the worker index has to travel with the value.
//!
//! ### Notes
//!
//! - `thread::scope` lets a worker borrow without an `Arc`, but the chunk itself should be moved in.
//! - `rx.iter()` and `for value in rx` both stop at disconnection. Neither will stop if you still hold a sender.
//! - `send` returns a `Result`, which is only an error if the receiver is gone.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - The order is: create the channel, spawn all workers with `tx.clone()`, `drop(tx)`, *then* drain the receiver.
//! - If a test hangs instead of failing, you forgot the `drop`.
//! - `collect_all` can `let mut out: Vec<u64> = rx.iter().collect();` and then `out.sort_unstable()`.
//! - For `sum_by_worker`, allocate `vec![0; chunks.len()]` first, then `for (index, value) in rx { sums[index] += value; }`.
//! - `chunks.into_iter().enumerate()` gives you the index and the owned chunk together.
//!
//! </details>
//!
//! Run: `cargo run --example rf_mpsc_fan_in`
//! Test: `cargo test --example rf_mpsc_fan_in`
//! Filter: `cargo test --example rf_mpsc_fan_in <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::sync::mpsc;
use std::thread;

/// Runs one thread per chunk and gathers every value through a single
/// channel, returned in ascending order.
pub fn collect_all(chunks: Vec<Vec<u64>>) -> Vec<u64> {
    // TODO: clone a sender into every worker, drop the original, then
    // drain the receiver
    unimplemented!()
}

/// Sums each chunk, keeping the results lined up with the input.
pub fn sum_by_worker(chunks: Vec<Vec<u64>>) -> Vec<u64> {
    // TODO: tag every message with the worker index so the consumer can
    // put it back in the right slot
    unimplemented!()
}

// Example usage
pub fn main() {
    let chunks = vec![vec![3, 1], vec![2], vec![]];

    println!("{:?}", collect_all(chunks.clone()));
    println!("{:?}", sum_by_worker(chunks));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    mod collect_all_tests {
        use super::*;

        #[test]
        fn no_chunks_gives_nothing() {
            assert_eq!(collect_all(vec![]), Vec::<u64>::new());
        }

        #[test]
        fn only_empty_chunks_gives_nothing() {
            assert_eq!(collect_all(vec![vec![], vec![], vec![]]), Vec::<u64>::new());
        }

        #[test]
        fn a_single_chunk_comes_back_sorted() {
            assert_eq!(collect_all(vec![vec![5, 1, 3]]), vec![1, 3, 5]);
        }

        #[test]
        fn values_from_every_worker_arrive() {
            let chunks = vec![vec![3, 1], vec![2], vec![], vec![4, 6, 5]];

            assert_eq!(collect_all(chunks), vec![1, 2, 3, 4, 5, 6]);
        }

        #[test]
        fn duplicates_are_kept() {
            let chunks = vec![vec![7, 7], vec![7], vec![7]];

            assert_eq!(collect_all(chunks), vec![7, 7, 7, 7]);
        }

        #[test]
        fn nothing_is_lost_under_load() {
            let chunks: Vec<Vec<u64>> = (0..8)
                .map(|worker: u64| ((worker * 50)..(worker * 50 + 50)).collect())
                .collect();

            let collected = collect_all(chunks);

            assert_eq!(collected.len(), 400);
            assert_eq!(collected, (0..400u64).collect::<Vec<_>>());
        }
    }

    mod sum_by_worker_tests {
        use super::*;

        #[test]
        fn no_chunks_gives_no_sums() {
            assert_eq!(sum_by_worker(vec![]), Vec::<u64>::new());
        }

        #[test]
        fn one_sum_per_chunk_including_empty_ones() {
            let chunks = vec![vec![1, 2, 3], vec![], vec![10]];

            assert_eq!(sum_by_worker(chunks), vec![6, 0, 10]);
        }

        #[test]
        fn sums_stay_in_input_order() {
            let chunks = vec![vec![100], vec![1], vec![10], vec![1000]];

            assert_eq!(sum_by_worker(chunks), vec![100, 1, 10, 1000]);
        }

        #[test]
        fn uneven_chunk_sizes_do_not_shuffle_the_result() {
            // Descending sizes: a solution that assumes messages arrive in
            // worker order, or that indexes by arrival, gets this wrong.
            let chunks: Vec<Vec<u64>> = (0..6)
                .map(|worker: u64| vec![1; (6 - worker) as usize * 20])
                .collect();

            let expected: Vec<u64> = (0..6).map(|worker: u64| (6 - worker) * 20).collect();

            assert_eq!(sum_by_worker(chunks), expected);
        }

        #[test]
        fn totals_match_a_sequential_fold() {
            let chunks: Vec<Vec<u64>> = (0..8)
                .map(|worker: u64| ((worker * 30)..(worker * 30 + 30)).collect())
                .collect();

            let expected: Vec<u64> = chunks.iter().map(|chunk| chunk.iter().sum()).collect();

            assert_eq!(sum_by_worker(chunks), expected);
        }
    }
}
