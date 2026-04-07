// LeetCode 51: N-Queens [Hard]
// https://leetcode.com/problems/n-queens/
//
// First benchmark: 10_000 iterations, 2025-07-08
//   My solution avg: 7387.278us
//   AI solution avg: 274.631us
//   Ratio: AI 26.90x faster
//
// Place n queens on an n x n chessboard such that no two queens attack each
// other. Return all distinct solutions, each as a board configuration of
// strings where 'Q' and '.' indicate a queen and empty space.
//
// Example 1: n=4 -> [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
// Example 2: n=1 -> [["Q"]]
//
// Constraints: 1 <= n <= 9
//
// Run:  cargo run --example lc_0051_n_queens
// Test: cargo test --example lc_0051_n_queens
// Bench: cargo test --example lc_0051_n_queens bench -- --nocapture

/// My solution — backtracking with HashSet for conflict detection.
///
/// Place queens row by row. Track occupied columns, diagonals (r-c),
/// and anti-diagonals (r+c) using HashSets.
///
/// Time: O(n!) — prune branches via conflict sets.
/// Space: O(n^2) for output + O(n) for the sets.
mod my_solution {
    use std::collections::HashSet;

    pub struct Solution;

    impl Solution {
        /// Solve the N-Queens problem.
        ///
        /// # Arguments
        /// * `n` - board size and number of queens
        ///
        /// # Returns
        /// All valid board configurations
        pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
            let n = n as usize;
            let mut result = Vec::new();
            let mut queens: Vec<usize> = Vec::with_capacity(n);
            let mut cols = HashSet::new();
            let mut diags = HashSet::new();       // row - col
            let mut anti_diags = HashSet::new();  // row + col
            Self::backtrack(n, 0, &mut queens, &mut cols, &mut diags, &mut anti_diags, &mut result);
            result
        }

        fn backtrack(
            n: usize,
            row: usize,
            queens: &mut Vec<usize>,
            cols: &mut HashSet<i32>,
            diags: &mut HashSet<i32>,
            anti_diags: &mut HashSet<i32>,
            result: &mut Vec<Vec<String>>,
        ) {
            if row == n {
                let board = queens
                    .iter()
                    .map(|&c| {
                        let mut s = vec![b'.'; n];
                        s[c] = b'Q';
                        // Safety: only ASCII bytes
                        String::from_utf8(s).expect("valid ASCII board row")
                    })
                    .collect();
                result.push(board);
                return;
            }
            for col in 0..n {
                let c = col as i32;
                let r = row as i32;
                if cols.contains(&c) || diags.contains(&(r - c)) || anti_diags.contains(&(r + c)) {
                    continue;
                }
                cols.insert(c);
                diags.insert(r - c);
                anti_diags.insert(r + c);
                queens.push(col);
                Self::backtrack(n, row + 1, queens, cols, diags, anti_diags, result);
                queens.pop();
                cols.remove(&c);
                diags.remove(&(r - c));
                anti_diags.remove(&(r + c));
            }
        }
    }
}

/// AI solution — backtracking with bitmask conflict detection.
///
/// Replace HashSets with integer bitmasks for columns, diagonals,
/// and anti-diagonals. Bitwise operations are faster than hash lookups.
///
/// Time: O(n!) — same pruning.
/// Space: O(n^2) for output + O(1) for bitmask state.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Solve the N-Queens problem.
        ///
        /// # Arguments
        /// * `n` - board size and number of queens
        ///
        /// # Returns
        /// All valid board configurations
        pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
            let n = n as usize;
            let mut result = Vec::new();
            let mut queens: Vec<usize> = Vec::with_capacity(n);
            // Bitmasks: bit i set means column/diagonal i is occupied
            Self::backtrack(n, 0, 0, 0, 0, &mut queens, &mut result);
            result
        }

        fn backtrack(
            n: usize,
            row: usize,
            cols: u32,
            diags: u32,
            anti_diags: u32,
            queens: &mut Vec<usize>,
            result: &mut Vec<Vec<String>>,
        ) {
            if row == n {
                let board = queens
                    .iter()
                    .map(|&c| {
                        let mut s = vec![b'.'; n];
                        s[c] = b'Q';
                        String::from_utf8(s).expect("valid ASCII board row")
                    })
                    .collect();
                result.push(board);
                return;
            }
            for col in 0..n {
                let bit = 1u32 << col;
                // Diagonal index offset by n to keep non-negative
                let d_bit = 1u32 << (row + n - col);
                let ad_bit = 1u32 << (row + col);
                if cols & bit != 0 || diags & d_bit != 0 || anti_diags & ad_bit != 0 {
                    continue;
                }
                queens.push(col);
                Self::backtrack(
                    n,
                    row + 1,
                    cols | bit,
                    diags | d_bit,
                    anti_diags | ad_bit,
                    queens,
                    result,
                );
                queens.pop();
            }
        }
    }
}

fn main() {
    for n in [1, 4] {
        let my = my_solution::Solution::solve_n_queens(n);
        let ai = ai_solution::Solution::solve_n_queens(n);
        println!("n={n}: my has {} solutions, ai has {} solutions", my.len(), ai.len());
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_n_queens {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                fn sorted(mut v: Vec<Vec<String>>) -> Vec<Vec<String>> {
                    v.sort();
                    v
                }

                #[test]
                fn test_n1() {
                    let result = <$solution>::solve_n_queens(1);
                    assert_eq!(result, vec![vec!["Q".to_string()]]);
                }

                #[test]
                fn test_n4() {
                    let result = sorted(<$solution>::solve_n_queens(4));
                    assert_eq!(result.len(), 2);
                    let expected = sorted(vec![
                        vec![".Q..", "...Q", "Q...", "..Q."],
                        vec!["..Q.", "Q...", "...Q", ".Q.."],
                    ]
                    .into_iter()
                    .map(|b| b.into_iter().map(String::from).collect())
                    .collect());
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_n2() {
                    let result = <$solution>::solve_n_queens(2);
                    assert!(result.is_empty());
                }

                #[test]
                fn test_n3() {
                    let result = <$solution>::solve_n_queens(3);
                    assert!(result.is_empty());
                }

                #[test]
                fn test_n8() {
                    let result = <$solution>::solve_n_queens(8);
                    assert_eq!(result.len(), 92); // known count
                }
            }
        };
    }

    test_n_queens!(my, crate::my_solution::Solution);
    test_n_queens!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 10_000;
        let n = 8;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::solve_n_queens(std::hint::black_box(n)));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::solve_n_queens(std::hint::black_box(n)));
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
