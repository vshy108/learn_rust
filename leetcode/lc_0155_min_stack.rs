// LeetCode 155: Min Stack [Medium]
// https://leetcode.com/problems/min-stack/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.845us
//   AI solution avg: 0.290us
//   Ratio: AI 2.91x faster
//
// Design a stack that supports push, pop, top, and retrieving the minimum
// element in constant time.
//
// Constraints: -2^31 <= val <= 2^31-1
//   pop, top, getMin always called on non-empty stacks
//   At most 3*10^4 calls
//
// Run:  cargo run --example lc_0155_min_stack
// Test: cargo test --example lc_0155_min_stack
// Bench: cargo test --example lc_0155_min_stack bench -- --nocapture

/// My solution — two stacks: one for values, one for running minimums.
///
/// Time: O(1) for all operations.
/// Space: O(n) — two stacks of size n.
mod my_solution {
    pub struct MinStack {
        stack: Vec<i32>,
        min_stack: Vec<i32>, // parallel stack of minimum at each level
    }

    impl MinStack {
        /// Create an empty MinStack.
        pub fn new() -> Self {
            MinStack {
                stack: Vec::new(),
                min_stack: Vec::new(),
            }
        }

        /// Push element onto the stack.
        pub fn push(&mut self, val: i32) {
            self.stack.push(val);
            let cur_min = self.min_stack.last().copied().unwrap_or(i32::MAX);
            self.min_stack.push(val.min(cur_min));
        }

        /// Remove the top element.
        pub fn pop(&mut self) {
            self.stack.pop();
            self.min_stack.pop();
        }

        /// Get the top element.
        pub fn top(&self) -> i32 {
            *self.stack.last().expect("non-empty stack")
        }

        /// Retrieve the minimum element.
        pub fn get_min(&self) -> i32 {
            *self.min_stack.last().expect("non-empty stack")
        }
    }
}

/// AI solution — single stack storing (value, current_min) tuples.
///
/// Time: O(1) for all operations.
/// Space: O(n) — one stack of tuples.
mod ai_solution {
    pub struct MinStack {
        stack: Vec<(i32, i32)>, // (value, running_min)
    }

    impl MinStack {
        /// Create an empty MinStack.
        pub fn new() -> Self {
            MinStack { stack: Vec::new() }
        }

        /// Push element onto the stack.
        pub fn push(&mut self, val: i32) {
            let min = self.stack.last().map(|&(_, m)| m.min(val)).unwrap_or(val);
            self.stack.push((val, min));
        }

        /// Remove the top element.
        pub fn pop(&mut self) {
            self.stack.pop();
        }

        /// Get the top element.
        pub fn top(&self) -> i32 {
            self.stack.last().expect("non-empty").0
        }

        /// Retrieve the minimum element.
        pub fn get_min(&self) -> i32 {
            self.stack.last().expect("non-empty").1
        }
    }
}

fn main() {
    let mut s = my_solution::MinStack::new();
    s.push(-2);
    s.push(0);
    s.push(-3);
    println!("min={}, top={}", s.get_min(), s.top());
    s.pop();
    println!("after pop: min={}, top={}", s.get_min(), s.top());

    let mut s2 = ai_solution::MinStack::new();
    s2.push(-2);
    s2.push(0);
    s2.push(-3);
    println!("ai min={}, top={}", s2.get_min(), s2.top());
    s2.pop();
    println!("ai after pop: min={}, top={}", s2.get_min(), s2.top());
}

#[cfg(test)]
mod tests {
    macro_rules! test_min_stack {
        ($mod_name:ident, $stack_type:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    let mut s = <$stack_type>::new();
                    s.push(-2);
                    s.push(0);
                    s.push(-3);
                    assert_eq!(s.get_min(), -3);
                    s.pop();
                    assert_eq!(s.top(), 0);
                    assert_eq!(s.get_min(), -2);
                }

                #[test]
                fn test_ascending() {
                    let mut s = <$stack_type>::new();
                    s.push(1);
                    s.push(2);
                    s.push(3);
                    assert_eq!(s.get_min(), 1);
                    s.pop();
                    assert_eq!(s.get_min(), 1);
                }

                #[test]
                fn test_descending() {
                    let mut s = <$stack_type>::new();
                    s.push(3);
                    s.push(2);
                    s.push(1);
                    assert_eq!(s.get_min(), 1);
                    s.pop();
                    assert_eq!(s.get_min(), 2);
                    s.pop();
                    assert_eq!(s.get_min(), 3);
                }

                #[test]
                fn test_duplicates() {
                    let mut s = <$stack_type>::new();
                    s.push(0);
                    s.push(0);
                    assert_eq!(s.get_min(), 0);
                    s.pop();
                    assert_eq!(s.get_min(), 0);
                }
            }
        };
    }

    test_min_stack!(my, crate::my_solution::MinStack);
    test_min_stack!(ai, crate::ai_solution::MinStack);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut s = crate::my_solution::MinStack::new();
            for i in 0..50 {
                s.push(i * 7 % 13 - 6);
            }
            for _ in 0..25 {
                std::hint::black_box(s.get_min());
                s.pop();
            }
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut s = crate::ai_solution::MinStack::new();
            for i in 0..50 {
                s.push(i * 7 % 13 - 6);
            }
            for _ in 0..25 {
                std::hint::black_box(s.get_min());
                s.pop();
            }
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );
        println!(
            "AI solution: {:.3}us avg",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("Both solutions are roughly equal");
        }
    }
}
