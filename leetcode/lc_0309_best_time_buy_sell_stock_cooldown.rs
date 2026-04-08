// LeetCode 309: Best Time to Buy and Sell Stock with Cooldown [Medium]
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/
// Find max profit with unlimited transactions and 1-day cooldown after selling.
//
// Benchmark: 100_000 iterations
// My solution (DP array):         avg 0.082us
// AI solution (O(1) space DP):    avg 0.029us
// Winner: AI solution (2.84x faster)

// ------ My Solution: DP with f[i][0] and f[i][1] ------
// Time: O(n) — single pass
// Space: O(n) — dp array of size n x 2
/// `f[i][0]` = max profit on day i without holding stock.
/// `f[i][1]` = max profit on day i holding stock.
/// Transitions:
///   f[i][0] = max(f[i-1][0], f[i-1][1] + prices[i])  (hold or sell)
///   f[i][1] = max(f[i-1][1], f[i-2][0] - prices[i])  (hold or buy after cooldown)
mod my_solution {
    pub struct Solution;

    impl Solution {
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            let n = prices.len();
            if n <= 1 {
                return 0;
            }
            // f[i][0] = not holding, f[i][1] = holding
            let mut f = vec![[0i32; 2]; n];
            f[0][1] = -prices[0];
            for i in 1..n {
                f[i][0] = f[i - 1][0].max(f[i - 1][1] + prices[i]);
                // Can only buy if cooldown passed: use f[i-2][0]
                let prev_no_hold = if i > 1 { f[i - 2][0] } else { 0 };
                f[i][1] = f[i - 1][1].max(prev_no_hold - prices[i]);
            }
            f[n - 1][0]
        }
    }
}

// ------ AI Solution: O(1) space DP with 3 variables ------
// Time: O(n) — single pass
// Space: O(1) — three variables
/// `f` = profit without stock two days ago, `f0` = profit without stock yesterday,
/// `f1` = profit with stock yesterday. Each day, compute new f0 and f1.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            // f = f[i-2][0], f0 = f[i-1][0], f1 = f[i-1][1]
            let (mut f, mut f0, mut f1) = (0, 0, -prices[0]);
            for &x in &prices[1..] {
                let g0 = f0.max(f1 + x); // sell or hold (no stock)
                f1 = f1.max(f - x); // hold or buy (cooldown from f[i-2])
                f = f0; // advance "two days ago" window
                f0 = g0;
            }
            f0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::Solution as AiSolution;
    use super::my_solution::Solution as MySolution;

    #[test]
    fn test_example1() {
        let prices = vec![1, 2, 3, 0, 2];
        assert_eq!(MySolution::max_profit(prices.clone()), 3);
        assert_eq!(AiSolution::max_profit(prices), 3);
    }

    #[test]
    fn test_example2() {
        assert_eq!(MySolution::max_profit(vec![1]), 0);
        assert_eq!(AiSolution::max_profit(vec![1]), 0);
    }

    #[test]
    fn test_two_days_profit() {
        assert_eq!(MySolution::max_profit(vec![1, 2]), 1);
        assert_eq!(AiSolution::max_profit(vec![1, 2]), 1);
    }

    #[test]
    fn test_two_days_no_profit() {
        assert_eq!(MySolution::max_profit(vec![2, 1]), 0);
        assert_eq!(AiSolution::max_profit(vec![2, 1]), 0);
    }

    #[test]
    fn test_decreasing() {
        assert_eq!(MySolution::max_profit(vec![5, 4, 3, 2, 1]), 0);
        assert_eq!(AiSolution::max_profit(vec![5, 4, 3, 2, 1]), 0);
    }

    #[test]
    fn test_increasing() {
        // Buy day 0, sell day 4: profit = 4
        assert_eq!(MySolution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(AiSolution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn test_cooldown_matters() {
        // Without cooldown: buy 1, sell 4, buy 1, sell 3 = 5
        // With cooldown: buy 1, sell 4, cooldown, buy 1, sell 3 = 5 (if enough days)
        let prices = vec![1, 4, 2, 1, 3];
        // buy@1, sell@4, cooldown, buy@1, sell@3 = 3+2=5
        assert_eq!(MySolution::max_profit(prices.clone()), 5);
        assert_eq!(AiSolution::max_profit(prices), 5);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(MySolution::max_profit(vec![3, 3, 3, 3]), 0);
        assert_eq!(AiSolution::max_profit(vec![3, 3, 3, 3]), 0);
    }

    #[test]
    fn test_zigzag() {
        // 1, 5, 1, 5: buy@1 sell@5, cooldown, buy@1 sell@5 = 8? No, cooldown!
        // buy@1 sell@5 (+4), cooldown at day 2, buy@1(day3) sell@5(day3)? No 4 days only
        // 4 days: buy@0 sell@1=+4, cooldown day2, buy@3 not possible only 4 days(0-3)
        // Actually [1,5,1,5] has 4 elements, indices 0-3
        // buy@0(1), sell@1(5)=+4, cooldown@2, buy@3(5)? That loses money.
        // Best: just buy@0 sell@1=4
        // Or: buy@2 sell@3=4. Same.
        // But buy@0 sell@1, cooldown@2, can't buy@3 profitably.
        assert_eq!(MySolution::max_profit(vec![1, 5, 1, 5]), 4);
        assert_eq!(AiSolution::max_profit(vec![1, 5, 1, 5]), 4);
    }

    #[test]
    fn test_longer_zigzag() {
        // [1, 5, 1, 5, 1, 5]: buy@0 sell@1(+4), cooldown@2, buy@3? sell@3(0),
        // buy@2(1) sell@3(5)=+4, cooldown@4, buy@?
        // Optimal: buy@0 sell@1=+4, cooldown@2, buy@4(1) sell@5(5)=+4 => 8
        assert_eq!(MySolution::max_profit(vec![1, 5, 1, 5, 1, 5]), 8);
        assert_eq!(AiSolution::max_profit(vec![1, 5, 1, 5, 1, 5]), 8);
    }

    #[test]
    fn test_single_price() {
        assert_eq!(MySolution::max_profit(vec![100]), 0);
        assert_eq!(AiSolution::max_profit(vec![100]), 0);
    }
}

fn main() {
    println!("LeetCode 309: Best Time to Buy and Sell Stock with Cooldown");

    let prices = vec![1, 2, 3, 0, 2];
    println!(
        "Input: {:?}, My: {}, AI: {}",
        prices,
        my_solution::Solution::max_profit(prices.clone()),
        ai_solution::Solution::max_profit(prices.clone())
    );

    // Benchmark
    let iterations = 100_000;

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::Solution::max_profit(std::hint::black_box(
            prices.clone(),
        )));
    }
    let my_time = start.elapsed();

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::Solution::max_profit(std::hint::black_box(
            prices.clone(),
        )));
    }
    let ai_time = start.elapsed();

    let my_avg = my_time.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_time.as_nanos() as f64 / iterations as f64 / 1000.0;

    println!("\nBenchmark ({iterations} iterations):");
    println!("  My solution:  {my_avg:.3}us avg");
    println!("  AI solution:  {ai_avg:.3}us avg");
    if my_avg < ai_avg {
        println!("  Winner: My solution ({:.2}x faster)", ai_avg / my_avg);
    } else {
        println!("  Winner: AI solution ({:.2}x faster)", my_avg / ai_avg);
    }
}
