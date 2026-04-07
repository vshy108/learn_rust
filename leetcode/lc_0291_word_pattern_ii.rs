// LeetCode 291: Word Pattern II [Medium] (Premium)
// https://leetcode.com/problems/word-pattern-ii/
//
// First benchmark: 100_000 iterations, 2025-07-06
//   My solution avg: 3.831us
//   AI solution avg: 1.703us
//   Ratio: AI 2.25x faster
//
// Given a pattern and a string s, return true if s matches the pattern.
// A bijective mapping of single characters to non-empty strings must exist
// such that replacing each pattern char by its mapped string yields s.
//
// Example 1: pattern = "abab", s = "redblueredblue" -> true (a->red, b->blue)
// Example 2: pattern = "aaaa", s = "asdasdasdasd" -> true (a->asd)
// Example 3: pattern = "aabb", s = "xyzabcxzyabc" -> false
//
// Constraints:
//   - 1 <= pattern.length, s.length <= 20
//   - pattern and s consist of only lowercase English letters
//
// Run:  cargo run --example lc_0291_word_pattern_ii
// Test: cargo test --example lc_0291_word_pattern_ii
// Bench: cargo test --example lc_0291_word_pattern_ii bench -- --nocapture

use std::collections::{HashMap, HashSet};

struct Solution;

/// My solution — Backtracking with char->string and string->char maps (doocs).
///
/// DFS tries every possible substring for the current pattern char.
/// If the char is already mapped, only proceed if the mapping matches.
/// If unmapped, try each possible substring, add to both maps, recurse,
/// then backtrack. Uses a `vis` set to ensure bijection.
///
/// Time:  O(n^m) worst case where m = pattern length, n = string length
/// Space: O(m + n)
mod my_solution {
    use super::*;

    impl Solution {
        pub fn word_pattern_match_my(pattern: &str, s: &str) -> bool {
            let p: Vec<u8> = pattern.bytes().collect();
            let sb = s.as_bytes();
            let mut d: HashMap<u8, Vec<u8>> = HashMap::new();
            let mut vis: HashSet<Vec<u8>> = HashSet::new();
            Self::dfs_my(&p, sb, 0, 0, &mut d, &mut vis)
        }

        fn dfs_my(
            p: &[u8],
            s: &[u8],
            i: usize,
            j: usize,
            d: &mut HashMap<u8, Vec<u8>>,
            vis: &mut HashSet<Vec<u8>>,
        ) -> bool {
            let m = p.len();
            let n = s.len();
            if i == m && j == n {
                return true;
            }
            // Prune: not enough chars left, or one side exhausted
            if i == m || j == n || m - i > n - j {
                return false;
            }
            let c = p[i];
            for k in (j + 1)..=n {
                let t = s[j..k].to_vec();
                // If char is already mapped, check if mapping matches
                if let Some(mapped) = d.get(&c) {
                    if *mapped == t {
                        if Self::dfs_my(p, s, i + 1, k, d, vis) {
                            return true;
                        }
                    }
                    // mapped != t, skip this substring length
                } else if !vis.contains(&t) {
                    // Try mapping c -> t
                    d.insert(c, t.clone());
                    vis.insert(t.clone());
                    if Self::dfs_my(p, s, i + 1, k, d, vis) {
                        return true;
                    }
                    d.remove(&c);
                    vis.remove(&t);
                }
            }
            false
        }
    }
}

/// AI solution — Backtracking with early match-check optimization.
///
/// Same backtracking approach but when a char already has a mapping,
/// directly check if the mapped string is a prefix at position j
/// instead of iterating all substring lengths.
///
/// Time:  O(n^m) worst case
/// Space: O(m + n)
mod ai_solution {
    use super::*;

    impl Solution {
        pub fn word_pattern_match_ai(pattern: &str, s: &str) -> bool {
            let p: Vec<u8> = pattern.bytes().collect();
            let sb = s.as_bytes();
            let mut d: HashMap<u8, Vec<u8>> = HashMap::new();
            let mut vis: HashSet<Vec<u8>> = HashSet::new();
            Self::dfs_ai(&p, sb, 0, 0, &mut d, &mut vis)
        }

        fn dfs_ai(
            p: &[u8],
            s: &[u8],
            i: usize,
            j: usize,
            d: &mut HashMap<u8, Vec<u8>>,
            vis: &mut HashSet<Vec<u8>>,
        ) -> bool {
            let m = p.len();
            let n = s.len();
            if i == m && j == n {
                return true;
            }
            if i == m || j == n || m - i > n - j {
                return false;
            }
            let c = p[i];

            // If char already mapped, just verify prefix and recurse
            if let Some(mapped) = d.get(&c) {
                let len = mapped.len();
                if j + len <= n && &s[j..j + len] == mapped.as_slice() {
                    return Self::dfs_ai(p, s, i + 1, j + len, d, vis);
                }
                return false;
            }

            // Try all possible substrings for this new char
            for k in (j + 1)..=n {
                let t = s[j..k].to_vec();
                if !vis.contains(&t) {
                    d.insert(c, t.clone());
                    vis.insert(t.clone());
                    if Self::dfs_ai(p, s, i + 1, k, d, vis) {
                        return true;
                    }
                    d.remove(&c);
                    vis.remove(&t);
                }
            }
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_wp {
        ($name:ident, $pattern:expr, $s:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    Solution::word_pattern_match_my($pattern, $s),
                    $expected,
                    "my_solution failed"
                );
                assert_eq!(
                    Solution::word_pattern_match_ai($pattern, $s),
                    $expected,
                    "ai_solution failed"
                );
            }
        };
    }

    test_wp!(test_example_1, "abab", "redblueredblue", true);
    test_wp!(test_example_2, "aaaa", "asdasdasdasd", true);
    test_wp!(test_example_3, "aabb", "xyzabcxzyabc", false);

    // Single char pattern maps to entire string
    test_wp!(test_single_char, "a", "hello", true);

    // Two different chars
    test_wp!(test_ab, "ab", "ab", true);

    // Entire string must be consumed
    test_wp!(test_no_leftover, "a", "ab", true);

    // Pattern longer than string
    test_wp!(test_pattern_too_long, "abcde", "ab", false);

    // Same char must map to same string
    test_wp!(test_aa_same, "aa", "abab", true);
    test_wp!(test_aa_diff, "aa", "abc", false);

    // Bijection: different chars must map to different strings
    test_wp!(test_bijection_fail, "ab", "aa", false);

    // Complex backtracking
    test_wp!(test_complex_true, "abba", "dogcatcatdog", true);
    test_wp!(test_complex_false, "abba", "dogcatcatfish", false);
}

fn main() {
    println!("LeetCode 291: Word Pattern II");
    println!(
        "my_solution: word_pattern_match(\"abab\", \"redblueredblue\") = {}",
        Solution::word_pattern_match_my("abab", "redblueredblue")
    );
    println!(
        "ai_solution: word_pattern_match(\"abab\", \"redblueredblue\") = {}",
        Solution::word_pattern_match_ai("abab", "redblueredblue")
    );
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        let pattern = "abab";
        let s = "redblueredblue";
        let iterations = 100_000;

        // Warm up
        for _ in 0..1_000 {
            std::hint::black_box(Solution::word_pattern_match_my(pattern, s));
            std::hint::black_box(Solution::word_pattern_match_ai(pattern, s));
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(Solution::word_pattern_match_my(pattern, s));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(Solution::word_pattern_match_ai(pattern, s));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ratio = if my_avg < ai_avg {
            format!("My {:.2}x faster", ai_avg / my_avg)
        } else {
            format!("AI {:.2}x faster", my_avg / ai_avg)
        };

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution avg: {my_avg:.3}us");
        println!("AI solution avg: {ai_avg:.3}us");
        println!("Ratio: {ratio}");
    }
}
