// LeetCode 436: Find Right Interval [Medium]
// https://leetcode.com/problems/find-right-interval/
//
// Given an array of intervals where intervals[i] = [start_i, end_i] and each
// start_i is unique, the right interval for interval i is interval j such that
// start_j >= end_i and start_j is minimized. Return an array of right interval
// indices. If no right interval exists, put -1.
//
// Examples:
//   [[1,2]] -> [-1]
//   [[3,4],[2,3],[1,2]] -> [-1,0,1]
//   [[1,4],[2,3],[3,4]] -> [-1,2,-1]
//
// Constraints:
//   1 <= intervals.length <= 2 * 10^4
//   intervals[i].length == 2
//   -10^6 <= start_i <= end_i <= 10^6
//   The start point of each interval is unique
//
// Run:  cargo run --example lc_0436_find_right_interval
// Test: cargo test --example lc_0436_find_right_interval
// Bench: cargo test --example lc_0436_find_right_interval bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 279ns
//   AI solution: avg 275ns
//   Ratio: 1.01x (same algorithm)

/// My solution module — sort + binary search.
///
/// Store (start, original_index) pairs, sort by start. For each interval's
/// end value, binary search for the first start >= end.
///
/// Time:  O(n log n) -- sorting + n binary searches
/// Space: O(n)       -- sorted array of pairs

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the right interval index for each interval.
        ///
        /// # Arguments
        ///
        /// * `intervals` - Vector of [start, end] pairs with unique starts
        ///
        /// # Returns
        ///
        /// Vector of right interval indices (-1 if none exists)
        pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
            let n = intervals.len();
            // Store (start, original_index), sorted by start
            let mut arr: Vec<(i32, usize)> = intervals
                .iter()
                .enumerate()
                .map(|(i, v)| (v[0], i))
                .collect();
            arr.sort_unstable();
            intervals
                .iter()
                .map(|iv| {
                    let end = iv[1];
                    // Binary search for first start >= end
                    let j = arr.partition_point(|&(s, _)| s < end);
                    if j < n { arr[j].1 as i32 } else { -1 }
                })
                .collect()
        }
    }
}

/// AI solution module — sort + binary search with partition_point.
///
/// Same approach but uses Rust's slice::partition_point for cleaner
/// binary search (equivalent to lower_bound).
///
/// Time:  O(n log n) -- sorting + n binary searches
/// Space: O(n)       -- sorted array of pairs
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the right interval index for each interval.
        ///
        /// # Arguments
        ///
        /// * `intervals` - Vector of [start, end] pairs with unique starts
        ///
        /// # Returns
        ///
        /// Vector of right interval indices (-1 if none exists)
        pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
            let n = intervals.len();
            // Collect (start_value, original_index) and sort by start
            let mut arr: Vec<(i32, usize)> = intervals
                .iter()
                .enumerate()
                .map(|(i, iv)| (iv[0], i))
                .collect();
            arr.sort_unstable();

            intervals
                .iter()
                .map(|iv| {
                    let end = iv[1];
                    // Find first start >= end using partition_point
                    let j = arr.partition_point(|&(st, _)| st < end);
                    if j < n { arr[j].1 as i32 } else { -1 }
                })
                .collect()
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;

        #[test]
        fn test_single_interval() {
            assert_eq!(Solution::find_right_interval(vec![vec![1, 2]]), vec![-1]);
        }

        #[test]
        fn test_three_intervals() {
            assert_eq!(
                Solution::find_right_interval(vec![vec![3, 4], vec![2, 3], vec![1, 2]]),
                vec![-1, 0, 1]
            );
        }

        #[test]
        fn test_overlapping() {
            assert_eq!(
                Solution::find_right_interval(vec![vec![1, 4], vec![2, 3], vec![3, 4]]),
                vec![-1, 2, -1]
            );
        }

        #[test]
        fn test_self_right() {
            // An interval can be its own right interval
            assert_eq!(Solution::find_right_interval(vec![vec![1, 1]]), vec![0]);
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
    F: Fn(Vec<Vec<i32>>) -> Vec<i32>,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(vec![
            vec![3, 4],
            vec![2, 3],
            vec![1, 2],
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
    bench_solution("My solution", my_solution::Solution::find_right_interval);
    bench_solution("AI solution", ai_solution::Solution::find_right_interval);
}

fn main() {
    let result =
        ai_solution::Solution::find_right_interval(vec![vec![3, 4], vec![2, 3], vec![1, 2]]);
    println!("Right intervals: {:?}", result);
}
