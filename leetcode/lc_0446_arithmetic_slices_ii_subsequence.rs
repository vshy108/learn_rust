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

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
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
            todo!()
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
