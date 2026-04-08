// LeetCode 494: Target Sum [Medium]
// https://leetcode.com/problems/target-sum/
//
// Given an integer array nums and an integer target, assign '+' or '-' to
// each element and find the number of different expressions that evaluate
// to target.
//
// Key insight: Let P = sum of positives, N = sum of negatives.
// P - N = target, P + N = total_sum => P = (target + total_sum) / 2.
// This becomes a subset-sum / 0-1 knapsack counting problem.
//
// Examples:
//   nums = [1,1,1,1,1], target = 3 -> 5
//   nums = [1], target = 1          -> 1
//
// Constraints:
//   1 <= nums.len() <= 20
//   0 <= nums[i] <= 1000
//   0 <= sum(nums[i]) <= 1000
//   -1000 <= target <= 1000
//
// Run:  cargo run --example lc_0494_target_sum
// Test: cargo test --example lc_0494_target_sum
// Bench: cargo test --example lc_0494_target_sum bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 1.365us (DFS + memoization)
//   AI solution: avg 88ns    (subset sum DP)
//   Ratio: 15.5x (bottom-up DP hugely faster than top-down DFS)

/// My solution module — subset sum DP.
///
/// Time:  O(n * sum) where sum = (total + target) / 2
/// Space: O(sum)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
            // DFS with memoization (top-down approach)
            use std::collections::HashMap;
            fn dfs(
                nums: &[i32],
                idx: usize,
                curr_sum: i32,
                target: i32,
                memo: &mut HashMap<(usize, i32), i32>,
            ) -> i32 {
                if idx == nums.len() {
                    return if curr_sum == target { 1 } else { 0 };
                }
                if let Some(&cached) = memo.get(&(idx, curr_sum)) {
                    return cached;
                }
                let result = dfs(nums, idx + 1, curr_sum + nums[idx], target, memo)
                    + dfs(nums, idx + 1, curr_sum - nums[idx], target, memo);
                memo.insert((idx, curr_sum), result);
                result
            }
            let mut memo = HashMap::new();
            dfs(&nums, 0, 0, target, &mut memo)
        }
    }
}

/// AI solution module — convert to subset-sum counting.
///
/// P = (target + total) / 2. Count subsets summing to P.
///
/// Time:  O(n * P)
/// Space: O(P)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
            let total: i32 = nums.iter().sum();
            // (target + total) must be non-negative and even
            let sum = target + total;
            if sum < 0 || sum % 2 != 0 {
                return 0;
            }
            let p = (sum / 2) as usize;

            // dp[j] = number of subsets summing to j
            let mut dp = vec![0i32; p + 1];
            dp[0] = 1;

            for &num in &nums {
                let num = num as usize;
                // Iterate in reverse for 0-1 knapsack (each element used once)
                for j in (num..=p).rev() {
                    dp[j] += dp[j - num];
                }
            }

            dp[p]
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_five_ones() {
            assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
        }
        #[test]
        fn test_single() {
            assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
        }
        #[test]
        fn test_impossible() {
            assert_eq!(Solution::find_target_sum_ways(vec![1], 2), 0);
        }
    };
}

#[cfg(test)]
mod tests_my {
    solution_tests!(my_solution);
}
#[cfg(test)]
mod tests_ai {
    solution_tests!(ai_solution);
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(Vec<i32>, i32) -> i32,
{
    let iterations = 500_000;
    let input = vec![1, 1, 1, 1, 1];
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(
            std::hint::black_box(input.clone()),
            std::hint::black_box(3),
        ));
    }
    let elapsed = start.elapsed();
    println!(
        "{}: {} iterations in {:?} (avg {:?})",
        name,
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::find_target_sum_ways);
    bench_solution("AI solution", ai_solution::Solution::find_target_sum_ways);
}

fn main() {
    println!(
        "{}",
        ai_solution::Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3)
    );
}
