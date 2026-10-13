//! # Rustfinity: Joinset Concurrent Tasks
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/joinset-concurrent-tasks
//!
//! `tokio::join!` needs to know its futures at compile time. `Vec<JoinHandle<T>>` handles a dynamic count, but awaiting it in order means a slow first task hides five fast ones that already finished.
//!
//! `JoinSet` is the collection for the general case: spawn as many tasks as you like, whenever you like, and pull results out in the order they complete.
//!
//! ```rust
//! use tokio::task::JoinSet;
//!
//! let mut set = JoinSet::new();
//! set.spawn(work(1));
//! set.spawn(work(2));
//!
//! while let Some(res) = set.join_next().await {
//!     println!("{}", res.unwrap());
//! }
//! ```
//!
//! `join_next` returns `None` when the set is empty, so the `while let` ends on its own. Each item is a `Result`, because a task can panic or be aborted.
//!
//! Two more things it gives you:
//!
//! - `abort_all()` cancels every task still running.
//! - Dropping the set aborts them too, so a `?` on an early return does not leak tasks.
//!
//! ## Bounded concurrency
//!
//! Because you can spawn into a set that is already running, `JoinSet` is how you cap how much runs at once: prime it with `limit` tasks, then start one more each time `join_next` hands you a result.
//!
//! ## Your Task
//!
//! `work` is written for you. It sleeps for `id` milliseconds and returns `id * 10`, so a smaller id always finishes first.
//!
//! Implement three functions:
//!
//! - `async results_in_completion_order(ids: Vec<u64>) -> Vec<u64>`, everything at once, results in finishing order
//! - `async first_n_to_finish(ids: Vec<u64>, n: usize) -> Vec<u64>`, the first `n` results, with the rest abandoned rather than awaited
//! - `async with_limit(ids: Vec<u64>, limit: usize) -> Vec<u64>`, at most `limit` tasks in flight, results in finishing order
//!
//! So `results_in_completion_order(vec![30, 10, 20])` is `[100, 200, 300]`, and `with_limit(vec![50, 10, 25, 5], 2)` is `[100, 250, 50, 500]`.
//!
//! ### Notes
//!
//! - `first_n_to_finish` must return as soon as it has `n` results. Waiting for the slow ones first and truncating afterwards is not the same thing.
//! - Handle `n` larger than the number of ids, and `n` of zero.
//! - Assume `limit` is at least one.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - `set.spawn(work(id))` takes the future itself, not a closure.
//! - `while let Some(result) = set.join_next().await` is the whole drain loop.
//! - For `first_n_to_finish`, loop while `results.len() < n` and break when `join_next` gives `None`.
//! - For `with_limit`, keep the remaining ids in an iterator: `ids.into_iter()`. Prime the set with `by_ref().take(limit)`, then call `next()` on it after each completed result.
//!
//! </details>
//!
//! Run: `cargo run --example rf_joinset_concurrent_tasks`
//! Test: `cargo test --example rf_joinset_concurrent_tasks`
//! Filter: `cargo test --example rf_joinset_concurrent_tasks <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::time::Duration;

/// Stands in for a unit of work whose duration you do not control.
///
/// This one is written for you. It sleeps for `id` milliseconds and
/// then returns `id * 10`, so a smaller id always finishes first.
pub async fn work(id: u64) -> u64 {
    tokio::time::sleep(Duration::from_millis(id)).await;
    id * 10
}

/// Runs every id at once and returns the results in the order they
/// finished, not the order they were spawned.
pub async fn results_in_completion_order(ids: Vec<u64>) -> Vec<u64> {
    // TODO
    unimplemented!()
}

/// Runs every id at once, keeps the first `n` results, and abandons the
/// rest.
pub async fn first_n_to_finish(ids: Vec<u64>, n: usize) -> Vec<u64> {
    // TODO
    unimplemented!()
}

/// Runs the ids with at most `limit` in flight at a time, starting the
/// next one every time a task finishes. Results come back in completion
/// order.
pub async fn with_limit(ids: Vec<u64>, limit: usize) -> Vec<u64> {
    // TODO
    unimplemented!()
}

// Example usage
#[tokio::main]
pub async fn main() {
    println!("{:?}", results_in_completion_order(vec![30, 10, 20]).await);
    println!("{:?}", first_n_to_finish(vec![50, 10, 20], 2).await);
    println!("{:?}", with_limit(vec![50, 10, 25, 5], 2).await);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::time::Duration;

    use crate::{first_n_to_finish, results_in_completion_order, with_limit};
    use tokio::time::Instant;

    const MS: Duration = Duration::from_millis(1);

    #[tokio::test(start_paused = true)]
    async fn no_ids_means_no_results() {
        assert_eq!(results_in_completion_order(Vec::new()).await, Vec::new());
        assert_eq!(first_n_to_finish(Vec::new(), 3).await, Vec::new());
        assert_eq!(with_limit(Vec::new(), 2).await, Vec::new());
    }

    #[tokio::test(start_paused = true)]
    async fn one_id_gives_one_result() {
        assert_eq!(results_in_completion_order(vec![7]).await, vec![70]);
    }

    #[tokio::test(start_paused = true)]
    async fn results_arrive_shortest_first() {
        assert_eq!(
            results_in_completion_order(vec![30, 10, 20]).await,
            vec![100, 200, 300]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn the_spawn_order_does_not_decide_the_result_order() {
        assert_eq!(
            results_in_completion_order(vec![50, 40, 30, 20, 10]).await,
            vec![100, 200, 300, 400, 500]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn everything_runs_at_once_so_the_slowest_sets_the_pace() {
        let start = Instant::now();
        results_in_completion_order(vec![10, 20, 30]).await;

        assert_eq!(start.elapsed(), MS * 30);
    }

    #[tokio::test(start_paused = true)]
    async fn taking_two_gives_the_two_quickest() {
        assert_eq!(first_n_to_finish(vec![50, 10, 20], 2).await, vec![100, 200]);
    }

    #[tokio::test(start_paused = true)]
    async fn the_stragglers_are_not_waited_for() {
        let start = Instant::now();
        first_n_to_finish(vec![10, 20, 900], 2).await;

        assert_eq!(start.elapsed(), MS * 20);
    }

    #[tokio::test(start_paused = true)]
    async fn asking_for_more_than_exists_returns_everything() {
        assert_eq!(first_n_to_finish(vec![20, 10], 9).await, vec![100, 200]);
    }

    #[tokio::test(start_paused = true)]
    async fn asking_for_none_costs_nothing() {
        let start = Instant::now();

        assert_eq!(first_n_to_finish(vec![10, 20], 0).await, Vec::new());
        assert_eq!(start.elapsed(), Duration::ZERO);
    }

    #[tokio::test(start_paused = true)]
    async fn a_limit_of_one_runs_them_one_after_another() {
        let start = Instant::now();
        let results = with_limit(vec![20, 30, 40], 1).await;

        assert_eq!(results, vec![200, 300, 400]);
        assert_eq!(start.elapsed(), MS * 90);
    }

    #[tokio::test(start_paused = true)]
    async fn a_limit_wider_than_the_work_runs_it_all_at_once() {
        let start = Instant::now();
        let results = with_limit(vec![30, 10, 20], 10).await;

        assert_eq!(results, vec![100, 200, 300]);
        assert_eq!(start.elapsed(), MS * 30);
    }

    #[tokio::test(start_paused = true)]
    async fn a_finished_task_makes_room_for_the_next_one() {
        let start = Instant::now();
        let results = with_limit(vec![50, 10, 25, 5], 2).await;

        assert_eq!(results, vec![100, 250, 50, 500]);
        assert_eq!(start.elapsed(), MS * 50);
    }

    #[tokio::test(start_paused = true)]
    async fn the_limit_holds_for_a_long_queue() {
        let start = Instant::now();
        let results = with_limit(vec![10, 20, 30, 40, 50, 60], 2).await;

        assert_eq!(results, vec![100, 200, 300, 400, 500, 600]);
        assert_eq!(start.elapsed(), MS * 120);
    }
}
