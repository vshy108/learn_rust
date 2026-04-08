// LeetCode 245: Shortest Word Distance III [Medium] (Premium)
// https://leetcode.com/problems/shortest-word-distance-iii/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 2.702us
//   AI solution avg: 2.530us
//   Ratio: AI 1.07x faster
//
// Given an array of strings `wordsDict` and two strings `word1` and `word2`,
// return the shortest distance between their occurrences in the list.
// NOTE: word1 and word2 may be the same. They represent two individual words.
//
// Example 1:
//   Input: wordsDict = ["practice","makes","perfect","coding","makes"],
//          word1 = "makes", word2 = "coding"
//   Output: 1
//
// Example 2:
//   Input: wordsDict = ["practice","makes","perfect","coding","makes"],
//          word1 = "makes", word2 = "makes"
//   Output: 3
//
// Constraints:
//   - 1 <= wordsDict.length <= 10^5
//   - 1 <= wordsDict[i].length <= 10
//   - wordsDict[i] consists of lowercase English letters
//   - word1 and word2 are in wordsDict
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0245.Shortest%20Word%20Distance%20III/README_EN.md
//
// Run:  cargo run --example lc_0245_shortest_word_distance_iii
// Test: cargo test --example lc_0245_shortest_word_distance_iii
// Bench: cargo test --example lc_0245_shortest_word_distance_iii bench -- --nocapture

/// My solution — case analysis: same vs different words (translated from Python in doocs).
///
/// If word1 == word2, track a single previous index `j` for that word
/// and update distance each time we see another occurrence.
/// If word1 != word2, track indices `i` and `j` independently and
/// update whenever both have been seen.
///
/// Time: O(n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return shortest distance between word1 and word2 (may be same word).
        pub fn shortest_word_distance(
            words_dict: Vec<String>,
            word1: String,
            word2: String,
        ) -> i32 {
            let n = words_dict.len() as i32;
            let mut ans = n;
            if word1 == word2 {
                // Same word — find closest pair of occurrences
                let mut j: i32 = -1;
                for (i, w) in words_dict.iter().enumerate() {
                    if *w == word1 {
                        if j != -1 {
                            ans = ans.min(i as i32 - j);
                        }
                        j = i as i32;
                    }
                }
            } else {
                // Different words — standard two-index tracking
                let mut i: i32 = -1;
                let mut j: i32 = -1;
                for (k, w) in words_dict.iter().enumerate() {
                    if *w == word1 {
                        i = k as i32;
                    }
                    if *w == word2 {
                        j = k as i32;
                    }
                    if i != -1 && j != -1 {
                        ans = ans.min((i - j).abs());
                    }
                }
            }
            ans
        }
    }
}

/// AI solution — unified single-pass without branching on word equality.
///
/// Use a single variable `prev` to track the last-seen index of either
/// word. When we see word1 or word2, check if `prev` was set to the
/// *other* word's index (or, if same word, the previous occurrence).
/// This avoids the if/else branch on word equality.
///
/// Time: O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return shortest distance between word1 and word2 (may be same word).
        pub fn shortest_word_distance(
            words_dict: Vec<String>,
            word1: String,
            word2: String,
        ) -> i32 {
            let same = word1 == word2;
            let mut ans = words_dict.len() as i32;
            // i1 = last index of word1, i2 = last index of word2
            let mut i1: i32 = -1;
            let mut i2: i32 = -1;
            for (k, w) in words_dict.iter().enumerate() {
                let ki = k as i32;
                if *w == word1 {
                    // If same word, shift previous word1 to i2 before updating i1
                    // so we compare the two most recent occurrences
                    if same {
                        i2 = i1;
                    }
                    i1 = ki;
                } else if *w == word2 {
                    i2 = ki;
                }
                if i1 != -1 && i2 != -1 {
                    ans = ans.min((i1 - i2).abs());
                }
            }
            ans
        }
    }
}

fn main() {
    let words: Vec<String> = vec!["practice", "makes", "perfect", "coding", "makes"]
        .into_iter()
        .map(String::from)
        .collect();
    println!(
        "my (diff): {}",
        my_solution::Solution::shortest_word_distance(
            words.clone(),
            "makes".into(),
            "coding".into()
        )
    );
    println!(
        "my (same): {}",
        my_solution::Solution::shortest_word_distance(
            words.clone(),
            "makes".into(),
            "makes".into()
        )
    );
    println!(
        "ai (diff): {}",
        ai_solution::Solution::shortest_word_distance(
            words.clone(),
            "makes".into(),
            "coding".into()
        )
    );
    println!(
        "ai (same): {}",
        ai_solution::Solution::shortest_word_distance(words, "makes".into(), "makes".into())
    );
}

#[cfg(test)]
mod tests {
    macro_rules! test_shortest_word_distance {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Sol;

                fn make(words: &[&str]) -> Vec<String> {
                    words.iter().map(|s| s.to_string()).collect()
                }

                #[test]
                fn test_example_1_different() {
                    let words = make(&["practice", "makes", "perfect", "coding", "makes"]);
                    assert_eq!(
                        Sol::shortest_word_distance(words, "makes".into(), "coding".into()),
                        1
                    );
                }

                #[test]
                fn test_example_2_same() {
                    let words = make(&["practice", "makes", "perfect", "coding", "makes"]);
                    assert_eq!(
                        Sol::shortest_word_distance(words, "makes".into(), "makes".into()),
                        3
                    );
                }

                #[test]
                fn test_adjacent_same() {
                    let words = make(&["a", "a"]);
                    assert_eq!(
                        Sol::shortest_word_distance(words, "a".into(), "a".into()),
                        1
                    );
                }

                #[test]
                fn test_adjacent_different() {
                    let words = make(&["a", "b"]);
                    assert_eq!(
                        Sol::shortest_word_distance(words, "a".into(), "b".into()),
                        1
                    );
                }

                #[test]
                fn test_same_three_occurrences() {
                    // "a" at indices 0, 2, 5 -> min distance = 2
                    let words = make(&["a", "b", "a", "c", "d", "a"]);
                    assert_eq!(
                        Sol::shortest_word_distance(words, "a".into(), "a".into()),
                        2
                    );
                }

                #[test]
                fn test_different_multiple() {
                    let words = make(&["a", "b", "c", "a", "d", "b"]);
                    assert_eq!(
                        Sol::shortest_word_distance(words, "a".into(), "b".into()),
                        1
                    );
                }

                #[test]
                fn test_reversed_args() {
                    let words = make(&["a", "x", "b"]);
                    assert_eq!(
                        Sol::shortest_word_distance(words, "b".into(), "a".into()),
                        2
                    );
                }

                #[test]
                fn test_same_consecutive() {
                    let words = make(&["x", "a", "a", "x"]);
                    assert_eq!(
                        Sol::shortest_word_distance(words, "a".into(), "a".into()),
                        1
                    );
                }
            }
        };
    }

    test_shortest_word_distance!(my, crate::my_solution::Solution);
    test_shortest_word_distance!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let words: Vec<String> = vec![
            "practice", "makes", "perfect", "coding", "makes", "the", "sky", "is", "blue",
            "practice", "more", "coding", "every", "day", "makes", "you", "better", "at", "coding",
            "practice",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::shortest_word_distance(
                words.clone(),
                "makes".into(),
                "coding".into(),
            ));
            std::hint::black_box(crate::my_solution::Solution::shortest_word_distance(
                words.clone(),
                "makes".into(),
                "makes".into(),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::shortest_word_distance(
                words.clone(),
                "makes".into(),
                "coding".into(),
            ));
            std::hint::black_box(crate::ai_solution::Solution::shortest_word_distance(
                words.clone(),
                "makes".into(),
                "makes".into(),
            ));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else if ai_avg < my_avg {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        } else {
            println!("  Ratio: ~tied");
        }
    }
}
