//! # Rustfinity: Async Mutex
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/async-mutex
//!
//! `std::sync::Mutex` and `tokio::sync::Mutex` look almost the same. The difference is what happens when the lock is already taken.
//!
//! `std::sync::Mutex::lock` blocks the thread. On an async runtime that thread is a worker shared by many tasks, so blocking it stalls all of them. Worse, an async runtime may have far fewer threads than tasks, and enough blocked workers is a deadlock.
//!
//! `tokio::sync::Mutex::lock` is `async`. It yields, the runtime runs something else, and your task wakes when the lock is free.
//!
//! ```rust
//! use tokio::sync::Mutex;
//!
//! let guard = shared.lock().await;
//! ```
//!
//! ## The rule about guards and awaits
//!
//! A `std::sync::MutexGuard` is not `Send`. Hold one across an `.await` and the whole future stops being `Send`, so `tokio::spawn` refuses it. The compiler catches the mistake, and the error is confusing the first time you hit it.
//!
//! So: if the critical section contains an `.await`, use `tokio::sync::Mutex`. If it does not, keep `std::sync::Mutex`, which is faster and simpler.
//!
//! Holding a lock across an await is still a real cost. Every other caller is stopped for the duration. Sometimes that is the entire point.
//!
//! ## Your Task
//!
//! Build a cache whose misses are filled by an async loader, where a key that several tasks ask for at once is loaded exactly once.
//!
//! `load_value` is written for you. It sleeps, then returns a string.
//!
//! ```rust
//! pub struct Cache {
//!     entries: Mutex<HashMap<String, String>>,
//!     loads: AtomicUsize,
//! }
//! ```
//!
//! Implement four methods:
//!
//! - `new() -> Self`, an empty cache
//! - `async get_or_load(&self, key: &str) -> String`, the value for `key`, calling `load_value` only on a miss
//! - `loads(&self) -> usize`, how many times the loader has run
//! - `async snapshot(&self) -> Vec<(String, String)>`, every cached pair sorted by key
//!
//! Eight tasks calling `get_or_load("config")` at the same time must leave `loads()` at `1`, and all eight must get the same value.
//!
//! ### Notes
//!
//! - The dedup falls out of holding the guard across the load. Do not drop it and take it again.
//! - Sorting a `Vec<(String, String)>` sorts by key first, which is what you want.
//! - `Ordering::SeqCst` is a fine default for the counter here.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - `#[derive(Default)]` is already on the struct, so `new` can be `Self::default()`.
//! - In `get_or_load`, take `let mut entries = self.entries.lock().await;` first, before anything else.
//! - Return early with a clone if `entries.get(key)` is `Some`.
//! - On a miss: await the load, bump the counter, insert a clone, return the value.
//! - For `snapshot`, collect cloned pairs out of the map, then `sort()`.
//!
//! </details>
//!
//! Run: `cargo run --example rf_async_mutex`
//! Test: `cargo test --example rf_async_mutex`
//! Filter: `cargo test --example rf_async_mutex <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::time::Duration;

use tokio::sync::Mutex;
use tokio::time::sleep;

/// Stands in for an expensive lookup: a network call, a query, a file read.
///
/// This one is written for you. It only sleeps and builds a string.
pub async fn load_value(key: &str) -> String {
    sleep(Duration::from_millis(10)).await;
    format!("value-for-{key}")
}

/// A cache whose misses are filled by an async loader.
#[derive(Default)]
pub struct Cache {
    entries: Mutex<HashMap<String, String>>,
    loads: AtomicUsize,
}

impl Cache {
    pub fn new() -> Self {
        // TODO
        unimplemented!()
    }

    /// Returns the value for `key`, loading it exactly once.
    pub async fn get_or_load(&self, key: &str) -> String {
        // TODO
        unimplemented!()
    }

    /// How many times the loader has actually run.
    pub fn loads(&self) -> usize {
        // TODO
        unimplemented!()
    }

    /// Every cached pair, sorted by key.
    pub async fn snapshot(&self) -> Vec<(String, String)> {
        // TODO
        unimplemented!()
    }
}

// Example usage
#[tokio::main]
pub async fn main() {
    use std::sync::Arc;

    let cache = Arc::new(Cache::new());
    let mut handles = Vec::new();

    for _ in 0..8 {
        let cache = Arc::clone(&cache);
        handles.push(tokio::spawn(
            async move { cache.get_or_load("config").await },
        ));
    }

    for handle in handles {
        println!("{}", handle.await.unwrap());
    }

    println!("loads: {}", cache.loads());
    println!("{:?}", cache.snapshot().await);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::sync::Arc;
    use std::time::Duration;

    use crate::{Cache, load_value};
    use tokio::time::Instant;

    const LOAD_TIME: Duration = Duration::from_millis(10);

    #[tokio::test(start_paused = true)]
    async fn a_fresh_cache_has_loaded_nothing() {
        let cache = Cache::new();

        assert_eq!(cache.loads(), 0);
        assert_eq!(cache.snapshot().await, Vec::new());
    }

    #[tokio::test(start_paused = true)]
    async fn a_miss_returns_what_the_loader_produced() {
        let cache = Cache::new();

        assert_eq!(
            cache.get_or_load("config").await,
            load_value("config").await
        );
        assert_eq!(cache.loads(), 1);
    }

    #[tokio::test(start_paused = true)]
    async fn a_second_look_at_the_same_key_does_not_load_again() {
        let cache = Cache::new();

        let first = cache.get_or_load("config").await;
        let second = cache.get_or_load("config").await;

        assert_eq!(first, second);
        assert_eq!(cache.loads(), 1);
    }

    #[tokio::test(start_paused = true)]
    async fn a_hit_costs_no_time_at_all() {
        let cache = Cache::new();
        cache.get_or_load("config").await;

        let start = Instant::now();
        cache.get_or_load("config").await;

        assert!(start.elapsed() < LOAD_TIME);
    }

    #[tokio::test(start_paused = true)]
    async fn different_keys_load_separately() {
        let cache = Cache::new();

        cache.get_or_load("a").await;
        cache.get_or_load("b").await;
        cache.get_or_load("a").await;

        assert_eq!(cache.loads(), 2);
    }

    #[tokio::test(start_paused = true)]
    async fn the_snapshot_is_sorted_by_key() {
        let cache = Cache::new();

        for key in ["pear", "apple", "quince"] {
            cache.get_or_load(key).await;
        }

        let keys: Vec<String> = cache
            .snapshot()
            .await
            .into_iter()
            .map(|(key, _)| key)
            .collect();

        assert_eq!(keys, vec!["apple", "pear", "quince"]);
    }

    #[tokio::test(start_paused = true)]
    async fn the_snapshot_pairs_every_key_with_its_value() {
        let cache = Cache::new();
        cache.get_or_load("config").await;

        assert_eq!(
            cache.snapshot().await,
            vec![("config".to_string(), load_value("config").await)]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn eight_tasks_racing_for_one_key_load_it_once() {
        let cache = Arc::new(Cache::new());
        let mut handles = Vec::new();

        for _ in 0..8 {
            let cache = Arc::clone(&cache);
            handles.push(tokio::spawn(
                async move { cache.get_or_load("config").await },
            ));
        }

        let expected = load_value("config").await;

        for handle in handles {
            assert_eq!(handle.await.unwrap(), expected);
        }

        assert_eq!(cache.loads(), 1);
    }

    #[tokio::test(start_paused = true)]
    async fn racing_tasks_across_four_keys_load_four_times() {
        let cache = Arc::new(Cache::new());
        let mut handles = Vec::new();

        for index in 0..16 {
            let cache = Arc::clone(&cache);
            let key = format!("key-{}", index % 4);
            handles.push(tokio::spawn(async move { cache.get_or_load(&key).await }));
        }

        for handle in handles {
            handle.await.unwrap();
        }

        assert_eq!(cache.loads(), 4);
        assert_eq!(cache.snapshot().await.len(), 4);
    }

    #[tokio::test(start_paused = true)]
    async fn a_cache_can_be_shared_across_threads() {
        fn assert_shareable<T: Send + Sync + 'static>() {}

        assert_shareable::<Cache>();
    }
}
