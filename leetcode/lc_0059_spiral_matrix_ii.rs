// LeetCode 59: Spiral Matrix II [Medium]
// https://leetcode.com/problems/spiral-matrix-ii/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 2.460us
//   AI solution avg: 3.260us
//   Ratio: My 1.33x faster
//
// Given a positive integer n, generate an n x n matrix filled with
// elements from 1 to n^2 in spiral order.
//
// Example 1: n=3 -> [[1,2,3],[8,9,4],[7,6,5]]
// Example 2: n=1 -> [[1]]
//
// Constraints: 1 <= n <= 20
//
// Run:  cargo run --example lc_0059_spiral_matrix_ii
// Test: cargo test --example lc_0059_spiral_matrix_ii
// Bench: cargo test --example lc_0059_spiral_matrix_ii bench -- --nocapture

/// My solution — boundary shrinking fill.
///
/// Same boundary approach as Spiral Matrix I, but filling instead of reading.
///
/// Time: O(n^2).
/// Space: O(n^2) for the output matrix.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Generate n x n spiral matrix.
        ///
        /// # Arguments
        /// * `n` - matrix dimension
        ///
        /// # Returns
        /// n x n matrix filled in spiral order
        pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
            let n = n as usize;
            let mut matrix = vec![vec![0i32; n]; n];
            let (mut top, mut bottom) = (0i32, n as i32 - 1);
            let (mut left, mut right) = (0i32, n as i32 - 1);
            let mut num = 1i32;

            while top <= bottom && left <= right {
                for col in left..=right {
                    matrix[top as usize][col as usize] = num;
                    num += 1;
                }
                top += 1;
                for row in top..=bottom {
                    matrix[row as usize][right as usize] = num;
                    num += 1;
                }
                right -= 1;
                if top <= bottom {
                    for col in (left..=right).rev() {
                        matrix[bottom as usize][col as usize] = num;
                        num += 1;
                    }
                    bottom -= 1;
                }
                if left <= right {
                    for row in (top..=bottom).rev() {
                        matrix[row as usize][left as usize] = num;
                        num += 1;
                    }
                    left += 1;
                }
            }
            matrix
        }
    }
}

/// AI solution — direction vector simulation.
///
/// Use direction vectors and visited tracking to fill in spiral order.
///
/// Time: O(n^2).
/// Space: O(n^2).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Generate n x n spiral matrix.
        ///
        /// # Arguments
        /// * `n` - matrix dimension
        ///
        /// # Returns
        /// n x n matrix filled in spiral order
        pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
            let n = n as usize;
            let mut matrix = vec![vec![0i32; n]; n];
            let dirs: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            let (mut r, mut c, mut d) = (0i32, 0i32, 0usize);

            for num in 1..=(n * n) as i32 {
                matrix[r as usize][c as usize] = num;
                let nr = r + dirs[d].0;
                let nc = c + dirs[d].1;
                if nr < 0
                    || nr >= n as i32
                    || nc < 0
                    || nc >= n as i32
                    || matrix[nr as usize][nc as usize] != 0
                {
                    d = (d + 1) % 4;
                }
                r += dirs[d].0;
                c += dirs[d].1;
            }
            matrix
        }
    }
}

fn main() {
    for n in [1, 3, 4] {
        let my = my_solution::Solution::generate_matrix(n);
        let ai = ai_solution::Solution::generate_matrix(n);
        println!("n={n}: my:{my:?}");
        println!("n={n}: ai:{ai:?}");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_generate_matrix {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_n3() {
                    assert_eq!(
                        <$solution>::generate_matrix(3),
                        vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
                    );
                }

                #[test]
                fn test_n1() {
                    assert_eq!(<$solution>::generate_matrix(1), vec![vec![1]]);
                }

                #[test]
                fn test_n2() {
                    assert_eq!(
                        <$solution>::generate_matrix(2),
                        vec![vec![1, 2], vec![4, 3]]
                    );
                }

                #[test]
                fn test_n4() {
                    let m = <$solution>::generate_matrix(4);
                    assert_eq!(m[0], vec![1, 2, 3, 4]);
                    assert_eq!(m[3], vec![10, 9, 8, 7]);
                }
            }
        };
    }

    test_generate_matrix!(my, crate::my_solution::Solution);
    test_generate_matrix!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let n = 10;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::generate_matrix(std::hint::black_box(n)),
            );
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::generate_matrix(std::hint::black_box(n)),
            );
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
