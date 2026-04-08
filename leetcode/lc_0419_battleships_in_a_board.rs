// Benchmark: 100,000 iterations (2025-04-08)
// My solution:  avg 13.26us  | Imperative nested loops, check top-left corner
// AI solution:  avg 11.50us  | Iterator flat_map + filter, same logic
// -> AI solution is 1.15x faster (tied)
//
// LeetCode 419: Battleships in a Board [Medium]
// https://leetcode.com/problems/battleships-in-a-board/
//
// Given an m x n board with 'X' (battleship) and '.' (empty),
// count the number of battleships. Battleships are 1xk or kx1,
// separated by at least one cell.
//
// Examples:
//   [["X",".",".","X"],[".",".",".","X"],[".",".",".","X"]] -> 2
//   [["."]]) -> 0
//
// Constraints:
//   m == board.length, n == board[i].length
//   1 <= m, n <= 200
//   board[i][j] is '.' or 'X'
//
// Run:  cargo run --example lc_0419_battleships_in_a_board
// Test: cargo test --example lc_0419_battleships_in_a_board
// Bench: cargo test --example lc_0419_battleships_in_a_board bench -- --nocapture

/// My solution module — count top-left corners only.
///
/// A cell is the "start" of a battleship if it is 'X' and neither
/// the cell above nor the cell to the left is 'X'. O(1) extra space,
/// single pass, no modifications to the board.
///
/// Time:  O(m * n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Count battleships on the board.
        ///
        /// # Arguments
        ///
        /// * `board` - m x n grid of 'X' and '.'
        ///
        /// # Returns
        ///
        /// Number of battleships
        pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
            let m = board.len();
            let n = board[0].len();
            let mut count = 0;

            for i in 0..m {
                for j in 0..n {
                    if board[i][j] == '.' {
                        continue;
                    }
                    // Skip if part of a ship already counted from above
                    if i > 0 && board[i - 1][j] == 'X' {
                        continue;
                    }
                    // Skip if part of a ship already counted from the left
                    if j > 0 && board[i][j - 1] == 'X' {
                        continue;
                    }
                    count += 1;
                }
            }

            count
        }
    }
}

/// AI solution module — same logic using iterators and flat_map.
///
/// Functional-style: enumerate each cell with (i,j), filter by
/// "top-left corner" condition, count the results.
///
/// Time:  O(m * n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Count battleships on the board.
        ///
        /// # Arguments
        ///
        /// * `board` - m x n grid of 'X' and '.'
        ///
        /// # Returns
        ///
        /// Number of battleships
        pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
            // Use a reference to avoid move-vs-borrow conflict:
            // `board.iter()` borrows board, but the inner closure
            // also needs to index `board[i-1][j]`, so we bind a
            // shared reference first.
            let b = &board;
            b.iter()
                .enumerate()
                .flat_map(|(i, row)| {
                    row.iter().enumerate().filter(move |(j, c)| {
                        **c == 'X'
                            && (i == 0 || b[i - 1][*j] != 'X')
                            && (*j == 0 || b[i][j - 1] != 'X')
                    })
                })
                .count() as i32
        }
    }
}

fn main() {
    let cases: Vec<(Vec<Vec<char>>, i32)> = vec![
        (
            vec![
                vec!['X', '.', '.', 'X'],
                vec!['.', '.', '.', 'X'],
                vec!['.', '.', '.', 'X'],
            ],
            2,
        ),
        (vec![vec!['.']], 0),
    ];

    println!("--- My Solution ---");
    for (board, expected) in &cases {
        let result =
            my_solution::Solution::count_battleships(board.clone());
        println!("  result={result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (board, expected) in &cases {
        let result =
            ai_solution::Solution::count_battleships(board.clone());
        println!("  result={result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_battleships {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    let board = vec![
                        vec!['X', '.', '.', 'X'],
                        vec!['.', '.', '.', 'X'],
                        vec!['.', '.', '.', 'X'],
                    ];
                    assert_eq!(Solution::count_battleships(board), 2);
                }

                #[test]
                fn test_empty() {
                    assert_eq!(
                        Solution::count_battleships(vec![vec!['.']]),
                        0,
                    );
                }

                #[test]
                fn test_single_ship() {
                    assert_eq!(
                        Solution::count_battleships(vec![vec!['X']]),
                        1,
                    );
                }

                #[test]
                fn test_horizontal_ship() {
                    assert_eq!(
                        Solution::count_battleships(
                            vec![vec!['X', 'X', 'X']],
                        ),
                        1,
                    );
                }

                #[test]
                fn test_multiple_separated() {
                    let board = vec![
                        vec!['X', '.', 'X'],
                        vec!['.', '.', '.'],
                        vec!['X', '.', 'X'],
                    ];
                    assert_eq!(Solution::count_battleships(board), 4);
                }
            }
        };
    }

    test_battleships!(my_tests, crate::my_solution::Solution);
    test_battleships!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;

        // 100x100 grid with scattered ships
        let mut board = vec![vec!['.'; 100]; 100];
        for i in (0..100).step_by(3) {
            for j in (0..100).step_by(3) {
                board[i][j] = 'X';
            }
        }

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::count_battleships(
                    std::hint::black_box(board.clone()),
                ),
            );
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::count_battleships(
                    std::hint::black_box(board.clone()),
                ),
            );
        }
        let ai_duration = start.elapsed();

        let my_avg = my_duration / iterations;
        let ai_avg = ai_duration / iterations;

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution:  {my_duration:.2?} (avg {my_avg:.2?})");
        println!("AI solution:  {ai_duration:.2?} (avg {ai_avg:.2?})");

        if my_avg <= ai_avg {
            let ratio = ai_avg.as_nanos() as f64
                / my_avg.as_nanos().max(1) as f64;
            println!("-> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_avg.as_nanos() as f64
                / ai_avg.as_nanos().max(1) as f64;
            println!("-> AI solution is {ratio:.2}x faster");
        }
    }
}
