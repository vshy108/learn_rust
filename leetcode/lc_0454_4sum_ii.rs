// LeetCode 454: 4Sum II [Medium]
// https://leetcode.com/problems/4sum-ii/
//
// Given four integer arrays nums1, nums2, nums3, nums4 all of length n,
// return the number of tuples (i, j, k, l) such that
// nums1[i] + nums2[j] + nums3[k] + nums4[l] == 0.
//
// Examples:
//   nums1=[1,2], nums2=[-2,-1], nums3=[-1,2], nums4=[0,2] -> 2
//   nums1=[0], nums2=[0], nums3=[0], nums4=[0] -> 1
//
// Constraints:
//   n == nums1/2/3/4.length, 1 <= n <= 200
//   -2^28 <= nums[i] <= 2^28
//
// Run:  cargo run --example lc_0454_4sum_ii
// Test: cargo test --example lc_0454_4sum_ii
// Bench: cargo test --example lc_0454_4sum_ii bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 275ns
//   AI solution: avg 275ns
//   Ratio: 1.00x (same algorithm)

/// My solution module — hash map: store sums of first two, look up complement.
///
/// Time:  O(n^2)
/// Space: O(n^2)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn four_sum_count(
            nums1: Vec<i32>,
            nums2: Vec<i32>,
            nums3: Vec<i32>,
            nums4: Vec<i32>,
        ) -> i32 {
            use std::collections::HashMap;
            // Store all (a+b) sums, then look for -(c+d)
            let mut ab_sums: HashMap<i32, i32> = HashMap::new();
            for &a in &nums1 {
                for &b in &nums2 {
                    *ab_sums.entry(a + b).or_insert(0) += 1;
                }
            }
            let mut count = 0;
            for &c in &nums3 {
                for &d in &nums4 {
                    if let Some(&v) = ab_sums.get(&-(c + d)) {
                        count += v;
                    }
                }
            }
            count
        }
    }
}

/// AI solution module — hash map (doocs Rust approach).
///
/// Time:  O(n^2)
/// Space: O(n^2)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn four_sum_count(
            nums1: Vec<i32>,
            nums2: Vec<i32>,
            nums3: Vec<i32>,
            nums4: Vec<i32>,
        ) -> i32 {
            use std::collections::HashMap;
            let mut cnt: HashMap<i32, i32> = HashMap::new();
            for &a in &nums1 {
                for &b in &nums2 {
                    *cnt.entry(a + b).or_insert(0) += 1;
                }
            }
            let mut ans = 0;
            for &c in &nums3 {
                for &d in &nums4 {
                    if let Some(&count) = cnt.get(&(-(c + d))) {
                        ans += count;
                    }
                }
            }
            ans
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
                Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
                2
            );
        }
        #[test]
        fn test_all_zeros() {
            assert_eq!(
                Solution::four_sum_count(vec![0], vec![0], vec![0], vec![0]),
                1
            );
        }
        #[test]
        fn test_no_solution() {
            assert_eq!(
                Solution::four_sum_count(vec![1], vec![1], vec![1], vec![1]),
                0
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
    F: Fn(Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>) -> i32,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(
            std::hint::black_box(vec![1, 2]),
            std::hint::black_box(vec![-2, -1]),
            std::hint::black_box(vec![-1, 2]),
            std::hint::black_box(vec![0, 2]),
        ));
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
    bench_solution("My solution", my_solution::Solution::four_sum_count);
    bench_solution("AI solution", ai_solution::Solution::four_sum_count);
}

fn main() {
    println!(
        "{}",
        ai_solution::Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2])
    );
}
