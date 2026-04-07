// LeetCode 33: Search in Rotated Sorted Array [Medium]
// https://leetcode.com/problems/search-in-rotated-sorted-array/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 478ns
//   AI solution avg: 473ns
//   Ratio: ~tied
//
// An integer array nums sorted in ascending order (with distinct values)
// is possibly rotated at an unknown pivot index. Given the rotated array
// and a target value, return its index, or -1 if not found.
// Must run in O(log n) time.
//
// Example 1: nums = [4,5,6,7,0,1,2], target = 0 -> 4
// Example 2: nums = [4,5,6,7,0,1,2], target = 3 -> -1
// Example 3: nums = [1], target = 0 -> -1
//
// Constraints:
//   1 <= nums.length <= 5000
//   -10^4 <= nums[i] <= 10^4
//   All values are distinct
//   nums is an ascending array that is possibly rotated
//   -10^4 <= target <= 10^4
//
// Run:  cargo run --example lc_0033_search_in_rotated_sorted_array
// Test: cargo test --example lc_0033_search_in_rotated_sorted_array
// Bench: cargo test --example lc_0033_search_in_rotated_sorted_array bench -- --nocapture

/// My solution — modified binary search.
///
/// At each step, determine which half is sorted. If the target lies in the
/// sorted half, narrow there; otherwise, search the other half.
///
/// Time: O(log n).
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Search for target in a rotated sorted array.
        ///
        /// # Arguments
        /// * `nums` - rotated sorted array of distinct integers
        /// * `target` - value to find
        ///
        /// # Returns
        /// Index of target, or -1 if not found.
        pub fn search(nums: Vec<i32>, target: i32) -> i32 {
            let (mut lo, mut hi) = (0i32, nums.len() as i32 - 1);

            while lo <= hi {
                let mid = lo + (hi - lo) / 2;
                let m = mid as usize;

                if nums[m] == target {
                    return mid;
                }

                // Check which half is sorted.
                if nums[lo as usize] <= nums[m] {
                    // Left half [lo..mid] is sorted.
                    if nums[lo as usize] <= target && target < nums[m] {
                        hi = mid - 1; // target is in the sorted left half
                    } else {
                        lo = mid + 1; // search the right half
                    }
                } else {
                    // Right half [mid..hi] is sorted.
                    if nums[m] < target && target <= nums[hi as usize] {
                        lo = mid + 1; // target is in the sorted right half
                    } else {
                        hi = mid - 1; // search the left half
                    }
                }
            }

            -1
        }
    }
}

/// AI-optimized solution — same algorithm (binary search is already optimal).
///
/// Identical logic with usize indices to avoid i32 casts.
///
/// Time: O(log n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Search for target in a rotated sorted array.
        ///
        /// # Arguments
        /// * `nums` - rotated sorted array of distinct integers
        /// * `target` - value to find
        ///
        /// # Returns
        /// Index of target, or -1 if not found.
        pub fn search(nums: Vec<i32>, target: i32) -> i32 {
            let n = nums.len();
            if n == 0 {
                return -1;
            }

            let (mut lo, mut hi) = (0usize, n - 1);

            while lo <= hi {
                let mid = lo + (hi - lo) / 2;

                if nums[mid] == target {
                    return mid as i32;
                }

                if nums[lo] <= nums[mid] {
                    // Left half is sorted.
                    if nums[lo] <= target && target < nums[mid] {
                        // Prevent underflow: mid is at least 1 here because
                        // nums[lo] <= nums[mid] && target < nums[mid] implies mid > lo.
                        hi = mid - 1;
                    } else {
                        lo = mid + 1;
                    }
                } else {
                    // Right half is sorted.
                    if nums[mid] < target && target <= nums[hi] {
                        lo = mid + 1;
                    } else {
                        if mid == 0 {
                            break; // prevent usize underflow
                        }
                        hi = mid - 1;
                    }
                }
            }

            -1
        }
    }
}

fn main() {
    let cases = [
        (vec![4, 5, 6, 7, 0, 1, 2], 0),
        (vec![4, 5, 6, 7, 0, 1, 2], 3),
        (vec![1], 0),
    ];
    for (nums, target) in &cases {
        println!(
            "{nums:?}, target={target} -> my:{}, ai:{}",
            my_solution::Solution::search(nums.clone(), *target),
            ai_solution::Solution::search(nums.clone(), *target),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_search {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
                }

                #[test]
                fn test_single_not_found() {
                    assert_eq!(Solution::search(vec![1], 0), -1);
                }

                #[test]
                fn test_single_found() {
                    assert_eq!(Solution::search(vec![1], 1), 0);
                }

                #[test]
                fn test_no_rotation() {
                    assert_eq!(Solution::search(vec![1, 2, 3, 4, 5], 3), 2);
                }

                #[test]
                fn test_first_element() {
                    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 4), 0);
                }

                #[test]
                fn test_last_element() {
                    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 2), 6);
                }

                #[test]
                fn test_two_elements_rotated() {
                    assert_eq!(Solution::search(vec![2, 1], 1), 1);
                }

                #[test]
                fn test_pivot_at_end() {
                    assert_eq!(Solution::search(vec![2, 3, 4, 5, 1], 1), 4);
                }

                #[test]
                fn test_target_at_pivot() {
                    assert_eq!(Solution::search(vec![3, 4, 5, 1, 2], 5), 2);
                }
            }
        };
    }

    test_search!(my_tests, crate::my_solution::Solution);
    test_search!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        let nums: Vec<i32> = (500..5000).chain(0..500).collect();
        let target = 42;

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::search(
                std::hint::black_box(nums.clone()),
                std::hint::black_box(target),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::search(
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
