// LeetCode 85: Maximal Rectangle [Hard]
// https://leetcode.com/problems/maximal-rectangle/
//
// First benchmark: 10_000 iterations, 2025-07-08
//   My solution avg: 6.077us
//   AI solution avg: 6.072us
//   Ratio: ~tied
//
// Given a rows x cols binary matrix of '0's and '1's, find the largest
// rectangle containing only '1's and return its area.
//
// Constraints: rows == matrix.length, cols == matrix[i].length,
//              1 <= rows, cols <= 200, matrix[i][j] is '0' or '1'
//
// Run:  cargo run --example lc_0085_maximal_rectangle
// Test: cargo test --example lc_0085_maximal_rectangle
// Bench: cargo test --example lc_0085_maximal_rectangle bench -- --nocapture

/// My solution — brute force: for each cell, expand rectangle.
///
/// For each (i, j), compute maximum rectangle with (i, j) as top-left corner.
///
/// Time: O(rows^2 * cols^2).
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the maximal rectangle of '1's.
        ///
        /// # Arguments
        /// * `matrix` - binary matrix of chars
        ///
        /// # Returns
        /// Area of largest rectangle of '1's
        pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
            if matrix.is_empty() || matrix[0].is_empty() {
                return 0;
            }
            let rows = matrix.len();
            let cols = matrix[0].len();
            let mut max_area = 0;

            for i in 0..rows {
                for j in 0..cols {
                    if matrix[i][j] == '0' {
                        continue;
                    }
                    // Track min width as we extend rows downward
                    let mut min_width = cols;
                    for r in i..rows {
                        if matrix[r][j] == '0' {
                            break;
                        }
                        // Find width of consecutive '1's starting at (r, j)
                        let mut w = 0;
                        while j + w < cols && matrix[r][j + w] == '1' {
                            w += 1;
                        }
                        min_width = min_width.min(w);
                        let height = r - i + 1;
                        max_area = max_area.max(min_width * height);
                    }
                }
            }
            max_area as i32
        }
    }
}

/// AI solution — histogram approach using monotonic stack per row.
///
/// Build histogram heights row by row, then apply LC 84 (largest
/// rectangle in histogram) for each row.
///
/// Time: O(rows * cols).
/// Space: O(cols).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the maximal rectangle of '1's.
        ///
        /// # Arguments
        /// * `matrix` - binary matrix of chars
        ///
        /// # Returns
        /// Area of largest rectangle of '1's
        pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
            if matrix.is_empty() || matrix[0].is_empty() {
                return 0;
            }
            let cols = matrix[0].len();
            let mut heights = vec![0i32; cols];
            let mut max_area = 0;

            for row in &matrix {
                // Update histogram heights
                for j in 0..cols {
                    heights[j] = if row[j] == '1' { heights[j] + 1 } else { 0 };
                }
                max_area = max_area.max(Self::largest_rect_in_histogram(&heights));
            }
            max_area
        }

        /// LC 84: largest rectangle in histogram using monotonic stack.
        fn largest_rect_in_histogram(heights: &[i32]) -> i32 {
            let n = heights.len();
            let mut stack: Vec<usize> = Vec::with_capacity(n);
            let mut max_area = 0;

            for i in 0..=n {
                let h = if i < n { heights[i] } else { 0 };
                while let Some(&top) = stack.last() {
                    if heights[top] <= h {
                        break;
                    }
                    stack.pop();
                    let w = if stack.is_empty() {
                        i as i32
                    } else {
                        (i - stack.last().expect("not empty") - 1) as i32
                    };
                    max_area = max_area.max(heights[top] * w);
                }
                stack.push(i);
            }
            max_area
        }
    }
}

fn main() {
    let matrix = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ];
    let my = my_solution::Solution::maximal_rectangle(matrix.clone());
    let ai = ai_solution::Solution::maximal_rectangle(matrix);
    println!("my: {my}, ai: {ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_maximal_rect {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    let matrix = vec![
                        vec!['1', '0', '1', '0', '0'],
                        vec!['1', '0', '1', '1', '1'],
                        vec!['1', '1', '1', '1', '1'],
                        vec!['1', '0', '0', '1', '0'],
                    ];
                    assert_eq!(<$solution>::maximal_rectangle(matrix), 6);
                }

                #[test]
                fn test_empty() {
                    assert_eq!(<$solution>::maximal_rectangle(vec![]), 0);
                }

                #[test]
                fn test_single_zero() {
                    assert_eq!(
                        <$solution>::maximal_rectangle(vec![vec!['0']]),
                        0
                    );
                }

                #[test]
                fn test_single_one() {
                    assert_eq!(
                        <$solution>::maximal_rectangle(vec![vec!['1']]),
                        1
                    );
                }

                #[test]
                fn test_all_ones() {
                    let matrix = vec![
                        vec!['1', '1'],
                        vec!['1', '1'],
                    ];
                    assert_eq!(<$solution>::maximal_rectangle(matrix), 4);
                }
            }
        };
    }

    test_maximal_rect!(my, crate::my_solution::Solution);
    test_maximal_rect!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 10_000;
        let matrix: Vec<Vec<char>> = (0..10)
            .map(|i| {
                (0..10)
                    .map(|j| if (i + j) % 3 != 0 { '1' } else { '0' })
                    .collect()
            })
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::maximal_rectangle(
                std::hint::black_box(matrix.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::maximal_rectangle(
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
