//! # Rustfinity: Racing With Select
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/racing-with-select
//!
//! `join!` waits for everything. Sometimes you want the opposite: whichever of several things happens first, and none of the rest.
//!
//! `tokio::select!` polls every branch and completes as soon as one of them does.
//!
//! ```rust
//! tokio::select! {
//!     value = fetch_from_cache() => use_it(value),
//!     value = fetch_from_disk()  => use_it(value),
//! }
//! ```
//!
//! Each arm is `pattern = future => expression`. Every arm must produce the same type, exactly like a `match`.
//!
//! ## Every loser is cancelled
//!
//! When one branch wins, the others are dropped mid-flight. Whatever they had done so far is thrown away.
//!
//! That makes `select!` in a loop a sharp tool. A future that is safe to drop half-finished and poll again from scratch is called **cancel safe**. `Receiver::recv` is: a message is either taken or still in the channel, never lost in between. Reading half a line from a socket into a local buffer is not.
//!
//! If several branches are ready at once, `select!` picks between them at random, so never write code that depends on branch order.
//!
//! ## Reusing a future across iterations
//!
//! A future created inside a loop body starts over on every pass. For a deadline that is exactly wrong: the timer resets after each message and never fires. Create it once, pin it, then poll the same one with `&mut`.
//!
//! ```rust
//! let deadline = sleep(limit);
//! tokio::pin!(deadline);
//!
//! loop {
//!     tokio::select! {
//!         v = rx.recv() => { /* ... */ }
//!         _ = &mut deadline => break,
//!     }
//! }
//! ```
//!
//! `tokio::pin!` pins the future in place on the stack, which is what lets you take `&mut` of it.
//!
//! ## Your Task
//!
//! ### Implement `first_of`
//!
//! ```rust
//! pub async fn first_of<A, B>(a: A, b: B) -> String
//! where
//!     A: Future<Output = String>,
//!     B: Future<Output = String>,
//! ```
//!
//! Return whichever future produces a value first, and drop the other.
//!
//! ### Implement `collect_until_deadline`
//!
//! ```rust
//! pub async fn collect_until_deadline(
//!     rx: &mut mpsc::Receiver<i32>,
//!     limit: Duration,
//! ) -> Vec<i32>
//! ```
//!
//! Collect messages in arrival order and stop at whichever comes first: the channel closing, or `limit` elapsing since the call started.
//!
//! A producer sending steadily forever must not keep the collection running past `limit`.
//!
//! ### Notes
//!
//! - The deadline is measured from the start of the call, not from the last message.
//! - `rx.recv()` yields `None` when every sender is gone. That ends the loop too.
//! - Both branches of a `select!` arm must return the same type, so do the work in a block.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - `first_of` is one `select!` with two arms, each binding a value and returning it.
//! - In `collect_until_deadline`, build the sleep and `tokio::pin!` it before the loop.
//! - Match on the `Option` the receive arm produces: push on `Some`, `break` on `None`.
//! - The deadline arm ignores its output and just breaks.
//! - If a test hangs, the deadline is being created inside the loop instead of outside it.
//!
//! </details>
//!
//! Run: `cargo run --example rf_racing_with_select`
//! Test: `cargo test --example rf_racing_with_select`
//! Filter: `cargo test --example rf_racing_with_select <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::future::Future;
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time::sleep;

/// Returns whichever of the two futures produces a value first.
pub async fn first_of<A, B>(a: A, b: B) -> String
where
    A: Future<Output = String>,
    B: Future<Output = String>,
{
    // TODO
    unimplemented!()
}

/// Collects messages until the channel closes or `limit` elapses.
pub async fn collect_until_deadline(rx: &mut mpsc::Receiver<i32>, limit: Duration) -> Vec<i32> {
    // TODO
    unimplemented!()
}

// Example usage
#[tokio::main]
pub async fn main() {
    let slow = async {
        sleep(Duration::from_millis(50)).await;
        "slow".to_string()
    };
    let quick = async {
        sleep(Duration::from_millis(5)).await;
        "quick".to_string()
    };

    println!("{}", first_of(slow, quick).await);

    let (tx, mut rx) = mpsc::channel(8);
    tokio::spawn(async move {
        for value in 0..3 {
            tx.send(value).await.unwrap();
        }
    });

    let collected = collect_until_deadline(&mut rx, Duration::from_millis(50)).await;
    println!("{collected:?}");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    use crate::{collect_until_deadline, first_of};
    use tokio::sync::mpsc;
    use tokio::time::{Instant, sleep};

    const TICK: Duration = Duration::from_millis(10);

    async fn after(delay: Duration, label: &str) -> String {
        sleep(delay).await;
        label.to_string()
    }

    #[tokio::test(start_paused = true)]
    async fn the_quicker_of_the_two_wins() {
        let winner = first_of(after(TICK * 5, "slow"), after(TICK, "quick")).await;

        assert_eq!(winner, "quick");
    }

    #[tokio::test(start_paused = true)]
    async fn the_first_branch_can_win_too() {
        let winner = first_of(after(TICK, "quick"), after(TICK * 5, "slow")).await;

        assert_eq!(winner, "quick");
    }

    #[tokio::test(start_paused = true)]
    async fn racing_takes_as_long_as_the_winner_not_the_loser() {
        let start = Instant::now();

        first_of(after(TICK * 50, "slow"), after(TICK, "quick")).await;

        assert!(start.elapsed() < TICK * 2);
    }

    #[tokio::test(start_paused = true)]
    async fn the_losing_future_is_cancelled_where_it_stands() {
        static LOSER_FINISHED: AtomicUsize = AtomicUsize::new(0);

        let loser = async {
            sleep(TICK * 5).await;
            LOSER_FINISHED.fetch_add(1, Ordering::SeqCst);
            "slow".to_string()
        };

        assert_eq!(first_of(loser, after(TICK, "quick")).await, "quick");

        sleep(TICK * 20).await;
        assert_eq!(LOSER_FINISHED.load(Ordering::SeqCst), 0);
    }

    #[tokio::test(start_paused = true)]
    async fn a_ready_future_beats_anything_that_has_to_wait() {
        let winner = first_of(async { "instant".to_string() }, after(TICK, "quick")).await;

        assert_eq!(winner, "instant");
    }

    #[tokio::test(start_paused = true)]
    async fn a_channel_that_closes_early_ends_the_collection() {
        let (tx, mut rx) = mpsc::channel(8);

        for value in [3, 1, 4] {
            tx.send(value).await.unwrap();
        }
        drop(tx);

        let start = Instant::now();
        let collected = collect_until_deadline(&mut rx, TICK * 100).await;

        assert_eq!(collected, vec![3, 1, 4]);
        assert!(start.elapsed() < TICK);
    }

    #[tokio::test(start_paused = true)]
    async fn a_closed_empty_channel_collects_nothing() {
        let (tx, mut rx) = mpsc::channel::<i32>(4);
        drop(tx);

        assert_eq!(collect_until_deadline(&mut rx, TICK * 10).await, Vec::new());
    }

    #[tokio::test(start_paused = true)]
    async fn the_deadline_cuts_a_producer_off_mid_stream() {
        let (tx, mut rx) = mpsc::channel(1);

        tokio::spawn(async move {
            for value in 0..100 {
                sleep(TICK).await;
                if tx.send(value).await.is_err() {
                    break;
                }
            }
        });

        let collected = collect_until_deadline(&mut rx, TICK * 5 + TICK / 2).await;

        assert_eq!(collected, vec![0, 1, 2, 3, 4]);
    }

    #[tokio::test(start_paused = true)]
    async fn a_steady_producer_does_not_push_the_deadline_back() {
        let (tx, mut rx) = mpsc::channel(1);

        tokio::spawn(async move {
            for value in 0..1000 {
                sleep(TICK).await;
                if tx.send(value).await.is_err() {
                    break;
                }
            }
        });

        let start = Instant::now();
        let collected = collect_until_deadline(&mut rx, TICK * 3 + TICK / 2).await;

        assert_eq!(collected, vec![0, 1, 2]);
        assert!(start.elapsed() < TICK * 4);
    }

    #[tokio::test(start_paused = true)]
    async fn a_deadline_that_is_up_before_any_message_collects_nothing() {
        let (tx, mut rx) = mpsc::channel(1);

        tokio::spawn(async move {
            sleep(TICK * 10).await;
            let _ = tx.send(1).await;
        });

        assert_eq!(collect_until_deadline(&mut rx, TICK).await, Vec::new());
    }
}
