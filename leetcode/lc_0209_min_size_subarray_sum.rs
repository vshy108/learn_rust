// LeetCode 209: Minimum Size Subarray Sum [Medium]
// https://leetcode.com/problems/minimum-size-subarray-sum/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.700us
//   AI solution avg: 0.527us
//   Ratio: AI 1.33x faster
//
// Given an array of positive integers nums and a positive integer target,
// return the minimal length of a contiguous subarray whose sum is >= target.
// If no such subarray exists, return 0.
//
// Constraints: 1 <= target <= 10^9, 1 <= nums.length <= 10^5,
//   1 <= nums[i] <= 10^4
//
// Run:  cargo run --example lc_0209_min_size_subarray_sum
// Test: cargo test --example lc_0209_min_size_subarray_sum
// Bench: cargo test --example lc_0209_min_size_subarray_sum bench -- --nocapture

/// My solution — todo.
///
/// Time: O(?).
/// Space: O(?).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return minimal length of subarray with sum >= target.
        pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
            let mut left = 0;
            let mut sum = 0i64;
            let target = target as i64;
            let mut min_len = usize::MAX;
            for right in 0..nums.len() {
                sum += nums[right] as i64;
                while sum >= target {
                    min_len = min_len.min(right - left + 1);
                    sum -= nums[left] as i64;
                    left += 1;
                }
            }
            if min_len == usize::MAX { 0 } else { min_len as i32 }
        }
    }
}

/// AI solution — sliding window (two pointers).
///
/// Expand right pointer to grow the window sum; when sum >= target,
/// shrink from left and track the minimum window length.
///
/// Time: O(n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Sliding window approach.
        pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
            let mut left = 0usize;
            let mut sum = 0i64;
            let target = target as i64;
            let mut min_len = usize::MAX;

            for right in 0..nums.len() {
                sum += nums[right] as i64;
                // Shrink window from the left while sum is still >= target
                while sum >= target {
                    min_len = min_len.min(right - left + 1);
                    sum -= nums[left] as i64;
                    left += 1;
                }
            }
            if min_len == usize::MAX { 0 } else { min_len as i32 }
        }
    }
}

fn main() {
    let my = my_solution::Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]);
    let ai = ai_solution::Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]);
    println!("min length: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_msa {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(
                        <$solution>::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]),
                        2,
                    );
                }

                #[test]
                fn test_exact() {
                    assert_eq!(
                        <$solution>::min_sub_array_len(4, vec![1, 4, 4]),
                        1,
                    );
                }

                #[test]
                fn test_impossible() {
                    assert_eq!(
                        <$solution>::min_sub_array_len(
                            11,
                            vec![1, 1, 1, 1, 1, 1, 1, 1],
                        ),
                        0,
                    );
                }

                #[test]
                fn test_whole_array() {
                    assert_eq!(
                        <$solution>::min_sub_array_len(15, vec![1, 2, 3, 4, 5]),
                        5,
                    );
                }

                #[test]
                fn test_single_element() {
                    assert_eq!(
                        <$solution>::min_sub_array_len(5, vec![5]),
                        1,
                    );
                }
            }
        };
    }

    test_msa!(my, crate::my_solution::Solution);
    test_msa!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums: Vec<i32> = (1..=500).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::min_sub_array_len(
                std::hint::black_box(1000),
                std::hint::black_box(nums.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::min_sub_array_len(
                std::hint::black_box(1000),
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
        } else {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        }
    }
}
