// LeetCode 188: Best Time to Buy and Sell Stock IV [Hard]
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 1.583us
//   AI solution avg: 1.394us
//   Ratio: AI 1.14x faster
//
// Given prices array and integer k, find the maximum profit with at most
// k transactions. You may not hold more than one share at a time.
//
// Constraints: 1 <= k <= 100, 1 <= prices.length <= 1000,
//   0 <= prices[i] <= 1000
//
// Run:  cargo run --example lc_0188_best_time_stock_iv
// Test: cargo test --example lc_0188_best_time_stock_iv
// Bench: cargo test --example lc_0188_best_time_stock_iv bench -- --nocapture

/// My solution — if k >= n/2 use greedy (sum all positive diffs);
/// otherwise use O(k) DP with buy[j] and sell[j] arrays tracking the
/// best profit after the j-th buy and j-th sell respectively.
///
/// Time: O(n · k) in general; O(n) when k >= n/2.
/// Space: O(k).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return max profit with at most k transactions.
        pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
            let n = prices.len();
            if n < 2 {
                return 0;
            }
            let k = k as usize;
            // If k >= n/2, unlimited transactions
            if k >= n / 2 {
                return prices.windows(2).map(|w| (w[1] - w[0]).max(0)).sum();
            }
            // 2D DP: dp[j][0] = not holding, dp[j][1] = holding after j-th tx
            let mut buy = vec![i32::MIN; k];
            let mut sell = vec![0; k];
            for &price in &prices {
                for j in 0..k {
                    let prev_sell = if j == 0 { 0 } else { sell[j - 1] };
                    buy[j] = buy[j].max(prev_sell - price);
                    sell[j] = sell[j].max(buy[j] + price);
                }
            }
            *sell.last().expect("k >= 1")
        }
    }
}

/// AI solution — DP with states: buy[j] and sell[j] for j-th transaction.
///
/// If k >= n/2, it's equivalent to unlimited transactions (greedy sum
/// of all positive diffs). Otherwise, use O(k) DP.
///
/// buy[j]  = max profit so far when holding one share after j-th buy.
/// sell[j] = max profit so far after j-th sell.
///
/// Time: O(n * k) or O(n) when k >= n/2.
/// Space: O(k).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// DP approach with optimization for large k.
        pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
            let n = prices.len();
            if n < 2 {
                return 0;
            }
            let k = k as usize;

            // If k >= n/2, unlimited transactions
            if k >= n / 2 {
                return prices.windows(2).map(|w| (w[1] - w[0]).max(0)).sum();
            }

            // buy[j]  = max profit holding stock after j-th buy
            // sell[j] = max profit not holding stock after j-th sell
            let mut buy = vec![i32::MIN; k];
            let mut sell = vec![0; k];

            for &price in &prices {
                for j in 0..k {
                    let prev_sell = if j == 0 { 0 } else { sell[j - 1] };
                    buy[j] = buy[j].max(prev_sell - price);
                    sell[j] = sell[j].max(buy[j] + price);
                }
            }
            *sell.last().expect("k >= 1")
        }
    }
}

fn main() {
    let my = my_solution::Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]);
    let ai = ai_solution::Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]);
    println!("max profit: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_stock {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(<$solution>::max_profit(2, vec![2, 4, 1]), 2);
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        <$solution>::max_profit(2, vec![3, 2, 6, 5, 0, 3]),
                        7, // buy@2,sell@6 + buy@0,sell@3
                    );
                }

                #[test]
                fn test_k_one() {
                    assert_eq!(<$solution>::max_profit(1, vec![7, 1, 5, 3, 6, 4]), 5,);
                }

                #[test]
                fn test_decreasing() {
                    assert_eq!(<$solution>::max_profit(2, vec![5, 4, 3, 2, 1]), 0);
                }

                #[test]
                fn test_unlimited_k() {
                    assert_eq!(
                        <$solution>::max_profit(100, vec![1, 2, 3, 4, 5]),
                        4, // just hold from 1 to 5
                    );
                }
            }
        };
    }

    test_stock!(my, crate::my_solution::Solution);
    test_stock!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let prices: Vec<i32> = (0..200).map(|i| ((i * 7 + 3) % 100) as i32).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::max_profit(
                std::hint::black_box(5),
                std::hint::black_box(prices.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::max_profit(
                std::hint::black_box(5),
                std::hint::black_box(prices.clone()),
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
