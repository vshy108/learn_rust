// LeetCode 90: Subsets II [Medium]
// https://leetcode.com/problems/subsets-ii/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 11.557us
//   AI solution avg: 16.058us
//   Ratio: My 1.39x faster
//
// Given an integer array nums that may contain duplicates, return all possible
// subsets (the power set). The solution must not contain duplicate subsets.
//
// Constraints: 1 <= nums.length <= 10, -10 <= nums[i] <= 10
//
// Run:  cargo run --example lc_0090_subsets_ii
// Test: cargo test --example lc_0090_subsets_ii
// Bench: cargo test --example lc_0090_subsets_ii bench -- --nocapture

/// My solution — sort + backtracking, skip duplicates.
///
/// Time: O(n * 2^n).
/// Space: O(n) recursion depth.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Generate all unique subsets.
        ///
        /// # Arguments
        /// * `nums` - array that may contain duplicates
        ///
        /// # Returns
        /// All unique subsets
        pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
            nums.sort();
            let mut result = Vec::new();
            let mut current = Vec::new();
            Self::backtrack(&nums, 0, &mut current, &mut result);
            result
        }

        fn backtrack(
            nums: &[i32],
            start: usize,
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            result.push(current.clone());
            for i in start..nums.len() {
                // Skip duplicates at the same recursion level
                if i > start && nums[i] == nums[i - 1] {
                    continue;
                }
                current.push(nums[i]);
                Self::backtrack(nums, i + 1, current, result);
                current.pop();
            }
        }
    }
}

/// AI solution — iterative, track where to start adding for duplicates.
///
/// Time: O(n * 2^n).
/// Space: O(2^n * n) for output.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Generate all unique subsets.
        ///
        /// # Arguments
        /// * `nums` - array that may contain duplicates
        ///
        /// # Returns
        /// All unique subsets
        pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
            nums.sort();
            let mut result: Vec<Vec<i32>> = vec![vec![]];
            let mut prev_size = 0;

            for i in 0..nums.len() {
                let start = if i > 0 && nums[i] == nums[i - 1] {
                    prev_size // Only extend subsets added in the previous round
                } else {
                    0
                };
                prev_size = result.len();
                let mut new_subsets = Vec::new();
                for j in start..prev_size {
                    let mut subset = result[j].clone();
                    subset.push(nums[i]);
                    new_subsets.push(subset);
                }
                result.extend(new_subsets);
            }
            result
        }
    }
}

fn main() {
    let my = my_solution::Solution::subsets_with_dup(vec![1, 2, 2]);
    let ai = ai_solution::Solution::subsets_with_dup(vec![1, 2, 2]);
    println!("my: {my:?}");
    println!("ai: {ai:?}");
}

#[cfg(test)]
mod tests {
    fn sorted(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for row in &mut v {
            row.sort();
        }
        v.sort();
        v
    }

    macro_rules! test_subsets_dup {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::sorted;

                #[test]
                fn test_example1() {
                    let result = sorted(<$solution>::subsets_with_dup(vec![1, 2, 2]));
                    let expected = sorted(vec![
                        vec![],
                        vec![1],
                        vec![2],
                        vec![1, 2],
                        vec![2, 2],
                        vec![1, 2, 2],
                    ]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_single() {
                    let result = sorted(<$solution>::subsets_with_dup(vec![0]));
                    let expected = sorted(vec![vec![], vec![0]]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_all_same() {
                    let result = sorted(<$solution>::subsets_with_dup(vec![1, 1, 1]));
                    let expected = sorted(vec![vec![], vec![1], vec![1, 1], vec![1, 1, 1]]);
                    assert_eq!(result, expected);
                }
            }
        };
    }

    test_subsets_dup!(my, crate::my_solution::Solution);
    test_subsets_dup!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums = vec![1, 2, 2, 3, 3, 3, 4, 5];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::subsets_with_dup(
                std::hint::black_box(nums.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::subsets_with_dup(
                std::hint::black_box(nums.clone()),
            ));
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
