// LeetCode 407: Trapping Rain Water II [Hard]
// https://leetcode.com/problems/trapping-rain-water-ii/
// Benchmark: 10000 iterations
//   My solution (min-heap BFS, Vec<Vec<bool>>): avg 12.57us
//   AI solution (min-heap BFS, flat Vec<bool>): avg 10.05us
//   -> AI solution is 1.25x faster
//
// Given an m x n integer matrix heightMap representing a 2D elevation map,
// return the volume of water it can trap after raining.
//
// Examples:
//   [[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]] -> 4
//   [[3,3,3,3,3],[3,2,2,2,3],[3,2,1,2,3],[3,2,2,2,3],[3,3,3,3,3]] -> 10
//
// Constraints:
//   m == heightMap.length, n == heightMap[i].length
//   1 <= m, n <= 200
//   0 <= heightMap[i][j] <= 2 * 10^4
//
// Run:  cargo run --example lc_0407_trapping_rain_water_ii
// Test: cargo test --example lc_0407_trapping_rain_water_ii
// Bench: cargo test --example lc_0407_trapping_rain_water_ii bench -- --nocapture

/// My solution module — BFS from boundary using a min-heap.
///
/// Start by pushing all border cells into a priority queue (min-heap).
/// Pop the lowest cell, check all 4 neighbors. If a neighbor is lower
/// than the current water level, we trap the difference. Push the
/// neighbor with max(its height, current water level) into the heap.
///
/// Time:  O(m*n * log(m*n))
/// Space: O(m*n)
mod my_solution {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    pub struct Solution;

    impl Solution {
        /// Calculate how much water the 2D elevation map can trap.
        ///
        /// # Arguments
        ///
        /// * `height_map` - 2D grid of cell heights
        ///
        /// # Returns
        ///
        /// Total volume of trapped water
        pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
            let m = height_map.len();
            let n = height_map[0].len();
            if m < 3 || n < 3 {
                return 0; // no interior cells
            }

            // Min-heap of (height, row, col)
            let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> =
                BinaryHeap::new();
            let mut visited = vec![vec![false; n]; m];

            // Push all border cells
            for i in 0..m {
                for j in 0..n {
                    if i == 0 || i == m - 1 || j == 0 || j == n - 1 {
                        heap.push(Reverse((height_map[i][j], i, j)));
                        visited[i][j] = true;
                    }
                }
            }

            let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let mut water = 0;

            while let Some(Reverse((h, r, c))) = heap.pop() {
                for (dr, dc) in &dirs {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                        let (nr, nc) = (nr as usize, nc as usize);
                        if !visited[nr][nc] {
                            visited[nr][nc] = true;
                            // If neighbor is lower, trap water
                            water += (h - height_map[nr][nc]).max(0);
                            // Push neighbor with the effective water level
                            heap.push(Reverse((
                                h.max(height_map[nr][nc]),
                                nr,
                                nc,
                            )));
                        }
                    }
                }
            }

            water
        }
    }
}

/// AI solution module — same min-heap BFS, flat visited array.
///
/// Uses a flat `Vec<bool>` instead of `Vec<Vec<bool>>` for slightly
/// better cache locality. Same algorithmic approach.
///
/// Time:  O(m*n * log(m*n))
/// Space: O(m*n)
mod ai_solution {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    pub struct Solution;

    impl Solution {
        /// Calculate how much water the 2D elevation map can trap.
        ///
        /// # Arguments
        ///
        /// * `height_map` - 2D grid of cell heights
        ///
        /// # Returns
        ///
        /// Total volume of trapped water
        pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
            let m = height_map.len();
            let n = height_map[0].len();
            if m < 3 || n < 3 {
                return 0;
            }

            let mut heap: BinaryHeap<Reverse<(i32, usize)>> =
                BinaryHeap::new();
            // Flat visited array for better cache performance
            let mut visited = vec![false; m * n];

            // Push border cells using flat index
            for i in 0..m {
                for j in 0..n {
                    if i == 0 || i == m - 1 || j == 0 || j == n - 1 {
                        let idx = i * n + j;
                        heap.push(Reverse((height_map[i][j], idx)));
                        visited[idx] = true;
                    }
                }
            }

            let mut water = 0;

            while let Some(Reverse((h, idx))) = heap.pop() {
                let r = idx / n;
                let c = idx % n;

                // Check 4 neighbors using conditional bounds
                let neighbors = [
                    if r > 0 { Some((r - 1) * n + c) } else { None },
                    if r + 1 < m { Some((r + 1) * n + c) } else { None },
                    if c > 0 { Some(r * n + c - 1) } else { None },
                    if c + 1 < n { Some(r * n + c + 1) } else { None },
                ];

                for ni in neighbors.into_iter().flatten() {
                    if !visited[ni] {
                        visited[ni] = true;
                        let nh = height_map[ni / n][ni % n];
                        water += (h - nh).max(0);
                        heap.push(Reverse((h.max(nh), ni)));
                    }
                }
            }

            water
        }
    }
}

fn main() {
    let cases: Vec<(Vec<Vec<i32>>, i32)> = vec![
        (
            vec![
                vec![1, 4, 3, 1, 3, 2],
                vec![3, 2, 1, 3, 2, 4],
                vec![2, 3, 3, 2, 3, 1],
            ],
            4,
        ),
        (
            vec![
                vec![3, 3, 3, 3, 3],
                vec![3, 2, 2, 2, 3],
                vec![3, 2, 1, 2, 3],
                vec![3, 2, 2, 2, 3],
                vec![3, 3, 3, 3, 3],
            ],
            10,
        ),
    ];

    println!("--- My Solution ---");
    for (grid, expected) in &cases {
        let result = my_solution::Solution::trap_rain_water(grid.clone());
        println!("  result={result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (grid, expected) in &cases {
        let result = ai_solution::Solution::trap_rain_water(grid.clone());
        println!("  result={result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_trap {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    let grid = vec![
                        vec![1, 4, 3, 1, 3, 2],
                        vec![3, 2, 1, 3, 2, 4],
                        vec![2, 3, 3, 2, 3, 1],
                    ];
                    assert_eq!(Solution::trap_rain_water(grid), 4);
                }

                #[test]
                fn test_example2() {
                    let grid = vec![
                        vec![3, 3, 3, 3, 3],
                        vec![3, 2, 2, 2, 3],
                        vec![3, 2, 1, 2, 3],
                        vec![3, 2, 2, 2, 3],
                        vec![3, 3, 3, 3, 3],
                    ];
                    assert_eq!(Solution::trap_rain_water(grid), 10);
                }

                #[test]
                fn test_flat_surface() {
                    let grid = vec![
                        vec![5, 5, 5],
                        vec![5, 5, 5],
                        vec![5, 5, 5],
                    ];
                    assert_eq!(Solution::trap_rain_water(grid), 0);
                }

                #[test]
                fn test_no_interior() {
                    // Only 2 rows, no interior cell
                    let grid = vec![
                        vec![1, 2, 3],
                        vec![4, 5, 6],
                    ];
                    assert_eq!(Solution::trap_rain_water(grid), 0);
                }

                #[test]
                fn test_single_cell_pool() {
                    let grid = vec![
                        vec![5, 5, 5],
                        vec![5, 1, 5],
                        vec![5, 5, 5],
                    ];
                    // The center cell (height 1) can hold 4 units
                    assert_eq!(Solution::trap_rain_water(grid), 4);
                }
            }
        };
    }

    test_trap!(my_tests, crate::my_solution::Solution);
    test_trap!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 10_000_u32;

        // Build a larger test grid: 20x20 with low interior
        let m = 20_usize;
        let n = 20_usize;
        let mut grid = vec![vec![10; n]; m];
        for row in grid.iter_mut().take(m - 1).skip(1) {
            for cell in row.iter_mut().take(n - 1).skip(1) {
                *cell = 1;
            }
        }

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::trap_rain_water(
                    std::hint::black_box(grid.clone()),
                ),
            );
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::trap_rain_water(
                    std::hint::black_box(grid.clone()),
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
            let ratio = ai_avg.as_nanos() as f64 / my_avg.as_nanos().max(1) as f64;
            println!("-> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_avg.as_nanos() as f64 / ai_avg.as_nanos().max(1) as f64;
            println!("-> AI solution is {ratio:.2}x faster");
        }
    }
}
