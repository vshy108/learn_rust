// LeetCode 505: The Maze II [Medium] (Premium)
// https://leetcode.com/problems/the-maze-ii/
//
// Note: Premium problem -- description sourced from doocs/leetcode.
//
// There is a ball in a maze with empty spaces (0) and walls (1). The ball
// rolls in a direction until hitting a wall. Given the maze, start position,
// and destination, return the shortest distance for the ball to stop at the
// destination. If impossible, return -1.
//
// Distance = number of empty spaces traveled (start excluded, destination
// included).
//
// Examples:
//   maze = [[0,0,1,0,0],[0,0,0,0,0],[0,0,0,1,0],[1,1,0,1,1],[0,0,0,0,0]]
//   start = [0,4], destination = [4,4] -> 12
//   start = [0,4], destination = [3,2] -> -1
//
// Constraints:
//   m == maze.length, n == maze[i].length
//   1 <= m, n <= 100
//   maze[i][j] is 0 or 1
//   start.length == destination.length == 2
//   Both ball and destination are in empty spaces, not initially same position
//
// Run:  cargo run --example lc_0505_the_maze_ii
// Test: cargo test --example lc_0505_the_maze_ii
// Bench: cargo test --example lc_0505_the_maze_ii bench -- --nocapture
//
// First benchmark (100K iterations, 2026-04-08):
//   My solution: avg 972ns
//   AI solution: avg 1.03us
//   ~tied (ratio: 0.95)
//   Reason: BFS and Dijkstra are comparable on small grids; Dijkstra's heap
//   overhead offsets its early-exit advantage on this input size.

/// My solution — BFS with distance tracking.
///
/// Use a queue-based BFS. For each position, roll in all four directions
/// until hitting a wall. If we reach a position with a shorter distance,
/// update and enqueue. Translated from Java solution in doocs/leetcode.
///
/// Time:  O(m * n * max(m, n))
/// Space: O(m * n)
mod my_solution {
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// Return the shortest distance for the ball to stop at destination.
        ///
        /// # Arguments
        /// * `maze` - 2D grid (0 = empty, 1 = wall)
        /// * `start` - [row, col] of start position
        /// * `destination` - [row, col] of target stop position
        ///
        /// # Returns
        /// Shortest distance, or -1 if impossible
        pub fn shortest_distance(
            maze: Vec<Vec<i32>>,
            start: Vec<i32>,
            destination: Vec<i32>,
        ) -> i32 {
            let m = maze.len();
            let n = maze[0].len();
            let si = start[0] as usize;
            let sj = start[1] as usize;
            let di = destination[0] as usize;
            let dj = destination[1] as usize;

            let mut dist = vec![vec![i32::MAX; n]; m];
            dist[si][sj] = 0;

            let mut queue = VecDeque::new();
            queue.push_back((si, sj));

            let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

            while let Some((i, j)) = queue.pop_front() {
                for &(dr, dc) in &dirs {
                    let mut x = i as i32;
                    let mut y = j as i32;
                    let mut k = dist[i][j];

                    // Roll until hitting a wall
                    while x + dr >= 0
                        && x + dr < m as i32
                        && y + dc >= 0
                        && y + dc < n as i32
                        && maze[(x + dr) as usize][(y + dc) as usize] == 0
                    {
                        x += dr;
                        y += dc;
                        k += 1;
                    }

                    let (ux, uy) = (x as usize, y as usize);
                    if k < dist[ux][uy] {
                        dist[ux][uy] = k;
                        queue.push_back((ux, uy));
                    }
                }
            }

            if dist[di][dj] == i32::MAX {
                -1
            } else {
                dist[di][dj]
            }
        }
    }
}

/// AI solution — Dijkstra's algorithm with BinaryHeap.
///
/// Uses a min-heap (via Reverse) to always process the shortest-distance
/// position first, giving true shortest paths.
///
/// Time:  O(m * n * max(m, n) * log(m * n))
/// Space: O(m * n)
mod ai_solution {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    pub struct Solution;

    impl Solution {
        /// Return the shortest distance using Dijkstra's algorithm.
        ///
        /// # Arguments
        /// * `maze` - 2D grid (0 = empty, 1 = wall)
        /// * `start` - [row, col] of start position
        /// * `destination` - [row, col] of target stop position
        ///
        /// # Returns
        /// Shortest distance, or -1 if impossible
        pub fn shortest_distance(
            maze: Vec<Vec<i32>>,
            start: Vec<i32>,
            destination: Vec<i32>,
        ) -> i32 {
            let m = maze.len();
            let n = maze[0].len();
            let si = start[0] as usize;
            let sj = start[1] as usize;
            let di = destination[0] as usize;
            let dj = destination[1] as usize;

            let mut dist = vec![vec![i32::MAX; n]; m];
            dist[si][sj] = 0;

            // Min-heap: (distance, row, col)
            let mut heap = BinaryHeap::new();
            heap.push(Reverse((0i32, si, sj)));

            let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

            while let Some(Reverse((d, i, j))) = heap.pop() {
                // Skip stale entries
                if d > dist[i][j] {
                    continue;
                }
                // Early exit if we reached the destination
                if i == di && j == dj {
                    return d;
                }
                for &(dr, dc) in &dirs {
                    let mut x = i as i32;
                    let mut y = j as i32;
                    let mut k = d;

                    while x + dr >= 0
                        && x + dr < m as i32
                        && y + dc >= 0
                        && y + dc < n as i32
                        && maze[(x + dr) as usize][(y + dc) as usize] == 0
                    {
                        x += dr;
                        y += dc;
                        k += 1;
                    }

                    let (ux, uy) = (x as usize, y as usize);
                    if k < dist[ux][uy] {
                        dist[ux][uy] = k;
                        heap.push(Reverse((k, ux, uy)));
                    }
                }
            }

            -1
        }
    }
}

fn main() {
    let maze = vec![
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0],
        vec![1, 1, 0, 1, 1],
        vec![0, 0, 0, 0, 0],
    ];
    let my = my_solution::Solution::shortest_distance(maze.clone(), vec![0, 4], vec![4, 4]);
    let ai = ai_solution::Solution::shortest_distance(maze, vec![0, 4], vec![4, 4]);
    println!("my: {my}, ai: {ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_maze {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                fn maze1() -> Vec<Vec<i32>> {
                    vec![
                        vec![0, 0, 1, 0, 0],
                        vec![0, 0, 0, 0, 0],
                        vec![0, 0, 0, 1, 0],
                        vec![1, 1, 0, 1, 1],
                        vec![0, 0, 0, 0, 0],
                    ]
                }

                #[test]
                fn test_example1() {
                    assert_eq!(
                        <$solution>::shortest_distance(maze1(), vec![0, 4], vec![4, 4],),
                        12
                    );
                }

                #[test]
                fn test_example2_unreachable() {
                    assert_eq!(
                        <$solution>::shortest_distance(maze1(), vec![0, 4], vec![3, 2],),
                        -1
                    );
                }

                #[test]
                fn test_example3() {
                    let maze = vec![
                        vec![0, 0, 0, 0, 0],
                        vec![1, 1, 0, 0, 1],
                        vec![0, 0, 0, 0, 0],
                        vec![0, 1, 0, 0, 1],
                        vec![0, 1, 0, 0, 0],
                    ];
                    assert_eq!(
                        <$solution>::shortest_distance(maze, vec![4, 3], vec![0, 1],),
                        -1
                    );
                }

                #[test]
                fn test_simple_direct() {
                    let maze = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
                    // Ball rolls from (0,0) right -> stops at (0,2), dist=2
                    // Then rolls down -> stops at (2,2), dist=2+2=4
                    assert_eq!(
                        <$solution>::shortest_distance(maze, vec![0, 0], vec![2, 2],),
                        4
                    );
                }
            }
        };
    }

    test_maze!(my, crate::my_solution::Solution);
    test_maze!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let maze = vec![
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0],
            vec![1, 1, 0, 1, 1],
            vec![0, 0, 0, 0, 0],
        ];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::shortest_distance(
                std::hint::black_box(maze.clone()),
                std::hint::black_box(vec![0, 4]),
                std::hint::black_box(vec![4, 4]),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::shortest_distance(
                std::hint::black_box(maze.clone()),
                std::hint::black_box(vec![0, 4]),
                std::hint::black_box(vec![4, 4]),
            ));
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
            println!("~tied (ratio: {ratio:.2})");
        }
    }
}
