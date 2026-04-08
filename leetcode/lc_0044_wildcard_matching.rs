// LeetCode 44: Wildcard Matching [Hard]
// https://leetcode.com/problems/wildcard-matching/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 52.997µs
//   AI solution avg: 35.227µs
//   Ratio: AI solution is 1.50x faster
//
// Given an input string s and a pattern p, implement wildcard matching:
//   '?' matches any single character
//   '*' matches any sequence of characters (including empty)
//
// The matching should cover the entire input string.
//
// Example 1: s = "aa", p = "a"    -> false
// Example 2: s = "aa", p = "*"    -> true
// Example 3: s = "cb", p = "?a"   -> false
//
// Constraints:
//   0 <= s.length, p.length <= 2000
//   s contains only lowercase English letters
//   p contains only lowercase English letters, '?' and '*'
//
// Run:  cargo run --example lc_0044_wildcard_matching
// Test: cargo test --example lc_0044_wildcard_matching
// Bench: cargo test --example lc_0044_wildcard_matching bench -- --nocapture

/// My solution — dynamic programming.
///
/// dp[i][j] = true if s[0..i] matches p[0..j].
/// Transition:
///   - p[j-1] == s[i-1] or '?' => dp[i][j] = dp[i-1][j-1]
///   - p[j-1] == '*' => dp[i][j] = dp[i][j-1] (star matches empty)
///     || dp[i-1][j] (star matches one more char)
///
/// Time: O(m*n) where m = s.len(), n = p.len().
/// Space: O(m*n) for the DP table.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Check if string s matches wildcard pattern p.
        ///
        /// # Arguments
        /// * `s` - input string (lowercase letters)
        /// * `p` - pattern with '?' and '*'
        ///
        /// # Returns
        /// `true` if s fully matches p.
        pub fn is_match(s: String, p: String) -> bool {
            let s = s.as_bytes();
            let p = p.as_bytes();
            let m = s.len();
            let n = p.len();

            // dp[i][j]: does s[0..i] match p[0..j]?
            let mut dp = vec![vec![false; n + 1]; m + 1];
            dp[0][0] = true;

            // Base: s is empty, p must be all '*' to match.
            for j in 1..=n {
                if p[j - 1] == b'*' {
                    dp[0][j] = dp[0][j - 1];
                }
            }

            for i in 1..=m {
                for j in 1..=n {
                    if p[j - 1] == b'*' {
                        // '*' matches empty (dp[i][j-1]) or one more char (dp[i-1][j]).
                        dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
                    } else if p[j - 1] == b'?' || p[j - 1] == s[i - 1] {
                        dp[i][j] = dp[i - 1][j - 1];
                    }
                }
            }

            dp[m][n]
        }
    }
}

/// AI-optimized solution — 1D DP (space-optimized).
///
/// Since dp[i][j] only depends on dp[i-1][j], dp[i][j-1], and dp[i-1][j-1],
/// we can use a single row plus a saved diagonal value.
///
/// Time: O(m*n).
/// Space: O(n) — single row.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Check if string s matches wildcard pattern p.
        pub fn is_match(s: String, p: String) -> bool {
            let s = s.as_bytes();
            let p = p.as_bytes();
            let n = p.len();

            // dp[j]: does s[0..current_i] match p[0..j]?
            let mut dp = vec![false; n + 1];
            dp[0] = true;

            // Initialize for empty s: leading '*'s match empty.
            for j in 1..=n {
                if p[j - 1] == b'*' {
                    dp[j] = dp[j - 1];
                }
            }

            for &sc in s {
                let mut prev = dp[0]; // dp[i-1][j-1] for j=1 (i.e., dp[i-1][0])
                dp[0] = false; // empty pattern can't match non-empty s

                for j in 1..=n {
                    let temp = dp[j]; // save dp[i-1][j] before overwriting

                    if p[j - 1] == b'*' {
                        // dp[i][j-1] || dp[i-1][j]
                        dp[j] = dp[j - 1] || dp[j];
                    } else if p[j - 1] == b'?' || p[j - 1] == sc {
                        dp[j] = prev; // dp[i-1][j-1]
                    } else {
                        dp[j] = false;
                    }

                    prev = temp; // dp[i-1][j] becomes dp[i-1][j-1] for next j
                }
            }

            dp[n]
        }
    }
}

fn main() {
    let cases = [("aa", "a"), ("aa", "*"), ("cb", "?a"), ("adceb", "*a*b")];
    for (s, p) in &cases {
        println!(
            "s=\"{s}\", p=\"{p}\" -> my:{}, ai:{}",
            my_solution::Solution::is_match(s.to_string(), p.to_string()),
            ai_solution::Solution::is_match(s.to_string(), p.to_string()),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_is_match {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_no_wildcard_fail() {
                    assert!(!Solution::is_match("aa".into(), "a".into()));
                }

                #[test]
                fn test_star_matches_all() {
                    assert!(Solution::is_match("aa".into(), "*".into()));
                }

                #[test]
                fn test_question_mismatch() {
                    assert!(!Solution::is_match("cb".into(), "?a".into()));
                }

                #[test]
                fn test_star_in_middle() {
                    assert!(Solution::is_match("adceb".into(), "*a*b".into()));
                }

                #[test]
                fn test_trailing_star() {
                    assert!(!Solution::is_match("acdcb".into(), "a*c?b".into()));
                }

                #[test]
                fn test_empty_both() {
                    assert!(Solution::is_match("".into(), "".into()));
                }

                #[test]
                fn test_empty_s_star_p() {
                    assert!(Solution::is_match("".into(), "*".into()));
                }

                #[test]
                fn test_empty_s_nonempty_p() {
                    assert!(!Solution::is_match("".into(), "abc".into()));
                }

                #[test]
                fn test_exact_match() {
                    assert!(Solution::is_match("hello".into(), "hello".into()));
                }

                #[test]
                fn test_all_questions() {
                    assert!(Solution::is_match("abc".into(), "???".into()));
                }
            }
        };
    }

    test_is_match!(my_tests, crate::my_solution::Solution);
    test_is_match!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        let s = "a".repeat(50);
        let p = format!("*{}*", "?".repeat(50));

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::is_match(
                std::hint::black_box(s.clone()),
                std::hint::black_box(p.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::is_match(
                std::hint::black_box(s.clone()),
                std::hint::black_box(p.clone()),
            ));
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
