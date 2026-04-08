// LeetCode 57: Insert Interval [Medium]
// https://leetcode.com/problems/insert-interval/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 15.724us
//   AI solution avg: 15.535us
//   Ratio: ~tied
//
// You are given an array of non-overlapping intervals sorted by start_i.
// Insert newInterval and merge if necessary. Return sorted non-overlapping.
//
// Example 1: intervals=[[1,3],[6,9]], new=[2,5] -> [[1,5],[6,9]]
// Example 2: intervals=[[1,2],[3,5],[6,7],[8,10],[12,16]], new=[4,8]
//            -> [[1,2],[3,10],[12,16]]
//
// Constraints:
//   0 <= intervals.length <= 10^4
//   intervals[i].length == 2, newInterval.length == 2
//   0 <= start_i <= end_i <= 10^5
//
// Run:  cargo run --example lc_0057_insert_interval
// Test: cargo test --example lc_0057_insert_interval
// Bench: cargo test --example lc_0057_insert_interval bench -- --nocapture

/// My solution — three-phase linear scan.
///
/// 1. Add all intervals ending before newInterval starts.
/// 2. Merge all overlapping intervals with newInterval.
/// 3. Add remaining intervals.
///
/// Time: O(n) — single pass.
/// Space: O(n) for output.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Insert and merge a new interval into sorted non-overlapping list.
        ///
        /// # Arguments
        /// * `intervals` - sorted non-overlapping intervals
        /// * `new_interval` - interval to insert
        ///
        /// # Returns
        /// Merged result
        pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
            let mut result = Vec::with_capacity(intervals.len() + 1);
            let mut i = 0;
            let (mut start, mut end) = (new_interval[0], new_interval[1]);

            // Phase 1: intervals entirely before new_interval
            while i < intervals.len() && intervals[i][1] < start {
                result.push(intervals[i].clone());
                i += 1;
            }
            // Phase 2: merge overlapping intervals
            while i < intervals.len() && intervals[i][0] <= end {
                start = start.min(intervals[i][0]);
                end = end.max(intervals[i][1]);
                i += 1;
            }
            result.push(vec![start, end]);
            // Phase 3: intervals entirely after
            while i < intervals.len() {
                result.push(intervals[i].clone());
                i += 1;
            }
            result
        }
    }
}

/// AI solution — same algorithm (canonical optimal for sorted input).
///
/// Three-phase scan is already optimal for this problem.
///
/// Time: O(n).
/// Space: O(n).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Insert and merge a new interval into sorted non-overlapping list.
        ///
        /// # Arguments
        /// * `intervals` - sorted non-overlapping intervals
        /// * `new_interval` - interval to insert
        ///
        /// # Returns
        /// Merged result
        pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
            let mut result = Vec::with_capacity(intervals.len() + 1);
            let (mut start, mut end) = (new_interval[0], new_interval[1]);
            let mut inserted = false;

            for iv in &intervals {
                if iv[1] < start {
                    result.push(iv.clone());
                } else if iv[0] > end {
                    if !inserted {
                        result.push(vec![start, end]);
                        inserted = true;
                    }
                    result.push(iv.clone());
                } else {
                    start = start.min(iv[0]);
                    end = end.max(iv[1]);
                }
            }
            if !inserted {
                result.push(vec![start, end]);
            }
            result
        }
    }
}

fn main() {
    let cases = [
        (vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
        (
            vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16],
            ],
            vec![4, 8],
        ),
    ];
    for (intervals, new) in &cases {
        println!(
            "{intervals:?} + {new:?} -> my:{:?}, ai:{:?}",
            my_solution::Solution::insert(intervals.clone(), new.clone()),
            ai_solution::Solution::insert(intervals.clone(), new.clone()),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_insert {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_basic() {
                    assert_eq!(
                        <$solution>::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
                        vec![vec![1, 5], vec![6, 9]]
                    );
                }

                #[test]
                fn test_merge_multiple() {
                    assert_eq!(
                        <$solution>::insert(
                            vec![
                                vec![1, 2],
                                vec![3, 5],
                                vec![6, 7],
                                vec![8, 10],
                                vec![12, 16]
                            ],
                            vec![4, 8]
                        ),
                        vec![vec![1, 2], vec![3, 10], vec![12, 16]]
                    );
                }

                #[test]
                fn test_empty() {
                    assert_eq!(<$solution>::insert(vec![], vec![5, 7]), vec![vec![5, 7]]);
                }

                #[test]
                fn test_no_overlap_before() {
                    assert_eq!(
                        <$solution>::insert(vec![vec![3, 5]], vec![1, 2]),
                        vec![vec![1, 2], vec![3, 5]]
                    );
                }

                #[test]
                fn test_no_overlap_after() {
                    assert_eq!(
                        <$solution>::insert(vec![vec![1, 2]], vec![5, 7]),
                        vec![vec![1, 2], vec![5, 7]]
                    );
                }

                #[test]
                fn test_total_merge() {
                    assert_eq!(
                        <$solution>::insert(vec![vec![1, 5], vec![6, 8]], vec![0, 10]),
                        vec![vec![0, 10]]
                    );
                }
            }
        };
    }

    test_insert!(my, crate::my_solution::Solution);
    test_insert!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let intervals: Vec<Vec<i32>> = (0..100).map(|i| vec![i * 5, i * 5 + 2]).collect();
        let new_interval = vec![250, 350];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::insert(
                std::hint::black_box(intervals.clone()),
                std::hint::black_box(new_interval.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::insert(
                std::hint::black_box(intervals.clone()),
                std::hint::black_box(new_interval.clone()),
            ));
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
