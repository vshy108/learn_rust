// LeetCode 259: 3Sum Smaller [Medium] (Premium)
// https://leetcode.com/problems/3sum-smaller/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 13.135us
//   AI solution avg: 16.633us
//   Ratio: My 1.27x faster
//
// Given an array of n integers nums and an integer target, find the number
// of index triplets (i, j, k) with 0 <= i < j < k < n that satisfy
// nums[i] + nums[j] + nums[k] < target.
//
// Example 1: nums = [-2,0,1,3], target = 2 -> 2
//   Triplets: [-2,0,1], [-2,0,3]
// Example 2: nums = [], target = 0 -> 0
// Example 3: nums = [0], target = 0 -> 0
//
// Constraints:
//   - n == nums.length, 0 <= n <= 3500
//   - -100 <= nums[i] <= 100
//   - -100 <= target <= 100
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0259.3Sum%20Smaller/README_EN.md
//
// Run:  cargo run --example lc_0259_3sum_smaller
// Test: cargo test --example lc_0259_3sum_smaller
// Bench: cargo test --example lc_0259_3sum_smaller bench -- --nocapture

/// My solution — sort + two pointers (from doocs Python).
///
/// Sort the array. For each i, use two pointers j and k on [i+1..n-1].
/// If nums[i]+nums[j]+nums[k] < target, all k' in [j+1..k] also work,
/// so add (k-j) to answer and advance j. Otherwise shrink k.
///
/// Time: O(n^2)
/// Space: O(log n) for sort
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return count of triplets with sum < target.
        pub fn three_sum_smaller(mut nums: Vec<i32>, target: i32) -> i32 {
            nums.sort_unstable();
            let n = nums.len();
            let mut ans = 0i32;
            if n < 3 {
                return 0;
            }
            for i in 0..n - 2 {
                let mut j = i + 1;
                let mut k = n - 1;
                while j < k {
                    let sum = nums[i] + nums[j] + nums[k];
                    if sum < target {
                        // All pairs (j, j+1..k) satisfy the condition
                        ans += (k - j) as i32;
                        j += 1;
                    } else {
                        k -= 1;
                    }
                }
            }
            ans
        }
    }
}

/// AI solution — sort + binary search for each (i, j) pair.
///
/// For each pair (i, j), binary search for the largest k > j such that
/// nums[i] + nums[j] + nums[k] < target. Count valid indices.
///
/// Time: O(n^2 log n)
/// Space: O(log n) for sort
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return count of triplets with sum < target.
        pub fn three_sum_smaller(mut nums: Vec<i32>, target: i32) -> i32 {
            nums.sort_unstable();
            let n = nums.len();
            let mut ans = 0i32;
            if n < 3 {
                return 0;
            }
            for i in 0..n - 2 {
                for j in i + 1..n - 1 {
                    // Find largest k in [j+1..n-1] where nums[k] < target - nums[i] - nums[j]
                    let remain = target - nums[i] - nums[j];
                    // Binary search: partition_point gives first index where nums[k] >= remain
                    let bound = nums[j + 1..].partition_point(|&x| x < remain);
                    ans += bound as i32;
                }
            }
            ans
        }
    }
}

fn main() {
    println!(
        "my: {}",
        my_solution::Solution::three_sum_smaller(vec![-2, 0, 1, 3], 2)
    );
    println!(
        "ai: {}",
        ai_solution::Solution::three_sum_smaller(vec![-2, 0, 1, 3], 2)
    );
}

#[cfg(test)]
mod tests {
    macro_rules! test_3sum_smaller {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Sol;

                #[test]
                fn test_example_1() {
                    assert_eq!(Sol::three_sum_smaller(vec![-2, 0, 1, 3], 2), 2);
                }

                #[test]
                fn test_example_2_empty() {
                    assert_eq!(Sol::three_sum_smaller(vec![], 0), 0);
                }

                #[test]
                fn test_example_3_single() {
                    assert_eq!(Sol::three_sum_smaller(vec![0], 0), 0);
                }

                #[test]
                fn test_two_elements() {
                    assert_eq!(Sol::three_sum_smaller(vec![1, 2], 5), 0);
                }

                #[test]
                fn test_all_below() {
                    // All triplets sum < 100
                    assert_eq!(Sol::three_sum_smaller(vec![1, 2, 3, 4], 100), 4);
                }

                #[test]
                fn test_none_below() {
                    assert_eq!(Sol::three_sum_smaller(vec![1, 2, 3], 0), 0);
                }

                #[test]
                fn test_negatives() {
                    assert_eq!(
                        Sol::three_sum_smaller(vec![-2, -1, 0, 1, 2], 0),
                        4, // (-2,-1,0), (-2,-1,1), (-2,-1,2), (-2,0,1)
                    );
                }

                #[test]
                fn test_exact_boundary() {
                    // sum = 6 is not < 6, so 0
                    assert_eq!(Sol::three_sum_smaller(vec![1, 2, 3], 6), 0);
                    // sum = 6 < 7, so 1
                    assert_eq!(Sol::three_sum_smaller(vec![1, 2, 3], 7), 1);
                }

                #[test]
                fn test_duplicates() {
                    assert_eq!(Sol::three_sum_smaller(vec![0, 0, 0, 0], 1), 4);
                }
            }
        };
    }

    test_3sum_smaller!(my, crate::my_solution::Solution);
    test_3sum_smaller!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums: Vec<i32> = (-50..50).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::three_sum_smaller(
                nums.clone(),
                10,
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::three_sum_smaller(
                nums.clone(),
                10,
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
