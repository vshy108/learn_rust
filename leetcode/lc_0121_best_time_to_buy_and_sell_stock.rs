// LeetCode 121: Best Time to Buy and Sell Stock [Easy]
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
//
// You are given an array prices where prices[i] is the price of a given
// stock on the ith day. You want to maximize your profit by choosing a
// single day to buy one stock and choosing a different day in the future
// to sell that stock. Return the maximum profit you can achieve from this
// transaction. If you cannot achieve any profit, return 0.
//
// Examples:
//   prices = [7,1,5,3,6,4] -> 5  (buy day 2, sell day 5: 6 - 1 = 5)
//   prices = [7,6,4,3,1]   -> 0  (no profitable transaction)
//
// Constraints:
//   - 1 <= prices.length <= 10^5
//   - 0 <= prices[i] <= 10^4
//
// Run:  cargo run --example lc_0121_best_time_to_buy_and_sell_stock
// Test: cargo test --example lc_0121_best_time_to_buy_and_sell_stock
// Bench: cargo test --example lc_0121_best_time_to_buy_and_sell_stock bench -- --nocapture
//
// --- Rust concepts used ---
// - Slice borrowing: &prices[1..] produces &[i32] from Vec<i32>
// - DST vs fat pointer: [i32] is unsized; &[i32] is a fat pointer that
//   implements IntoIterator (see Slices section in CHEATSHEET.md)
// - Pattern destructuring in for: `for &price in &slice` copies i32 from &i32
// - Iterator fold: .fold((acc1, acc2), |acc, &item| ...) for stateful reduction
// - Tuple destructuring: .1 to extract second element from fold result
// - i32::MAX as sentinel initial value for min tracking
//
// --- Benchmark (100_000 iterations, 2026-04-02) ---
// My solution:  47.08ms (avg 470ns)
// AI solution:  52.35ms (avg 523ns)
// -> My solution is 1.11x faster
// Reason: imperative loop with early slice skip (prices[1..]) avoids
//   processing the first element twice, and the branch predictor handles
//   the simple if/min/max pattern well. The fold version evaluates
//   min_price.min(price) before max_profit.max(price - min_price) on the
//   first element, comparing against i32::MAX — a harmless but extra step.

/// My solution module — imperative single-pass with tracking min price.
///
/// Time:  O(n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the maximum profit from a single buy-sell transaction.
        ///
        /// # Arguments
        ///
        /// * `prices` - Stock prices on each day
        ///
        /// # Returns
        ///
        /// Maximum profit achievable, or 0 if no profit possible
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            if prices.len() < 2 { return 0; };

            let mut min_price = prices[0];
            let mut max_profit = 0;
            // &prices[1..] borrows the slice — prices[1..] alone is an owned
            // [i32] which doesn't implement IntoIterator (unsized DST can't
            // call into_iter()). The & gives &[i32] (fat pointer: address +
            // length) which does implement IntoIterator, yielding &i32.
            // &price destructures each &i32 into i32 (Copy type).
            for &price in &prices[1..]{
              let potential_profit = price - min_price;
              max_profit = max_profit.max(potential_profit);
              min_price = min_price.min(price);
            }

            max_profit
        }
    }
}

/// AI solution module — iterator fold approach.
///
/// Same O(n) algorithm but expressed as a functional fold instead of an
/// imperative loop. The fold accumulates (min_price, max_profit) in a
/// single pass with no indexing or early-return branching.
///
/// Time:  O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the maximum profit from a single buy-sell transaction.
        ///
        /// # Arguments
        ///
        /// * `prices` - Stock prices on each day
        ///
        /// # Returns
        ///
        /// Maximum profit achievable, or 0 if no profit possible
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            // fold accumulates (min_price_so_far, max_profit_so_far) in one pass.
            // .iter() yields &i32; destructure with |&price| to get i32 (Copy).
            // Initial min = i32::MAX so any first price becomes the new minimum.
            prices
                .iter()
                .fold((i32::MAX, 0), |(min_price, max_profit), &price| {
                    (min_price.min(price), max_profit.max(price - min_price))
                })
                .1 // extract max_profit from the (min_price, max_profit) tuple
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32)> =
        vec![(vec![7, 1, 5, 3, 6, 4], 5), (vec![7, 6, 4, 3, 1], 0)];

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
                fn test_basic_profit() {
                    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
                }

                #[test]
                fn test_no_profit() {
                    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
                }

                #[test]
                fn test_single_day() {
                    assert_eq!(Solution::max_profit(vec![5]), 0);
                }

                #[test]
                fn test_two_days_profit() {
                    assert_eq!(Solution::max_profit(vec![1, 2]), 1);
                }

                #[test]
                fn test_two_days_no_profit() {
                    assert_eq!(Solution::max_profit(vec![2, 1]), 0);
                }

                #[test]
                fn test_all_same_price() {
                    assert_eq!(Solution::max_profit(vec![3, 3, 3, 3]), 0);
                }

                #[test]
                fn test_profit_at_end() {
                    assert_eq!(Solution::max_profit(vec![2, 4, 1, 7]), 6);
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
            vec![7, 6, 4, 3, 1],
            vec![2, 4, 1, 7],
            vec![3, 3, 3, 3],
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
