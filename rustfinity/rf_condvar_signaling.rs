//! # Rustfinity: Condvar Signaling
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/condvar-signaling
//!
//! A consumer with nothing to do has two bad options: spin in a loop burning CPU, or sleep for a fixed time and add latency. `Condvar` is the third option. It parks the thread until another thread says the state has changed.
//!
//! A condvar is always paired with a mutex. The mutex protects the data; the condvar carries the announcement.
//!
//! ```rust
//! let mut items = self.items.lock().unwrap();
//!
//! while items.is_empty() {
//!     items = self.ready.wait(items).unwrap();
//! }
//! ```
//!
//! `wait` consumes the guard, releases the lock, parks the thread, and hands the guard back re-locked once it wakes. Giving up the lock is essential: the producer needs it to add anything.
//!
//! The `while` is not a stylistic choice, and swapping it for an `if` is the classic bug. `wait` is allowed to return without any matching notification at all, and even a real notification can be stolen by another consumer that reached the lock first. Always re-check the condition after waking.
//!
//! On the producer side:
//!
//! ```rust
//! self.items.lock().unwrap().push_back(item);
//! self.ready.notify_one();
//! ```
//!
//! A notification sent while nobody is waiting is simply lost, which is the other half of why the predicate must be checked before waiting rather than after.
//!
//! `notify_one` wakes a single waiter; `notify_all` wakes every one of them, which you want when the state change could satisfy several different predicates.
//!
//! `wait_timeout` is the version that gives up. It returns the guard plus a `WaitTimeoutResult`, and the predicate still decides the outcome:
//!
//! ```rust
//! let (guard, result) =
//!     self.ready.wait_timeout(items, timeout).unwrap();
//! ```
//!
//! ## Your Task
//!
//! Implement a small blocking queue.
//!
//! ```rust
//! pub struct Queue<T> {
//!     items: Mutex<VecDeque<T>>,
//!     ready: Condvar,
//! }
//! ```
//!
//! The `items` field is given. Add the `Condvar` and implement five methods.
//!
//! ### Implement `new`, `len` and `is_empty`
//!
//! ```rust
//! pub fn new() -> Self
//! pub fn len(&self) -> usize
//! pub fn is_empty(&self) -> bool
//! ```
//!
//! An empty queue, the current number of waiting items, and whether that count is zero.
//!
//! ### Implement `push`
//!
//! ```rust
//! pub fn push(&self, item: T)
//! ```
//!
//! Adds `item` to the back and wakes one waiting consumer. Note the `&self`: pushing takes no `&mut`, because the mutex provides the mutability.
//!
//! ### Implement `pop`
//!
//! ```rust
//! pub fn pop(&self) -> T
//! ```
//!
//! Removes and returns the oldest item, blocking until one exists. Items come out in the order they went in.
//!
//! ### Implement `pop_timeout`
//!
//! ```rust
//! pub fn pop_timeout(&self, timeout: Duration) -> Option<T>
//! ```
//!
//! The same, but returns `None` if nothing arrives within `timeout`.
//!
//! ### Notes
//!
//! - `Condvar::new()` takes no arguments and needs no relationship to the mutex declared up front.
//! - Drop the guard before calling `notify_one`, or the thread you just woke immediately blocks again on the lock you are still holding.
//! - Do not hold the lock across anything slow. The whole queue stalls while you do.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - `push` can be one line plus the notify: `self.items.lock().unwrap().push_back(item);` ends the temporary guard's life at the semicolon.
//! - `pop` is: lock, `while` the deque is empty reassign the guard from `self.ready.wait(guard).unwrap()`, then `pop_front().unwrap()`.
//! - For `pop_timeout`, destructure `let (guard, result) = self.ready.wait_timeout(guard, timeout).unwrap();` and return `None` when the queue is still empty and `result.timed_out()`.
//! - `len` can lock and read; `is_empty` can just call `len`.
//!
//! </details>
//!
//! Run: `cargo run --example rf_condvar_signaling`
//! Test: `cargo test --example rf_condvar_signaling`
//! Filter: `cargo test --example rf_condvar_signaling <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::time::Duration;

/// An unbounded queue that a consumer can block on until work arrives.
#[derive(Debug, Default)]
pub struct Queue<T> {
    // TODO: hold the items behind a Mutex, and add a Condvar to signal on
    items: Mutex<VecDeque<T>>,
}

impl<T> Queue<T> {
    /// Creates an empty queue.
    pub fn new() -> Self {
        // TODO
        unimplemented!()
    }

    /// How many items are waiting.
    pub fn len(&self) -> usize {
        // TODO
        unimplemented!()
    }

    /// Whether the queue is currently empty.
    pub fn is_empty(&self) -> bool {
        // TODO
        unimplemented!()
    }

    /// Adds an item and wakes one waiting consumer.
    pub fn push(&self, item: T) {
        // TODO
        unimplemented!()
    }

    /// Removes the oldest item, blocking until one exists.
    pub fn pop(&self) -> T {
        // TODO: lock, then wait in a loop while the queue is empty
        unimplemented!()
    }

    /// Like `pop`, but gives up after `timeout` and returns `None`.
    pub fn pop_timeout(&self, timeout: Duration) -> Option<T> {
        // TODO: same shape as `pop`, but with `wait_timeout`
        unimplemented!()
    }
}

// Example usage
pub fn main() {
    let queue = Queue::new();
    queue.push("first");
    println!("{}", queue.pop());
    println!("{:?}", queue.pop_timeout(Duration::from_millis(5)));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::thread;
    use std::time::{Duration, Instant};

    mod basic_tests {
        use super::*;

        #[test]
        fn a_new_queue_is_empty() {
            let queue: Queue<u32> = Queue::new();

            assert_eq!(queue.len(), 0);
            assert!(queue.is_empty());
        }

        #[test]
        fn push_then_pop_is_fifo() {
            let queue = Queue::new();
            queue.push(1);
            queue.push(2);
            queue.push(3);

            assert_eq!(queue.len(), 3);
            assert_eq!(queue.pop(), 1);
            assert_eq!(queue.pop(), 2);
            assert_eq!(queue.pop(), 3);
            assert!(queue.is_empty());
        }

        #[test]
        fn works_with_non_copy_items() {
            let queue = Queue::new();
            queue.push(String::from("alpha"));
            queue.push(String::from("beta"));

            assert_eq!(queue.pop(), "alpha");
            assert_eq!(queue.pop(), "beta");
        }
    }

    mod pop_timeout_tests {
        use super::*;

        #[test]
        fn returns_none_on_an_empty_queue() {
            let queue: Queue<u32> = Queue::new();

            assert_eq!(queue.pop_timeout(Duration::from_millis(10)), None);
        }

        #[test]
        fn actually_waits_before_giving_up() {
            let queue: Queue<u32> = Queue::new();
            let started = Instant::now();

            assert_eq!(queue.pop_timeout(Duration::from_millis(40)), None);

            assert!(
                started.elapsed() >= Duration::from_millis(30),
                "pop_timeout returned after {:?}, it must wait for the timeout",
                started.elapsed()
            );
        }

        #[test]
        fn returns_an_item_that_is_already_there() {
            let queue = Queue::new();
            queue.push(9);

            assert_eq!(queue.pop_timeout(Duration::from_millis(10)), Some(9));
        }

        #[test]
        fn wakes_up_when_a_producer_pushes() {
            let queue: Queue<u32> = Queue::new();

            thread::scope(|scope| {
                scope.spawn(|| {
                    thread::sleep(Duration::from_millis(20));
                    queue.push(42);
                });

                assert_eq!(queue.pop_timeout(Duration::from_secs(2)), Some(42));
            });
        }

        #[test]
        fn does_not_lose_a_notification_sent_before_the_wait() {
            // A `notify` with nobody waiting is dropped on the floor. Only a
            // predicate loop that checks the queue *before* waiting survives
            // this ordering.
            let queue: Queue<u32> = Queue::new();
            queue.push(5);
            thread::sleep(Duration::from_millis(10));

            assert_eq!(queue.pop_timeout(Duration::from_millis(50)), Some(5));
        }
    }

    mod blocking_tests {
        use super::*;

        #[test]
        fn pop_blocks_until_an_item_arrives() {
            let queue: Queue<u32> = Queue::new();

            thread::scope(|scope| {
                scope.spawn(|| {
                    thread::sleep(Duration::from_millis(20));
                    queue.push(7);
                });

                assert_eq!(queue.pop(), 7);
            });
        }

        #[test]
        fn four_consumers_split_the_work_with_nothing_lost() {
            let queue: Queue<u64> = Queue::new();

            let totals = thread::scope(|scope| {
                let consumers: Vec<_> = (0..4)
                    .map(|_| {
                        scope.spawn(|| {
                            let mut sum = 0;
                            for _ in 0..25 {
                                sum += queue.pop();
                            }
                            sum
                        })
                    })
                    .collect();

                for value in 1..=100u64 {
                    queue.push(value);
                }

                consumers
                    .into_iter()
                    .map(|handle| handle.join().unwrap())
                    .collect::<Vec<_>>()
            });

            assert_eq!(totals.iter().sum::<u64>(), 5050);
            assert!(queue.is_empty());
        }

        #[test]
        fn many_producers_and_one_consumer() {
            let queue: Queue<u64> = Queue::new();

            let queue = &queue;

            let total = thread::scope(|scope| {
                for producer in 0..5u64 {
                    scope.spawn(move || {
                        for step in 0..10u64 {
                            queue.push(producer * 10 + step);
                        }
                    });
                }

                scope
                    .spawn(move || (0..50).map(|_| queue.pop()).sum::<u64>())
                    .join()
                    .unwrap()
            });

            assert_eq!(total, (0..50u64).sum::<u64>());
        }
    }
}
