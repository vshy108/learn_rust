//! # Rustfinity: Spawning Threads
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/spawning-threads
//!
//! A thread is a second line of execution inside the same process. In Rust you start one with `std::thread::spawn`, which takes a closure and immediately returns a `JoinHandle`. The closure starts running right away, in parallel with the code that spawned it.
//!
//! The handle is the only way back. Calling `join()` on it blocks the current thread until the spawned one finishes, and hands you whatever the closure returned. That return value is how work crosses the thread boundary: no channels, no shared memory, just a value coming back out of `join()`.
//!
//! Because the closure may outlive the function that created it, `spawn` requires the closure to own everything it touches. That is what the `move` keyword is for, and it is why `values` below is moved into the thread rather than borrowed.
//!
//! ## Your Task
//!
//! Implement two functions.
//!
//! ### Implement `sum_in_thread`
//!
//! ```rust
//! pub fn sum_in_thread(values: Vec<u64>) -> u64
//! ```
//!
//! Spawn a thread that adds up `values`, join it, and return the total.
//!
//! ```rust
//! assert_eq!(sum_in_thread(vec![1, 2, 3]), 6);
//! assert_eq!(sum_in_thread(vec![]), 0);
//! ```
//!
//! ### Implement `spawn_counter`
//!
//! ```rust
//! pub fn spawn_counter(
//!     start: u64,
//!     steps: u64,
//! ) -> JoinHandle<u64>
//! ```
//!
//! Spawn a thread that adds up `steps` consecutive numbers beginning at `start`, and return the handle **without** joining it. The caller decides when to block.
//!
//! ```rust
//! // 10 + 11 + 12
//! let h = spawn_counter(10, 3);
//! assert_eq!(h.join().unwrap(), 33);
//!
//! // no steps means nothing to add
//! assert_eq!(spawn_counter(7, 0).join().unwrap(), 0);
//! ```
//!
//! ### Notes
//!
//! - `join()` returns a `Result`. Panics inside a thread are a topic of their own, so `unwrap()` is fine here.
//! - Returning the handle is the interesting part of the second function. It lets the caller start work now and collect it later.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - `thread::spawn(move || { ... })` returns a `JoinHandle<T>`, where `T` is the closure's return type.
//! - The last expression of the closure is its return value, exactly like a function body.
//! - For the counter, `(0..steps).map(|i| start + i).sum()` avoids a manual loop.
//! - If the compiler complains that a captured variable does not live long enough, you are missing `move`.
//!
//! </details>
//!
//! Run: `cargo run --example rf_spawning_threads`
//! Test: `cargo test --example rf_spawning_threads`
//! Filter: `cargo test --example rf_spawning_threads <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::thread::{self, JoinHandle};

/// Sums a list of numbers on a separate thread and returns the total.
pub fn sum_in_thread(values: Vec<u64>) -> u64 {
    // TODO: spawn a thread that sums `values`, then join it
    unimplemented!()
}

/// Starts a thread that counts and returns the handle immediately.
///
/// The thread adds up `start + (start + 1) + ... + (start + steps - 1)`.
pub fn spawn_counter(start: u64, steps: u64) -> JoinHandle<u64> {
    // TODO: spawn the thread and return its handle without joining
    unimplemented!()
}

// Example usage
pub fn main() {
    println!("sum: {}", sum_in_thread(vec![1, 2, 3, 4]));

    let handle = spawn_counter(10, 3);
    println!("counter: {}", handle.join().unwrap());
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    mod sum_in_thread_tests {
        use super::*;

        #[test]
        fn sums_a_short_list() {
            assert_eq!(sum_in_thread(vec![1, 2, 3]), 6);
        }

        #[test]
        fn sums_an_empty_list() {
            assert_eq!(sum_in_thread(vec![]), 0);
        }

        #[test]
        fn sums_a_single_element() {
            assert_eq!(sum_in_thread(vec![42]), 42);
        }

        #[test]
        fn sums_a_longer_list() {
            let values: Vec<u64> = (1..=100).collect();
            assert_eq!(sum_in_thread(values), 5050);
        }

        #[test]
        fn sums_large_values_without_overflow() {
            let big = u32::MAX as u64;
            assert_eq!(sum_in_thread(vec![big, big]), big * 2);
        }
    }

    mod spawn_counter_tests {
        use super::*;

        #[test]
        fn counts_three_steps() {
            assert_eq!(spawn_counter(10, 3).join().unwrap(), 33);
        }

        #[test]
        fn zero_steps_is_zero() {
            assert_eq!(spawn_counter(7, 0).join().unwrap(), 0);
        }

        #[test]
        fn one_step_is_the_start_value() {
            assert_eq!(spawn_counter(5, 1).join().unwrap(), 5);
        }

        #[test]
        fn counts_from_zero() {
            assert_eq!(spawn_counter(0, 5).join().unwrap(), 10);
        }

        #[test]
        fn several_handles_can_run_at_once() {
            let handles: Vec<_> = (0..4).map(|i| spawn_counter(i, 4)).collect();

            let results: Vec<u64> = handles.into_iter().map(|h| h.join().unwrap()).collect();

            assert_eq!(results, vec![6, 10, 14, 18]);
        }

        #[test]
        fn work_happens_before_join_is_called() {
            // The handle is created first and joined last, so the caller is
            // free to do other work in between.
            let handle = spawn_counter(100, 3);
            let local = 1 + 1;

            assert_eq!(local, 2);
            assert_eq!(handle.join().unwrap(), 303);
        }
    }
}
