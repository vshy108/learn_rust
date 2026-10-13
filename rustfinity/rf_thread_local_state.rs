//! # Rustfinity: Thread Local State
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/thread-local-state
//!
//! Sharing state between threads costs something. An `Arc<Mutex<T>>` means an allocation, an atomic refcount, and a lock on every access. Sometimes that is exactly right. Often the threads never actually needed to see each other's data, and the whole apparatus is paid for nothing.
//!
//! `thread_local!` gives each thread its own private copy of a static:
//!
//! ```rust
//! thread_local! {
//!     static CALLS: Cell<u64> = const { Cell::new(0) };
//! }
//! ```
//!
//! Every thread that touches `CALLS` gets a fresh `Cell` initialised on first use, dropped when that thread ends. There is no sharing, so there is nothing to lock. It is a static in the sense that it lives at the top level and outlives every function call, but it is not one value: there are as many as there are threads.
//!
//! You never get a reference out. Access goes through `with`, which hands your closure a shared reference for the length of the call:
//!
//! ```rust
//! CALLS.with(|calls| calls.set(calls.get() + 1));
//! ```
//!
//! The reference cannot escape, which is precisely what makes it safe.
//!
//! Because the reference is shared, the value has to provide its own interior mutability. `Cell<T>` is the cheap option for `Copy` types. For anything else, `RefCell<T>` does the borrow check at runtime, within the one thread that owns it:
//!
//! ```rust
//! thread_local! {
//!     static LOG: RefCell<Vec<String>> = const {
//!         RefCell::new(Vec::new())
//!     };
//! }
//!
//! LOG.with_borrow_mut(|log| log.push("event".into()));
//! LOG.with_borrow(|log| log.len());
//! ```
//!
//! `with_borrow` and `with_borrow_mut` are shorthand for `with` plus `borrow`. The `const { ... }` initialiser is an optimisation: when the value can be built in a const context, the compiler skips the lazy initialisation check on every access.
//!
//! ## Your Task
//!
//! Keep a per thread counter and a per thread log, then prove they do not leak between threads.
//!
//! ### Implement `bump`
//!
//! ```rust
//! pub fn bump() -> u64
//! ```
//!
//! Increment the calling thread's counter and return the new value. A thread that has never called it starts from `0`, so the first call returns `1`.
//!
//! ### Implement `record` and `events`
//!
//! ```rust
//! pub fn record(event: &str)
//! pub fn events() -> Vec<String>
//! ```
//!
//! `record` appends to the calling thread's log. `events` returns a copy of it, in the order things were recorded, and leaves the log alone.
//!
//! ```rust
//! record("open");
//! record("close");
//! assert_eq!(events(), vec!["open", "close"]);
//! ```
//!
//! ### Implement `counts_per_thread`
//!
//! ```rust
//! pub fn counts_per_thread(bumps: &[u64]) -> Vec<u64>
//! ```
//!
//! Spawn one thread per entry, have it call `bump` that many times, and return each thread's final count in input order.
//!
//! ```rust
//! assert_eq!(counts_per_thread(&[3, 1, 2]), vec![3, 1, 2]);
//! ```
//!
//! The result being identical to the input is the lesson, not a coincidence. Each worker counts alone.
//!
//! ### Notes
//!
//! - The counter and the log are separate pieces of state and do not interfere.
//! - `events` has to clone. The borrow only lives as long as the closure.
//! - A worker calling `bump` must not change the count on the thread that spawned it.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - Two entries in one `thread_local!` block is fine, separated by semicolons.
//! - `Cell::get` and `Cell::set` are all `bump` needs. There is no `+=` on a `Cell`.
//! - `LOG.with_borrow(|log| log.clone())` is the whole of `events`.
//! - Collect the `JoinHandle`s first and join them afterwards. Joining inside the spawn loop runs the workers one at a time.
//! - `thread::scope` lets the closures borrow from `bumps` without any cloning.
//!
//! </details>
//!
//! Run: `cargo run --example rf_thread_local_state`
//! Test: `cargo test --example rf_thread_local_state`
//! Filter: `cargo test --example rf_thread_local_state <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::cell::{Cell, RefCell};
use std::thread;

thread_local! {
    // TODO: one Cell<u64> counter and one RefCell<Vec<String>> log,
    // both per thread
}

/// Increments the calling thread's counter and returns the new value.
pub fn bump() -> u64 {
    // TODO
    unimplemented!()
}

/// Appends an event to the calling thread's log.
pub fn record(event: &str) {
    // TODO
    unimplemented!()
}

/// Returns a copy of the calling thread's log.
pub fn events() -> Vec<String> {
    // TODO
    unimplemented!()
}

/// Runs one thread per entry, bumping the counter that many times, and
/// returns each thread's final count.
pub fn counts_per_thread(bumps: &[u64]) -> Vec<u64> {
    // TODO: spawn a thread per entry, and remember that each one starts
    // its own counter at 0
    unimplemented!()
}

// Example usage
pub fn main() {
    println!("{:?}", counts_per_thread(&[3, 1, 2]));

    record("hello");
    println!("{:?}", events());
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::thread;
    // Every exact assertion runs inside a thread this test spawned, so it
    // never depends on what the harness thread did earlier.
    fn in_fresh_thread<T, F>(body: F) -> T
    where
        T: Send + 'static,
        F: FnOnce() -> T + Send + 'static,
    {
        thread::spawn(body).join().unwrap()
    }

    mod bump_tests {
        use super::*;

        #[test]
        fn a_fresh_thread_starts_at_zero() {
            assert_eq!(in_fresh_thread(bump), 1);
        }

        #[test]
        fn it_returns_the_value_after_incrementing() {
            let counts = in_fresh_thread(|| vec![bump(), bump(), bump()]);

            assert_eq!(counts, vec![1, 2, 3]);
        }

        #[test]
        fn each_call_moves_the_count_by_exactly_one() {
            let first = bump();
            let second = bump();

            assert_eq!(second, first + 1);
        }

        #[test]
        fn one_thread_never_sees_another_thread_count() {
            let a = in_fresh_thread(|| {
                for _ in 0..49 {
                    bump();
                }
                bump()
            });
            let b = in_fresh_thread(bump);

            assert_eq!(a, 50);
            assert_eq!(b, 1, "a new thread must not inherit an old thread's count");
        }
    }

    mod log_tests {
        use super::*;

        #[test]
        fn a_fresh_thread_has_an_empty_log() {
            assert_eq!(in_fresh_thread(events), Vec::<String>::new());
        }

        #[test]
        fn events_come_back_in_the_order_they_were_recorded() {
            let log = in_fresh_thread(|| {
                record("open");
                record("read");
                record("close");
                events()
            });

            assert_eq!(log, vec!["open", "read", "close"]);
        }

        #[test]
        fn duplicates_are_kept() {
            let log = in_fresh_thread(|| {
                record("tick");
                record("tick");
                events()
            });

            assert_eq!(log, vec!["tick", "tick"]);
        }

        #[test]
        fn reading_the_log_does_not_empty_it() {
            let (first, second) = in_fresh_thread(|| {
                record("once");
                (events(), events())
            });

            assert_eq!(first, vec!["once"]);
            assert_eq!(second, vec!["once"]);
        }

        #[test]
        fn logs_do_not_leak_between_threads() {
            in_fresh_thread(|| record("private"));

            assert_eq!(in_fresh_thread(events), Vec::<String>::new());
        }

        #[test]
        fn the_counter_and_the_log_are_independent() {
            let (count, log) = in_fresh_thread(|| {
                record("a");
                let count = bump();
                record("b");
                (count, events())
            });

            assert_eq!(count, 1);
            assert_eq!(log, vec!["a", "b"]);
        }
    }

    mod counts_per_thread_tests {
        use super::*;

        #[test]
        fn no_workers_gives_no_counts() {
            assert_eq!(counts_per_thread(&[]), Vec::<u64>::new());
        }

        #[test]
        fn each_worker_reports_its_own_total() {
            assert_eq!(counts_per_thread(&[3, 1, 2]), vec![3, 1, 2]);
        }

        #[test]
        fn a_worker_with_no_bumps_reports_zero() {
            assert_eq!(counts_per_thread(&[0, 2, 0]), vec![0, 2, 0]);
        }

        #[test]
        fn results_stay_in_input_order() {
            // Descending work: a solution that collects by completion order
            // rather than by index gets this wrong.
            let bumps: Vec<u64> = (0..8).map(|worker| (8 - worker) * 40).collect();
            let expected = bumps.clone();

            assert_eq!(counts_per_thread(&bumps), expected);
        }

        #[test]
        fn nothing_is_shared_between_the_workers() {
            let counts = counts_per_thread(&[100; 12]);

            assert_eq!(
                counts,
                vec![100; 12],
                "shared state would make these totals overlap"
            );
        }

        #[test]
        fn the_calling_thread_is_unaffected_by_its_workers() {
            let before = bump();
            counts_per_thread(&[500, 500]);
            let after = bump();

            assert_eq!(after, before + 1);
        }
    }
}
