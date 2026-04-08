// LeetCode 416: Partition Equal Subset Sum [Medium]
// https://leetcode.com/problems/partition-equal-subset-sum/
// Benchmark: 100000 iterations
//   My solution (1D DP boolean):        avg 2.12ms
//   AI solution (bitset DP, u128 blocks): avg 33.70us
//   -> AI solution is 62.78x faster
//
// Given an integer array `nums`, return true if you can partition it into
// two subsets with equal sum.
//
// Examples:
//   [1,5,11,5] -> true   ([1,5,5] and [11])
//   [1,2,3,5]  -> false
//
// Constraints:
//   1 <= nums.length <= 200
//   1 <= nums[i] <= 100
//
// Run:  cargo run --example lc_0416_partition_equal_subset_sum
// Test: cargo test --example lc_0416_partition_equal_subset_sum
// Bench: cargo test --example lc_0416_partition_equal_subset_sum bench -- --nocapture

/// My solution module — 1D DP (space-optimized subset sum).
///
/// If total sum is odd, return false. Otherwise target = sum/2.
/// dp[j] = whether we can form sum j from a subset.
/// Process each number, iterate j from target down to num.
///
/// Time:  O(n * target) where target = sum/2
/// Space: O(target)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Check if the array can be partitioned into two equal-sum subsets.
        ///
        /// # Arguments
        ///
        /// * `nums` - Positive integer array
        ///
        /// # Returns
        ///
        /// true if partition is possible
        pub fn can_partition(nums: Vec<i32>) -> bool {
            let total: i32 = nums.iter().sum();
            if total % 2 != 0 {
                return false;
            }
            let target = (total / 2) as usize;

            let mut dp = vec![false; target + 1];
            dp[0] = true;

            for &num in &nums {
                let num = num as usize;
                // Iterate backwards to avoid using the same element twice
                for j in (num..=target).rev() {
                    dp[j] = dp[j] || dp[j - num];
                }
            }

            dp[target]
        }
    }
}

/// AI solution module — bitset DP using u128 blocks.
///
/// Represent the boolean dp array as a bitset. For each number,
/// shift the bitset left by `num` and OR with original.
/// Uses a Vec<u128> to represent a bitset up to `target+1` bits.
///
/// Time:  O(n * target / 128) -- bitwise parallelism
/// Space: O(target / 128)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Check if the array can be partitioned into two equal-sum subsets.
        ///
        /// # Arguments
        ///
        /// * `nums` - Positive integer array
        ///
        /// # Returns
        ///
        /// true if partition is possible
        pub fn can_partition(nums: Vec<i32>) -> bool {
            let total: i32 = nums.iter().sum();
            if total % 2 != 0 {
                return false;
            }
            let target = (total / 2) as usize;

            // Bitset: bit j is set if sum j is achievable
            let n_blocks = target / 128 + 1;
            let mut bits = vec![0_u128; n_blocks];
            bits[0] = 1; // sum 0 is achievable

            for &num in &nums {
                let num = num as usize;
                let block_shift = num / 128;
                let bit_shift = num % 128;

                // Shift entire bitset left by `num` and OR
                // Process from high to low to avoid overwriting
                for i in (0..n_blocks).rev() {
                    let mut shifted = 0_u128;
                    if i >= block_shift {
                        shifted = bits[i - block_shift] << bit_shift;
                        // Add carry from the block below
                        if bit_shift > 0 && i > block_shift {
                            shifted |=
                                bits[i - block_shift - 1] >> (128 - bit_shift);
                        }
                    }
                    bits[i] |= shifted;
                }
            }

            // Check if bit `target` is set
            let block = target / 128;
            let bit = target % 128;
            (bits[block] >> bit) & 1 == 1
        }
    }
}

fn main() {
    let cases = [
        (vec![1, 5, 11, 5], true),
        (vec![1, 2, 3, 5], false),
    ];

    println!("--- My Solution ---");
    for (nums, expected) in &cases {
        let result = my_solution::Solution::can_partition(nums.clone());
        println!("  {nums:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (nums, expected) in &cases {
        let result = ai_solution::Solution::can_partition(nums.clone());
        println!("  {nums:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_partition {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert!(Solution::can_partition(vec![1, 5, 11, 5]));
                }

                #[test]
                fn test_example2() {
                    assert!(!Solution::can_partition(vec![1, 2, 3, 5]));
                }

                #[test]
                fn test_single_element() {
                    assert!(!Solution::can_partition(vec![1]));
                }

                #[test]
                fn test_two_equal() {
                    assert!(Solution::can_partition(vec![1, 1]));
                }

                #[test]
                fn test_odd_sum() {
                    // sum=7 is odd, impossible
                    assert!(!Solution::can_partition(vec![1, 2, 4]));
                }

                #[test]
                fn test_large_equal() {
                    // [100, 100] -> true
                    assert!(Solution::can_partition(vec![100, 100]));
                }

                #[test]
                fn test_complex() {
                    // [1,2,3,4,5,6,7] sum=28, target=14
                    // {7,6,1} = 14
                    assert!(Solution::can_partition(
                        vec![1, 2, 3, 4, 5, 6, 7],
                    ));
                }
            }
        };
    }

    test_partition!(my_tests, crate::my_solution::Solution);
    test_partition!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        // Worst case: 200 elements of 100 -> sum=20000, target=10000
        let nums: Vec<i32> = vec![100; 200];

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::can_partition(
                    std::hint::black_box(nums.clone()),
                ),
            );
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::can_partition(
                    std::hint::black_box(nums.clone()),
                ),
            );
        }
        let ai_duration = start.elapsed();

        let my_avg = my_duration / iterations;
        let ai_avg = ai_duration / iterations;

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution:  {my_duration:.2?} (avg {my_avg:.2?})");
        println!("AI solution:  {ai_duration:.2?} (avg {ai_avg:.2?})");

        if my_avg <= ai_avg {
            let ratio = ai_avg.as_nanos() as f64
                / my_avg.as_nanos().max(1) as f64;
            println!("-> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_avg.as_nanos() as f64
                / ai_avg.as_nanos().max(1) as f64;
            println!("-> AI solution is {ratio:.2}x faster");
        }
    }
}
