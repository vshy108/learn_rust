// LeetCode 16: 3Sum Closest [Medium]
// https://leetcode.com/problems/3sum-closest/
//
// Given an integer array `nums` of length n and an integer `target`, find
// three integers in `nums` such that the sum is closest to `target`.
// Return the sum of the three integers.
// You may assume that each input would have exactly one solution.
//
// Example 1: nums = [-1,2,1,-4], target = 1 -> 2  (because -1 + 2 + 1 = 2)
// Example 2: nums = [0,0,0], target = 1 -> 0      (because 0 + 0 + 0 = 0)
//
// Constraints:
//   3 <= nums.length <= 500
//   -1000 <= nums[i] <= 1000
//   -10^4 <= target <= 10^4
//
// Run:  cargo run --example lc_0016_3sum_closest
// Test: cargo test --example lc_0016_3sum_closest
// Bench: cargo test --example lc_0016_3sum_closest bench -- --nocapture
//
// --- Benchmark (100k iterations, 2026-04-07) ---
//   My solution: 6.001us avg (600ms total)
//   AI solution: 6.003us avg (600ms total)
//   Virtually identical (1.00x)
//   Reason: On a 500-element sorted array where the target is reachable,
//   both solutions find the exact match quickly via two-pointer convergence.
//   The AI's bound pruning doesn't help much when the answer is near the
//   middle of the range. Pruning shines on skewed targets.

/// My solution — sort + two-pointer, tracking the closest sum seen so far.
///
/// Sort the array, then for each element fix it as the first of the triplet
/// and use two pointers on the remaining subarray. Update the closest sum
/// whenever the absolute difference improves.
///
/// Time: O(n^2) — outer loop O(n), inner two-pointer O(n) each.
/// Space: O(1) — ignoring the sort's O(log n) stack.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the sum of three elements closest to `target`.
        ///
        /// # Arguments
        /// * `nums` - integer array (length >= 3)
        /// * `target` - the target sum
        ///
        /// # Returns
        /// The three-element sum closest to `target`.
        pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
            nums.sort_unstable();
            let n = nums.len();
            // Initialize with the first three elements' sum.
            let mut closest = nums[0] + nums[1] + nums[2];

            for i in 0..n.saturating_sub(2) {
                // Skip duplicate first elements — same value gives same
                // two-pointer sweep, so it can't improve the answer.
                if i > 0 && nums[i] == nums[i - 1] {
                    continue;
                }

                let mut left = i + 1;
                let mut right = n - 1;

                while left < right {
                    let sum = nums[i] + nums[left] + nums[right];

                    if sum == target {
                        // Exact match — can't get closer than 0 difference.
                        return target;
                    }

                    // Update closest if this sum is nearer to target.
                    // .abs() on the difference gives the distance to compare.
                    if (sum - target).abs() < (closest - target).abs() {
                        closest = sum;
                    }

                    if sum < target {
                        left += 1;
                    } else {
                        right -= 1;
                    }
                }
            }

            closest
        }
    }
}

/// AI-optimized solution — sort + two-pointer with early exit and tighter
/// pruning.
///
/// Same O(n^2) algorithm but adds:
/// 1. Lower-bound prune: if the minimum possible sum with nums[i] already
///    exceeds the current best distance, skip.
/// 2. Upper-bound prune: if the maximum possible sum with nums[i] is still
///    below target and farther than current best, skip.
/// 3. Exact match returns immediately (same as my_solution).
///
/// Time: O(n^2) — same asymptotic, fewer iterations in practice.
/// Space: O(1) — ignoring the sort's O(log n) stack.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the sum of three elements closest to `target`.
        ///
        /// # Arguments
        /// * `nums` - integer array (length >= 3)
        /// * `target` - the target sum
        ///
        /// # Returns
        /// The three-element sum closest to `target`.
        pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
            nums.sort_unstable();
            let n = nums.len();
            let mut closest = nums[0] + nums[1] + nums[2];

            for i in 0..n.saturating_sub(2) {
                if i > 0 && nums[i] == nums[i - 1] {
                    continue;
                }

                // Lower-bound prune: the smallest sum with nums[i] as first
                // element is nums[i] + nums[i+1] + nums[i+2].
                let min_sum = nums[i] + nums[i + 1] + nums[i + 2];
                if min_sum > target && (min_sum - target).abs() >= (closest - target).abs() {
                    // All sums with this i are >= min_sum, which is already
                    // farther from target than our current best. Since the
                    // array is sorted and i only increases, future min_sums
                    // are even larger — we can break entirely.
                    break;
                }

                // Upper-bound prune: the largest sum with nums[i] is
                // nums[i] + nums[n-2] + nums[n-1].
                let max_sum = nums[i] + nums[n - 2] + nums[n - 1];
                if max_sum < target {
                    // Best we can do with this i is max_sum. If it's closer
                    // than current best, update closest and skip to next i.
                    if (max_sum - target).abs() < (closest - target).abs() {
                        closest = max_sum;
                    }
                    continue;
                }

                let mut left = i + 1;
                let mut right = n - 1;

                while left < right {
                    let sum = nums[i] + nums[left] + nums[right];

                    if sum == target {
                        return target;
                    }

                    if (sum - target).abs() < (closest - target).abs() {
                        closest = sum;
                    }

                    if sum < target {
                        left += 1;
                    } else {
                        right -= 1;
                    }
                }
            }

            closest
        }
    }
}

fn main() {
    let nums = vec![-1, 2, 1, -4];
    let target = 1;

    println!("--- My Solution ---");
    let result = my_solution::Solution::three_sum_closest(nums.clone(), target);
    println!("three_sum_closest([-1,2,1,-4], {target}) = {result}");

    println!("--- AI Solution ---");
    let result = ai_solution::Solution::three_sum_closest(nums, target);
    println!("three_sum_closest([-1,2,1,-4], {target}) = {result}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_three_sum_closest {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_basic() {
                    assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
                }

                #[test]
                fn test_all_zeros() {
                    assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
                }

                #[test]
                fn test_exact_match() {
                    assert_eq!(Solution::three_sum_closest(vec![1, 1, 1, 0], 3), 3);
                }

                #[test]
                fn test_negative_target() {
                    assert_eq!(
                        Solution::three_sum_closest(vec![-3, -2, -5, 3, -4], -1),
                        -2 // -3 + -2 + 3 = -2
                    );
                }

                #[test]
                fn test_minimum_length() {
                    assert_eq!(Solution::three_sum_closest(vec![1, 2, 3], 6), 6);
                }

                #[test]
                fn test_large_target() {
                    // Target far above all sums — closest is the max sum.
                    assert_eq!(
                        Solution::three_sum_closest(vec![1, 2, 3, 4, 5], 100),
                        12 // 3 + 4 + 5
                    );
                }

                #[test]
                fn test_duplicates() {
                    assert_eq!(
                        Solution::three_sum_closest(vec![1, 1, 1, 1], 2),
                        3 // 1 + 1 + 1
                    );
                }
            }
        };
    }

    test_three_sum_closest!(my_tests, crate::my_solution::Solution);
    test_three_sum_closest!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        // 500-element array spanning [-250, 249] — exercises the pruning.
        let nums: Vec<i32> = (-250..250).collect();
        let target = 37;

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::three_sum_closest(
                std::hint::black_box(nums.clone()),
                target,
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::three_sum_closest(
                std::hint::black_box(nums.clone()),
                target,
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
