//! # Rustfinity: Send And Sync Bounds
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/send-and-sync-bounds
//!
//! `Send` and `Sync` are the two marker traits that make Rust's threading safe, and you mostly never mention them. They are auto-implemented for anything built out of parts that already have them, and the compiler checks them at every thread boundary.
//!
//! - **`Send`** means a value can be *moved* to another thread.
//! - **`Sync`** means `&T` can be shared with another thread. Equivalently, `T: Sync` exactly when `&T: Send`.
//!
//! `Rc<T>` is neither. Its reference count is a plain non-atomic integer, so two threads bumping it at once would corrupt it, and the compiler simply refuses:
//!
//! ```
//! `Rc<i32>` cannot be sent between threads safely
//! ```
//!
//! `Arc<T>` is the same type with an atomic count, so it is `Send + Sync` whenever `T` is. `Cell` and `RefCell` are `Send` but not `Sync`: moving one is fine, sharing one is not, because their borrow tracking is not atomic either.
//!
//! You start writing these bounds yourself the moment you write a generic helper that spawns:
//!
//! ```rust
//! pub fn run<T, F>(f: F) -> T
//! where
//!     F: FnOnce() -> T + Send + 'static,
//!     T: Send + 'static,
//! ```
//!
//! That is `thread::spawn`'s own signature. The `'static` is not "lives forever"; it is "contains no borrows of anything shorter", which is what `spawn` needs because the thread may outlive the caller. `thread::scope` is the escape hatch that drops it.
//!
//! The closure trait matters too:
//!
//! - `FnOnce` is consumed by the call. A closure that moves out a captured `String` is only `FnOnce`.
//! - `Fn` can be called repeatedly through `&self`, which is what you need if one closure is shared by many threads.
//!
//! And sharing a closure across threads is a `Sync` requirement, not a `Send` one: `Arc<F>` is only `Send` when `F: Send + Sync`.
//!
//! ## Your Task
//!
//! Write the bounds for two generic helpers. The bodies are ordinary; the `where` clauses are the exercise.
//!
//! ### Implement `parallel_map`
//!
//! ```rust
//! pub fn parallel_map<T, U, F>(items: Vec<T>, f: F) -> Vec<U>
//! ```
//!
//! Runs `f` on every item, each on its own thread, and returns the results in the same order as the input. One `f` is shared by all of the threads, so wrap it in an `Arc` and clone that per thread.
//!
//! ```rust
//! let doubled = parallel_map(vec![1, 2, 3], |n: i32| n * 2);
//! assert_eq!(doubled, vec![2, 4, 6]);
//! ```
//!
//! Your bounds must be tight enough to compile and loose enough to accept non-`Copy` items and results, such as `Vec<String>` in and `Vec<usize>` out.
//!
//! ### Implement `run_jobs`
//!
//! ```rust
//! pub fn run_jobs<T, F>(jobs: Vec<F>) -> Vec<T>
//! ```
//!
//! Runs each job on its own thread and returns the results in job order. Each job is called exactly once and is moved rather than shared, so its bounds are looser than `parallel_map`'s in one specific way. It must accept `Box<dyn FnOnce() -> T + Send>` and closures that consume what they captured.
//!
//! ### Notes
//!
//! - Do not add `Clone`, `Copy`, or `Debug`. Anything you require, a caller has to satisfy, and the tests call these with types that do not have them.
//! - Order comes from spawning everything first and joining afterwards, not from joining as you go.
//! - `handle.join()` returns a `Result`; unwrapping it is fine here.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - `parallel_map` needs `T: Send + 'static`, `U: Send + 'static`, and `F: Fn(T) -> U + Send + Sync + 'static`.
//! - The `Sync` on `F` is what `Arc<F>: Send` demands. Drop it and the error names `Arc<F>` rather than `F`.
//! - `run_jobs` needs `T: Send + 'static` and `F: FnOnce() -> T + Send + 'static`. No `Sync`, because each job is moved into exactly one thread.
//! - Collect the `JoinHandle`s into a `Vec` before joining any of them. `.map(...).map(|h| h.join())` in one chain joins each thread immediately and runs everything sequentially.
//! - `jobs.into_iter().map(thread::spawn)` is enough for the spawn half of `run_jobs`.
//!
//! </details>
//!
//! Run: `cargo run --example rf_send_and_sync_bounds`
//! Test: `cargo test --example rf_send_and_sync_bounds`
//! Filter: `cargo test --example rf_send_and_sync_bounds <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::sync::Arc;
use std::thread;

/// Applies `f` to every item on its own thread, preserving order.
pub fn parallel_map<T, U, F>(items: Vec<T>, f: F) -> Vec<U>
where
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> U + Send + Sync + 'static,
{
    unimplemented!()
}

/// Runs every job on its own thread and returns the results in order.
pub fn run_jobs<T, F>(jobs: Vec<F>) -> Vec<T>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    unimplemented!()
}

// Example usage
pub fn main() {
    let doubled: Vec<i32> = parallel_map(vec![1, 2, 3], |n: i32| n * 2);
    println!("{doubled:?}");

    let jobs: Vec<Box<dyn FnOnce() -> i32 + Send>> = vec![Box::new(|| 1 + 1), Box::new(|| 2 + 2)];
    let results: Vec<i32> = run_jobs(jobs);
    println!("{results:?}");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    mod parallel_map_tests {
        use super::*;

        #[test]
        fn empty_input_gives_empty_output() {
            assert_eq!(
                parallel_map(Vec::<u32>::new(), |n| n + 1),
                Vec::<u32>::new()
            );
        }

        #[test]
        fn maps_every_item() {
            assert_eq!(parallel_map(vec![1, 2, 3, 4], |n| n * 2), vec![2, 4, 6, 8]);
        }

        #[test]
        fn output_order_matches_input_order() {
            // Descending work per item, so a result order that depends on
            // completion time comes out backwards.
            let items: Vec<u64> = (0..12).collect();

            let result = parallel_map(items, |n| {
                let mut acc = 0u64;
                for step in 0..(12 - n) * 2_000 {
                    acc = acc.wrapping_add(step);
                }
                let _ = acc;
                n
            });

            assert_eq!(result, (0..12u64).collect::<Vec<_>>());
        }

        #[test]
        fn accepts_items_that_are_not_copy() {
            let items = vec![String::from("a"), String::from("bb"), String::from("ccc")];

            assert_eq!(parallel_map(items, |s| s.len()), vec![1, 2, 3]);
        }

        #[test]
        fn produces_results_that_are_not_copy() {
            let result = parallel_map(vec![1u32, 2], |n| n.to_string());

            assert_eq!(result, vec!["1", "2"]);
        }

        #[test]
        fn the_same_closure_is_shared_by_every_thread() {
            let calls = Arc::new(AtomicUsize::new(0));
            let counter = Arc::clone(&calls);

            let result = parallel_map(vec![1, 2, 3, 4, 5], move |n: u32| {
                counter.fetch_add(1, Ordering::Relaxed);
                n
            });

            assert_eq!(result, vec![1, 2, 3, 4, 5]);
            assert_eq!(calls.load(Ordering::Relaxed), 5);
        }

        #[test]
        fn a_closure_holding_shared_state_still_fits_the_bounds() {
            let seen = Arc::new(Mutex::new(Vec::new()));
            let recorder = Arc::clone(&seen);

            parallel_map(vec![10u32, 20, 30], move |n| {
                recorder.lock().unwrap().push(n);
            });

            let mut recorded = seen.lock().unwrap().clone();
            recorded.sort_unstable();

            assert_eq!(recorded, vec![10, 20, 30]);
        }
    }

    mod run_jobs_tests {
        use super::*;

        #[test]
        fn no_jobs_gives_no_results() {
            let jobs: Vec<Box<dyn FnOnce() -> u32 + Send>> = vec![];

            assert_eq!(run_jobs(jobs), Vec::<u32>::new());
        }

        #[test]
        fn results_come_back_in_job_order() {
            let jobs: Vec<Box<dyn FnOnce() -> u32 + Send>> = vec![
                Box::new(|| 1),
                Box::new(|| 2),
                Box::new(|| 3),
                Box::new(|| 4),
            ];

            assert_eq!(run_jobs(jobs), vec![1, 2, 3, 4]);
        }

        #[test]
        fn a_job_may_consume_what_it_captured() {
            // `FnOnce`, not `Fn`: this closure moves its captured String out.
            let owned = String::from("rustfinity");
            let jobs: Vec<Box<dyn FnOnce() -> String + Send>> =
                vec![Box::new(move || owned), Box::new(|| String::from("!"))];

            assert_eq!(run_jobs(jobs), vec!["rustfinity", "!"]);
        }

        #[test]
        fn plain_function_pointers_work_too() {
            fn double(n: u32) -> u32 {
                n * 2
            }

            let jobs: Vec<Box<dyn FnOnce() -> u32 + Send>> = (1..=5u32)
                .map(|n| Box::new(move || double(n)) as _)
                .collect();

            assert_eq!(run_jobs(jobs), vec![2, 4, 6, 8, 10]);
        }

        #[test]
        fn every_job_really_runs_on_its_own_thread() {
            let ids = Arc::new(Mutex::new(Vec::new()));

            let jobs: Vec<Box<dyn FnOnce() + Send>> = (0..6)
                .map(|_| {
                    let ids = Arc::clone(&ids);
                    Box::new(move || {
                        ids.lock().unwrap().push(std::thread::current().id());
                    }) as _
                })
                .collect();

            run_jobs(jobs);

            let ids = ids.lock().unwrap();
            let unique: std::collections::HashSet<_> = ids.iter().collect();

            assert_eq!(ids.len(), 6);
            assert_eq!(unique.len(), 6, "each job must get its own thread");
        }
    }
}
