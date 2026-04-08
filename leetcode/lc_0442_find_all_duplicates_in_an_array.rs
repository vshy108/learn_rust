// LeetCode 442: Find All Duplicates in an Array [Medium]
// https://leetcode.com/problems/find-all-duplicates-in-an-array/
//
// Given an integer array nums of length n where all integers are in [1, n]
// and each integer appears at most twice, return all integers that appear twice.
// Must run in O(n) time with O(1) auxiliary space.
//
// Examples:
//   [4,3,2,7,8,2,3,1] -> [2,3]
//   [1,1,2] -> [1]
//   [1] -> []
//
// Constraints:
//   n == nums.length
//   1 <= n <= 10^5
//   1 <= nums[i] <= n
//   Each element appears once or twice
//
// Run:  cargo run --example lc_0442_find_all_duplicates_in_an_array
// Test: cargo test --example lc_0442_find_all_duplicates_in_an_array
// Bench: cargo test --example lc_0442_find_all_duplicates_in_an_array bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 86ns  (negation marking)
//   AI solution: avg 96ns  (cyclic sort)
//   Ratio: 0.90x (negation slightly faster)

/// My solution module — in-place negation marking.
///
/// For each value v, use index |v|-1 as a marker. If nums[|v|-1] is already
/// negative, v is a duplicate. Otherwise negate nums[|v|-1].
///
/// Time:  O(n)
/// Space: O(1) auxiliary

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find all integers appearing twice.
        pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
            // Negation marking: flip sign at index |val|-1
            let mut result = Vec::new();
            for i in 0..nums.len() {
                let idx = (nums[i].abs() - 1) as usize;
                if nums[idx] < 0 {
                    result.push(nums[i].abs());
                } else {
                    nums[idx] = -nums[idx];
                }
            }
            result
        }
    }
}

/// AI solution module — cyclic sort.
///
/// Place each number at its "correct" index (nums[i] at index nums[i]-1).
/// After sorting, any position where nums[i] != i+1 means nums[i] is a dup.
///
/// Time:  O(n) -- each element moved at most once
/// Space: O(1) auxiliary
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find all integers appearing twice.
        pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
            let n = nums.len();
            for i in 0..n {
                // Swap nums[i] to its correct position nums[i]-1
                // until it's in place or a duplicate blocks it
                while nums[i] != nums[nums[i] as usize - 1] {
                    let j = nums[i] as usize - 1;
                    nums.swap(i, j);
                }
            }
            // Any position where nums[i] != i+1 holds a duplicate
            (0..n)
                .filter(|&i| nums[i] != (i as i32 + 1))
                .map(|i| nums[i])
                .collect()
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;

        #[test]
        fn test_example1() {
            let mut result = Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]);
            result.sort();
            assert_eq!(result, vec![2, 3]);
        }

        #[test]
        fn test_example2() {
            assert_eq!(Solution::find_duplicates(vec![1, 1, 2]), vec![1]);
        }

        #[test]
        fn test_no_duplicates() {
            assert_eq!(Solution::find_duplicates(vec![1]), Vec::<i32>::new());
        }

        #[test]
        fn test_all_duplicates() {
            let mut result = Solution::find_duplicates(vec![2, 2, 1, 1]);
            result.sort();
            assert_eq!(result, vec![1, 2]);
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
    F: Fn(Vec<i32>) -> Vec<i32>,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(vec![4, 3, 2, 7, 8, 2, 3, 1])));
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
    bench_solution("My solution", my_solution::Solution::find_duplicates);
    bench_solution("AI solution", ai_solution::Solution::find_duplicates);
}

fn main() {
    let result = ai_solution::Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]);
    println!("Duplicates: {:?}", result);
}
