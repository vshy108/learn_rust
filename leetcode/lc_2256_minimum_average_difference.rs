// LeetCode 2256: Minimum Average Difference [Medium]
// https://leetcode.com/problems/minimum-average-difference/
//
// You are given a 0-indexed integer array nums of length n.
//
// The average difference of the index i is the absolute difference between
// the average of the first i + 1 elements of nums and the average of the
// last n - i - 1 elements of nums. Both averages should be rounded down
// to the nearest integer.
//
// Return the index with the minimum average difference. If there are
// multiple such indices, return the smallest one.
//
// Note: The average of n elements is the sum of the n elements divided
// (integer division) by n. The average of 0 elements is considered to be 0.
//
// Examples:
//   nums = [2,5,3,9,5,3] -> 3
//   nums = [0]            -> 0
//
// Constraints:
//   - 1 <= nums.length <= 10^5
//   - 0 <= nums[i] <= 10^5
//
// Run:  cargo run --example lc_2256_minimum_average_difference
// Test: cargo test --example lc_2256_minimum_average_difference
// Bench: cargo test --example lc_2256_minimum_average_difference bench -- --nocapture
//
// --- Benchmark (100_000 iterations, 2026-04-07) ---
// My solution:  56.86ms (avg 568ns)
// AI solution:  57.19ms (avg 571ns)
// -> My solution is 1.01x faster
// Reason: essentially identical algorithms; negligible difference.

/// My solution module — single-pass prefix sum.
///
/// Time:  O(n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the index with the minimum average difference.
        ///
        /// # Arguments
        ///
        /// * `nums` - 0-indexed integer array
        ///
        /// # Returns
        ///
        /// Index with the smallest average difference (smallest index on tie)
        pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            // Use i64 to avoid overflow with large sums
            let total_sum: i64 = nums.iter().map(|&x| x as i64).sum();
            let mut left_sum: i64 = 0;
            let mut min_diff = i64::MAX;
            let mut result_index = 0;
            for i in 0..n {
                left_sum += nums[i] as i64;
                let right_sum = total_sum - left_sum;
                let left_avg = left_sum / (i as i64 + 1);
                let right_avg = if i == n - 1 {
                    0
                } else {
                    right_sum / (n as i64 - i as i64 - 1)
                };
                let diff = (left_avg - right_avg).abs();
                if diff < min_diff {
                    min_diff = diff;
                    result_index = i;
                }
            }
            result_index as i32
        }
    }
}

/// AI solution module — same approach with `enumerate` and `min_by_key`.
///
/// Functional style: build (diff, index) pairs and select minimum.
///
/// Time:  O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the index with the minimum average difference.
        ///
        /// # Arguments
        ///
        /// * `nums` - 0-indexed integer array
        ///
        /// # Returns
        ///
        /// Index with the smallest average difference (smallest index on tie)
        pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            let total: i64 = nums.iter().map(|&x| x as i64).sum();
            let mut left: i64 = 0;
            let mut best_idx = 0;
            let mut best_diff = i64::MAX;
            for (i, &num) in nums.iter().enumerate() {
                left += num as i64;
                let right = total - left;
                let left_avg = left / (i as i64 + 1);
                let right_avg = if i == n - 1 {
                    0
                } else {
                    right / (n as i64 - i as i64 - 1)
                };
                let diff = (left_avg - right_avg).abs();
                if diff < best_diff {
                    best_diff = diff;
                    best_idx = i;
                }
            }
            best_idx as i32
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32)> = vec![(vec![2, 5, 3, 9, 5, 3], 3), (vec![0], 0)];

    println!("--- My Solution ---");
    for (nums, expected) in &test_cases {
        let result = my_solution::Solution::minimum_average_difference(nums.clone());
        println!("  {nums:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (nums, expected) in &test_cases {
        let result = ai_solution::Solution::minimum_average_difference(nums.clone());
        println!("  {nums:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_minimum_average_difference {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(
                        Solution::minimum_average_difference(vec![2, 5, 3, 9, 5, 3]),
                        3
                    );
                }

                #[test]
                fn test_single_element() {
                    assert_eq!(Solution::minimum_average_difference(vec![0]), 0);
                }

                #[test]
                fn test_two_elements() {
                    assert_eq!(Solution::minimum_average_difference(vec![1, 1]), 0);
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(Solution::minimum_average_difference(vec![5, 5, 5, 5]), 0);
                }

                #[test]
                fn test_increasing() {
                    // diffs: [2, 3, 2, 3, 3] -> min=2 at index 0
                    assert_eq!(Solution::minimum_average_difference(vec![1, 2, 3, 4, 5]), 0);
                }

                #[test]
                fn test_large_values() {
                    // diffs: [100000, 50000, 33333, 25000, 20000] -> min at index 4
                    assert_eq!(
                        Solution::minimum_average_difference(vec![100000, 0, 0, 0, 0]),
                        4
                    );
                }

                #[test]
                fn test_all_zeros() {
                    assert_eq!(Solution::minimum_average_difference(vec![0, 0, 0]), 0);
                }
            }
        };
    }

    test_minimum_average_difference!(my_tests, crate::my_solution::Solution);
    test_minimum_average_difference!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<Vec<i32>> = vec![vec![2, 5, 3, 9, 5, 3], vec![0], vec![1, 2, 3, 4, 5]];

        let start = Instant::now();
        for _ in 0..iterations {
            for nums in &inputs {
                std::hint::black_box(crate::my_solution::Solution::minimum_average_difference(
                    std::hint::black_box(nums.clone()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for nums in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::minimum_average_difference(
                    std::hint::black_box(nums.clone()),
                ));
            }
        }
        let ai_duration = start.elapsed();

        println!("\n=== Benchmark ({iterations} iterations) ===");
        println!(
            "  My solution: {:?} (avg {:?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "  AI solution: {:?} (avg {:?})",
            ai_duration,
            ai_duration / iterations
        );

        if my_duration < ai_duration {
            let ratio = ai_duration.as_nanos() as f64 / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64 / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
