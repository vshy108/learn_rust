// LeetCode 41: First Missing Positive [Hard]
// https://leetcode.com/problems/first-missing-positive/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 19.363µs
//   AI solution avg: 19.769µs
//   Ratio: ~tied
//
// Given an unsorted integer array nums, return the smallest missing
// positive integer. Must run in O(n) time and O(1) auxiliary space.
//
// Example 1: nums = [1,2,0] -> 3
// Example 2: nums = [3,4,-1,1] -> 2
// Example 3: nums = [7,8,9,11,12] -> 1
//
// Constraints:
//   1 <= nums.length <= 10^5
//   -2^31 <= nums[i] <= 2^31 - 1
//
// Run:  cargo run --example lc_0041_first_missing_positive
// Test: cargo test --example lc_0041_first_missing_positive
// Bench: cargo test --example lc_0041_first_missing_positive bench -- --nocapture

/// My solution — cyclic sort (index-as-hash).
///
/// Place each value v in nums[v-1] if v is in [1, n]. After rearranging,
/// the first index i where nums[i] != i+1 gives the answer.
///
/// Time: O(n) — each element swapped at most once.
/// Space: O(1) — in-place rearrangement.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the smallest missing positive integer.
        ///
        /// # Arguments
        /// * `nums` - unsorted integer array
        ///
        /// # Returns
        /// Smallest positive integer not in nums.
        pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
            let n = nums.len();

            // Cyclic sort: put each value v at index v-1.
            for i in 0..n {
                // Keep swapping until nums[i] is in the right place or invalid.
                while nums[i] > 0
                    && (nums[i] as usize) <= n
                    && nums[nums[i] as usize - 1] != nums[i]
                {
                    let target = nums[i] as usize - 1;
                    nums.swap(i, target);
                }
            }

            // Find the first index where nums[i] != i+1.
            for i in 0..n {
                if nums[i] != (i as i32 + 1) {
                    return i as i32 + 1;
                }
            }

            // All 1..=n are present.
            n as i32 + 1
        }
    }
}

/// AI-optimized solution — same cyclic sort (it's already optimal).
///
/// The cyclic sort approach is the standard O(n) time / O(1) space solution.
/// Both implementations are identical in logic.
///
/// Time: O(n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the smallest missing positive integer.
        pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
            let n = nums.len();

            for i in 0..n {
                while nums[i] > 0
                    && (nums[i] as usize) <= n
                    && nums[nums[i] as usize - 1] != nums[i]
                {
                    let target = nums[i] as usize - 1;
                    nums.swap(i, target);
                }
            }

            for i in 0..n {
                if nums[i] != (i as i32 + 1) {
                    return i as i32 + 1;
                }
            }

            n as i32 + 1
        }
    }
}

fn main() {
    let cases = [vec![1, 2, 0], vec![3, 4, -1, 1], vec![7, 8, 9, 11, 12]];
    for nums in &cases {
        println!(
            "{nums:?} -> my:{}, ai:{}",
            my_solution::Solution::first_missing_positive(nums.clone()),
            ai_solution::Solution::first_missing_positive(nums.clone()),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_first_missing_positive {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
                }

                #[test]
                fn test_example_3() {
                    assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
                }

                #[test]
                fn test_single_one() {
                    assert_eq!(Solution::first_missing_positive(vec![1]), 2);
                }

                #[test]
                fn test_single_two() {
                    assert_eq!(Solution::first_missing_positive(vec![2]), 1);
                }

                #[test]
                fn test_all_negative() {
                    assert_eq!(Solution::first_missing_positive(vec![-1, -2, -3]), 1);
                }

                #[test]
                fn test_consecutive() {
                    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5]), 6);
                }

                #[test]
                fn test_duplicates() {
                    assert_eq!(Solution::first_missing_positive(vec![1, 1]), 2);
                }

                #[test]
                fn test_large_values() {
                    assert_eq!(
                        Solution::first_missing_positive(vec![i32::MAX, i32::MIN, 1, 2]),
                        3,
                    );
                }
            }
        };
    }

    test_first_missing_positive!(my_tests, crate::my_solution::Solution);
    test_first_missing_positive!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        // Array of 1..=999 with 500 missing.
        let nums: Vec<i32> = (1..=999).filter(|x| x % 2 == 0).collect();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::first_missing_positive(
                std::hint::black_box(nums.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::first_missing_positive(
                std::hint::black_box(nums.clone()),
            ));
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
