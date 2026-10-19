//! # Rustfinity: Rwlock Readers Writers
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/rwlock-readers-writers
//!
//! A `Mutex` gives exactly one thread access at a time, whether that thread is changing the data or only looking at it. For data that is read far more often than it is written, that is a waste: ten threads reading the same map cannot interfere with each other, yet a mutex makes nine of them wait.
//!
//! `RwLock<T>` splits the lock in two:
//!
//! - `read()` returns a shared guard. **Many** readers can hold one at the same time.
//! - `write()` returns an exclusive guard. While it is held, nothing else gets in, reader or writer.
//!
//! ```rust
//! let lock = RwLock::new(vec![1, 2, 3]);
//!
//! let a = lock.read().unwrap();
//! let b = lock.read().unwrap(); // fine, both alive
//! assert_eq!(a.len(), b.len());
//! ```
//!
//! Both methods return a `Result` for the same reason `Mutex::lock` does: the lock is poisoned if a writer panics while holding it.
//!
//! `RwLock` is the right pick when reads dominate and each one is long enough that the extra bookkeeping pays for itself. For a lock held for a couple of instructions, a plain `Mutex` is usually faster.
//!
//! ## Your Task
//!
//! Implement `WordCounts`, a tally where one method writes and three read.
//!
//! ```rust
//! pub struct WordCounts {
//!     counts: RwLock<HashMap<String, usize>>,
//! }
//! ```
//!
//! ### Implement `new`
//!
//! ```rust
//! pub fn new() -> Self
//! ```
//!
//! An empty tally.
//!
//! ### Implement `record`
//!
//! ```rust
//! pub fn record(&self, word: &str)
//! ```
//!
//! Adds one sighting of `word`. This is the only method that takes a write lock.
//!
//! ### Implement `count` and `total`
//!
//! ```rust
//! pub fn count(&self, word: &str) -> usize
//! pub fn total(&self) -> usize
//! ```
//!
//! `count` returns how many times `word` was recorded, or 0 if it was never seen. `total` returns the sum over every word. Both take read locks.
//!
//! ### Implement `most_common`
//!
//! ```rust
//! pub fn most_common(&self)
//!     -> Option<(String, usize)>
//! ```
//!
//! The word with the highest count, and that count. Returns `None` for an empty tally. **Ties are broken by the alphabetically smallest word**, so the result never depends on hash order or thread timing.
//!
//! ```rust
//! let counts = WordCounts::new();
//! counts.record("zebra");
//! counts.record("ant");
//! assert_eq!(
//!     counts.most_common(),
//!     Some(("ant".to_string(), 1))
//! );
//! ```
//!
//! ### Notes
//!
//! - Words are compared exactly. `"Rust"` and `"rust"` are two different words.
//! - Take the read lock in the three readers. Using `write()` everywhere passes the tests but throws away the whole point of `RwLock`.
//! - Never hold a read guard and then ask for a write guard on the same lock in the same thread. That deadlocks.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - `*guard.entry(word.to_string()).or_insert(0) += 1;` inserts a zero on first sight and bumps it.
//! - `HashMap::get` returns `Option<&usize>`; `.copied().unwrap_or(0)` turns it into the `usize` you want.
//! - `guard.values().sum()` gives the total.
//! - For `most_common`, iterate and compare with `min_by`: sort descending on the count first, then ascending on the word with `.then_with(...)`.
//! - Both `read()` and `write()` return a `Result`, so `.unwrap()` after each is expected here.
//!
//! </details>
//!
//! Run: `cargo run --example rf_rwlock_readers_writers`
//! Test: `cargo test --example rf_rwlock_readers_writers`
//! Filter: `cargo test --example rf_rwlock_readers_writers <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::sync::RwLock;

/// A read-heavy word tally guarded by an `RwLock`.
#[derive(Default)]
pub struct WordCounts {
    counts: RwLock<HashMap<String, usize>>,
}

impl WordCounts {
    /// Creates an empty tally.
    pub fn new() -> Self {
        // TODO
        unimplemented!()
    }

    /// Records one sighting of `word`. This is the only writer.
    pub fn record(&self, word: &str) {
        // TODO: take a write lock and bump the entry
        unimplemented!()
    }

    /// How many times `word` has been recorded.
    pub fn count(&self, word: &str) -> usize {
        // TODO: take a read lock
        unimplemented!()
    }

    /// The total number of sightings across every word.
    pub fn total(&self) -> usize {
        // TODO: take a read lock
        unimplemented!()
    }

    /// The word with the most sightings, and its count.
    ///
    /// Ties are broken by taking the alphabetically smallest word.
    pub fn most_common(&self) -> Option<(String, usize)> {
        // TODO: take a read lock
        unimplemented!()
    }
}

// Example usage
pub fn main() {
    let counts = WordCounts::new();
    counts.record("ferris");
    counts.record("ferris");
    counts.record("crab");
    println!("{:?} {}", counts.most_common(), counts.total());
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::thread;

    mod basics {
        use super::*;

        #[test]
        fn a_new_tally_is_empty() {
            let counts = WordCounts::new();
            assert_eq!(counts.total(), 0);
            assert_eq!(counts.count("anything"), 0);
            assert_eq!(counts.most_common(), None);
        }

        #[test]
        fn record_counts_repeats() {
            let counts = WordCounts::new();
            counts.record("ferris");
            counts.record("ferris");
            counts.record("crab");

            assert_eq!(counts.count("ferris"), 2);
            assert_eq!(counts.count("crab"), 1);
        }

        #[test]
        fn unknown_words_count_zero() {
            let counts = WordCounts::new();
            counts.record("ferris");
            assert_eq!(counts.count("gopher"), 0);
        }

        #[test]
        fn total_sums_every_word() {
            let counts = WordCounts::new();
            for word in ["a", "b", "b", "c", "c", "c"] {
                counts.record(word);
            }

            assert_eq!(counts.total(), 6);
        }

        #[test]
        fn words_are_case_sensitive() {
            let counts = WordCounts::new();
            counts.record("Rust");
            counts.record("rust");

            assert_eq!(counts.count("Rust"), 1);
            assert_eq!(counts.count("rust"), 1);
        }
    }

    mod most_common_tests {
        use super::*;

        #[test]
        fn picks_the_highest_count() {
            let counts = WordCounts::new();
            for word in ["a", "b", "b", "b", "c"] {
                counts.record(word);
            }

            assert_eq!(counts.most_common(), Some(("b".to_string(), 3)));
        }

        #[test]
        fn ties_go_to_the_alphabetically_smallest() {
            let counts = WordCounts::new();
            for word in ["zebra", "ant", "moose"] {
                counts.record(word);
            }

            assert_eq!(counts.most_common(), Some(("ant".to_string(), 1)));
        }

        #[test]
        fn a_single_word_wins() {
            let counts = WordCounts::new();
            counts.record("solo");
            assert_eq!(counts.most_common(), Some(("solo".to_string(), 1)));
        }

        #[test]
        fn tie_breaking_is_stable_across_repeated_calls() {
            let counts = WordCounts::new();
            for word in ["delta", "alpha", "charlie", "bravo"] {
                counts.record(word);
            }

            let first = counts.most_common();
            for _ in 0..50 {
                assert_eq!(counts.most_common(), first);
            }
            assert_eq!(first, Some(("alpha".to_string(), 1)));
        }
    }

    mod concurrency {
        use super::*;

        #[test]
        fn concurrent_writers_lose_nothing() {
            let counts = WordCounts::new();

            thread::scope(|s| {
                for _ in 0..8 {
                    s.spawn(|| {
                        for _ in 0..100 {
                            counts.record("shared");
                        }
                    });
                }
            });

            assert_eq!(counts.count("shared"), 800);
            assert_eq!(counts.total(), 800);
        }

        #[test]
        fn writers_on_different_words_do_not_interfere() {
            let counts = WordCounts::new();
            let counts = &counts;
            let words = ["alpha", "bravo", "charlie", "delta"];

            thread::scope(|s| {
                for (index, word) in words.iter().enumerate() {
                    s.spawn(move || {
                        for _ in 0..(index + 1) * 10 {
                            counts.record(word);
                        }
                    });
                }
            });

            assert_eq!(counts.count("alpha"), 10);
            assert_eq!(counts.count("bravo"), 20);
            assert_eq!(counts.count("charlie"), 30);
            assert_eq!(counts.count("delta"), 40);
            assert_eq!(counts.most_common(), Some(("delta".to_string(), 40)));
        }

        #[test]
        fn readers_run_alongside_a_writer_without_deadlocking() {
            let counts = WordCounts::new();
            counts.record("seed");

            thread::scope(|s| {
                s.spawn(|| {
                    for _ in 0..200 {
                        counts.record("seed");
                    }
                });

                for _ in 0..6 {
                    s.spawn(|| {
                        for _ in 0..200 {
                            // Every read must observe a value in range: readers
                            // never see a half-applied increment.
                            let seen = counts.count("seed");
                            assert!((1..=201).contains(&seen), "saw {seen}");
                        }
                    });
                }
            });

            assert_eq!(counts.count("seed"), 201);
        }
    }
}
