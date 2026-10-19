//! # Rustfinity: Timeouts And Sleep
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/timeouts-and-sleep
//!
//! `std::thread::sleep` parks the whole thread. Inside an async runtime that is a disaster: one task naps and every other task sharing that worker thread stops making progress.
//!
//! `tokio::time::sleep` is the async version. It yields, the runtime goes off and polls everything else, and your task is woken when the timer fires.
//!
//! ```rust
//! use tokio::time::sleep;
//! use std::time::Duration;
//!
//! sleep(Duration::from_millis(50)).await;
//! ```
//!
//! ## Putting a limit on a future
//!
//! `tokio::time::timeout` wraps any future and races it against a timer.
//!
//! ```rust
//! use tokio::time::timeout;
//!
//! let limit = Duration::from_millis(100);
//!
//! match timeout(limit, slow_work()).await {
//!     Ok(value) => println!("got {value}"),
//!     Err(_) => println!("gave up"),
//! }
//! ```
//!
//! The error type, `Elapsed`, carries nothing beyond "the time was up", so `.ok()` is usually all you want from it.
//!
//! The important part is what happens to the inner future when the timer wins: it is **dropped**. Not signalled, not asked to stop. Dropped, part-finished, at whatever `.await` it was suspended on. Anything after that point never runs. This is what people mean when they say async Rust has cancellation built in.
//!
//! ## Your Task
//!
//! ### Implement `with_timeout`
//!
//! ```rust
//! pub async fn with_timeout<F>(
//!     limit: Duration,
//!     work: F,
//! ) -> Option<F::Output>
//! where
//!     F: Future,
//! ```
//!
//! Run `work` with a deadline. Return `Some(output)` if it finished in time, `None` if the limit ran out first.
//!
//! ### Implement `retry`
//!
//! ```rust
//! pub async fn retry(
//!     attempts: usize,
//!     gap: Duration,
//!     op: impl FnMut() -> bool,
//! ) -> Option<usize>
//! ```
//!
//! Call `op` until it returns `true`, at most `attempts` times, waiting `gap` between one call and the next. Return the zero-based number of the attempt that succeeded, or `None` if none did.
//!
//! The gap goes **between** attempts only. One attempt sleeps not at all, four failing attempts sleep three times.
//!
//! ```rust
//! let mut tries = 0;
//! let found = retry(5, gap, || {
//!     tries += 1;
//!     tries == 3
//! })
//! .await;
//!
//! assert_eq!(found, Some(2));
//! ```
//!
//! ### Notes
//!
//! - `op` needs to be `mut` to be called, since it is `FnMut`.
//! - `attempts` of `0` means `op` is never called at all.
//! - Keep every duration small. The test runner has a hard timeout.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - `timeout(limit, work).await.ok()` is the whole body of `with_timeout`.
//! - Take the closure as `mut op` in the parameter list.
//! - Loop over `0..attempts` and sleep at the top of the body whenever the index is not `0`. That is easier to get right than sleeping at the bottom.
//! - Return from inside the loop the moment `op()` is true, so no further attempt happens.
//! - Fall through to `None` after the loop.
//!
//! </details>
//!
//! Run: `cargo run --example rf_timeouts_and_sleep`
//! Test: `cargo test --example rf_timeouts_and_sleep`
//! Filter: `cargo test --example rf_timeouts_and_sleep <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::future::Future;
use std::time::Duration;

use tokio::time::{sleep, timeout};

/// Runs `work`, giving up if it takes longer than `limit`.
pub async fn with_timeout<F>(limit: Duration, work: F) -> Option<F::Output>
where
    F: Future,
{
    // TODO
    unimplemented!()
}

/// Calls `op` until it succeeds, waiting `gap` between tries.
pub async fn retry(attempts: usize, gap: Duration, op: impl FnMut() -> bool) -> Option<usize> {
    // TODO
    unimplemented!()
}

// Example usage
#[tokio::main]
pub async fn main() {
    let slow = async {
        sleep(Duration::from_millis(50)).await;
        "finished"
    };

    println!("{:?}", with_timeout(Duration::from_millis(10), slow).await);

    let mut tries = 0;
    let found = retry(5, Duration::from_millis(10), || {
        tries += 1;
        tries == 3
    })
    .await;

    println!("{found:?}");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    use crate::{retry, with_timeout};
    use tokio::time::{Instant, sleep};

    const TICK: Duration = Duration::from_millis(10);

    #[tokio::test(start_paused = true)]
    async fn work_that_finishes_in_time_hands_its_value_back() {
        let value = with_timeout(TICK * 10, async {
            sleep(TICK).await;
            "done"
        })
        .await;

        assert_eq!(value, Some("done"));
    }

    #[tokio::test(start_paused = true)]
    async fn work_that_runs_long_times_out() {
        let value = with_timeout(TICK, async {
            sleep(TICK * 10).await;
            "done"
        })
        .await;

        assert_eq!(value, None);
    }

    #[tokio::test(start_paused = true)]
    async fn an_already_finished_future_never_waits() {
        let start = Instant::now();

        assert_eq!(with_timeout(TICK * 100, async { 7 }).await, Some(7));
        assert!(start.elapsed() < TICK);
    }

    #[tokio::test(start_paused = true)]
    async fn a_timeout_returns_as_soon_as_the_limit_is_up() {
        let start = Instant::now();

        with_timeout(TICK * 2, sleep(TICK * 100)).await;

        assert!(start.elapsed() < TICK * 3);
    }

    #[tokio::test(start_paused = true)]
    async fn a_timed_out_future_is_dropped_before_it_finishes() {
        static REACHED_THE_END: AtomicUsize = AtomicUsize::new(0);

        with_timeout(TICK, async {
            sleep(TICK * 10).await;
            REACHED_THE_END.fetch_add(1, Ordering::SeqCst);
        })
        .await;

        assert_eq!(REACHED_THE_END.load(Ordering::SeqCst), 0);
    }

    #[tokio::test(start_paused = true)]
    async fn a_first_try_success_reports_attempt_zero() {
        let start = Instant::now();

        assert_eq!(retry(5, TICK, || true).await, Some(0));
        assert!(start.elapsed() < TICK);
    }

    #[tokio::test(start_paused = true)]
    async fn a_later_success_reports_its_own_attempt_number() {
        let mut tries = 0;
        let found = retry(5, TICK, || {
            tries += 1;
            tries == 3
        })
        .await;

        assert_eq!(found, Some(2));
    }

    #[tokio::test(start_paused = true)]
    async fn the_gap_is_only_ever_between_attempts() {
        let start = Instant::now();

        let mut tries = 0;
        retry(5, TICK, || {
            tries += 1;
            tries == 3
        })
        .await;

        assert_eq!(start.elapsed(), TICK * 2);
    }

    #[tokio::test(start_paused = true)]
    async fn giving_up_costs_one_gap_less_than_the_attempt_count() {
        let start = Instant::now();

        assert_eq!(retry(4, TICK, || false).await, None);
        assert_eq!(start.elapsed(), TICK * 3);
    }

    #[tokio::test(start_paused = true)]
    async fn zero_attempts_means_op_is_never_called() {
        let mut calls = 0;
        let found = retry(0, TICK, || {
            calls += 1;
            true
        })
        .await;

        assert_eq!(found, None);
        assert_eq!(calls, 0);
    }

    #[tokio::test(start_paused = true)]
    async fn op_is_not_called_again_once_it_succeeds() {
        let mut calls = 0;
        retry(10, TICK, || {
            calls += 1;
            calls == 2
        })
        .await;

        assert_eq!(calls, 2);
    }
}
