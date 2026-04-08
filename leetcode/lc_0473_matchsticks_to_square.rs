// LeetCode 473: Matchsticks to Square [Medium]
// https://leetcode.com/problems/matchsticks-to-square/
//
// You are given an integer array matchsticks where matchsticks[i] is the
// length of the ith matchstick. Return true if you can use all matchsticks
// to make one square (each side of the square must use some matchsticks and
// every matchstick must be used exactly once).
//
// Examples:
//   [1,1,2,2,2] -> true  (sides: [1,2], [1,2], [2], [2]... wait, actually 1+1+2+2+2=8, side=2, [2],[2],[1,1],[2])
//   [3,3,3,3,4] -> false
//
// Constraints:
//   1 <= matchsticks.len() <= 15
//   1 <= matchsticks[i] <= 10^8
//
// Run:  cargo run --example lc_0473_matchsticks_to_square
// Test: cargo test --example lc_0473_matchsticks_to_square
// Bench: cargo test --example lc_0473_matchsticks_to_square bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 351ns
//   AI solution: avg 349ns
//   Ratio: 1.01x (same algorithm)

/// My solution module — backtracking with pruning.
///
/// Time:  O(4^n) worst case, pruned heavily
/// Space: O(n) — recursion depth

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
            let total: i32 = matchsticks.iter().sum();
            if total % 4 != 0 {
                return false;
            }
            let side = total / 4;
            matchsticks.sort_unstable_by(|a, b| b.cmp(a));
            if matchsticks[0] > side {
                return false;
            }
            let mut sides = [0i32; 4];
            fn bt(sticks: &[i32], sides: &mut [i32; 4], idx: usize, t: i32) -> bool {
                if idx == sticks.len() {
                    return sides.iter().all(|&s| s == t);
                }
                let mut tried = std::collections::HashSet::new();
                for i in 0..4 {
                    if sides[i] + sticks[idx] > t {
                        continue;
                    }
                    if !tried.insert(sides[i]) {
                        continue;
                    }
                    sides[i] += sticks[idx];
                    if bt(sticks, sides, idx + 1, t) {
                        return true;
                    }
                    sides[i] -= sticks[idx];
                }
                false
            }
            bt(&matchsticks, &mut sides, 0, side)
        }
    }
}

/// AI solution module — backtracking: try placing each matchstick into one
/// of 4 sides. Sort descending for early pruning.
///
/// Time:  O(4^n) worst case
/// Space: O(n)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
            let total: i32 = matchsticks.iter().sum();
            if total % 4 != 0 {
                return false;
            }
            let side = total / 4;
            // Sort descending so larger sticks fail faster (pruning)
            matchsticks.sort_unstable_by(|a, b| b.cmp(a));
            if matchsticks[0] > side {
                return false;
            }
            let mut sides = [0i32; 4];
            Self::backtrack(&matchsticks, &mut sides, 0, side)
        }

        fn backtrack(sticks: &[i32], sides: &mut [i32; 4], idx: usize, target: i32) -> bool {
            if idx == sticks.len() {
                // All sticks placed; check all sides equal target
                return sides.iter().all(|&s| s == target);
            }
            let stick = sticks[idx];
            // Track which side lengths we've already tried (skip duplicate states)
            let mut tried = std::collections::HashSet::new();
            for i in 0..4 {
                if sides[i] + stick > target {
                    continue;
                }
                if !tried.insert(sides[i]) {
                    continue;
                } // skip duplicate side values
                sides[i] += stick;
                if Self::backtrack(sticks, sides, idx + 1, target) {
                    return true;
                }
                sides[i] -= stick;
            }
            false
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_true() {
            assert!(Solution::makesquare(vec![1, 1, 2, 2, 2]));
        }
        #[test]
        fn test_false() {
            assert!(!Solution::makesquare(vec![3, 3, 3, 3, 4]));
        }
        #[test]
        fn test_single() {
            assert!(!Solution::makesquare(vec![5]));
        }
        #[test]
        fn test_four_equal() {
            assert!(Solution::makesquare(vec![4, 4, 4, 4]));
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
    F: Fn(Vec<i32>) -> bool,
{
    let iterations = 500_000;
    let input = vec![1, 1, 2, 2, 2];
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
    bench_solution("My solution", my_solution::Solution::makesquare);
    bench_solution("AI solution", ai_solution::Solution::makesquare);
}

fn main() {
    println!("{}", ai_solution::Solution::makesquare(vec![1, 1, 2, 2, 2]));
}
