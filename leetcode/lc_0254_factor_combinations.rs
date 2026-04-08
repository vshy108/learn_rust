// LeetCode 254: Factor Combinations [Medium] (Premium)
// https://leetcode.com/problems/factor-combinations/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 3.165us
//   AI solution avg: 5.015us
//   Ratio: My 1.58x faster
//
// Numbers can be regarded as the product of their factors.
// Given an integer n, return all possible combinations of its factors.
// Factors should be in the range [2, n-1].
//
// Example 1: n = 1  -> []
// Example 2: n = 12 -> [[2,6],[3,4],[2,2,3]]
// Example 3: n = 37 -> []
//
// Constraints:
//   - 1 <= n <= 10^7
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0254.Factor%20Combinations/README_EN.md
//
// Run:  cargo run --example lc_0254_factor_combinations
// Test: cargo test --example lc_0254_factor_combinations
// Bench: cargo test --example lc_0254_factor_combinations bench -- --nocapture

/// My solution — backtracking DFS (from doocs Python).
///
/// Start factor j from `i` (avoids duplicates). For each j where j*j <= n
/// and n % j == 0, recurse with n/j. When partial list `t` is non-empty,
/// snapshot t + [remaining n] as a valid combination.
///
/// Time: O(sqrt(n)^k) where k is max factorization depth
/// Space: O(k) recursion depth + O(answer size)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return all factor combinations of `n`.
        pub fn get_factors(n: i32) -> Vec<Vec<i32>> {
            let mut ans: Vec<Vec<i32>> = Vec::new();
            let mut t: Vec<i32> = Vec::new();
            Self::dfs(n, 2, &mut t, &mut ans);
            ans
        }

        /// Recursive helper: find factors of `n` starting from `i`.
        fn dfs(n: i32, i: i32, t: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            // If we already have at least one factor, t + [n] is a valid combo
            if !t.is_empty() {
                let mut combo = t.clone();
                combo.push(n);
                ans.push(combo);
            }
            // Try each factor j from i up to sqrt(n)
            let mut j = i;
            while j * j <= n {
                if n % j == 0 {
                    t.push(j);
                    Self::dfs(n / j, j, t, ans);
                    t.pop();
                }
                j += 1;
            }
        }
    }
}

/// AI solution — iterative DFS with explicit stack.
///
/// Uses an explicit stack to simulate the recursive backtracking,
/// avoiding recursion overhead. Each stack frame stores (remaining n,
/// min_factor, current partial list).
///
/// Time: O(sqrt(n)^k) where k is max factorization depth
/// Space: O(k * answer_count)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return all factor combinations of `n`.
        pub fn get_factors(n: i32) -> Vec<Vec<i32>> {
            let mut ans: Vec<Vec<i32>> = Vec::new();
            // Stack: (remaining value, min factor to try, partial factors)
            let mut stack: Vec<(i32, i32, Vec<i32>)> = vec![(n, 2, Vec::new())];

            while let Some((rem, min_f, partial)) = stack.pop() {
                // Record combination if partial is non-empty
                if !partial.is_empty() {
                    let mut combo = partial.clone();
                    combo.push(rem);
                    ans.push(combo);
                }
                // Try factors from min_f up to sqrt(rem)
                let mut j = min_f;
                while j * j <= rem {
                    if rem % j == 0 {
                        let mut next = partial.clone();
                        next.push(j);
                        stack.push((rem / j, j, next));
                    }
                    j += 1;
                }
            }
            ans
        }
    }
}

fn main() {
    println!("my n=12: {:?}", my_solution::Solution::get_factors(12));
    println!("ai n=12: {:?}", ai_solution::Solution::get_factors(12));
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    /// Sort each inner vec and collect into a HashSet for order-independent comparison.
    fn normalize(mut v: Vec<Vec<i32>>) -> HashSet<Vec<i32>> {
        for inner in &mut v {
            inner.sort_unstable();
        }
        v.into_iter().collect()
    }

    macro_rules! test_factor_combinations {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::normalize;
                use $solution as Sol;

                #[test]
                fn test_n_1() {
                    assert!(Sol::get_factors(1).is_empty());
                }

                #[test]
                fn test_n_12() {
                    let result = normalize(Sol::get_factors(12));
                    let expected = normalize(vec![vec![2, 6], vec![3, 4], vec![2, 2, 3]]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_n_37_prime() {
                    assert!(Sol::get_factors(37).is_empty());
                }

                #[test]
                fn test_n_2() {
                    assert!(Sol::get_factors(2).is_empty());
                }

                #[test]
                fn test_n_4() {
                    let result = normalize(Sol::get_factors(4));
                    let expected = normalize(vec![vec![2, 2]]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_n_8() {
                    let result = normalize(Sol::get_factors(8));
                    let expected = normalize(vec![vec![2, 4], vec![2, 2, 2]]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_n_32() {
                    let result = normalize(Sol::get_factors(32));
                    let expected = normalize(vec![
                        vec![2, 16],
                        vec![4, 8],
                        vec![2, 2, 8],
                        vec![2, 4, 4],
                        vec![2, 2, 2, 4],
                        vec![2, 2, 2, 2, 2],
                    ]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_n_16() {
                    let result = normalize(Sol::get_factors(16));
                    let expected = normalize(vec![
                        vec![2, 8],
                        vec![4, 4],
                        vec![2, 2, 4],
                        vec![2, 2, 2, 2],
                    ]);
                    assert_eq!(result, expected);
                }
            }
        };
    }

    test_factor_combinations!(my, crate::my_solution::Solution);
    test_factor_combinations!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let n = 1000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::get_factors(n));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::get_factors(n));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else if ai_avg < my_avg {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        } else {
            println!("  Ratio: ~tied");
        }
    }
}
