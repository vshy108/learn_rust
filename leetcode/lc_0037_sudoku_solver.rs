// LeetCode 37: Sudoku Solver [Hard]
// https://leetcode.com/problems/sudoku-solver/
//
// First benchmark: 10_000 iterations, 2025-07-08
//   My solution avg: 4.287ms
//   AI solution avg: 1.918ms
//   Ratio: AI solution is 2.23x faster
//
// Write a program to solve a Sudoku puzzle by filling the empty cells.
// A sudoku solution must satisfy all rules of Valid Sudoku (LC 36).
// The input board contains '.' for empty cells. There is exactly one solution.
//
// Constraints:
//   board.length == 9, board[i].length == 9
//   board[i][j] is a digit or '.'
//   Guaranteed exactly one solution
//
// Run:  cargo run --example lc_0037_sudoku_solver
// Test: cargo test --example lc_0037_sudoku_solver
// Bench: cargo test --example lc_0037_sudoku_solver bench -- --nocapture

/// My solution — backtracking with HashSet.
///
/// Find the first empty cell, try digits 1-9, check validity via sets,
/// recurse, and backtrack on failure.
///
/// Time: O(9^m) where m = number of empty cells (worst case).
/// Space: O(m) recursion depth + O(81) for tracking sets.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Solve the board in-place.
        ///
        /// # Arguments
        /// * `board` - 9x9 grid to fill
        pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
            Self::solve(board);
        }

        /// Recursive backtracking solver.
        fn solve(board: &mut Vec<Vec<char>>) -> bool {
            // Find the first empty cell.
            for r in 0..9 {
                for c in 0..9 {
                    if board[r][c] == '.' {
                        // Try each digit.
                        for d in b'1'..=b'9' {
                            let ch = d as char;
                            if Self::is_valid(board, r, c, ch) {
                                board[r][c] = ch;
                                if Self::solve(board) {
                                    return true;
                                }
                                board[r][c] = '.'; // backtrack
                            }
                        }
                        return false; // no valid digit — backtrack
                    }
                }
            }
            true // no empty cells left — solved
        }

        /// Check if placing `ch` at (r, c) is valid.
        fn is_valid(board: &[Vec<char>], r: usize, c: usize, ch: char) -> bool {
            for i in 0..9 {
                // Check row.
                if board[r][i] == ch {
                    return false;
                }
                // Check column.
                if board[i][c] == ch {
                    return false;
                }
                // Check 3x3 box.
                let br = (r / 3) * 3 + i / 3;
                let bc = (c / 3) * 3 + i % 3;
                if board[br][bc] == ch {
                    return false;
                }
            }
            true
        }
    }
}

/// AI-optimized solution — backtracking with bitmask constraint propagation.
///
/// Maintain bitmasks for rows, cols, and boxes. When placing a digit, update
/// the bitmask; when backtracking, clear it. Validity check is a single
/// bitwise AND instead of three loops of 9.
///
/// Time: O(9^m) worst case, but much faster in practice due to O(1) checks.
/// Space: O(m) recursion + 27 u16 bitmasks.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Solve the board in-place.
        ///
        /// # Arguments
        /// * `board` - 9x9 grid to fill
        pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
            let mut rows = [0u16; 9];
            let mut cols = [0u16; 9];
            let mut boxes = [0u16; 9];

            // Initialize bitmasks from the given board.
            for r in 0..9 {
                for c in 0..9 {
                    if board[r][c] != '.' {
                        let bit = 1u16 << (board[r][c] as u16 - b'0' as u16);
                        rows[r] |= bit;
                        cols[c] |= bit;
                        boxes[(r / 3) * 3 + c / 3] |= bit;
                    }
                }
            }

            Self::solve(board, &mut rows, &mut cols, &mut boxes);
        }

        /// Recursive solver using bitmask state.
        fn solve(
            board: &mut Vec<Vec<char>>,
            rows: &mut [u16; 9],
            cols: &mut [u16; 9],
            boxes: &mut [u16; 9],
        ) -> bool {
            // Find first empty cell.
            for r in 0..9 {
                for c in 0..9 {
                    if board[r][c] == '.' {
                        let box_idx = (r / 3) * 3 + c / 3;
                        // Combined mask of digits already used.
                        let used = rows[r] | cols[c] | boxes[box_idx];

                        for d in 1u16..=9 {
                            let bit = 1u16 << d;
                            if used & bit != 0 {
                                continue; // digit already used
                            }
                            // Place digit.
                            board[r][c] = (b'0' + d as u8) as char;
                            rows[r] |= bit;
                            cols[c] |= bit;
                            boxes[box_idx] |= bit;

                            if Self::solve(board, rows, cols, boxes) {
                                return true;
                            }

                            // Backtrack.
                            board[r][c] = '.';
                            rows[r] &= !bit;
                            cols[c] &= !bit;
                            boxes[box_idx] &= !bit;
                        }
                        return false;
                    }
                }
            }
            true
        }
    }
}

fn main() {
    let initial = vec![
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

    println!("--- My Solution ---");
    let mut board = initial.clone();
    my_solution::Solution::solve_sudoku(&mut board);
    for row in &board {
        let line: String = row.iter().collect();
        println!("{line}");
    }

    println!("--- AI Solution ---");
    let mut board = initial;
    ai_solution::Solution::solve_sudoku(&mut board);
    for row in &board {
        let line: String = row.iter().collect();
        println!("{line}");
    }
}

#[cfg(test)]
mod tests {
    fn puzzle() -> Vec<Vec<char>> {
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

    fn solution() -> Vec<Vec<char>> {
        vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ]
    }

    macro_rules! test_sudoku_solver {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_standard_puzzle() {
                    let mut board = super::puzzle();
                    Solution::solve_sudoku(&mut board);
                    assert_eq!(board, super::solution());
                }

                #[test]
                fn test_already_solved() {
                    let mut board = super::solution();
                    Solution::solve_sudoku(&mut board);
                    assert_eq!(board, super::solution());
                }
            }
        };
    }

    test_sudoku_solver!(my_tests, crate::my_solution::Solution);
    test_sudoku_solver!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        // Sudoku solving is heavier, use fewer iterations.
        let iterations = 10_000;
        let base = puzzle();

        let start = Instant::now();
        for _ in 0..iterations {
            let mut board = base.clone();
            crate::my_solution::Solution::solve_sudoku(std::hint::black_box(&mut board));
            std::hint::black_box(&board);
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            let mut board = base.clone();
            crate::ai_solution::Solution::solve_sudoku(std::hint::black_box(&mut board));
            std::hint::black_box(&board);
        }
        let ai_duration = start.elapsed();

        println!("\n=== Benchmark ({iterations} iterations) ===");
        println!(
            "  My solution: {:?} (avg {:?})",
            my_duration,
            my_duration / iterations as u32
        );
        println!(
            "  AI solution: {:?} (avg {:?})",
            ai_duration,
            ai_duration / iterations as u32
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
