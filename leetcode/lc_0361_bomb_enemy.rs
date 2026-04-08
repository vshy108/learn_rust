// @lc app=leetcode id=361 lang=rust
// #[Medium] - Bomb Enemy
// Iterations: 100000 | Date: 2025-07-10
// my_solution  | avg: 0.230us
// ai_solution  | avg: 0.075us
// Performance: AI 3.06x faster

/// # 361. Bomb Enemy
///
/// Given an m x n matrix where each cell is 'W' (wall), 'E' (enemy), or '0' (empty),
/// return the maximum enemies you can kill using one bomb placed in an empty cell.
/// The bomb kills all enemies in the same row and column until it hits a wall.
///
/// Constraints:
/// - m == grid.length, n == grid[i].length
/// - 1 <= m, n <= 500
/// - grid[i][j] is 'W', 'E', or '0'

/// ## My Solution — Prefix sweep from 4 directions O(m*n)
///
/// For each cell, accumulate enemy counts by scanning left-to-right, right-to-left,
/// top-to-bottom, bottom-to-top. Reset counter when hitting a wall.
///
/// - Time: O(m * n)
/// - Space: O(m * n)
mod my_solution {
    pub fn max_killed_enemies(grid: &[Vec<char>]) -> i32 {
        let m = grid.len();
        if m == 0 {
            return 0;
        }
        let n = grid[0].len();
        // g[i][j] accumulates enemy kills from all 4 directions
        let mut g = vec![vec![0i32; n]; m];

        // Horizontal sweeps (each row)
        for i in 0..m {
            // Left to right
            let mut t = 0;
            for j in 0..n {
                if grid[i][j] == 'W' {
                    t = 0;
                } else if grid[i][j] == 'E' {
                    t += 1;
                }
                g[i][j] += t;
            }
            // Right to left
            t = 0;
            for j in (0..n).rev() {
                if grid[i][j] == 'W' {
                    t = 0;
                } else if grid[i][j] == 'E' {
                    t += 1;
                }
                g[i][j] += t;
            }
        }

        // Vertical sweeps (each column)
        for j in 0..n {
            // Top to bottom
            let mut t = 0;
            for i in 0..m {
                if grid[i][j] == 'W' {
                    t = 0;
                } else if grid[i][j] == 'E' {
                    t += 1;
                }
                g[i][j] += t;
            }
            // Bottom to top
            t = 0;
            for i in (0..m).rev() {
                if grid[i][j] == 'W' {
                    t = 0;
                } else if grid[i][j] == 'E' {
                    t += 1;
                }
                g[i][j] += t;
            }
        }

        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '0' {
                    ans = ans.max(g[i][j]);
                }
            }
        }
        ans
    }
}

/// ## AI Solution — Row-cache + column-cache with lazy recompute
///
/// Only recompute row enemy count when at column 0 or after a wall;
/// only recompute column enemy count when in row 0 or after a wall above.
///
/// - Time: O(m * n)
/// - Space: O(n) for column caches
mod ai_solution {
    pub fn max_killed_enemies(grid: &[Vec<char>]) -> i32 {
        let m = grid.len();
        if m == 0 {
            return 0;
        }
        let n = grid[0].len();
        let mut ans = 0;
        let mut row_hits = 0;
        let mut col_hits = vec![0i32; n];

        for i in 0..m {
            for j in 0..n {
                // Recompute row_hits when starting a new segment
                if j == 0 || grid[i][j - 1] == 'W' {
                    row_hits = 0;
                    for k in j..n {
                        if grid[i][k] == 'W' {
                            break;
                        }
                        if grid[i][k] == 'E' {
                            row_hits += 1;
                        }
                    }
                }
                // Recompute col_hits[j] when starting a new segment
                if i == 0 || grid[i - 1][j] == 'W' {
                    col_hits[j] = 0;
                    for k in i..m {
                        if grid[k][j] == 'W' {
                            break;
                        }
                        if grid[k][j] == 'E' {
                            col_hits[j] += 1;
                        }
                    }
                }
                if grid[i][j] == '0' {
                    ans = ans.max(row_hits + col_hits[j]);
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&[Vec<char>]) -> i32) {
        let grid1 = vec![
            vec!['0', 'E', '0', '0'],
            vec!['E', '0', 'W', 'E'],
            vec!['0', 'E', '0', '0'],
        ];
        assert_eq!(f(&grid1), 3);

        let grid2 = vec![
            vec!['W', 'W', 'W'],
            vec!['0', '0', '0'],
            vec!['E', 'E', 'E'],
        ];
        assert_eq!(f(&grid2), 1);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::max_killed_enemies);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::max_killed_enemies);
    }
}

fn main() {
    let grid = vec![
        vec!['0', 'E', '0', '0'],
        vec!['E', '0', 'W', 'E'],
        vec!['0', 'E', '0', '0'],
    ];
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::max_killed_enemies(std::hint::black_box(&grid)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::max_killed_enemies(std::hint::black_box(&grid)));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::max_killed_enemies(std::hint::black_box(&grid)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::max_killed_enemies(std::hint::black_box(&grid)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 361 - Bomb Enemy");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
