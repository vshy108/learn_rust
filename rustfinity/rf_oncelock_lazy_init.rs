//! # Rustfinity: Oncelock Lazy Init
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/oncelock-lazy-init
//!
//! Some values are expensive to build and should only be built if something actually asks for them: a parsed config, a compiled regex, a lookup table. A `static` cannot help, because a `static` initializer has to be a constant expression evaluated at compile time.
//!
//! `OnceLock<T>` is the answer. It is a slot that starts empty and can be filled exactly once:
//!
//! ```rust
//! static SETTINGS: OnceLock<Settings> = OnceLock::new();
//!
//! fn settings() -> &'static Settings {
//!     SETTINGS.get_or_init(build_settings)
//! }
//! ```
//!
//! `get_or_init` is where the concurrency lives. If ten threads call it at the same moment on an empty lock, exactly one runs `build_settings`. The other nine block until it finishes and then get a reference to that same value. The initializer never runs twice, and no caller ever sees a half-built value.
//!
//! Because the value can never be replaced, handing out `&'static T` is sound, and callers need no lock at all to read it.
//!
//! `LazyLock<T>` is the same machinery with the initializer stored inside, so it can be used like a normal static:
//!
//! ```rust
//! static TABLE: LazyLock<Vec<u64>> =
//!     LazyLock::new(|| expensive());
//! ```
//!
//! Reach for `LazyLock` when the initializer is a self-contained closure, and `OnceLock` when the value comes from somewhere the static cannot see, such as a command-line argument set during startup.
//!
//! ## Your Task
//!
//! Fill in three items. `Settings` and the `BUILD_COUNT` counter are given to you.
//!
//! ```rust
//! pub struct Settings {
//!     pub name: String,
//!     pub retries: u32,
//! }
//! ```
//!
//! `build_settings()` is also given. It bumps `BUILD_COUNT` and returns `Settings { name: "rustfinity", retries: 3 }`. Do not change it: the tests use that counter to prove your initializer runs exactly once.
//!
//! ### The storage slot
//!
//! Declare a `static` `OnceLock<Settings>` for `settings()` to fill.
//!
//! ### Implement `settings`
//!
//! ```rust
//! pub fn settings() -> &'static Settings
//! ```
//!
//! Returns the shared settings, building them with `build_settings` on the first call only. Every call, from every thread, must return a reference to the same value.
//!
//! ### Implement `build_count`
//!
//! ```rust
//! pub fn build_count() -> usize
//! ```
//!
//! Reads `BUILD_COUNT`. After any number of `settings()` calls from any number of threads, this must be 1.
//!
//! ### Declare `TRIANGULAR`
//!
//! ```rust
//! pub static TRIANGULAR: LazyLock<Vec<u64>>
//! ```
//!
//! The first 32 triangular numbers, computed on first use: `0, 1, 3, 6, 10, 15, ...`, where entry `n` is the sum of every integer from 0 to `n`.
//!
//! ```rust
//! assert_eq!(TRIANGULAR[..6], [0, 1, 3, 6, 10, 15]);
//! assert_eq!(TRIANGULAR.len(), 32);
//! ```
//!
//! ### Notes
//!
//! - `OnceLock::new()` is a `const fn`, which is what lets it appear in a `static`.
//! - There is a closed form for the nth triangular number, but a running sum works just as well.
//! - `LazyLock<Vec<u64>>` derefs to `Vec<u64>`, so `TRIANGULAR[3]` and `TRIANGULAR.len()` work without any extra unwrapping.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - The slot is `static SETTINGS: OnceLock<Settings> = OnceLock::new();`.
//! - `get_or_init` takes the initializer as a function, so `SETTINGS.get_or_init(build_settings)` is the whole body of `settings`. No parentheses after `build_settings`: you are passing the function, not calling it.
//! - `build_count` is one `load(Ordering::Relaxed)` on the counter.
//! - For the table: `(0..32).map(|n: u64| n * (n + 1) / 2).collect()`.
//! - If the compiler complains that the closure type does not match, remember `LazyLock::new` wants a closure taking no arguments.
//!
//! </details>
//!
//! Run: `cargo run --example rf_oncelock_lazy_init`
//! Test: `cargo test --example rf_oncelock_lazy_init`
//! Filter: `cargo test --example rf_oncelock_lazy_init <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{LazyLock, OnceLock};

/// Settings built once, on first use, and shared for the rest of the process.
#[derive(Debug, PartialEq, Eq)]
pub struct Settings {
    pub name: String,
    pub retries: u32,
}

/// How many times the `Settings` initializer has actually run.
static BUILD_COUNT: AtomicUsize = AtomicUsize::new(0);

// TODO: declare the OnceLock<Settings> slot here

/// Builds the settings. Leave the counter bump in place: the tests use it
/// to prove the initializer runs exactly once.
fn build_settings() -> Settings {
    BUILD_COUNT.fetch_add(1, Ordering::Relaxed);

    Settings {
        name: "rustfinity".to_string(),
        retries: 3,
    }
}

/// Returns the process-wide settings, building them on first access.
pub fn settings() -> &'static Settings {
    // TODO: initialize the slot on first call and return a reference to it
    unimplemented!()
}

/// How many times the settings initializer ran. Always 0 or 1.
pub fn build_count() -> usize {
    // TODO
    unimplemented!()
}

/// The first 32 triangular numbers, computed on first use.
pub static TRIANGULAR: LazyLock<Vec<u64>> = LazyLock::new(|| todo!());

// Example usage
pub fn main() {
    println!("{:?}", settings());
    println!("{} {:?}", build_count(), &TRIANGULAR[..4]);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::thread;

    // These tests share one process-wide `OnceLock`, so they are written to
    // hold no matter which one runs first.

    mod settings_tests {
        use super::*;

        #[test]
        fn has_the_expected_values() {
            assert_eq!(settings().name, "rustfinity");
            assert_eq!(settings().retries, 3);
        }

        #[test]
        fn repeated_calls_return_the_same_value() {
            assert_eq!(settings(), settings());
        }

        #[test]
        fn every_caller_gets_the_same_allocation() {
            let first: *const Settings = settings();
            let second: *const Settings = settings();

            assert!(
                std::ptr::eq(first, second),
                "settings() must hand out the one stored value, not a fresh copy"
            );
        }

        #[test]
        fn the_initializer_runs_at_most_once() {
            for _ in 0..100 {
                settings();
            }

            assert_eq!(build_count(), 1);
        }

        #[test]
        fn racing_threads_still_build_it_once() {
            let pointers = thread::scope(|s| {
                let handles: Vec<_> = (0..8)
                    .map(|_| s.spawn(|| settings() as *const Settings as usize))
                    .collect();

                handles
                    .into_iter()
                    .map(|h| h.join().unwrap())
                    .collect::<Vec<_>>()
            });

            assert_eq!(build_count(), 1);
            assert!(
                pointers.windows(2).all(|pair| pair[0] == pair[1]),
                "all threads must observe the same value"
            );
        }
    }

    mod triangular_tests {
        use super::*;

        #[test]
        fn has_32_entries() {
            assert_eq!(TRIANGULAR.len(), 32);
        }

        #[test]
        fn starts_with_the_known_values() {
            assert_eq!(TRIANGULAR[..6], [0, 1, 3, 6, 10, 15]);
        }

        #[test]
        fn last_entry_is_correct() {
            assert_eq!(TRIANGULAR[31], 31 * 32 / 2);
        }

        #[test]
        fn each_entry_is_the_running_sum() {
            for n in 1..32usize {
                assert_eq!(TRIANGULAR[n], TRIANGULAR[n - 1] + n as u64, "at {n}");
            }
        }

        #[test]
        fn readable_from_many_threads_at_once() {
            thread::scope(|s| {
                for _ in 0..8 {
                    s.spawn(|| {
                        assert_eq!(TRIANGULAR[10], 55);
                    });
                }
            });
        }
    }
}
