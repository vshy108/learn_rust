// LeetCode 56: Merge Intervals [Medium]
// https://leetcode.com/problems/merge-intervals/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 11.614us
//   AI solution avg: 11.241us
//   Ratio: ~tied
//
// Given an array of intervals where intervals[i] = [start_i, end_i],
// merge all overlapping intervals.
//
// Example 1: intervals = [[1,3],[2,6],[8,10],[15,18]] -> [[1,6],[8,10],[15,18]]
// Example 2: intervals = [[1,4],[4,5]] -> [[1,5]]
//
// Constraints:
//   1 <= intervals.length <= 10^4
//   intervals[i].length == 2
//   0 <= start_i <= end_i <= 10^4
//
// Run:  cargo run --example lc_0056_merge_intervals
// Test: cargo test --example lc_0056_merge_intervals
// Bench: cargo test --example lc_0056_merge_intervals bench -- --nocapture

/// My solution — sort then merge linearly.
///
/// Sort intervals by start time. Iterate and merge overlapping intervals
/// by extending the end of the last merged interval.
///
/// Time: O(n log n) for sorting + O(n) merge pass.
/// Space: O(n) for output.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Merge overlapping intervals.
        ///
        /// # Arguments
        /// * `intervals` - array of [start, end] intervals
        ///
        /// # Returns
        /// Merged non-overlapping intervals
        pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            intervals.sort_unstable_by_key(|i| i[0]);
            let mut merged: Vec<Vec<i32>> = Vec::new();
            for interval in intervals {
                if let Some(last) = merged.last_mut() {
                    if interval[0] <= last[1] {
                        // Overlapping — extend end
                        last[1] = last[1].max(interval[1]);
                        continue;
                    }
                }
                merged.push(interval);
            }
            merged
        }
    }
}

/// AI solution — same approach (this is the canonical optimal solution).
///
/// Sort + single-pass merge. Both solutions are identical in approach.
///
/// Time: O(n log n).
/// Space: O(n).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Merge overlapping intervals.
        ///
        /// # Arguments
        /// * `intervals` - array of [start, end] intervals
        ///
        /// # Returns
        /// Merged non-overlapping intervals
        pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            intervals.sort_unstable_by_key(|i| i[0]);
            let mut merged: Vec<Vec<i32>> = Vec::with_capacity(intervals.len());
            for interval in intervals {
                if let Some(last) = merged.last_mut() {
                    if interval[0] <= last[1] {
                        last[1] = last[1].max(interval[1]);
                        continue;
                    }
                }
                merged.push(interval);
            }
            merged
        }
    }
}

fn main() {
    let cases = [
        vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]],
        vec![vec![1, 4], vec![4, 5]],
    ];
    for intervals in &cases {
        println!(
            "{intervals:?} -> my:{:?}, ai:{:?}",
            my_solution::Solution::merge(intervals.clone()),
            ai_solution::Solution::merge(intervals.clone()),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_merge {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_basic() {
                    assert_eq!(
                        <$solution>::merge(vec![
                            vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]
                        ]),
                        vec![vec![1, 6], vec![8, 10], vec![15, 18]]
                    );
                }

                #[test]
                fn test_touching() {
                    assert_eq!(
                        <$solution>::merge(vec![vec![1, 4], vec![4, 5]]),
                        vec![vec![1, 5]]
                    );
                }

                #[test]
                fn test_single() {
                    assert_eq!(
                        <$solution>::merge(vec![vec![1, 5]]),
                        vec![vec![1, 5]]
                    );
                }

                #[test]
                fn test_no_overlap() {
                    assert_eq!(
                        <$solution>::merge(vec![vec![1, 2], vec![4, 5], vec![7, 8]]),
                        vec![vec![1, 2], vec![4, 5], vec![7, 8]]
                    );
                }

                #[test]
                fn test_all_overlap() {
                    assert_eq!(
                        <$solution>::merge(vec![vec![1, 10], vec![2, 6], vec![3, 5]]),
                        vec![vec![1, 10]]
                    );
                }

                #[test]
                fn test_unsorted() {
                    assert_eq!(
                        <$solution>::merge(vec![vec![3, 4], vec![1, 2], vec![2, 3]]),
                        vec![vec![1, 4]]
                    );
                }
            }
        };
    }

    test_merge!(my, crate::my_solution::Solution);
    test_merge!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let intervals: Vec<Vec<i32>> = (0..100)
            .map(|i| vec![i * 3, i * 3 + 2])
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::merge(std::hint::black_box(intervals.clone())),
            );
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::merge(std::hint::black_box(intervals.clone())),
            );
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg ({:.0}ms total)",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            my_duration.as_millis(),
        );
        println!(
            "AI solution: {:.3}us avg ({:.0}ms total)",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            ai_duration.as_millis(),
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
