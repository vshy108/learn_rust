// LeetCode 70: Climbing Stairs [Easy]
// https://leetcode.com/problems/climbing-stairs/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 0.624us
//   AI solution avg: 0.242us
//   Ratio: AI 2.58x faster
//
// You are climbing a staircase with n steps. Each time you can climb
// 1 or 2 steps. How many distinct ways can you climb to the top?
//
// Example 1: n=2 -> 2 (1+1, 2)
// Example 2: n=3 -> 3 (1+1+1, 1+2, 2+1)
//
// Constraints: 1 <= n <= 45
//
// Run:  cargo run --example lc_0070_climbing_stairs
// Test: cargo test --example lc_0070_climbing_stairs
// Bench: cargo test --example lc_0070_climbing_stairs bench -- --nocapture

/// My solution — bottom-up DP with array.
///
/// dp[i] = dp[i-1] + dp[i-2] (Fibonacci-like).
///
/// Time: O(n).
/// Space: O(n).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Count distinct ways to climb n stairs.
        ///
        /// # Arguments
        /// * `n` - number of stairs
        ///
        /// # Returns
        /// Number of distinct ways
        pub fn climb_stairs(n: i32) -> i32 {
            if n <= 2 {
                return n;
            }
            let n = n as usize;
            let mut dp = vec![0; n + 1];
            dp[1] = 1;
            dp[2] = 2;
            for i in 3..=n {
                dp[i] = dp[i - 1] + dp[i - 2];
            }
            dp[n]
        }
    }
}

/// AI solution — O(1) space with two variables.
///
/// Only need the last two values (Fibonacci sliding window).
///
/// Time: O(n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Count distinct ways to climb n stairs.
        ///
        /// # Arguments
        /// * `n` - number of stairs
        ///
        /// # Returns
        /// Number of distinct ways
        pub fn climb_stairs(n: i32) -> i32 {
            if n <= 2 {
                return n;
            }
            let (mut a, mut b) = (1, 2);
            for _ in 3..=n {
                let next = a + b;
                a = b;
                b = next;
            }
            b
        }
    }
}

fn main() {
    for n in [1, 2, 3, 5, 10, 45] {
        println!(
            "n={n} -> my:{}, ai:{}",
            my_solution::Solution::climb_stairs(n),
            ai_solution::Solution::climb_stairs(n),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_climb_stairs {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_n2() {
                    assert_eq!(<$solution>::climb_stairs(2), 2);
                }

                #[test]
                fn test_n3() {
                    assert_eq!(<$solution>::climb_stairs(3), 3);
                }

                #[test]
                fn test_n1() {
                    assert_eq!(<$solution>::climb_stairs(1), 1);
                }

                #[test]
                fn test_n5() {
                    assert_eq!(<$solution>::climb_stairs(5), 8);
                }

                #[test]
                fn test_n10() {
                    assert_eq!(<$solution>::climb_stairs(10), 89);
                }

                #[test]
                fn test_n45() {
                    assert_eq!(<$solution>::climb_stairs(45), 1836311903);
                }
            }
        };
    }

    test_climb_stairs!(my, crate::my_solution::Solution);
    test_climb_stairs!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::climb_stairs(std::hint::black_box(45)),
            );
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::climb_stairs(std::hint::black_box(45)),
            );
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg ({:.0}ms total)",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            my_duration.as_millis(),
        );
        println!(
            "AI solution: {:.3}us avg ({:.0}ms total)",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            ai_duration.as_millis(),
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
