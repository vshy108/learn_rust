// LeetCode 478: Generate Random Point in a Circle [Medium]
// https://leetcode.com/problems/generate-random-point-in-a-circle/
//
// Given the radius and (x,y) center of a circle, implement randPoint()
// that generates a uniform random point inside the circle.
//
// Examples:
//   Solution(1.0, 0.0, 0.0) -> randPoint() -> [x,y] where x^2+y^2 <= 1
//
// Constraints:
//   0 < radius <= 10^8
//   -10^7 <= x_center, y_center <= 10^7
//   At most 3 * 10^4 calls to randPoint
//
// Run:  cargo run --example lc_0478_generate_random_point_in_a_circle
// Test: cargo test --example lc_0478_generate_random_point_in_a_circle
// Bench: cargo test --example lc_0478_generate_random_point_in_a_circle bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 56ns
//   AI solution: avg 54ns
//   Ratio: 1.04x (rejection vs polar, similar)

/// My solution — rejection sampling.
///
/// Generate random (x,y) in the bounding square [-r,r] x [-r,r].
/// Accept if x^2 + y^2 <= r^2, else retry.
/// Expected acceptance rate: pi/4 ~ 78.5%.
///
/// Time:  O(1) expected
/// Space: O(1)
#[allow(dead_code)]
mod my_solution {
    use rand::Rng;

    /// Circle for random point generation using rejection sampling.
    pub struct Solution {
        radius: f64,
        x_center: f64,
        y_center: f64,
    }

    impl Solution {
        /// Create a new circle.
        ///
        /// # Arguments
        /// * `radius` - circle radius
        /// * `x_center` - x coordinate of center
        /// * `y_center` - y coordinate of center
        pub fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
            Solution {
                radius,
                x_center,
                y_center,
            }
        }

        /// Generate a uniform random point inside the circle.
        ///
        /// # Returns
        /// [x, y] coordinates of the random point.
        pub fn rand_point(&self) -> Vec<f64> {
            let mut rng = rand::rng();
            loop {
                // Random point in [-1, 1] x [-1, 1]
                let x: f64 = rng.random_range(-1.0..1.0);
                let y: f64 = rng.random_range(-1.0..1.0);
                if x * x + y * y <= 1.0 {
                    return vec![
                        self.x_center + x * self.radius,
                        self.y_center + y * self.radius,
                    ];
                }
            }
        }
    }
}

/// AI solution — polar coordinates with sqrt for uniform distribution.
///
/// Generate angle theta in [0, 2*pi) and radius r = R * sqrt(uniform(0,1)).
/// The sqrt ensures uniform area distribution (not clustered at center).
///
/// Time:  O(1) — no rejection needed
/// Space: O(1)
mod ai_solution {
    use rand::Rng;

    /// Circle for random point generation using polar coordinates.
    pub struct Solution {
        radius: f64,
        x_center: f64,
        y_center: f64,
    }

    impl Solution {
        pub fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
            Solution {
                radius,
                x_center,
                y_center,
            }
        }

        /// Generate point using polar coordinates with sqrt correction.
        pub fn rand_point(&self) -> Vec<f64> {
            let mut rng = rand::rng();
            let theta: f64 = rng.random_range(0.0..std::f64::consts::TAU);
            // sqrt ensures uniform distribution over the area
            let r = self.radius * rng.random_range(0.0_f64..1.0).sqrt();
            vec![
                self.x_center + r * theta.cos(),
                self.y_center + r * theta.sin(),
            ]
        }
    }
}

fn main() {
    let my_sol = my_solution::Solution::new(1.0, 0.0, 0.0);
    println!("my:  {:?}", my_sol.rand_point());
    let ai_sol = ai_solution::Solution::new(1.0, 0.0, 0.0);
    println!("ai:  {:?}", ai_sol.rand_point());
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;

        #[test]
        fn test_points_in_circle() {
            let sol = Solution::new(1.0, 0.0, 0.0);
            for _ in 0..1000 {
                let p = sol.rand_point();
                let dist_sq = p[0] * p[0] + p[1] * p[1];
                assert!(
                    dist_sq <= 1.0 + 1e-9,
                    "point ({}, {}) outside unit circle, dist^2 = {}",
                    p[0],
                    p[1],
                    dist_sq
                );
            }
        }

        #[test]
        fn test_offset_circle() {
            let sol = Solution::new(2.0, 5.0, 3.0);
            for _ in 0..500 {
                let p = sol.rand_point();
                let dx = p[0] - 5.0;
                let dy = p[1] - 3.0;
                let dist_sq = dx * dx + dy * dy;
                assert!(
                    dist_sq <= 4.0 + 1e-9,
                    "point outside circle(r=2, cx=5, cy=3)"
                );
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
    F: Fn() -> Vec<f64>,
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
    let my_sol = my_solution::Solution::new(1.0, 0.0, 0.0);
    bench_solution("My solution", || my_sol.rand_point());
    let ai_sol = ai_solution::Solution::new(1.0, 0.0, 0.0);
    bench_solution("AI solution", || ai_sol.rand_point());
}
