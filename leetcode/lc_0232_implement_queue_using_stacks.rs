// LeetCode 232: Implement Queue using Stacks [Easy]
// https://leetcode.com/problems/implement-queue-using-stacks/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.962us
//   AI solution avg: 0.348us
//   Ratio: AI 2.77x faster
//
// Implement a FIFO queue using only two stacks. The implemented queue
// should support push, peek, pop, and empty.
//
// void push(int x) — Pushes element x to the back of the queue.
// int pop() — Removes and returns the front element.
// int peek() — Returns the front element.
// boolean empty() — Returns true if the queue is empty.
//
// Constraints: 1 <= x <= 9, At most 100 calls total.
// All calls to pop and peek are valid.
//
// Run:  cargo run --example lc_0232_implement_queue_using_stacks
// Test: cargo test --example lc_0232_implement_queue_using_stacks
// Bench: cargo test --example lc_0232_implement_queue_using_stacks bench -- --nocapture

/// My solution — eager transfer on push.
///
/// Use two stacks: `input` and `output`. On push, move everything from
/// `output` to `input`, push the new element, then move everything back.
/// This makes push O(n) but pop/peek O(1).
///
/// Time: push O(n), pop O(1), peek O(1).
/// Space: O(n).
mod my_solution {
    /// Queue backed by two internal stacks, with O(n) push.
    pub struct MyQueue {
        input: Vec<i32>,
        output: Vec<i32>,
    }

    impl MyQueue {
        /// Create a new empty queue.
        pub fn new() -> Self {
            Self {
                input: Vec::new(),
                output: Vec::new(),
            }
        }

        /// Push element x to the back of the queue.
        pub fn push(&mut self, x: i32) {
            // Transfer all elements from output to input (reverse order)
            while let Some(val) = self.output.pop() {
                self.input.push(val);
            }
            // Push new element onto input
            self.input.push(x);
            // Transfer everything back to output (FIFO order restored)
            while let Some(val) = self.input.pop() {
                self.output.push(val);
            }
        }

        /// Remove and return the front element of the queue.
        pub fn pop(&mut self) -> i32 {
            self.output.pop().expect("pop called on non-empty queue")
        }

        /// Return the front element without removing it.
        pub fn peek(&self) -> i32 {
            *self.output.last().expect("peek called on non-empty queue")
        }

        /// Check whether the queue is empty.
        pub fn empty(&self) -> bool {
            self.output.is_empty()
        }
    }
}

/// AI solution — lazy transfer (amortized O(1) pop/peek).
///
/// Use two stacks: `input` for pushes, `output` for pops. Only transfer
/// from `input` to `output` when `output` is empty and we need to
/// pop/peek. Each element is moved at most twice total, giving amortized
/// O(1) per operation.
///
/// Time: push O(1), pop amortized O(1), peek amortized O(1).
/// Space: O(n).
mod ai_solution {
    /// Queue backed by two internal stacks, with amortized O(1) operations.
    pub struct MyQueue {
        input: Vec<i32>,
        output: Vec<i32>,
    }

    impl MyQueue {
        /// Create a new empty queue.
        pub fn new() -> Self {
            Self {
                input: Vec::new(),
                output: Vec::new(),
            }
        }

        /// Push element x to the back of the queue — just push to input.
        pub fn push(&mut self, x: i32) {
            self.input.push(x);
        }

        /// Transfer all elements from input to output (reversing the order)
        /// only when output is empty. This ensures FIFO ordering.
        fn transfer_if_needed(&mut self) {
            if self.output.is_empty() {
                while let Some(val) = self.input.pop() {
                    self.output.push(val);
                }
            }
        }

        /// Remove and return the front element of the queue.
        pub fn pop(&mut self) -> i32 {
            self.transfer_if_needed();
            self.output.pop().expect("pop called on non-empty queue")
        }

        /// Return the front element without removing it.
        pub fn peek(&mut self) -> i32 {
            self.transfer_if_needed();
            *self.output.last().expect("peek called on non-empty queue")
        }

        /// Check whether the queue is empty.
        pub fn empty(&self) -> bool {
            self.input.is_empty() && self.output.is_empty()
        }
    }
}

fn main() {
    let mut my_q = my_solution::MyQueue::new();
    my_q.push(1);
    my_q.push(2);
    println!("My peek: {} (expected 1)", my_q.peek());
    println!("My pop: {} (expected 1)", my_q.pop());
    println!("My empty: {} (expected false)", my_q.empty());

    let mut ai_q = ai_solution::MyQueue::new();
    ai_q.push(1);
    ai_q.push(2);
    println!("AI peek: {} (expected 1)", ai_q.peek());
    println!("AI pop: {} (expected 1)", ai_q.pop());
    println!("AI empty: {} (expected false)", ai_q.empty());
}

#[cfg(test)]
mod tests {
    macro_rules! test_queue {
        ($mod_name:ident, $queue_type:ty) => {
            mod $mod_name {
                #[test]
                fn test_example() {
                    let mut q = <$queue_type>::new();
                    q.push(1);
                    q.push(2);
                    assert_eq!(q.peek(), 1);
                    assert_eq!(q.pop(), 1);
                    assert!(!q.empty());
                }

                #[test]
                fn test_fifo_order() {
                    let mut q = <$queue_type>::new();
                    q.push(1);
                    q.push(2);
                    q.push(3);
                    assert_eq!(q.pop(), 1);
                    assert_eq!(q.pop(), 2);
                    assert_eq!(q.pop(), 3);
                    assert!(q.empty());
                }

                #[test]
                fn test_interleaved() {
                    let mut q = <$queue_type>::new();
                    q.push(1);
                    q.push(2);
                    assert_eq!(q.pop(), 1);
                    q.push(3);
                    assert_eq!(q.peek(), 2);
                    assert_eq!(q.pop(), 2);
                    assert_eq!(q.pop(), 3);
                    assert!(q.empty());
                }

                #[test]
                fn test_single() {
                    let mut q = <$queue_type>::new();
                    q.push(5);
                    assert_eq!(q.peek(), 5);
                    assert_eq!(q.pop(), 5);
                    assert!(q.empty());
                }

                #[test]
                fn test_peek_idempotent() {
                    let mut q = <$queue_type>::new();
                    q.push(7);
                    assert_eq!(q.peek(), 7);
                    assert_eq!(q.peek(), 7);
                    assert!(!q.empty());
                }
            }
        };
    }

    test_queue!(my, crate::my_solution::MyQueue);
    test_queue!(ai, crate::ai_solution::MyQueue);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut q = crate::my_solution::MyQueue::new();
            for x in 0..20 {
                q.push(x);
            }
            for _ in 0..20 {
                std::hint::black_box(q.pop());
            }
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut q = crate::ai_solution::MyQueue::new();
            for x in 0..20 {
                q.push(x);
            }
            for _ in 0..20 {
                std::hint::black_box(q.pop());
            }
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else if ai_avg < my_avg {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        } else {
            println!("  Ratio: ~tied");
        }
    }
}
