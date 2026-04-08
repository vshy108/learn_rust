// LeetCode 435: Non-overlapping Intervals [Medium]
// https://leetcode.com/problems/non-overlapping-intervals/
//
// Given an array of intervals where intervals[i] = [start_i, end_i], return
// the minimum number of intervals you need to remove to make the rest
// non-overlapping. Intervals that only touch at a point are non-overlapping.
//
// Examples:
//   [[1,2],[2,3],[3,4],[1,3]] -> 1  (remove [1,3])
//   [[1,2],[1,2],[1,2]] -> 2
//   [[1,2],[2,3]] -> 0
//
// Constraints:
//   1 <= intervals.length <= 10^5
//   intervals[i].length == 2
//   -5 * 10^4 <= start_i < end_i <= 5 * 10^4
//
// Run:  cargo run --example lc_0435_non_overlapping_intervals
// Test: cargo test --example lc_0435_non_overlapping_intervals
// Bench: cargo test --example lc_0435_non_overlapping_intervals bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 213ns
//   AI solution: avg 212ns
//   Ratio: 1.00x (same algorithm)

/// My solution module — greedy: sort by end, count kept.
///
/// Sort intervals by right boundary. Greedily keep intervals whose
/// left boundary >= the previous kept interval's right boundary.
/// Answer = total - kept.
///
/// Time:  O(n log n) -- dominated by sorting
/// Space: O(log n)   -- sorting stack

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find minimum intervals to remove for non-overlap.
        ///
        /// # Arguments
        ///
        /// * `intervals` - Vector of [start, end] pairs
        ///
        /// # Returns
        ///
        /// Minimum number of intervals to remove
        pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
            // Greedy: sort by end, count non-overlapping
            intervals.sort_unstable_by_key(|v| v[1]);
            let mut ans = intervals.len() as i32;
            let mut prev_end = i32::MIN;
            for iv in &intervals {
                if prev_end <= iv[0] {
                    ans -= 1;
                    prev_end = iv[1];
                }
            }
            ans
        }
    }
}

/// AI solution module — same greedy, count removals directly.
///
/// Sort by end. Track the right boundary of the last kept interval.
/// When overlap is found, increment removal count (skip current interval).
///
/// Time:  O(n log n) -- dominated by sorting
/// Space: O(log n)   -- sorting stack
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find minimum intervals to remove for non-overlap.
        ///
        /// # Arguments
        ///
        /// * `intervals` - Vector of [start, end] pairs
        ///
        /// # Returns
        ///
        /// Minimum number of intervals to remove
        pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
            // Sort by right boundary for greedy selection
            intervals.sort_unstable_by_key(|iv| iv[1]);
            let mut ans = intervals.len() as i32;
            let mut pre = i32::MIN;
            for iv in &intervals {
                let (l, r) = (iv[0], iv[1]);
                if pre <= l {
                    // No overlap: keep this interval
                    ans -= 1;
                    pre = r;
                }
                // Else: overlap, skip (remove) this interval
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
        fn test_remove_one() {
            assert_eq!(
                Solution::erase_overlap_intervals(vec![
                    vec![1, 2],
                    vec![2, 3],
                    vec![3, 4],
                    vec![1, 3]
                ]),
                1
            );
        }

        #[test]
        fn test_remove_two() {
            assert_eq!(
                Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]]),
                2
            );
        }

        #[test]
        fn test_no_removal() {
            assert_eq!(
                Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]),
                0
            );
        }

        #[test]
        fn test_single_interval() {
            assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2]]), 0);
        }

        #[test]
        fn test_all_same() {
            assert_eq!(
                Solution::erase_overlap_intervals(vec![
                    vec![0, 1],
                    vec![0, 1],
                    vec![0, 1],
                    vec![0, 1]
                ]),
                3
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
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![1, 3],
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
    bench_solution(
        "My solution",
        my_solution::Solution::erase_overlap_intervals,
    );
    bench_solution(
        "AI solution",
        ai_solution::Solution::erase_overlap_intervals,
    );
}

fn main() {
    let result = ai_solution::Solution::erase_overlap_intervals(vec![
        vec![1, 2],
        vec![2, 3],
        vec![3, 4],
        vec![1, 3],
    ]);
    println!("Intervals to remove: {}", result);
}
