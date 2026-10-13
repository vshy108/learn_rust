//! # Rustfinity: Handling Thread Panics
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/handling-thread-panics
//!
//! A panic unwinds only the thread it happens on. The rest of the program keeps running, which is exactly why `JoinHandle::join` returns a `Result` rather than the value directly. If the worker panicked, the calling thread finds out by getting an `Err` back instead of being taken down with it.
//!
//! The error side is `Box<dyn Any + Send>`, the **panic payload**. It is deliberately untyped, because a panic can carry anything. In practice it carries one of two things: `panic!("literal")` stores a `&'static str`, and a formatted `panic!("{}", value)` stores a `String`. Reading the message means downcasting to each of those in turn.
//!
//! This is also why blindly writing `handle.join().unwrap()` is worth thinking about. It works, but it converts a worker's panic into a panic in your own thread. Sometimes that is what you want. Often you would rather turn it into a value and decide what to do.
//!
//! ## Your Task
//!
//! Implement two functions.
//!
//! ### Implement `describe_panic`
//!
//! ```rust
//! pub fn describe_panic(
//!     job: Box<dyn FnOnce() + Send + 'static>,
//! ) -> Option<String>
//! ```
//!
//! Run `job` on a spawned thread and join it.
//!
//! - Finished normally: return `None`.
//! - Panicked: return `Some(message)` with the panic message.
//! - Payload that is neither a `&'static str` nor a `String`: return `Some("unknown panic")`.
//!
//! ```rust
//! let ok = describe_panic(Box::new(|| {}));
//! assert_eq!(ok, None);
//!
//! let bad = describe_panic(Box::new(|| panic!("boom")));
//! assert_eq!(bad, Some("boom".to_string()));
//! ```
//!
//! ### Implement `divide_in_thread`
//!
//! ```rust
//! pub fn divide_in_thread(
//!     a: i32,
//!     b: i32,
//! ) -> Result<i32, String>
//! ```
//!
//! Compute `a / b` on a spawned thread. When `b` is 0 the thread must panic with the message `"division by zero"`. Join it and turn a panic into `Err(message)`.
//!
//! ```rust
//! assert_eq!(divide_in_thread(10, 2), Ok(5));
//! assert_eq!(
//!     divide_in_thread(1, 0),
//!     Err("division by zero".to_string()),
//! );
//! ```
//!
//! ### Notes
//!
//! - Your job is to panic and then recover from it, not to avoid the panic. Do not check for zero and return early.
//! - The test output will show panic messages on stderr. That is expected and does not mean a test failed.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - `match handle.join() { Ok(value) => ..., Err(payload) => ... }`.
//! - `payload.downcast_ref::<&'static str>()` covers `panic!("literal")`.
//! - `payload.downcast_ref::<String>()` covers `panic!("{}", x)`. Try both before giving up.
//! - `downcast_ref` hands you a reference, so finish with `.to_string()` or `.clone()`.
//! - For the second function, `join()` gives a `Result` already, so `map_err` is a tidy way to convert only the error side.
//!
//! </details>
//!
//! Run: `cargo run --example rf_handling_thread_panics`
//! Test: `cargo test --example rf_handling_thread_panics`
//! Filter: `cargo test --example rf_handling_thread_panics <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::thread;

/// Runs a job on a thread and reports whether it panicked.
///
/// Returns `None` when the job finished normally, or `Some(message)`
/// with the panic message. Payloads that are neither `&'static str` nor
/// `String` become `"unknown panic"`.
pub fn describe_panic(job: Box<dyn FnOnce() + Send + 'static>) -> Option<String> {
    // TODO: spawn the job, join it, and inspect the error payload
    unimplemented!()
}

/// Divides on a worker thread, turning a panic into an `Err`.
pub fn divide_in_thread(a: i32, b: i32) -> Result<i32, String> {
    // TODO: panic with "division by zero" inside the thread when b is 0,
    // then map the joined panic payload into Err
    unimplemented!()
}

// Example usage
pub fn main() {
    println!("{:?}", describe_panic(Box::new(|| {})));
    println!("{:?}", describe_panic(Box::new(|| panic!("boom"))));

    println!("{:?}", divide_in_thread(10, 2));
    println!("{:?}", divide_in_thread(1, 0));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    mod describe_panic_tests {
        use super::*;

        #[test]
        fn clean_job_reports_nothing() {
            assert_eq!(describe_panic(Box::new(|| {})), None);
        }

        #[test]
        fn job_that_does_work_reports_nothing() {
            assert_eq!(
                describe_panic(Box::new(|| {
                    let _total: u64 = (0..1000).sum();
                })),
                None
            );
        }

        #[test]
        fn captures_a_literal_panic_message() {
            assert_eq!(
                describe_panic(Box::new(|| panic!("boom"))),
                Some("boom".to_string())
            );
        }

        #[test]
        fn captures_a_formatted_panic_message() {
            let code = 42;
            assert_eq!(
                describe_panic(Box::new(move || panic!("failed with {code}"))),
                Some("failed with 42".to_string())
            );
        }

        #[test]
        fn captures_an_unwrap_failure() {
            let message = describe_panic(Box::new(|| {
                let none: Option<i32> = None;
                none.unwrap();
            }));

            assert!(message.is_some(), "expected the panic to be reported");
        }

        #[test]
        fn captures_an_index_out_of_bounds() {
            let message = describe_panic(Box::new(|| {
                let v = vec![1, 2, 3];
                let _ = v[10];
            }));

            assert!(message.is_some(), "expected the panic to be reported");
        }

        #[test]
        fn a_panicking_job_does_not_kill_the_caller() {
            let _ = describe_panic(Box::new(|| panic!("boom")));

            // Still here, so the caller survived the worker's panic.
            assert_eq!(describe_panic(Box::new(|| {})), None);
        }
    }

    mod divide_in_thread_tests {
        use super::*;

        #[test]
        fn divides_evenly() {
            assert_eq!(divide_in_thread(10, 2), Ok(5));
        }

        #[test]
        fn truncates_toward_zero() {
            assert_eq!(divide_in_thread(7, 2), Ok(3));
            assert_eq!(divide_in_thread(-7, 2), Ok(-3));
        }

        #[test]
        fn dividing_by_zero_is_an_error() {
            assert_eq!(divide_in_thread(1, 0), Err("division by zero".to_string()));
        }

        #[test]
        fn zero_numerator_is_fine() {
            assert_eq!(divide_in_thread(0, 5), Ok(0));
        }

        #[test]
        fn zero_over_zero_is_still_an_error() {
            assert_eq!(divide_in_thread(0, 0), Err("division by zero".to_string()));
        }

        #[test]
        fn negative_divisor() {
            assert_eq!(divide_in_thread(9, -3), Ok(-3));
        }

        #[test]
        fn the_caller_keeps_running_after_an_error() {
            assert!(divide_in_thread(1, 0).is_err());
            assert_eq!(divide_in_thread(8, 4), Ok(2));
        }
    }
}
