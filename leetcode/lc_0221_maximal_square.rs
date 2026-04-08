// LeetCode 221: Maximal Square [Medium]
// https://leetcode.com/problems/maximal-square/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 2.907us
//   AI solution avg: 1.954us
//   Ratio: AI 1.49x faster
//
// Given an m x n binary matrix filled with '0's and '1's, find the
// largest square containing only '1's and return its area.
//
// Constraints: m == matrix.length, n == matrix[i].length,
//              1 <= m, n <= 300, matrix[i][j] is '0' or '1'
//
// Run:  cargo run --example lc_0221_maximal_square
// Test: cargo test --example lc_0221_maximal_square
// Bench: cargo test --example lc_0221_maximal_square bench -- --nocapture

/// My solution — 2D DP table.
///
/// dp[i][j] = side length of the largest square whose bottom-right corner
/// is at (i, j). The recurrence is:
///   dp[i][j] = min(dp[i-1][j], dp[i][j-1], dp[i-1][j-1]) + 1
/// if matrix[i][j] == '1', otherwise dp[i][j] = 0.
///
/// Time: O(m * n).
/// Space: O(m * n) for the DP table.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the area of the largest square containing only '1's.
        pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
            let m = matrix.len();
            let n = matrix[0].len();
            let mut dp = vec![vec![0i32; n]; m];
            let mut max_side = 0;

            for i in 0..m {
                for j in 0..n {
                    if matrix[i][j] == '1' {
                        if i == 0 || j == 0 {
                            // First row or first column: max square side is 1
                            dp[i][j] = 1;
                        } else {
                            // Take the min of the three neighbors and add 1
                            dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
                        }
                        max_side = max_side.max(dp[i][j]);
                    }
                    // matrix[i][j] == '0' => dp[i][j] stays 0
                }
            }

            max_side * max_side
        }
    }
}

/// AI solution — 1D rolling DP array.
///
/// Instead of a full 2D table, keep only the previous row's DP values in
/// a single array. Use a `prev` variable to remember dp[i-1][j-1]
/// (the diagonal) before it gets overwritten.
///
/// Time: O(m * n).
/// Space: O(n) — single row plus a scalar.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the area of the largest square containing only '1's.
        pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
            let m = matrix.len();
            let n = matrix[0].len();
            // dp[j] stores the side length of the largest square ending
            // at column j in the previous row
            let mut dp = vec![0i32; n];
            let mut max_side = 0;

            for i in 0..m {
                // `prev` tracks dp[i-1][j-1] — the diagonal value that
                // gets overwritten when we update dp[j-1]
                let mut prev = 0;

                for j in 0..n {
                    // Save dp[j] before overwriting; it becomes the new diagonal
                    // for the next column
                    let old = dp[j];

                    if matrix[i][j] == '1' {
                        if i == 0 || j == 0 {
                            dp[j] = 1;
                        } else {
                            // dp[j] = old value = dp_prev_row[j] (top)
                            // dp[j-1] = current row's (left, already updated)
                            // prev = dp_prev_row[j-1] (diagonal)
                            dp[j] = old.min(dp[j - 1]).min(prev) + 1;
                        }
                        max_side = max_side.max(dp[j]);
                    } else {
                        dp[j] = 0;
                    }

                    prev = old;
                }
            }

            max_side * max_side
        }
    }
}

/// Convert a 2D array of &str rows into Vec<Vec<char>> for convenience
/// in main() and tests.
fn to_char_matrix(rows: &[&str]) -> Vec<Vec<char>> {
    rows.iter().map(|row| row.chars().collect()).collect()
}

fn main() {
    let matrix1 = to_char_matrix(&["10100", "10111", "11111", "10010"]);
    let my1 = my_solution::Solution::maximal_square(matrix1.clone());
    let ai1 = ai_solution::Solution::maximal_square(matrix1);
    println!("Example 1: my={my1}, ai={ai1} (expected 4)");

    let matrix2 = to_char_matrix(&["01", "10"]);
    let my2 = my_solution::Solution::maximal_square(matrix2.clone());
    let ai2 = ai_solution::Solution::maximal_square(matrix2);
    println!("Example 2: my={my2}, ai={ai2} (expected 1)");

    let matrix3 = to_char_matrix(&["0"]);
    let my3 = my_solution::Solution::maximal_square(matrix3.clone());
    let ai3 = ai_solution::Solution::maximal_square(matrix3);
    println!("Example 3: my={my3}, ai={ai3} (expected 0)");
}

#[cfg(test)]
mod tests {
    macro_rules! test_max_sq {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::super::to_char_matrix;

                #[test]
                fn test_example1() {
                    let matrix = to_char_matrix(&["10100", "10111", "11111", "10010"]);
                    assert_eq!(<$solution>::maximal_square(matrix), 4);
                }

                #[test]
                fn test_example2() {
                    let matrix = to_char_matrix(&["01", "10"]);
                    assert_eq!(<$solution>::maximal_square(matrix), 1);
                }

                #[test]
                fn test_all_zeros() {
                    let matrix = to_char_matrix(&["0"]);
                    assert_eq!(<$solution>::maximal_square(matrix), 0);
                }

                #[test]
                fn test_all_ones_3x3() {
                    let matrix = to_char_matrix(&["111", "111", "111"]);
                    assert_eq!(<$solution>::maximal_square(matrix), 9);
                }

                #[test]
                fn test_single_one() {
                    let matrix = to_char_matrix(&["1"]);
                    assert_eq!(<$solution>::maximal_square(matrix), 1);
                }

                #[test]
                fn test_row_of_ones() {
                    // A single row of all ones — best square is 1x1
                    let matrix = to_char_matrix(&["11111"]);
                    assert_eq!(<$solution>::maximal_square(matrix), 1);
                }

                #[test]
                fn test_column_of_ones() {
                    // A single column of all ones — best square is 1x1
                    let matrix = to_char_matrix(&["1", "1", "1", "1"]);
                    assert_eq!(<$solution>::maximal_square(matrix), 1);
                }

                #[test]
                fn test_2x2_square() {
                    let matrix = to_char_matrix(&["011", "011", "000"]);
                    assert_eq!(<$solution>::maximal_square(matrix), 4);
                }
            }
        };
    }

    test_max_sq!(my, crate::my_solution::Solution);
    test_max_sq!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        // Build a deterministic 20x20 matrix with a mix of '0' and '1'
        let matrix: Vec<Vec<char>> = (0..20)
            .map(|i| {
                (0..20)
                    .map(|j| if (i * 7 + j * 13) % 3 != 0 { '1' } else { '0' })
                    .collect()
            })
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::maximal_square(
                std::hint::black_box(matrix.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::maximal_square(
                std::hint::black_box(matrix.clone()),
            ));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        }
    }
}
