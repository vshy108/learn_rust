// LeetCode 63: Unique Paths II [Medium]
// https://leetcode.com/problems/unique-paths-ii/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 4.429us
//   AI solution avg: 39.610us
//   Ratio: My 8.94x faster
//
// Same as Unique Paths but with obstacles. obstacleGrid[i][j] == 1 means blocked.
//
// Example 1: grid = [[0,0,0],[0,1,0],[0,0,0]] -> 2
// Example 2: grid = [[0,1],[0,0]] -> 1
//
// Constraints: m,n in [1,100], grid values are 0 or 1
//
// Run:  cargo run --example lc_0063_unique_paths_ii
// Test: cargo test --example lc_0063_unique_paths_ii
// Bench: cargo test --example lc_0063_unique_paths_ii bench -- --nocapture

/// My solution — 2D DP with obstacle check.
///
/// Time: O(m*n).
/// Space: O(m*n).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Count unique paths avoiding obstacles.
        ///
        /// # Arguments
        /// * `obstacle_grid` - grid with 0=free, 1=blocked
        ///
        /// # Returns
        /// Number of unique paths
        pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
            let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
            if obstacle_grid[0][0] == 1 || obstacle_grid[m - 1][n - 1] == 1 {
                return 0;
            }
            let mut dp = vec![vec![0i32; n]; m];
            dp[0][0] = 1;
            // Fill first row
            for j in 1..n {
                dp[0][j] = if obstacle_grid[0][j] == 1 { 0 } else { dp[0][j - 1] };
            }
            // Fill first column
            for i in 1..m {
                dp[i][0] = if obstacle_grid[i][0] == 1 { 0 } else { dp[i - 1][0] };
            }
            for i in 1..m {
                for j in 1..n {
                    dp[i][j] = if obstacle_grid[i][j] == 1 {
                        0
                    } else {
                        dp[i - 1][j] + dp[i][j - 1]
                    };
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
        /// Count unique paths avoiding obstacles.
        ///
        /// # Arguments
        /// * `obstacle_grid` - grid with 0=free, 1=blocked
        ///
        /// # Returns
        /// Number of unique paths
        pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
            let n = obstacle_grid[0].len();
            let mut row = vec![0i32; n];
            row[0] = 1;

            for grid_row in &obstacle_grid {
                for j in 0..n {
                    if grid_row[j] == 1 {
                        row[j] = 0;
                    } else if j > 0 {
                        row[j] += row[j - 1];
                    }
                }
            }
            row[n - 1]
        }
    }
}

fn main() {
    let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    println!(
        "my:{}, ai:{}",
        my_solution::Solution::unique_paths_with_obstacles(grid.clone()),
        ai_solution::Solution::unique_paths_with_obstacles(grid),
    );
}

#[cfg(test)]
mod tests {
    macro_rules! test_unique_paths_obstacles {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_basic() {
                    let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
                    assert_eq!(<$solution>::unique_paths_with_obstacles(grid), 2);
                }

                #[test]
                fn test_blocked_path() {
                    let grid = vec![vec![0, 1], vec![0, 0]];
                    assert_eq!(<$solution>::unique_paths_with_obstacles(grid), 1);
                }

                #[test]
                fn test_start_blocked() {
                    let grid = vec![vec![1, 0], vec![0, 0]];
                    assert_eq!(<$solution>::unique_paths_with_obstacles(grid), 0);
                }

                #[test]
                fn test_end_blocked() {
                    let grid = vec![vec![0, 0], vec![0, 1]];
                    assert_eq!(<$solution>::unique_paths_with_obstacles(grid), 0);
                }

                #[test]
                fn test_single_cell() {
                    assert_eq!(<$solution>::unique_paths_with_obstacles(vec![vec![0]]), 1);
                }

                #[test]
                fn test_single_blocked() {
                    assert_eq!(<$solution>::unique_paths_with_obstacles(vec![vec![1]]), 0);
                }
            }
        };
    }

    test_unique_paths_obstacles!(my, crate::my_solution::Solution);
    test_unique_paths_obstacles!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let grid: Vec<Vec<i32>> = (0..50)
            .map(|i| (0..50).map(|j| if (i + j) % 7 == 0 && (i, j) != (0, 0) { 1 } else { 0 }).collect())
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::unique_paths_with_obstacles(
                    std::hint::black_box(grid.clone()),
                ),
            );
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::unique_paths_with_obstacles(
                    std::hint::black_box(grid.clone()),
                ),
            );
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
