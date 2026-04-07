// LeetCode 78: Subsets [Medium]
// https://leetcode.com/problems/subsets/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 27.888us
//   AI solution avg: 48.519us
//   Ratio: My 1.74x faster
//
// Given an integer array nums of unique elements, return all possible subsets
// (the power set). The solution set must not contain duplicate subsets.
//
// Constraints: 1 <= nums.length <= 10, -10 <= nums[i] <= 10, all unique
//
// Run:  cargo run --example lc_0078_subsets
// Test: cargo test --example lc_0078_subsets
// Bench: cargo test --example lc_0078_subsets bench -- --nocapture

/// My solution — backtracking.
///
/// Build subsets by choosing to include or skip each element.
///
/// Time: O(n * 2^n) — 2^n subsets, each up to n elements to copy.
/// Space: O(n) recursion depth.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Generate all subsets of nums.
        ///
        /// # Arguments
        /// * `nums` - array of unique integers
        ///
        /// # Returns
        /// Power set
        pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
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
                current.push(nums[i]);
                Self::backtrack(nums, i + 1, current, result);
                current.pop();
            }
        }
    }
}

/// AI solution — iterative bitmask enumeration.
///
/// Enumerate all 2^n bitmasks; each bit indicates element inclusion.
///
/// Time: O(n * 2^n).
/// Space: O(1) extra beyond output.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Generate all subsets of nums using bit manipulation.
        ///
        /// # Arguments
        /// * `nums` - array of unique integers
        ///
        /// # Returns
        /// Power set
        pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
            let n = nums.len();
            let total = 1usize << n; // 2^n
            let mut result = Vec::with_capacity(total);

            for mask in 0..total {
                let mut subset = Vec::new();
                for i in 0..n {
                    if mask & (1 << i) != 0 {
                        subset.push(nums[i]);
                    }
                }
                result.push(subset);
            }
            result
        }
    }
}

fn main() {
    let nums = vec![1, 2, 3];
    let my = my_solution::Solution::subsets(nums.clone());
    let ai = ai_solution::Solution::subsets(nums);
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

    macro_rules! test_subsets {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::sorted;

                #[test]
                fn test_example1() {
                    let result = sorted(<$solution>::subsets(vec![1, 2, 3]));
                    let expected = sorted(vec![
                        vec![], vec![1], vec![2], vec![3],
                        vec![1, 2], vec![1, 3], vec![2, 3],
                        vec![1, 2, 3],
                    ]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_single() {
                    let result = sorted(<$solution>::subsets(vec![0]));
                    let expected = sorted(vec![vec![], vec![0]]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_count() {
                    // 2^4 = 16 subsets
                    let result = <$solution>::subsets(vec![1, 2, 3, 4]);
                    assert_eq!(result.len(), 16);
                }
            }
        };
    }

    test_subsets!(my, crate::my_solution::Solution);
    test_subsets!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8]; // 2^8 = 256 subsets

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::subsets(
                std::hint::black_box(nums.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::subsets(
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
