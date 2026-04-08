// LeetCode 502: IPO [Hard]
// https://leetcode.com/problems/ipo/
//
// LeetCode will start its IPO soon. To sell shares at a good price, it wants
// to work on some projects to increase its capital. Given n projects where the
// i-th has pure profit profits[i] and requires minimum capital capital[i],
// and starting with w capital, pick at most k distinct projects to maximize
// final capital. Profit from finished projects adds to your capital.
//
// Examples:
//   k=2, w=0, profits=[1,2,3], capital=[0,1,1] -> 4
//   k=3, w=0, profits=[1,2,3], capital=[0,1,2] -> 6
//
// Constraints:
//   1 <= k <= 10^5
//   0 <= w <= 10^9
//   n == profits.length == capital.length
//   1 <= n <= 10^5
//   0 <= profits[i] <= 10^4
//   0 <= capital[i] <= 10^9
//
// Run:  cargo run --example lc_0502_ipo
// Test: cargo test --example lc_0502_ipo
// Bench: cargo test --example lc_0502_ipo bench -- --nocapture
//
// First benchmark (100K iterations, 2026-04-08):
//   My solution: avg 318ns
//   AI solution: avg 256ns
//   AI is 1.24x faster
//   Reason: AI uses a sorted array + single index scan instead of a min-heap,
//   avoiding heap pop overhead for unlocking affordable projects.

/// My solution — greedy with two heaps (sort by capital + max-heap on profit).
///
/// Sort projects by required capital. Use a min-heap to track available
/// projects, and a max-heap to always pick the most profitable available one.
/// Translated from Java solution in doocs/leetcode.
///
/// Time:  O(n log n + k log n)
/// Space: O(n)
mod my_solution {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    pub struct Solution;

    impl Solution {
        /// Return the maximized capital after completing at most k projects.
        ///
        /// # Arguments
        /// * `k` - max number of projects to complete
        /// * `w` - initial capital
        /// * `profits` - pure profit for each project
        /// * `capital` - minimum capital required for each project
        ///
        /// # Returns
        /// Final maximized capital
        pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
            let n = profits.len();
            // Min-heap sorted by required capital: (capital, profit)
            let mut q1: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::with_capacity(n);
            for i in 0..n {
                q1.push(Reverse((capital[i], profits[i])));
            }
            // Max-heap of available profits
            let mut q2: BinaryHeap<i32> = BinaryHeap::new();
            let mut w = w;
            let mut k = k;

            while k > 0 {
                // Move all affordable projects into the profit heap
                while let Some(&Reverse((c, p))) = q1.peek() {
                    if c <= w {
                        q1.pop();
                        q2.push(p);
                    } else {
                        break;
                    }
                }
                // Pick the most profitable available project
                if let Some(profit) = q2.pop() {
                    w += profit;
                } else {
                    break; // No affordable projects left
                }
                k -= 1;
            }
            w
        }
    }
}

/// AI solution — sort + single max-heap with index tracking.
///
/// Sort projects by capital ascending, then greedily unlock projects as
/// capital grows. Uses a single pass over the sorted array instead of a
/// min-heap.
///
/// Time:  O(n log n + k log n)
/// Space: O(n)
mod ai_solution {
    use std::collections::BinaryHeap;

    pub struct Solution;

    impl Solution {
        /// Return the maximized capital using sorted array + max-heap.
        ///
        /// # Arguments
        /// * `k` - max number of projects to complete
        /// * `w` - initial capital
        /// * `profits` - pure profit for each project
        /// * `capital` - minimum capital required for each project
        ///
        /// # Returns
        /// Final maximized capital
        pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
            let n = profits.len();
            // Sort indices by required capital ascending
            let mut indices: Vec<usize> = (0..n).collect();
            indices.sort_unstable_by_key(|&i| capital[i]);

            let mut heap: BinaryHeap<i32> = BinaryHeap::new();
            let mut w = w;
            let mut idx = 0;

            for _ in 0..k {
                // Unlock projects we can now afford
                while idx < n && capital[indices[idx]] <= w {
                    heap.push(profits[indices[idx]]);
                    idx += 1;
                }
                if let Some(profit) = heap.pop() {
                    w += profit;
                } else {
                    break;
                }
            }
            w
        }
    }
}

fn main() {
    let k = 2;
    let w = 0;
    let profits = vec![1, 2, 3];
    let capital = vec![0, 1, 1];
    let my = my_solution::Solution::find_maximized_capital(k, w, profits.clone(), capital.clone());
    let ai = ai_solution::Solution::find_maximized_capital(k, w, profits, capital);
    println!("my: {my}, ai: {ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_ipo {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(
                        <$solution>::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]),
                        4
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        <$solution>::find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2]),
                        6
                    );
                }

                #[test]
                fn test_cannot_start_any() {
                    // All projects require capital > w
                    assert_eq!(
                        <$solution>::find_maximized_capital(2, 0, vec![1, 2, 3], vec![1, 1, 1]),
                        0
                    );
                }

                #[test]
                fn test_single_project() {
                    assert_eq!(
                        <$solution>::find_maximized_capital(1, 0, vec![5], vec![0]),
                        5
                    );
                }

                #[test]
                fn test_k_larger_than_n() {
                    // k > number of projects — should just do all affordable
                    assert_eq!(
                        <$solution>::find_maximized_capital(10, 0, vec![1, 2, 3], vec![0, 1, 2]),
                        6
                    );
                }
            }
        };
    }

    test_ipo!(my, crate::my_solution::Solution);
    test_ipo!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let profits = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let capital = vec![0, 1, 1, 2, 2, 3, 3, 4, 4, 5];
        let k = 5;
        let w = 0;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::find_maximized_capital(
                std::hint::black_box(k),
                std::hint::black_box(w),
                std::hint::black_box(profits.clone()),
                std::hint::black_box(capital.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::find_maximized_capital(
                std::hint::black_box(k),
                std::hint::black_box(w),
                std::hint::black_box(profits.clone()),
                std::hint::black_box(capital.clone()),
            ));
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
            println!("~tied (ratio: {ratio:.2})");
        }
    }
}
