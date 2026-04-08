// LeetCode 490: The Maze [Medium]
// https://leetcode.com/problems/the-maze/
//
// There is a ball in a maze with empty spaces (0) and walls (1). The ball
// can go through empty spaces by rolling up, down, left or right, but it
// won't stop rolling until hitting a wall. When the ball stops, it can
// choose the next direction.
//
// Given the maze, the ball's start position, and the destination, determine
// whether the ball can stop at the destination.
//
// Examples:
//   maze = [[0,0,1,0,0],
//           [0,0,0,0,0],
//           [0,0,0,1,0],
//           [1,1,0,1,1],
//           [0,0,0,0,0]]
//   start = [0,4], dest = [4,4]  -> true
//   start = [0,4], dest = [3,2]  -> false
//
// Constraints:
//   1 <= m, n <= 100
//   maze[i][j] is 0 or 1
//   start and destination are empty spaces, and won't be the same position
//
// Run:  cargo run --example lc_0490_the_maze
// Test: cargo test --example lc_0490_the_maze
// Bench: cargo test --example lc_0490_the_maze bench -- --nocapture
//
// First benchmark (100K iterations, 2026-04-08):
//   My solution: avg 11.791us
//   AI solution: avg 11.341us
//   Ratio: 1.04x (DFS vs BFS similar)

/// Time: O(m*n), Space: O(m*n)
/// DFS. From each stop position, try all 4 directions. Roll the ball in
/// that direction until hitting a wall or boundary, then the ball stops
/// one cell back. If the stop position hasn't been visited, recurse.
#[allow(dead_code)]
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Determine if the ball can stop at the destination.
        ///
        /// # Arguments
        /// * `maze` - 2D grid where 0 = empty, 1 = wall
        /// * `start` - Starting position [row, col]
        /// * `destination` - Target position [row, col]
        ///
        /// # Returns
        /// `true` if the ball can stop at destination
        pub fn has_path(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> bool {
            let rows = maze.len();
            let cols = maze[0].len();
            let mut visited = vec![vec![false; cols]; rows];
            let dest = (destination[0] as usize, destination[1] as usize);
            Self::dfs(
                &maze,
                start[0] as usize,
                start[1] as usize,
                dest,
                &mut visited,
            )
        }

        /// Recursive DFS from position (r, c).
        ///
        /// Marks (r, c) as visited, then tries rolling in each of the
        /// 4 cardinal directions. The ball rolls until it hits a wall
        /// or boundary, stopping one cell before the obstacle.
        fn dfs(
            maze: &[Vec<i32>],
            r: usize,
            c: usize,
            dest: (usize, usize),
            visited: &mut [Vec<bool>],
        ) -> bool {
            if (r, c) == dest {
                return true;
            }
            visited[r][c] = true;

            let rows = maze.len() as i32;
            let cols = maze[0].len() as i32;
            // Directions: up, down, left, right
            let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

            for (dr, dc) in &dirs {
                // Roll until hitting a wall or boundary
                let mut nr = r as i32 + dr;
                let mut nc = c as i32 + dc;
                while nr >= 0
                    && nr < rows
                    && nc >= 0
                    && nc < cols
                    && maze[nr as usize][nc as usize] == 0
                {
                    nr += dr;
                    nc += dc;
                }
                // Step back to last valid (empty) cell
                let stop_r = (nr - dr) as usize;
                let stop_c = (nc - dc) as usize;

                if !visited[stop_r][stop_c] && Self::dfs(maze, stop_r, stop_c, dest, visited) {
                    return true;
                }
            }
            false
        }
    }
}

/// Time: O(m*n), Space: O(m*n)
/// BFS. Queue-based iterative approach with the same rolling logic.
/// Enqueue each new stop position; mark visited when enqueued so we
/// don't revisit.
#[allow(dead_code)]
mod ai_solution {
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// Determine if the ball can stop at the destination.
        ///
        /// # Arguments
        /// * `maze` - 2D grid where 0 = empty, 1 = wall
        /// * `start` - Starting position [row, col]
        /// * `destination` - Target position [row, col]
        ///
        /// # Returns
        /// `true` if the ball can stop at destination
        pub fn has_path(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> bool {
            let rows = maze.len();
            let cols = maze[0].len();
            let dest = (destination[0] as usize, destination[1] as usize);
            let sr = start[0] as usize;
            let sc = start[1] as usize;

            let mut visited = vec![vec![false; cols]; rows];
            visited[sr][sc] = true;

            let mut queue = VecDeque::new();
            queue.push_back((sr, sc));

            let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let rows_i = rows as i32;
            let cols_i = cols as i32;

            while let Some((r, c)) = queue.pop_front() {
                if (r, c) == dest {
                    return true;
                }

                for (dr, dc) in &dirs {
                    // Roll until hitting a wall or boundary
                    let mut nr = r as i32 + dr;
                    let mut nc = c as i32 + dc;
                    while nr >= 0
                        && nr < rows_i
                        && nc >= 0
                        && nc < cols_i
                        && maze[nr as usize][nc as usize] == 0
                    {
                        nr += dr;
                        nc += dc;
                    }
                    // Step back to last valid cell
                    let stop_r = (nr - dr) as usize;
                    let stop_c = (nc - dc) as usize;

                    if !visited[stop_r][stop_c] {
                        visited[stop_r][stop_c] = true;
                        queue.push_back((stop_r, stop_c));
                    }
                }
            }
            false
        }
    }
}

fn main() {
    println!("LeetCode 490: The Maze");
    let maze = vec![
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0],
        vec![1, 1, 0, 1, 1],
        vec![0, 0, 0, 0, 0],
    ];
    let cases: Vec<(Vec<i32>, Vec<i32>, bool)> = vec![
        (vec![0, 4], vec![4, 4], true),
        (vec![0, 4], vec![3, 2], false),
    ];
    for (start, dest, expected) in &cases {
        let r1 = my_solution::Solution::has_path(maze.clone(), start.clone(), dest.clone());
        let r2 = ai_solution::Solution::has_path(maze.clone(), start.clone(), dest.clone());
        println!("  start={start:?}, dest={dest:?} => my={r1}, ai={r2} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_both {
        ($name:ident, $maze:expr, $start:expr, $dest:expr, $expected:expr) => {
            mod $name {
                use super::*;
                #[test]
                fn my() {
                    assert_eq!(
                        my_solution::Solution::has_path($maze, $start, $dest),
                        $expected
                    );
                }
                #[test]
                fn ai() {
                    assert_eq!(
                        ai_solution::Solution::has_path($maze, $start, $dest),
                        $expected
                    );
                }
            }
        };
    }

    fn default_maze() -> Vec<Vec<i32>> {
        vec![
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0],
            vec![1, 1, 0, 1, 1],
            vec![0, 0, 0, 0, 0],
        ]
    }

    test_both!(
        example1_reachable,
        default_maze(),
        vec![0, 4],
        vec![4, 4],
        true
    );
    test_both!(
        example2_unreachable,
        default_maze(),
        vec![0, 4],
        vec![3, 2],
        false
    );
    test_both!(
        straight_line,
        vec![vec![0, 0, 0, 0, 0]],
        vec![0, 0],
        vec![0, 4],
        true
    );
    test_both!(
        straight_line_blocked_mid,
        vec![vec![0, 0, 1, 0, 0]],
        vec![0, 0],
        vec![0, 4],
        false
    );
    test_both!(
        column_roll,
        vec![vec![0], vec![0], vec![0], vec![0]],
        vec![0, 0],
        vec![3, 0],
        true
    );
    test_both!(
        cannot_stop_midway,
        vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
        vec![0, 0],
        vec![1, 0],
        false
    );
    test_both!(
        corner_bounce,
        vec![vec![0, 0, 0, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]],
        vec![0, 0],
        vec![2, 3],
        true
    );
    test_both!(
        large_empty,
        vec![vec![0; 5]; 5],
        vec![0, 0],
        vec![4, 4],
        true
    );
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(Vec<Vec<i32>>, Vec<i32>, Vec<i32>) -> bool,
{
    let iterations = 100_000;
    // Build a 100x100 maze with scattered walls
    let size = 100;
    let mut maze = vec![vec![0i32; size]; size];
    for i in (0..size).step_by(3) {
        for j in (1..size - 1).step_by(4) {
            maze[i][j] = 1;
        }
    }
    // Ensure start and dest are open
    maze[0][0] = 0;
    maze[size - 1][size - 1] = 0;

    let start = vec![0, 0];
    let dest = vec![(size - 1) as i32, (size - 1) as i32];

    let start_time = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(
            std::hint::black_box(maze.clone()),
            std::hint::black_box(start.clone()),
            std::hint::black_box(dest.clone()),
        ));
    }
    let elapsed = start_time.elapsed();
    println!(
        "{name}: {iterations} iterations in {elapsed:?} (avg {:?})",
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::has_path);
    bench_solution("AI solution", ai_solution::Solution::has_path);
}
