// LeetCode 36: Valid Sudoku [Medium]
// https://leetcode.com/problems/valid-sudoku/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 37.804µs
//   AI solution avg: 2.381µs
//   Ratio: AI solution is 15.88x faster
//
// Determine if a 9x9 Sudoku board is valid. Only the filled cells need to
// be validated according to the rules:
//   1. Each row must contain digits 1-9 without repetition.
//   2. Each column must contain digits 1-9 without repetition.
//   3. Each of the nine 3x3 sub-boxes must contain digits 1-9 without repetition.
//
// A partially filled board (with '.') can be valid even if not solvable.
//
// Constraints:
//   board.length == 9
//   board[i].length == 9
//   board[i][j] is a digit 1-9 or '.'
//
// Run:  cargo run --example lc_0036_valid_sudoku
// Test: cargo test --example lc_0036_valid_sudoku
// Bench: cargo test --example lc_0036_valid_sudoku bench -- --nocapture

/// My solution — HashSet-based validation.
///
/// For each cell, check if the digit already appeared in its row, column,
/// or 3x3 box using three arrays of HashSets.
///
/// Time: O(1) — board is always 9x9 = 81 cells.
/// Space: O(1) — fixed-size sets (at most 9 elements each, 27 sets).
mod my_solution {
    use std::collections::HashSet;

    pub struct Solution;

    impl Solution {
        /// Check if the board is a valid Sudoku configuration.
        ///
        /// # Arguments
        /// * `board` - 9x9 grid of chars ('1'-'9' or '.')
        ///
        /// # Returns
        /// `true` if the board is valid.
        pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
            let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
            let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); 9];
            let mut boxes: Vec<HashSet<char>> = vec![HashSet::new(); 9];

            for r in 0..9 {
                for c in 0..9 {
                    let ch = board[r][c];
                    if ch == '.' {
                        continue;
                    }

                    // Box index: (r/3)*3 + c/3 maps each 3x3 block to 0..8.
                    let box_idx = (r / 3) * 3 + c / 3;

                    if !rows[r].insert(ch) || !cols[c].insert(ch) || !boxes[box_idx].insert(ch) {
                        return false;
                    }
                }
            }

            true
        }
    }
}

/// AI-optimized solution — bitmask validation.
///
/// Replace HashSets with u16 bitmasks (bit i represents digit i).
/// Checking and inserting is a single bitwise AND + OR.
///
/// Time: O(1) — 81 cells.
/// Space: O(1) — 27 u16 values on the stack.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Check if the board is a valid Sudoku configuration.
        ///
        /// # Arguments
        /// * `board` - 9x9 grid of chars ('1'-'9' or '.')
        ///
        /// # Returns
        /// `true` if the board is valid.
        pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
            // Each u16 uses bits 1..=9 to track which digits are present.
            let mut rows = [0u16; 9];
            let mut cols = [0u16; 9];
            let mut boxes = [0u16; 9];

            for r in 0..9 {
                for c in 0..9 {
                    let ch = board[r][c];
                    if ch == '.' {
                        continue;
                    }

                    // Convert '1'-'9' to bit position 1..9.
                    let bit = 1u16 << (ch as u16 - b'0' as u16);
                    let box_idx = (r / 3) * 3 + c / 3;

                    // If the bit is already set in any of the three masks, duplicate.
                    if (rows[r] | cols[c] | boxes[box_idx]) & bit != 0 {
                        return false;
                    }

                    rows[r] |= bit;
                    cols[c] |= bit;
                    boxes[box_idx] |= bit;
                }
            }

            true
        }
    }
}

fn main() {
    let board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    println!(
        "valid = my:{}, ai:{}",
        my_solution::Solution::is_valid_sudoku(board.clone()),
        ai_solution::Solution::is_valid_sudoku(board),
    );
}

#[cfg(test)]
mod tests {
    fn valid_board() -> Vec<Vec<char>> {
        vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]
    }

    fn invalid_board_row_dup() -> Vec<Vec<char>> {
        // Duplicate '8' in row 0.
        vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]
    }

    fn invalid_board_col_dup() -> Vec<Vec<char>> {
        // Duplicate '5' in column 0.
        vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['5', '.', '.', '1', '9', '.', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '.'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]
    }

    fn invalid_board_box_dup() -> Vec<Vec<char>> {
        // Duplicate '9' in top-right 3x3 box.
        vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '9', '.'],
            vec!['.', '.', '.', '.', '.', '.', '9', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ]
    }

    fn empty_board() -> Vec<Vec<char>> {
        vec![vec!['.'; 9]; 9]
    }

    macro_rules! test_valid_sudoku {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_valid() {
                    assert!(Solution::is_valid_sudoku(super::valid_board()));
                }

                #[test]
                fn test_invalid_row() {
                    assert!(!Solution::is_valid_sudoku(super::invalid_board_row_dup()));
                }

                #[test]
                fn test_invalid_col() {
                    assert!(!Solution::is_valid_sudoku(super::invalid_board_col_dup()));
                }

                #[test]
                fn test_invalid_box() {
                    assert!(!Solution::is_valid_sudoku(super::invalid_board_box_dup()));
                }

                #[test]
                fn test_empty() {
                    assert!(Solution::is_valid_sudoku(super::empty_board()));
                }
            }
        };
    }

    test_valid_sudoku!(my_tests, crate::my_solution::Solution);
    test_valid_sudoku!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        let board = valid_board();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::is_valid_sudoku(
                std::hint::black_box(board.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::is_valid_sudoku(
                std::hint::black_box(board.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n=== Benchmark ({iterations} iterations) ===");
        println!(
            "  My solution: {:?} (avg {:?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "  AI solution: {:?} (avg {:?})",
            ai_duration,
            ai_duration / iterations
        );

        if my_duration < ai_duration {
            let ratio = ai_duration.as_nanos() as f64 / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64 / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
