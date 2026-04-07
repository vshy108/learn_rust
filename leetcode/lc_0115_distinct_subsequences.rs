// LeetCode 115: Distinct Subsequences [Hard]
// https://leetcode.com/problems/distinct-subsequences/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 2.525us
//   AI solution avg: 0.266us
//   Ratio: AI 9.50x faster
//
// Given two strings s and t, return the number of distinct subsequences of s
// which equals t.
//
// Constraints: 1 <= s.length, t.length <= 1000, s and t consist of English letters
//
// Run:  cargo run --example lc_0115_distinct_subsequences
// Test: cargo test --example lc_0115_distinct_subsequences
// Bench: cargo test --example lc_0115_distinct_subsequences bench -- --nocapture

/// My solution — 2D DP: dp[i][j] = # of ways to form t[0..j] from s[0..i].
///
/// Time: O(m * n) where m = s.len(), n = t.len().
/// Space: O(m * n).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Count distinct subsequences of s that equal t.
        pub fn num_distinct(s: String, t: String) -> i32 {
            let s = s.as_bytes();
            let t = t.as_bytes();
            let m = s.len();
            let n = t.len();
            if n > m { return 0; }

            // dp[i][j] = number of ways to match t[0..j] using s[0..i]
            let mut dp = vec![vec![0u64; n + 1]; m + 1];
            // Empty t can be matched by any prefix of s (one way: take nothing)
            for row in &mut dp {
                row[0] = 1;
            }

            for i in 1..=m {
                for j in 1..=n {
                    dp[i][j] = dp[i - 1][j]; // Skip s[i-1]
                    if s[i - 1] == t[j - 1] {
                        dp[i][j] += dp[i - 1][j - 1]; // Use s[i-1] to match t[j-1]
                    }
                }
            }
            dp[m][n] as i32
        }
    }
}

/// AI solution — 1D DP (space-optimized), iterating j in reverse.
///
/// Time: O(m * n).
/// Space: O(n).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Count distinct subsequences using 1D DP.
        pub fn num_distinct(s: String, t: String) -> i32 {
            let s = s.as_bytes();
            let t = t.as_bytes();
            let n = t.len();
            if n > s.len() { return 0; }

            let mut dp = vec![0u64; n + 1];
            dp[0] = 1; // Empty t always matches

            for &sc in s {
                // Iterate backwards to avoid using updated values
                for j in (1..=n).rev() {
                    if sc == t[j - 1] {
                        dp[j] += dp[j - 1];
                    }
                }
            }
            dp[n] as i32
        }
    }
}

fn main() {
    let my = my_solution::Solution::num_distinct("rabbbit".into(), "rabbit".into());
    let ai = ai_solution::Solution::num_distinct("rabbbit".into(), "rabbit".into());
    println!("distinct subsequences: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_distinct {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_rabbbit() {
                    assert_eq!(
                        <$solution>::num_distinct("rabbbit".into(), "rabbit".into()),
                        3,
                    );
                }

                #[test]
                fn test_babgbag() {
                    assert_eq!(
                        <$solution>::num_distinct("babgbag".into(), "bag".into()),
                        5,
                    );
                }

                #[test]
                fn test_no_match() {
                    assert_eq!(
                        <$solution>::num_distinct("abc".into(), "def".into()),
                        0,
                    );
                }

                #[test]
                fn test_t_longer() {
                    assert_eq!(
                        <$solution>::num_distinct("a".into(), "ab".into()),
                        0,
                    );
                }

                #[test]
                fn test_single_char() {
                    assert_eq!(
                        <$solution>::num_distinct("aaa".into(), "a".into()),
                        3,
                    );
                }

                #[test]
                fn test_equal() {
                    assert_eq!(
                        <$solution>::num_distinct("abc".into(), "abc".into()),
                        1,
                    );
                }
            }
        };
    }

    test_distinct!(my, crate::my_solution::Solution);
    test_distinct!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let s: String = "aabbaabbccdd".repeat(3);
        let t: String = "abcd".to_string();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::num_distinct(
                std::hint::black_box(s.clone()),
                std::hint::black_box(t.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::num_distinct(
                std::hint::black_box(s.clone()),
                std::hint::black_box(t.clone()),
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
