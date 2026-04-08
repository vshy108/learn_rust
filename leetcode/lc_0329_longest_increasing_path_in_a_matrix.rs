// @lc app=leetcode id=329 lang=rust
// #[Hard] - Longest Increasing Path in a Matrix
// Iterations: 100000 | Date: 2025-04-08
// my_solution  | avg: 0.234us
// ai_solution  | avg: 0.332us
// Performance: My 1.42x faster

/// # 329. Longest Increasing Path in a Matrix
///
/// Given an m x n matrix, return the length of the longest increasing path.
/// From each cell, move in 4 directions (no diagonal, no wrap-around).
///
/// Constraints:
/// - m == matrix.length, n == matrix[0].length
/// - 1 <= m, n <= 200
/// - 0 <= matrix[i][j] <= 2^31 - 1

/// ## My Solution — DFS + Memoization
///
/// For each cell, DFS to all 4 neighbors that have strictly greater values.
/// Cache results in a 2D memo array. No visited set needed because the
/// strictly-increasing constraint prevents cycles.
///
/// - Time: O(m * n) — each cell computed once
/// - Space: O(m * n)
mod my_solution {
    pub fn longest_increasing_path(matrix: &[Vec<i32>]) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        // memo[i][j] = 0 means not computed yet
        let mut memo = vec![vec![0i32; n]; m];
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                ans = ans.max(dfs(matrix, &mut memo, i, j, m, n));
            }
        }
        ans
    }

    fn dfs(
        matrix: &[Vec<i32>],
        memo: &mut [Vec<i32>],
        i: usize,
        j: usize,
        m: usize,
        n: usize,
    ) -> i32 {
        if memo[i][j] != 0 {
            return memo[i][j];
        }
        let mut best = 1;
        // 4 directions encoded as (di, dj) pairs via dirs array
        const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for &(di, dj) in &DIRS {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                let (ni, nj) = (ni as usize, nj as usize);
                if matrix[ni][nj] > matrix[i][j] {
                    best = best.max(1 + dfs(matrix, memo, ni, nj, m, n));
                }
            }
        }
        memo[i][j] = best;
        best
    }
}

/// ## AI Solution — Topological sort (Kahn's) on increasing edges
///
/// Build in-degree for each cell based on strictly-less neighbors.
/// Cells with in-degree 0 are starting points. BFS layer-by-layer;
/// the number of layers = longest increasing path.
///
/// - Time: O(m * n)
/// - Space: O(m * n)
mod ai_solution {
    use std::collections::VecDeque;

    pub fn longest_increasing_path(matrix: &[Vec<i32>]) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut indegree = vec![vec![0u16; n]; m];
        const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        // Compute in-degree: for each cell, count neighbors with smaller value
        // (those neighbors have an edge into this cell)
        for i in 0..m {
            for j in 0..n {
                for &(di, dj) in &DIRS {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                        if matrix[ni as usize][nj as usize] < matrix[i][j] {
                            indegree[i][j] += 1;
                        }
                    }
                }
            }
        }

        // Start BFS from all cells with in-degree 0 (local minima)
        let mut queue = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if indegree[i][j] == 0 {
                    queue.push_back((i, j));
                }
            }
        }

        let mut layers = 0;
        while !queue.is_empty() {
            layers += 1;
            for _ in 0..queue.len() {
                let (i, j) = queue.pop_front().expect("queue should not be empty");
                for &(di, dj) in &DIRS {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                        let (ni, nj) = (ni as usize, nj as usize);
                        if matrix[ni][nj] > matrix[i][j] {
                            indegree[ni][nj] -= 1;
                            if indegree[ni][nj] == 0 {
                                queue.push_back((ni, nj));
                            }
                        }
                    }
                }
            }
        }
        layers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&[Vec<i32>]) -> i32) {
        // Example 1: longest path [1,2,6,9] length 4
        assert_eq!(f(&[vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]), 4);
        // Example 2: longest path [3,4,5,6] length 4
        assert_eq!(f(&[vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]]), 4);
        // Single cell
        assert_eq!(f(&[vec![1]]), 1);
        // Uniform — no increasing path beyond length 1
        assert_eq!(f(&[vec![7, 7], vec![7, 7]]), 1);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::longest_increasing_path);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::longest_increasing_path);
    }
}

fn main() {
    let matrix = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
    let iterations = 100_000;

    // Warm up
    for _ in 0..1000 {
        std::hint::black_box(my_solution::longest_increasing_path(std::hint::black_box(
            &matrix,
        )));
    }

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::longest_increasing_path(std::hint::black_box(
            &matrix,
        )));
    }
    let my_dur = start.elapsed();

    // Warm up
    for _ in 0..1000 {
        std::hint::black_box(ai_solution::longest_increasing_path(std::hint::black_box(
            &matrix,
        )));
    }

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::longest_increasing_path(std::hint::black_box(
            &matrix,
        )));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 329 - Longest Increasing Path in a Matrix");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
