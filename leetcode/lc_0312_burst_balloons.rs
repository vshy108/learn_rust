// LeetCode 312. Burst Balloons
// https://leetcode.com/problems/burst-balloons/
//
// Benchmark results (100_000 iterations, release mode, 2025-01-XX):
//   my_solution  avg: 0.551us
//   ai_solution  avg: 0.591us
//   Ratio: my_solution 1.07x faster

/// You are given n balloons indexed 0..n-1. Each balloon has a number nums[i].
/// If you burst balloon i, you get nums[i-1] * nums[i] * nums[i+1] coins.
/// Out of bounds treated as balloon with value 1. Return max coins.
///
/// Example 1:
///   Input: nums = [3,1,5,8]
///   Output: 167
///   Explanation: 3*1*5 + 3*5*8 + 1*3*8 + 1*8*1 = 167
///
/// Example 2:
///   Input: nums = [1,5]
///   Output: 10
///
/// Constraints:
///   - n == nums.length
///   - 1 <= n <= 300
///   - 0 <= nums[i] <= 100

/// # my_solution: Interval DP (bottom-up)
/// Time: O(n^3) — three nested loops over intervals
/// Space: O(n^2) — 2D DP table
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find maximum coins from bursting all balloons.
        ///
        /// Key insight: think of k as the LAST balloon to burst in interval (i, j).
        /// Then f[i][j] = max over k of (f[i][k] + f[k][j] + arr[i]*arr[k]*arr[j]).
        ///
        /// # Arguments
        /// * `nums` - Values painted on balloons
        ///
        /// # Returns
        /// Maximum coins collectible
        pub fn max_coins(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            // Pad with 1 on both ends: arr = [1, nums[0], ..., nums[n-1], 1]
            let mut arr = vec![1i32; n + 2];
            arr[1..=n].copy_from_slice(&nums);

            // f[i][j] = max coins from bursting all balloons strictly between i and j
            let mut f = vec![vec![0i32; n + 2]; n + 2];

            // Iterate i from bottom up, j from left to right
            for i in (0..n).rev() {
                for j in i + 2..n + 2 {
                    for k in i + 1..j {
                        f[i][j] = f[i][j].max(f[i][k] + f[k][j] + arr[i] * arr[k] * arr[j]);
                    }
                }
            }

            f[0][n + 1]
        }
    }
}

/// # ai_solution: Interval DP (top-down memoization)
/// Time: O(n^3) — each subproblem computed once
/// Space: O(n^2) — memo table + recursion stack O(n)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find maximum coins using recursive memoization.
        ///
        /// Same DP recurrence as bottom-up, but computed lazily via recursion.
        ///
        /// # Arguments
        /// * `nums` - Values painted on balloons
        ///
        /// # Returns
        /// Maximum coins collectible
        pub fn max_coins(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            let mut arr = vec![1i32; n + 2];
            arr[1..=n].copy_from_slice(&nums);

            // -1 means "not computed yet"
            let mut memo = vec![vec![-1i32; n + 2]; n + 2];
            dfs(&arr, &mut memo, 0, n + 1)
        }
    }

    /// Recursively compute max coins for balloons strictly between left and right.
    fn dfs(arr: &[i32], memo: &mut [Vec<i32>], left: usize, right: usize) -> i32 {
        // No balloons between left and right
        if left + 1 >= right {
            return 0;
        }
        if memo[left][right] != -1 {
            return memo[left][right];
        }

        let mut best = 0;
        // Try each k as the last balloon to burst in (left, right)
        for k in left + 1..right {
            let coins = dfs(arr, memo, left, k)
                + dfs(arr, memo, k, right)
                + arr[left] * arr[k] * arr[right];
            best = best.max(coins);
        }

        memo[left][right] = best;
        best
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::Solution as AiSolution;
    use super::my_solution::Solution as MySolution;

    fn run_tests(max_coins: fn(Vec<i32>) -> i32) {
        // Example 1
        assert_eq!(max_coins(vec![3, 1, 5, 8]), 167);

        // Example 2
        assert_eq!(max_coins(vec![1, 5]), 10);

        // Single balloon
        assert_eq!(max_coins(vec![5]), 5); // 1*5*1 = 5

        // All zeros
        assert_eq!(max_coins(vec![0, 0, 0]), 0);

        // Single large value
        assert_eq!(max_coins(vec![100]), 100);

        // Increasing sequence
        assert_eq!(max_coins(vec![1, 2, 3, 4]), 40);
    }

    #[test]
    fn test_my_solution() {
        run_tests(MySolution::max_coins);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(AiSolution::max_coins);
    }
}

fn main() {
    use std::hint::black_box;
    use std::time::Instant;

    let nums = vec![3, 1, 5, 8];
    println!("Input: nums = {:?}", nums);
    println!(
        "Output (my): {}",
        my_solution::Solution::max_coins(nums.clone())
    );
    println!(
        "Output (ai): {}",
        ai_solution::Solution::max_coins(nums.clone())
    );

    let iterations = 100_000;
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(my_solution::Solution::max_coins(black_box(nums.clone())));
    }
    let my_dur = start.elapsed();

    let start = Instant::now();
    for _ in 0..iterations {
        black_box(ai_solution::Solution::max_coins(black_box(nums.clone())));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    println!("\nBenchmark ({iterations} iterations, release mode):");
    println!("  my_solution  avg: {my_avg:.3}us");
    println!("  ai_solution  avg: {ai_avg:.3}us");
    if my_avg < ai_avg {
        println!("  my_solution is {:.2}x faster", ai_avg / my_avg);
    } else {
        println!("  ai_solution is {:.2}x faster", my_avg / ai_avg);
    }
}
