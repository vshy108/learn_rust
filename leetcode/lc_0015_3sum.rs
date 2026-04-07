// LeetCode 15: 3Sum [Medium]
// https://leetcode.com/problems/3sum/
//
// Given an integer array `nums`, return all the triplets
// [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k,
// and nums[i] + nums[j] + nums[k] == 0.
//
// Notice that the solution set must not contain duplicate triplets.
//
// Example 1: nums = [-1,0,1,2,-1,-4] -> [[-1,-1,2],[-1,0,1]]
// Example 2: nums = [0,1,1] -> []
// Example 3: nums = [0,0,0] -> [[0,0,0]]
//
// Constraints:
//   3 <= nums.len() <= 3000
//   -10^5 <= nums[i] <= 10^5
//
// Run:  cargo run --example lc_0015_3sum
// Test: cargo test --example lc_0015_3sum
// Bench: cargo test --example lc_0015_3sum bench -- --nocapture
//
// --- Benchmark (100k iterations, 2026-04-07) ---
//   My solution: 201.286us avg (20.13s total)
//   AI solution: 173.820us avg (17.38s total)
//   AI is 1.16x faster
//   Reason: AI adds an upper-bound prune (a + two largest < 0 → skip) that
//   eliminates entire outer-loop iterations on wide-range inputs; my_solution
//   always enters the two-pointer loop even when no valid pair can exist.

/// My solution — sort + two-pointer approach (straightforward).
///
/// Sort the array first, then for each element, use two pointers to find
/// pairs that sum to the negation of the current element. Skip duplicates
/// at every level to avoid duplicate triplets.
///
/// Time: O(n^2) — outer loop O(n), inner two-pointer O(n) each iteration.
/// Space: O(1) — ignoring the output vector and the sort's O(log n) stack.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Returns all unique triplets that sum to zero.
        ///
        /// # Arguments
        /// * `nums` - input array of integers (length >= 3)
        ///
        /// # Returns
        /// A vector of triplets (each a Vec<i32>) summing to zero.
        pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
            let mut result: Vec<Vec<i32>> = Vec::new();
            let n = nums.len();

            // Sort so we can use two pointers and skip duplicates easily.
            nums.sort_unstable();

            for i in 0..n.saturating_sub(2) {
                // Skip duplicate values for the first element.
                if i > 0 && nums[i] == nums[i - 1] {
                    continue;
                }

                // Early termination: if the smallest remaining value is
                // positive, no triplet can sum to zero.
                if nums[i] > 0 {
                    break;
                }

                let mut left = i + 1;
                let mut right = n - 1;

                while left < right {
                    let sum = nums[i] + nums[left] + nums[right];

                    if sum == 0 {
                        result.push(vec![nums[i], nums[left], nums[right]]);

                        // Skip duplicates for the second element.
                        while left < right && nums[left] == nums[left + 1] {
                            left += 1;
                        }
                        // Skip duplicates for the third element.
                        while left < right && nums[right] == nums[right - 1] {
                            right -= 1;
                        }

                        left += 1;
                        right -= 1;
                    } else if sum < 0 {
                        left += 1;
                    } else {
                        right -= 1;
                    }
                }
            }

            result
        }
    }
}

/// AI-optimized solution — sort + two-pointer with tighter pruning.
///
/// Same algorithmic approach as my_solution (O(n^2) is optimal for this
/// problem), but adds an upper-bound pruning check: if nums[i] + the two
/// largest remaining values is still negative, skip ahead. This cuts
/// iterations on inputs with wide value ranges.
///
/// Time: O(n^2) — same asymptotic bound, fewer constant-factor iterations.
/// Space: O(1) — ignoring the output vector and the sort's O(log n) stack.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Returns all unique triplets that sum to zero.
        ///
        /// # Arguments
        /// * `nums` - input array of integers (length >= 3)
        ///
        /// # Returns
        /// A vector of triplets (each a Vec<i32>) summing to zero.
        pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
            let mut result: Vec<Vec<i32>> = Vec::new();
            let n = nums.len();

            nums.sort_unstable();

            // i iterates through potential first elements of the triplet.
            let mut i = 0;
            while i < n.saturating_sub(2) {
                let a = nums[i];

                // Lower-bound prune: if a > 0, no triplet can sum to 0.
                if a > 0 {
                    break;
                }

                // Upper-bound prune: if a + the two largest remaining values
                // is still < 0, no valid triplet exists with this a — skip.
                if a + nums[n - 2] + nums[n - 1] < 0 {
                    i += 1;
                    // Skip duplicates of a.
                    while i < n.saturating_sub(2) && nums[i] == nums[i - 1] {
                        i += 1;
                    }
                    continue;
                }

                let target = -a;
                let mut left = i + 1;
                let mut right = n - 1;

                while left < right {
                    let sum = nums[left] + nums[right];

                    if sum == target {
                        result.push(vec![a, nums[left], nums[right]]);

                        // Advance past duplicates on both sides.
                        let l_val = nums[left];
                        let r_val = nums[right];
                        while left < right && nums[left] == l_val {
                            left += 1;
                        }
                        while left < right && nums[right] == r_val {
                            right -= 1;
                        }
                    } else if sum < target {
                        left += 1;
                    } else {
                        right -= 1;
                    }
                }

                // Skip duplicates of a.
                i += 1;
                while i < n.saturating_sub(2) && nums[i] == nums[i - 1] {
                    i += 1;
                }
            }

            result
        }
    }
}

fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];

    println!("--- My Solution ---");
    let result = my_solution::Solution::three_sum(nums.clone());
    println!("three_sum([-1,0,1,2,-1,-4]) = {result:?}");

    println!("--- AI Solution ---");
    let result = ai_solution::Solution::three_sum(nums);
    println!("three_sum([-1,0,1,2,-1,-4]) = {result:?}");
}

#[cfg(test)]
mod tests {
    /// Macro that generates the same set of tests for any Solution type.
    macro_rules! test_three_sum {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                /// Helper: sort each inner vec and the outer vec for
                /// order-independent comparison.
                fn normalize(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
                    for inner in &mut v {
                        inner.sort_unstable();
                    }
                    v.sort_unstable();
                    v
                }

                #[test]
                fn test_basic() {
                    let result = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
                    assert_eq!(
                        normalize(result),
                        normalize(vec![vec![-1, -1, 2], vec![-1, 0, 1]])
                    );
                }

                #[test]
                fn test_no_triplets() {
                    let result = Solution::three_sum(vec![0, 1, 1]);
                    assert_eq!(result, Vec::<Vec<i32>>::new());
                }

                #[test]
                fn test_all_zeros() {
                    let result = Solution::three_sum(vec![0, 0, 0]);
                    assert_eq!(result, vec![vec![0, 0, 0]]);
                }

                #[test]
                fn test_no_solution_all_positive() {
                    let result = Solution::three_sum(vec![1, 2, 3, 4, 5]);
                    assert_eq!(result, Vec::<Vec<i32>>::new());
                }

                #[test]
                fn test_multiple_duplicates() {
                    let result =
                        Solution::three_sum(vec![-2, 0, 0, 2, 2]);
                    assert_eq!(normalize(result), vec![vec![-2, 0, 2]]);
                }

                #[test]
                fn test_large_range() {
                    // Mix of negatives and positives with one valid triplet.
                    let result =
                        Solution::three_sum(vec![-100000, 50000, 50000]);
                    assert_eq!(result, vec![vec![-100000, 50000, 50000]]);
                }

                #[test]
                fn test_minimum_length() {
                    let result = Solution::three_sum(vec![0, 0, 0]);
                    assert_eq!(result, vec![vec![0, 0, 0]]);
                }
            }
        };
    }

    test_three_sum!(my_tests, crate::my_solution::Solution);
    test_three_sum!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        // Input with many duplicates to exercise dedup logic.
        let nums: Vec<i32> = {
            let mut v = Vec::with_capacity(300);
            for x in -50..=50 {
                v.push(x);
                v.push(x);
                v.push(x);
            }
            v
        };

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::three_sum(
                std::hint::black_box(nums.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::three_sum(
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
