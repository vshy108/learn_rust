// LeetCode 228: Summary Ranges [Easy]
// https://leetcode.com/problems/summary-ranges/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.830us
//   AI solution avg: 0.707us
//   Ratio: AI 1.17x faster
//
// You are given a sorted unique integer array nums. A range [a,b] is
// the set of all integers from a to b (inclusive). Return the smallest
// sorted list of ranges that cover all the numbers in the array exactly.
//
// Each range [a,b] should be output as: "a->b" if a != b, "a" if a == b.
//
// Constraints: 0 <= nums.length <= 20, -2^31 <= nums[i] <= 2^31 - 1,
// All values are unique, nums is sorted in ascending order.
//
// Run:  cargo run --example lc_0228_summary_ranges
// Test: cargo test --example lc_0228_summary_ranges
// Bench: cargo test --example lc_0228_summary_ranges bench -- --nocapture

/// My solution — two-pointer scan.
///
/// Walk through the array with a start pointer. For each group of
/// consecutive numbers, advance an end pointer. Format the range
/// and move start past the group.
///
/// Time: O(n).
/// Space: O(1) extra (output excluded).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return summary ranges for the sorted unique array `nums`.
        pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
            let mut result = Vec::new();
            let n = nums.len();
            let mut i = 0;

            while i < n {
                let start = nums[i];
                // Advance while the next element is consecutive
                while i + 1 < n && nums[i + 1] == nums[i] + 1 {
                    i += 1;
                }
                let end = nums[i];

                if start == end {
                    result.push(format!("{start}"));
                } else {
                    result.push(format!("{start}->{end}"));
                }
                i += 1;
            }

            result
        }
    }
}

/// AI solution — iterator-based with `windows`.
///
/// Use `split` positions where consecutive elements differ by more than 1.
/// Collect each group's first and last element into the range string.
///
/// Time: O(n).
/// Space: O(1) extra (output excluded).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return summary ranges for the sorted unique array `nums`.
        pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
            if nums.is_empty() {
                return Vec::new();
            }

            let mut result = Vec::new();
            let mut start = 0; // index of the beginning of the current range

            for i in 1..=nums.len() {
                // At the end or when a gap is found, close the current range
                if i == nums.len() || nums[i] != nums[i - 1] + 1 {
                    if start == i - 1 {
                        result.push(format!("{}", nums[start]));
                    } else {
                        result.push(format!("{}->{}", nums[start], nums[i - 1]));
                    }
                    start = i;
                }
            }

            result
        }
    }
}

fn main() {
    let cases: Vec<(Vec<i32>, Vec<&str>)> = vec![
        (vec![0, 1, 2, 4, 5, 7], vec!["0->2", "4->5", "7"]),
        (vec![0, 2, 3, 4, 6, 8, 9], vec!["0", "2->4", "6", "8->9"]),
    ];

    for (nums, expected) in &cases {
        let my = my_solution::Solution::summary_ranges(nums.clone());
        let ai = ai_solution::Solution::summary_ranges(nums.clone());
        println!("{nums:?} => my={my:?}, ai={ai:?} (expected {expected:?})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_ranges {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(
                        <$solution>::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
                        vec!["0->2", "4->5", "7"],
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        <$solution>::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
                        vec!["0", "2->4", "6", "8->9"],
                    );
                }

                #[test]
                fn test_empty() {
                    let empty: Vec<String> = Vec::new();
                    assert_eq!(<$solution>::summary_ranges(vec![]), empty);
                }

                #[test]
                fn test_single() {
                    assert_eq!(
                        <$solution>::summary_ranges(vec![5]),
                        vec!["5"],
                    );
                }

                #[test]
                fn test_all_consecutive() {
                    assert_eq!(
                        <$solution>::summary_ranges(vec![1, 2, 3, 4, 5]),
                        vec!["1->5"],
                    );
                }

                #[test]
                fn test_no_consecutive() {
                    assert_eq!(
                        <$solution>::summary_ranges(vec![1, 3, 5, 7]),
                        vec!["1", "3", "5", "7"],
                    );
                }

                #[test]
                fn test_negative_numbers() {
                    assert_eq!(
                        <$solution>::summary_ranges(vec![-3, -2, -1, 1, 2]),
                        vec!["-3->-1", "1->2"],
                    );
                }
            }
        };
    }

    test_ranges!(my, crate::my_solution::Solution);
    test_ranges!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums: Vec<i32> = vec![0, 1, 2, 4, 5, 7, 10, 11, 12, 13, 20, 22, 23, 25, 26, 27, 28, 30];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::summary_ranges(
                std::hint::black_box(nums.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::summary_ranges(
                std::hint::black_box(nums.clone()),
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
