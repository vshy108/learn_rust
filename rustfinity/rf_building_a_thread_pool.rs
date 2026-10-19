//! # Rustfinity: Building A Thread Pool
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/building-a-thread-pool
//!
//! Spawning a thread per task works until there are a lot of tasks. Each `thread::spawn` asks the OS for a real thread with its own stack, and for work measured in microseconds the spawn costs more than the job. A **thread pool** pays that cost once: a fixed number of workers start up, then take jobs off a shared queue for as long as the pool lives.
//!
//! A job is a boxed closure:
//!
//! ```rust
//! type Job = Box<dyn FnOnce() + Send + 'static>;
//! ```
//!
//! `FnOnce` because it runs once, `Send` because it crosses into another thread, `'static` because the pool cannot prove the job finishes before any borrow it captured expires.
//!
//! The queue is an `mpsc` channel. Many threads send, but a `Receiver` cannot be cloned, so the workers share one behind a lock:
//!
//! ```rust
//! let receiver = Arc::new(Mutex::new(receiver));
//! ```
//!
//! Each worker loops: take the lock, pull one job, release the lock, run the job.
//!
//! ```rust
//! loop {
//!     let job = receiver.lock().unwrap().recv();
//!
//!     match job {
//!         Ok(job) => job(),
//!         Err(_) => break,
//!     }
//! }
//! ```
//!
//! The line to stare at is `let job = ...recv();`. The temporary guard is dropped at the end of that statement, before `job()` runs, so the other workers can pull their own jobs while this one is busy. Write it as `while let Ok(job) = receiver.lock().unwrap().recv()` and the guard lives for the whole body: your pool has as many threads as you asked for and exactly one of them ever works.
//!
//! Shutdown is the other half. `recv` returns `Err` when every `Sender` is gone, which is the signal for a worker to break out of its loop. So `Drop` has to drop the pool's sender *before* joining:
//!
//! ```rust
//! fn drop(&mut self) {
//!     drop(self.sender.take());
//!
//!     for worker in &mut self.workers {
//!         if let Some(handle) = worker.take() {
//!             handle.join().unwrap();
//!         }
//!     }
//! }
//! ```
//!
//! Both fields are `Option` because `drop` only gets `&mut self` and cannot move out of it. `take()` swaps in `None` and hands you the value. Join first and drop second and you wait forever on workers that are still blocked on `recv`.
//!
//! ## Your Task
//!
//! Build `ThreadPool`.
//!
//! ### Implement `new`
//!
//! ```rust
//! pub fn new(size: usize) -> ThreadPool
//! ```
//!
//! Start `size` worker threads sharing one job queue. Panic if `size` is `0`: a pool with no workers would accept jobs and silently never run them.
//!
//! ### Implement `execute`
//!
//! ```rust
//! pub fn execute<F>(&self, job: F)
//! where
//!     F: FnOnce() + Send + 'static,
//! ```
//!
//! Box the job and put it on the queue. Note the `&self`: submitting work needs no exclusive access.
//!
//! ### Implement `size`
//!
//! ```rust
//! pub fn size(&self) -> usize
//! ```
//!
//! How many workers the pool has.
//!
//! ### Implement `Drop` for the pool
//!
//! Close the queue, then join every worker. When the pool is dropped, every job already queued must have finished.
//!
//! ```rust
//! let counter = Arc::new(AtomicUsize::new(0));
//!
//! {
//!     let pool = ThreadPool::new(4);
//!
//!     for _ in 0..20 {
//!         let counter = Arc::clone(&counter);
//!         pool.execute(move || {
//!             counter.fetch_add(1, Ordering::Relaxed);
//!         });
//!     }
//! }
//!
//! assert_eq!(counter.load(Ordering::Relaxed), 20);
//! ```
//!
//! ### Notes
//!
//! - Jobs run on `size` threads at most, and all `size` of them must be able to run at the same time.
//! - A pool of one worker still has to run everything, just one at a time.
//! - The pool can be reused for as many batches as you like.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - `struct ThreadPool { sender: Option<Sender<Job>>, workers: Vec<Option<JoinHandle<()>>> }`.
//! - `Arc::clone(&receiver)` once per worker, before the `move` closure.
//! - Bind the received job to a `let` on its own line so the mutex guard is released before the job runs.
//! - `assert!(size > 0, "...")` at the top of `new`.
//! - In `Drop`, `self.sender.take()` returns the `Option`; dropping it disconnects the channel.
//! - `for worker in &mut self.workers` plus `worker.take()` gives you an owned `JoinHandle` to join.
//!
//! </details>
//!
//! Run: `cargo run --example rf_building_a_thread_pool`
//! Test: `cargo test --example rf_building_a_thread_pool`
//! Filter: `cargo test --example rf_building_a_thread_pool <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

/// A unit of work handed to the pool.
type Job = Box<dyn FnOnce() + Send + 'static>;

/// A fixed size pool of worker threads sharing one job queue.
pub struct ThreadPool {
    // TODO: the sending half of the job queue, and the worker handles.
    // Both need to be taken out again in `drop`.
}

impl ThreadPool {
    /// Creates a pool with `size` worker threads.
    ///
    /// # Panics
    ///
    /// Panics if `size` is 0.
    pub fn new(size: usize) -> ThreadPool {
        // TODO: one channel, an Arc<Mutex<Receiver<Job>>> shared by every
        // worker, and a loop in each worker that pulls jobs until the
        // channel disconnects
        unimplemented!()
    }

    /// Queues a job for the next free worker.
    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // TODO
        unimplemented!()
    }

    /// The number of worker threads in the pool.
    pub fn size(&self) -> usize {
        // TODO
        unimplemented!()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // TODO: close the queue first, then join every worker
    }
}

// Example usage
pub fn main() {
    let pool = ThreadPool::new(3);

    for id in 0..6 {
        pool.execute(move || println!("job {id}"));
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use crate::ThreadPool;
    use std::collections::HashSet;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Condvar, Mutex};
    use std::time::Duration;

    #[test]
    fn a_pool_reports_its_size() {
        assert_eq!(ThreadPool::new(1).size(), 1);
        assert_eq!(ThreadPool::new(7).size(), 7);
    }

    #[test]
    #[should_panic]
    fn a_pool_with_no_workers_is_rejected() {
        ThreadPool::new(0);
    }

    #[test]
    fn a_single_job_runs() {
        let flag = Arc::new(AtomicUsize::new(0));

        {
            let pool = ThreadPool::new(2);
            let flag = Arc::clone(&flag);
            pool.execute(move || {
                flag.store(1, Ordering::SeqCst);
            });
        }

        assert_eq!(flag.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn an_unused_pool_shuts_down_cleanly() {
        let pool = ThreadPool::new(4);
        drop(pool);
    }

    #[test]
    fn dropping_the_pool_waits_for_every_queued_job() {
        let done = Arc::new(AtomicUsize::new(0));

        {
            let pool = ThreadPool::new(4);

            for _ in 0..200 {
                let done = Arc::clone(&done);
                pool.execute(move || {
                    done.fetch_add(1, Ordering::SeqCst);
                });
            }
        } // drop joins the workers

        assert_eq!(
            done.load(Ordering::SeqCst),
            200,
            "drop must finish the queue before returning"
        );
    }

    #[test]
    fn jobs_can_move_owned_values_in() {
        let results = Arc::new(Mutex::new(Vec::new()));

        {
            let pool = ThreadPool::new(3);

            for id in 0..10u64 {
                let results = Arc::clone(&results);
                let label = format!("job-{id}");

                pool.execute(move || {
                    results.lock().unwrap().push((id, label));
                });
            }
        }

        let mut collected = Arc::try_unwrap(results).unwrap().into_inner().unwrap();
        collected.sort();

        let expected: Vec<(u64, String)> = (0..10u64).map(|id| (id, format!("job-{id}"))).collect();

        assert_eq!(collected, expected);
    }

    #[test]
    fn work_is_spread_over_at_most_size_threads() {
        let seen: Arc<Mutex<HashSet<std::thread::ThreadId>>> = Arc::new(Mutex::new(HashSet::new()));

        {
            let pool = ThreadPool::new(3);

            for _ in 0..60 {
                let seen = Arc::clone(&seen);
                pool.execute(move || {
                    seen.lock().unwrap().insert(std::thread::current().id());
                });
            }
        }

        let count = seen.lock().unwrap().len();

        assert!(
            count <= 3,
            "a pool of 3 must not run jobs on {count} different threads"
        );
        assert!(count >= 1, "the jobs never ran");
    }

    #[test]
    fn every_worker_can_run_at_the_same_time() {
        // Four jobs on a pool of four. Each one waits for the other three to
        // arrive, so this only completes if the workers really do run
        // concurrently. The timeout keeps a broken pool from hanging.
        let size = 4;
        let gate = Arc::new((Mutex::new(0usize), Condvar::new()));
        let all_arrived = Arc::new(AtomicUsize::new(0));

        {
            let pool = ThreadPool::new(size);

            for _ in 0..size {
                let gate = Arc::clone(&gate);
                let all_arrived = Arc::clone(&all_arrived);

                pool.execute(move || {
                    let (lock, condvar) = &*gate;
                    let mut arrived = lock.lock().unwrap();

                    *arrived += 1;
                    condvar.notify_all();

                    let deadline = Duration::from_secs(2);
                    while *arrived < size {
                        let (guard, timeout) = condvar.wait_timeout(arrived, deadline).unwrap();
                        arrived = guard;

                        if timeout.timed_out() {
                            return;
                        }
                    }

                    all_arrived.fetch_add(1, Ordering::SeqCst);
                });
            }
        }

        assert_eq!(
            all_arrived.load(Ordering::SeqCst),
            size,
            "all {size} workers should have been running together"
        );
    }

    #[test]
    fn a_pool_of_one_still_runs_everything() {
        let total = Arc::new(AtomicUsize::new(0));

        {
            let pool = ThreadPool::new(1);

            for value in 1..=50 {
                let total = Arc::clone(&total);
                pool.execute(move || {
                    total.fetch_add(value, Ordering::SeqCst);
                });
            }
        }

        assert_eq!(total.load(Ordering::SeqCst), (1..=50).sum::<usize>());
    }

    #[test]
    fn a_pool_can_be_reused_for_several_batches() {
        let total = Arc::new(AtomicUsize::new(0));
        let pool = ThreadPool::new(3);

        for _ in 0..5 {
            for _ in 0..20 {
                let total = Arc::clone(&total);
                pool.execute(move || {
                    total.fetch_add(1, Ordering::SeqCst);
                });
            }
        }

        drop(pool);

        assert_eq!(total.load(Ordering::SeqCst), 100);
    }
}
