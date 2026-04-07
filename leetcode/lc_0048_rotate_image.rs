// LeetCode 48: Rotate Image [Medium]
// https://leetcode.com/problems/rotate-image/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 1.235us
//   AI solution avg: 0.889us
//   Ratio: AI 1.39x faster
//
// You are given an n x n 2D matrix representing an image, rotate the image
// by 90 degrees (clockwise). You have to rotate the image in-place.
//
// Example 1: matrix = [[1,2,3],[4,5,6],[7,8,9]] -> [[7,4,1],[8,5,2],[9,6,3]]
// Example 2: matrix = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
//            -> [[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]
//
// Constraints:
//   n == matrix.length == matrix[i].length
//   1 <= n <= 20
//   -1000 <= matrix[i][j] <= 1000
//
// Run:  cargo run --example lc_0048_rotate_image
// Test: cargo test --example lc_0048_rotate_image
// Bench: cargo test --example lc_0048_rotate_image bench -- --nocapture

/// My solution — transpose then reverse each row.
///
/// Step 1: Transpose the matrix (swap matrix[i][j] with matrix[j][i]).
/// Step 2: Reverse each row.
/// This achieves a 90-degree clockwise rotation.
///
/// Time: O(n^2) — visit each element twice.
/// Space: O(1) — in-place.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Rotate the matrix 90 degrees clockwise in-place.
        ///
        /// # Arguments
        /// * `matrix` - n x n matrix to rotate
        pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
            let n = matrix.len();
            // Transpose: swap across the main diagonal
            for i in 0..n {
                for j in (i + 1)..n {
                    let tmp = matrix[i][j];
                    matrix[i][j] = matrix[j][i];
                    matrix[j][i] = tmp;
                }
            }
            // Reverse each row
            for row in matrix.iter_mut() {
                row.reverse();
            }
        }
    }
}

/// AI solution — layer-by-layer four-way swap.
///
/// Rotate four elements at a time in concentric layers from outside in.
/// Each element moves directly to its final position in one pass.
///
/// Time: O(n^2) — each element moved exactly once.
/// Space: O(1) — in-place, only one temp variable.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Rotate the matrix 90 degrees clockwise in-place.
        ///
        /// # Arguments
        /// * `matrix` - n x n matrix to rotate
        pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
            let n = matrix.len();
            // Process concentric layers from outside in
            for layer in 0..n / 2 {
                let last = n - 1 - layer;
                for i in layer..last {
                    let offset = i - layer;
                    // Save top
                    let top = matrix[layer][i];
                    // Left -> Top
                    matrix[layer][i] = matrix[last - offset][layer];
                    // Bottom -> Left
                    matrix[last - offset][layer] = matrix[last][last - offset];
                    // Right -> Bottom
                    matrix[last][last - offset] = matrix[i][last];
                    // Top -> Right
                    matrix[i][last] = top;
                }
            }
        }
    }
}

fn main() {
    let mut m1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let mut m2 = m1.clone();
    my_solution::Solution::rotate(&mut m1);
    ai_solution::Solution::rotate(&mut m2);
    println!("my:{m1:?}");
    println!("ai:{m2:?}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_rotate {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_3x3() {
                    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
                    <$solution>::rotate(&mut matrix);
                    assert_eq!(
                        matrix,
                        vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]
                    );
                }

                #[test]
                fn test_4x4() {
                    let mut matrix = vec![
                        vec![5, 1, 9, 11],
                        vec![2, 4, 8, 10],
                        vec![13, 3, 6, 7],
                        vec![15, 14, 12, 16],
                    ];
                    <$solution>::rotate(&mut matrix);
                    assert_eq!(
                        matrix,
                        vec![
                            vec![15, 13, 2, 5],
                            vec![14, 3, 4, 1],
                            vec![12, 6, 8, 9],
                            vec![16, 7, 10, 11],
                        ]
                    );
                }

                #[test]
                fn test_1x1() {
                    let mut matrix = vec![vec![1]];
                    <$solution>::rotate(&mut matrix);
                    assert_eq!(matrix, vec![vec![1]]);
                }

                #[test]
                fn test_2x2() {
                    let mut matrix = vec![vec![1, 2], vec![3, 4]];
                    <$solution>::rotate(&mut matrix);
                    assert_eq!(matrix, vec![vec![3, 1], vec![4, 2]]);
                }

                #[test]
                fn test_5x5() {
                    let mut matrix = vec![
                        vec![1, 2, 3, 4, 5],
                        vec![6, 7, 8, 9, 10],
                        vec![11, 12, 13, 14, 15],
                        vec![16, 17, 18, 19, 20],
                        vec![21, 22, 23, 24, 25],
                    ];
                    <$solution>::rotate(&mut matrix);
                    assert_eq!(
                        matrix,
                        vec![
                            vec![21, 16, 11, 6, 1],
                            vec![22, 17, 12, 7, 2],
                            vec![23, 18, 13, 8, 3],
                            vec![24, 19, 14, 9, 4],
                            vec![25, 20, 15, 10, 5],
                        ]
                    );
                }
            }
        };
    }

    test_rotate!(my, crate::my_solution::Solution);
    test_rotate!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let template = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut m = template.clone();
            std::hint::black_box(crate::my_solution::Solution::rotate(&mut m));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut m = template.clone();
            std::hint::black_box(crate::ai_solution::Solution::rotate(&mut m));
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
