// LeetCode 253: Meeting Rooms II [Medium] (Premium)
// https://leetcode.com/problems/meeting-rooms-ii/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 5.929us
//   AI solution avg: 7.128us
//   Ratio: My 1.20x faster
//
// Given an array of meeting time intervals, return the minimum number of
// conference rooms required.
//
// Example 1: intervals = [[0,30],[5,10],[15,20]] -> 2
// Example 2: intervals = [[7,10],[2,4]]          -> 1
//
// Constraints:
//   - 1 <= intervals.length <= 10^4
//   - 0 <= start_i < end_i <= 10^6
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0253.Meeting%20Rooms%20II/README_EN.md
//
// Run:  cargo run --example lc_0253_meeting_rooms_ii
// Test: cargo test --example lc_0253_meeting_rooms_ii
// Bench: cargo test --example lc_0253_meeting_rooms_ii bench -- --nocapture

/// My solution — difference array (from doocs Rust solution).
///
/// Create a difference array spanning [0..max_end]. For each interval,
/// +1 at start, -1 at end. Prefix sum gives concurrent meetings at each
/// time; the max prefix sum is the answer.
///
/// Time: O(n + m) where m = max end time
/// Space: O(m)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return minimum conference rooms required.
        pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
            let mut m = 0;
            for e in &intervals {
                m = m.max(e[1]);
            }
            // Build difference array: +1 at start, -1 at end
            let mut d = vec![0i32; (m + 1) as usize];
            for e in intervals {
                d[e[0] as usize] += 1;
                d[e[1] as usize] -= 1;
            }
            // Prefix sum to find peak concurrent meetings
            let mut ans = 0;
            let mut s = 0;
            for v in d {
                s += v;
                ans = ans.max(s);
            }
            ans
        }
    }
}

/// AI solution — sorted events (start/end) with sweep line.
///
/// Collect all start and end times as events. Sort them. Sweep through:
/// +1 for a start, -1 for an end. Track the maximum concurrent count.
/// When a start and end happen at the same time, process end first
/// (a room frees up before a new one is needed).
///
/// Time: O(n log n)
/// Space: O(n)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return minimum conference rooms required.
        pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
            let mut events: Vec<(i32, i32)> = Vec::with_capacity(intervals.len() * 2);
            for iv in &intervals {
                events.push((iv[0], 1)); // meeting start -> +1
                events.push((iv[1], -1)); // meeting end   -> -1
            }
            // Sort by time; if tied, ends (-1) before starts (+1)
            events.sort_unstable();
            let mut ans = 0;
            let mut rooms = 0;
            for (_, delta) in events {
                rooms += delta;
                ans = ans.max(rooms);
            }
            ans
        }
    }
}

fn main() {
    println!(
        "my: {}",
        my_solution::Solution::min_meeting_rooms(vec![vec![0, 30], vec![5, 10], vec![15, 20]])
    );
    println!(
        "ai: {}",
        ai_solution::Solution::min_meeting_rooms(vec![vec![7, 10], vec![2, 4]])
    );
}

#[cfg(test)]
mod tests {
    macro_rules! test_meeting_rooms_ii {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Sol;

                #[test]
                fn test_example_1() {
                    assert_eq!(
                        Sol::min_meeting_rooms(vec![vec![0, 30], vec![5, 10], vec![15, 20]]),
                        2
                    );
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(Sol::min_meeting_rooms(vec![vec![7, 10], vec![2, 4]]), 1);
                }

                #[test]
                fn test_single() {
                    assert_eq!(Sol::min_meeting_rooms(vec![vec![1, 5]]), 1);
                }

                #[test]
                fn test_all_overlap() {
                    assert_eq!(
                        Sol::min_meeting_rooms(vec![vec![0, 10], vec![0, 10], vec![0, 10]]),
                        3
                    );
                }

                #[test]
                fn test_sequential() {
                    assert_eq!(
                        Sol::min_meeting_rooms(vec![vec![0, 5], vec![5, 10], vec![10, 15]]),
                        1
                    );
                }

                #[test]
                fn test_partial_overlap() {
                    assert_eq!(
                        Sol::min_meeting_rooms(vec![vec![0, 10], vec![5, 15], vec![10, 20],]),
                        2
                    );
                }

                #[test]
                fn test_nested() {
                    assert_eq!(
                        Sol::min_meeting_rooms(vec![vec![1, 10], vec![2, 6], vec![3, 5]]),
                        3
                    );
                }

                #[test]
                fn test_same_end_new_start() {
                    // Meeting ends at 5, new one starts at 5 -> reuse room
                    assert_eq!(Sol::min_meeting_rooms(vec![vec![1, 5], vec![5, 10]]), 1);
                }
            }
        };
    }

    test_meeting_rooms_ii!(my, crate::my_solution::Solution);
    test_meeting_rooms_ii!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let intervals: Vec<Vec<i32>> = (0..100).map(|i| vec![i * 5, i * 5 + 20]).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::min_meeting_rooms(
                intervals.clone(),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::min_meeting_rooms(
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
