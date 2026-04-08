// LeetCode 497: Random Point in Non-overlapping Rectangles [Medium]
// https://leetcode.com/problems/random-point-in-non-overlapping-rectangles/
//
// Given a list of non-overlapping axis-aligned rectangles rects, where
// rects[i] = [ai, bi, xi, yi] represents (bottom-left, top-right),
// implement pick() that randomly and uniformly picks an integer point
// in the covered space.
//
// Examples:
//   Solution([[-2,-2,1,1],[2,2,4,6]]) -> pick() -> [x,y] in one of the rects
//
// Constraints:
//   1 <= rects.length <= 100
//   rects[i].length == 4
//   -10^9 <= ai < xi <= 10^9
//   -10^9 <= bi < yi <= 10^9
//   At most 10^4 calls to pick
//
// Run:  cargo run --example lc_0497_random_point_in_non_overlapping_rectangles
// Test: cargo test --example lc_0497_random_point_in_non_overlapping_rectangles
// Bench: cargo test --example lc_0497_random_point_in_non_overlapping_rectangles bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 41ns
//   AI solution: avg 46ns
//   Ratio: 0.89x (similar binary search)

/// My solution — prefix sum + binary search for weighted random rect selection.
///
/// Each rectangle has area = (x2-x1+1)*(y2-y1+1) integer points.
/// Build a prefix sum of areas, pick a random point index, binary search
/// to find which rect it falls in, then compute the exact (x,y).
///
/// Time:  O(n) init, O(log n) per pick
/// Space: O(n)
#[allow(dead_code)]
mod my_solution {
    use rand::Rng;

    /// Weighted random picker for non-overlapping rectangles.
    pub struct Solution {
        rects: Vec<Vec<i32>>,
        prefix: Vec<i64>,
        total: i64,
    }

    impl Solution {
        /// Initialize with a list of rectangles [a, b, x, y].
        ///
        /// # Arguments
        /// * `rects` - non-overlapping rectangles
        pub fn new(rects: Vec<Vec<i32>>) -> Self {
            let mut prefix = Vec::with_capacity(rects.len());
            let mut sum: i64 = 0;
            for r in &rects {
                // Number of integer points in this rectangle
                let area = (r[2] - r[0] + 1) as i64 * (r[3] - r[1] + 1) as i64;
                sum += area;
                prefix.push(sum);
            }
            Solution {
                rects,
                prefix,
                total: sum,
            }
        }

        /// Pick a random integer point uniformly from all rectangles.
        ///
        /// # Returns
        /// [x, y] coordinates of the picked point.
        pub fn pick(&self) -> Vec<i32> {
            let mut rng = rand::rng();
            let target = rng.random_range(1..=self.total);
            // Binary search for the rectangle containing this point index
            let idx = self.prefix.partition_point(|&p| p < target);
            let r = &self.rects[idx];
            let (a, b, x, _y) = (r[0], r[1], r[2], r[3]);
            let width = x - a + 1;
            // Offset within this rectangle
            let prev = if idx == 0 { 0 } else { self.prefix[idx - 1] };
            let offset = (target - prev - 1) as i32;
            vec![a + offset % width, b + offset / width]
        }
    }
}

/// AI solution — area-weighted random with uniform rect-internal pick.
///
/// Same prefix sum approach but picks the rect first, then generates
/// random (x, y) within that rect independently (simpler per-pick logic).
///
/// Time:  O(n) init, O(log n) per pick
/// Space: O(n)
mod ai_solution {
    use rand::Rng;

    pub struct Solution {
        rects: Vec<Vec<i32>>,
        prefix: Vec<i64>,
        total: i64,
    }

    impl Solution {
        pub fn new(rects: Vec<Vec<i32>>) -> Self {
            let mut prefix = Vec::with_capacity(rects.len());
            let mut sum: i64 = 0;
            for r in &rects {
                sum += (r[2] - r[0] + 1) as i64 * (r[3] - r[1] + 1) as i64;
                prefix.push(sum);
            }
            Solution {
                rects,
                prefix,
                total: sum,
            }
        }

        /// Pick using separate rect selection + internal random point.
        pub fn pick(&self) -> Vec<i32> {
            let mut rng = rand::rng();
            let target = rng.random_range(1..=self.total);
            let idx = self.prefix.partition_point(|&p| p < target);
            let r = &self.rects[idx];
            // Generate random point within this specific rectangle
            let x = rng.random_range(r[0]..=r[2]);
            let y = rng.random_range(r[1]..=r[3]);
            vec![x, y]
        }
    }
}

fn main() {
    let rects = vec![vec![-2, -2, 1, 1], vec![2, 2, 4, 6]];
    let my_sol = my_solution::Solution::new(rects.clone());
    println!("my:  {:?}", my_sol.pick());
    let ai_sol = ai_solution::Solution::new(rects);
    println!("ai:  {:?}", ai_sol.pick());
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;

        #[test]
        fn test_points_in_rects() {
            let rects = vec![vec![-2, -2, 1, 1], vec![2, 2, 4, 6]];
            let sol = Solution::new(rects.clone());
            for _ in 0..500 {
                let p = sol.pick();
                let in_rect = rects
                    .iter()
                    .any(|r| p[0] >= r[0] && p[0] <= r[2] && p[1] >= r[1] && p[1] <= r[3]);
                assert!(in_rect, "point {:?} not in any rect", p);
            }
        }

        #[test]
        fn test_single_rect() {
            let rects = vec![vec![0, 0, 0, 0]]; // single point (0,0)
            let sol = Solution::new(rects);
            for _ in 0..10 {
                assert_eq!(sol.pick(), vec![0, 0]);
            }
        }
    };
}

#[cfg(test)]
mod tests_my {
    solution_tests!(my_solution);
}
#[cfg(test)]
mod tests_ai {
    solution_tests!(ai_solution);
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn() -> Vec<i32>,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f());
    }
    let elapsed = start.elapsed();
    println!(
        "{}: {} iterations in {:?} (avg {:?})",
        name,
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    let rects = vec![vec![-2, -2, 1, 1], vec![2, 2, 4, 6]];
    let my_sol = my_solution::Solution::new(rects.clone());
    bench_solution("My solution", || my_sol.pick());
    let ai_sol = ai_solution::Solution::new(rects);
    bench_solution("AI solution", || ai_sol.pick());
}
