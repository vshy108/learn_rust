// LeetCode 149: Max Points on a Line [Hard]
// https://leetcode.com/problems/max-points-on-a-line/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 8.114us
//   AI solution avg: 7.844us
//   Ratio: ~tied
//
// Given an array of points on the X-Y plane, return the maximum number of
// points that lie on the same straight line.
//
// Constraints: 1 <= points.length <= 300, -10^4 <= xi, yi <= 10^4
//   All points are unique
//
// Run:  cargo run --example lc_0149_max_points_on_line
// Test: cargo test --example lc_0149_max_points_on_line
// Bench: cargo test --example lc_0149_max_points_on_line bench -- --nocapture

use std::collections::HashMap;

/// My solution — for each point, group slopes as (dy/gcd, dx/gcd) fractions.
///
/// Time: O(n^2).
/// Space: O(n) for the slope map.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return maximum points on a single line.
        pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
            let n = points.len();
            if n <= 2 { return n as i32; }

            let mut global_max = 2;
            for i in 0..n {
                let mut slopes: HashMap<(i32, i32), i32> = HashMap::new();
                for j in i + 1..n {
                    let dx = points[j][0] - points[i][0];
                    let dy = points[j][1] - points[i][1];
                    let key = Self::normalize(dx, dy);
                    let count = slopes.entry(key).or_insert(1);
                    *count += 1;
                    global_max = global_max.max(*count);
                }
            }
            global_max
        }

        /// Normalize slope to a canonical (dy, dx) pair using GCD.
        fn normalize(mut dx: i32, mut dy: i32) -> (i32, i32) {
            if dx == 0 {
                return (1, 0); // vertical line
            }
            if dy == 0 {
                return (0, 1); // horizontal line
            }
            // Ensure consistent sign: make dx positive
            if dx < 0 {
                dx = -dx;
                dy = -dy;
            }
            let g = Self::gcd(dx.abs(), dy.abs());
            (dy / g, dx / g)
        }

        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 { a } else { Self::gcd(b, a % b) }
        }
    }
}

/// AI solution — same approach but uses i64 cross product to avoid GCD.
///
/// Time: O(n^2).
/// Space: O(n) for the slope map.
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return maximum points on a single line.
        pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
            let n = points.len();
            if n <= 2 { return n as i32; }

            let mut global_max = 2;
            for i in 0..n {
                // Same GCD-normalized slope approach, slightly different normalization
                let mut slopes: HashMap<(i32, i32), i32> = HashMap::new();
                for j in i + 1..n {
                    let dx = points[j][0] - points[i][0];
                    let dy = points[j][1] - points[i][1];

                    let g = Self::gcd(dx.abs(), dy.abs());
                    let (mut ndx, mut ndy) = (dx / g, dy / g);
                    // Canonical form: first non-zero component is positive
                    if ndx < 0 || (ndx == 0 && ndy < 0) {
                        ndx = -ndx;
                        ndy = -ndy;
                    }

                    let count = slopes.entry((ndy, ndx)).or_insert(1);
                    *count += 1;
                    global_max = global_max.max(*count);
                }
            }
            global_max
        }

        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 { a } else { Self::gcd(b, a % b) }
        }
    }
}

fn main() {
    let my = my_solution::Solution::max_points(
        vec![vec![1, 1], vec![2, 2], vec![3, 3]],
    );
    let ai = ai_solution::Solution::max_points(
        vec![vec![1, 1], vec![2, 2], vec![3, 3]],
    );
    println!("max points: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_max_points {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(
                        <$solution>::max_points(
                            vec![vec![1, 1], vec![2, 2], vec![3, 3]],
                        ),
                        3,
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        <$solution>::max_points(vec![
                            vec![1, 1], vec![3, 2], vec![5, 3],
                            vec![4, 1], vec![2, 3], vec![1, 4],
                        ]),
                        4,
                    );
                }

                #[test]
                fn test_single_point() {
                    assert_eq!(
                        <$solution>::max_points(vec![vec![0, 0]]),
                        1,
                    );
                }

                #[test]
                fn test_two_points() {
                    assert_eq!(
                        <$solution>::max_points(
                            vec![vec![0, 0], vec![1, 1]],
                        ),
                        2,
                    );
                }

                #[test]
                fn test_vertical() {
                    assert_eq!(
                        <$solution>::max_points(
                            vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![1, 0]],
                        ),
                        3,
                    );
                }
            }
        };
    }

    test_max_points!(my, crate::my_solution::Solution);
    test_max_points!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let points: Vec<Vec<i32>> = (0..30)
            .map(|i| vec![i * 3 + 1, i * 2 + 7])
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::max_points(
                std::hint::black_box(points.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::max_points(
                std::hint::black_box(points.clone()),
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
            println!("Both solutions are roughly equal");
        }
    }
}
