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

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
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
            // n = nums.length
            // total_sum = nums.sum
            // min_diff = nil; result_index = 0; left_sum = 0
            // (0...n).each do |i|
            //   left_sum += nums[i]
            //   total_sum -= nums[i]
            //   left_avg = left_sum / (i + 1)
            //   right_avg = i == n - 1 ? 0 : total_sum / (n - i - 1)
            //   diff = (left_avg - right_avg).abs
            //   if min_diff.nil? || diff < min_diff
            //     min_diff = diff; result_index = i
            //   end
            // end
            // result_index
            todo!()
        }
    }
}

/// AI solution module.
///
/// Time:  O(?)
/// Space: O(?)
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
            todo!()
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
                    assert_eq!(Solution::minimum_average_difference(vec![1, 2, 3, 4, 5]), 2);
                }

                #[test]
                fn test_large_values() {
                    assert_eq!(
                        Solution::minimum_average_difference(vec![100000, 0, 0, 0, 0]),
                        1
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
