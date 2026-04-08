// LeetCode 441: Arranging Coins [Easy]
// https://leetcode.com/problems/arranging-coins/
//
// You have n coins and want to build a staircase. The i-th row has exactly i
// coins. The last row may be incomplete. Return the number of complete rows.
//
// Examples:
//   n = 5 -> 2  (rows: 1, 2, [2 remaining, row 3 incomplete])
//   n = 8 -> 3  (rows: 1, 2, 3, [2 remaining, row 4 incomplete])
//
// Constraints:
//   1 <= n <= 2^31 - 1
//
// Run:  cargo run --example lc_0441_arranging_coins
// Test: cargo test --example lc_0441_arranging_coins
// Bench: cargo test --example lc_0441_arranging_coins bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 110ns (binary search)
//   AI solution: avg ~0ns  (sqrt formula)
//   Ratio: >>1x (math formula much faster than binary search)

/// My solution module — binary search.
///
/// Binary search for the largest k such that k*(k+1)/2 <= n.
/// Use i64 arithmetic to avoid overflow.
///
/// Time:  O(log n)
/// Space: O(1)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the number of complete staircase rows.
        pub fn arrange_coins(n: i32) -> i32 {
            // Binary search approach instead of sqrt formula
            let n = n as i64;
            let (mut lo, mut hi) = (1i64, n);
            while lo <= hi {
                let mid = lo + (hi - lo) / 2;
                let used = mid * (mid + 1) / 2;
                if used <= n {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
            hi as i32
        }
    }
}

/// AI solution module — math formula.
///
/// Solve k*(k+1)/2 <= n via the quadratic formula:
/// k = floor(sqrt(2*n + 0.25) - 0.5)
///
/// Time:  O(1)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the number of complete staircase rows.
        pub fn arrange_coins(n: i32) -> i32 {
            // k*(k+1)/2 <= n  =>  k = floor(sqrt(2n + 0.25) - 0.5)
            (((2.0_f64 * n as f64 + 0.25).sqrt() - 0.5) as i64).min(n as i64) as i32
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;

        #[test]
        fn test_n5() {
            assert_eq!(Solution::arrange_coins(5), 2);
        }

        #[test]
        fn test_n8() {
            assert_eq!(Solution::arrange_coins(8), 3);
        }

        #[test]
        fn test_n1() {
            assert_eq!(Solution::arrange_coins(1), 1);
        }

        #[test]
        fn test_n3() {
            // Rows: 1, 2 -> exactly 3 coins, 2 complete rows
            assert_eq!(Solution::arrange_coins(3), 2);
        }

        #[test]
        fn test_n0_edge() {
            // Smallest valid is 1 per constraints, but testing n=1
            assert_eq!(Solution::arrange_coins(1), 1);
        }

        #[test]
        fn test_large() {
            assert_eq!(Solution::arrange_coins(i32::MAX), 65535);
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
    F: Fn(i32) -> i32,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(i32::MAX)));
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
    bench_solution("My solution", my_solution::Solution::arrange_coins);
    bench_solution("AI solution", ai_solution::Solution::arrange_coins);
}

fn main() {
    println!(
        "arrange_coins(5) = {}",
        ai_solution::Solution::arrange_coins(5)
    );
    println!(
        "arrange_coins(8) = {}",
        ai_solution::Solution::arrange_coins(8)
    );
}
