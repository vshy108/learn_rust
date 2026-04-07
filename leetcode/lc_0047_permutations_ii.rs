// LeetCode 47: Permutations II [Medium]
// https://leetcode.com/problems/permutations-ii/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 8.005us
//   AI solution avg: 8.025us
//   Ratio: ~tied
//
// Given a collection of numbers, `nums`, that might contain duplicates,
// return all possible unique permutations in any order.
//
// Example 1: nums = [1,1,2] -> [[1,1,2],[1,2,1],[2,1,1]]
// Example 2: nums = [1,2,3] -> [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
//
// Constraints:
//   1 <= nums.length <= 8
//   -10 <= nums[i] <= 10
//
// Run:  cargo run --example lc_0047_permutations_ii
// Test: cargo test --example lc_0047_permutations_ii
// Bench: cargo test --example lc_0047_permutations_ii bench -- --nocapture

/// My solution — sort + backtracking with skip-duplicate logic.
///
/// Sort first so duplicates are adjacent. During backtracking, skip a
/// candidate if it equals the previous candidate at the same recursion level.
///
/// Time: O(n! / (d1! * d2! * ...)) where di is the frequency of duplicate i.
/// Space: O(n) recursion depth + O(n!) output storage.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Generate all unique permutations of `nums`.
        ///
        /// # Arguments
        /// * `nums` - collection that may contain duplicates
        ///
        /// # Returns
        /// All unique permutations
        pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
            nums.sort_unstable();
            let mut result = Vec::new();
            let mut current = Vec::with_capacity(nums.len());
            let mut used = vec![false; nums.len()];
            Self::backtrack(&nums, &mut used, &mut current, &mut result);
            result
        }

        fn backtrack(
            nums: &[i32],
            used: &mut Vec<bool>,
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if current.len() == nums.len() {
                result.push(current.clone());
                return;
            }
            for i in 0..nums.len() {
                // Skip used elements
                if used[i] {
                    continue;
                }
                // Skip duplicates: if nums[i] == nums[i-1] and nums[i-1] was
                // NOT used at this level, we'd generate a duplicate branch.
                if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                    continue;
                }
                used[i] = true;
                current.push(nums[i]);
                Self::backtrack(nums, used, current, result);
                current.pop();
                used[i] = false;
            }
        }
    }
}

/// AI solution — same sort+backtrack approach (this is the canonical optimal).
///
/// The duplicate-skip condition `!used[i-1]` (as opposed to `used[i-1]`)
/// prunes earlier, producing fewer recursive calls on average.
///
/// Time: O(n! / (d1! * d2! * ...))
/// Space: O(n) recursion depth + O(n!) output storage.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Generate all unique permutations of `nums`.
        ///
        /// # Arguments
        /// * `nums` - collection that may contain duplicates
        ///
        /// # Returns
        /// All unique permutations
        pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
            nums.sort_unstable();
            let n = nums.len();
            let mut result = Vec::new();
            let mut current = Vec::with_capacity(n);
            let mut used = vec![false; n];
            Self::backtrack(&nums, &mut used, &mut current, &mut result);
            result
        }

        fn backtrack(
            nums: &[i32],
            used: &mut Vec<bool>,
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if current.len() == nums.len() {
                result.push(current.clone());
                return;
            }
            for i in 0..nums.len() {
                if used[i] {
                    continue;
                }
                // Identical skip logic: prune duplicate branches
                if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                    continue;
                }
                used[i] = true;
                current.push(nums[i]);
                Self::backtrack(nums, used, current, result);
                current.pop();
                used[i] = false;
            }
        }
    }
}

fn main() {
    let cases = [vec![1, 1, 2], vec![1, 2, 3]];
    for nums in &cases {
        let my = my_solution::Solution::permute_unique(nums.clone());
        let ai = ai_solution::Solution::permute_unique(nums.clone());
        println!("{nums:?} -> my:{my:?}");
        println!("{nums:?} -> ai:{ai:?}");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_permute_unique {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::super::*;

                fn sorted(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
                    v.sort();
                    v
                }

                #[test]
                fn test_with_duplicates() {
                    let result = sorted(<$solution>::permute_unique(vec![1, 1, 2]));
                    let expected = sorted(vec![
                        vec![1, 1, 2],
                        vec![1, 2, 1],
                        vec![2, 1, 1],
                    ]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_no_duplicates() {
                    let result = sorted(<$solution>::permute_unique(vec![1, 2, 3]));
                    assert_eq!(result.len(), 6);
                }

                #[test]
                fn test_single() {
                    let result = <$solution>::permute_unique(vec![1]);
                    assert_eq!(result, vec![vec![1]]);
                }

                #[test]
                fn test_all_same() {
                    let result = <$solution>::permute_unique(vec![1, 1, 1]);
                    assert_eq!(result, vec![vec![1, 1, 1]]);
                }

                #[test]
                fn test_two_elements() {
                    let result = sorted(<$solution>::permute_unique(vec![2, 1]));
                    assert_eq!(result, vec![vec![1, 2], vec![2, 1]]);
                }

                #[test]
                fn test_two_same() {
                    let result = <$solution>::permute_unique(vec![1, 1]);
                    assert_eq!(result, vec![vec![1, 1]]);
                }

                #[test]
                fn test_negative_numbers() {
                    let result = sorted(<$solution>::permute_unique(vec![-1, -1, 2]));
                    let expected = sorted(vec![
                        vec![-1, -1, 2],
                        vec![-1, 2, -1],
                        vec![2, -1, -1],
                    ]);
                    assert_eq!(result, expected);
                }
            }
        };
    }

    test_permute_unique!(my, crate::my_solution::Solution);
    test_permute_unique!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums = vec![1, 1, 2, 2, 3];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::permute_unique(std::hint::black_box(nums.clone())),
            );
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::permute_unique(std::hint::black_box(nums.clone())),
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
