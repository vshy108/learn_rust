// LeetCode 446: Arithmetic Slices II - Subsequence [Hard]
// https://leetcode.com/problems/arithmetic-slices-ii-subsequence/
//
// Given an integer array `nums`, return the number of all the arithmetic
// subsequences of `nums`.
//
// A sequence is arithmetic if it consists of at least three elements and the
// difference between any two consecutive elements is the same.
//
// A subsequence of an array is a sequence that can be formed by deleting some
// elements (possibly none) without changing the order of the remaining elements.
//
// Examples:
//   nums = [2,4,6,8,10] -> 7
//     The 7 arithmetic subsequences are:
//     [2,4,6], [4,6,8], [6,8,10], [2,4,6,8], [4,6,8,10], [2,4,6,8,10],
//     [2,6,10]
//   nums = [7,7,7,7,7]  -> 16
//
// Constraints:
//   - 1 <= nums.length <= 1000
//   - -2^31 <= nums[i] <= 2^31 - 1
//
// Run:  cargo run --example lc_0446_arithmetic_slices_ii_subsequence
// Test: cargo test --example lc_0446_arithmetic_slices_ii_subsequence
// Bench: cargo test --example lc_0446_arithmetic_slices_ii_subsequence bench -- --nocapture
//
// --- Rust concepts used ---
// - Vec<HashMap<i64, i32>> per-index DP: each element has its own map
//   of diff -> count
// - i64 key for difference: nums[i] as i64 - nums[j] as i64 avoids i32
//   overflow when nums values span the full i32 range
// - HashMap::with_capacity(n) pre-allocation: reduces rehashing in the
//   inner loop
//   (Previously covered: HashMap entry API -> lc_0380, unwrap_or -> lc_0295,
//    vec![expr; n] init -> lc_0006)
//
// --- Benchmark (10_000 iterations, 2026-04-07) ---
// My solution:  128.26ms (avg 12.82us)
// AI solution:  119.37ms (avg 11.93us)
// -> AI solution is 1.07x faster
// Reason: pre-allocated HashMap capacity reduces rehashing overhead in
//   the O(n^2) inner loop.

/// My solution module — DP with per-index HashMap of diffs.
///
/// Time:  O(n^2)
/// Space: O(n^2) for the HashMaps
mod my_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Return the number of arithmetic subsequences of length >= 3.
        ///
        /// # Arguments
        ///
        /// * `nums` - Integer array
        ///
        /// # Returns
        ///
        /// Count of all arithmetic subsequences
        pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            if n < 3 {
                return 0;
            }
            // dp[i] maps diff -> count of subsequences ending at i with length >= 2
            let mut dp: Vec<HashMap<i64, i32>> = vec![HashMap::new(); n];
            let mut total = 0;
            for i in 0..n {
                for j in 0..i {
                    // Use i64 to avoid overflow: nums[i] - nums[j] can exceed i32
                    let diff = nums[i] as i64 - nums[j] as i64;
                    let count_at_j = *dp[j].get(&diff).unwrap_or(&0);
                    // count_at_j subsequences of length >= 2 at j become length >= 3 at i
                    *dp[i].entry(diff).or_insert(0) += count_at_j + 1;
                    total += count_at_j;
                }
            }
            total
        }
    }
}

/// AI solution module — same DP, uses i64 diff key with FxHashMap-like pattern.
///
/// Same algorithm but pre-allocates HashMap capacity to reduce rehashing.
///
/// Time:  O(n^2)
/// Space: O(n^2)
mod ai_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Return the number of arithmetic subsequences of length >= 3.
        ///
        /// # Arguments
        ///
        /// * `nums` - Integer array
        ///
        /// # Returns
        ///
        /// Count of all arithmetic subsequences
        pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            if n < 3 {
                return 0;
            }
            // Pre-allocate each map with capacity n to reduce rehashing
            let mut dp: Vec<HashMap<i64, i32>> =
                (0..n).map(|_| HashMap::with_capacity(n)).collect();
            let mut total = 0;
            for i in 1..n {
                for j in 0..i {
                    let diff = nums[i] as i64 - nums[j] as i64;
                    let count_at_j = *dp[j].get(&diff).unwrap_or(&0);
                    *dp[i].entry(diff).or_insert(0) += count_at_j + 1;
                    total += count_at_j;
                }
            }
            total
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32)> =
        vec![(vec![2, 4, 6, 8, 10], 7), (vec![7, 7, 7, 7, 7], 16)];

    println!("--- My Solution ---");
    for (nums, expected) in &test_cases {
        let result = my_solution::Solution::number_of_arithmetic_slices(nums.clone());
        println!("  {nums:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (nums, expected) in &test_cases {
        let result = ai_solution::Solution::number_of_arithmetic_slices(nums.clone());
        println!("  {nums:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_arithmetic_slices {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(
                        Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]),
                        7
                    );
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(
                        Solution::number_of_arithmetic_slices(vec![7, 7, 7, 7, 7]),
                        16
                    );
                }

                #[test]
                fn test_no_arithmetic() {
                    assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2, 4]), 0);
                }

                #[test]
                fn test_single_element() {
                    assert_eq!(Solution::number_of_arithmetic_slices(vec![1]), 0);
                }

                #[test]
                fn test_two_elements() {
                    assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2]), 0);
                }

                #[test]
                fn test_three_elements_arithmetic() {
                    assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2, 3]), 1);
                }

                #[test]
                fn test_negative_diff() {
                    assert_eq!(Solution::number_of_arithmetic_slices(vec![5, 3, 1, -1]), 3);
                }
            }
        };
    }

    test_arithmetic_slices!(my_tests, crate::my_solution::Solution);
    test_arithmetic_slices!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 10_000_u32;
        let inputs: Vec<Vec<i32>> = vec![vec![2, 4, 6, 8, 10], vec![7, 7, 7, 7, 7], vec![1, 2, 3]];

        let start = Instant::now();
        for _ in 0..iterations {
            for nums in &inputs {
                std::hint::black_box(crate::my_solution::Solution::number_of_arithmetic_slices(
                    std::hint::black_box(nums.clone()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for nums in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::number_of_arithmetic_slices(
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
