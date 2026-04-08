// @lc app=leetcode id=375 lang=rust
// #[Medium] - Guess Number Higher or Lower II
// Iterations: 100000 | Date: 2025-07-10
// my_solution  | avg: 57.597us
// ai_solution  | avg: 94.636us
// Performance: My 1.64x faster

/// # 375. Guess Number Higher or Lower II
///
/// Given n, return the minimum money needed to guarantee a win in the guessing game
/// where wrong guesses cost you the guessed number.
///
/// Constraints:
/// - 1 <= n <= 200

/// ## My Solution — Bottom-up DP O(n^3)
///
/// dp[i][j] = min cost to guarantee finding the number in [i..j].
/// For each possible guess k in [i..j]:
///   dp[i][j] = min over k of (k + max(dp[i][k-1], dp[k+1][j]))
///
/// - Time: O(n^3)
/// - Space: O(n^2)
mod my_solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0i32; n + 2]; n + 2];

        // Fill diagonals: intervals of increasing length
        for len in 2..=n {
            for i in 1..=n - len + 1 {
                let j = i + len - 1;
                dp[i][j] = i32::MAX;
                for k in i..=j {
                    let cost = k as i32 + dp[i][k.wrapping_sub(1)].max(dp[k + 1][j]);
                    dp[i][j] = dp[i][j].min(cost);
                }
            }
        }
        dp[1][n]
    }
}

/// ## AI Solution — Top-down memoized recursion
///
/// Same recurrence but with memoization instead of bottom-up table.
///
/// - Time: O(n^3)
/// - Space: O(n^2)
mod ai_solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        let mut memo = vec![vec![-1i32; n + 1]; n + 1];
        solve(1, n, &mut memo)
    }

    fn solve(lo: usize, hi: usize, memo: &mut [Vec<i32>]) -> i32 {
        if lo >= hi {
            return 0;
        }
        if memo[lo][hi] >= 0 {
            return memo[lo][hi];
        }
        let mut best = i32::MAX;
        for k in lo..=hi {
            let cost = k as i32 + solve(lo, k.wrapping_sub(1), memo).max(solve(k + 1, hi, memo));
            best = best.min(cost);
        }
        memo[lo][hi] = best;
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(i32) -> i32) {
        assert_eq!(f(10), 16);
        assert_eq!(f(1), 0);
        assert_eq!(f(2), 1);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::get_money_amount);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::get_money_amount);
    }
}

fn main() {
    let n = 50;
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::get_money_amount(std::hint::black_box(n)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::get_money_amount(std::hint::black_box(n)));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::get_money_amount(std::hint::black_box(n)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::get_money_amount(std::hint::black_box(n)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 375 - Guess Number Higher or Lower II");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
