// LeetCode 491: Non-decreasing Subsequences [Medium]
// https://leetcode.com/problems/non-decreasing-subsequences/
//
// Given an integer array nums, return all the different possible
// non-decreasing subsequences of the given array with at least two elements.
//
// Examples:
//   [4,6,7,7]    -> [[4,6],[4,6,7],[4,6,7,7],[4,7],[4,7,7],[6,7],[6,7,7],[7,7]]
//   [4,4,3,2,1]  -> [[4,4]]
//
// Constraints:
//   1 <= nums.len() <= 15
//   -100 <= nums[i] <= 100
//
// Run:  cargo run --example lc_0491_non_decreasing_subsequences
// Test: cargo test --example lc_0491_non_decreasing_subsequences
// Bench: cargo test --example lc_0491_non_decreasing_subsequences bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 1.196us
//   AI solution: avg 1.203us
//   Ratio: 0.99x (same algorithm)

/// My solution module — backtracking / DFS.
///
/// Time:  O(2^n * n) worst case
/// Space: O(n * 2^n)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
            use std::collections::HashSet;
            let mut result = Vec::new();
            let mut path = Vec::new();
            fn dfs(nums: &[i32], start: usize, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
                if path.len() >= 2 {
                    result.push(path.clone());
                }
                let mut used = HashSet::new();
                for i in start..nums.len() {
                    if !path.is_empty() && nums[i] < *path.last().unwrap() {
                        continue;
                    }
                    if !used.insert(nums[i]) {
                        continue;
                    }
                    path.push(nums[i]);
                    dfs(nums, i + 1, path, result);
                    path.pop();
                }
            }
            dfs(&nums, 0, &mut path, &mut result);
            result
        }
    }
}

/// AI solution module — DFS backtracking with HashSet dedup at each level.
///
/// At each position, decide to include or skip. Use a set at each recursion
/// level to avoid duplicate picks (since we can't sort to skip duplicates —
/// we must preserve order for subsequences).
///
/// Time:  O(2^n * n)
/// Space: O(n * 2^n)
mod ai_solution {
    use std::collections::HashSet;

    pub struct Solution;
    impl Solution {
        pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
            let mut result = Vec::new();
            let mut path = Vec::new();
            Self::dfs(&nums, 0, &mut path, &mut result);
            result
        }

        fn dfs(nums: &[i32], start: usize, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if path.len() >= 2 {
                result.push(path.clone());
            }
            // Track used values at this recursion level
            let mut used = HashSet::new();
            for i in start..nums.len() {
                // Must be non-decreasing
                if !path.is_empty() && nums[i] < *path.last().expect("non-empty path") {
                    continue;
                }
                // Skip duplicate values at same level
                if !used.insert(nums[i]) {
                    continue;
                }

                path.push(nums[i]);
                Self::dfs(nums, i + 1, path, result);
                path.pop();
            }
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_example1() {
            let mut result = Solution::find_subsequences(vec![4, 6, 7, 7]);
            result.sort();
            let mut expected = vec![
                vec![4, 6],
                vec![4, 6, 7],
                vec![4, 6, 7, 7],
                vec![4, 7],
                vec![4, 7, 7],
                vec![6, 7],
                vec![6, 7, 7],
                vec![7, 7],
            ];
            expected.sort();
            assert_eq!(result, expected);
        }
        #[test]
        fn test_example2() {
            assert_eq!(
                Solution::find_subsequences(vec![4, 4, 3, 2, 1]),
                vec![vec![4, 4]]
            );
        }
    };
}

#[cfg(test)]
mod tests_my {
    solution_tests!(my_solution);
}
#[cfg(test)]
mod tests_ai {
    solution_tests!(ai_solution);
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(Vec<i32>) -> Vec<Vec<i32>>,
{
    let iterations = 500_000;
    let input = vec![4, 6, 7, 7];
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(input.clone())));
    }
    let elapsed = start.elapsed();
    println!(
        "{}: {} iterations in {:?} (avg {:?})",
        name,
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::find_subsequences);
    bench_solution("AI solution", ai_solution::Solution::find_subsequences);
}

fn main() {
    println!(
        "{:?}",
        ai_solution::Solution::find_subsequences(vec![4, 6, 7, 7])
    );
}
