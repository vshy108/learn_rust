// LeetCode 413: Arithmetic Slices [Medium]
// https://leetcode.com/problems/arithmetic-slices/
// Benchmark: 1000000 iterations
//   My solution (iterative streak counting): avg 2.71us
//   AI solution (DP space-optimized):        avg 3.00us
//   -> Roughly tied (My 1.11x faster)
//
// An arithmetic subarray has at least 3 elements with equal consecutive diffs.
// Return the number of arithmetic subarrays of `nums`.
//
// Examples:
//   [1,2,3,4] -> 3  ([1,2,3], [2,3,4], [1,2,3,4])
//   [1]       -> 0
//
// Constraints:
//   1 <= nums.length <= 5000
//   -1000 <= nums[i] <= 1000
//
// Run:  cargo run --example lc_0413_arithmetic_slices
// Test: cargo test --example lc_0413_arithmetic_slices
// Bench: cargo test --example lc_0413_arithmetic_slices bench -- --nocapture

/// My solution module — iterative counting with running streak.
///
/// Track the current arithmetic difference `d` and a counter `cnt`
/// representing how many consecutive pairs share that difference.
/// When cnt >= 2 (meaning 3+ elements), we have (cnt-1) new slices
/// ending at the current position. Equivalently, accumulate cnt
/// after each extension.
///
/// Time:  O(n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Count the number of arithmetic subarrays.
        ///
        /// # Arguments
        ///
        /// * `nums` - Integer array
        ///
        /// # Returns
        ///
        /// Number of arithmetic subarrays (length >= 3)
        pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
            if nums.len() < 3 {
                return 0;
            }

            let mut ans = 0;
            let mut cnt = 0; // length of current arithmetic extension
            let mut d = i32::MAX; // sentinel for initial difference

            for i in 1..nums.len() {
                let diff = nums[i] - nums[i - 1];
                if diff == d {
                    cnt += 1;
                } else {
                    d = diff;
                    cnt = 0;
                }
                // `cnt` equals the number of new arithmetic slices
                // ending at index i
                ans += cnt;
            }

            ans
        }
    }
}

/// AI solution module — DP with space-optimized recurrence.
///
/// dp[i] = number of arithmetic slices ending at index i.
/// If nums[i] - nums[i-1] == nums[i-1] - nums[i-2], then
/// dp[i] = dp[i-1] + 1. Otherwise dp[i] = 0.
/// Answer = sum of all dp[i]. Only need previous value.
///
/// Time:  O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Count the number of arithmetic subarrays.
        ///
        /// # Arguments
        ///
        /// * `nums` - Integer array
        ///
        /// # Returns
        ///
        /// Number of arithmetic subarrays (length >= 3)
        pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            if n < 3 {
                return 0;
            }

            let mut dp = 0; // dp value for current index
            let mut ans = 0;

            for i in 2..n {
                if nums[i] - nums[i - 1] == nums[i - 1] - nums[i - 2] {
                    dp += 1;
                    ans += dp;
                } else {
                    dp = 0;
                }
            }

            ans
        }
    }
}

fn main() {
    let cases = [
        (vec![1, 2, 3, 4], 3),
        (vec![1], 0),
        (vec![1, 2, 3, 4, 5], 6),
    ];

    println!("--- My Solution ---");
    for (nums, expected) in &cases {
        let result =
            my_solution::Solution::number_of_arithmetic_slices(nums.clone());
        println!("  {nums:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (nums, expected) in &cases {
        let result =
            ai_solution::Solution::number_of_arithmetic_slices(nums.clone());
        println!("  {nums:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_arith {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert_eq!(
                        Solution::number_of_arithmetic_slices(
                            vec![1, 2, 3, 4],
                        ),
                        3,
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        Solution::number_of_arithmetic_slices(vec![1]),
                        0,
                    );
                }

                #[test]
                fn test_five_elements() {
                    // [1,2,3], [2,3,4], [3,4,5], [1,2,3,4], [2,3,4,5],
                    // [1,2,3,4,5] = 6
                    assert_eq!(
                        Solution::number_of_arithmetic_slices(
                            vec![1, 2, 3, 4, 5],
                        ),
                        6,
                    );
                }

                #[test]
                fn test_no_arithmetic() {
                    assert_eq!(
                        Solution::number_of_arithmetic_slices(
                            vec![1, 3, 2, 5],
                        ),
                        0,
                    );
                }

                #[test]
                fn test_two_elements() {
                    assert_eq!(
                        Solution::number_of_arithmetic_slices(vec![1, 2]),
                        0,
                    );
                }

                #[test]
                fn test_constant_sequence() {
                    // [7,7,7] = 1 slice
                    assert_eq!(
                        Solution::number_of_arithmetic_slices(
                            vec![7, 7, 7],
                        ),
                        1,
                    );
                }
            }
        };
    }

    test_arith!(my_tests, crate::my_solution::Solution);
    test_arith!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 1_000_000_u32;
        // Build a long arithmetic array
        let nums: Vec<i32> = (0..5000).collect();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::number_of_arithmetic_slices(
                    std::hint::black_box(nums.clone()),
                ),
            );
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::number_of_arithmetic_slices(
                    std::hint::black_box(nums.clone()),
                ),
            );
        }
        let ai_duration = start.elapsed();

        let my_avg = my_duration / iterations;
        let ai_avg = ai_duration / iterations;

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution:  {my_duration:.2?} (avg {my_avg:.2?})");
        println!("AI solution:  {ai_duration:.2?} (avg {ai_avg:.2?})");

        if my_avg <= ai_avg {
            let ratio = ai_avg.as_nanos() as f64
                / my_avg.as_nanos().max(1) as f64;
            println!("-> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_avg.as_nanos() as f64
                / ai_avg.as_nanos().max(1) as f64;
            println!("-> AI solution is {ratio:.2}x faster");
        }
    }
}
