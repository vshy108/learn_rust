// LeetCode 474: Ones and Zeroes [Medium]
// https://leetcode.com/problems/ones-and-zeroes/
//
// You are given an array of binary strings strs and two integers m and n.
// Return the size of the largest subset of strs such that there are at most
// m 0's and n 1's in the subset.
//
// This is a 2D knapsack problem where the "weights" are (count_0, count_1).
//
// Examples:
//   strs = ["10","0001","111001","1","0"], m = 5, n = 3 -> 4
//   strs = ["10","0","1"], m = 1, n = 1 -> 2
//
// Constraints:
//   1 <= strs.len() <= 600
//   1 <= strs[i].len() <= 100
//   strs[i] consists only of '0' and '1'
//   1 <= m, n <= 100
//
// Run:  cargo run --example lc_0474_ones_and_zeroes
// Test: cargo test --example lc_0474_ones_and_zeroes
// Bench: cargo test --example lc_0474_ones_and_zeroes bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 736ns
//   AI solution: avg 739ns
//   Ratio: 1.00x (same algorithm)

/// My solution module — 2D knapsack DP.
///
/// Time:  O(l * m * n) where l = strs.len()
/// Space: O(m * n)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
            let (m, n) = (m as usize, n as usize);
            let mut dp = vec![vec![0i32; n + 1]; m + 1];
            for s in &strs {
                let zeros = s.bytes().filter(|&b| b == b'0').count();
                let ones = s.len() - zeros;
                for i in (zeros..=m).rev() {
                    for j in (ones..=n).rev() {
                        dp[i][j] = dp[i][j].max(dp[i - zeros][j - ones] + 1);
                    }
                }
            }
            dp[m][n]
        }
    }
}

/// AI solution module — 2D 0/1 knapsack DP.
///
/// dp[i][j] = max subset size using at most i zeros and j ones.
/// Iterate strings in reverse to avoid using a string twice.
///
/// Time:  O(l * m * n)
/// Space: O(m * n)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
            let m = m as usize;
            let n = n as usize;
            let mut dp = vec![vec![0i32; n + 1]; m + 1];

            for s in &strs {
                let zeros = s.bytes().filter(|&b| b == b'0').count();
                let ones = s.len() - zeros;
                // Iterate in reverse so each string is used at most once
                for i in (zeros..=m).rev() {
                    for j in (ones..=n).rev() {
                        dp[i][j] = dp[i][j].max(dp[i - zeros][j - ones] + 1);
                    }
                }
            }

            dp[m][n]
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_example1() {
            let strs: Vec<String> = vec!["10", "0001", "111001", "1", "0"]
                .into_iter()
                .map(String::from)
                .collect();
            assert_eq!(Solution::find_max_form(strs, 5, 3), 4);
        }
        #[test]
        fn test_example2() {
            let strs: Vec<String> = vec!["10", "0", "1"].into_iter().map(String::from).collect();
            assert_eq!(Solution::find_max_form(strs, 1, 1), 2);
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
    F: Fn(Vec<String>, i32, i32) -> i32,
{
    let iterations = 500_000;
    let strs: Vec<String> = vec!["10", "0001", "111001", "1", "0"]
        .into_iter()
        .map(String::from)
        .collect();
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(
            std::hint::black_box(strs.clone()),
            std::hint::black_box(5),
            std::hint::black_box(3),
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
    bench_solution("My solution", my_solution::Solution::find_max_form);
    bench_solution("AI solution", ai_solution::Solution::find_max_form);
}

fn main() {
    let strs: Vec<String> = vec!["10", "0001", "111001", "1", "0"]
        .into_iter()
        .map(String::from)
        .collect();
    println!("{}", ai_solution::Solution::find_max_form(strs, 5, 3));
}
