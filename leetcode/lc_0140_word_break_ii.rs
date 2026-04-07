// LeetCode 140: Word Break II [Hard]
// https://leetcode.com/problems/word-break-ii/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 1.640us
//   AI solution avg: 2.009us
//   Ratio: My 1.22x faster
//
// Given a string s and a dictionary of strings wordDict, add spaces in s to
// construct a sentence where each word is a valid dictionary word. Return all
// such possible sentences in any order.
//
// Constraints: 1 <= s.length <= 20, 1 <= wordDict.length <= 1000,
//   1 <= wordDict[i].length <= 10, all strings are lowercase English
//
// Run:  cargo run --example lc_0140_word_break_ii
// Test: cargo test --example lc_0140_word_break_ii
// Bench: cargo test --example lc_0140_word_break_ii bench -- --nocapture

use std::collections::HashSet;

/// My solution — backtracking with early termination via DP feasibility check.
///
/// Time: O(2^n * n) worst case.
/// Space: O(2^n * n) for storing results.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return all valid sentence segmentations.
        pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
            let dict: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
            let mut result = Vec::new();
            let mut path = Vec::new();
            Self::backtrack(&s, 0, &dict, &mut path, &mut result);
            result
        }

        fn backtrack(
            s: &str,
            start: usize,
            dict: &HashSet<&str>,
            path: &mut Vec<String>,
            result: &mut Vec<String>,
        ) {
            if start == s.len() {
                result.push(path.join(" "));
                return;
            }
            for end in start + 1..=s.len() {
                let word = &s[start..end];
                if dict.contains(word) {
                    path.push(word.to_owned());
                    Self::backtrack(s, end, dict, path, result);
                    path.pop();
                }
            }
        }
    }
}

/// AI solution — memoized DFS caching sentences from each index.
///
/// Time: O(2^n * n) worst case, but memoization avoids recomputation.
/// Space: O(2^n * n) for cached results.
mod ai_solution {
    use super::*;
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Return all valid sentences using memoized DFS.
        pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
            let dict: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
            let max_len = dict.iter().map(|w| w.len()).max().unwrap_or(0);
            let mut memo: HashMap<usize, Vec<String>> = HashMap::new();
            Self::dfs(&s, 0, &dict, max_len, &mut memo)
        }

        fn dfs(
            s: &str,
            start: usize,
            dict: &HashSet<&str>,
            max_len: usize,
            memo: &mut HashMap<usize, Vec<String>>,
        ) -> Vec<String> {
            if let Some(cached) = memo.get(&start) {
                return cached.clone();
            }
            if start == s.len() {
                return vec![String::new()];
            }

            let mut sentences = Vec::new();
            let end_limit = (start + max_len).min(s.len());
            for end in start + 1..=end_limit {
                let word = &s[start..end];
                if dict.contains(word) {
                    let rest = Self::dfs(s, end, dict, max_len, memo);
                    for sentence in &rest {
                        if sentence.is_empty() {
                            sentences.push(word.to_owned());
                        } else {
                            sentences.push(format!("{word} {sentence}"));
                        }
                    }
                }
            }
            memo.insert(start, sentences.clone());
            sentences
        }
    }
}

fn main() {
    let my = my_solution::Solution::word_break(
        "catsanddog".into(),
        vec!["cat".into(), "cats".into(), "and".into(), "sand".into(), "dog".into()],
    );
    let ai = ai_solution::Solution::word_break(
        "catsanddog".into(),
        vec!["cat".into(), "cats".into(), "and".into(), "sand".into(), "dog".into()],
    );
    println!("my: {my:?}");
    println!("ai: {ai:?}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_word_break {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                fn sorted(mut v: Vec<String>) -> Vec<String> {
                    v.sort();
                    v
                }

                #[test]
                fn test_example1() {
                    let result = sorted(<$solution>::word_break(
                        "catsanddog".into(),
                        vec![
                            "cat".into(), "cats".into(), "and".into(),
                            "sand".into(), "dog".into(),
                        ],
                    ));
                    let expected = sorted(vec![
                        "cats and dog".into(),
                        "cat sand dog".into(),
                    ]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_example2() {
                    let result = sorted(<$solution>::word_break(
                        "pineapplepenapple".into(),
                        vec![
                            "apple".into(), "pen".into(), "applepen".into(),
                            "pine".into(), "pineapple".into(),
                        ],
                    ));
                    let expected = sorted(vec![
                        "pine apple pen apple".into(),
                        "pineapple pen apple".into(),
                        "pine applepen apple".into(),
                    ]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_no_break() {
                    let result = <$solution>::word_break(
                        "catsandog".into(),
                        vec![
                            "cats".into(), "dog".into(), "sand".into(),
                            "and".into(), "cat".into(),
                        ],
                    );
                    assert!(result.is_empty());
                }

                #[test]
                fn test_single_word() {
                    let result = <$solution>::word_break(
                        "a".into(), vec!["a".into()],
                    );
                    assert_eq!(result, vec!["a".to_string()]);
                }
            }
        };
    }

    test_word_break!(my, crate::my_solution::Solution);
    test_word_break!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let s = "catsanddog".to_string();
        let dict = vec![
            "cat".into(), "cats".into(), "and".into(),
            "sand".into(), "dog".into(),
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
