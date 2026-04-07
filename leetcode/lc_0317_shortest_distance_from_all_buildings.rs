// LeetCode 317. Shortest Distance from All Buildings (Premium)
// https://leetcode.com/problems/shortest-distance-from-all-buildings/
//
// Benchmark results (100_000 iterations, release mode, 2025-01-XX):
//   my_solution  avg: 1.804us
//   ai_solution  avg: 1.126us
//   Ratio: ai_solution 1.60x faster

/// Given an m x n grid (0=empty, 1=building, 2=obstacle).
/// Build a house on empty land reaching ALL buildings with shortest total distance.
/// Return -1 if impossible.
///
/// Example 1:
///   Input: grid = [[1,0,2,0,1],[0,0,0,0,0],[0,0,1,0,0]]
///   Output: 7  (at (1,2): 3+3+1=7)
///
/// Example 2:
///   Input: grid = [[1,0]]
///   Output: 1
///
/// Example 3:
///   Input: grid = [[1]]
///   Output: -1
///
/// Constraints:
///   - 1 <= m, n <= 50
///   - grid[i][j] is 0, 1, or 2

/// # my_solution: Multi-source BFS from each building
/// Time: O(B * m * n) where B = number of buildings
/// Space: O(m * n) for dist and cnt arrays
mod my_solution {
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// Find shortest total distance to all buildings.
        ///
        /// BFS from each building, accumulating distances to empty cells.
        /// Track how many buildings can reach each cell; valid candidates
        /// must be reachable from all buildings.
        ///
        /// # Arguments
        /// * `grid` - 2D grid of 0/1/2
        ///
        /// # Returns
        /// Minimum total distance, or -1 if impossible
        pub fn shortest_distance(grid: Vec<Vec<i32>>) -> i32 {
            let m = grid.len();
            let n = grid[0].len();
            let mut total = 0; // total number of buildings
            let mut dist = vec![vec![0i32; n]; m];
            let mut cnt = vec![vec![0i32; n]; m];
            let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == 1 {
                        total += 1;
                        let mut vis = vec![vec![false; n]; m];
                        let mut queue = VecDeque::new();
                        queue.push_back((i, j));
                        vis[i][j] = true;
                        let mut d = 0i32;

                        while !queue.is_empty() {
                            d += 1;
                            // Process one BFS level at a time
                            for _ in 0..queue.len() {
                                let (r, c) = queue.pop_front().unwrap();
                                for &(dr, dc) in &dirs {
                                    let nr = r as i32 + dr;
                                    let nc = c as i32 + dc;
                                    if nr >= 0
                                        && nr < m as i32
                                        && nc >= 0
                                        && nc < n as i32
                                    {
                                        let (x, y) =
                                            (nr as usize, nc as usize);
                                        if !vis[x][y] && grid[x][y] == 0 {
                                            vis[x][y] = true;
                                            cnt[x][y] += 1;
                                            dist[x][y] += d;
                                            queue.push_back((x, y));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let mut ans = i32::MAX;
            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == 0 && cnt[i][j] == total {
                        ans = ans.min(dist[i][j]);
                    }
                }
            }

            if ans == i32::MAX { -1 } else { ans }
        }
    }
}

/// # ai_solution: BFS with early termination via hit-count
/// Time: O(B * m * n) where B = number of buildings
/// Space: O(m * n)
mod ai_solution {
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// Find shortest total distance using BFS with hit-count optimization.
        ///
        /// Same multi-source BFS, but uses a single "expected hit count" value
        /// that decreases each round. Only cells matching the expected count
        /// from the previous building BFS are eligible — this efficiently
        /// prunes unreachable cells without a separate visited array.
        ///
        /// # Arguments
        /// * `grid` - 2D grid of 0/1/2
        ///
        /// # Returns
        /// Minimum total distance, or -1 if impossible
        pub fn shortest_distance(grid: Vec<Vec<i32>>) -> i32 {
            let m = grid.len();
            let n = grid[0].len();
            let mut dist = vec![vec![0i32; n]; m];
            // hit[i][j] counts how many buildings have reached cell (i,j)
            let mut hit = vec![vec![0i32; n]; m];
            let mut building_count = 0i32;
            let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == 1 {
                        let mut queue = VecDeque::new();
                        queue.push_back((i, j));
                        let mut d = 0i32;
                        let mut found = false;

                        while !queue.is_empty() {
                            d += 1;
                            for _ in 0..queue.len() {
                                let (r, c) = queue.pop_front().unwrap();
                                for &(dr, dc) in &dirs {
                                    let nr = r as i32 + dr;
                                    let nc = c as i32 + dc;
                                    if nr >= 0
                                        && nr < m as i32
                                        && nc >= 0
                                        && nc < n as i32
                                    {
                                        let (x, y) =
                                            (nr as usize, nc as usize);
                                        // Only visit empty cells hit by
                                        // all previous buildings
                                        if grid[x][y] == 0
                                            && hit[x][y] == building_count
                                        {
                                            hit[x][y] += 1;
                                            dist[x][y] += d;
                                            queue.push_back((x, y));
                                            found = true;
                                        }
                                    }
                                }
                            }
                        }

                        building_count += 1;
                        if !found && building_count > 1 {
                            // This building can't reach any empty cell
                            // that was reachable by all prior buildings
                            return -1;
                        }
                    }
                }
            }

            let mut ans = i32::MAX;
            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == 0 && hit[i][j] == building_count {
                        ans = ans.min(dist[i][j]);
                    }
                }
            }

            if ans == i32::MAX { -1 } else { ans }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::Solution as AiSolution;
    use super::my_solution::Solution as MySolution;

    fn run_tests(solve: fn(Vec<Vec<i32>>) -> i32) {
        // Example 1
        assert_eq!(
            solve(vec![
                vec![1, 0, 2, 0, 1],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0],
            ]),
            7
        );

        // Example 2
        assert_eq!(solve(vec![vec![1, 0]]), 1);

        // Example 3: single building, no empty land
        assert_eq!(solve(vec![vec![1]]), -1);

        // No path possible: building surrounded by obstacles
        assert_eq!(
            solve(vec![vec![1, 2, 0], vec![2, 2, 0], vec![0, 0, 1]]),
            -1
        );

        // Two buildings with empty between
        assert_eq!(solve(vec![vec![1, 0, 0, 1]]), 3); // (0,1): 1+2=3, (0,2): 2+1=3

        // Single empty cell between two buildings
        assert_eq!(solve(vec![vec![1, 0, 1]]), 2);
    }

    #[test]
    fn test_my_solution() {
        run_tests(MySolution::shortest_distance);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(AiSolution::shortest_distance);
    }
}

fn main() {
    use std::hint::black_box;
    use std::time::Instant;

    let grid = vec![
        vec![1, 0, 2, 0, 1],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0],
    ];
    println!("Input: grid = {:?}", grid);
    println!(
        "Output (my): {}",
        my_solution::Solution::shortest_distance(grid.clone())
    );
    println!(
        "Output (ai): {}",
        ai_solution::Solution::shortest_distance(grid.clone())
    );

    let iterations = 100_000;
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(my_solution::Solution::shortest_distance(
            black_box(grid.clone()),
        ));
    }
    let my_dur = start.elapsed();

    let start = Instant::now();
    for _ in 0..iterations {
        black_box(ai_solution::Solution::shortest_distance(
            black_box(grid.clone()),
        ));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    println!("\nBenchmark ({iterations} iterations, release mode):");
    println!("  my_solution  avg: {my_avg:.3}us");
    println!("  ai_solution  avg: {ai_avg:.3}us");
    if my_avg < ai_avg {
        println!("  my_solution is {:.2}x faster", ai_avg / my_avg);
    } else {
        println!("  ai_solution is {:.2}x faster", my_avg / ai_avg);
    }
}
