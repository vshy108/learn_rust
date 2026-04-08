// LeetCode 240: Search a 2D Matrix II [Medium]
// https://leetcode.com/problems/search-a-2d-matrix-ii/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 1.315us
//   AI solution avg: 1.159us
//   Ratio: AI 1.13x faster
//
// Write an efficient algorithm that searches for a value `target` in an
// m x n integer matrix. This matrix has the following properties:
//   - Integers in each row are sorted in ascending from left to right.
//   - Integers in each column are sorted in ascending from top to bottom.
//
// Constraints: m == matrix.length, n == matrix[i].length,
// 1 <= n, m <= 300, -10^9 <= matrix[i][j] <= 10^9
//
// Run:  cargo run --example lc_0240_search_2d_matrix_ii
// Test: cargo test --example lc_0240_search_2d_matrix_ii
// Bench: cargo test --example lc_0240_search_2d_matrix_ii bench -- --nocapture

/// My solution — binary search per row.
///
/// For each row, run a binary search for the target. Skip rows whose
/// first element exceeds the target (would be nice but not needed since
/// binary search returns false anyway).
///
/// Time: O(m * log n).
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Search for `target` in the sorted 2D matrix.
        pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
            for row in &matrix {
                if row.binary_search(&target).is_ok() {
                    return true;
                }
            }
            false
        }
    }
}

/// AI solution — staircase search (top-right corner).
///
/// Start from the top-right corner. If the current value equals target,
/// return true. If it's greater, move left (eliminate that column).
/// If it's less, move down (eliminate that row).
///
/// Time: O(m + n) — we move at most m steps down and n steps left.
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Search for `target` in the sorted 2D matrix.
        pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
            if matrix.is_empty() || matrix[0].is_empty() {
                return false;
            }

            let m = matrix.len();
            let n = matrix[0].len();
            // Start at top-right corner
            let mut row: usize = 0;
            let mut col: isize = (n - 1) as isize;

            while row < m && col >= 0 {
                let val = matrix[row][col as usize];
                if val == target {
                    return true;
                } else if val > target {
                    // Current value too large — move left
                    col -= 1;
                } else {
                    // Current value too small — move down
                    row += 1;
                }
            }

            false
        }
    }
}

fn main() {
    let matrix = vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ];

    let my1 = my_solution::Solution::search_matrix(matrix.clone(), 5);
    let ai1 = ai_solution::Solution::search_matrix(matrix.clone(), 5);
    println!("target=5: my={my1}, ai={ai1} (expected true)");

    let my2 = my_solution::Solution::search_matrix(matrix.clone(), 20);
    let ai2 = ai_solution::Solution::search_matrix(matrix, 20);
    println!("target=20: my={my2}, ai={ai2} (expected false)");
}

#[cfg(test)]
mod tests {
    macro_rules! test_search {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                fn sample_matrix() -> Vec<Vec<i32>> {
                    vec![
                        vec![1, 4, 7, 11, 15],
                        vec![2, 5, 8, 12, 19],
                        vec![3, 6, 9, 16, 22],
                        vec![10, 13, 14, 17, 24],
                        vec![18, 21, 23, 26, 30],
                    ]
                }

                #[test]
                fn test_found() {
                    assert!(<$solution>::search_matrix(sample_matrix(), 5));
                }

                #[test]
                fn test_not_found() {
                    assert!(!<$solution>::search_matrix(sample_matrix(), 20));
                }

                #[test]
                fn test_top_left() {
                    assert!(<$solution>::search_matrix(sample_matrix(), 1));
                }

                #[test]
                fn test_bottom_right() {
                    assert!(<$solution>::search_matrix(sample_matrix(), 30));
                }

                #[test]
                fn test_single_element_found() {
                    assert!(<$solution>::search_matrix(vec![vec![5]], 5));
                }

                #[test]
                fn test_single_element_not_found() {
                    assert!(!<$solution>::search_matrix(vec![vec![5]], 3));
                }

                #[test]
                fn test_negative_values() {
                    let matrix = vec![vec![-5, -3], vec![-1, 0]];
                    assert!(<$solution>::search_matrix(matrix, -3));
                }
            }
        };
    }

    test_search!(my, crate::my_solution::Solution);
    test_search!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let matrix: Vec<Vec<i32>> = (0..20)
            .map(|i| (0..20).map(|j| i * 20 + j).collect())
            .collect();
        let target = 215; // does not exist

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::search_matrix(
                std::hint::black_box(matrix.clone()),
                target,
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::search_matrix(
                std::hint::black_box(matrix.clone()),
                target,
            ));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else if ai_avg < my_avg {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        } else {
            println!("  Ratio: ~tied");
        }
    }
}
