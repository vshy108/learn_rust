// LeetCode 30: Substring with Concatenation of All Words [Hard]
// https://leetcode.com/problems/substring-with-concatenation-of-all-words/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 14.512µs
//   AI solution avg: 16.081µs
//   Ratio: ~tied (my 1.11x faster)
//
// Given a string s and an array of strings words (all same length), return
// all starting indices of substrings in s that are a concatenation of each
// word in words exactly once, in any order.
//
// Example 1: s="barfoothefoobarman", words=["foo","bar"] -> [0,9]
// Example 2: s="wordgoodgoodgoodbestword", words=["word","good","best","word"] -> []
// Example 3: s="barfoofoobarthefoobarman", words=["bar","foo","the"] -> [6,9,12]
//
// Constraints:
//   1 <= s.length <= 10^4
//   1 <= words.length <= 5000
//   1 <= words[i].length <= 30
//   s and words[i] consist of lowercase English letters
//
// Run:  cargo run --example lc_0030_substring_with_concatenation
// Test: cargo test --example lc_0030_substring_with_concatenation
// Bench: cargo test --example lc_0030_substring_with_concatenation bench -- --nocapture

/// My solution — brute-force check at every position.
///
/// For each starting index, extract word-length chunks and compare
/// against a frequency map of `words`. If all words are consumed
/// exactly, record the index.
///
/// Time: O(n * m * w) where n = s.len(), m = words.len(), w = word length.
/// Space: O(m) for the frequency map.
mod my_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Find all starting indices of concatenated substrings.
        ///
        /// # Arguments
        /// * `s` - the string to search in
        /// * `words` - array of equal-length words
        ///
        /// # Returns
        /// Starting indices where a valid concatenation begins.
        pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
            if words.is_empty() || s.is_empty() {
                return vec![];
            }

            let word_len = words[0].len();
            let total_len = word_len * words.len();
            let s_bytes = s.as_bytes();

            if s.len() < total_len {
                return vec![];
            }

            // Build frequency map from words.
            let mut word_count: HashMap<&[u8], i32> = HashMap::new();
            for w in &words {
                *word_count.entry(w.as_bytes()).or_insert(0) += 1;
            }

            let mut result = Vec::new();

            // Try each possible starting position.
            for i in 0..=(s.len() - total_len) {
                let mut seen: HashMap<&[u8], i32> = HashMap::new();
                let mut matched = true;

                for j in 0..words.len() {
                    let start = i + j * word_len;
                    let chunk = &s_bytes[start..start + word_len];

                    match word_count.get(chunk) {
                        Some(&needed) => {
                            let count = seen.entry(chunk).or_insert(0);
                            *count += 1;
                            if *count > needed {
                                matched = false;
                                break;
                            }
                        }
                        None => {
                            matched = false;
                            break;
                        }
                    }
                }

                if matched {
                    result.push(i as i32);
                }
            }

            result
        }
    }
}

/// AI-optimized solution — sliding window per offset.
///
/// Since all words have the same length `w`, run `w` sliding windows
/// starting at offsets 0..w. Each window slides by `w` characters,
/// maintaining a running count map. This avoids rebuilding the map
/// from scratch at each position.
///
/// Time: O(n * w) — w windows each doing O(n/w) work = O(n) per window.
/// Space: O(m) for the count map.
mod ai_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Find all starting indices of concatenated substrings.
        ///
        /// # Arguments
        /// * `s` - the string to search in
        /// * `words` - array of equal-length words
        ///
        /// # Returns
        /// Starting indices where a valid concatenation begins.
        pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
            if words.is_empty() || s.is_empty() {
                return vec![];
            }

            let word_len = words[0].len();
            let num_words = words.len();
            let total_len = word_len * num_words;
            let s_bytes = s.as_bytes();

            if s.len() < total_len {
                return vec![];
            }

            let mut word_count: HashMap<&[u8], i32> = HashMap::new();
            for w in &words {
                *word_count.entry(w.as_bytes()).or_insert(0) += 1;
            }

            let mut result = Vec::new();

            // Run one sliding window per offset in [0, word_len).
            for offset in 0..word_len {
                let mut window: HashMap<&[u8], i32> = HashMap::new();
                let mut matched = 0usize; // number of words matched exactly
                let mut left = offset;

                let mut right = offset;
                while right + word_len <= s.len() {
                    let chunk = &s_bytes[right..right + word_len];
                    right += word_len;

                    if let Some(&needed) = word_count.get(chunk) {
                        let count = window.entry(chunk).or_insert(0);
                        *count += 1;
                        if *count == needed {
                            matched += 1;
                        } else if *count == needed + 1 {
                            // Went over — no longer an exact match.
                            matched -= 1;
                        }
                    }

                    // Shrink window if it exceeds num_words slots.
                    let window_words = (right - left) / word_len;
                    if window_words > num_words {
                        let old = &s_bytes[left..left + word_len];
                        left += word_len;
                        if let Some(&needed) = word_count.get(old) {
                            let count = window.get_mut(old).unwrap();
                            if *count == needed {
                                matched -= 1;
                            } else if *count == needed + 1 {
                                matched += 1;
                            }
                            *count -= 1;
                        }
                    }

                    // If the chunk was not in word_count, reset the window.
                    if !word_count.contains_key(chunk) {
                        window.clear();
                        matched = 0;
                        left = right;
                        continue;
                    }

                    // All words matched exactly.
                    if matched == word_count.len() {
                        result.push(left as i32);
                    }
                }
            }

            result
        }
    }
}

fn main() {
    let cases: Vec<(&str, Vec<&str>)> = vec![
        ("barfoothefoobarman", vec!["foo", "bar"]),
        (
            "wordgoodgoodgoodbestword",
            vec!["word", "good", "best", "word"],
        ),
        ("barfoofoobarthefoobarman", vec!["bar", "foo", "the"]),
    ];
    for (s, words) in &cases {
        let ws: Vec<String> = words.iter().map(|w| w.to_string()).collect();
        let my = my_solution::Solution::find_substring(s.to_string(), ws.clone());
        let ai = ai_solution::Solution::find_substring(s.to_string(), ws);
        println!("{s:?}, {words:?} -> my:{my:?}, ai:{ai:?}");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_find_substring {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                fn to_words(w: &[&str]) -> Vec<String> {
                    w.iter().map(|s| s.to_string()).collect()
                }

                #[test]
                fn test_example_1() {
                    let mut result = Solution::find_substring(
                        "barfoothefoobarman".into(),
                        to_words(&["foo", "bar"]),
                    );
                    result.sort();
                    assert_eq!(result, vec![0, 9]);
                }

                #[test]
                fn test_example_2() {
                    let result = Solution::find_substring(
                        "wordgoodgoodgoodbestword".into(),
                        to_words(&["word", "good", "best", "word"]),
                    );
                    assert_eq!(result, vec![]);
                }

                #[test]
                fn test_example_3() {
                    let mut result = Solution::find_substring(
                        "barfoofoobarthefoobarman".into(),
                        to_words(&["bar", "foo", "the"]),
                    );
                    result.sort();
                    assert_eq!(result, vec![6, 9, 12]);
                }

                #[test]
                fn test_empty_words() {
                    let result = Solution::find_substring("hello".into(), vec![]);
                    assert_eq!(result, vec![]);
                }

                #[test]
                fn test_s_too_short() {
                    let result = Solution::find_substring("ab".into(), to_words(&["abc"]));
                    assert_eq!(result, vec![]);
                }

                #[test]
                fn test_single_word() {
                    let result = Solution::find_substring("foobarfoo".into(), to_words(&["foo"]));
                    let mut r = result;
                    r.sort();
                    assert_eq!(r, vec![0, 6]);
                }

                #[test]
                fn test_duplicate_words() {
                    let mut result =
                        Solution::find_substring("aaaaaa".into(), to_words(&["aa", "aa"]));
                    result.sort();
                    // Positions 0,1,2 all yield "aaaa" = "aa"+"aa".
                    assert_eq!(result, vec![0, 1, 2]);
                }

                #[test]
                fn test_no_match() {
                    let result = Solution::find_substring("abcdef".into(), to_words(&["gh", "ij"]));
                    assert_eq!(result, vec![]);
                }
            }
        };
    }

    test_find_substring!(my_tests, crate::my_solution::Solution);
    test_find_substring!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        let s = "barfoofoobarthefoobarman".to_string();
        let words: Vec<String> = vec!["bar".into(), "foo".into(), "the".into()];

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::find_substring(
                std::hint::black_box(s.clone()),
                std::hint::black_box(words.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::find_substring(
                std::hint::black_box(s.clone()),
                std::hint::black_box(words.clone()),
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
