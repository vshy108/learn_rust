// LeetCode 485: Max Consecutive Ones [Easy]
// https://leetcode.com/problems/max-consecutive-ones/
//
// Given a binary array nums, return the maximum number of consecutive 1's.
//
// Examples:
//   [1,1,0,1,1,1] -> 3
//   [1,0,1,1,0,1] -> 2
//
// Constraints:
//   1 <= nums.len() <= 10^5
//   nums[i] is 0 or 1
//
// Run:  cargo run --example lc_0485_max_consecutive_ones
// Test: cargo test --example lc_0485_max_consecutive_ones
// Bench: cargo test --example lc_0485_max_consecutive_ones bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 32ns
//   AI solution: avg 32ns
//   Ratio: 1.00x (same algorithm)

/// My solution module — single pass.
///
/// Time:  O(n)
/// Space: O(1)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
            let (mut max_count, mut count) = (0i32, 0i32);
            for &num in &nums {
                if num == 1 {
                    count += 1;
                    max_count = max_count.max(count);
                } else {
                    count = 0;
                }
            }
            max_count
        }
    }
}

/// AI solution module — track current streak and max streak.
///
/// Time:  O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
            let mut max_count = 0i32;
            let mut count = 0i32;
            for &num in &nums {
                if num == 1 {
                    count += 1;
                    max_count = max_count.max(count);
                } else {
                    count = 0;
                }
            }
            max_count
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_example1() {
            assert_eq!(
                Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
                3
            );
        }
        #[test]
        fn test_example2() {
            assert_eq!(
                Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
                2
            );
        }
        #[test]
        fn test_all_ones() {
            assert_eq!(Solution::find_max_consecutive_ones(vec![1, 1, 1]), 3);
        }
        #[test]
        fn test_no_ones() {
            assert_eq!(Solution::find_max_consecutive_ones(vec![0, 0, 0]), 0);
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
    let input = vec![1, 1, 0, 1, 1, 1, 0, 0, 1, 1];
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
    bench_solution(
        "My solution",
        my_solution::Solution::find_max_consecutive_ones,
    );
    bench_solution(
        "AI solution",
        ai_solution::Solution::find_max_consecutive_ones,
    );
}

fn main() {
    println!(
        "{}",
        ai_solution::Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1])
    );
}
