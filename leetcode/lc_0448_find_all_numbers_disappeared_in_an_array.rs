// LeetCode 448: Find All Numbers Disappeared in an Array [Easy]
// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/
//
// Given an array nums of n integers where nums[i] is in [1, n], return all
// integers in [1, n] that do not appear in nums.
// Follow up: O(n) time, O(1) extra space (output excluded).
//
// Examples:
//   [4,3,2,7,8,2,3,1] -> [5,6]
//   [1,1] -> [2]
//
// Constraints:
//   n == nums.length
//   1 <= n <= 10^5
//   1 <= nums[i] <= n
//
// Run:  cargo run --example lc_0448_find_all_numbers_disappeared_in_an_array
// Test: cargo test --example lc_0448_find_all_numbers_disappeared_in_an_array
// Bench: cargo test --example lc_0448_find_all_numbers_disappeared_in_an_array bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 99ns (index swap)
//   AI solution: avg 96ns (negation marking)
//   Ratio: 1.03x (similar)

/// My solution module — boolean seen array.
///
/// Mark each value as seen, then collect all unmarked indices.
///
/// Time:  O(n)
/// Space: O(n) -- boolean array

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find all missing numbers in [1, n].
        pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
            // Index-swap approach: place each number at its correct index
            let n = nums.len();
            for i in 0..n {
                while nums[i] != (i as i32 + 1) {
                    let target = (nums[i] - 1) as usize;
                    if nums[target] == nums[i] {
                        break;
                    }
                    nums.swap(i, target);
                }
            }
            // Numbers not at their correct position are missing
            (0..n)
                .filter(|&i| nums[i] != i as i32 + 1)
                .map(|i| i as i32 + 1)
                .collect()
        }
    }
}

/// AI solution module — in-place negation (O(1) extra space).
///
/// For each value v, negate nums[|v|-1] if positive. Then collect indices
/// where nums[i] is still positive => i+1 is missing.
///
/// Time:  O(n)
/// Space: O(1) auxiliary
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find all missing numbers in [1, n].
        pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
            let n = nums.len();
            for i in 0..n {
                // Map value to index: |nums[i]| - 1
                let idx = (nums[i].abs() - 1) as usize;
                if nums[idx] > 0 {
                    nums[idx] = -nums[idx];
                }
            }
            // Indices still positive => their 1-based number is missing
            (0..n)
                .filter(|&i| nums[i] > 0)
                .map(|i| i as i32 + 1)
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
            let mut result = Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]);
            result.sort();
            assert_eq!(result, vec![5, 6]);
        }

        #[test]
        fn test_example2() {
            assert_eq!(Solution::find_disappeared_numbers(vec![1, 1]), vec![2]);
        }

        #[test]
        fn test_no_missing() {
            assert_eq!(
                Solution::find_disappeared_numbers(vec![1, 2, 3]),
                Vec::<i32>::new()
            );
        }

        #[test]
        fn test_all_same() {
            let mut result = Solution::find_disappeared_numbers(vec![1, 1, 1]);
            result.sort();
            assert_eq!(result, vec![2, 3]);
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
    bench_solution(
        "My solution",
        my_solution::Solution::find_disappeared_numbers,
    );
    bench_solution(
        "AI solution",
        ai_solution::Solution::find_disappeared_numbers,
    );
}

fn main() {
    let result = ai_solution::Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]);
    println!("Disappeared: {:?}", result);
}
