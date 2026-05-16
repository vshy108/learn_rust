// LeetCode 200: Number of Islands [Medium]
// https://leetcode.com/problems/number-of-islands/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 2.902us
//   AI solution avg: 5.091us
//   Ratio: My 1.75x faster
//
// Given a 2D grid of '1's (land) and '0's (water), count the number of
// islands. An island is surrounded by water and formed by connecting
// adjacent lands horizontally or vertically.
//
// Constraints: m,n in [1,300], grid[i][j] is '0' or '1'
//
// Run:  cargo run --example lc_0200_number_of_islands
// Test: cargo test --example lc_0200_number_of_islands
// Bench: cargo test --example lc_0200_number_of_islands bench -- --nocapture

/// My solution — DFS flood-fill: when a '1' cell is found, increment
/// the count and recursively mark all connected '1' cells as '0'.
///
/// Time: O(m * n) — each cell visited at most once.
/// Space: O(m * n) for the recursion stack in the worst case.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the number of islands.
        pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
            let mut grid = grid;
            let m = grid.len();
            let n = grid[0].len();
            let mut count = 0;

            fn dfs(grid: &mut Vec<Vec<char>>, r: usize, c: usize, m: usize, n: usize) {
                if r >= m || c >= n || grid[r][c] == '0' {
                    return;
                }
                grid[r][c] = '0';
                if r > 0 {
                    dfs(grid, r - 1, c, m, n);
                }
                if r + 1 < m {
                    dfs(grid, r + 1, c, m, n);
                }
                if c > 0 {
                    dfs(grid, r, c - 1, m, n);
                }
                if c + 1 < n {
                    dfs(grid, r, c + 1, m, n);
                }
            }

            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == '1' {
                        count += 1;
                        dfs(&mut grid, i, j, m, n);
                    }
                }
            }
            count
        }
    }
}

/// AI solution — DFS flood-fill: iterate cells, when '1' found, DFS
/// to mark the entire island as '0', increment count.
///
/// Time: O(m*n).
/// Space: O(m*n) worst case for DFS stack.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Count islands using iterative DFS (avoids stack overflow).
        pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
            let mut grid = grid;
            let m = grid.len();
            let n = grid[0].len();
            let mut count = 0;

            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == '1' {
                        count += 1;
                        // Iterative DFS to sink the island
                        let mut stack = vec![(i, j)];
                        grid[i][j] = '0';
                        while let Some((r, c)) = stack.pop() {
                            for (dr, dc) in [(0, 1), (0, -1_i32), (1, 0), (-1_i32, 0)] {
                                let nr = r as i32 + dr;
                                let nc = c as i32 + dc;
                                if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                                    let (nr, nc) = (nr as usize, nc as usize);
                                    if grid[nr][nc] == '1' {
                                        grid[nr][nc] = '0';
                                        stack.push((nr, nc));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            count
        }
    }
}

fn main() {
    let grid = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    let my = my_solution::Solution::num_islands(grid.clone());
    let ai = ai_solution::Solution::num_islands(grid);
    println!("islands: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_islands {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    let grid = vec![
                        vec!['1', '1', '1', '1', '0'],
                        vec!['1', '1', '0', '1', '0'],
                        vec!['1', '1', '0', '0', '0'],
                        vec!['0', '0', '0', '0', '0'],
                    ];
                    assert_eq!(<$solution>::num_islands(grid), 1);
                }

                #[test]
                fn test_example2() {
                    let grid = vec![
                        vec!['1', '1', '0', '0', '0'],
                        vec!['1', '1', '0', '0', '0'],
                        vec!['0', '0', '1', '0', '0'],
                        vec!['0', '0', '0', '1', '1'],
                    ];
                    assert_eq!(<$solution>::num_islands(grid), 3);
                }

                #[test]
                fn test_all_water() {
                    let grid = vec![vec!['0', '0'], vec!['0', '0']];
                    assert_eq!(<$solution>::num_islands(grid), 0);
                }

                #[test]
                fn test_all_land() {
                    let grid = vec![vec!['1', '1'], vec!['1', '1']];
                    assert_eq!(<$solution>::num_islands(grid), 1);
                }

                #[test]
                fn test_single_cell() {
                    assert_eq!(<$solution>::num_islands(vec![vec!['1']]), 1);
                }
            }
        };
    }

    test_islands!(my, crate::my_solution::Solution);
    test_islands!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let grid: Vec<Vec<char>> = (0..20)
            .map(|i| {
                (0..20)
                    .map(|j| if (i + j) % 3 == 0 { '1' } else { '0' })
                    .collect()
            })
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::num_islands(
                std::hint::black_box(grid.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::num_islands(
                std::hint::black_box(grid.clone()),
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
