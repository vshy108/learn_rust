// LeetCode 198: House Robber [Medium]
// https://leetcode.com/problems/house-robber/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.260us
//   AI solution avg: 0.159us
//   Ratio: AI 1.63x faster
//
// Given an integer array nums representing the amount of money per house,
// return the maximum amount of money you can rob without robbing two
// adjacent houses.
//
// Constraints: 1 <= nums.length <= 100, 0 <= nums[i] <= 400
//
// Run:  cargo run --example lc_0198_house_robber
// Test: cargo test --example lc_0198_house_robber
// Bench: cargo test --example lc_0198_house_robber bench -- --nocapture

/// My solution — DP with a full array: dp[i] = max(dp[i-1], dp[i-2] + nums[i]).
///
/// Time: O(n).
/// Space: O(n) for the dp array.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the maximum amount of money you can rob.
        pub fn rob(nums: Vec<i32>) -> i32 {
            // DP with full array
            let n = nums.len();
            if n == 1 {
                return nums[0];
            }
            let mut dp = vec![0; n];
            dp[0] = nums[0];
            dp[1] = nums[0].max(nums[1]);
            for i in 2..n {
                dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
            }
            dp[n - 1]
        }
    }
}

/// AI solution — DP with two variables.
///
/// At each house, choose between robbing it (prev2 + nums[i]) or
/// skipping it (prev1). Only need the last two cumulative values.
///
/// Time: O(n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// DP with two rolling variables — O(1) space.
        pub fn rob(nums: Vec<i32>) -> i32 {
            // prev2 = best up to i-2, prev1 = best up to i-1
            let (mut prev2, mut prev1) = (0, 0);
            for &money in &nums {
                let current = prev1.max(prev2 + money);
                prev2 = prev1;
                prev1 = current;
            }
            prev1
        }
    }
}

fn main() {
    let my = my_solution::Solution::rob(vec![2, 7, 9, 3, 1]);
    let ai = ai_solution::Solution::rob(vec![2, 7, 9, 3, 1]);
    println!("max rob: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_rob {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(<$solution>::rob(vec![1, 2, 3, 1]), 4);
                }

                #[test]
                fn test_example2() {
                    assert_eq!(<$solution>::rob(vec![2, 7, 9, 3, 1]), 12);
                }

                #[test]
                fn test_single() {
                    assert_eq!(<$solution>::rob(vec![5]), 5);
                }

                #[test]
                fn test_two() {
                    assert_eq!(<$solution>::rob(vec![3, 10]), 10);
                }

                #[test]
                fn test_all_same() {
                    // [4,4,4,4] -> rob 0 and 2 = 8, or 1 and 3 = 8
                    assert_eq!(<$solution>::rob(vec![4, 4, 4, 4]), 8);
                }
            }
        };
    }

    test_rob!(my, crate::my_solution::Solution);
    test_rob!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums: Vec<i32> = (0..100).map(|i| (i * 3 + 7) % 400).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::rob(std::hint::black_box(
                nums.clone(),
            )));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::rob(std::hint::black_box(
                nums.clone(),
            )));
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
