// LeetCode 123: Best Time to Buy and Sell Stock III [Hard]
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.670us
//   AI solution avg: 0.303us
//   Ratio: AI 2.21x faster
//
// You are given an array prices. Find the maximum profit you can achieve with
// at most two transactions. You may not engage in multiple transactions
// simultaneously (you must sell before you buy again).
//
// Constraints: 1 <= prices.length <= 10^5, 0 <= prices[i] <= 10^5
//
// Run:  cargo run --example lc_0123_best_time_stock_iii
// Test: cargo test --example lc_0123_best_time_stock_iii
// Bench: cargo test --example lc_0123_best_time_stock_iii bench -- --nocapture

/// My solution — split array at each point, compute best single trade on each side.
///
/// Time: O(n) using prefix/suffix max profit arrays.
/// Space: O(n) for the two arrays.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find max profit with at most 2 transactions.
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            let n = prices.len();
            if n < 2 { return 0; }

            // left[i] = max profit from one transaction in prices[0..=i]
            let mut left = vec![0; n];
            let mut min_price = prices[0];
            for i in 1..n {
                min_price = min_price.min(prices[i]);
                left[i] = left[i - 1].max(prices[i] - min_price);
            }

            // right[i] = max profit from one transaction in prices[i..n]
            let mut right = vec![0; n];
            let mut max_price = prices[n - 1];
            for i in (0..n - 1).rev() {
                max_price = max_price.max(prices[i]);
                right[i] = right[i + 1].max(max_price - prices[i]);
            }

            // Max of left[i] + right[i] for all split points
            (0..n).map(|i| left[i] + right[i]).max().expect("n >= 1")
        }
    }
}

/// AI solution — state machine with 4 variables, O(1) space.
///
/// Track: buy1, sell1, buy2, sell2
///
/// Time: O(n) single pass.
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find max profit with state machine approach.
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            // buy1: min cost after first buy
            // sell1: max profit after first sell
            // buy2: min cost after second buy (profit from sell1 offsets)
            // sell2: max profit after second sell
            let mut buy1 = i32::MIN;
            let mut sell1 = 0;
            let mut buy2 = i32::MIN;
            let mut sell2 = 0;

            for price in prices {
                buy1 = buy1.max(-price);
                sell1 = sell1.max(buy1 + price);
                buy2 = buy2.max(sell1 - price);
                sell2 = sell2.max(buy2 + price);
            }
            sell2
        }
    }
}

fn main() {
    let prices = vec![3, 3, 5, 0, 0, 3, 1, 4];
    let my = my_solution::Solution::max_profit(prices.clone());
    let ai = ai_solution::Solution::max_profit(prices);
    println!("max profit: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_stock {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(
                        <$solution>::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]),
                        6,
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        <$solution>::max_profit(vec![1, 2, 3, 4, 5]),
                        4,
                    );
                }

                #[test]
                fn test_no_profit() {
                    assert_eq!(
                        <$solution>::max_profit(vec![7, 6, 4, 3, 1]),
                        0,
                    );
                }

                #[test]
                fn test_single() {
                    assert_eq!(<$solution>::max_profit(vec![1]), 0);
                }

                #[test]
                fn test_two_clear_trades() {
                    assert_eq!(
                        <$solution>::max_profit(vec![1, 5, 1, 5]),
                        8,
                    );
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(<$solution>::max_profit(vec![3, 3, 3, 3]), 0);
                }
            }
        };
    }

    test_stock!(my, crate::my_solution::Solution);
    test_stock!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let prices: Vec<i32> = (0..200).map(|i| (i * 37 + 13) % 100).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::max_profit(
                std::hint::black_box(prices.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::max_profit(
                std::hint::black_box(prices.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );
        println!(
            "AI solution: {:.3}us avg",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("~tied (ratio: {ratio:.2})");
        }
    }
}
