// LeetCode 46: Permutations [Medium]
// https://leetcode.com/problems/permutations/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 193.859µs
//   AI solution avg: 194.401µs
//   Ratio: ~tied
//
// Given an array nums of distinct integers, return all possible permutations
// in any order.
//
// Example 1: nums = [1,2,3] -> [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
// Example 2: nums = [0,1]   -> [[0,1],[1,0]]
// Example 3: nums = [1]     -> [[1]]
//
// Constraints:
//   1 <= nums.length <= 6
//   -10 <= nums[i] <= 10
//   All integers are unique
//
// Run:  cargo run --example lc_0046_permutations
// Test: cargo test --example lc_0046_permutations
// Bench: cargo test --example lc_0046_permutations bench -- --nocapture

/// My solution — backtracking with swap.
///
/// For each position, swap the current element with each subsequent element,
/// recurse, then swap back.
///
/// Time: O(n * n!) — n! permutations, each takes O(n) to copy.
/// Space: O(n) recursion depth.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Generate all permutations of nums.
        ///
        /// # Arguments
        /// * `nums` - array of distinct integers
        ///
        /// # Returns
        /// All permutations.
        pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
            let mut result = Vec::new();
            Self::backtrack(&mut nums, 0, &mut result);
            result
        }

        fn backtrack(nums: &mut Vec<i32>, start: usize, result: &mut Vec<Vec<i32>>) {
            if start == nums.len() {
                result.push(nums.clone());
                return;
            }
            for i in start..nums.len() {
                nums.swap(start, i);
                Self::backtrack(nums, start + 1, result);
                nums.swap(start, i); // undo swap
            }
        }
    }
}

/// AI-optimized solution — same swap-based backtracking.
///
/// For n <= 6 (max 720 permutations), both approaches are identical
/// and optimal.
///
/// Time: O(n * n!).
/// Space: O(n).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Generate all permutations of nums.
        pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
            let mut result = Vec::new();
            Self::backtrack(&mut nums, 0, &mut result);
            result
        }

        fn backtrack(nums: &mut Vec<i32>, start: usize, result: &mut Vec<Vec<i32>>) {
            if start == nums.len() {
                result.push(nums.clone());
                return;
            }
            for i in start..nums.len() {
                nums.swap(start, i);
                Self::backtrack(nums, start + 1, result);
                nums.swap(start, i);
            }
        }
    }
}

fn main() {
    let my = my_solution::Solution::permute(vec![1, 2, 3]);
    let ai = ai_solution::Solution::permute(vec![1, 2, 3]);
    println!("my:{my:?}");
    println!("ai:{ai:?}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_permute {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                fn sorted(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
                    v.sort();
                    v
                }

                #[test]
                fn test_three() {
                    let result = sorted(Solution::permute(vec![1, 2, 3]));
                    let expected = sorted(vec![
                        vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3],
                        vec![2, 3, 1], vec![3, 1, 2], vec![3, 2, 1],
                    ]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_two() {
                    let result = sorted(Solution::permute(vec![0, 1]));
                    assert_eq!(result, sorted(vec![vec![0, 1], vec![1, 0]]));
                }

                #[test]
                fn test_single() {
                    assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
                }

                #[test]
                fn test_count_six() {
                    // 6! = 720
                    let result = Solution::permute(vec![1, 2, 3, 4, 5, 6]);
                    assert_eq!(result.len(), 720);
                }
            }
        };
    }

    test_permute!(my_tests, crate::my_solution::Solution);
    test_permute!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        let nums = vec![1, 2, 3, 4, 5, 6];

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::permute(
                std::hint::black_box(nums.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::permute(
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
