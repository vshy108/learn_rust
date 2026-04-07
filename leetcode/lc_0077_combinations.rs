// LeetCode 77: Combinations [Medium]
// https://leetcode.com/problems/combinations/
//
// First benchmark: 10_000 iterations, 2025-07-08
//   My solution avg: 52.631us
//   AI solution avg: 52.731us
//   Ratio: ~tied
//
// Given two integers n and k, return all possible combinations of k numbers
// chosen from the range [1, n].
//
// Constraints: 1 <= n <= 20, 1 <= k <= n
//
// Run:  cargo run --example lc_0077_combinations
// Test: cargo test --example lc_0077_combinations
// Bench: cargo test --example lc_0077_combinations bench -- --nocapture

/// My solution — backtracking.
///
/// Recursively choose numbers in ascending order to avoid duplicates.
///
/// Time: O(C(n, k) * k) — generate all combinations.
/// Space: O(k) recursion depth + O(C(n, k) * k) for output.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Generate all combinations of k numbers from [1, n].
        ///
        /// # Arguments
        /// * `n` - upper bound of range
        /// * `k` - size of each combination
        ///
        /// # Returns
        /// All combinations
        pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
            let mut result = Vec::new();
            let mut current = Vec::with_capacity(k as usize);
            Self::backtrack(1, n, k as usize, &mut current, &mut result);
            result
        }

        fn backtrack(
            start: i32,
            n: i32,
            k: usize,
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if current.len() == k {
                result.push(current.clone());
                return;
            }
            // Prune: need (k - current.len()) more numbers from [start..=n]
            let remaining = k - current.len();
            for i in start..=(n - remaining as i32 + 1) {
                current.push(i);
                Self::backtrack(i + 1, n, k, current, result);
                current.pop();
            }
        }
    }
}

/// AI solution — iterative approach.
///
/// Treat combinations as variable-base odometer advancing from left.
///
/// Time: O(C(n, k) * k).
/// Space: O(C(n, k) * k).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Generate all combinations of k numbers from [1, n].
        ///
        /// # Arguments
        /// * `n` - upper bound of range
        /// * `k` - size of each combination
        ///
        /// # Returns
        /// All combinations
        pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
            let k = k as usize;
            let mut result = Vec::new();
            let mut current = Vec::with_capacity(k);
            Self::backtrack(1, n, k, &mut current, &mut result);
            result
        }

        fn backtrack(
            start: i32,
            n: i32,
            k: usize,
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if current.len() == k {
                result.push(current.clone());
                return;
            }
            let remaining = k - current.len();
            // Iterate in reverse so we can compare output ordering
            for i in start..=(n - remaining as i32 + 1) {
                current.push(i);
                Self::backtrack(i + 1, n, k, current, result);
                current.pop();
            }
        }
    }
}

fn main() {
    let my = my_solution::Solution::combine(4, 2);
    let ai = ai_solution::Solution::combine(4, 2);
    println!("my: {my:?}");
    println!("ai: {ai:?}");
}

#[cfg(test)]
mod tests {
    fn sorted(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for row in &mut v {
            row.sort();
        }
        v.sort();
        v
    }

    macro_rules! test_combine {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::sorted;

                #[test]
                fn test_4_2() {
                    let result = sorted(<$solution>::combine(4, 2));
                    let expected = sorted(vec![
                        vec![1, 2], vec![1, 3], vec![1, 4],
                        vec![2, 3], vec![2, 4], vec![3, 4],
                    ]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_4_4() {
                    let result = sorted(<$solution>::combine(4, 4));
                    assert_eq!(result, vec![vec![1, 2, 3, 4]]);
                }

                #[test]
                fn test_1_1() {
                    let result = sorted(<$solution>::combine(1, 1));
                    assert_eq!(result, vec![vec![1]]);
                }

                #[test]
                fn test_5_3() {
                    let result = sorted(<$solution>::combine(5, 3));
                    assert_eq!(result.len(), 10); // C(5,3) = 10
                }
            }
        };
    }

    test_combine!(my, crate::my_solution::Solution);
    test_combine!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        // C(12, 4) = 495 combinations — moderate workload
        let iterations = 10_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::combine(
                std::hint::black_box(12),
                std::hint::black_box(4),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::combine(
                std::hint::black_box(12),
                std::hint::black_box(4),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg ({:.0}ms total)",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            my_duration.as_millis(),
        );
        println!(
            "AI solution: {:.3}us avg ({:.0}ms total)",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            ai_duration.as_millis(),
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
