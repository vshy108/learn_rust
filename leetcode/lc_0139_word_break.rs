// LeetCode 139: Word Break [Medium]
// https://leetcode.com/problems/word-break/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 2.325us
//   AI solution avg: 0.821us
//   Ratio: AI 2.83x faster
//
// Given a string s and a dictionary of strings wordDict, return true if s can
// be segmented into a space-separated sequence of one or more dictionary words.
// The same dictionary word may be reused multiple times.
//
// Constraints: 1 <= s.length <= 300, 1 <= wordDict.length <= 1000,
//   1 <= wordDict[i].length <= 20
//
// Run:  cargo run --example lc_0139_word_break
// Test: cargo test --example lc_0139_word_break
// Bench: cargo test --example lc_0139_word_break bench -- --nocapture

use std::collections::HashSet;

/// My solution — bottom-up DP.
///
/// Time: O(n^2 * m) where n = s.len(), m = max word length (substring check).
/// Space: O(n + d) where d = total dictionary size.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return true if s can be segmented into dictionary words.
        pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
            let dict: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
            let n = s.len();
            // dp[i] = true if s[0..i] can be segmented
            let mut dp = vec![false; n + 1];
            dp[0] = true; // empty prefix is valid

            for i in 1..=n {
                for j in 0..i {
                    if dp[j] && dict.contains(&s[j..i]) {
                        dp[i] = true;
                        break;
                    }
                }
            }
            dp[n]
        }
    }
}

/// AI solution — DP scanning only up to max word length (pruned).
///
/// Time: O(n * L) where L = max word length.
/// Space: O(n + d).
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return true if s can be segmented (pruned by max word length).
        pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
            let dict: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
            let max_len = dict.iter().map(|w| w.len()).max().unwrap_or(0);
            let n = s.len();
            let mut dp = vec![false; n + 1];
            dp[0] = true;

            for i in 1..=n {
                // Only look back up to max_len characters — avoids O(n^2)
                let start = i.saturating_sub(max_len);
                for j in start..i {
                    if dp[j] && dict.contains(&s[j..i]) {
                        dp[i] = true;
                        break;
                    }
                }
            }
            dp[n]
        }
    }
}

fn main() {
    let my =
        my_solution::Solution::word_break("leetcode".into(), vec!["leet".into(), "code".into()]);
    let ai =
        ai_solution::Solution::word_break("leetcode".into(), vec!["leet".into(), "code".into()]);
    println!("word break: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_word_break {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert!(<$solution>::word_break(
                        "leetcode".into(),
                        vec!["leet".into(), "code".into()],
                    ));
                }

                #[test]
                fn test_example2() {
                    assert!(<$solution>::word_break(
                        "applepenapple".into(),
                        vec!["apple".into(), "pen".into()],
                    ));
                }

                #[test]
                fn test_example3() {
                    assert!(!<$solution>::word_break(
                        "catsandog".into(),
                        vec![
                            "cats".into(),
                            "dog".into(),
                            "sand".into(),
                            "and".into(),
                            "cat".into(),
                        ],
                    ));
                }

                #[test]
                fn test_single_char() {
                    assert!(<$solution>::word_break("a".into(), vec!["a".into()],));
                }

                #[test]
                fn test_reuse() {
                    assert!(<$solution>::word_break(
                        "aaaa".into(),
                        vec!["a".into(), "aa".into()],
                    ));
                }
            }
        };
    }

    test_word_break!(my, crate::my_solution::Solution);
    test_word_break!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let s = "aaaaaaaaaaaaaaaaaab".to_string();
        let dict: Vec<String> = vec![
            "a".into(),
            "aa".into(),
            "aaa".into(),
            "aaaa".into(),
            "aaaaa".into(),
        ];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::word_break(
                std::hint::black_box(s.clone()),
                std::hint::black_box(dict.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::word_break(
                std::hint::black_box(s.clone()),
                std::hint::black_box(dict.clone()),
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
            println!("Both solutions are roughly equal");
        }
    }
}
