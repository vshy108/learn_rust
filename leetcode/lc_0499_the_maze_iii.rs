// LeetCode 499: The Maze III [Hard]
// https://leetcode.com/problems/the-maze-iii/
//
// There is a ball in a maze with empty spaces (0) and walls (1). The ball
// rolls in a chosen direction until hitting a wall. There is also a hole
// in the maze. If the ball rolls over the hole, it drops in. Given the
// ball position and hole position, find the shortest distance path for the
// ball to drop into the hole. If multiple shortest paths exist, return
// the lexicographically smallest instruction string. Return "impossible"
// if the ball cannot reach the hole.
//
// Instructions: 'd' (down), 'l' (left), 'r' (right), 'u' (up).
//
// Examples:
//   maze = [[0,0,0,0,0],[1,1,0,0,1],[0,0,0,0,0],[0,1,0,0,1],[0,1,0,0,0]]
//   ball = [4,3], hole = [0,1]  -> "lul"  (distance 6)
//
//   maze = [[0,0,0,0,0],[1,1,0,0,1],[0,0,0,0,0],[0,1,0,0,1],[0,1,0,0,0]]
//   ball = [4,3], hole = [3,0]  -> "impossible"
//
// Constraints:
//   1 <= m, n <= 100
//   maze[i][j] is 0 or 1
//   ball and hole are at empty positions and are different
//
// Run:  cargo run --example lc_0499_the_maze_iii
// Test: cargo test --example lc_0499_the_maze_iii
// Bench: cargo test --example lc_0499_the_maze_iii bench -- --nocapture
//
// First benchmark (50K iterations, 2026-04-08):
//   My solution: avg 14.955us
//   AI solution: avg 9.515us
//   Ratio: 1.57x (AI Dijkstra faster than BFS)

/// Time: O(m*n*(m+n)), Space: O(m*n)
/// BFS with distance+path tracking. For each stop position, roll in all
/// 4 directions. While rolling, check if the ball passes over the hole;
/// if so, stop at the hole. Update a neighbor if we found a shorter
/// distance, or the same distance but a lexicographically smaller path.
#[allow(dead_code)]
mod my_solution {
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// Find the shortest path instruction to drop the ball into the hole.
        ///
        /// # Arguments
        /// * `maze` - 2D grid where 0 = empty, 1 = wall
        /// * `ball` - Ball starting position [row, col]
        /// * `hole` - Hole position [row, col]
        ///
        /// # Returns
        /// Lexicographically smallest shortest instruction string, or
        /// "impossible" if unreachable
        pub fn find_shortest_way(maze: Vec<Vec<i32>>, ball: Vec<i32>, hole: Vec<i32>) -> String {
            let rows = maze.len();
            let cols = maze[0].len();
            let hr = hole[0] as usize;
            let hc = hole[1] as usize;
            let br = ball[0] as usize;
            let bc = ball[1] as usize;

            // dist[r][c] = (shortest distance, lexicographically smallest path)
            // Initialize to (i32::MAX, empty) meaning unvisited
            let mut dist: Vec<Vec<(i32, String)>> =
                vec![vec![(i32::MAX, String::new()); cols]; rows];
            dist[br][bc] = (0, String::new());

            let mut queue = VecDeque::new();
            queue.push_back((br, bc));

            // Directions in alphabetical order: d, l, r, u
            let dirs: [(i32, i32, char); 4] =
                [(1, 0, 'd'), (0, -1, 'l'), (0, 1, 'r'), (-1, 0, 'u')];

            while let Some((r, c)) = queue.pop_front() {
                let (cur_dist, ref cur_path) = dist[r][c].clone();

                for &(dr, dc, dir_ch) in &dirs {
                    let mut nr = r as i32;
                    let mut nc = c as i32;
                    let mut steps = 0;
                    let mut fell_in_hole = false;

                    // Roll until hitting a wall/boundary or falling in hole
                    loop {
                        nr += dr;
                        nc += dc;
                        // Check boundary or wall
                        if nr < 0
                            || nr >= rows as i32
                            || nc < 0
                            || nc >= cols as i32
                            || maze[nr as usize][nc as usize] == 1
                        {
                            // Step back to last valid position
                            nr -= dr;
                            nc -= dc;
                            break;
                        }
                        steps += 1;
                        // Check if ball passes over the hole
                        if nr as usize == hr && nc as usize == hc {
                            fell_in_hole = true;
                            break;
                        }
                    }

                    if steps == 0 {
                        continue;
                    }

                    let new_dist = cur_dist + steps;
                    let mut new_path = cur_path.clone();
                    new_path.push(dir_ch);
                    let stop_r = nr as usize;
                    let stop_c = nc as usize;

                    let (ref old_dist, ref old_path) = dist[stop_r][stop_c];
                    // Update if shorter distance, or same distance with
                    // lexicographically smaller path
                    if new_dist < *old_dist || (new_dist == *old_dist && new_path < *old_path) {
                        dist[stop_r][stop_c] = (new_dist, new_path);
                        // Only re-enqueue if the ball didn't fall in the hole
                        // (hole is a terminal state)
                        if !fell_in_hole {
                            queue.push_back((stop_r, stop_c));
                        }
                    }
                }
            }

            let (d, ref path) = dist[hr][hc];
            if d == i32::MAX {
                "impossible".to_string()
            } else {
                path.clone()
            }
        }
    }
}

/// Time: O(m*n*log(m*n)), Space: O(m*n)
/// Dijkstra's algorithm with a min-heap ordered by (distance, path).
/// Guarantees the first time we pop a position it has the optimal
/// (distance, path). Same rolling + hole-drop logic as BFS solution.
#[allow(dead_code)]
mod ai_solution {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    pub struct Solution;

    impl Solution {
        /// Find the shortest path instruction to drop the ball into the hole.
        ///
        /// # Arguments
        /// * `maze` - 2D grid where 0 = empty, 1 = wall
        /// * `ball` - Ball starting position [row, col]
        /// * `hole` - Hole position [row, col]
        ///
        /// # Returns
        /// Lexicographically smallest shortest instruction string, or
        /// "impossible" if unreachable
        pub fn find_shortest_way(maze: Vec<Vec<i32>>, ball: Vec<i32>, hole: Vec<i32>) -> String {
            let rows = maze.len();
            let cols = maze[0].len();
            let hr = hole[0] as usize;
            let hc = hole[1] as usize;
            let br = ball[0] as usize;
            let bc = ball[1] as usize;

            // (distance, path) for each cell; None means unvisited
            let mut best: Vec<Vec<Option<(i32, String)>>> = vec![vec![None; cols]; rows];

            // Min-heap: Reverse((distance, path, row, col))
            // Reverse because BinaryHeap is a max-heap
            let mut heap: BinaryHeap<Reverse<(i32, String, usize, usize)>> = BinaryHeap::new();
            heap.push(Reverse((0, String::new(), br, bc)));

            // Directions in alphabetical order: d, l, r, u
            let dirs: [(i32, i32, char); 4] =
                [(1, 0, 'd'), (0, -1, 'l'), (0, 1, 'r'), (-1, 0, 'u')];

            while let Some(Reverse((d, path, r, c))) = heap.pop() {
                // If already finalized with a better or equal result, skip
                if let Some((bd, ref bp)) = best[r][c] {
                    if d > bd || (d == bd && path >= *bp) {
                        continue;
                    }
                }
                best[r][c] = Some((d, path.clone()));

                // If this is the hole, we found the answer (Dijkstra guarantees
                // optimality on first extraction)
                if r == hr && c == hc {
                    return path;
                }

                for &(dr, dc, dir_ch) in &dirs {
                    let mut nr = r as i32;
                    let mut nc = c as i32;
                    let mut steps = 0i32;

                    loop {
                        nr += dr;
                        nc += dc;
                        if nr < 0
                            || nr >= rows as i32
                            || nc < 0
                            || nc >= cols as i32
                            || maze[nr as usize][nc as usize] == 1
                        {
                            nr -= dr;
                            nc -= dc;
                            break;
                        }
                        steps += 1;
                        // Ball drops into the hole when passing over it
                        if nr as usize == hr && nc as usize == hc {
                            break;
                        }
                    }

                    if steps == 0 {
                        continue;
                    }

                    let new_dist = d + steps;
                    let mut new_path = path.clone();
                    new_path.push(dir_ch);
                    let stop_r = nr as usize;
                    let stop_c = nc as usize;

                    // Only push if potentially better than what we have
                    let dominated = match &best[stop_r][stop_c] {
                        Some((bd, bp)) => new_dist > *bd || (new_dist == *bd && new_path >= *bp),
                        None => false,
                    };
                    if !dominated {
                        heap.push(Reverse((new_dist, new_path, stop_r, stop_c)));
                    }
                }
            }

            "impossible".to_string()
        }
    }
}

fn main() {
    println!("LeetCode 499: The Maze III");
    let maze = vec![
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 1],
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 1],
        vec![0, 1, 0, 0, 0],
    ];
    let cases: Vec<(Vec<i32>, Vec<i32>, &str)> = vec![
        (vec![4, 3], vec![0, 1], "lul"),
        (vec![4, 3], vec![3, 0], "impossible"),
    ];
    for (ball, hole, expected) in &cases {
        let r1 = my_solution::Solution::find_shortest_way(maze.clone(), ball.clone(), hole.clone());
        let r2 = ai_solution::Solution::find_shortest_way(maze.clone(), ball.clone(), hole.clone());
        println!(
            "  ball={ball:?}, hole={hole:?} => my=\"{r1}\", ai=\"{r2}\" \
             (expected \"{expected}\")"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_both {
        ($name:ident, $maze:expr, $ball:expr, $hole:expr, $expected:expr) => {
            mod $name {
                use super::*;
                #[test]
                fn my() {
                    assert_eq!(
                        my_solution::Solution::find_shortest_way($maze, $ball, $hole),
                        $expected
                    );
                }
                #[test]
                fn ai() {
                    assert_eq!(
                        ai_solution::Solution::find_shortest_way($maze, $ball, $hole),
                        $expected
                    );
                }
            }
        };
    }

    fn default_maze() -> Vec<Vec<i32>> {
        vec![
            vec![0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1],
            vec![0, 1, 0, 0, 0],
        ]
    }

    test_both!(example1_lul, default_maze(), vec![4, 3], vec![0, 1], "lul");
    test_both!(
        example2_impossible,
        default_maze(),
        vec![4, 3],
        vec![3, 0],
        "impossible"
    );
    test_both!(
        straight_drop,
        vec![vec![0, 0, 0, 0, 0]],
        vec![0, 4],
        vec![0, 0],
        "l"
    );
    test_both!(
        hole_midway_horizontal,
        vec![vec![0, 0, 0, 0, 0]],
        vec![0, 4],
        vec![0, 2],
        "l"
    );
    test_both!(
        hole_midway_vertical,
        vec![vec![0], vec![0], vec![0], vec![0]],
        vec![0, 0],
        vec![2, 0],
        "d"
    );
    test_both!(
        simple_impossible,
        vec![vec![0, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
        vec![0, 0],
        vec![0, 2],
        "impossible"
    );
    test_both!(
        adjacent_hole,
        vec![vec![0, 0], vec![0, 0]],
        vec![0, 0],
        vec![0, 1],
        "r"
    );
    test_both!(
        need_bounce,
        vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
        vec![0, 0],
        vec![2, 2],
        "dr"
    );
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(Vec<Vec<i32>>, Vec<i32>, Vec<i32>) -> String,
{
    let iterations = 50_000;
    let size = 50;
    let mut maze = vec![vec![0i32; size]; size];
    // Add scattered walls but leave edges mostly open
    for i in (2..size - 2).step_by(3) {
        for j in (2..size - 2).step_by(4) {
            maze[i][j] = 1;
        }
    }
    maze[0][0] = 0;
    maze[size / 2][size / 2] = 0;

    let ball = vec![0, 0];
    let hole = vec![(size / 2) as i32, (size / 2) as i32];

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(
            std::hint::black_box(maze.clone()),
            std::hint::black_box(ball.clone()),
            std::hint::black_box(hole.clone()),
        ));
    }
    let elapsed = start.elapsed();
    println!(
        "{name}: {iterations} iterations in {elapsed:?} (avg {:?})",
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::find_shortest_way);
    bench_solution("AI solution", ai_solution::Solution::find_shortest_way);
}
