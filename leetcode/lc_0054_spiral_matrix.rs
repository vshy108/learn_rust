// LeetCode 54: Spiral Matrix [Medium]
// https://leetcode.com/problems/spiral-matrix/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 2.984us
//   AI solution avg: 5.195us
//   Ratio: My 1.74x faster
//
// Given an m x n matrix, return all elements in spiral order.
//
// Example 1: matrix = [[1,2,3],[4,5,6],[7,8,9]] -> [1,2,3,6,9,8,7,4,5]
// Example 2: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
//            -> [1,2,3,4,8,12,11,10,9,5,6,7]
//
// Constraints:
//   m == matrix.length, n == matrix[i].length
//   1 <= m, n <= 10
//   -100 <= matrix[i][j] <= 100
//
// Run:  cargo run --example lc_0054_spiral_matrix
// Test: cargo test --example lc_0054_spiral_matrix
// Bench: cargo test --example lc_0054_spiral_matrix bench -- --nocapture

/// My solution — boundary shrinking.
///
/// Track top, bottom, left, right boundaries. Traverse right, down, left,
/// up in order, shrinking the boundary after each pass.
///
/// Time: O(m*n) — visit every element once.
/// Space: O(1) extra (output is O(m*n)).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return elements of matrix in spiral order.
        ///
        /// # Arguments
        /// * `matrix` - m x n matrix
        ///
        /// # Returns
        /// Elements in spiral order
        pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
            let (m, n) = (matrix.len(), matrix[0].len());
            let mut result = Vec::with_capacity(m * n);
            let (mut top, mut bottom) = (0i32, m as i32 - 1);
            let (mut left, mut right) = (0i32, n as i32 - 1);

            while top <= bottom && left <= right {
                // Traverse right
                for col in left..=right {
                    result.push(matrix[top as usize][col as usize]);
                }
                top += 1;
                // Traverse down
                for row in top..=bottom {
                    result.push(matrix[row as usize][right as usize]);
                }
                right -= 1;
                // Traverse left (if rows remain)
                if top <= bottom {
                    for col in (left..=right).rev() {
                        result.push(matrix[bottom as usize][col as usize]);
                    }
                    bottom -= 1;
                }
                // Traverse up (if columns remain)
                if left <= right {
                    for row in (top..=bottom).rev() {
                        result.push(matrix[row as usize][left as usize]);
                    }
                    left += 1;
                }
            }
            result
        }
    }
}

/// AI solution — direction vector simulation.
///
/// Use direction vectors (dr, dc) and rotate when hitting a visited cell
/// or boundary. Mark visited cells.
///
/// Time: O(m*n) — visit every element once.
/// Space: O(m*n) for the visited matrix.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return elements of matrix in spiral order.
        ///
        /// # Arguments
        /// * `matrix` - m x n matrix
        ///
        /// # Returns
        /// Elements in spiral order
        pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
            let (m, n) = (matrix.len(), matrix[0].len());
            let total = m * n;
            let mut result = Vec::with_capacity(total);
            let mut visited = vec![vec![false; n]; m];

            // Directions: right, down, left, up
            let dirs: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            let (mut r, mut c, mut d) = (0i32, 0i32, 0usize);

            for _ in 0..total {
                result.push(matrix[r as usize][c as usize]);
                visited[r as usize][c as usize] = true;
                // Check next cell in current direction
                let nr = r + dirs[d].0;
                let nc = c + dirs[d].1;
                if nr < 0
                    || nr >= m as i32
                    || nc < 0
                    || nc >= n as i32
                    || visited[nr as usize][nc as usize]
                {
                    // Turn clockwise
                    d = (d + 1) % 4;
                }
                r += dirs[d].0;
                c += dirs[d].1;
            }
            result
        }
    }
}

fn main() {
    let m1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("my:{:?}", my_solution::Solution::spiral_order(m1.clone()),);
    println!("ai:{:?}", ai_solution::Solution::spiral_order(m1),);
}

#[cfg(test)]
mod tests {
    macro_rules! test_spiral_order {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_3x3() {
                    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
                    assert_eq!(
                        <$solution>::spiral_order(matrix),
                        vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
                    );
                }

                #[test]
                fn test_3x4() {
                    let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
                    assert_eq!(
                        <$solution>::spiral_order(matrix),
                        vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
                    );
                }

                #[test]
                fn test_1x1() {
                    assert_eq!(<$solution>::spiral_order(vec![vec![1]]), vec![1]);
                }

                #[test]
                fn test_1_row() {
                    assert_eq!(
                        <$solution>::spiral_order(vec![vec![1, 2, 3]]),
                        vec![1, 2, 3]
                    );
                }

                #[test]
                fn test_1_col() {
                    assert_eq!(
                        <$solution>::spiral_order(vec![vec![1], vec![2], vec![3]]),
                        vec![1, 2, 3]
                    );
                }

                #[test]
                fn test_2x2() {
                    assert_eq!(
                        <$solution>::spiral_order(vec![vec![1, 2], vec![3, 4]]),
                        vec![1, 2, 4, 3]
                    );
                }
            }
        };
    }

    test_spiral_order!(my, crate::my_solution::Solution);
    test_spiral_order!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let matrix: Vec<Vec<i32>> = (0..10)
            .map(|r| (0..10).map(|c| r * 10 + c).collect())
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::spiral_order(
                std::hint::black_box(matrix.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::spiral_order(
                std::hint::black_box(matrix.clone()),
            ));
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
