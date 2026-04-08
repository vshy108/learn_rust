// LeetCode 462: Minimum Moves to Equal Array Elements II [Medium]
// https://leetcode.com/problems/minimum-moves-to-equal-array-elements-ii/
//
// Given an integer array nums of size n, return the minimum number of moves
// required to make all array elements equal.
// In one move, you can increment or decrement an element by 1.
//
// The optimal target is the median of the array.
//
// Examples:
//   [1,2,3]   -> 2  (move to 2)
//   [1,10,2,9] -> 16 (move to median ~2 or ~9, both equal 16)
//
// Constraints:
//   n == nums.len(), 1 <= n <= 10^5
//   -10^9 <= nums[i] <= 10^9
//
// Run:  cargo run --example lc_0462_minimum_moves_to_equal_array_elements_ii
// Test: cargo test --example lc_0462_minimum_moves_to_equal_array_elements_ii
// Bench: cargo test --example lc_0462_minimum_moves_to_equal_array_elements_ii bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 45ns
//   AI solution: avg 44ns
//   Ratio: 1.02x (same algorithm)

/// My solution module — sort + median.
///
/// Time:  O(n log n) — sorting
/// Space: O(1) — in-place sort (ignoring sort stack)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
            // Sort and sum distances from median
            nums.sort_unstable();
            let median = nums[nums.len() / 2];
            nums.iter().map(|&x| (x - median).abs()).sum()
        }
    }
}

/// AI solution module — sort + median.
///
/// Time:  O(n log n) — sorting
/// Space: O(1)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
            nums.sort_unstable();
            let median = nums[nums.len() / 2];
            // Sum of absolute differences from the median
            nums.iter().map(|&x| (x - median).abs()).sum()
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_basic() {
            assert_eq!(Solution::min_moves2(vec![1, 2, 3]), 2);
        }
        #[test]
        fn test_even() {
            assert_eq!(Solution::min_moves2(vec![1, 10, 2, 9]), 16);
        }
        #[test]
        fn test_single() {
            assert_eq!(Solution::min_moves2(vec![5]), 0);
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
    F: Fn(Vec<i32>) -> i32,
{
    let iterations = 500_000;
    let input = vec![1, 10, 2, 9, 3, 8, 4, 7, 5, 6];
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
    bench_solution("My solution", my_solution::Solution::min_moves2);
    bench_solution("AI solution", ai_solution::Solution::min_moves2);
}

fn main() {
    println!("{}", ai_solution::Solution::min_moves2(vec![1, 2, 3]));
}
