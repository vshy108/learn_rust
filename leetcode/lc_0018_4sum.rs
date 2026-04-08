// LeetCode 18: 4Sum [Medium]
// https://leetcode.com/problems/4sum/
//
// Given an array `nums` of n integers, return an array of all unique
// quadruplets [nums[a], nums[b], nums[c], nums[d]] such that:
//   - 0 <= a, b, c, d < n and all distinct
//   - nums[a] + nums[b] + nums[c] + nums[d] == target
//
// Example 1: nums = [1,0,-1,0,-2,2], target = 0
//            -> [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
// Example 2: nums = [2,2,2,2,2], target = 8
//            -> [[2,2,2,2]]
//
// Constraints:
//   1 <= nums.length <= 200
//   -10^9 <= nums[i] <= 10^9
//   -10^9 <= target <= 10^9
//
// Run:  cargo run --example lc_0018_4sum
// Test: cargo test --example lc_0018_4sum
// Bench: cargo test --example lc_0018_4sum bench -- --nocapture
//
// --- Benchmark (10k iterations, 2026-04-07) ---
//   My solution: 3.101ms avg (31.01s total)
//   AI solution: 3.276ms avg (32.76s total)
//   My solution is 1.06x faster
//   Reason: Both use identical O(n^3) sort + two-loop + two-pointer with
//   bounding prunes. The AI's while-loop dedup style has marginally more
//   branching overhead vs the for-loop + continue approach. Functionally
//   equivalent; the difference is within noise.

/// My solution — sort + two nested loops + two-pointer.
///
/// Generalises the 3Sum approach: sort, fix two elements with nested loops,
/// then use two pointers for the remaining pair. Skip duplicates at every
/// level. Uses i64 for the intermediate sum to avoid i32 overflow since
/// nums[i] can be up to 10^9.
///
/// Time: O(n^3) — two outer loops O(n^2), inner two-pointer O(n).
/// Space: O(1) — ignoring output and sort stack.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return all unique quadruplets summing to `target`.
        ///
        /// # Arguments
        /// * `nums` - integer array (length >= 1)
        /// * `target` - target sum
        ///
        /// # Returns
        /// Vector of quadruplets (each a Vec<i32>).
        pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
            let mut result: Vec<Vec<i32>> = Vec::new();
            let n = nums.len();
            if n < 4 {
                return result;
            }

            nums.sort_unstable();
            // Cast target to i64 to prevent overflow when summing four i32s.
            let target = target as i64;

            for i in 0..n - 3 {
                // Skip duplicate first element.
                if i > 0 && nums[i] == nums[i - 1] {
                    continue;
                }

                // Lower-bound prune: smallest possible sum with nums[i].
                let min_sum =
                    nums[i] as i64 + nums[i + 1] as i64 + nums[i + 2] as i64 + nums[i + 3] as i64;
                if min_sum > target {
                    break;
                }

                // Upper-bound prune: largest possible sum with nums[i].
                let max_sum =
                    nums[i] as i64 + nums[n - 3] as i64 + nums[n - 2] as i64 + nums[n - 1] as i64;
                if max_sum < target {
                    continue;
                }

                for j in (i + 1)..n - 2 {
                    // Skip duplicate second element.
                    if j > i + 1 && nums[j] == nums[j - 1] {
                        continue;
                    }

                    // Inner lower-bound prune.
                    let min_sum =
                        nums[i] as i64 + nums[j] as i64 + nums[j + 1] as i64 + nums[j + 2] as i64;
                    if min_sum > target {
                        break;
                    }

                    // Inner upper-bound prune.
                    let max_sum =
                        nums[i] as i64 + nums[j] as i64 + nums[n - 2] as i64 + nums[n - 1] as i64;
                    if max_sum < target {
                        continue;
                    }

                    let remainder = target - nums[i] as i64 - nums[j] as i64;
                    let mut left = j + 1;
                    let mut right = n - 1;

                    while left < right {
                        let sum = nums[left] as i64 + nums[right] as i64;

                        if sum == remainder {
                            result.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                            // Skip duplicates on both sides.
                            while left < right && nums[left] == nums[left + 1] {
                                left += 1;
                            }
                            while left < right && nums[right] == nums[right - 1] {
                                right -= 1;
                            }
                            left += 1;
                            right -= 1;
                        } else if sum < remainder {
                            left += 1;
                        } else {
                            right -= 1;
                        }
                    }
                }
            }

            result
        }
    }
}

/// AI-optimized solution — same algorithm with identical pruning.
///
/// For 4Sum, the sort + two loops + two-pointer with bounding prunes is
/// already the optimal practical approach. This implementation uses the
/// same strategy but structures the dedup slightly differently — advancing
/// pointers past duplicates before the next iteration rather than checking
/// previous values.
///
/// Time: O(n^3) — same asymptotic complexity.
/// Space: O(1) — ignoring output and sort stack.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return all unique quadruplets summing to `target`.
        ///
        /// # Arguments
        /// * `nums` - integer array (length >= 1)
        /// * `target` - target sum
        ///
        /// # Returns
        /// Vector of quadruplets (each a Vec<i32>).
        pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
            let mut result: Vec<Vec<i32>> = Vec::new();
            let n = nums.len();
            if n < 4 {
                return result;
            }

            nums.sort_unstable();
            let target = target as i64;

            let mut i = 0;
            while i < n - 3 {
                // Lower-bound prune.
                let lo =
                    nums[i] as i64 + nums[i + 1] as i64 + nums[i + 2] as i64 + nums[i + 3] as i64;
                if lo > target {
                    break;
                }
                // Upper-bound prune.
                let hi =
                    nums[i] as i64 + nums[n - 3] as i64 + nums[n - 2] as i64 + nums[n - 1] as i64;
                if hi < target {
                    i += 1;
                    while i < n - 3 && nums[i] == nums[i - 1] {
                        i += 1;
                    }
                    continue;
                }

                let mut j = i + 1;
                while j < n - 2 {
                    // Inner lower-bound prune.
                    let lo =
                        nums[i] as i64 + nums[j] as i64 + nums[j + 1] as i64 + nums[j + 2] as i64;
                    if lo > target {
                        break;
                    }
                    // Inner upper-bound prune.
                    let hi =
                        nums[i] as i64 + nums[j] as i64 + nums[n - 2] as i64 + nums[n - 1] as i64;
                    if hi < target {
                        j += 1;
                        while j < n - 2 && nums[j] == nums[j - 1] {
                            j += 1;
                        }
                        continue;
                    }

                    let need = target - nums[i] as i64 - nums[j] as i64;
                    let mut lo_ptr = j + 1;
                    let mut hi_ptr = n - 1;

                    while lo_ptr < hi_ptr {
                        let s = nums[lo_ptr] as i64 + nums[hi_ptr] as i64;
                        if s == need {
                            result.push(vec![nums[i], nums[j], nums[lo_ptr], nums[hi_ptr]]);
                            let lv = nums[lo_ptr];
                            let rv = nums[hi_ptr];
                            while lo_ptr < hi_ptr && nums[lo_ptr] == lv {
                                lo_ptr += 1;
                            }
                            while lo_ptr < hi_ptr && nums[hi_ptr] == rv {
                                hi_ptr -= 1;
                            }
                        } else if s < need {
                            lo_ptr += 1;
                        } else {
                            hi_ptr -= 1;
                        }
                    }

                    // Advance j past duplicates.
                    j += 1;
                    while j < n - 2 && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                }

                // Advance i past duplicates.
                i += 1;
                while i < n - 3 && nums[i] == nums[i - 1] {
                    i += 1;
                }
            }

            result
        }
    }
}

fn main() {
    let nums = vec![1, 0, -1, 0, -2, 2];
    let target = 0;

    println!("--- My Solution ---");
    let result = my_solution::Solution::four_sum(nums.clone(), target);
    println!("four_sum([1,0,-1,0,-2,2], {target}) = {result:?}");

    println!("--- AI Solution ---");
    let result = ai_solution::Solution::four_sum(nums, target);
    println!("four_sum([1,0,-1,0,-2,2], {target}) = {result:?}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_four_sum {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                fn normalize(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
                    for inner in &mut v {
                        inner.sort_unstable();
                    }
                    v.sort_unstable();
                    v
                }

                #[test]
                fn test_basic() {
                    let result = Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0);
                    assert_eq!(
                        normalize(result),
                        normalize(vec![
                            vec![-2, -1, 1, 2],
                            vec![-2, 0, 0, 2],
                            vec![-1, 0, 0, 1],
                        ])
                    );
                }

                #[test]
                fn test_all_same() {
                    let result = Solution::four_sum(vec![2, 2, 2, 2, 2], 8);
                    assert_eq!(result, vec![vec![2, 2, 2, 2]]);
                }

                #[test]
                fn test_too_short() {
                    let result = Solution::four_sum(vec![1, 2, 3], 6);
                    assert!(result.is_empty());
                }

                #[test]
                fn test_no_solution() {
                    let result = Solution::four_sum(vec![1, 2, 3, 4], 100);
                    assert!(result.is_empty());
                }

                #[test]
                fn test_large_values() {
                    // Tests i64 overflow protection: 10^9 * 4 > i32::MAX.
                    let result = Solution::four_sum(
                        vec![1000000000, 1000000000, 1000000000, 1000000000],
                        -294967296, // not reachable
                    );
                    assert!(result.is_empty());
                }

                #[test]
                fn test_negative_target() {
                    let result = Solution::four_sum(vec![-3, -2, -1, 0, 0, 1, 2, 3], -2);
                    // Expected: [-3,-2,0,3], [-3,-2,1,2], [-3,-1,0,2],
                    //           [-3,0,0,1], [-2,-1,0,1]
                    assert_eq!(normalize(result).len(), 5);
                }

                #[test]
                fn test_exact_four() {
                    let result = Solution::four_sum(vec![0, 0, 0, 0], 0);
                    assert_eq!(result, vec![vec![0, 0, 0, 0]]);
                }
            }
        };
    }

    test_four_sum!(my_tests, crate::my_solution::Solution);
    test_four_sum!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 10_000;
        let nums: Vec<i32> = (-50..=50).collect(); // 101 elements
        let target = 0;

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::four_sum(
                std::hint::black_box(nums.clone()),
                target,
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::four_sum(
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
