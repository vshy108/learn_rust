// LeetCode 286: Walls and Gates [Medium] (Premium)
// https://leetcode.com/problems/walls-and-gates/
//
// First benchmark: 100_000 iterations, 2025-07-06
//   My solution avg: 18.514us
//   AI solution avg: 18.981us
//   Ratio: My 1.03x faster
//
// You are given an m x n grid `rooms` initialized with three possible values:
//   -1  : A wall or an obstacle.
//    0  : A gate.
//   INF : An empty room (2^31 - 1 = 2147483647).
//
// Fill each empty room with the distance to its nearest gate. If it is
// impossible to reach a gate, it should be filled with INF.
//
// Example 1:
//   Input:  [[INF,-1,0,INF],[INF,INF,INF,-1],[INF,-1,INF,-1],[0,-1,INF,INF]]
//   Output: [[3,-1,0,1],[2,2,1,-1],[1,-1,2,-1],[0,-1,3,4]]
//
// Example 2:
//   Input:  [[-1]]
//   Output: [[-1]]
//
// Constraints:
//   - m == rooms.length, n == rooms[i].length
//   - 1 <= m, n <= 250
//   - rooms[i][j] is -1, 0, or 2^31 - 1
//
// Run:  cargo run --example lc_0286_walls_and_gates
// Test: cargo test --example lc_0286_walls_and_gates
// Bench: cargo test --example lc_0286_walls_and_gates bench -- --nocapture

use std::collections::VecDeque;

const INF: i32 = i32::MAX;

/// My solution — Multi-source BFS from all gates simultaneously (doocs).
///
/// Enqueue all gate positions, then BFS outward layer by layer.
/// Each layer increments distance by 1. Only visit cells that are
/// still INF (unvisited empty rooms).
///
/// Time:  O(m * n)
/// Space: O(m * n) for the queue
mod my_solution {
    use super::*;

    pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
        let m = rooms.len();
        let n = rooms[0].len();
        let mut q = VecDeque::new();

        // Seed BFS with all gate positions (distance 0)
        for i in 0..m {
            for j in 0..n {
                if rooms[i][j] == 0 {
                    q.push_back((i, j));
                }
            }
        }

        let dirs: [i32; 5] = [-1, 0, 1, 0, -1];
        let mut d = 0;

        while !q.is_empty() {
            d += 1;
            // Process all cells at the current distance level
            for _ in 0..q.len() {
                let (r, c) = q.pop_front().unwrap();
                for k in 0..4 {
                    let x = r as i32 + dirs[k];
                    let y = c as i32 + dirs[k + 1];
                    if x >= 0
                        && x < m as i32
                        && y >= 0
                        && y < n as i32
                        && rooms[x as usize][y as usize] == INF
                    {
                        rooms[x as usize][y as usize] = d;
                        q.push_back((x as usize, y as usize));
                    }
                }
            }
        }
    }
}

/// AI solution — BFS using separate visited tracking.
///
/// Same multi-source BFS idea but tracks distance inline: instead of
/// a global distance counter, looks up `rooms[r][c] + 1` for each
/// neighbor. This avoids the inner loop over the current layer.
///
/// Time:  O(m * n)
/// Space: O(m * n)
mod ai_solution {
    use super::*;

    pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
        let m = rooms.len();
        let n = rooms[0].len();
        let mut q = VecDeque::new();

        for i in 0..m {
            for j in 0..n {
                if rooms[i][j] == 0 {
                    q.push_back((i, j));
                }
            }
        }

        while let Some((r, c)) = q.pop_front() {
            let next_dist = rooms[r][c] + 1;
            for (dr, dc) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                    let (nr, nc) = (nr as usize, nc as usize);
                    if rooms[nr][nc] == INF {
                        rooms[nr][nc] = next_dist;
                        q.push_back((nr, nc));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_walls {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let mut rooms1 = $input;
                let mut rooms2 = rooms1.clone();
                my_solution::walls_and_gates(&mut rooms1);
                assert_eq!(rooms1, $expected, "my_solution failed");
                ai_solution::walls_and_gates(&mut rooms2);
                assert_eq!(rooms2, $expected, "ai_solution failed");
            }
        };
    }

    test_walls!(
        test_example_1,
        vec![
            vec![INF, -1, 0, INF],
            vec![INF, INF, INF, -1],
            vec![INF, -1, INF, -1],
            vec![0, -1, INF, INF],
        ],
        vec![
            vec![3, -1, 0, 1],
            vec![2, 2, 1, -1],
            vec![1, -1, 2, -1],
            vec![0, -1, 3, 4],
        ]
    );

    test_walls!(test_example_2, vec![vec![-1]], vec![vec![-1]]);

    // All gates
    test_walls!(
        test_all_gates,
        vec![vec![0, 0], vec![0, 0]],
        vec![vec![0, 0], vec![0, 0]]
    );

    // All walls
    test_walls!(
        test_all_walls,
        vec![vec![-1, -1], vec![-1, -1]],
        vec![vec![-1, -1], vec![-1, -1]]
    );

    // All empty, no gates
    test_walls!(
        test_no_gates,
        vec![vec![INF, INF], vec![INF, INF]],
        vec![vec![INF, INF], vec![INF, INF]]
    );

    // Single gate
    test_walls!(test_single_gate, vec![vec![0]], vec![vec![0]]);

    // Single empty room
    test_walls!(test_single_empty, vec![vec![INF]], vec![vec![INF]]);

    // Gate in corner
    test_walls!(
        test_gate_corner,
        vec![vec![0, INF], vec![INF, INF]],
        vec![vec![0, 1], vec![1, 2]]
    );

    // Unreachable room (surrounded by walls)
    test_walls!(
        test_unreachable,
        vec![vec![0, -1, INF], vec![-1, -1, -1], vec![INF, -1, INF],],
        vec![vec![0, -1, INF], vec![-1, -1, -1], vec![INF, -1, INF],]
    );

    // Multiple gates
    test_walls!(
        test_multiple_gates,
        vec![vec![INF, 0, INF], vec![INF, INF, INF], vec![INF, 0, INF],],
        vec![vec![1, 0, 1], vec![2, 1, 2], vec![1, 0, 1],]
    );

    // 1 x n row
    test_walls!(
        test_single_row,
        vec![vec![0, INF, INF, -1, INF]],
        vec![vec![0, 1, 2, -1, INF]]
    );

    // n x 1 column
    test_walls!(
        test_single_col,
        vec![vec![INF], vec![0], vec![INF], vec![INF]],
        vec![vec![1], vec![0], vec![1], vec![2]]
    );
}

fn main() {
    println!("LeetCode 286: Walls and Gates");
    let mut rooms = vec![
        vec![INF, -1, 0, INF],
        vec![INF, INF, INF, -1],
        vec![INF, -1, INF, -1],
        vec![0, -1, INF, INF],
    ];
    println!("Before: {:?}", rooms);
    let mut rooms2 = rooms.clone();
    my_solution::walls_and_gates(&mut rooms);
    ai_solution::walls_and_gates(&mut rooms2);
    println!("After (my): {:?}", rooms);
    println!("After (ai): {:?}", rooms2);
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        // Build a 50x50 grid with gates at corners
        let m = 50;
        let n = 50;
        let base: Vec<Vec<i32>> = (0..m)
            .map(|i| {
                (0..n)
                    .map(|j| {
                        if (i == 0 && j == 0) || (i == m - 1 && j == n - 1) {
                            0
                        } else {
                            INF
                        }
                    })
                    .collect()
            })
            .collect();

        let iterations = 100_000;

        // Warm up
        for _ in 0..1_000 {
            let mut r = base.clone();
            std::hint::black_box(my_solution::walls_and_gates(&mut r));
            let mut r = base.clone();
            std::hint::black_box(ai_solution::walls_and_gates(&mut r));
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut r = base.clone();
            std::hint::black_box(my_solution::walls_and_gates(&mut r));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut r = base.clone();
            std::hint::black_box(ai_solution::walls_and_gates(&mut r));
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
