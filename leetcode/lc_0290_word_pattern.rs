// LeetCode 290: Word Pattern [Easy]
// https://leetcode.com/problems/word-pattern/
//
// First benchmark: 100_000 iterations, 2025-07-06
//   My solution avg: 2.782us
//   AI solution avg: 3.108us
//   Ratio: My 1.12x faster
//
// Given a pattern and a string s, determine if s follows the same pattern.
// "Follow" means a bijection between each letter in pattern and each word in s.
//
// Example 1: pattern = "abba", s = "dog cat cat dog" -> true
// Example 2: pattern = "abba", s = "dog cat cat fish" -> false
// Example 3: pattern = "aaaa", s = "dog cat cat dog" -> false
//
// Constraints:
//   - 1 <= pattern.length <= 300
//   - pattern contains only lower-case English letters
//   - 1 <= s.length <= 3000
//   - s contains only lowercase English letters and spaces
//   - No leading/trailing spaces, words separated by single space
//
// Run:  cargo run --example lc_0290_word_pattern
// Test: cargo test --example lc_0290_word_pattern
// Bench: cargo test --example lc_0290_word_pattern bench -- --nocapture

use std::collections::HashMap;

struct Solution;

/// My solution — Two HashMaps for bijection (doocs approach).
///
/// Map char->word and word->char. If any existing mapping conflicts,
/// return false. Ensures bijection (no two chars map to same word).
///
/// Time:  O(m + n)
/// Space: O(m + n)
mod my_solution {
    use super::*;

    impl Solution {
        pub fn word_pattern_my(pattern: &str, s: &str) -> bool {
            let words: Vec<&str> = s.split_whitespace().collect();
            let chars: Vec<char> = pattern.chars().collect();
            if chars.len() != words.len() {
                return false;
            }
            let mut d1: HashMap<char, &str> = HashMap::new();
            let mut d2: HashMap<&str, char> = HashMap::new();
            for (&a, &b) in chars.iter().zip(words.iter()) {
                if let Some(&mapped) = d1.get(&a) {
                    if mapped != b {
                        return false;
                    }
                }
                if let Some(&mapped) = d2.get(&b) {
                    if mapped != a {
                        return false;
                    }
                }
                d1.insert(a, b);
                d2.insert(b, a);
            }
            true
        }
    }
}

/// AI solution — First-occurrence index comparison (doocs Rust).
///
/// For each position, record the first index where each char/word appears.
/// If char's first-index != word's first-index at any position, no bijection.
///
/// Time:  O(m + n)
/// Space: O(m + n)
mod ai_solution {
    use super::*;

    impl Solution {
        pub fn word_pattern_ai(pattern: &str, s: &str) -> bool {
            let words: Vec<&str> = s.split_whitespace().collect();
            let chars: Vec<char> = pattern.chars().collect();
            if chars.len() != words.len() {
                return false;
            }
            let mut map1: HashMap<char, usize> = HashMap::new();
            let mut map2: HashMap<&str, usize> = HashMap::new();
            for i in 0..chars.len() {
                // Record first occurrence index for char and word
                if !map1.contains_key(&chars[i]) {
                    map1.insert(chars[i], i);
                }
                if !map2.contains_key(&words[i]) {
                    map2.insert(words[i], i);
                }
                // If first-occurrence indices differ, bijection fails
                if map1.get(&chars[i]) != map2.get(&words[i]) {
                    return false;
                }
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_pattern {
        ($name:ident, $pattern:expr, $s:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    Solution::word_pattern_my($pattern, $s),
                    $expected,
                    "my_solution failed"
                );
                assert_eq!(
                    Solution::word_pattern_ai($pattern, $s),
                    $expected,
                    "ai_solution failed"
                );
            }
        };
    }

    test_pattern!(test_example_1, "abba", "dog cat cat dog", true);
    test_pattern!(test_example_2, "abba", "dog cat cat fish", false);
    test_pattern!(test_example_3, "aaaa", "dog cat cat dog", false);

    // Single char, single word
    test_pattern!(test_single, "a", "word", true);

    // Length mismatch
    test_pattern!(test_length_mismatch, "ab", "dog", false);
    test_pattern!(test_length_mismatch_2, "a", "dog cat", false);

    // All same pattern, all same word
    test_pattern!(test_all_same, "aaa", "dog dog dog", true);

    // All different
    test_pattern!(test_all_different, "abc", "dog cat fish", true);

    // Same word mapped to two different chars
    test_pattern!(test_not_injective, "ab", "dog dog", false);

    // Pattern repeat
    test_pattern!(test_abab, "abab", "dog cat dog cat", true);
    test_pattern!(test_abab_fail, "abab", "dog cat dog fish", false);

    // Longer example
    test_pattern!(test_long_match, "abcabc", "x y z x y z", true);
    test_pattern!(test_long_fail, "abcabc", "x y z x y w", false);
}

fn main() {
    println!("LeetCode 290: Word Pattern");
    let pattern = "abba";
    let s = "dog cat cat dog";
    println!(
        "my_solution:  word_pattern(\"{}\", \"{}\") = {}",
        pattern,
        s,
        Solution::word_pattern_my(pattern, s)
    );
    println!(
        "ai_solution:  word_pattern(\"{}\", \"{}\") = {}",
        pattern,
        s,
        Solution::word_pattern_ai(pattern, s)
    );
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        let pattern = "abcdefghijklmnop";
        let s = "one two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen sixteen";
        let iterations = 100_000;

        // Warm up
        for _ in 0..1_000 {
            std::hint::black_box(Solution::word_pattern_my(pattern, s));
            std::hint::black_box(Solution::word_pattern_ai(pattern, s));
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(Solution::word_pattern_my(pattern, s));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(Solution::word_pattern_ai(pattern, s));
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
