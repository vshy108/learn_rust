//! # Rustfinity: Mutex Poisoning
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/mutex-poisoning
//!
//! `Mutex::lock` does not return a guard. It returns a `Result<MutexGuard<T>, PoisonError<...>>`, and most code papers over that with `.unwrap()`. That `unwrap` is not there for the usual "this can't fail" reason: it is there because a mutex can be **poisoned**.
//!
//! A mutex becomes poisoned when a thread panics while holding its guard. Rust assumes the panicking thread was halfway through a change and left the data in a state the rest of the program was never meant to see, so every later `lock()` call returns `Err` as a warning.
//!
//! The data itself is untouched and still reachable. `PoisonError` owns the guard, and `into_inner` gives it to you:
//!
//! ```rust
//! let guard = mutex
//!     .lock()
//!     .unwrap_or_else(|e| e.into_inner());
//! ```
//!
//! Whether that is the right call depends on the invariant. For a plain counter, a half-finished increment is not dangerous, so recovering is reasonable. For a data structure with an invariant spanning several fields, it may not be.
//!
//! ## Your Task
//!
//! Implement `ResilientCounter`, a counter that never panics on a poisoned lock.
//!
//! ```rust
//! pub struct ResilientCounter {
//!     value: Mutex<i64>,
//! }
//! ```
//!
//! ### Implement `new`
//!
//! ```rust
//! pub fn new(start: i64) -> Self
//! ```
//!
//! Creates a counter holding `start`.
//!
//! ### Implement `with`
//!
//! ```rust
//! pub fn with<R>(
//!     &self,
//!     f: impl FnOnce(&mut i64) -> R,
//! ) -> R
//! ```
//!
//! Takes the lock, calls `f` with a mutable reference to the value, and returns whatever `f` returned. If the mutex is poisoned, recover the guard and run `f` anyway.
//!
//! ```rust
//! let counter = ResilientCounter::new(0);
//! counter.with(|value| *value += 21);
//! assert_eq!(counter.get(), 21);
//! ```
//!
//! ### Implement `get`
//!
//! ```rust
//! pub fn get(&self) -> i64
//! ```
//!
//! Returns the current value, poisoned or not.
//!
//! ### Implement `is_poisoned`
//!
//! ```rust
//! pub fn is_poisoned(&self) -> bool
//! ```
//!
//! Reports whether a thread has ever panicked while holding the lock. Recovering the guard does **not** clear the flag, so this stays `true` afterwards.
//!
//! ### Notes
//!
//! - The lock is only poisoned if the panic happens while the guard is alive. A thread that locks, releases, and then panics leaves the mutex healthy.
//! - `with` must not hold the lock after it returns. Locking inside `with` and releasing at the end of `with` is what makes nesting easy to reason about.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - `unwrap_or_else` on a `Result` gives you the error value to work with: `self.value.lock().unwrap_or_else(|e| e.into_inner())`.
//! - `into_inner` consumes the `PoisonError` and returns the `MutexGuard` that was inside it.
//! - A `MutexGuard<i64>` derefs to `i64`, so `f(&mut guard)` coerces to the `&mut i64` that `f` expects.
//! - `get` does not need its own locking code. Write it as `self.with(|value| *value)`.
//! - `Mutex` has an `is_poisoned` method already, so the last one is a single line.
//!
//! </details>
//!
//! Run: `cargo run --example rf_mutex_poisoning`
//! Test: `cargo test --example rf_mutex_poisoning`
//! Filter: `cargo test --example rf_mutex_poisoning <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::sync::Mutex;

/// A counter that keeps working after a thread panics while holding the lock.
pub struct ResilientCounter {
    value: Mutex<i64>,
}

impl ResilientCounter {
    /// Creates a counter starting at `start`.
    pub fn new(start: i64) -> Self {
        // TODO: build the counter
        unimplemented!()
    }

    /// Runs `f` on the counter while holding the lock, and returns its result.
    ///
    /// The lock is taken even when the mutex is poisoned.
    pub fn with<R>(&self, f: impl FnOnce(&mut i64) -> R) -> R {
        // TODO: take the lock, recovering it if it is poisoned, then call `f`
        unimplemented!()
    }

    /// Reads the counter, even if the mutex is poisoned.
    pub fn get(&self) -> i64 {
        // TODO
        unimplemented!()
    }

    /// Reports whether some thread has panicked while holding the lock.
    pub fn is_poisoned(&self) -> bool {
        // TODO
        unimplemented!()
    }
}

// Example usage
pub fn main() {
    let counter = ResilientCounter::new(0);
    counter.with(|value| *value += 3);
    println!("{} {}", counter.get(), counter.is_poisoned());
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::thread;

    /// Panics inside a thread while it holds `counter`'s lock, which poisons it.
    /// The panicking thread is joined here, so nothing leaks out into the test.
    fn poison(counter: &ResilientCounter) {
        let result = thread::scope(|s| {
            s.spawn(|| {
                counter.with(|value| {
                    *value += 1;
                    panic!("worker exploded");
                })
            })
            .join()
        });

        assert!(result.is_err(), "the worker was supposed to panic");
    }

    mod basics {
        use super::*;

        #[test]
        fn starts_at_the_given_value() {
            assert_eq!(ResilientCounter::new(5).get(), 5);
        }

        #[test]
        fn with_can_mutate_and_return() {
            let counter = ResilientCounter::new(0);
            counter.with(|value| *value += 3);
            let doubled = counter.with(|value| {
                *value -= 1;
                *value * 2
            });

            assert_eq!(doubled, 4);
            assert_eq!(counter.get(), 2);
        }

        #[test]
        fn a_fresh_counter_is_not_poisoned() {
            assert!(!ResilientCounter::new(0).is_poisoned());
        }

        #[test]
        fn negative_start_is_fine() {
            assert_eq!(ResilientCounter::new(-40).get(), -40);
        }
    }

    mod poisoning {
        use super::*;

        #[test]
        fn a_panicking_holder_poisons_the_mutex() {
            let counter = ResilientCounter::new(0);
            poison(&counter);
            assert!(counter.is_poisoned());
        }

        #[test]
        fn get_still_works_after_poisoning() {
            let counter = ResilientCounter::new(10);
            poison(&counter);
            assert_eq!(counter.get(), 11);
        }

        #[test]
        fn with_still_works_after_poisoning() {
            let counter = ResilientCounter::new(0);
            poison(&counter);
            counter.with(|value| *value += 41);
            assert_eq!(counter.get(), 42);
        }

        #[test]
        fn the_write_before_the_panic_is_kept() {
            let counter = ResilientCounter::new(100);
            poison(&counter);
            assert_eq!(counter.get(), 101, "the +1 happened before the panic");
        }

        #[test]
        fn poisoning_twice_is_harmless() {
            let counter = ResilientCounter::new(0);
            poison(&counter);
            poison(&counter);
            assert_eq!(counter.get(), 2);
            assert!(counter.is_poisoned());
        }
    }

    mod concurrency {
        use super::*;

        #[test]
        fn many_threads_add_without_losing_updates() {
            let counter = ResilientCounter::new(0);

            thread::scope(|s| {
                for _ in 0..8 {
                    s.spawn(|| {
                        for _ in 0..250 {
                            counter.with(|value| *value += 1);
                        }
                    });
                }
            });

            assert_eq!(counter.get(), 2000);
        }

        #[test]
        fn healthy_threads_keep_going_after_one_panics() {
            let counter = ResilientCounter::new(0);
            poison(&counter);

            thread::scope(|s| {
                for _ in 0..4 {
                    s.spawn(|| {
                        for _ in 0..100 {
                            counter.with(|value| *value += 1);
                        }
                    });
                }
            });

            assert_eq!(counter.get(), 401);
            assert!(counter.is_poisoned());
        }
    }
}
