// LeetCode 243: Shortest Word Distance [Easy] (Premium)
// https://leetcode.com/problems/shortest-word-distance/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 1.290us
//   AI solution avg: 1.390us
//   Ratio: My 1.08x faster
//
// Given an array of strings `wordsDict` and two different strings `word1`
// and `word2` that already exist in the array, return the shortest distance
// between these two words in the list.
//
// Example 1:
//   Input: wordsDict = ["practice","makes","perfect","coding","makes"],
//          word1 = "coding", word2 = "practice"
//   Output: 3
//
// Example 2:
//   Input: wordsDict = ["practice","makes","perfect","coding","makes"],
//          word1 = "makes", word2 = "coding"
//   Output: 1
//
// Constraints:
//   - 2 <= wordsDict.length <= 3 * 10^4
//   - 1 <= wordsDict[i].length <= 10
//   - wordsDict[i] consists of lowercase English letters
//   - word1 and word2 are in wordsDict
//   - word1 != word2
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0243.Shortest%20Word%20Distance/README_EN.md
//
// Run:  cargo run --example lc_0243_shortest_word_distance
// Test: cargo test --example lc_0243_shortest_word_distance
// Bench: cargo test --example lc_0243_shortest_word_distance bench -- --nocapture

/// My solution — single-pass two-pointer tracking (translated from Python in doocs/leetcode).
///
/// Track the last-seen index of word1 (i) and word2 (j). On each match,
/// if both have been seen, update the minimum |i - j|.
///
/// Time: O(n) — single pass through wordsDict
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the shortest distance between word1 and word2 in wordsDict.
        pub fn shortest_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
            // i, j track the last-seen index of word1, word2 respectively
            // -1 means "not yet seen"
            let mut i: i32 = -1;
            let mut j: i32 = -1;
            let mut ans = i32::MAX;
            for (k, w) in words_dict.iter().enumerate() {
                if *w == word1 {
                    i = k as i32;
                }
                if *w == word2 {
                    j = k as i32;
                }
                // Only compute distance when both words have been seen
                if i != -1 && j != -1 {
                    ans = ans.min((i - j).abs());
                }
            }
            ans
        }
    }
}

/// AI solution — collect indices then two-pointer merge.
///
/// Gather all indices where word1 and word2 appear into two sorted
/// vectors, then use a two-pointer scan to find the minimum absolute
/// difference. This separates collection from comparison.
///
/// Time: O(n) — one pass to collect, one pass to merge (both O(n))
/// Space: O(m) where m = total occurrences of word1 + word2
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the shortest distance between word1 and word2 in wordsDict.
        pub fn shortest_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
            // Collect indices for each word
            let mut idx1 = Vec::new();
            let mut idx2 = Vec::new();
            for (k, w) in words_dict.iter().enumerate() {
                if *w == word1 {
                    idx1.push(k as i32);
                } else if *w == word2 {
                    idx2.push(k as i32);
                }
            }
            // Two-pointer scan: both vectors are already sorted (indices
            // appear in order). Advance the pointer with the smaller value
            // to try to close the gap.
            let mut ans = i32::MAX;
            let (mut a, mut b) = (0, 0);
            while a < idx1.len() && b < idx2.len() {
                ans = ans.min((idx1[a] - idx2[b]).abs());
                if idx1[a] < idx2[b] {
                    a += 1;
                } else {
                    b += 1;
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
        "my: {}",
        my_solution::Solution::shortest_distance(words.clone(), "coding".into(), "practice".into())
    );
    println!(
        "ai: {}",
        ai_solution::Solution::shortest_distance(words, "coding".into(), "practice".into())
    );
}

#[cfg(test)]
mod tests {
    macro_rules! test_shortest_distance {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Sol;

                fn make(words: &[&str]) -> Vec<String> {
                    words.iter().map(|s| s.to_string()).collect()
                }

                #[test]
                fn test_example_1() {
                    let words = make(&["practice", "makes", "perfect", "coding", "makes"]);
                    assert_eq!(
                        Sol::shortest_distance(words, "coding".into(), "practice".into()),
                        3
                    );
                }

                #[test]
                fn test_example_2() {
                    let words = make(&["practice", "makes", "perfect", "coding", "makes"]);
                    assert_eq!(
                        Sol::shortest_distance(words, "makes".into(), "coding".into()),
                        1
                    );
                }

                #[test]
                fn test_adjacent() {
                    let words = make(&["a", "b"]);
                    assert_eq!(Sol::shortest_distance(words, "a".into(), "b".into()), 1);
                }

                #[test]
                fn test_far_apart() {
                    let words = make(&["a", "c", "c", "c", "b"]);
                    assert_eq!(Sol::shortest_distance(words, "a".into(), "b".into()), 4);
                }

                #[test]
                fn test_multiple_occurrences() {
                    let words = make(&["a", "b", "c", "a", "d", "b"]);
                    assert_eq!(Sol::shortest_distance(words, "a".into(), "b".into()), 1);
                }

                #[test]
                fn test_reversed_args() {
                    let words = make(&["a", "b", "c", "a", "d", "b"]);
                    assert_eq!(Sol::shortest_distance(words, "b".into(), "a".into()), 1);
                }

                #[test]
                fn test_only_two_words() {
                    let words = make(&["x", "y"]);
                    assert_eq!(Sol::shortest_distance(words, "x".into(), "y".into()), 1);
                }

                #[test]
                fn test_closest_at_end() {
                    let words = make(&["a", "x", "x", "x", "b", "x", "a"]);
                    assert_eq!(Sol::shortest_distance(words, "a".into(), "b".into()), 2);
                }
            }
        };
    }

    test_shortest_distance!(my, crate::my_solution::Solution);
    test_shortest_distance!(ai, crate::ai_solution::Solution);

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
            std::hint::black_box(crate::my_solution::Solution::shortest_distance(
                words.clone(),
                "practice".into(),
                "coding".into(),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::shortest_distance(
                words.clone(),
                "practice".into(),
                "coding".into(),
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
