// LeetCode 244: Shortest Word Distance II [Medium] (Premium)
// https://leetcode.com/problems/shortest-word-distance-ii/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 3.094us
//   AI solution avg: 3.221us
//   Ratio: My 1.04x faster
//
// Design a data structure initialized with a string array that answers
// queries of the shortest distance between two different strings.
//
// Implement WordDistance:
//   - WordDistance(wordsDict) initializes with the strings array
//   - int shortest(word1, word2) returns shortest distance between them
//
// Example:
//   WordDistance(["practice","makes","perfect","coding","makes"])
//   shortest("coding", "practice") -> 3
//   shortest("makes", "coding")    -> 1
//
// Constraints:
//   - 1 <= wordsDict.length <= 3 * 10^4
//   - 1 <= wordsDict[i].length <= 10
//   - wordsDict[i] consists of lowercase English letters
//   - word1 and word2 are in wordsDict
//   - word1 != word2
//   - At most 5000 calls to shortest
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0244.Shortest%20Word%20Distance%20II/README_EN.md
//
// Run:  cargo run --example lc_0244_shortest_word_distance_ii
// Test: cargo test --example lc_0244_shortest_word_distance_ii
// Bench: cargo test --example lc_0244_shortest_word_distance_ii bench -- --nocapture

/// My solution — HashMap of index lists + two-pointer merge (translated from Python in doocs).
///
/// Pre-build a HashMap mapping each word to its sorted list of indices.
/// For `shortest`, retrieve both lists and use a two-pointer scan to
/// find the minimum |a[i] - b[j]|. Advance the smaller pointer each step.
///
/// Time: constructor O(n), shortest O(m1 + m2) where m1, m2 = occurrences
/// Space: O(n)
mod my_solution {
    use std::collections::HashMap;

    pub struct WordDistance {
        d: HashMap<String, Vec<i32>>,
    }

    impl WordDistance {
        /// Initialize with the words dictionary, recording each word's indices.
        pub fn new(words_dict: Vec<String>) -> Self {
            let mut d: HashMap<String, Vec<i32>> = HashMap::new();
            for (i, w) in words_dict.into_iter().enumerate() {
                d.entry(w).or_default().push(i as i32);
            }
            Self { d }
        }

        /// Return shortest distance between word1 and word2.
        pub fn shortest(&mut self, word1: &str, word2: &str) -> i32 {
            let a = &self.d[word1];
            let b = &self.d[word2];
            let mut ans = i32::MAX;
            let (mut i, mut j) = (0, 0);
            // Both lists are sorted so two-pointer finds min distance in O(m1+m2)
            while i < a.len() && j < b.len() {
                ans = ans.min((a[i] - b[j]).abs());
                if a[i] <= b[j] {
                    i += 1;
                } else {
                    j += 1;
                }
            }
            ans
        }
    }
}

/// AI solution — HashMap + cached results for repeated queries.
///
/// Same index-list approach for construction, but caches query results
/// in a separate HashMap<(word1, word2), i32> so repeated shortest()
/// calls for the same pair return in O(1). Normalizes key order so
/// (word1, word2) and (word2, word1) share the same cache entry.
///
/// Time: constructor O(n), first shortest O(m1+m2), repeated O(1)
/// Space: O(n + q) where q = unique queried pairs
mod ai_solution {
    use std::collections::HashMap;

    pub struct WordDistance {
        d: HashMap<String, Vec<i32>>,
        cache: HashMap<(String, String), i32>,
    }

    impl WordDistance {
        /// Initialize with the words dictionary.
        pub fn new(words_dict: Vec<String>) -> Self {
            let mut d: HashMap<String, Vec<i32>> = HashMap::new();
            for (i, w) in words_dict.into_iter().enumerate() {
                d.entry(w).or_default().push(i as i32);
            }
            Self {
                d,
                cache: HashMap::new(),
            }
        }

        /// Return shortest distance, caching results for repeated queries.
        pub fn shortest(&mut self, word1: &str, word2: &str) -> i32 {
            // Normalize key so (a,b) and (b,a) share cache entry
            let key = if word1 <= word2 {
                (word1.to_string(), word2.to_string())
            } else {
                (word2.to_string(), word1.to_string())
            };
            if let Some(&cached) = self.cache.get(&key) {
                return cached;
            }
            let a = &self.d[word1];
            let b = &self.d[word2];
            let mut ans = i32::MAX;
            let (mut i, mut j) = (0, 0);
            while i < a.len() && j < b.len() {
                ans = ans.min((a[i] - b[j]).abs());
                if a[i] <= b[j] {
                    i += 1;
                } else {
                    j += 1;
                }
            }
            self.cache.insert(key, ans);
            ans
        }
    }
}

fn main() {
    let words: Vec<String> = vec!["practice", "makes", "perfect", "coding", "makes"]
        .into_iter()
        .map(String::from)
        .collect();

    let mut wd = my_solution::WordDistance::new(words.clone());
    println!("my: coding-practice={}", wd.shortest("coding", "practice"));
    println!("my: makes-coding={}", wd.shortest("makes", "coding"));

    let mut wd = ai_solution::WordDistance::new(words);
    println!("ai: coding-practice={}", wd.shortest("coding", "practice"));
    println!("ai: makes-coding={}", wd.shortest("makes", "coding"));
}

#[cfg(test)]
mod tests {
    macro_rules! test_word_distance {
        ($mod_name:ident, $solution:ident) => {
            mod $mod_name {
                use crate::$solution::WordDistance;

                fn make(words: &[&str]) -> Vec<String> {
                    words.iter().map(|s| s.to_string()).collect()
                }

                #[test]
                fn test_example() {
                    let words = make(&["practice", "makes", "perfect", "coding", "makes"]);
                    let mut wd = WordDistance::new(words);
                    assert_eq!(wd.shortest("coding", "practice"), 3);
                    assert_eq!(wd.shortest("makes", "coding"), 1);
                }

                #[test]
                fn test_adjacent() {
                    let mut wd = WordDistance::new(make(&["a", "b"]));
                    assert_eq!(wd.shortest("a", "b"), 1);
                }

                #[test]
                fn test_multiple_calls_same_pair() {
                    let words = make(&["a", "b", "c", "a"]);
                    let mut wd = WordDistance::new(words);
                    assert_eq!(wd.shortest("a", "c"), 1);
                    assert_eq!(wd.shortest("a", "c"), 1);
                }

                #[test]
                fn test_reversed_args() {
                    let words = make(&["a", "x", "x", "b"]);
                    let mut wd = WordDistance::new(words);
                    assert_eq!(wd.shortest("a", "b"), 3);
                    assert_eq!(wd.shortest("b", "a"), 3);
                }

                #[test]
                fn test_multiple_occurrences() {
                    let words = make(&["a", "b", "c", "b", "a"]);
                    let mut wd = WordDistance::new(words);
                    assert_eq!(wd.shortest("a", "b"), 1);
                    assert_eq!(wd.shortest("a", "c"), 2);
                }

                #[test]
                fn test_different_queries() {
                    let words = make(&["a", "b", "c", "d", "e"]);
                    let mut wd = WordDistance::new(words);
                    assert_eq!(wd.shortest("a", "e"), 4);
                    assert_eq!(wd.shortest("b", "d"), 2);
                    assert_eq!(wd.shortest("c", "d"), 1);
                }
            }
        };
    }

    test_word_distance!(my, my_solution);
    test_word_distance!(ai, ai_solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let words: Vec<String> = vec![
            "practice", "makes", "perfect", "coding", "makes", "the",
            "sky", "is", "blue", "practice", "more", "coding", "every",
            "day", "makes", "you", "better", "at", "coding", "practice",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut wd = crate::my_solution::WordDistance::new(words.clone());
            std::hint::black_box(wd.shortest("practice", "coding"));
            std::hint::black_box(wd.shortest("makes", "blue"));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut wd = crate::ai_solution::WordDistance::new(words.clone());
            std::hint::black_box(wd.shortest("practice", "coding"));
            std::hint::black_box(wd.shortest("makes", "blue"));
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
