// LeetCode 447: Number of Boomerangs [Medium]
// https://leetcode.com/problems/number-of-boomerangs/
//
// Given n distinct points in a plane, a boomerang is a tuple (i, j, k) such
// that dist(i,j) == dist(i,k). Order matters. Return the number of boomerangs.
//
// Examples:
//   [[0,0],[1,0],[2,0]] -> 2  ([[1,0],[0,0],[2,0]] and [[1,0],[2,0],[0,0]])
//   [[1,1],[2,2],[3,3]] -> 2
//   [[1,1]] -> 0
//
// Constraints:
//   n == points.length
//   1 <= n <= 500
//   points[i].length == 2
//   -10^4 <= x_i, y_i <= 10^4
//   All points are unique
//
// Run:  cargo run --example lc_0447_number_of_boomerangs
// Test: cargo test --example lc_0447_number_of_boomerangs
// Bench: cargo test --example lc_0447_number_of_boomerangs bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 332ns
//   AI solution: avg 362ns
//   Ratio: 0.92x (same algorithm)

/// My solution module — hash map counting distances.
///
/// For each point i, compute squared distance to every other point j.
/// Group by distance. For each group of size x, there are x*(x-1) ordered
/// pairs (j, k).
///
/// Time:  O(n^2)
/// Space: O(n)  -- hash map per anchor point

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Count the number of boomerang tuples.
        pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
            use std::collections::HashMap;
            let mut count = 0;
            for i in 0..points.len() {
                let mut dist_map: HashMap<i64, i32> = HashMap::new();
                for j in 0..points.len() {
                    if i == j {
                        continue;
                    }
                    let dx = (points[i][0] - points[j][0]) as i64;
                    let dy = (points[i][1] - points[j][1]) as i64;
                    let d = dx * dx + dy * dy;
                    *dist_map.entry(d).or_insert(0) += 1;
                }
                for &v in dist_map.values() {
                    count += v * (v - 1); // permutations P(v, 2)
                }
            }
            count
        }
    }
}

/// AI solution module — incremental counting (doocs approach).
///
/// For each anchor point, as we encounter each new point at distance d,
/// add 2 * (existing count at d) to the answer. This avoids a second pass
/// through the hash map.
///
/// Time:  O(n^2)
/// Space: O(n)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Count the number of boomerang tuples.
        pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
            use std::collections::HashMap;

            let mut ans = 0i32;
            for p1 in &points {
                let mut cnt: HashMap<i64, i32> = HashMap::new();
                for p2 in &points {
                    // Squared distance avoids floating point
                    let dx = (p1[0] - p2[0]) as i64;
                    let dy = (p1[1] - p2[1]) as i64;
                    let d = dx * dx + dy * dy;
                    let prev = cnt.entry(d).or_insert(0);
                    // Each previously seen point at same distance forms
                    // 2 new boomerangs with the current point (j,k) and (k,j)
                    ans += 2 * *prev;
                    *prev += 1;
                }
            }
            ans
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;

        #[test]
        fn test_three_collinear() {
            assert_eq!(
                Solution::number_of_boomerangs(vec![vec![0, 0], vec![1, 0], vec![2, 0]]),
                2
            );
        }

        #[test]
        fn test_three_diagonal() {
            assert_eq!(
                Solution::number_of_boomerangs(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
                2
            );
        }

        #[test]
        fn test_single_point() {
            assert_eq!(Solution::number_of_boomerangs(vec![vec![1, 1]]), 0);
        }

        #[test]
        fn test_square() {
            // Square: (0,0),(0,1),(1,0),(1,1)
            // Each corner has 2 equidistant neighbors at distance 1
            // -> 4 corners * 2 boomerangs = 8
            assert_eq!(
                Solution::number_of_boomerangs(vec![
                    vec![0, 0],
                    vec![0, 1],
                    vec![1, 0],
                    vec![1, 1]
                ]),
                8
            );
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
    F: Fn(Vec<Vec<i32>>) -> i32,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(vec![
            vec![0, 0],
            vec![1, 0],
            vec![2, 0],
        ])));
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
    bench_solution("My solution", my_solution::Solution::number_of_boomerangs);
    bench_solution("AI solution", ai_solution::Solution::number_of_boomerangs);
}

fn main() {
    let result =
        ai_solution::Solution::number_of_boomerangs(vec![vec![0, 0], vec![1, 0], vec![2, 0]]);
    println!("Boomerangs: {}", result);
}
