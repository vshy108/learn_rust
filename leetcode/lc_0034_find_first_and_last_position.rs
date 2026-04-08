// LeetCode 34: Find First and Last Position of Element in Sorted Array [Medium]
// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 2.080µs
//   AI solution avg: 2.191µs
//   Ratio: ~tied
//
// Given a sorted array of integers, find the starting and ending position
// of a given target value. If not found, return [-1, -1].
// Must run in O(log n) time.
//
// Example 1: nums = [5,7,7,8,8,10], target = 8 -> [3,4]
// Example 2: nums = [5,7,7,8,8,10], target = 6 -> [-1,-1]
// Example 3: nums = [], target = 0 -> [-1,-1]
//
// Constraints:
//   0 <= nums.length <= 10^5
//   -10^9 <= nums[i] <= 10^9
//   nums is sorted non-decreasing
//   -10^9 <= target <= 10^9
//
// Run:  cargo run --example lc_0034_find_first_and_last_position
// Test: cargo test --example lc_0034_find_first_and_last_position
// Bench: cargo test --example lc_0034_find_first_and_last_position bench -- --nocapture

/// My solution — two binary searches.
///
/// Run binary search twice: once to find the leftmost occurrence,
/// once to find the rightmost.
///
/// Time: O(log n) — two binary searches.
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the first and last index of target.
        ///
        /// # Arguments
        /// * `nums` - sorted array
        /// * `target` - value to find
        ///
        /// # Returns
        /// [first, last] indices, or [-1, -1] if absent.
        pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let first = Self::find_bound(&nums, target, true);
            if first == -1 {
                return vec![-1, -1];
            }
            let last = Self::find_bound(&nums, target, false);
            vec![first, last]
        }

        /// Binary search for leftmost (find_left=true) or rightmost occurrence.
        fn find_bound(nums: &[i32], target: i32, find_left: bool) -> i32 {
            let (mut lo, mut hi) = (0i32, nums.len() as i32 - 1);
            let mut result = -1i32;

            while lo <= hi {
                let mid = lo + (hi - lo) / 2;
                if nums[mid as usize] == target {
                    result = mid;
                    if find_left {
                        hi = mid - 1; // keep searching left
                    } else {
                        lo = mid + 1; // keep searching right
                    }
                } else if nums[mid as usize] < target {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }

            result
        }
    }
}

/// AI-optimized solution — partition_point (stdlib).
///
/// Use Rust's `partition_point` to find lower_bound and upper_bound
/// via the standard library's built-in binary search.
///
/// Time: O(log n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the first and last index of target.
        ///
        /// # Arguments
        /// * `nums` - sorted array
        /// * `target` - value to find
        ///
        /// # Returns
        /// [first, last] indices, or [-1, -1] if absent.
        pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
            // partition_point returns the first index where the predicate is false.
            // lower_bound: first index where nums[i] >= target
            let lo = nums.partition_point(|&x| x < target);
            // upper_bound: first index where nums[i] > target
            let hi = nums.partition_point(|&x| x <= target);

            if lo < hi {
                vec![lo as i32, (hi - 1) as i32]
            } else {
                vec![-1, -1]
            }
        }
    }
}

fn main() {
    let cases = [
        (vec![5, 7, 7, 8, 8, 10], 8),
        (vec![5, 7, 7, 8, 8, 10], 6),
        (vec![], 0),
    ];
    for (nums, target) in &cases {
        println!(
            "{nums:?}, target={target} -> my:{:?}, ai:{:?}",
            my_solution::Solution::search_range(nums.clone(), *target),
            ai_solution::Solution::search_range(nums.clone(), *target),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_search_range {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(
                        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
                        vec![3, 4],
                    );
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(
                        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
                        vec![-1, -1],
                    );
                }

                #[test]
                fn test_empty() {
                    assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1],);
                }

                #[test]
                fn test_single_found() {
                    assert_eq!(Solution::search_range(vec![5], 5), vec![0, 0],);
                }

                #[test]
                fn test_single_not_found() {
                    assert_eq!(Solution::search_range(vec![5], 3), vec![-1, -1],);
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(Solution::search_range(vec![2, 2, 2, 2], 2), vec![0, 3],);
                }

                #[test]
                fn test_first_element() {
                    assert_eq!(Solution::search_range(vec![1, 2, 3, 4, 5], 1), vec![0, 0],);
                }

                #[test]
                fn test_last_element() {
                    assert_eq!(Solution::search_range(vec![1, 2, 3, 4, 5], 5), vec![4, 4],);
                }

                #[test]
                fn test_target_smaller_than_all() {
                    assert_eq!(Solution::search_range(vec![3, 4, 5], 1), vec![-1, -1],);
                }

                #[test]
                fn test_target_larger_than_all() {
                    assert_eq!(Solution::search_range(vec![3, 4, 5], 10), vec![-1, -1],);
                }
            }
        };
    }

    test_search_range!(my_tests, crate::my_solution::Solution);
    test_search_range!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        // Large sorted array with duplicates.
        let mut nums: Vec<i32> = (0..10000)
            .flat_map(|x| std::iter::repeat(x).take(3))
            .collect();
        nums.sort_unstable();
        let target = 5000;

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::search_range(
                std::hint::black_box(nums.clone()),
                std::hint::black_box(target),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::search_range(
                std::hint::black_box(nums.clone()),
                std::hint::black_box(target),
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
