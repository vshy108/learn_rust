// LeetCode 296: Best Meeting Point [Hard] (Premium)
// https://leetcode.com/problems/best-meeting-point/
//
// First benchmark: 100_000 iterations, 2025-07-06
//   My solution avg: 2.260us
//   AI solution avg: 2.247us
//   Ratio: AI 1.01x faster
//
// Given an m x n binary grid where 1 marks a friend's home, return the minimal
// total Manhattan distance to a meeting point.
//
// Key insight: The optimal meeting point's row is the median of all rows, and
// the optimal column is the median of all columns (independent 1D problems).
//
// Example 1: grid = [[1,0,0,0,1],[0,0,0,0,0],[0,0,1,0,0]] -> 6
// Example 2: grid = [[1,1]] -> 1
//
// Constraints:
//   - 1 <= m, n <= 200
//   - grid[i][j] is 0 or 1
//   - At least two friends
//
// Run:  cargo run --example lc_0296_best_meeting_point
// Test: cargo test --example lc_0296_best_meeting_point
// Bench: cargo test --example lc_0296_best_meeting_point bench -- --nocapture

struct Solution;

/// My solution — Collect rows & cols, sort cols, use median (doocs).
///
/// Rows are already sorted (we iterate top to bottom). Cols need sorting.
/// Median minimizes sum of absolute deviations.
///
/// Time:  O(m*n + k*log(k)) where k = number of friends
/// Space: O(k)
mod my_solution {
    use super::Solution;

    impl Solution {
        pub fn min_total_distance_my(grid: &[Vec<i32>]) -> i32 {
            let m = grid.len();
            let n = grid[0].len();
            let mut rows = Vec::new();
            let mut cols = Vec::new();

            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == 1 {
                        rows.push(i as i32);
                        cols.push(j as i32);
                    }
                }
            }

            // rows already sorted; cols need sorting
            cols.sort_unstable();

            let med_r = rows[rows.len() / 2];
            let med_c = cols[cols.len() / 2];

            let dist =
                |arr: &[i32], target: i32| -> i32 { arr.iter().map(|&v| (v - target).abs()).sum() };

            dist(&rows, med_r) + dist(&cols, med_c)
        }
    }
}

/// AI solution — Two-pointer sum without finding explicit median.
///
/// Collect sorted rows and sorted cols. Use two pointers from both ends
/// to compute sum of distances: for sorted array, total distance to
/// median = sum of (arr[hi] - arr[lo]) converging inward.
///
/// Time:  O(m*n + k*log(k))
/// Space: O(k)
mod ai_solution {
    use super::Solution;

    impl Solution {
        pub fn min_total_distance_ai(grid: &[Vec<i32>]) -> i32 {
            let m = grid.len();
            let n = grid[0].len();
            let mut rows = Vec::new();
            let mut cols = Vec::new();

            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == 1 {
                        rows.push(i as i32);
                        cols.push(j as i32);
                    }
                }
            }

            cols.sort_unstable();

            // Sum distances using two-pointer approach from ends
            fn sum_dist(sorted: &[i32]) -> i32 {
                let mut total = 0;
                let mut lo = 0;
                let mut hi = sorted.len() - 1;
                while lo < hi {
                    total += sorted[hi] - sorted[lo];
                    lo += 1;
                    hi -= 1;
                }
                total
            }

            sum_dist(&rows) + sum_dist(&cols)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_meeting {
        ($name:ident, $grid:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    Solution::min_total_distance_my(&$grid),
                    $expected,
                    "my_solution failed"
                );
                assert_eq!(
                    Solution::min_total_distance_ai(&$grid),
                    $expected,
                    "ai_solution failed"
                );
            }
        };
    }

    test_meeting!(
        test_example_1,
        vec![
            vec![1, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
        ],
        6
    );

    test_meeting!(test_example_2, vec![vec![1, 1]], 1);

    // All in same row
    test_meeting!(test_same_row, vec![vec![1, 0, 1, 0, 1]], 4);

    // All in same column
    test_meeting!(
        test_same_col,
        vec![vec![1], vec![0], vec![1], vec![0], vec![1]],
        4
    );

    // 2x2 all ones
    test_meeting!(test_2x2_all, vec![vec![1, 1], vec![1, 1]], 4);

    // Two friends adjacent
    test_meeting!(test_adjacent, vec![vec![1, 1, 0]], 1);

    // Corner friends
    test_meeting!(
        test_corners,
        vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1],],
        8
    );

    // Single row, friends at ends
    test_meeting!(test_ends, vec![vec![1, 0, 0, 0, 1]], 4);

    // Three friends in a line
    test_meeting!(
        test_line,
        vec![vec![0, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
        2
    );
}

fn main() {
    println!("LeetCode 296: Best Meeting Point");
    let grid = vec![
        vec![1, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0],
    ];
    println!(
        "my_solution: min_total_distance = {}",
        Solution::min_total_distance_my(&grid)
    );
    println!(
        "ai_solution: min_total_distance = {}",
        Solution::min_total_distance_ai(&grid)
    );
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        // 50x50 grid with friends in a diagonal
        let grid: Vec<Vec<i32>> = (0..50)
            .map(|i| (0..50).map(|j| if i == j { 1 } else { 0 }).collect())
            .collect();
        let iterations = 100_000;

        // Warm up
        for _ in 0..1_000 {
            std::hint::black_box(Solution::min_total_distance_my(&grid));
            std::hint::black_box(Solution::min_total_distance_ai(&grid));
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(Solution::min_total_distance_my(&grid));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(Solution::min_total_distance_ai(&grid));
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
