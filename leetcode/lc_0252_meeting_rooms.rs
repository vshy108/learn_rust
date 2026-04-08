// LeetCode 252: Meeting Rooms [Easy] (Premium)
// https://leetcode.com/problems/meeting-rooms/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 5.472us
//   AI solution avg: 5.329us
//   Ratio: AI 1.03x faster
//
// Given an array of meeting time intervals where intervals[i] = [start_i, end_i],
// determine if a person could attend all meetings (no overlap).
//
// Example 1: intervals = [[0,30],[5,10],[15,20]] -> false
// Example 2: intervals = [[7,10],[2,4]]          -> true
//
// Constraints:
//   - 0 <= intervals.length <= 10^4
//   - intervals[i].length == 2
//   - 0 <= start_i < end_i <= 10^6
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0252.Meeting%20Rooms/README_EN.md
//
// Run:  cargo run --example lc_0252_meeting_rooms
// Test: cargo test --example lc_0252_meeting_rooms
// Bench: cargo test --example lc_0252_meeting_rooms bench -- --nocapture

/// My solution — sort by start time then check adjacent (from doocs Rust solution).
///
/// Sort intervals by start time, then iterate checking if any
/// meeting's start time is before the previous meeting's end time.
///
/// Time: O(n log n)
/// Space: O(log n) — sorting overhead
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return true if all meetings can be attended without overlap.
        pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
            intervals.sort_by(|a, b| a[0].cmp(&b[0]));
            for i in 1..intervals.len() {
                if intervals[i - 1][1] > intervals[i][0] {
                    return false;
                }
            }
            true
        }
    }
}

/// AI solution — sort by end time, use windows(2) for idiomatic check.
///
/// Sort by end time instead. Overlaps are detected when a meeting
/// starts before the previous one ends. Uses `windows(2)` for a
/// more idiomatic Rust approach.
///
/// Time: O(n log n)
/// Space: O(log n)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return true if all meetings can be attended without overlap.
        pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
            // Sort by start time using unstable sort (faster, no allocation)
            intervals.sort_unstable_by_key(|iv| iv[0]);
            // Check all consecutive pairs — no overlap means prev end <= curr start
            intervals.windows(2).all(|w| w[0][1] <= w[1][0])
        }
    }
}

fn main() {
    println!(
        "my: {}",
        my_solution::Solution::can_attend_meetings(vec![vec![0, 30], vec![5, 10], vec![15, 20]])
    );
    println!(
        "ai: {}",
        ai_solution::Solution::can_attend_meetings(vec![vec![7, 10], vec![2, 4]])
    );
}

#[cfg(test)]
mod tests {
    macro_rules! test_meeting_rooms {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Sol;

                #[test]
                fn test_example_1_overlap() {
                    assert!(!Sol::can_attend_meetings(vec![
                        vec![0, 30],
                        vec![5, 10],
                        vec![15, 20],
                    ]));
                }

                #[test]
                fn test_example_2_no_overlap() {
                    assert!(Sol::can_attend_meetings(vec![vec![7, 10], vec![2, 4]]));
                }

                #[test]
                fn test_empty() {
                    assert!(Sol::can_attend_meetings(vec![]));
                }

                #[test]
                fn test_single() {
                    assert!(Sol::can_attend_meetings(vec![vec![1, 5]]));
                }

                #[test]
                fn test_touching() {
                    // [1,5] and [5,10] don't overlap (end == start is ok)
                    assert!(Sol::can_attend_meetings(vec![vec![1, 5], vec![5, 10]]));
                }

                #[test]
                fn test_exact_overlap() {
                    assert!(!Sol::can_attend_meetings(vec![vec![1, 5], vec![3, 7]]));
                }

                #[test]
                fn test_contained() {
                    assert!(!Sol::can_attend_meetings(vec![vec![1, 10], vec![2, 5]]));
                }

                #[test]
                fn test_sorted_no_overlap() {
                    assert!(Sol::can_attend_meetings(vec![
                        vec![1, 2],
                        vec![3, 4],
                        vec![5, 6],
                    ]));
                }
            }
        };
    }

    test_meeting_rooms!(my, crate::my_solution::Solution);
    test_meeting_rooms!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let intervals: Vec<Vec<i32>> = (0..100).map(|i| vec![i * 10, i * 10 + 5]).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::can_attend_meetings(
                intervals.clone(),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::can_attend_meetings(
                intervals.clone(),
            ));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else if ai_avg < my_avg {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        } else {
            println!("  Ratio: ~tied");
        }
    }
}
