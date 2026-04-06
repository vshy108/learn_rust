// LeetCode 122: Best Time to Buy and Sell Stock II [Medium]
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/
//
// You are given an integer array prices where prices[i] is the price of
// a given stock on the ith day. On each day, you may decide to buy and/or
// sell the stock. You can only hold at most one share at any time. However,
// you can buy it then immediately sell it on the same day. Find and return
// the maximum profit you can achieve.
//
// Examples:
//   prices = [7,1,5,3,6,4] -> 7  (buy@1 sell@5 + buy@3 sell@6 = 4+3 = 7)
//   prices = [1,2,3,4,5]   -> 4  (buy@1 sell@5 = 4)
//   prices = [7,6,4,3,1]   -> 0  (no profitable transaction)
//
// Constraints:
//   - 1 <= prices.length <= 3 * 10^4
//   - 0 <= prices[i] <= 10^4
//
// Run:  cargo run --example lc_0122_best_time_to_buy_and_sell_stock_ii
// Test: cargo test --example lc_0122_best_time_to_buy_and_sell_stock_ii
// Bench: cargo test --example lc_0122_best_time_to_buy_and_sell_stock_ii bench -- --nocapture
//
// --- Rust concepts used ---
// - .windows(2): iterator adaptor yielding overlapping &[i32] slices of
//   length 2 — no index variables, no off-by-one risk
// - Greedy proof: summing all positive consecutive differences equals the
//   optimal multi-transaction profit (every uphill segment is captured)
//   (Previously covered: index-based loop with range lc_0027,
//    iterator chain .filter().map().sum() lc_0005)
//
// --- Benchmark (100_000 iterations, 2026-04-02) ---
// My solution:  53.04ms (avg 530ns)
// AI solution:  59.30ms (avg 593ns)
// -> My solution is 1.12x faster
// Reason: the index-based loop has less overhead per iteration than
//   .windows(2) which creates a slice reference + closure dispatch per
//   element; for these small inputs the pipeline setup cost dominates.

/// My solution module — greedy index-based loop.
///
/// Collect every upward price movement: if today > yesterday, add the
/// difference. This works because buying at every local minimum and
/// selling at every local maximum is equivalent to summing all positive
/// consecutive differences.
///
/// Time:  O(n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the maximum profit with unlimited buy-sell transactions.
        ///
        /// # Arguments
        ///
        /// * `prices` - Stock prices on each day
        ///
        /// # Returns
        ///
        /// Maximum profit achievable with multiple transactions
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            let mut profit = 0;
            for i in 1..prices.len() {
                if prices[i] > prices[i - 1] {
                    profit += prices[i] - prices[i - 1];
                }
            }

            profit
        }
    }
}

/// AI solution module — iterator windows approach.
///
/// Same greedy idea but uses `.windows(2)` to iterate consecutive pairs
/// instead of manual indexing. More idiomatic Rust — no index variables,
/// no bounds checking, and the slice pattern `[a, b]` destructures directly.
///
/// Time:  O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the maximum profit with unlimited buy-sell transactions.
        ///
        /// # Arguments
        ///
        /// * `prices` - Stock prices on each day
        ///
        /// # Returns
        ///
        /// Maximum profit achievable with multiple transactions
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            // .windows(2) yields overlapping slices [prices[i], prices[i+1]].
            // Slice pattern [a, b] destructures each window into two &i32 refs.
            // .filter(|w| w[1] > w[0]) keeps only upward movements.
            // .map(|w| w[1] - w[0]) extracts the profit from each gain.
            // .sum() collects all positive differences into the total profit.
            prices
                .windows(2)
                .filter(|w| w[1] > w[0])
                .map(|w| w[1] - w[0])
                .sum()
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32)> = vec![
        (vec![7, 1, 5, 3, 6, 4], 7),
        (vec![1, 2, 3, 4, 5], 4),
        (vec![7, 6, 4, 3, 1], 0),
    ];

    println!("--- My Solution ---");
    for (prices, expected) in &test_cases {
        let result = my_solution::Solution::max_profit(prices.clone());
        println!("  {prices:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (prices, expected) in &test_cases {
        let result = ai_solution::Solution::max_profit(prices.clone());
        println!("  {prices:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_max_profit {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_multiple_transactions() {
                    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
                }

                #[test]
                fn test_ascending_prices() {
                    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
                }

                #[test]
                fn test_descending_prices() {
                    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
                }

                #[test]
                fn test_single_day() {
                    assert_eq!(Solution::max_profit(vec![5]), 0);
                }

                #[test]
                fn test_two_days_profit() {
                    assert_eq!(Solution::max_profit(vec![1, 5]), 4);
                }

                #[test]
                fn test_all_same_price() {
                    assert_eq!(Solution::max_profit(vec![3, 3, 3]), 0);
                }

                #[test]
                fn test_zigzag() {
                    // buy@1 sell@5, buy@2 sell@8 = 4 + 6 = 10
                    assert_eq!(Solution::max_profit(vec![1, 5, 2, 8]), 10);
                }
            }
        };
    }

    test_max_profit!(my_tests, crate::my_solution::Solution);
    test_max_profit!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<Vec<i32>> = vec![
            vec![7, 1, 5, 3, 6, 4],
            vec![1, 2, 3, 4, 5],
            vec![7, 6, 4, 3, 1],
            vec![1, 5, 2, 8],
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for prices in &inputs {
                std::hint::black_box(crate::my_solution::Solution::max_profit(
                    std::hint::black_box(prices.clone()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for prices in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::max_profit(
                    std::hint::black_box(prices.clone()),
                ));
            }
        }
        let ai_duration = start.elapsed();

        println!("\n=== Benchmark ({iterations} iterations) ===");
        println!(
            "  My solution: {:?} (avg {:?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "  AI solution: {:?} (avg {:?})",
            ai_duration,
            ai_duration / iterations
        );

        if my_duration < ai_duration {
            let ratio = ai_duration.as_nanos() as f64 / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64 / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
