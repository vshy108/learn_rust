// LeetCode 396: Rotate Function [Medium]
// https://leetcode.com/problems/rotate-function/
//
// Benchmark: 200,000 iterations (2026-04-08)
//   My solution (loop recurrence): avg 174.03us
//   AI solution (fold recurrence): avg 172.60us
//   -> AI solution is 1.01x faster (effectively tied)
//
// Given an integer array `nums` of length n, define:
//   arr_k = nums rotated k positions clockwise
//   F(k) = 0*arr_k[0] + 1*arr_k[1] + ... + (n-1)*arr_k[n-1]
//
// Return max(F(0), F(1), ..., F(n-1)).
//
// Examples:
//   [4,3,2,6] -> 26
//     F(0)=25, F(1)=16, F(2)=23, F(3)=26
//   [100] -> 0
//
// Constraints:
//   - n == nums.length
//   - 1 <= n <= 10^5
//   - -100 <= nums[i] <= 100
//
// Run:  cargo run --example lc_0396_rotate_function
// Test: cargo test --example lc_0396_rotate_function
// Bench: cargo test --example lc_0396_rotate_function bench -- --nocapture

/// My solution module — O(n) recurrence relation.
///
/// Key insight: F(k) - F(k-1) = sum - n * nums[n-k].
/// When rotating one step clockwise, every element's weight increases by 1
/// (gaining +sum), but the element that wraps from last to first loses
/// n * its value.
///
/// Time:  O(n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the maximum value of F(0)..F(n-1).
        ///
        /// # Arguments
        ///
        /// * `nums` - Integer array
        ///
        /// # Returns
        ///
        /// Maximum rotation function value
        pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            let sum: i64 = nums.iter().map(|&v| v as i64).sum();
            // Compute F(0)
            let mut f: i64 = nums
                .iter()
                .enumerate()
                .map(|(i, &v)| i as i64 * v as i64)
                .sum();
            let mut ans = f;

            // Use i64 to prevent overflow on large inputs
            for i in 1..n {
                // F(i) = F(i-1) + sum - n * nums[n - i]
                f = f + sum - n as i64 * nums[n - i] as i64;
                ans = ans.max(f);
            }
            ans as i32
        }
    }
}

/// AI solution module — same recurrence, iterator-style.
///
/// Computes F(0) and sum in one pass, then uses the recurrence in a
/// functional chain with `fold` to track the maximum.
///
/// Time:  O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the maximum value of F(0)..F(n-1).
        ///
        /// # Arguments
        ///
        /// * `nums` - Integer array
        ///
        /// # Returns
        ///
        /// Maximum rotation function value
        pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
            let n = nums.len() as i64;
            let sum: i64 = nums.iter().map(|&v| v as i64).sum();
            let f0: i64 = nums
                .iter()
                .enumerate()
                .map(|(i, &v)| i as i64 * v as i64)
                .sum();

            // Iterate k from 1..n, computing F(k) incrementally
            let (_, ans) = (1..nums.len()).fold((f0, f0), |(prev, best), i| {
                let cur = prev + sum - n * nums[nums.len() - i] as i64;
                (cur, best.max(cur))
            });
            ans as i32
        }
    }
}

fn main() {
    let cases = [
        (vec![4, 3, 2, 6], 26),
        (vec![100], 0),
    ];

    println!("--- My Solution ---");
    for (nums, expected) in &cases {
        let result =
            my_solution::Solution::max_rotate_function(nums.clone());
        println!("  {nums:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (nums, expected) in &cases {
        let result =
            ai_solution::Solution::max_rotate_function(nums.clone());
        println!("  {nums:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_rotate {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert_eq!(
                        Solution::max_rotate_function(vec![4, 3, 2, 6]),
                        26
                    );
                }

                #[test]
                fn test_single() {
                    assert_eq!(
                        Solution::max_rotate_function(vec![100]),
                        0
                    );
                }

                #[test]
                fn test_two_elements() {
                    // F(0) = 0*1 + 1*2 = 2, F(1) = 0*2 + 1*1 = 1
                    assert_eq!(
                        Solution::max_rotate_function(vec![1, 2]),
                        2
                    );
                }

                #[test]
                fn test_negatives() {
                    // F(0)= 0*(-1) + 1*2 + 2*(-3) = -4
                    // F(1)= 0*(-3) + 1*(-1) + 2*2 = 3
                    // F(2)= 0*2 + 1*(-3) + 2*(-1) = -5
                    assert_eq!(
                        Solution::max_rotate_function(vec![-1, 2, -3]),
                        3
                    );
                }

                #[test]
                fn test_all_zeros() {
                    assert_eq!(
                        Solution::max_rotate_function(vec![0, 0, 0]),
                        0
                    );
                }
            }
        };
    }

    test_rotate!(my_tests, crate::my_solution::Solution);
    test_rotate!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 200_000_u32;
        let nums: Vec<i32> = (0..100_000).map(|i| (i % 201) - 100).collect();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::max_rotate_function(
                    std::hint::black_box(nums.clone()),
                ),
            );
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::max_rotate_function(
                    std::hint::black_box(nums.clone()),
                ),
            );
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution:  {:.2?} (avg {:.2?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "AI solution:  {:.2?} (avg {:.2?})",
            ai_duration,
            ai_duration / iterations
        );
        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.0 {
            println!("-> AI solution is {ratio:.2}x faster");
        } else {
            println!("-> My solution is {:.2}x faster", 1.0 / ratio);
        }
    }
}
