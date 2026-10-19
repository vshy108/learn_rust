//! # Rustfinity: Async Channels
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/async-channels
//!
//! `std::sync::mpsc` blocks the thread when you call `recv()`. Inside an async runtime that is a bug: the worker thread stops polling every other task on it while it waits. Async code needs a channel whose "wait" is an `.await`.
//!
//! ```rust
//! use tokio::sync::mpsc;
//!
//! let (tx, mut rx) = mpsc::channel(32);
//! ```
//!
//! `tx.send(value).await` and `rx.recv().await` both yield instead of blocking, so the runtime can run other tasks while this one waits.
//!
//! ## Bounded or unbounded
//!
//! `mpsc::channel(capacity)` is bounded. Once `capacity` messages are queued, `send` waits for the consumer to take one. That wait is **backpressure**: a producer that outruns its consumer is slowed down instead of filling memory.
//!
//! `mpsc::unbounded_channel()` never makes the producer wait, and its `send` is not even `async`. It is the right tool when the producer physically cannot outrun the consumer, and a memory leak waiting to happen otherwise. Prefer bounded, and pick a capacity.
//!
//! ## `None` means finished
//!
//! ```rust
//! while let Some(value) = rx.recv().await {
//!     // ...
//! }
//! ```
//!
//! `recv()` returns `None` when the channel is empty **and** every `Sender` has been dropped. Until then it waits, however long that takes.
//!
//! This is the part that catches people out. The channel counts senders, and clones count. If the task that owns the original `tx` finishes but a clone is still alive somewhere, the loop above never ends. Move the sender into the producing task so it is dropped when that task returns, or `drop(tx)` explicitly once you are done sending.
//!
//! `send` has the mirror-image behaviour: it returns `Err(SendError(value))` when the receiver is gone, handing your value back to you.
//!
//! ## Your Task
//!
//! ### Implement `drain`
//!
//! ```rust
//! pub async fn drain(rx: mpsc::Receiver<i32>) -> Vec<i32>
//! ```
//!
//! Receive everything until the channel closes, and return the values in the order they arrived.
//!
//! ### Implement `produce_and_sum`
//!
//! ```rust
//! pub async fn produce_and_sum(n: i32, capacity: usize) -> i64
//! ```
//!
//! Build a bounded channel of the given capacity. Spawn a task that sends `0`, `1`, ... up to but not including `n`. Receive them all and return the sum.
//!
//! ```rust
//! assert_eq!(produce_and_sum(5, 4).await, 10);
//! assert_eq!(produce_and_sum(500, 1).await, 124_750);
//! ```
//!
//! A capacity of `1` means the producer spends most of its life waiting. It must still finish, and the totals must match.
//!
//! ### Notes
//!
//! - `rx` needs to be mutable to receive from, and `drain` takes it by value.
//! - Sum into an `i64`. The totals overflow `i32` less easily that way.
//! - Do not join the producer task. Waiting for `None` is what tells you it is done.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - `pub async fn drain(mut rx: ...)` puts the `mut` in the pattern, no signature change needed.
//! - `while let Some(value) = rx.recv().await { ... }` is the whole loop.
//! - `let (tx, mut rx) = mpsc::channel(capacity);` then `tokio::spawn(async move { ... })` captures `tx` by move.
//! - Inside the producer, `tx.send(value).await` returns a `Result`; `break` on `Err` or unwrap it.
//! - If a test hangs, a `Sender` is still alive somewhere outside the producer task.
//!
//! </details>
//!
//! Run: `cargo run --example rf_async_channels`
//! Test: `cargo test --example rf_async_channels`
//! Filter: `cargo test --example rf_async_channels <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use tokio::sync::mpsc;

/// Receives every remaining message, in order, until the channel closes.
pub async fn drain(rx: mpsc::Receiver<i32>) -> Vec<i32> {
    // TODO
    unimplemented!()
}

/// Sends `0..n` over a bounded channel and sums what comes back.
pub async fn produce_and_sum(n: i32, capacity: usize) -> i64 {
    // TODO
    unimplemented!()
}

// Example usage
#[tokio::main]
pub async fn main() {
    let (tx, rx) = mpsc::channel(4);

    tokio::spawn(async move {
        for value in [1, 2, 3] {
            tx.send(value).await.unwrap();
        }
    });

    println!("{:?}", drain(rx).await);
    println!("{}", produce_and_sum(100, 1).await);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use crate::{drain, produce_and_sum};
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn everything_sent_comes_back_in_order() {
        let (tx, rx) = mpsc::channel(8);

        for value in [3, 1, 4, 1, 5] {
            tx.send(value).await.unwrap();
        }
        drop(tx);

        assert_eq!(drain(rx).await, vec![3, 1, 4, 1, 5]);
    }

    #[tokio::test]
    async fn a_closed_empty_channel_drains_to_nothing() {
        let (tx, rx) = mpsc::channel::<i32>(4);
        drop(tx);

        assert_eq!(drain(rx).await, Vec::<i32>::new());
    }

    #[tokio::test]
    async fn draining_waits_for_a_slow_producer() {
        let (tx, rx) = mpsc::channel(1);

        tokio::spawn(async move {
            for value in 0..50 {
                tx.send(value).await.unwrap();
            }
        });

        assert_eq!(drain(rx).await, (0..50).collect::<Vec<_>>());
    }

    #[tokio::test]
    async fn every_sender_has_to_go_before_draining_ends() {
        let (tx, rx) = mpsc::channel(8);
        let second = tx.clone();

        tx.send(1).await.unwrap();
        second.send(2).await.unwrap();

        drop(tx);
        drop(second);

        assert_eq!(drain(rx).await, vec![1, 2]);
    }

    #[tokio::test]
    async fn a_small_run_adds_up() {
        assert_eq!(produce_and_sum(5, 4).await, 10);
    }

    #[tokio::test]
    async fn sending_nothing_sums_to_zero() {
        assert_eq!(produce_and_sum(0, 4).await, 0);
    }

    #[tokio::test]
    async fn a_capacity_of_one_still_delivers_everything() {
        assert_eq!(produce_and_sum(500, 1).await, 124_750);
    }

    #[tokio::test]
    async fn a_roomy_channel_delivers_everything_too() {
        assert_eq!(produce_and_sum(500, 512).await, 124_750);
    }

    #[tokio::test]
    async fn a_rendezvous_sized_channel_is_not_a_deadlock() {
        assert_eq!(produce_and_sum(1000, 3).await, 499_500);
    }

    #[tokio::test]
    async fn one_item_arrives_intact() {
        assert_eq!(produce_and_sum(1, 1).await, 0);
        assert_eq!(produce_and_sum(2, 1).await, 1);
    }
}
