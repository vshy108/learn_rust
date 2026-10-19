//! # Rustfinity: Atomic Counters
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/atomic-counters
//!
//! A `Mutex<usize>` around a counter works, but it is heavy for the job. Taking a lock means a guard, a possible block, and a poisoning story, all so one thread at a time can run `+= 1`.
//!
//! The atomic types in `std::sync::atomic` do that increment directly. `AtomicUsize::fetch_add` is one instruction that reads, adds, and writes back without any other thread being able to slip in between. There is no guard, nothing to unwrap, and nothing to poison.
//!
//! ```rust
//! let hits = AtomicUsize::new(0);
//! hits.fetch_add(1, Ordering::Relaxed);
//! assert_eq!(hits.load(Ordering::Relaxed), 1);
//! ```
//!
//! Every atomic method takes an `Ordering`, which says how strongly this operation is ordered against the *other* memory your thread touches:
//!
//! - `Relaxed` guarantees the atomic operation itself is not torn or lost, and nothing more. This is what a standalone statistics counter wants.
//! - `Acquire` / `Release` pair up to publish other writes: a `Release` store makes everything the thread wrote beforehand visible to a thread that later does an `Acquire` load.
//! - `SeqCst` adds a single global order over all `SeqCst` operations. It is the safest default and the slowest.
//!
//! If the counter is only ever read for its own sake, `Relaxed` is correct. If reading it is a signal that some *other* data is ready, you need `Acquire` / `Release`.
//!
//! `swap` is the other operation worth knowing: it stores a new value and returns the old one in the same atomic step. That makes "read and reset" safe even while other threads are still incrementing.
//!
//! ## Your Task
//!
//! Implement `CacheStats`, a pair of lock-free counters.
//!
//! ```rust
//! pub struct CacheStats {
//!     hits: AtomicUsize,
//!     misses: AtomicUsize,
//! }
//! ```
//!
//! ### Implement `new`
//!
//! ```rust
//! pub fn new() -> Self
//! ```
//!
//! Both counters at zero.
//!
//! ### Implement `record`
//!
//! ```rust
//! pub fn record(&self, hit: bool)
//! ```
//!
//! Adds one to `hits` when `hit` is true, otherwise one to `misses`. Note it takes `&self`, not `&mut self`: that is the whole appeal.
//!
//! ### Implement `snapshot`
//!
//! ```rust
//! pub fn snapshot(&self) -> (usize, usize)
//! ```
//!
//! Returns `(hits, misses)` without changing anything.
//!
//! ### Implement `drain`
//!
//! ```rust
//! pub fn drain(&self) -> (usize, usize)
//! ```
//!
//! Returns `(hits, misses)` **and** resets both counters to zero. A lookup recorded before the reset must never be lost, so read and reset each counter in one atomic step rather than loading and then storing.
//!
//! ```rust
//! let stats = CacheStats::new();
//! stats.record(true);
//! stats.record(true);
//!
//! assert_eq!(stats.drain(), (2, 0));
//! assert_eq!(stats.snapshot(), (0, 0));
//! ```
//!
//! ### Notes
//!
//! - `Relaxed` is the right ordering throughout this challenge, and the tests do not check which one you used. Understanding *why* it is enough matters more than the keyword.
//! - `drain` reads two separate counters, so it is not atomic as a whole. That is fine for statistics; it would not be for an invariant spanning both.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - `AtomicUsize` implements `Default`, so `#[derive(Default)]` on the struct gives you `new` as `Self::default()`.
//! - Pick the counter first and then bump it: `let counter = if hit { &self.hits } else { &self.misses };`.
//! - Reading is `load(Ordering::Relaxed)`.
//! - `swap(0, Ordering::Relaxed)` returns the previous value and leaves zero behind. That is what `drain` needs on each counter.
//!
//! </details>
//!
//! Run: `cargo run --example rf_atomic_counters`
//! Test: `cargo test --example rf_atomic_counters`
//! Filter: `cargo test --example rf_atomic_counters <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::sync::atomic::{AtomicUsize, Ordering};

/// Cache statistics kept in atomics instead of behind a lock.
#[derive(Default)]
pub struct CacheStats {
    hits: AtomicUsize,
    misses: AtomicUsize,
}

impl CacheStats {
    /// Creates a fresh set of counters, both at zero.
    pub fn new() -> Self {
        // TODO
        unimplemented!()
    }

    /// Records one lookup: a hit if `hit` is true, otherwise a miss.
    pub fn record(&self, hit: bool) {
        // TODO: bump the right counter atomically
        unimplemented!()
    }

    /// Returns `(hits, misses)` as they stand right now.
    pub fn snapshot(&self) -> (usize, usize) {
        // TODO
        unimplemented!()
    }

    /// Returns `(hits, misses)` and sets both counters back to zero.
    pub fn drain(&self) -> (usize, usize) {
        // TODO: read and reset each counter in a single atomic step
        unimplemented!()
    }
}

// Example usage
pub fn main() {
    let stats = CacheStats::new();
    stats.record(true);
    stats.record(false);
    println!("{:?} {:?}", stats.snapshot(), stats.drain());
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::thread;

    mod basics {
        use super::*;

        #[test]
        fn starts_at_zero() {
            assert_eq!(CacheStats::new().snapshot(), (0, 0));
        }

        #[test]
        fn records_hits_and_misses_separately() {
            let stats = CacheStats::new();
            stats.record(true);
            stats.record(true);
            stats.record(false);

            assert_eq!(stats.snapshot(), (2, 1));
        }

        #[test]
        fn snapshot_does_not_reset() {
            let stats = CacheStats::new();
            stats.record(true);

            assert_eq!(stats.snapshot(), (1, 0));
            assert_eq!(stats.snapshot(), (1, 0));
        }

        #[test]
        fn only_misses() {
            let stats = CacheStats::new();
            for _ in 0..5 {
                stats.record(false);
            }

            assert_eq!(stats.snapshot(), (0, 5));
        }
    }

    mod drain_tests {
        use super::*;

        #[test]
        fn returns_the_values_it_clears() {
            let stats = CacheStats::new();
            stats.record(true);
            stats.record(true);
            stats.record(false);

            assert_eq!(stats.drain(), (2, 1));
            assert_eq!(stats.snapshot(), (0, 0));
        }

        #[test]
        fn draining_twice_gives_zeroes_the_second_time() {
            let stats = CacheStats::new();
            stats.record(true);

            assert_eq!(stats.drain(), (1, 0));
            assert_eq!(stats.drain(), (0, 0));
        }

        #[test]
        fn counting_resumes_after_a_drain() {
            let stats = CacheStats::new();
            stats.record(true);
            stats.drain();
            stats.record(false);

            assert_eq!(stats.snapshot(), (0, 1));
        }

        #[test]
        fn draining_an_untouched_counter_is_zero() {
            assert_eq!(CacheStats::new().drain(), (0, 0));
        }
    }

    mod concurrency {
        use super::*;

        #[test]
        fn no_increment_is_lost_under_contention() {
            let stats = CacheStats::new();

            thread::scope(|s| {
                for _ in 0..8 {
                    s.spawn(|| {
                        for i in 0..500 {
                            stats.record(i % 4 != 0);
                        }
                    });
                }
            });

            // 500 lookups per thread, one in four a miss.
            assert_eq!(stats.snapshot(), (8 * 375, 8 * 125));
        }

        #[test]
        fn concurrent_drains_partition_the_counts() {
            let stats = CacheStats::new();
            let stats = &stats;

            thread::scope(|s| {
                let writer = s.spawn(move || {
                    for _ in 0..2000 {
                        stats.record(true);
                    }
                });

                let drainer = s.spawn(move || {
                    let mut collected = 0;
                    for _ in 0..500 {
                        collected += stats.drain().0;
                    }
                    collected
                });

                writer.join().unwrap();
                let drained = drainer.join().unwrap();
                let leftover = stats.drain().0;

                // Every recorded hit is counted exactly once: swap hands the
                // old value out and puts zero in, atomically.
                assert_eq!(drained + leftover, 2000);
            });
        }
    }
}
