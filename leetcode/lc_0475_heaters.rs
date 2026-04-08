// LeetCode 475: Heaters [Medium]
// https://leetcode.com/problems/heaters/
//
// You are given positions of houses and heaters on a number line. Find the
// minimum radius such that all houses are covered by at least one heater.
//
// Examples:
//   houses = [1,2,3], heaters = [2]       -> 1
//   houses = [1,2,3,4], heaters = [1,4]   -> 1
//   houses = [1,5], heaters = [2]         -> 3
//
// Constraints:
//   1 <= houses.len(), heaters.len() <= 3 * 10^4
//   1 <= houses[i], heaters[i] <= 10^9
//
// Run:  cargo run --example lc_0475_heaters
// Test: cargo test --example lc_0475_heaters
// Bench: cargo test --example lc_0475_heaters bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 84ns
//   AI solution: avg 85ns
//   Ratio: 0.99x (same algorithm)

/// My solution module — binary search.
///
/// Time:  O((m + n) log n) — sort heaters, binary search per house
/// Space: O(1) — in-place sort

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_radius(houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
            heaters.sort_unstable();
            let mut radius = 0;
            for &house in &houses {
                let idx = heaters.partition_point(|&h| h < house);
                let right_dist = if idx < heaters.len() {
                    heaters[idx] - house
                } else {
                    i32::MAX
                };
                let left_dist = if idx > 0 {
                    house - heaters[idx - 1]
                } else {
                    i32::MAX
                };
                radius = radius.max(left_dist.min(right_dist));
            }
            radius
        }
    }
}

/// AI solution module — sort heaters, binary search for nearest heater
/// for each house, then take the maximum of the minimum distances.
///
/// Time:  O((m + n) log n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_radius(houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
            heaters.sort_unstable();
            let mut radius = 0i32;
            for &house in &houses {
                // Find insertion point for house in heaters
                let idx = heaters.partition_point(|&h| h < house);
                // Distance to the heater on the right (if exists)
                let right_dist = if idx < heaters.len() {
                    heaters[idx] - house
                } else {
                    i32::MAX
                };
                // Distance to the heater on the left (if exists)
                let left_dist = if idx > 0 {
                    house - heaters[idx - 1]
                } else {
                    i32::MAX
                };
                // Minimum distance to nearest heater
                let min_dist = left_dist.min(right_dist);
                radius = radius.max(min_dist);
            }
            radius
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_single_heater() {
            assert_eq!(Solution::find_radius(vec![1, 2, 3], vec![2]), 1);
        }
        #[test]
        fn test_two_heaters() {
            assert_eq!(Solution::find_radius(vec![1, 2, 3, 4], vec![1, 4]), 1);
        }
        #[test]
        fn test_far() {
            assert_eq!(Solution::find_radius(vec![1, 5], vec![2]), 3);
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
    F: Fn(Vec<i32>, Vec<i32>) -> i32,
{
    let iterations = 500_000;
    let houses = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let heaters = vec![1, 5, 10];
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(
            std::hint::black_box(houses.clone()),
            std::hint::black_box(heaters.clone()),
        ));
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
    bench_solution("My solution", my_solution::Solution::find_radius);
    bench_solution("AI solution", ai_solution::Solution::find_radius);
}

fn main() {
    println!(
        "{}",
        ai_solution::Solution::find_radius(vec![1, 2, 3], vec![2])
    );
}
