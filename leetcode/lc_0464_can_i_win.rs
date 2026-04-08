// LeetCode 464: Can I Win [Medium]
// https://leetcode.com/problems/can-i-win/
//
// Two players take turns picking integers from 1 to maxChoosableInteger.
// Each integer can only be picked once. The player whose running total
// reaches or exceeds desiredTotal wins. Return true if the first player
// can force a win (assuming both play optimally).
//
// Examples:
//   maxChoosableInteger = 10, desiredTotal = 11 -> false
//   maxChoosableInteger = 10, desiredTotal = 0  -> true
//   maxChoosableInteger = 10, desiredTotal = 1  -> true
//
// Constraints:
//   1 <= maxChoosableInteger <= 20
//   0 <= desiredTotal <= 300
//
// Run:  cargo run --example lc_0464_can_i_win
// Test: cargo test --example lc_0464_can_i_win
// Bench: cargo test --example lc_0464_can_i_win bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 1.803us
//   AI solution: avg 1.804us
//   Ratio: 1.00x (same algorithm)

/// My solution module — bitmask DP / game theory.
///
/// Time:  O(2^n * n) where n = maxChoosableInteger
/// Space: O(2^n) — memoization table

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
            use std::collections::HashMap;
            let n = max_choosable_integer;
            if desired_total <= 0 {
                return true;
            }
            if n * (n + 1) / 2 < desired_total {
                return false;
            }
            let mut memo: HashMap<u32, bool> = HashMap::new();
            fn can_win(n: u32, remaining: u32, used: u32, memo: &mut HashMap<u32, bool>) -> bool {
                if let Some(&r) = memo.get(&used) {
                    return r;
                }
                for i in 1..=n {
                    let mask = 1u32 << i;
                    if used & mask != 0 {
                        continue;
                    }
                    if i >= remaining || !can_win(n, remaining - i, used | mask, memo) {
                        memo.insert(used, true);
                        return true;
                    }
                }
                memo.insert(used, false);
                false
            }
            can_win(n as u32, desired_total as u32, 0, &mut memo)
        }
    }
}

/// AI solution module — bitmask DP with memoization.
///
/// Use a bitmask to track which numbers have been chosen. For each state,
/// try picking each unchosen number. If picking it reaches the target or
/// the opponent cannot win from the resulting state, we win.
///
/// Time:  O(2^n * n)
/// Space: O(2^n)
mod ai_solution {
    use std::collections::HashMap;

    pub struct Solution;
    impl Solution {
        pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
            let n = max_choosable_integer;
            // If desired total is 0, first player wins immediately
            if desired_total <= 0 {
                return true;
            }
            // If sum of all choices < desired total, no one can win
            let total_sum = n * (n + 1) / 2;
            if total_sum < desired_total {
                return false;
            }

            let mut memo: HashMap<u32, bool> = HashMap::new();
            Self::can_win(n as u32, desired_total as u32, 0, &mut memo)
        }

        /// Returns true if the current player can force a win.
        fn can_win(n: u32, remaining: u32, used: u32, memo: &mut HashMap<u32, bool>) -> bool {
            if let Some(&result) = memo.get(&used) {
                return result;
            }
            for i in 1..=n {
                let mask = 1u32 << i;
                if used & mask != 0 {
                    continue;
                } // already chosen
                // If picking i reaches or exceeds remaining, current player wins
                if i >= remaining || !Self::can_win(n, remaining - i, used | mask, memo) {
                    memo.insert(used, true);
                    return true;
                }
            }
            memo.insert(used, false);
            false
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_10_11() {
            assert!(!Solution::can_i_win(10, 11));
        }
        #[test]
        fn test_10_0() {
            assert!(Solution::can_i_win(10, 0));
        }
        #[test]
        fn test_10_1() {
            assert!(Solution::can_i_win(10, 1));
        }
        #[test]
        fn test_impossible() {
            assert!(!Solution::can_i_win(5, 50));
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
    F: Fn(i32, i32) -> bool,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(10), std::hint::black_box(11)));
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
    bench_solution("My solution", my_solution::Solution::can_i_win);
    bench_solution("AI solution", ai_solution::Solution::can_i_win);
}

fn main() {
    println!("{}", ai_solution::Solution::can_i_win(10, 11));
}
