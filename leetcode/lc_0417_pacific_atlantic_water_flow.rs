// LeetCode 417: Pacific Atlantic Water Flow [Medium]
// https://leetcode.com/problems/pacific-atlantic-water-flow/
// Benchmark: 10000 iterations
//   My solution (BFS from both borders): avg 159.34us
//   AI solution (DFS from both borders): avg 195.01us
//   -> My solution is 1.22x faster
//
// An m x n island borders Pacific (top/left) and Atlantic (right/bottom).
// Water flows to neighbors with height <= current height.
// Return all cells that can flow to both oceans.
//
// Examples:
//   [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
//     -> [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
//   [[1]] -> [[0,0]]
//
// Constraints:
//   m == heights.length, n == heights[r].length
//   1 <= m, n <= 200
//   0 <= heights[r][c] <= 10^5
//
// Run:  cargo run --example lc_0417_pacific_atlantic_water_flow
// Test: cargo test --example lc_0417_pacific_atlantic_water_flow
// Bench: cargo test --example lc_0417_pacific_atlantic_water_flow bench -- --nocapture

/// My solution module — BFS from both ocean boundaries.
///
/// Reverse the flow: BFS from Pacific border cells (top+left) marking
/// reachable cells going "uphill" (neighbor >= current). Do the same
/// from Atlantic border cells (bottom+right). Cells in both sets
/// are the answer.
///
/// Time:  O(m * n)
/// Space: O(m * n)
mod my_solution {
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// Find cells that can flow to both Pacific and Atlantic oceans.
        ///
        /// # Arguments
        ///
        /// * `heights` - m x n grid of cell heights
        ///
        /// # Returns
        ///
        /// List of [row, col] coordinates
        pub fn pacific_atlantic(
            heights: Vec<Vec<i32>>,
        ) -> Vec<Vec<i32>> {
            let m = heights.len();
            let n = heights[0].len();
            let mut pacific = vec![vec![false; n]; m];
            let mut atlantic = vec![vec![false; n]; m];
            let mut q_pac = VecDeque::new();
            let mut q_atl = VecDeque::new();

            // Seed Pacific: top row + left column
            for i in 0..m {
                pacific[i][0] = true;
                q_pac.push_back((i, 0));
            }
            for j in 1..n {
                pacific[0][j] = true;
                q_pac.push_back((0, j));
            }

            // Seed Atlantic: bottom row + right column
            for i in 0..m {
                atlantic[i][n - 1] = true;
                q_atl.push_back((i, n - 1));
            }
            for j in 0..n - 1 {
                atlantic[m - 1][j] = true;
                q_atl.push_back((m - 1, j));
            }

            Self::bfs(&heights, &mut q_pac, &mut pacific, m, n);
            Self::bfs(&heights, &mut q_atl, &mut atlantic, m, n);

            // Collect cells reachable from both oceans
            let mut result = Vec::new();
            for i in 0..m {
                for j in 0..n {
                    if pacific[i][j] && atlantic[i][j] {
                        result.push(vec![i as i32, j as i32]);
                    }
                }
            }
            result
        }

        /// BFS expanding to neighbors with height >= current.
        fn bfs(
            heights: &[Vec<i32>],
            queue: &mut VecDeque<(usize, usize)>,
            visited: &mut [Vec<bool>],
            m: usize,
            n: usize,
        ) {
            let dirs: [(i32, i32); 4] =
                [(-1, 0), (1, 0), (0, -1), (0, 1)];

            while let Some((r, c)) = queue.pop_front() {
                for (dr, dc) in &dirs {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr >= 0
                        && nr < m as i32
                        && nc >= 0
                        && nc < n as i32
                    {
                        let (nr, nc) = (nr as usize, nc as usize);
                        if !visited[nr][nc]
                            && heights[nr][nc] >= heights[r][c]
                        {
                            visited[nr][nc] = true;
                            queue.push_back((nr, nc));
                        }
                    }
                }
            }
        }
    }
}

/// AI solution module — DFS from both ocean boundaries.
///
/// Same idea as BFS but uses recursive DFS. For each ocean,
/// start DFS from the border cells going "uphill".
///
/// Time:  O(m * n)
/// Space: O(m * n) -- recursion stack in worst case
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find cells that can flow to both Pacific and Atlantic oceans.
        ///
        /// # Arguments
        ///
        /// * `heights` - m x n grid of cell heights
        ///
        /// # Returns
        ///
        /// List of [row, col] coordinates
        pub fn pacific_atlantic(
            heights: Vec<Vec<i32>>,
        ) -> Vec<Vec<i32>> {
            let m = heights.len();
            let n = heights[0].len();
            let mut pacific = vec![vec![false; n]; m];
            let mut atlantic = vec![vec![false; n]; m];

            // DFS from Pacific borders
            for i in 0..m {
                Self::dfs(&heights, &mut pacific, i, 0, m, n);
            }
            for j in 0..n {
                Self::dfs(&heights, &mut pacific, 0, j, m, n);
            }

            // DFS from Atlantic borders
            for i in 0..m {
                Self::dfs(&heights, &mut atlantic, i, n - 1, m, n);
            }
            for j in 0..n {
                Self::dfs(&heights, &mut atlantic, m - 1, j, m, n);
            }

            let mut result = Vec::new();
            for i in 0..m {
                for j in 0..n {
                    if pacific[i][j] && atlantic[i][j] {
                        result.push(vec![i as i32, j as i32]);
                    }
                }
            }
            result
        }

        /// DFS marking all reachable cells going uphill from (r, c).
        fn dfs(
            heights: &[Vec<i32>],
            visited: &mut [Vec<bool>],
            r: usize,
            c: usize,
            m: usize,
            n: usize,
        ) {
            if visited[r][c] {
                return;
            }
            visited[r][c] = true;

            let dirs: [(i32, i32); 4] =
                [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dr, dc) in &dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0
                    && nr < m as i32
                    && nc >= 0
                    && nc < n as i32
                {
                    let (nr, nc) = (nr as usize, nc as usize);
                    if heights[nr][nc] >= heights[r][c] {
                        Self::dfs(heights, visited, nr, nc, m, n);
                    }
                }
            }
        }
    }
}

fn main() {
    let cases: Vec<(Vec<Vec<i32>>, Vec<Vec<i32>>)> = vec![
        (
            vec![
                vec![1, 2, 2, 3, 5],
                vec![3, 2, 3, 4, 4],
                vec![2, 4, 5, 3, 1],
                vec![6, 7, 1, 4, 5],
                vec![5, 1, 1, 2, 4],
            ],
            vec![
                vec![0, 4], vec![1, 3], vec![1, 4],
                vec![2, 2], vec![3, 0], vec![3, 1],
                vec![4, 0],
            ],
        ),
        (vec![vec![1]], vec![vec![0, 0]]),
    ];

    println!("--- My Solution ---");
    for (heights, expected) in &cases {
        let result =
            my_solution::Solution::pacific_atlantic(heights.clone());
        println!("  result: {result:?}");
        println!("  expected: {expected:?}");
    }

    println!("--- AI Solution ---");
    for (heights, expected) in &cases {
        let result =
            ai_solution::Solution::pacific_atlantic(heights.clone());
        println!("  result: {result:?}");
        println!("  expected: {expected:?}");
    }
}

#[cfg(test)]
mod tests {
    /// Helper to sort results for comparison (order doesn't matter)
    fn sorted(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        v.sort();
        v
    }

    macro_rules! test_pacific {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;
                use super::sorted;

                #[test]
                fn test_example1() {
                    let heights = vec![
                        vec![1, 2, 2, 3, 5],
                        vec![3, 2, 3, 4, 4],
                        vec![2, 4, 5, 3, 1],
                        vec![6, 7, 1, 4, 5],
                        vec![5, 1, 1, 2, 4],
                    ];
                    let expected = vec![
                        vec![0, 4], vec![1, 3], vec![1, 4],
                        vec![2, 2], vec![3, 0], vec![3, 1],
                        vec![4, 0],
                    ];
                    assert_eq!(
                        sorted(Solution::pacific_atlantic(heights)),
                        sorted(expected),
                    );
                }

                #[test]
                fn test_single_cell() {
                    assert_eq!(
                        Solution::pacific_atlantic(vec![vec![1]]),
                        vec![vec![0, 0]],
                    );
                }

                #[test]
                fn test_all_same() {
                    // All cells at same height -> every cell flows to both
                    let heights = vec![
                        vec![1, 1, 1],
                        vec![1, 1, 1],
                        vec![1, 1, 1],
                    ];
                    let result = Solution::pacific_atlantic(heights);
                    assert_eq!(result.len(), 9);
                }

                #[test]
                fn test_single_row() {
                    let heights = vec![vec![1, 2, 3]];
                    // All cells touch both oceans in single row
                    let result = sorted(Solution::pacific_atlantic(heights));
                    let expected = sorted(vec![
                        vec![0, 0], vec![0, 1], vec![0, 2],
                    ]);
                    assert_eq!(result, expected);
                }
            }
        };
    }

    test_pacific!(my_tests, crate::my_solution::Solution);
    test_pacific!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 10_000_u32;

        // 50x50 grid with a valley in the center
        let m = 50_usize;
        let n = 50_usize;
        let mut heights = vec![vec![0_i32; n]; m];
        for (i, row) in heights.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                // Higher near edges, lower in center
                *cell = (i.min(m - 1 - i).min(j.min(n - 1 - j))) as i32;
            }
        }

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::pacific_atlantic(
                    std::hint::black_box(heights.clone()),
                ),
            );
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::pacific_atlantic(
                    std::hint::black_box(heights.clone()),
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
