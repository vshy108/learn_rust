//! # Rustfinity: Joining Futures
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/joining-futures
//!
//! Three lookups, awaited one after the other:
//!
//! ```rust
//! let name = load_name(id).await;
//! let email = load_email(id).await;
//! let posts = load_posts(id).await;
//! ```
//!
//! If each takes 40ms this takes 120ms, and for 80 of those 120 milliseconds the program is sitting on its hands. The three lookups do not depend on each other, so they should be waiting at the same time.
//!
//! `tokio::join!` does exactly that:
//!
//! ```rust
//! let (name, email, posts) = tokio::join!(
//!     load_name(id),
//!     load_email(id),
//!     load_posts(id),
//! );
//! ```
//!
//! Now the call takes 40ms. `join!` takes the futures, polls each one whenever it can make progress, and returns a tuple of their outputs in the order you listed them, once all of them are done.
//!
//! ## `join!` is not `spawn`
//!
//! `tokio::spawn` hands a future to the runtime as an independent task. `join!` keeps everything on the current task and just interleaves the polling. That has real consequences:
//!
//! - Nothing is moved to another thread, so the futures do not have to be `Send` or `'static`. They can borrow local variables.
//! - There is no `JoinHandle` and no `Result` to unwrap.
//! - It is concurrency, not parallelism. Two CPU-bound futures under `join!` will not use two cores.
//!
//! Reach for `join!` when you have a fixed, known set of things to wait on. Reach for `spawn` when the work should outlive the current scope or genuinely needs its own thread.
//!
//! ## Failing fast with `try_join!`
//!
//! `join!` waits for everything, even after one branch has already returned an `Err`. When every branch returns a `Result` and one failure makes the rest pointless, use `try_join!`:
//!
//! ```rust
//! let (a, b) = tokio::try_join!(load_a(), load_b())?;
//! ```
//!
//! It returns `Ok((a, b))` when both succeeded, or the first `Err` it sees, dropping the other future mid-flight. Both branches must have the same error type.
//!
//! ## Your Task
//!
//! Five lookup functions are already written for you: `load_name`, `load_email`, `load_posts`, `load_quota` and `load_rate_limit`. Each one sleeps for 40ms. Do not change them.
//!
//! ### Implement `load_profile`
//!
//! ```rust
//! pub async fn load_profile(id: u32) -> Profile
//! ```
//!
//! Run `load_name`, `load_email` and `load_posts` concurrently and put the three results into a `Profile`. The call must take about 40ms, not 120ms.
//!
//! ### Implement `load_limits`
//!
//! ```rust
//! pub async fn load_limits(
//!     id: u32,
//! ) -> Result<(u32, u32), String>
//! ```
//!
//! Run `load_quota` and `load_rate_limit` concurrently. Return `Ok((quota, rate_limit))` if both succeed, or the error if either fails.
//!
//! ```rust
//! assert_eq!(load_limits(3).await, Ok((300, 60)));
//! assert_eq!(
//!     load_limits(0).await,
//!     Err("no quota for user 0".to_string())
//! );
//! ```
//!
//! ### Notes
//!
//! - Do not `spawn` anything. Both functions are a single macro call plus a little plumbing.
//! - Build the future values inside the macro, not before it. `join!` needs to own them.
//! - The tuple comes back in the order you listed the futures, not the order they finished.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - `let (name, email, posts) = tokio::join!(...)` destructures straight into the three bindings.
//! - `Profile { name, email, posts }` then uses field init shorthand.
//! - `tokio::try_join!(load_quota(id), load_rate_limit(id))` already has the exact return type `load_limits` needs, so it can be the last expression in the body.
//! - If the timing test fails, look for a stray `.await` before the macro.
//!
//! </details>
//!
//! Run: `cargo run --example rf_joining_futures`
//! Test: `cargo test --example rf_joining_futures`
//! Filter: `cargo test --example rf_joining_futures <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::time::Duration;
use tokio::time::sleep;

/// A user profile assembled from three separate lookups.
#[derive(Debug, PartialEq, Eq)]
pub struct Profile {
    pub name: String,
    pub email: String,
    pub posts: usize,
}

/// Pretend network lookup. Provided for you.
pub async fn load_name(id: u32) -> String {
    sleep(Duration::from_millis(40)).await;
    format!("user{id}")
}

/// Pretend network lookup. Provided for you.
pub async fn load_email(id: u32) -> String {
    sleep(Duration::from_millis(40)).await;
    format!("user{id}@example.com")
}

/// Pretend network lookup. Provided for you.
pub async fn load_posts(id: u32) -> usize {
    sleep(Duration::from_millis(40)).await;
    id as usize * 3
}

/// Pretend network lookup that can fail. Provided for you.
pub async fn load_quota(id: u32) -> Result<u32, String> {
    sleep(Duration::from_millis(40)).await;

    if id == 0 {
        Err("no quota for user 0".to_string())
    } else {
        Ok(id * 100)
    }
}

/// Pretend network lookup that can fail. Provided for you.
pub async fn load_rate_limit(id: u32) -> Result<u32, String> {
    sleep(Duration::from_millis(40)).await;

    if id > 1000 {
        Err("unknown user".to_string())
    } else {
        Ok(60)
    }
}

/// Loads all three fields concurrently.
pub async fn load_profile(id: u32) -> Profile {
    // TODO
    unimplemented!()
}

/// Loads both limits concurrently, giving up as soon as one fails.
pub async fn load_limits(id: u32) -> Result<(u32, u32), String> {
    // TODO
    unimplemented!()
}

// Example usage
#[tokio::main]
pub async fn main() {
    println!("{:?}", load_profile(7).await);
    println!("{:?}", load_limits(0).await);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use crate::{Profile, load_limits, load_profile};
    use std::time::{Duration, Instant};

    #[tokio::test(start_paused = true)]
    async fn a_profile_has_every_field_filled_in() {
        assert_eq!(
            load_profile(7).await,
            Profile {
                name: "user7".to_string(),
                email: "user7@example.com".to_string(),
                posts: 21,
            }
        );
    }

    #[tokio::test(start_paused = true)]
    async fn a_different_id_gives_different_data() {
        assert_eq!(
            load_profile(2).await,
            Profile {
                name: "user2".to_string(),
                email: "user2@example.com".to_string(),
                posts: 6,
            }
        );
    }

    #[tokio::test(start_paused = true)]
    async fn the_three_lookups_overlap() {
        let start = Instant::now();

        load_profile(1).await;

        let elapsed = start.elapsed();

        assert!(
            elapsed < Duration::from_millis(80),
            "three 40ms lookups took {elapsed:?}: they ran one after another. \
         Await them together instead of one at a time"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn limits_are_returned_in_order() {
        assert_eq!(load_limits(3).await, Ok((300, 60)));
        assert_eq!(load_limits(1).await, Ok((100, 60)));
    }

    #[tokio::test(start_paused = true)]
    async fn a_failing_quota_fails_the_whole_call() {
        assert_eq!(load_limits(0).await, Err("no quota for user 0".to_string()));
    }

    #[tokio::test(start_paused = true)]
    async fn a_failing_rate_limit_fails_the_whole_call() {
        assert_eq!(load_limits(5000).await, Err("unknown user".to_string()));
    }

    #[tokio::test(start_paused = true)]
    async fn the_two_limit_lookups_overlap_too() {
        let start = Instant::now();

        let _ = load_limits(9).await;

        let elapsed = start.elapsed();

        assert!(
            elapsed < Duration::from_millis(70),
            "two 40ms lookups took {elapsed:?}: run them concurrently"
        );
    }
}
