// LeetCode 453: Minimum Moves to Equal Array Elements [Medium]
// https://leetcode.com/problems/minimum-moves-to-equal-array-elements/
//
// Given an integer array nums, return the minimum number of moves to make all
// elements equal. In one move, you can increment n-1 elements by 1.
// Key insight: incrementing n-1 elements is equivalent to decrementing 1 element.
// So answer = sum - min * n.
//
// Examples:
//   [1,2,3] -> 3
//   [1,1,1] -> 0
//
// Constraints:
//   n == nums.length, 1 <= n <= 10^5
//   -10^9 <= nums[i] <= 10^9
//   Answer fits in a 32-bit integer
//
// Run:  cargo run --example lc_0453_minimum_moves_to_equal_array_elements
// Test: cargo test --example lc_0453_minimum_moves_to_equal_array_elements
// Bench: cargo test --example lc_0453_minimum_moves_to_equal_array_elements bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 22ns
//   AI solution: avg 22ns
//   Ratio: 1.00x (same formula)

/// My solution module — math: sum - min * n.
///
/// Time:  O(n)
/// Space: O(1)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn min_moves(nums: Vec<i32>) -> i32 {
            // Incrementing n-1 elements is equivalent to decrementing 1
            let min_val = *nums.iter().min().expect("non-empty");
            nums.iter().map(|&x| x - min_val).sum()
        }
    }
}

/// AI solution module — sum each element's difference from the minimum.
///
/// Time:  O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn min_moves(nums: Vec<i32>) -> i32 {
            let mi = *nums.iter().min().expect("nums is non-empty");
            nums.iter().map(|&x| x - mi).sum()
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_example1() {
            assert_eq!(Solution::min_moves(vec![1, 2, 3]), 3);
        }
        #[test]
        fn test_all_equal() {
            assert_eq!(Solution::min_moves(vec![1, 1, 1]), 0);
        }
        #[test]
        fn test_two_elements() {
            assert_eq!(Solution::min_moves(vec![1, 100]), 99);
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
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(vec![1, 2, 3])));
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
    bench_solution("My solution", my_solution::Solution::min_moves);
    bench_solution("AI solution", ai_solution::Solution::min_moves);
}

fn main() {
    println!("{}", ai_solution::Solution::min_moves(vec![1, 2, 3]));
}
