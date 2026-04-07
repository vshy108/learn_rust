// LeetCode 163: Missing Ranges [Easy] (Premium)
// https://leetcode.com/problems/missing-ranges/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 3.164us
//   AI solution avg: 2.942us
//   Ratio: AI 1.08x faster
//
// Given a sorted integer array nums and a closed interval [lower, upper],
// return a list of ranges that cover every number in [lower, upper]
// that is NOT in nums. Each range [a, b] means a <= x <= b.
//
// Example 1: nums = [0,1,3,50,75], lower = 0, upper = 99
//   -> [[2,2],[4,49],[51,74],[76,99]]
// Example 2: nums = [-1], lower = -1, upper = -1 -> []
//
// Constraints:
//   - -10^9 <= lower <= upper <= 10^9
//   - 0 <= nums.length <= 100
//   - lower <= nums[i] <= upper
//   - All values in nums are unique
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0100-0199/0163.Missing%20Ranges/README_EN.md
//
// Run:  cargo run --example lc_0163_missing_ranges
// Test: cargo test --example lc_0163_missing_ranges
// Bench: cargo test --example lc_0163_missing_ranges bench -- --nocapture

/// My solution — simulation (translated from Python in doocs/leetcode).
///
/// Handle gaps before first element, between consecutive elements,
/// and after the last element.
///
/// Time: O(n) — single pass through nums.
/// Space: O(1) — ignoring output.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return list of missing ranges as [start, end] pairs.
        pub fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<Vec<i32>> {
            let n = nums.len();
            if n == 0 {
                return vec![vec![lower, upper]];
            }

            let mut ans = Vec::new();

            // Gap before first element
            if nums[0] > lower {
                ans.push(vec![lower, nums[0] - 1]);
            }

            // Gaps between consecutive elements
            for i in 1..n {
                if nums[i] - nums[i - 1] > 1 {
                    ans.push(vec![nums[i - 1] + 1, nums[i] - 1]);
                }
            }

            // Gap after last element
            if nums[n - 1] < upper {
                ans.push(vec![nums[n - 1] + 1, upper]);
            }

            ans
        }
    }
}

/// AI solution — unified approach with sentinel values.
///
/// Prepend (lower - 1) and append (upper + 1) as sentinels, then scan
/// consecutive pairs. Any gap > 1 between consecutive values produces
/// a missing range. Uses i64 to avoid overflow with i32 boundaries.
///
/// Time: O(n)
/// Space: O(n) for the extended array (could be O(1) with iterators).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return list of missing ranges as [start, end] pairs.
        pub fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<Vec<i32>> {
            let mut extended: Vec<i64> = Vec::with_capacity(nums.len() + 2);
            // Sentinel before lower (using i64 to avoid overflow)
            extended.push(lower as i64 - 1);
            for &num in &nums {
                extended.push(num as i64);
            }
            // Sentinel after upper
            extended.push(upper as i64 + 1);

            let mut ans = Vec::new();
            for pair in extended.windows(2) {
                let (a, b) = (pair[0], pair[1]);
                if b - a > 1 {
                    ans.push(vec![(a + 1) as i32, (b - 1) as i32]);
                }
            }
            ans
        }
    }
}

fn main() {
    let cases: Vec<(Vec<i32>, i32, i32, Vec<Vec<i32>>)> = vec![
        (
            vec![0, 1, 3, 50, 75],
            0,
            99,
            vec![vec![2, 2], vec![4, 49], vec![51, 74], vec![76, 99]],
        ),
        (vec![-1], -1, -1, vec![]),
        (vec![], 1, 1, vec![vec![1, 1]]),
    ];

    for (nums, lower, upper, expected) in &cases {
        let my = my_solution::Solution::find_missing_ranges(nums.clone(), *lower, *upper);
        let ai = ai_solution::Solution::find_missing_ranges(nums.clone(), *lower, *upper);
        println!("{nums:?} [{lower},{upper}]: my={my:?}, ai={ai:?} (expected {expected:?})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_missing_ranges {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(
                        <$solution>::find_missing_ranges(vec![0, 1, 3, 50, 75], 0, 99),
                        vec![vec![2, 2], vec![4, 49], vec![51, 74], vec![76, 99]],
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        <$solution>::find_missing_ranges(vec![-1], -1, -1),
                        Vec::<Vec<i32>>::new(),
                    );
                }

                #[test]
                fn test_empty_nums() {
                    assert_eq!(
                        <$solution>::find_missing_ranges(vec![], 1, 1),
                        vec![vec![1, 1]],
                    );
                }

                #[test]
                fn test_no_gaps() {
                    assert_eq!(
                        <$solution>::find_missing_ranges(vec![1, 2, 3], 1, 3),
                        Vec::<Vec<i32>>::new(),
                    );
                }

                #[test]
                fn test_single_gap() {
                    assert_eq!(
                        <$solution>::find_missing_ranges(vec![1, 3], 1, 3),
                        vec![vec![2, 2]],
                    );
                }
            }
        };
    }

    test_missing_ranges!(my, crate::my_solution::Solution);
    test_missing_ranges!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums: Vec<i32> = (0..50).map(|i| i * 2).collect(); // even numbers 0..98
        let lower = 0;
        let upper = 99;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::find_missing_ranges(
                std::hint::black_box(nums.clone()),
                lower,
                upper,
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::find_missing_ranges(
                std::hint::black_box(nums.clone()),
                lower,
                upper,
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
