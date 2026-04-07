// LeetCode 120: Triangle [Medium]
// https://leetcode.com/problems/triangle/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 7.438us
//   AI solution avg: 4.103us
//   Ratio: AI 1.81x faster
//
// Given a triangle array, return the minimum path sum from top to bottom.
// At each step you may move to an adjacent number on the row below (index i or i+1).
//
// Constraints: 1 <= triangle.length <= 200, triangle[i].length == i + 1,
// -10^4 <= triangle[i][j] <= 10^4
//
// Run:  cargo run --example lc_0120_triangle
// Test: cargo test --example lc_0120_triangle
// Bench: cargo test --example lc_0120_triangle bench -- --nocapture

/// My solution — bottom-up DP with 2D table.
///
/// Time: O(n^2) where n = number of rows.
/// Space: O(n^2).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return minimum path sum from top to bottom of triangle.
        pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
            let n = triangle.len();
            // dp[i][j] = min path sum reaching triangle[i][j]
            let mut dp = triangle.clone();
            for i in 1..n {
                for j in 0..=i {
                    let from_left = if j > 0 { dp[i - 1][j - 1] } else { i32::MAX };
                    let from_above = if j < i { dp[i - 1][j] } else { i32::MAX };
                    dp[i][j] = triangle[i][j] + from_left.min(from_above);
                }
            }
            *dp[n - 1].iter().min().expect("non-empty triangle")
        }
    }
}

/// AI solution — bottom-up DP starting from last row, O(n) space.
///
/// Time: O(n^2).
/// Space: O(n) — single row buffer.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return minimum path sum using O(n) space bottom-up.
        pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
            let n = triangle.len();
            // Start with last row
            let mut dp = triangle[n - 1].clone();
            // Work upward: for each cell, pick min of two children below
            for i in (0..n - 1).rev() {
                for j in 0..=i {
                    dp[j] = triangle[i][j] + dp[j].min(dp[j + 1]);
                }
            }
            dp[0]
        }
    }
}

fn main() {
    let tri = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
    let my = my_solution::Solution::minimum_total(tri.clone());
    let ai = ai_solution::Solution::minimum_total(tri);
    println!("min path sum: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_triangle {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    let tri = vec![
                        vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3],
                    ];
                    assert_eq!(<$solution>::minimum_total(tri), 11);
                }

                #[test]
                fn test_single() {
                    assert_eq!(<$solution>::minimum_total(vec![vec![-10]]), -10);
                }

                #[test]
                fn test_two_rows() {
                    assert_eq!(
                        <$solution>::minimum_total(vec![vec![-1], vec![2, 3]]),
                        1,
                    );
                }

                #[test]
                fn test_negative() {
                    let tri = vec![vec![-1], vec![2, -5], vec![1, -2, 3]];
                    assert_eq!(<$solution>::minimum_total(tri), -8);
                }

                #[test]
                fn test_all_same() {
                    let tri = vec![vec![1], vec![1, 1], vec![1, 1, 1], vec![1, 1, 1, 1]];
                    assert_eq!(<$solution>::minimum_total(tri), 4);
                }
            }
        };
    }

    test_triangle!(my, crate::my_solution::Solution);
    test_triangle!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let tri: Vec<Vec<i32>> = (0..50).map(|i| (0..=i).collect()).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::minimum_total(
                std::hint::black_box(tri.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::minimum_total(
                std::hint::black_box(tri.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );
        println!(
            "AI solution: {:.3}us avg",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
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
