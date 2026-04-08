// LeetCode 52: N-Queens II [Hard]
// https://leetcode.com/problems/n-queens-ii/
//
// First benchmark: 10_000 iterations, 2025-07-08
//   My solution avg: 7263.259us
//   AI solution avg: 182.175us
//   Ratio: AI 39.87x faster
//
// Return the number of distinct solutions to the n-queens puzzle.
//
// Example 1: n=4 -> 2
// Example 2: n=1 -> 1
//
// Constraints: 1 <= n <= 9
//
// Run:  cargo run --example lc_0052_n_queens_ii
// Test: cargo test --example lc_0052_n_queens_ii
// Bench: cargo test --example lc_0052_n_queens_ii bench -- --nocapture

/// My solution — backtracking with HashSet (count only).
///
/// Same as N-Queens but only count valid placements, don't build boards.
///
/// Time: O(n!) — backtracking with pruning.
/// Space: O(n) — sets + recursion.
mod my_solution {
    use std::collections::HashSet;

    pub struct Solution;

    impl Solution {
        /// Count distinct N-Queens solutions.
        ///
        /// # Arguments
        /// * `n` - board size
        ///
        /// # Returns
        /// Number of valid placements
        pub fn total_n_queens(n: i32) -> i32 {
            let n = n as usize;
            let mut cols = HashSet::new();
            let mut diags = HashSet::new();
            let mut anti_diags = HashSet::new();
            Self::backtrack(n, 0, &mut cols, &mut diags, &mut anti_diags)
        }

        fn backtrack(
            n: usize,
            row: usize,
            cols: &mut HashSet<i32>,
            diags: &mut HashSet<i32>,
            anti_diags: &mut HashSet<i32>,
        ) -> i32 {
            if row == n {
                return 1;
            }
            let mut count = 0;
            for col in 0..n {
                let c = col as i32;
                let r = row as i32;
                if cols.contains(&c) || diags.contains(&(r - c)) || anti_diags.contains(&(r + c)) {
                    continue;
                }
                cols.insert(c);
                diags.insert(r - c);
                anti_diags.insert(r + c);
                count += Self::backtrack(n, row + 1, cols, diags, anti_diags);
                cols.remove(&c);
                diags.remove(&(r - c));
                anti_diags.remove(&(r + c));
            }
            count
        }
    }
}

/// AI solution — backtracking with bitmask (count only).
///
/// Use integer bitmasks instead of HashSets for O(1) conflict checks.
///
/// Time: O(n!) — same pruning.
/// Space: O(n) recursion depth, O(1) state.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Count distinct N-Queens solutions.
        ///
        /// # Arguments
        /// * `n` - board size
        ///
        /// # Returns
        /// Number of valid placements
        pub fn total_n_queens(n: i32) -> i32 {
            let n = n as usize;
            Self::backtrack(n, 0, 0, 0, 0)
        }

        fn backtrack(n: usize, row: usize, cols: u32, diags: u32, anti_diags: u32) -> i32 {
            if row == n {
                return 1;
            }
            let mut count = 0;
            for col in 0..n {
                let bit = 1u32 << col;
                let d_bit = 1u32 << (row + n - col);
                let ad_bit = 1u32 << (row + col);
                if cols & bit != 0 || diags & d_bit != 0 || anti_diags & ad_bit != 0 {
                    continue;
                }
                count +=
                    Self::backtrack(n, row + 1, cols | bit, diags | d_bit, anti_diags | ad_bit);
            }
            count
        }
    }
}

fn main() {
    for n in [1, 4, 8] {
        println!(
            "n={n}: my:{}, ai:{}",
            my_solution::Solution::total_n_queens(n),
            ai_solution::Solution::total_n_queens(n),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_total_n_queens {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_n1() {
                    assert_eq!(<$solution>::total_n_queens(1), 1);
                }

                #[test]
                fn test_n4() {
                    assert_eq!(<$solution>::total_n_queens(4), 2);
                }

                #[test]
                fn test_n2() {
                    assert_eq!(<$solution>::total_n_queens(2), 0);
                }

                #[test]
                fn test_n3() {
                    assert_eq!(<$solution>::total_n_queens(3), 0);
                }

                #[test]
                fn test_n8() {
                    assert_eq!(<$solution>::total_n_queens(8), 92);
                }

                #[test]
                fn test_n9() {
                    assert_eq!(<$solution>::total_n_queens(9), 352);
                }
            }
        };
    }

    test_total_n_queens!(my, crate::my_solution::Solution);
    test_total_n_queens!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 10_000;
        let n = 8;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::total_n_queens(
                std::hint::black_box(n),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::total_n_queens(
                std::hint::black_box(n),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg ({:.0}ms total)",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            my_duration.as_millis(),
        );
        println!(
            "AI solution: {:.3}us avg ({:.0}ms total)",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            ai_duration.as_millis(),
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("~tied (ratio: {ratio:.2})");
        }
    }
}
