// LeetCode 463: Island Perimeter [Easy]
// https://leetcode.com/problems/island-perimeter/
//
// You are given row x col grid where grid[i][j] = 1 represents land and
// grid[i][j] = 0 represents water. Cells are connected horizontally/vertically.
// The grid is completely surrounded by water, and there is exactly one island.
// Determine the perimeter of the island.
//
// Examples:
//   [[0,1,0,0],[1,1,1,0],[0,1,0,0],[1,1,0,0]] -> 16
//   [[1]]                                       -> 4
//   [[1,0]]                                     -> 4
//
// Constraints:
//   1 <= row, col <= 100
//   grid[i][j] is 0 or 1
//   There is exactly one island
//
// Run:  cargo run --example lc_0463_island_perimeter
// Test: cargo test --example lc_0463_island_perimeter
// Bench: cargo test --example lc_0463_island_perimeter bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 228ns
//   AI solution: avg 227ns
//   Ratio: 1.00x (same algorithm)

/// My solution module — count edges.
///
/// Time:  O(m * n)
/// Space: O(1)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
            let (rows, cols) = (grid.len(), grid[0].len());
            let mut perimeter = 0;
            for r in 0..rows {
                for c in 0..cols {
                    if grid[r][c] == 1 {
                        perimeter += 4;
                        if r > 0 && grid[r - 1][c] == 1 {
                            perimeter -= 2;
                        }
                        if c > 0 && grid[r][c - 1] == 1 {
                            perimeter -= 2;
                        }
                    }
                }
            }
            perimeter
        }
    }
}

/// AI solution module — for each land cell, start with 4 edges and subtract
/// shared edges with adjacent land cells.
///
/// Time:  O(m * n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
            let rows = grid.len();
            let cols = grid[0].len();
            let mut perimeter = 0;
            for r in 0..rows {
                for c in 0..cols {
                    if grid[r][c] == 1 {
                        // Each land cell contributes 4 edges minus shared edges
                        perimeter += 4;
                        // Subtract 2 for each adjacent land cell (shared edge counted twice)
                        if r > 0 && grid[r - 1][c] == 1 {
                            perimeter -= 2;
                        }
                        if c > 0 && grid[r][c - 1] == 1 {
                            perimeter -= 2;
                        }
                    }
                }
            }
            perimeter
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_example1() {
            let grid = vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0],
            ];
            assert_eq!(Solution::island_perimeter(grid), 16);
        }
        #[test]
        fn test_single() {
            assert_eq!(Solution::island_perimeter(vec![vec![1]]), 4);
        }
        #[test]
        fn test_row() {
            assert_eq!(Solution::island_perimeter(vec![vec![1, 0]]), 4);
        }
    };
}

#[cfg(test)]
mod tests_my {
    solution_tests!(my_solution);
}
#[cfg(test)]
mod tests_ai {
    solution_tests!(ai_solution);
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(Vec<Vec<i32>>) -> i32,
{
    let iterations = 500_000;
    let grid = vec![
        vec![0, 1, 0, 0],
        vec![1, 1, 1, 0],
        vec![0, 1, 0, 0],
        vec![1, 1, 0, 0],
    ];
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(grid.clone())));
    }
    let elapsed = start.elapsed();
    println!(
        "{}: {} iterations in {:?} (avg {:?})",
        name,
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::island_perimeter);
    bench_solution("AI solution", ai_solution::Solution::island_perimeter);
}

fn main() {
    let grid = vec![
        vec![0, 1, 0, 0],
        vec![1, 1, 1, 0],
        vec![0, 1, 0, 0],
        vec![1, 1, 0, 0],
    ];
    println!("{}", ai_solution::Solution::island_perimeter(grid));
}
