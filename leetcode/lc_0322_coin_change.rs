// LeetCode 322. Coin Change
// https://leetcode.com/problems/coin-change/
//
// Benchmark results (100_000 iterations, release mode, 2025-01-XX):
//   my_solution  avg: 0.142us
//   ai_solution  avg: 0.232us
//   Ratio: my_solution 1.63x faster

/// Given coins of different denominations and a total amount, return fewest
/// coins needed to make that amount. Return -1 if impossible.
///
/// Example 1: coins=[1,2,5], amount=11 -> 3 (5+5+1)
/// Example 2: coins=[2], amount=3 -> -1
/// Example 3: coins=[1], amount=0 -> 0
///
/// Constraints: 1<=coins.length<=12, 1<=coins[i]<=2^31-1, 0<=amount<=10^4

/// # my_solution: 1D DP (space-optimized complete knapsack)
/// Time: O(m * n) where m = coins count, n = amount
/// Space: O(n)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find minimum coins using 1D DP (from doocs Rust solution).
        ///
        /// f[j] = min coins to make amount j. For each coin x, update
        /// f[j] = min(f[j], f[j-x] + 1). Complete knapsack: inner loop
        /// goes forward since we can reuse coins.
        ///
        /// # Arguments
        /// * `coins` - Available coin denominations
        /// * `amount` - Target amount
        ///
        /// # Returns
        /// Minimum number of coins, or -1 if impossible
        pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
            let n = amount as usize;
            let mut f = vec![n + 1; n + 1];
            f[0] = 0;

            for &x in &coins {
                let x = x as usize;
                for j in x..=n {
                    f[j] = f[j].min(f[j - x] + 1);
                }
            }

            if f[n] > n { -1 } else { f[n] as i32 }
        }
    }
}

/// # ai_solution: BFS level-by-level
/// Time: O(amount * m) worst case
/// Space: O(amount) for visited set
mod ai_solution {
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// Find minimum coins using BFS.
        ///
        /// BFS from amount down to 0. Each level = one more coin used.
        /// The first time we reach 0 is the minimum number of coins.
        ///
        /// # Arguments
        /// * `coins` - Available coin denominations
        /// * `amount` - Target amount
        ///
        /// # Returns
        /// Minimum number of coins, or -1 if impossible
        pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
            if amount == 0 {
                return 0;
            }
            let n = amount as usize;
            let mut visited = vec![false; n + 1];
            visited[n] = true;
            let mut queue = VecDeque::new();
            queue.push_back(n);
            let mut depth = 0;

            while !queue.is_empty() {
                depth += 1;
                for _ in 0..queue.len() {
                    let curr = queue.pop_front().unwrap();
                    for &c in &coins {
                        let c = c as usize;
                        if curr >= c {
                            let next = curr - c;
                            if next == 0 {
                                return depth;
                            }
                            if !visited[next] {
                                visited[next] = true;
                                queue.push_back(next);
                            }
                        }
                    }
                }
            }

            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::Solution as AiSolution;
    use super::my_solution::Solution as MySolution;

    fn run_tests(solve: fn(Vec<i32>, i32) -> i32) {
        assert_eq!(solve(vec![1, 2, 5], 11), 3);
        assert_eq!(solve(vec![2], 3), -1);
        assert_eq!(solve(vec![1], 0), 0);
        assert_eq!(solve(vec![1], 1), 1);
        assert_eq!(solve(vec![1, 2, 5], 100), 20); // 20 x 5
        assert_eq!(solve(vec![2, 5, 10, 1], 27), 4); // 10+10+5+2
        assert_eq!(solve(vec![186, 419, 83, 408], 6249), 20);
    }

    #[test]
    fn test_my_solution() {
        run_tests(MySolution::coin_change);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(AiSolution::coin_change);
    }
}

fn main() {
    use std::hint::black_box;
    use std::time::Instant;

    let coins = vec![1, 2, 5];
    let amount = 11;
    println!("Input: coins={coins:?}, amount={amount}");
    println!(
        "Output (my): {}",
        my_solution::Solution::coin_change(coins.clone(), amount)
    );
    println!(
        "Output (ai): {}",
        ai_solution::Solution::coin_change(coins.clone(), amount)
    );

    let iterations = 100_000;
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(my_solution::Solution::coin_change(
            black_box(coins.clone()),
            black_box(amount),
        ));
    }
    let my_dur = start.elapsed();
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(ai_solution::Solution::coin_change(
            black_box(coins.clone()),
            black_box(amount),
        ));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    println!("\nBenchmark ({iterations} iterations, release mode):");
    println!("  my_solution  avg: {my_avg:.3}us");
    println!("  ai_solution  avg: {ai_avg:.3}us");
    if my_avg < ai_avg {
        println!("  my_solution is {:.2}x faster", ai_avg / my_avg);
    } else {
        println!("  ai_solution is {:.2}x faster", my_avg / ai_avg);
    }
}
