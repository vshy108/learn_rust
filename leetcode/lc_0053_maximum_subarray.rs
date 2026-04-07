// LeetCode 53: Maximum Subarray [Medium]
// https://leetcode.com/problems/maximum-subarray/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 4.321us
//   AI solution avg: 7.335us
//   Ratio: My 1.70x faster
//
// Given an integer array nums, find the subarray with the largest sum,
// and return its sum.
//
// Example 1: nums = [-2,1,-3,4,-1,2,1,-5,4] -> 6  ([4,-1,2,1])
// Example 2: nums = [1] -> 1
// Example 3: nums = [5,4,-1,7,8] -> 23
//
// Constraints:
//   1 <= nums.length <= 10^5
//   -10^4 <= nums[i] <= 10^4
//
// Run:  cargo run --example lc_0053_maximum_subarray
// Test: cargo test --example lc_0053_maximum_subarray
// Bench: cargo test --example lc_0053_maximum_subarray bench -- --nocapture

/// My solution — Kadane's algorithm (explicit if).
///
/// Track current running sum. If it drops below zero, reset to zero.
/// Track the maximum sum seen.
///
/// Time: O(n) — single pass.
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find maximum subarray sum.
        ///
        /// # Arguments
        /// * `nums` - integer array
        ///
        /// # Returns
        /// Largest subarray sum
        pub fn max_sub_array(nums: Vec<i32>) -> i32 {
            let mut max_sum = nums[0];
            let mut current = 0;
            for &num in &nums {
                current += num;
                if current > max_sum {
                    max_sum = current;
                }
                if current < 0 {
                    current = 0;
                }
            }
            max_sum
        }
    }
}

/// AI solution — Kadane's with max() (same algorithm, idiomatic Rust).
///
/// Uses std::cmp::max for both the running-sum reset and the global max.
///
/// Time: O(n) — single pass.
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find maximum subarray sum.
        ///
        /// # Arguments
        /// * `nums` - integer array
        ///
        /// # Returns
        /// Largest subarray sum
        pub fn max_sub_array(nums: Vec<i32>) -> i32 {
            let mut max_sum = i32::MIN;
            let mut current = 0;
            for &num in &nums {
                // Either extend the current subarray or start fresh at num
                current = num.max(current + num);
                max_sum = max_sum.max(current);
            }
            max_sum
        }
    }
}

fn main() {
    let cases = [
        vec![-2, 1, -3, 4, -1, 2, 1, -5, 4],
        vec![1],
        vec![5, 4, -1, 7, 8],
    ];
    for nums in &cases {
        println!(
            "{nums:?} -> my:{}, ai:{}",
            my_solution::Solution::max_sub_array(nums.clone()),
            ai_solution::Solution::max_sub_array(nums.clone()),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_max_sub_array {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_basic() {
                    assert_eq!(
                        <$solution>::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
                        6
                    );
                }

                #[test]
                fn test_single() {
                    assert_eq!(<$solution>::max_sub_array(vec![1]), 1);
                }

                #[test]
                fn test_all_positive() {
                    assert_eq!(<$solution>::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
                }

                #[test]
                fn test_all_negative() {
                    assert_eq!(<$solution>::max_sub_array(vec![-3, -2, -1]), -1);
                }

                #[test]
                fn test_single_negative() {
                    assert_eq!(<$solution>::max_sub_array(vec![-1]), -1);
                }

                #[test]
                fn test_mixed() {
                    assert_eq!(<$solution>::max_sub_array(vec![-2, -1, -3]), -1);
                }

                #[test]
                fn test_ends_with_max() {
                    assert_eq!(<$solution>::max_sub_array(vec![-1, 2, 3]), 5);
                }
            }
        };
    }

    test_max_sub_array!(my, crate::my_solution::Solution);
    test_max_sub_array!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums: Vec<i32> = (0..1000).map(|i| if i % 3 == 0 { -(i as i32) } else { i as i32 }).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::max_sub_array(std::hint::black_box(nums.clone())),
            );
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::max_sub_array(std::hint::black_box(nums.clone())),
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
