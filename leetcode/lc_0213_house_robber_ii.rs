// LeetCode 213: House Robber II [Medium]
// https://leetcode.com/problems/house-robber-ii/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.254us
//   AI solution avg: 0.161us
//   Ratio: AI 1.58x faster
//
// Houses are arranged in a circle. You cannot rob two adjacent houses,
// and the first and last houses are also adjacent. Return the maximum
// amount of money you can rob.
//
// Constraints: 1 <= nums.length <= 100, 0 <= nums[i] <= 1000
//
// Run:  cargo run --example lc_0213_house_robber_ii
// Test: cargo test --example lc_0213_house_robber_ii
// Bench: cargo test --example lc_0213_house_robber_ii bench -- --nocapture

/// My solution — DP with full arrays, two passes: once excluding the
/// last house, once excluding the first house, take the max.
///
/// Time: O(n).
/// Space: O(n) for the two DP arrays.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the maximum amount you can rob in a circular arrangement.
        pub fn rob(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            if n == 1 {
                return nums[0];
            }
            if n == 2 {
                return nums[0].max(nums[1]);
            }
            // Rob houses [0..n-2] (exclude last) or [1..n-1] (exclude first)
            Self::rob_range(&nums[..n - 1]).max(Self::rob_range(&nums[1..]))
        }

        /// Standard House Robber DP on a linear subarray using a full array.
        fn rob_range(houses: &[i32]) -> i32 {
            let n = houses.len();
            let mut dp = vec![0; n];
            dp[0] = houses[0];
            dp[1] = houses[0].max(houses[1]);
            for i in 2..n {
                dp[i] = dp[i - 1].max(dp[i - 2] + houses[i]);
            }
            dp[n - 1]
        }
    }
}

/// AI solution — same two-pass logic but with O(1) space using two
/// rolling variables instead of a full DP array.
///
/// Time: O(n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the maximum amount you can rob in a circular arrangement.
        pub fn rob(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            if n == 1 {
                return nums[0];
            }
            // Two passes with rolling variables
            Self::rob_range(&nums[..n - 1]).max(Self::rob_range(&nums[1..]))
        }

        /// House Robber DP on a linear subarray with O(1) space.
        fn rob_range(houses: &[i32]) -> i32 {
            // prev2 = best up to i-2, prev1 = best up to i-1
            let (mut prev2, mut prev1) = (0, 0);
            for &money in houses {
                let current = prev1.max(prev2 + money);
                prev2 = prev1;
                prev1 = current;
            }
            prev1
        }
    }
}

fn main() {
    let nums = vec![2, 3, 2];
    let my = my_solution::Solution::rob(nums.clone());
    let ai = ai_solution::Solution::rob(nums);
    println!("max rob: my={my}, ai={ai}");

    let nums2 = vec![1, 2, 3, 1];
    let my2 = my_solution::Solution::rob(nums2.clone());
    let ai2 = ai_solution::Solution::rob(nums2);
    println!("max rob: my={my2}, ai={ai2}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_rob {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    // [2,3,2] circle: can't rob both 2,2 => rob 3 = 3
                    assert_eq!(<$solution>::rob(vec![2, 3, 2]), 3);
                }

                #[test]
                fn test_example2() {
                    // [1,2,3,1] circle: rob 1+3 = 4
                    assert_eq!(<$solution>::rob(vec![1, 2, 3, 1]), 4);
                }

                #[test]
                fn test_example3() {
                    assert_eq!(<$solution>::rob(vec![1, 2, 3]), 3);
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
                    // [4,4,4,4] circle: best is rob two non-adjacent = 8
                    assert_eq!(<$solution>::rob(vec![4, 4, 4, 4]), 8);
                }

                #[test]
                fn test_large_values() {
                    // [200,3,140,20,10] circle: rob 200+140 = 340
                    // or 3+20 = 23, or 200+10 forbidden (adjacent in circle)
                    // range [0..3]: [200,3,140,20] => 200+140=340
                    // range [1..4]: [3,140,20,10] => 140+10=150
                    assert_eq!(<$solution>::rob(vec![200, 3, 140, 20, 10]), 340);
                }
            }
        };
    }

    test_rob!(my, crate::my_solution::Solution);
    test_rob!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums: Vec<i32> = (0..100).map(|i| (i * 7 + 3) % 1000).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::rob(
                std::hint::black_box(nums.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::rob(
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
