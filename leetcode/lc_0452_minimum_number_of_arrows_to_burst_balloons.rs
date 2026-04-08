// LeetCode 452: Minimum Number of Arrows to Burst Balloons [Medium]
// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/
//
// Balloons are on a flat wall, each spanning [xstart, xend]. Arrows shot
// vertically burst all overlapping balloons. Return the minimum arrows needed.
//
// Examples:
//   [[10,16],[2,8],[1,6],[7,12]] -> 2
//   [[1,2],[3,4],[5,6],[7,8]] -> 4
//   [[1,2],[2,3],[3,4],[4,5]] -> 2
//
// Constraints:
//   1 <= points.length <= 10^5
//   points[i].length == 2
//   -2^31 <= xstart < xend <= 2^31 - 1
//
// Run:  cargo run --example lc_0452_minimum_number_of_arrows_to_burst_balloons
// Test: cargo test --example lc_0452_minimum_number_of_arrows_to_burst_balloons
// Bench: cargo test --example lc_0452_minimum_number_of_arrows_to_burst_balloons bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 223ns
//   AI solution: avg 213ns
//   Ratio: 1.05x (same algorithm)

/// My solution module — greedy: sort by end, count non-overlapping groups.
///
/// Time:  O(n log n)
/// Space: O(log n) for sorting

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
            // Greedy: sort by end, one arrow per non-overlapping group
            points.sort_unstable_by_key(|p| p[1]);
            let mut arrows = 1;
            let mut arrow_pos = points[0][1];
            for p in points.iter().skip(1) {
                if p[0] > arrow_pos {
                    arrows += 1;
                    arrow_pos = p[1];
                }
            }
            arrows
        }
    }
}

/// AI solution module — greedy (sort by right endpoint).
///
/// Sort balloons by end coordinate. Greedily shoot at each balloon's end
/// point, bursting all overlapping balloons.
///
/// Time:  O(n log n)
/// Space: O(log n)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
            points.sort_unstable_by_key(|p| p[1]);
            let mut ans = 0;
            let mut last = i64::MIN; // Use i64 to handle i32::MIN edge case
            for p in &points {
                let (a, b) = (p[0] as i64, p[1] as i64);
                if a > last {
                    ans += 1;
                    last = b;
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
        fn test_example1() {
            assert_eq!(
                Solution::find_min_arrow_shots(vec![
                    vec![10, 16],
                    vec![2, 8],
                    vec![1, 6],
                    vec![7, 12]
                ]),
                2
            );
        }
        #[test]
        fn test_no_overlap() {
            assert_eq!(
                Solution::find_min_arrow_shots(vec![
                    vec![1, 2],
                    vec![3, 4],
                    vec![5, 6],
                    vec![7, 8]
                ]),
                4
            );
        }
        #[test]
        fn test_touching() {
            assert_eq!(
                Solution::find_min_arrow_shots(vec![
                    vec![1, 2],
                    vec![2, 3],
                    vec![3, 4],
                    vec![4, 5]
                ]),
                2
            );
        }
        #[test]
        fn test_single() {
            assert_eq!(Solution::find_min_arrow_shots(vec![vec![1, 2]]), 1);
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
            vec![10, 16],
            vec![2, 8],
            vec![1, 6],
            vec![7, 12],
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
    bench_solution("My solution", my_solution::Solution::find_min_arrow_shots);
    bench_solution("AI solution", ai_solution::Solution::find_min_arrow_shots);
}

fn main() {
    println!(
        "{}",
        ai_solution::Solution::find_min_arrow_shots(vec![
            vec![10, 16],
            vec![2, 8],
            vec![1, 6],
            vec![7, 12]
        ])
    );
}
