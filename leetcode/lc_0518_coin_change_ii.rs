// LeetCode 518: Coin Change II [Medium]
// https://leetcode.com/problems/coin-change-ii/
//
// Given an integer array `coins` representing coins of different denominations
// and an integer `amount` representing a total amount of money, return the
// number of combinations that make up that amount. If that amount cannot be
// made up by any combination of the coins, return 0.
//
// You may assume that you have an infinite number of each kind of coin.
// The answer is guaranteed to fit into a signed 32-bit integer.
//
// Examples:
//   amount=5, coins=[1,2,5] -> 4   (5, 2+2+1, 2+1+1+1, 1+1+1+1+1)
//   amount=3, coins=[2]     -> 0
//   amount=10, coins=[10]   -> 1
//
// Constraints:
//   1 <= coins.length <= 300
//   1 <= coins[i] <= 5000
//   All the values of coins are unique
//   0 <= amount <= 5000
//
// Run:  cargo run --example lc_0518_coin_change_ii
// Test: cargo test --example lc_0518_coin_change_ii
// Bench: cargo test --example lc_0518_coin_change_ii bench -- --nocapture
//
// First benchmark (100K iterations, 2026-04-08):
//   My solution: avg 5.90us
//   AI solution: avg 3.63us
//   AI is 1.62x faster
//   Reason: 1D DP avoids allocating and iterating a 2D table; single-row
//   updates have better cache locality and fewer memory accesses.

/// 2D DP (complete knapsack) — f[i][j] = number of ways to make amount j
/// using the first i coin types.
///
/// Adapted from doocs/leetcode Java solution.
/// Transition: f[i][j] = f[i-1][j] + f[i][j - coins[i-1]] (if j >= coin).
///
/// Time:  O(m * n) where m = coins.len(), n = amount
/// Space: O(m * n)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the number of coin combinations that sum to `amount`.
        ///
        /// # Arguments
        /// * `amount` - target sum
        /// * `coins` - available coin denominations
        ///
        /// # Returns
        /// Number of combinations
        pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
            let m = coins.len();
            let n = amount as usize;
            // f[i][j] = ways to make amount j using first i coin types
            let mut f = vec![vec![0i32; n + 1]; m + 1];
            // Base case: one way to make amount 0 (use no coins)
            f[0][0] = 1;
            for i in 1..=m {
                let coin = coins[i - 1] as usize;
                for j in 0..=n {
                    // Don't use coin i at all
                    f[i][j] = f[i - 1][j];
                    // Use at least one more of coin i (complete knapsack:
                    // reference f[i][j-coin] not f[i-1][j-coin])
                    if j >= coin {
                        f[i][j] += f[i][j - coin];
                    }
                }
            }
            f[m][n]
        }
    }
}

/// Space-optimized 1D DP — compress the 2D table to a single row.
///
/// Since f[i][j] depends on f[i-1][j] (same position, previous row)
/// and f[i][j-coin] (same row, earlier column), we can iterate left to
/// right and overwrite in place.
///
/// Time:  O(m * n)
/// Space: O(n)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the number of coin combinations that sum to `amount`.
        ///
        /// # Arguments
        /// * `amount` - target sum
        /// * `coins` - available coin denominations
        ///
        /// # Returns
        /// Number of combinations
        pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
            let n = amount as usize;
            let mut dp = vec![0i32; n + 1];
            dp[0] = 1; // one way to make amount 0
            // For each coin denomination, update the DP array
            for &coin in &coins {
                let c = coin as usize;
                for j in c..=n {
                    dp[j] += dp[j - c];
                }
            }
            dp[n]
        }
    }
}

fn main() {
    println!(
        "my  (5, [1,2,5]): {}",
        my_solution::Solution::change(5, vec![1, 2, 5])
    );
    println!(
        "ai  (5, [1,2,5]): {}",
        ai_solution::Solution::change(5, vec![1, 2, 5])
    );
    println!(
        "my  (3, [2]): {}",
        my_solution::Solution::change(3, vec![2])
    );
    println!(
        "ai  (3, [2]): {}",
        ai_solution::Solution::change(3, vec![2])
    );
    println!(
        "my  (10, [10]): {}",
        my_solution::Solution::change(10, vec![10])
    );
    println!(
        "ai  (10, [10]): {}",
        ai_solution::Solution::change(10, vec![10])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! coin_change_tests {
        ($mod:ident, $solution:path) => {
            mod $mod {
                use super::*;

                #[test]
                fn test_example1() {
                    assert_eq!(<$solution>::change(5, vec![1, 2, 5]), 4);
                }

                #[test]
                fn test_example2() {
                    assert_eq!(<$solution>::change(3, vec![2]), 0);
                }

                #[test]
                fn test_example3() {
                    assert_eq!(<$solution>::change(10, vec![10]), 1);
                }

                #[test]
                fn test_amount_zero() {
                    assert_eq!(<$solution>::change(0, vec![1, 2, 5]), 1);
                }

                #[test]
                fn test_single_coin_divides() {
                    assert_eq!(<$solution>::change(6, vec![3]), 1);
                }

                #[test]
                fn test_single_coin_no_divide() {
                    assert_eq!(<$solution>::change(7, vec![3]), 0);
                }

                #[test]
                fn test_multiple_coins() {
                    assert_eq!(<$solution>::change(5, vec![1, 2]), 3);
                }

                #[test]
                fn test_large_amount() {
                    // 500 with [1,2,5] has many combinations
                    assert_eq!(<$solution>::change(500, vec![1, 2, 5]), 12701);
                }
            }
        };
    }

    coin_change_tests!(my_tests, my_solution::Solution);
    coin_change_tests!(ai_tests, ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let amount = 500;
        let coins = vec![1, 2, 5, 10, 25, 50, 100];
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(my_solution::Solution::change(
                std::hint::black_box(amount),
                std::hint::black_box(coins.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(ai_solution::Solution::change(
                std::hint::black_box(amount),
                std::hint::black_box(coins.clone()),
            ));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur / iterations;
        let ai_avg = ai_dur / iterations;
        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("  My solution (2D DP): {my_avg:?} avg");
        println!("  AI solution (1D DP): {ai_avg:?} avg");
        if ai_avg < my_avg {
            println!(
                "  AI is {:.2}x faster",
                my_avg.as_nanos() as f64 / ai_avg.as_nanos() as f64
            );
        } else {
            println!(
                "  My is {:.2}x faster",
                ai_avg.as_nanos() as f64 / my_avg.as_nanos() as f64
            );
        }
    }
}
