// LeetCode 130: Surrounded Regions [Medium]
// https://leetcode.com/problems/surrounded-regions/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 1.809us
//   AI solution avg: 1.401us
//   Ratio: AI 1.29x faster
//
// Given an m x n matrix board containing 'X' and 'O', capture all regions
// that are 4-directionally surrounded by 'X'. A region is captured by flipping
// all 'O's into 'X's. 'O's on the border (and connected to border) are NOT captured.
//
// Constraints: m == board.length, n == board[i].length, 1 <= m, n <= 200
//
// Run:  cargo run --example lc_0130_surrounded_regions
// Test: cargo test --example lc_0130_surrounded_regions
// Bench: cargo test --example lc_0130_surrounded_regions bench -- --nocapture

/// My solution — DFS from border 'O's to mark safe cells, then flip rest.
///
/// Time: O(m * n).
/// Space: O(m * n) recursion stack worst case.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Capture surrounded regions in-place.
        pub fn solve(board: &mut Vec<Vec<char>>) {
            if board.is_empty() || board[0].is_empty() {
                return;
            }
            let (rows, cols) = (board.len(), board[0].len());

            // Mark border-connected 'O's as 'S' (safe)
            for r in 0..rows {
                for c in 0..cols {
                    if (r == 0 || r == rows - 1 || c == 0 || c == cols - 1) && board[r][c] == 'O' {
                        Self::dfs(board, r, c, rows, cols);
                    }
                }
            }

            // Flip: 'O' -> 'X' (captured), 'S' -> 'O' (restored)
            for r in 0..rows {
                for c in 0..cols {
                    match board[r][c] {
                        'O' => board[r][c] = 'X',
                        'S' => board[r][c] = 'O',
                        _ => {}
                    }
                }
            }
        }

        fn dfs(board: &mut Vec<Vec<char>>, r: usize, c: usize, rows: usize, cols: usize) {
            if r >= rows || c >= cols || board[r][c] != 'O' {
                return;
            }
            board[r][c] = 'S';
            if r > 0 {
                Self::dfs(board, r - 1, c, rows, cols);
            }
            if r + 1 < rows {
                Self::dfs(board, r + 1, c, rows, cols);
            }
            if c > 0 {
                Self::dfs(board, r, c - 1, rows, cols);
            }
            if c + 1 < cols {
                Self::dfs(board, r, c + 1, rows, cols);
            }
        }
    }
}

/// AI solution — BFS from border 'O's (avoids stack overflow on large boards).
///
/// Time: O(m * n).
/// Space: O(m * n) for the queue.
mod ai_solution {
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// Capture surrounded regions using BFS.
        pub fn solve(board: &mut Vec<Vec<char>>) {
            if board.is_empty() || board[0].is_empty() {
                return;
            }
            let (rows, cols) = (board.len(), board[0].len());
            let mut queue = VecDeque::new();

            // Enqueue border 'O' cells
            for r in 0..rows {
                for c in 0..cols {
                    if (r == 0 || r == rows - 1 || c == 0 || c == cols - 1) && board[r][c] == 'O' {
                        board[r][c] = 'S';
                        queue.push_back((r, c));
                    }
                }
            }

            // BFS to mark all border-connected 'O's
            let dirs: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            while let Some((r, c)) = queue.pop_front() {
                for (dr, dc) in &dirs {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                        let (nr, nc) = (nr as usize, nc as usize);
                        if board[nr][nc] == 'O' {
                            board[nr][nc] = 'S';
                            queue.push_back((nr, nc));
                        }
                    }
                }
            }

            // Flip
            for row in board.iter_mut() {
                for cell in row.iter_mut() {
                    match cell {
                        c if *c == 'O' => *c = 'X',
                        c if *c == 'S' => *c = 'O',
                        _ => {}
                    }
                }
            }
        }
    }
}

fn main() {
    let mut board = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    my_solution::Solution::solve(&mut board);
    println!("my: {board:?}");

    let mut board2 = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    ai_solution::Solution::solve(&mut board2);
    println!("ai: {board2:?}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_solve {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    let mut board = vec![
                        vec!['X', 'X', 'X', 'X'],
                        vec!['X', 'O', 'O', 'X'],
                        vec!['X', 'X', 'O', 'X'],
                        vec!['X', 'O', 'X', 'X'],
                    ];
                    <$solution>::solve(&mut board);
                    assert_eq!(
                        board,
                        vec![
                            vec!['X', 'X', 'X', 'X'],
                            vec!['X', 'X', 'X', 'X'],
                            vec!['X', 'X', 'X', 'X'],
                            vec!['X', 'O', 'X', 'X'],
                        ]
                    );
                }

                #[test]
                fn test_single_x() {
                    let mut board = vec![vec!['X']];
                    <$solution>::solve(&mut board);
                    assert_eq!(board, vec![vec!['X']]);
                }

                #[test]
                fn test_all_o_border() {
                    let mut board = vec![vec!['O', 'O'], vec!['O', 'O']];
                    <$solution>::solve(&mut board);
                    // All on border -> none captured
                    assert_eq!(board, vec![vec!['O', 'O'], vec!['O', 'O'],]);
                }

                #[test]
                fn test_border_connected() {
                    let mut board = vec![
                        vec!['X', 'O', 'X'],
                        vec!['O', 'O', 'X'],
                        vec!['X', 'X', 'X'],
                    ];
                    <$solution>::solve(&mut board);
                    // All O's connect to border -> none captured
                    assert_eq!(
                        board,
                        vec![
                            vec!['X', 'O', 'X'],
                            vec!['O', 'O', 'X'],
                            vec!['X', 'X', 'X'],
                        ]
                    );
                }
            }
        };
    }

    test_solve!(my, crate::my_solution::Solution);
    test_solve!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let make_board = || {
            let n = 20;
            let mut board = vec![vec!['X'; n]; n];
            // Create inner O region
            for r in 2..n - 2 {
                for c in 2..n - 2 {
                    board[r][c] = 'O';
                }
            }
            board
        };

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut board = make_board();
            std::hint::black_box(crate::my_solution::Solution::solve(std::hint::black_box(
                &mut board,
            )));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut board = make_board();
            std::hint::black_box(crate::ai_solution::Solution::solve(std::hint::black_box(
                &mut board,
            )));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );
        println!(
            "AI solution: {:.3}us avg",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("Both solutions are roughly equal");
        }
    }
}
