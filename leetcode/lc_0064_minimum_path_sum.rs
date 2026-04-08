// LeetCode 64: Minimum Path Sum [Medium]
// https://leetcode.com/problems/minimum-path-sum/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 80.513us
//   AI solution avg: 55.602us
//   Ratio: AI 1.45x faster
//
// Given an m x n grid filled with non-negative numbers, find a path from
// top-left to bottom-right that minimizes the sum. Can only move right or down.
//
// Example 1: grid = [[1,3,1],[1,5,1],[4,2,1]] -> 7 (1->3->1->1->1)
// Example 2: grid = [[1,2,3],[4,5,6]] -> 12
//
// Constraints: m,n in [1,200], 0 <= grid[i][j] <= 200
//
// Run:  cargo run --example lc_0064_minimum_path_sum
// Test: cargo test --example lc_0064_minimum_path_sum
// Bench: cargo test --example lc_0064_minimum_path_sum bench -- --nocapture

/// My solution — 2D DP.
///
/// dp[i][j] = min path sum to reach (i,j).
///
/// Time: O(m*n).
/// Space: O(m*n).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find minimum path sum from top-left to bottom-right.
        ///
        /// # Arguments
        /// * `grid` - m x n grid of non-negative integers
        ///
        /// # Returns
        /// Minimum path sum
        pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
            let (m, n) = (grid.len(), grid[0].len());
            let mut dp = vec![vec![0; n]; m];
            dp[0][0] = grid[0][0];
            for j in 1..n {
                dp[0][j] = dp[0][j - 1] + grid[0][j];
            }
            for i in 1..m {
                dp[i][0] = dp[i - 1][0] + grid[i][0];
            }
            for i in 1..m {
                for j in 1..n {
                    dp[i][j] = grid[i][j] + dp[i - 1][j].min(dp[i][j - 1]);
                }
            }
            dp[m - 1][n - 1]
        }
    }
}

/// AI solution — 1D DP (space-optimized).
///
/// Time: O(m*n).
/// Space: O(n).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find minimum path sum from top-left to bottom-right.
        ///
        /// # Arguments
        /// * `grid` - m x n grid of non-negative integers
        ///
        /// # Returns
        /// Minimum path sum
        pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
            let n = grid[0].len();
            let mut row = vec![0i32; n];
            row[0] = grid[0][0];
            for j in 1..n {
                row[j] = row[j - 1] + grid[0][j];
            }
            for i in 1..grid.len() {
                row[0] += grid[i][0];
                for j in 1..n {
                    row[j] = grid[i][j] + row[j].min(row[j - 1]);
                }
            }
            row[n - 1]
        }
    }
}

fn main() {
    let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    println!(
        "my:{}, ai:{}",
        my_solution::Solution::min_path_sum(grid.clone()),
        ai_solution::Solution::min_path_sum(grid),
    );
}

#[cfg(test)]
mod tests {
    macro_rules! test_min_path_sum {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_basic() {
                    let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
                    assert_eq!(<$solution>::min_path_sum(grid), 7);
                }

                #[test]
                fn test_2x3() {
                    let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
                    assert_eq!(<$solution>::min_path_sum(grid), 12);
                }

                #[test]
                fn test_single_cell() {
                    assert_eq!(<$solution>::min_path_sum(vec![vec![5]]), 5);
                }

                #[test]
                fn test_single_row() {
                    assert_eq!(<$solution>::min_path_sum(vec![vec![1, 2, 3]]), 6);
                }

                #[test]
                fn test_single_col() {
                    assert_eq!(
                        <$solution>::min_path_sum(vec![vec![1], vec![2], vec![3]]),
                        6
                    );
                }
            }
        };
    }

    test_min_path_sum!(my, crate::my_solution::Solution);
    test_min_path_sum!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let grid: Vec<Vec<i32>> = (0..50)
            .map(|i| (0..50).map(|j| (i + j) % 10).collect())
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::min_path_sum(
                std::hint::black_box(grid.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::min_path_sum(
                std::hint::black_box(grid.clone()),
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
