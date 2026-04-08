// @lc app=leetcode id=348 lang=rust
// #[Medium] - Design Tic-Tac-Toe
// Iterations: 100000 | Date: 2025-07-09
// my_solution  | avg: 0.184us
// ai_solution  | avg: 0.079us
// Performance: AI 2.32x faster

/// # 348. Design Tic-Tac-Toe
///
/// Implement TicTacToe on an n x n board. Each move checks for a winner in O(1).
///
/// Constraints:
/// - 2 <= n <= 100
/// - player is 1 or 2
/// - (row, col) unique per call
/// - At most n^2 calls to move

/// ## My Solution — Counter array O(1) per move
///
/// Track counts for each row, column, and 2 diagonals per player.
/// Use a flat array of length 2*(2n+2). Check if any counter reaches n.
///
/// - Time: O(1) per move
/// - Space: O(n)
mod my_solution {
    /// Plays all moves and returns the result of each move (0, 1, or 2).
    pub fn play(n: usize, moves: &[(usize, usize, usize)]) -> Vec<i32> {
        // cnt[player][row], cnt[player][n+col], cnt[player][2n]=diag, cnt[player][2n+1]=anti-diag
        let len = 2 * n + 2;
        let mut cnt = vec![vec![0i32; len]; 2];
        let mut results = Vec::with_capacity(moves.len());
        for &(row, col, player) in moves {
            let p = player - 1; // 0-indexed player
            cnt[p][row] += 1;
            cnt[p][n + col] += 1;
            if row == col {
                cnt[p][2 * n] += 1;
            }
            if row + col == n - 1 {
                cnt[p][2 * n + 1] += 1;
            }
            let n_i32 = n as i32;
            if cnt[p][row] == n_i32
                || cnt[p][n + col] == n_i32
                || cnt[p][2 * n] == n_i32
                || cnt[p][2 * n + 1] == n_i32
            {
                results.push(player as i32);
            } else {
                results.push(0);
            }
        }
        results
    }
}

/// ## AI Solution — Two 1D arrays with +1/-1 encoding
///
/// Instead of separate arrays per player, use a single array.
/// Player 1 adds +1, player 2 adds -1. Win when abs(count) == n.
///
/// - Time: O(1) per move
/// - Space: O(n)
mod ai_solution {
    /// Plays all moves and returns the result of each move (0, 1, or 2).
    pub fn play(n: usize, moves: &[(usize, usize, usize)]) -> Vec<i32> {
        let len = 2 * n + 2;
        let mut cnt = vec![0i32; len]; // single array, +1 for p1, -1 for p2
        let mut results = Vec::with_capacity(moves.len());
        let n_i32 = n as i32;
        for &(row, col, player) in moves {
            // +1 for player 1, -1 for player 2
            let delta: i32 = if player == 1 { 1 } else { -1 };
            cnt[row] += delta;
            cnt[n + col] += delta;
            if row == col {
                cnt[2 * n] += delta;
            }
            if row + col == n - 1 {
                cnt[2 * n + 1] += delta;
            }
            if cnt[row].abs() == n_i32
                || cnt[n + col].abs() == n_i32
                || cnt[2 * n].abs() == n_i32
                || cnt[2 * n + 1].abs() == n_i32
            {
                results.push(player as i32);
            } else {
                results.push(0);
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(usize, &[(usize, usize, usize)]) -> Vec<i32>) {
        // Example: n=3, moves produce [0,0,0,0,0,0,1]
        let moves = vec![
            (0, 0, 1),
            (0, 2, 2),
            (2, 2, 1),
            (1, 1, 2),
            (2, 0, 1),
            (1, 0, 2),
            (2, 1, 1), // player 1 wins (row 2 complete)
        ];
        assert_eq!(f(3, &moves), vec![0, 0, 0, 0, 0, 0, 1]);

        // Diagonal win
        let moves = vec![(0, 0, 1), (1, 0, 2), (1, 1, 1), (2, 0, 2), (2, 2, 1)];
        assert_eq!(f(3, &moves), vec![0, 0, 0, 0, 1]);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::play);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::play);
    }
}

fn main() {
    let n = 3;
    let moves = vec![
        (0, 0, 1),
        (0, 2, 2),
        (2, 2, 1),
        (1, 1, 2),
        (2, 0, 1),
        (1, 0, 2),
        (2, 1, 1),
    ];
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::play(n, std::hint::black_box(&moves)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::play(n, std::hint::black_box(&moves)));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::play(n, std::hint::black_box(&moves)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::play(n, std::hint::black_box(&moves)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 348 - Design Tic-Tac-Toe");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
