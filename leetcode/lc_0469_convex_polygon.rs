// LeetCode 469: Convex Polygon [Medium]
// https://leetcode.com/problems/convex-polygon/
//
// Given an array of points on the X-Y plane, the points form a polygon when
// joined sequentially. Return true if the polygon is convex, false otherwise.
//
// Examples:
//   points = [[0,0],[0,5],[5,5],[5,0]] -> true
//   points = [[0,0],[0,10],[10,10],[10,0],[5,5]] -> false
//
// Constraints:
//   3 <= points.length <= 10^4
//   points[i].length == 2
//   -10^4 <= xi, yi <= 10^4
//   All the given points are unique.
//
// Run:  cargo run --example lc_0469_convex_polygon
// Test: cargo test --example lc_0469_convex_polygon
// Bench: cargo test --example lc_0469_convex_polygon bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 5.489us
//   AI solution: avg 5.596us
//   Ratio: 0.98x (same algorithm, i64 vs i128)

/// Time: O(n), Space: O(1)
/// Cross product approach: for each consecutive triple of vertices,
/// compute cross product. If sign ever flips, polygon is not convex.
#[allow(dead_code)]
mod my_solution {
    pub struct Solution;

    impl Solution {
        pub fn is_convex(points: Vec<Vec<i32>>) -> bool {
            let n = points.len();
            // Track sign of cross products (positive, negative, or zero)
            let mut prev: i64 = 0;
            for i in 0..n {
                let x1 = (points[(i + 1) % n][0] - points[i][0]) as i64;
                let y1 = (points[(i + 1) % n][1] - points[i][1]) as i64;
                let x2 = (points[(i + 2) % n][0] - points[i][0]) as i64;
                let y2 = (points[(i + 2) % n][1] - points[i][1]) as i64;
                // Cross product of vectors (p[i]->p[i+1]) and (p[i]->p[i+2])
                let cur = x1 * y2 - x2 * y1;
                if cur != 0 {
                    if cur * prev < 0 {
                        return false;
                    }
                    prev = cur;
                }
            }
            true
        }
    }
}

/// Time: O(n), Space: O(1)
/// Same cross-product logic but uses i128 to avoid any overflow
/// concerns, and tracks sign as an enum for clarity.
#[allow(dead_code)]
mod ai_solution {
    pub struct Solution;

    impl Solution {
        pub fn is_convex(points: Vec<Vec<i32>>) -> bool {
            let n = points.len();
            let mut sign = 0i8; // -1 for negative, 0 for undetermined, 1 for positive
            for i in 0..n {
                let a = &points[i];
                let b = &points[(i + 1) % n];
                let c = &points[(i + 2) % n];
                // Cross product: (b-a) x (c-a)
                let cross = (b[0] as i128 - a[0] as i128) * (c[1] as i128 - a[1] as i128)
                    - (c[0] as i128 - a[0] as i128) * (b[1] as i128 - a[1] as i128);
                if cross > 0 {
                    if sign == -1 {
                        return false;
                    }
                    sign = 1;
                } else if cross < 0 {
                    if sign == 1 {
                        return false;
                    }
                    sign = -1;
                }
                // cross == 0 means collinear, which is fine for convexity
            }
            true
        }
    }
}

fn main() {
    println!("LeetCode 469: Convex Polygon");
    let cases = vec![
        (vec![vec![0, 0], vec![0, 5], vec![5, 5], vec![5, 0]], true),
        (
            vec![
                vec![0, 0],
                vec![0, 10],
                vec![10, 10],
                vec![10, 0],
                vec![5, 5],
            ],
            false,
        ),
    ];
    for (pts, expected) in &cases {
        let r1 = my_solution::Solution::is_convex(pts.clone());
        let r2 = ai_solution::Solution::is_convex(pts.clone());
        println!("  is_convex => my={r1}, ai={r2} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_both {
        ($name:ident, $points:expr, $expected:expr) => {
            mod $name {
                use super::*;
                #[test]
                fn my() {
                    assert_eq!(my_solution::Solution::is_convex($points), $expected);
                }
                #[test]
                fn ai() {
                    assert_eq!(ai_solution::Solution::is_convex($points), $expected);
                }
            }
        };
    }

    test_both!(
        square,
        vec![vec![0, 0], vec![0, 5], vec![5, 5], vec![5, 0]],
        true
    );
    test_both!(
        concave,
        vec![
            vec![0, 0],
            vec![0, 10],
            vec![10, 10],
            vec![10, 0],
            vec![5, 5]
        ],
        false
    );
    test_both!(triangle, vec![vec![0, 0], vec![1, 0], vec![0, 1]], true);
    // L-shaped concave polygon
    test_both!(
        l_shape,
        vec![
            vec![0, 0],
            vec![2, 0],
            vec![2, 1],
            vec![1, 1],
            vec![1, 2],
            vec![0, 2]
        ],
        false
    );
    // Regular pentagon-like (convex)
    test_both!(
        pentagon,
        vec![vec![0, 2], vec![2, 0], vec![4, 1], vec![3, 4], vec![1, 4]],
        true
    );
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(Vec<Vec<i32>>) -> bool,
{
    let iterations = 500_000;
    // Circle approximation (always convex)
    let pts: Vec<Vec<i32>> = (0..100)
        .map(|i| {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / 100.0;
            vec![(1000.0 * angle.cos()) as i32, (1000.0 * angle.sin()) as i32]
        })
        .collect();
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(pts.clone())));
    }
    let elapsed = start.elapsed();
    println!(
        "{name}: {iterations} iterations in {elapsed:?} (avg {:?})",
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::is_convex);
    bench_solution("AI solution", ai_solution::Solution::is_convex);
}
