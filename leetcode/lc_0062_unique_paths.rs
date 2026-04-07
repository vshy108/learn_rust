// LeetCode 62: Unique Paths [Medium]
// https://leetcode.com/problems/unique-paths/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 5.890us
//   AI solution avg: 2.459us
//   Ratio: AI 2.40x faster
//
// A robot is on an m x n grid at top-left corner. It can only move
// right or down. How many unique paths to the bottom-right corner?
//
// Example 1: m=3, n=7 -> 28
// Example 2: m=3, n=2 -> 3
//
// Constraints: 1 <= m, n <= 100
//
// Run:  cargo run --example lc_0062_unique_paths
// Test: cargo test --example lc_0062_unique_paths
// Bench: cargo test --example lc_0062_unique_paths bench -- --nocapture

/// My solution — 2D dynamic programming.
///
/// dp[i][j] = number of paths to reach cell (i, j).
/// dp[i][j] = dp[i-1][j] + dp[i][j-1].
///
/// Time: O(m*n).
/// Space: O(m*n).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Count unique paths from top-left to bottom-right.
        ///
        /// # Arguments
        /// * `m` - number of rows
        /// * `n` - number of columns
        ///
        /// # Returns
        /// Number of unique paths
        pub fn unique_paths(m: i32, n: i32) -> i32 {
            let (m, n) = (m as usize, n as usize);
            let mut dp = vec![vec![1i32; n]; m];
            for i in 1..m {
                for j in 1..n {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                }
            }
            dp[m - 1][n - 1]
        }
    }
}

/// AI solution — 1D DP (space-optimized).
///
/// Only keep one row of DP values. Update left-to-right.
///
/// Time: O(m*n).
/// Space: O(n).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Count unique paths from top-left to bottom-right.
        ///
        /// # Arguments
        /// * `m` - number of rows
        /// * `n` - number of columns
        ///
        /// # Returns
        /// Number of unique paths
        pub fn unique_paths(m: i32, n: i32) -> i32 {
            let (m, n) = (m as usize, n as usize);
            let mut row = vec![1i32; n];
            for _ in 1..m {
                for j in 1..n {
                    row[j] += row[j - 1];
                }
            }
            row[n - 1]
        }
    }
}

fn main() {
    let cases = [(3, 7), (3, 2), (7, 3), (1, 1)];
    for (m, n) in &cases {
        println!(
            "m={m}, n={n} -> my:{}, ai:{}",
            my_solution::Solution::unique_paths(*m, *n),
            ai_solution::Solution::unique_paths(*m, *n),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_unique_paths {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_3x7() {
                    assert_eq!(<$solution>::unique_paths(3, 7), 28);
                }

                #[test]
                fn test_3x2() {
                    assert_eq!(<$solution>::unique_paths(3, 2), 3);
                }

                #[test]
                fn test_1x1() {
                    assert_eq!(<$solution>::unique_paths(1, 1), 1);
                }

                #[test]
                fn test_1xn() {
                    assert_eq!(<$solution>::unique_paths(1, 10), 1);
                }

                #[test]
                fn test_mx1() {
                    assert_eq!(<$solution>::unique_paths(10, 1), 1);
                }

                #[test]
                fn test_large() {
                    assert_eq!(<$solution>::unique_paths(10, 10), 48620);
                }
            }
        };
    }

    test_unique_paths!(my, crate::my_solution::Solution);
    test_unique_paths!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::unique_paths(
                std::hint::black_box(15),
                std::hint::black_box(15),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::unique_paths(
                std::hint::black_box(15),
                std::hint::black_box(15),
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
