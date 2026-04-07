// LeetCode 225: Implement Stack using Queues [Easy]
// https://leetcode.com/problems/implement-stack-using-queues/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.626us
//   AI solution avg: 0.504us
//   Ratio: AI 1.24x faster
//
// Implement a last-in-first-out (LIFO) stack using only two queues.
// The implemented stack should support push, top, pop, and empty.
//
// void push(int x) — Pushes element x to the top of the stack.
// int pop() — Removes and returns the element on the top.
// int top() — Returns the element on the top.
// boolean empty() — Returns true if the stack is empty.
//
// You must use only standard queue operations: push to back, peek/pop
// from front, size, and is empty.
//
// Constraints: 1 <= x <= 9, At most 100 calls to push, pop, top, empty.
// All calls to pop and top are valid.
//
// Run:  cargo run --example lc_0225_implement_stack_using_queues
// Test: cargo test --example lc_0225_implement_stack_using_queues
// Bench: cargo test --example lc_0225_implement_stack_using_queues bench -- --nocapture

/// My solution — costly push approach.
///
/// Use a single VecDeque as a queue. On push, add the element to the back,
/// then rotate all previous elements behind it (pop from front, push to back).
/// This makes push O(n) but pop/top O(1).
///
/// Time: push O(n), pop O(1), top O(1).
/// Space: O(n) for the queue.
mod my_solution {
    use std::collections::VecDeque;

    /// Stack built on top of a single queue, with O(n) push.
    pub struct MyStack {
        queue: VecDeque<i32>,
    }

    impl MyStack {
        /// Create a new empty stack.
        pub fn new() -> Self {
            Self {
                queue: VecDeque::new(),
            }
        }

        /// Push element x onto the stack.
        ///
        /// Add to back, then rotate (n-1) elements from front to back
        /// so the newest element ends up at the front (LIFO order).
        pub fn push(&mut self, x: i32) {
            self.queue.push_back(x);
            // Rotate the older elements behind the new one
            let rotations = self.queue.len() - 1;
            for _ in 0..rotations {
                let front = self.queue.pop_front().expect("queue non-empty during rotation");
                self.queue.push_back(front);
            }
        }

        /// Remove and return the top element.
        pub fn pop(&mut self) -> i32 {
            self.queue.pop_front().expect("pop called on non-empty stack")
        }

        /// Return the top element without removing it.
        pub fn top(&self) -> i32 {
            *self.queue.front().expect("top called on non-empty stack")
        }

        /// Check whether the stack is empty.
        pub fn empty(&self) -> bool {
            self.queue.is_empty()
        }
    }
}

/// AI solution — costly pop approach.
///
/// Use a single VecDeque. Push is O(1) — just push to back. Pop rotates
/// all elements except the last one from front to back, then pops the
/// front (which was the last pushed element). This makes push O(1) but
/// pop O(n).
///
/// Time: push O(1), pop O(n), top O(n).
/// Space: O(n) for the queue.
mod ai_solution {
    use std::collections::VecDeque;

    /// Stack built on top of a single queue, with O(1) push and O(n) pop.
    pub struct MyStack {
        queue: VecDeque<i32>,
    }

    impl MyStack {
        /// Create a new empty stack.
        pub fn new() -> Self {
            Self {
                queue: VecDeque::new(),
            }
        }

        /// Push element x onto the stack — just enqueue.
        pub fn push(&mut self, x: i32) {
            self.queue.push_back(x);
        }

        /// Remove and return the top element.
        ///
        /// Rotate (n-1) elements from front to back so the last-pushed
        /// element (the "top") is now at the front, then dequeue it.
        pub fn pop(&mut self) -> i32 {
            let rotations = self.queue.len() - 1;
            for _ in 0..rotations {
                let front = self.queue.pop_front().expect("queue non-empty during rotation");
                self.queue.push_back(front);
            }
            self.queue.pop_front().expect("pop called on non-empty stack")
        }

        /// Return the top element without removing it.
        ///
        /// Same rotation as pop, but push the element back after reading it.
        pub fn top(&self) -> i32 {
            // The last element in the queue is the stack top
            *self.queue.back().expect("top called on non-empty stack")
        }

        /// Check whether the stack is empty.
        pub fn empty(&self) -> bool {
            self.queue.is_empty()
        }
    }
}

fn main() {
    let mut my_stack = my_solution::MyStack::new();
    my_stack.push(1);
    my_stack.push(2);
    println!("My top: {} (expected 2)", my_stack.top());
    println!("My pop: {} (expected 2)", my_stack.pop());
    println!("My empty: {} (expected false)", my_stack.empty());

    let mut ai_stack = ai_solution::MyStack::new();
    ai_stack.push(1);
    ai_stack.push(2);
    println!("AI top: {} (expected 2)", ai_stack.top());
    println!("AI pop: {} (expected 2)", ai_stack.pop());
    println!("AI empty: {} (expected false)", ai_stack.empty());
}

#[cfg(test)]
mod tests {
    macro_rules! test_stack {
        ($mod_name:ident, $stack_type:ty) => {
            mod $mod_name {
                #[test]
                fn test_example() {
                    let mut stack = <$stack_type>::new();
                    stack.push(1);
                    stack.push(2);
                    assert_eq!(stack.top(), 2);
                    assert_eq!(stack.pop(), 2);
                    assert!(!stack.empty());
                }

                #[test]
                fn test_single_element() {
                    let mut stack = <$stack_type>::new();
                    stack.push(5);
                    assert_eq!(stack.top(), 5);
                    assert_eq!(stack.pop(), 5);
                    assert!(stack.empty());
                }

                #[test]
                fn test_lifo_order() {
                    let mut stack = <$stack_type>::new();
                    stack.push(1);
                    stack.push(2);
                    stack.push(3);
                    assert_eq!(stack.pop(), 3);
                    assert_eq!(stack.pop(), 2);
                    assert_eq!(stack.pop(), 1);
                    assert!(stack.empty());
                }

                #[test]
                fn test_interleaved_push_pop() {
                    let mut stack = <$stack_type>::new();
                    stack.push(1);
                    stack.push(2);
                    assert_eq!(stack.pop(), 2);
                    stack.push(3);
                    assert_eq!(stack.top(), 3);
                    assert_eq!(stack.pop(), 3);
                    assert_eq!(stack.pop(), 1);
                    assert!(stack.empty());
                }

                #[test]
                fn test_top_does_not_remove() {
                    let mut stack = <$stack_type>::new();
                    stack.push(7);
                    assert_eq!(stack.top(), 7);
                    assert_eq!(stack.top(), 7);
                    assert!(!stack.empty());
                }
            }
        };
    }

    test_stack!(my, crate::my_solution::MyStack);
    test_stack!(ai, crate::ai_solution::MyStack);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut stack = crate::my_solution::MyStack::new();
            for x in 0..20 {
                stack.push(x);
            }
            for _ in 0..20 {
                std::hint::black_box(stack.pop());
            }
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut stack = crate::ai_solution::MyStack::new();
            for x in 0..20 {
                stack.push(x);
            }
            for _ in 0..20 {
                std::hint::black_box(stack.pop());
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
