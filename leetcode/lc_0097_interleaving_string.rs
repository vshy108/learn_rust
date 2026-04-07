// LeetCode 97: Interleaving String [Medium]
// https://leetcode.com/problems/interleaving-string/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 1.670us
//   AI solution avg: 0.769us
//   Ratio: AI 2.17x faster
//
// Given s1, s2, and s3, determine if s3 is formed by interleaving s1 and s2.
// s3 is an interleaving if it can be split into subsequences that alternate
// between s1 and s2 while preserving order within each.
//
// Constraints: 0 <= s1.length, s2.length <= 100, 0 <= s3.length <= 200
//
// Run:  cargo run --example lc_0097_interleaving_string
// Test: cargo test --example lc_0097_interleaving_string
// Bench: cargo test --example lc_0097_interleaving_string bench -- --nocapture

/// My solution — 2D DP.
///
/// dp[i][j] = true if s3[..i+j] can be formed by interleaving s1[..i] and s2[..j].
///
/// Time: O(m * n).
/// Space: O(m * n).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Check if s3 is an interleaving of s1 and s2.
        ///
        /// # Arguments
        /// * `s1` - first string
        /// * `s2` - second string
        /// * `s3` - target interleaved string
        ///
        /// # Returns
        /// true if s3 is a valid interleaving
        pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
            let b1 = s1.as_bytes();
            let b2 = s2.as_bytes();
            let b3 = s3.as_bytes();
            let m = b1.len();
            let n = b2.len();

            if m + n != b3.len() {
                return false;
            }

            let mut dp = vec![vec![false; n + 1]; m + 1];
            dp[0][0] = true;

            // First column: s1 prefix matches s3 prefix
            for i in 1..=m {
                dp[i][0] = dp[i - 1][0] && b1[i - 1] == b3[i - 1];
            }
            // First row: s2 prefix matches s3 prefix
            for j in 1..=n {
                dp[0][j] = dp[0][j - 1] && b2[j - 1] == b3[j - 1];
            }

            for i in 1..=m {
                for j in 1..=n {
                    dp[i][j] = (dp[i - 1][j] && b1[i - 1] == b3[i + j - 1])
                        || (dp[i][j - 1] && b2[j - 1] == b3[i + j - 1]);
                }
            }
            dp[m][n]
        }
    }
}

/// AI solution — space-optimized 1D DP.
///
/// Time: O(m * n).
/// Space: O(n).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Check if s3 is an interleaving of s1 and s2.
        ///
        /// # Arguments
        /// * `s1` - first string
        /// * `s2` - second string
        /// * `s3` - target interleaved string
        ///
        /// # Returns
        /// true if s3 is a valid interleaving
        pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
            let b1 = s1.as_bytes();
            let b2 = s2.as_bytes();
            let b3 = s3.as_bytes();
            let m = b1.len();
            let n = b2.len();

            if m + n != b3.len() {
                return false;
            }

            let mut dp = vec![false; n + 1];
            dp[0] = true;

            for j in 1..=n {
                dp[j] = dp[j - 1] && b2[j - 1] == b3[j - 1];
            }

            for i in 1..=m {
                dp[0] = dp[0] && b1[i - 1] == b3[i - 1];
                for j in 1..=n {
                    dp[j] = (dp[j] && b1[i - 1] == b3[i + j - 1])
                        || (dp[j - 1] && b2[j - 1] == b3[i + j - 1]);
                }
            }
            dp[n]
        }
    }
}

fn main() {
    let cases = [
        ("aabcc", "dbbca", "aadbbcbcac", true),
        ("aabcc", "dbbca", "aadbbbaccc", false),
    ];
    for (s1, s2, s3, expected) in cases {
        let my = my_solution::Solution::is_interleave(
            s1.to_string(), s2.to_string(), s3.to_string(),
        );
        let ai = ai_solution::Solution::is_interleave(
            s1.to_string(), s2.to_string(), s3.to_string(),
        );
        println!("{s1}+{s2}={s3}? my={my}, ai={ai} (expected={expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_interleave {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_true() {
                    assert!(<$solution>::is_interleave(
                        "aabcc".to_string(),
                        "dbbca".to_string(),
                        "aadbbcbcac".to_string(),
                    ));
                }

                #[test]
                fn test_false() {
                    assert!(!<$solution>::is_interleave(
                        "aabcc".to_string(),
                        "dbbca".to_string(),
                        "aadbbbaccc".to_string(),
                    ));
                }

                #[test]
                fn test_empty_all() {
                    assert!(<$solution>::is_interleave(
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                    ));
                }

                #[test]
                fn test_empty_s1() {
                    assert!(<$solution>::is_interleave(
                        "".to_string(),
                        "abc".to_string(),
                        "abc".to_string(),
                    ));
                }

                #[test]
                fn test_length_mismatch() {
                    assert!(!<$solution>::is_interleave(
                        "a".to_string(),
                        "b".to_string(),
                        "abc".to_string(),
                    ));
                }
            }
        };
    }

    test_interleave!(my, crate::my_solution::Solution);
    test_interleave!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbcbcac".to_string();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::is_interleave(
                std::hint::black_box(s1.clone()),
                std::hint::black_box(s2.clone()),
                std::hint::black_box(s3.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::is_interleave(
                std::hint::black_box(s1.clone()),
                std::hint::black_box(s2.clone()),
                std::hint::black_box(s3.clone()),
            ));
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
