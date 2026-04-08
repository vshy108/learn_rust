// LeetCode 486: Predict the Winner [Medium]
// https://leetcode.com/problems/predict-the-winner/
//
// Two players take turns picking from either end of an integer array.
// Player 1 goes first. The player with higher total score wins.
// Return true if Player 1 can win (or tie).
//
// Examples:
//   [1,5,2]   -> false (Player 1 picks 1 or 2, Player 2 always gets 5)
//   [1,5,233,7] -> true
//
// Constraints:
//   1 <= nums.len() <= 20
//   0 <= nums[i] <= 10^7
//
// Run:  cargo run --example lc_0486_predict_the_winner
// Test: cargo test --example lc_0486_predict_the_winner
// Bench: cargo test --example lc_0486_predict_the_winner bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 504ns (2D DP)
//   AI solution: avg 108ns (1D optimized DP)
//   Ratio: 4.67x (1D optimization much faster)

/// My solution module — interval DP / minimax.
///
/// Time:  O(n^2)
/// Space: O(n^2)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn predict_the_winner(nums: Vec<i32>) -> bool {
            // 2D interval DP: dp[i][j] = max advantage on nums[i..=j]
            let n = nums.len();
            let mut dp = vec![vec![0i32; n]; n];
            for i in 0..n {
                dp[i][i] = nums[i];
            }
            for gap in 1..n {
                for i in 0..n - gap {
                    let j = i + gap;
                    dp[i][j] = (nums[i] - dp[i + 1][j]).max(nums[j] - dp[i][j - 1]);
                }
            }
            dp[0][n - 1] >= 0
        }
    }
}

/// AI solution module — interval DP.
///
/// dp[i][j] = maximum score advantage the current player can achieve on
/// subarray nums[i..=j]. A player picks from either end and the opponent
/// then plays optimally on the remaining subarray.
///
/// dp[i][j] = max(nums[i] - dp[i+1][j], nums[j] - dp[i][j-1])
///
/// Player 1 wins if dp[0][n-1] >= 0.
///
/// Time:  O(n^2)
/// Space: O(n) — optimized to 1D
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn predict_the_winner(nums: Vec<i32>) -> bool {
            let n = nums.len();
            // dp[j] = advantage of current player on subarray ending at j
            let mut dp = nums.clone();
            // Build bottom-up: gap from 1 to n-1
            for gap in 1..n {
                for i in 0..n - gap {
                    let j = i + gap;
                    dp[j] = (nums[i] - dp[j]).max(nums[j] - dp[j - 1]);
                }
            }
            dp[n - 1] >= 0
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_lose() {
            assert!(!Solution::predict_the_winner(vec![1, 5, 2]));
        }
        #[test]
        fn test_win() {
            assert!(Solution::predict_the_winner(vec![1, 5, 233, 7]));
        }
        #[test]
        fn test_single() {
            assert!(Solution::predict_the_winner(vec![5]));
        }
        #[test]
        fn test_tie() {
            assert!(Solution::predict_the_winner(vec![1, 1]));
        }
    };
}

#[cfg(test)]
mod tests_my {
    solution_tests!(my_solution);
}
#[cfg(test)]
mod tests_ai {
    solution_tests!(ai_solution);
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(Vec<i32>) -> bool,
{
    let iterations = 500_000;
    let input = vec![1, 5, 233, 7, 10, 15, 20];
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(input.clone())));
    }
    let elapsed = start.elapsed();
    println!(
        "{}: {} iterations in {:?} (avg {:?})",
        name,
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::predict_the_winner);
    bench_solution("AI solution", ai_solution::Solution::predict_the_winner);
}

fn main() {
    println!(
        "{}",
        ai_solution::Solution::predict_the_winner(vec![1, 5, 233, 7])
    );
}
