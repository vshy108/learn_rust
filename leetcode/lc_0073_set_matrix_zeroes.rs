// LeetCode 73: Set Matrix Zeroes [Medium]
// https://leetcode.com/problems/set-matrix-zeroes/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 12.736us
//   AI solution avg: 12.624us
//   Ratio: ~tied
//
// Given an m x n integer matrix, if an element is 0, set its entire
// row and column to 0. Do it in place.
//
// Example 1: [[1,1,1],[1,0,1],[1,1,1]] -> [[1,0,1],[0,0,0],[1,0,1]]
// Example 2: [[0,1,2,0],[3,4,5,2],[1,3,1,5]] -> [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
//
// Constraints: m,n in [1,200], -2^31 <= matrix[i][j] <= 2^31-1
//
// Run:  cargo run --example lc_0073_set_matrix_zeroes
// Test: cargo test --example lc_0073_set_matrix_zeroes
// Bench: cargo test --example lc_0073_set_matrix_zeroes bench -- --nocapture

/// My solution — track zero rows/cols with separate sets.
///
/// Time: O(m*n).
/// Space: O(m+n) for the sets.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Set entire row and column to 0 for each 0 element.
        ///
        /// # Arguments
        /// * `matrix` - m x n matrix (modified in place)
        pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
            let (m, n) = (matrix.len(), matrix[0].len());
            let mut zero_rows = vec![false; m];
            let mut zero_cols = vec![false; n];

            for i in 0..m {
                for j in 0..n {
                    if matrix[i][j] == 0 {
                        zero_rows[i] = true;
                        zero_cols[j] = true;
                    }
                }
            }
            for i in 0..m {
                for j in 0..n {
                    if zero_rows[i] || zero_cols[j] {
                        matrix[i][j] = 0;
                    }
                }
            }
        }
    }
}

/// AI solution — O(1) extra space using first row/col as markers.
///
/// Use matrix[0][j] and matrix[i][0] as flags. Track whether
/// the first row/col themselves need zeroing separately.
///
/// Time: O(m*n).
/// Space: O(1) extra.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Set entire row and column to 0 for each 0 element.
        ///
        /// # Arguments
        /// * `matrix` - m x n matrix (modified in place)
        pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
            let (m, n) = (matrix.len(), matrix[0].len());
            let mut first_col_zero = false;

            for i in 0..m {
                if matrix[i][0] == 0 {
                    first_col_zero = true;
                }
                for j in 1..n {
                    if matrix[i][j] == 0 {
                        matrix[i][0] = 0;
                        matrix[0][j] = 0;
                    }
                }
            }
            // Zero out cells based on markers (bottom-to-top to avoid overwriting markers)
            for i in (0..m).rev() {
                for j in (1..n).rev() {
                    if matrix[i][0] == 0 || matrix[0][j] == 0 {
                        matrix[i][j] = 0;
                    }
                }
                if first_col_zero {
                    matrix[i][0] = 0;
                }
            }
        }
    }
}

fn main() {
    let mut m1 = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    let mut m2 = m1.clone();
    my_solution::Solution::set_zeroes(&mut m1);
    ai_solution::Solution::set_zeroes(&mut m2);
    println!("my:{m1:?}");
    println!("ai:{m2:?}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_set_zeroes {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_basic() {
                    let mut m = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
                    <$solution>::set_zeroes(&mut m);
                    assert_eq!(m, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
                }

                #[test]
                fn test_corners() {
                    let mut m = vec![
                        vec![0, 1, 2, 0],
                        vec![3, 4, 5, 2],
                        vec![1, 3, 1, 5],
                    ];
                    <$solution>::set_zeroes(&mut m);
                    assert_eq!(
                        m,
                        vec![
                            vec![0, 0, 0, 0],
                            vec![0, 4, 5, 0],
                            vec![0, 3, 1, 0],
                        ]
                    );
                }

                #[test]
                fn test_no_zeros() {
                    let mut m = vec![vec![1, 2], vec![3, 4]];
                    <$solution>::set_zeroes(&mut m);
                    assert_eq!(m, vec![vec![1, 2], vec![3, 4]]);
                }

                #[test]
                fn test_all_zeros() {
                    let mut m = vec![vec![0, 0], vec![0, 0]];
                    <$solution>::set_zeroes(&mut m);
                    assert_eq!(m, vec![vec![0, 0], vec![0, 0]]);
                }

                #[test]
                fn test_single_cell_zero() {
                    let mut m = vec![vec![0]];
                    <$solution>::set_zeroes(&mut m);
                    assert_eq!(m, vec![vec![0]]);
                }
            }
        };
    }

    test_set_zeroes!(my, crate::my_solution::Solution);
    test_set_zeroes!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let template: Vec<Vec<i32>> = (0..20)
            .map(|i| (0..20).map(|j| if (i + j) % 7 == 0 { 0 } else { 1 }).collect())
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut m = template.clone();
            std::hint::black_box(crate::my_solution::Solution::set_zeroes(&mut m));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut m = template.clone();
            std::hint::black_box(crate::ai_solution::Solution::set_zeroes(&mut m));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg ({:.0}ms total)",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            my_duration.as_millis(),
        );
        println!(
            "AI solution: {:.3}us avg ({:.0}ms total)",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            ai_duration.as_millis(),
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
