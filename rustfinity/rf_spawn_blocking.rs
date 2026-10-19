//! # Rustfinity: Spawn Blocking
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/spawn-blocking
//!
//! An async runtime runs your tasks on a small pool of worker threads. Tokio's default is one per CPU core. Every task on a worker shares that thread, and a task keeps it until it hits an `.await` and yields.
//!
//! Blocking code never yields. A `std::thread::sleep`, a synchronous file read, a password hash, a tight numeric loop: while any of those runs, every other task queued on that worker is frozen. With four workers, four blocking calls stop the entire runtime.
//!
//! `spawn_blocking` moves the work to a separate pool that exists for exactly this, and hands you a `JoinHandle` to await.
//!
//! ```rust
//! use tokio::task::spawn_blocking;
//!
//! let handle = spawn_blocking(|| expensive(input));
//! let value = handle.await.unwrap();
//! ```
//!
//! The closure must be `Send + 'static`, because it runs on another thread. That usually means taking ownership with `move`.
//!
//! ## Async or blocking?
//!
//! - Waiting on the network, a channel, a timer: async, `.await` it.
//! - Burning CPU, or calling a synchronous library: blocking, `spawn_blocking` it.
//!
//! Roughly: if it does not have an `.await` in it and it is not instant, it belongs on the blocking pool.
//!
//! ## Your Task
//!
//! `checksum` is written for you. It sleeps its thread for 30ms, then folds the bytes of a string into a `u64`. It is deliberately, honestly blocking.
//!
//! Implement two async wrappers around it:
//!
//! - `async checksum_one(data: String) -> u64`, one checksum, off the runtime's worker threads
//! - `async checksum_all(items: Vec<String>) -> Vec<u64>`, every input checksummed concurrently, results in input order
//!
//! Eight payloads must not take eight times as long as one, and a timer running alongside `checksum_all` must keep firing while the checksums are in flight.
//!
//! ### Notes
//!
//! - Spawn every closure first, collect the handles, and await them afterwards. Awaiting inside the loop makes the work sequential again.
//! - `handle.await` gives a `Result`. The task only fails if the closure panicked, so unwrapping it is fine here.
//! - `spawn_blocking` cannot be cancelled once it starts. Dropping the handle just discards the result.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - `checksum` takes `&str`, so a `move` closure that owns a `String` can call `checksum(&data)`.
//! - For `checksum_all`, `items.into_iter().map(...).collect::<Vec<_>>()` gives you the handles in one line.
//! - Then a plain `for handle in handles` loop pushes each awaited value onto the results.
//! - The order falls out for free: you await the handles in the order you spawned them.
//!
//! </details>
//!
//! Run: `cargo run --example rf_spawn_blocking`
//! Test: `cargo test --example rf_spawn_blocking`
//! Filter: `cargo test --example rf_spawn_blocking <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::time::Duration;

/// Stands in for slow, blocking work: a password hash, an image resize,
/// a synchronous file read.
///
/// This one is written for you. It sleeps the *thread* - not the task -
/// and then folds the bytes into a number.
pub fn checksum(data: &str) -> u64 {
    std::thread::sleep(Duration::from_millis(30));

    data.bytes().fold(0u64, |acc, byte| {
        acc.wrapping_mul(31).wrapping_add(byte as u64)
    })
}

/// Runs `checksum` without stalling the async runtime.
pub async fn checksum_one(data: String) -> u64 {
    // TODO
    unimplemented!()
}

/// Checksums every input concurrently, keeping the input order.
pub async fn checksum_all(items: Vec<String>) -> Vec<u64> {
    // TODO
    unimplemented!()
}

// Example usage
#[tokio::main]
pub async fn main() {
    println!("{}", checksum_one("hello".to_string()).await);

    let inputs = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    println!("{:?}", checksum_all(inputs).await);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

    use crate::{checksum, checksum_all, checksum_one};
    use tokio::time::sleep;

    /// How long one `checksum` call blocks its thread.
    const BLOCK: Duration = Duration::from_millis(30);

    fn inputs(count: usize) -> Vec<String> {
        (0..count).map(|index| format!("payload-{index}")).collect()
    }

    #[tokio::test]
    async fn one_checksum_matches_the_blocking_function() {
        assert_eq!(checksum_one("hello".to_string()).await, checksum("hello"));
    }

    #[tokio::test]
    async fn the_empty_input_checksums_too() {
        assert_eq!(checksum_one(String::new()).await, checksum(""));
    }

    #[tokio::test]
    async fn checksumming_nothing_gives_nothing() {
        assert_eq!(checksum_all(Vec::new()).await, Vec::<u64>::new());
    }

    #[tokio::test]
    async fn every_result_keeps_its_input_position() {
        let items = inputs(4);
        let expected: Vec<u64> = items.iter().map(|data| checksum(data)).collect();

        assert_eq!(checksum_all(items).await, expected);
    }

    #[tokio::test]
    async fn equal_inputs_produce_equal_results() {
        let results = checksum_all(vec!["same".to_string(), "same".to_string()]).await;

        assert_eq!(results[0], results[1]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn eight_payloads_do_not_cost_eight_blocks() {
        let start = Instant::now();
        checksum_all(inputs(8)).await;

        assert!(
            start.elapsed() < BLOCK * 4,
            "checksum_all took {:?}, which means the work ran one item at a time",
            start.elapsed()
        );
    }

    #[tokio::test]
    async fn the_runtime_keeps_running_while_the_work_blocks() {
        let ticks = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&ticks);

        let ticker = tokio::spawn(async move {
            for _ in 0..20 {
                sleep(Duration::from_millis(5)).await;
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });

        checksum_all(inputs(8)).await;
        ticker.abort();

        assert!(
            ticks.load(Ordering::SeqCst) >= 3,
            "the timer only ticked {} times, so the blocking work was run on the runtime's own thread",
            ticks.load(Ordering::SeqCst)
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn a_single_worker_thread_is_enough() {
        let items = inputs(6);
        let expected: Vec<u64> = items.iter().map(|data| checksum(data)).collect();

        let start = Instant::now();
        let results = checksum_all(items).await;

        assert_eq!(results, expected);
        assert!(
            start.elapsed() < BLOCK * 3,
            "one worker thread should still hand every payload to the blocking pool"
        );
    }
}
