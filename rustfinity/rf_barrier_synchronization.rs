//! # Rustfinity: Barrier Synchronization
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/barrier-synchronization
//!
//! Some parallel algorithms run in rounds. Every worker does its part of round 1, and only when *all* of them are finished may anyone start round 2. Cellular automata, iterative solvers, and simulation ticks all have this shape.
//!
//! `std::sync::Barrier` is the tool. You build it with a number of parties, and every thread that calls `wait()` blocks until that many threads have arrived. Then all of them are released together.
//!
//! ```rust
//! let barrier = Barrier::new(4);
//!
//! barrier.wait(); // blocks until 4 threads are here
//! ```
//!
//! The subtle part is that one barrier per round is usually not enough. Consider workers that each read their neighbour's cell and then overwrite their own:
//!
//! ```rust
//! let mine = cells[i].load(Relaxed);
//! let right = cells[i + 1].load(Relaxed);
//! cells[i].store(mine + right, Relaxed);
//! barrier.wait();
//! ```
//!
//! This is broken. Worker 0 can finish its store before worker 1 has done its loads, so worker 1 reads a round-2 value while computing round 2. The fix is to separate the phases with a barrier of their own:
//!
//! ```rust
//! // read phase
//! barrier.wait();
//! // write phase
//! barrier.wait();
//! ```
//!
//! Two waits per round: one that says "everyone has read", one that says "everyone has written".
//!
//! `wait()` also returns a `BarrierWaitResult`. Exactly one of the parties gets `is_leader() == true`, which is a cheap way to run a single piece of per-round bookkeeping without another lock.
//!
//! ## Your Task
//!
//! Implement two functions.
//!
//! ### Implement `run_rounds`
//!
//! ```rust
//! pub fn run_rounds(
//!     cells: Vec<i64>,
//!     rounds: usize,
//! ) -> Vec<i64>
//! ```
//!
//! Give every cell its own thread. In each round, a cell becomes itself plus its right neighbour, wrapping around at the end:
//!
//! ```
//! [1, 2, 3] -> [3, 5, 4] -> [8, 9, 7]
//! ```
//!
//! Every cell must read the *old* values of the round, so the read phase and the write phase have to be separated by a barrier. Return the cells after `rounds` rounds. An empty input, or zero rounds, returns the input unchanged.
//!
//! ### Implement `count_leaders`
//!
//! ```rust
//! pub fn count_leaders(parties: usize, rounds: usize) -> usize
//! ```
//!
//! Run `parties` threads through `rounds` barriers and count how many times a thread was told it was the leader. With at least one party the answer is always `rounds`; with zero parties it is 0.
//!
//! ### Notes
//!
//! - `Barrier::new(n)` releases threads in groups of `n`, and can be reused round after round.
//! - A barrier with fewer arrivals than parties blocks forever, so the party count and the thread count must match exactly.
//! - `thread::scope` lets the threads borrow the shared cells without an `Arc`.
//! - `AtomicI64` gives you shared mutable cells that several threads may touch without a lock.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - Build the cells as `Vec<AtomicI64>` and the barrier as `Barrier::new(cells.len())`, both outside the scope.
//! - The worker body is: load mine, load `(index + 1) % len`, `barrier.wait()`, store the sum, `barrier.wait()`.
//! - Return early for an empty input. `Barrier::new(0)` is not something you want to reason about.
//! - `AtomicI64::into_inner` turns the cells back into plain values once the threads are done.
//! - For `count_leaders`, `if barrier.wait().is_leader() { counter.fetch_add(1, Ordering::Relaxed); }`.
//!
//! </details>
//!
//! Run: `cargo run --example rf_barrier_synchronization`
//! Test: `cargo test --example rf_barrier_synchronization`
//! Filter: `cargo test --example rf_barrier_synchronization <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::sync::Barrier;
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::thread;

/// Runs one "read everybody, then write everybody" step per round.
///
/// Every cell is replaced by itself plus its right neighbour, wrapping at
/// the end. One thread owns one cell for the whole run.
pub fn run_rounds(cells: Vec<i64>, rounds: usize) -> Vec<i64> {
    // TODO: give every cell a thread, and use two barrier waits per round
    // so that no thread writes while another is still reading
    unimplemented!()
}

/// Counts how many threads were told they were the leader of a barrier.
///
/// Runs `parties` threads through `rounds` barriers.
pub fn count_leaders(parties: usize, rounds: usize) -> usize {
    // TODO: count the `wait()` results whose `is_leader()` is true
    unimplemented!()
}

// Example usage
pub fn main() {
    println!("{:?}", run_rounds(vec![1, 2, 3], 2));
    println!("{}", count_leaders(4, 3));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    /// Reference implementation of one round, done sequentially.
    fn step(cells: &[i64]) -> Vec<i64> {
        (0..cells.len())
            .map(|i| cells[i] + cells[(i + 1) % cells.len()])
            .collect()
    }

    mod run_rounds_tests {
        use super::*;

        #[test]
        fn zero_rounds_returns_the_input() {
            assert_eq!(run_rounds(vec![1, 2, 3], 0), vec![1, 2, 3]);
        }

        #[test]
        fn empty_input_stays_empty() {
            assert_eq!(run_rounds(vec![], 4), Vec::<i64>::new());
        }

        #[test]
        fn one_cell_doubles_every_round() {
            assert_eq!(run_rounds(vec![5], 3), vec![40]);
        }

        #[test]
        fn one_round_adds_the_right_neighbour() {
            assert_eq!(run_rounds(vec![1, 2, 3], 1), vec![3, 5, 4]);
        }

        #[test]
        fn two_rounds_build_on_the_first() {
            assert_eq!(run_rounds(vec![1, 2, 3], 2), vec![8, 9, 7]);
        }

        #[test]
        fn matches_the_sequential_reference() {
            let start = vec![3, -1, 4, 1, -5, 9, 2, 6];

            for rounds in 0..6 {
                let mut expected = start.clone();
                for _ in 0..rounds {
                    expected = step(&expected);
                }

                assert_eq!(
                    run_rounds(start.clone(), rounds),
                    expected,
                    "mismatch after {rounds} rounds"
                );
            }
        }

        #[test]
        fn phases_are_not_allowed_to_overlap() {
            // Without a barrier between the read and the write phase, a fast
            // thread's fresh value leaks into a slow thread's input and the
            // totals drift. Every round exactly doubles the sum, so a single
            // leak is visible here.
            let start: Vec<i64> = (1..=12).collect();
            let sum: i64 = start.iter().sum();

            let result = run_rounds(start, 5);

            assert_eq!(result.iter().sum::<i64>(), sum * 32);
        }
    }

    mod count_leaders_tests {
        use super::*;

        #[test]
        fn one_leader_per_round() {
            assert_eq!(count_leaders(4, 3), 3);
        }

        #[test]
        fn a_single_party_leads_every_time() {
            assert_eq!(count_leaders(1, 7), 7);
        }

        #[test]
        fn no_parties_means_no_leaders() {
            assert_eq!(count_leaders(0, 5), 0);
        }

        #[test]
        fn no_rounds_means_no_leaders() {
            assert_eq!(count_leaders(6, 0), 0);
        }

        #[test]
        fn holds_for_many_parties_and_rounds() {
            assert_eq!(count_leaders(8, 20), 20);
        }
    }
}
