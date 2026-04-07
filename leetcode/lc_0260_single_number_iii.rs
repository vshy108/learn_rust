// LeetCode 260: Single Number III [Medium]
// https://leetcode.com/problems/single-number-iii/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 0.294us
//   AI solution avg: 0.273us
//   Ratio: AI 1.08x faster
//
// Given an integer array nums where exactly two elements appear once and
// all others appear exactly twice, find the two unique elements.
// Must run in O(n) time and O(1) extra space.
//
// Example 1: nums = [1,2,1,3,2,5] -> [3,5]
// Example 2: nums = [-1,0]        -> [-1,0]
// Example 3: nums = [0,1]         -> [1,0]
//
// Constraints:
//   - 2 <= nums.length <= 3 * 10^4
//   - -2^31 <= nums[i] <= 2^31 - 1
//   - Each integer appears twice except two which appear once
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0260.Single%20Number%20III/README_EN.md
//
// Run:  cargo run --example lc_0260_single_number_iii
// Test: cargo test --example lc_0260_single_number_iii
// Bench: cargo test --example lc_0260_single_number_iii bench -- --nocapture

/// My solution — XOR + lowbit partition (from doocs Rust).
///
/// XOR all elements: duplicates cancel, leaving a ^ b.
/// Find lowest set bit (lowbit = xs & -xs). This bit differs between
/// a and b, so it splits the array into two groups. XOR within each
/// group isolates a and b.
///
/// Time: O(n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the two unique numbers in any order.
        pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
            // XOR all: duplicates cancel, leaving a ^ b
            let xs = nums.iter().fold(0i32, |acc, &v| acc ^ v);
            // Lowest set bit: this bit is 1 in one unique and 0 in the other
            let lb = xs & xs.wrapping_neg();
            let mut a = 0i32;
            for &x in &nums {
                if (x & lb) != 0 {
                    a ^= x;
                }
            }
            let b = xs ^ a;
            vec![a, b]
        }
    }
}

/// AI solution — single pass using two accumulators.
///
/// Same XOR + lowbit idea but uses a tuple accumulator to split in
/// one pass through the array.
///
/// Time: O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the two unique numbers in any order.
        pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
            let xs: i32 = nums.iter().copied().fold(0, |a, b| a ^ b);
            let lb = xs & xs.wrapping_neg();
            // Split and XOR in one pass using fold with two accumulators
            let (a, b) = nums.iter().fold((0i32, 0i32), |(a, b), &x| {
                if (x & lb) != 0 {
                    (a ^ x, b)
                } else {
                    (a, b ^ x)
                }
            });
            vec![a, b]
        }
    }
}

fn main() {
    println!("my: {:?}", my_solution::Solution::single_number(vec![1, 2, 1, 3, 2, 5]));
    println!("ai: {:?}", ai_solution::Solution::single_number(vec![-1, 0]));
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    macro_rules! test_single_number_iii {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;
                use $solution as Sol;

                fn check(nums: Vec<i32>, expected: [i32; 2]) {
                    let result = Sol::single_number(nums);
                    let got: HashSet<i32> = result.into_iter().collect();
                    let want: HashSet<i32> = expected.into_iter().collect();
                    assert_eq!(got, want);
                }

                #[test]
                fn test_example_1() {
                    check(vec![1, 2, 1, 3, 2, 5], [3, 5]);
                }

                #[test]
                fn test_example_2() {
                    check(vec![-1, 0], [-1, 0]);
                }

                #[test]
                fn test_example_3() {
                    check(vec![0, 1], [0, 1]);
                }

                #[test]
                fn test_negative_pair() {
                    check(vec![1, 1, -2, -3], [-2, -3]);
                }

                #[test]
                fn test_large_values() {
                    check(
                        vec![i32::MAX, i32::MIN, i32::MAX, 42, 42, i32::MIN, 7, -7],
                        [7, -7],
                    );
                }

                #[test]
                fn test_two_elements() {
                    check(vec![100, 200], [100, 200]);
                }

                #[test]
                fn test_interleaved() {
                    check(vec![3, 5, 3, 7, 5, 9, 7, 11], [9, 11]);
                }
            }
        };
    }

    test_single_number_iii!(my, crate::my_solution::Solution);
    test_single_number_iii!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        // 500 pairs + 2 uniques = 1002 elements
        let mut nums: Vec<i32> = (1..=500).flat_map(|x| [x, x]).collect();
        nums.push(999);
        nums.push(1001);

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::single_number(nums.clone()));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::single_number(nums.clone()));
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
