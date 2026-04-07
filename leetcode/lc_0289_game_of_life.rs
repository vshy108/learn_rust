// LeetCode 289: Game of Life [Medium]
// https://leetcode.com/problems/game-of-life/
//
// First benchmark: 100_000 iterations, 2025-07-06
//   My solution avg: 8.513us
//   AI solution avg: 9.291us
//   Ratio: My 1.09x faster
//
// Given an m x n grid of cells (0 = dead, 1 = live), apply Conway's Game of
// Life rules simultaneously and update the board in-place:
//   1. Live cell with < 2 live neighbors -> dies (under-population)
//   2. Live cell with 2 or 3 live neighbors -> survives
//   3. Live cell with > 3 live neighbors -> dies (over-population)
//   4. Dead cell with exactly 3 live neighbors -> becomes live (reproduction)
//
// Example 1: [[0,1,0],[0,0,1],[1,1,1],[0,0,0]] -> [[0,0,0],[1,0,1],[0,1,1],[0,1,0]]
// Example 2: [[1,1],[1,0]] -> [[1,1],[1,1]]
//
// Constraints:
//   - 1 <= m, n <= 25
//   - board[i][j] is 0 or 1
//
// Run:  cargo run --example lc_0289_game_of_life
// Test: cargo test --example lc_0289_game_of_life
// Bench: cargo test --example lc_0289_game_of_life bench -- --nocapture

struct Solution;

/// My solution — In-place marking with two extra states (doocs).
///
/// Use state 2 = was-live-now-dead, state -1 = was-dead-now-live.
/// First pass: count live neighbors (>0 means live) and mark transitions.
/// Second pass: convert state 2 -> 0 and -1 -> 1.
///
/// Time:  O(m * n)
/// Space: O(1)
mod my_solution {
    use super::Solution;

    impl Solution {
        pub fn game_of_life_my(board: &mut Vec<Vec<i32>>) {
            let m = board.len();
            let n = board[0].len();

            for i in 0..m {
                for j in 0..n {
                    // Count live neighbors (cells with value > 0 are currently alive)
                    let mut live = -board[i][j]; // subtract self if alive
                    for x in i.saturating_sub(1)..=(i + 1).min(m - 1) {
                        for y in j.saturating_sub(1)..=(j + 1).min(n - 1) {
                            if board[x][y] > 0 {
                                live += 1;
                            }
                        }
                    }
                    // Mark transitions
                    if board[i][j] == 1 && (live < 2 || live > 3) {
                        board[i][j] = 2; // was alive, now dies
                    }
                    if board[i][j] == 0 && live == 3 {
                        board[i][j] = -1; // was dead, now lives
                    }
                }
            }

            // Second pass: finalize states
            for row in board.iter_mut() {
                for cell in row.iter_mut() {
                    if *cell == 2 {
                        *cell = 0;
                    } else if *cell == -1 {
                        *cell = 1;
                    }
                }
            }
        }
    }
}

/// AI solution — Bit-packing approach.
///
/// Store the next state in bit 1 of each cell. Original state is in bit 0.
/// First pass: count neighbors via `cell & 1`, set bit 1 if next state is live.
/// Second pass: shift all cells right by 1 to get the new state.
///
/// Time:  O(m * n)
/// Space: O(1)
mod ai_solution {
    use super::Solution;

    impl Solution {
        pub fn game_of_life_ai(board: &mut Vec<Vec<i32>>) {
            let m = board.len();
            let n = board[0].len();

            for i in 0..m {
                for j in 0..n {
                    let mut live = 0i32;
                    for x in i.saturating_sub(1)..=(i + 1).min(m - 1) {
                        for y in j.saturating_sub(1)..=(j + 1).min(n - 1) {
                            if x == i && y == j {
                                continue; // skip self
                            }
                            live += board[x][y] & 1; // original state in bit 0
                        }
                    }
                    // Determine next state and store in bit 1
                    let currently_alive = board[i][j] & 1;
                    if currently_alive == 1 && (live == 2 || live == 3) {
                        board[i][j] |= 2; // set bit 1 (survives)
                    } else if currently_alive == 0 && live == 3 {
                        board[i][j] |= 2; // set bit 1 (reproduction)
                    }
                }
            }

            // Second pass: shift right to get new state
            for row in board.iter_mut() {
                for cell in row.iter_mut() {
                    *cell >>= 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_gol {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let mut board1 = $input;
                let mut board2 = board1.clone();
                Solution::game_of_life_my(&mut board1);
                assert_eq!(board1, $expected, "my_solution failed");
                Solution::game_of_life_ai(&mut board2);
                assert_eq!(board2, $expected, "ai_solution failed");
            }
        };
    }

    test_gol!(
        test_example_1,
        vec![
            vec![0, 1, 0],
            vec![0, 0, 1],
            vec![1, 1, 1],
            vec![0, 0, 0],
        ],
        vec![
            vec![0, 0, 0],
            vec![1, 0, 1],
            vec![0, 1, 1],
            vec![0, 1, 0],
        ]
    );

    test_gol!(
        test_example_2,
        vec![vec![1, 1], vec![1, 0]],
        vec![vec![1, 1], vec![1, 1]]
    );

    // All dead
    test_gol!(
        test_all_dead,
        vec![vec![0, 0], vec![0, 0]],
        vec![vec![0, 0], vec![0, 0]]
    );

    // All alive 2x2 -> all alive (each has 3 neighbors)
    test_gol!(
        test_all_alive_2x2,
        vec![vec![1, 1], vec![1, 1]],
        vec![vec![1, 1], vec![1, 1]]
    );

    // Blinker oscillator: horizontal -> vertical
    test_gol!(
        test_blinker,
        vec![vec![0, 0, 0], vec![1, 1, 1], vec![0, 0, 0]],
        vec![vec![0, 1, 0], vec![0, 1, 0], vec![0, 1, 0]]
    );

    // Single cell dies (under-population)
    test_gol!(
        test_single_cell,
        vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]],
        vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
    );

    // Block (still life) — 2x2 block in center stays
    test_gol!(
        test_block_still_life,
        vec![
            vec![0, 0, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0],
        ],
        vec![
            vec![0, 0, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0],
        ]
    );

    // Single row
    test_gol!(
        test_single_row,
        vec![vec![0, 1, 1, 1, 0]],
        vec![vec![0, 0, 1, 0, 0]]
    );

    // Single column
    test_gol!(
        test_single_col,
        vec![vec![0], vec![1], vec![1], vec![1], vec![0]],
        vec![vec![0], vec![0], vec![1], vec![0], vec![0]]
    );

    // Over-population: center cell with 4+ neighbors
    test_gol!(
        test_overpopulation,
        vec![
            vec![1, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 1],
        ],
        vec![
            vec![1, 0, 1],
            vec![0, 0, 0],
            vec![1, 0, 1],
        ]
    );

    // 1x1 grid
    test_gol!(test_1x1_alive, vec![vec![1]], vec![vec![0]]);
    test_gol!(test_1x1_dead, vec![vec![0]], vec![vec![0]]);
}

fn main() {
    println!("LeetCode 289: Game of Life");
    let mut board = vec![
        vec![0, 1, 0],
        vec![0, 0, 1],
        vec![1, 1, 1],
        vec![0, 0, 0],
    ];
    println!("Before: {:?}", board);
    Solution::game_of_life_my(&mut board);
    println!("After: {:?}", board);
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        // 25x25 grid with checkerboard pattern
        let base: Vec<Vec<i32>> = (0..25)
            .map(|i| (0..25).map(|j| ((i + j) % 2) as i32).collect())
            .collect();
        let iterations = 100_000;

        // Warm up
        for _ in 0..1_000 {
            let mut b = base.clone();
            std::hint::black_box(Solution::game_of_life_my(&mut b));
            let mut b = base.clone();
            std::hint::black_box(Solution::game_of_life_ai(&mut b));
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut b = base.clone();
            std::hint::black_box(Solution::game_of_life_my(&mut b));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut b = base.clone();
            std::hint::black_box(Solution::game_of_life_ai(&mut b));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ratio = if my_avg < ai_avg {
            format!("My {:.2}x faster", ai_avg / my_avg)
        } else {
            format!("AI {:.2}x faster", my_avg / ai_avg)
        };

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution avg: {my_avg:.3}us");
        println!("AI solution avg: {ai_avg:.3}us");
        println!("Ratio: {ratio}");
    }
}
