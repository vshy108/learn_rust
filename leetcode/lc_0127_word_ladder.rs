// LeetCode 127: Word Ladder [Hard]
// https://leetcode.com/problems/word-ladder/
//
// First benchmark: 10_000 iterations, 2026-04-07
//   My solution avg: 14.219us
//   AI solution avg: 15.246us
//   Ratio: ~tied
//
// Given beginWord, endWord, and a wordList, return the number of words in the
// shortest transformation sequence from beginWord to endWord (0 if no such
// sequence exists). Each adjacent pair of words differs by a single letter.
//
// Constraints: 1 <= beginWord.length <= 10, 1 <= wordList.length <= 5000,
// all words same length, lowercase English letters
//
// Run:  cargo run --example lc_0127_word_ladder
// Test: cargo test --example lc_0127_word_ladder
// Bench: cargo test --example lc_0127_word_ladder bench -- --nocapture

use std::collections::{HashSet, VecDeque};

/// My solution — BFS changing one character at a time.
///
/// Time: O(n * L * 26) where n = word count, L = word length.
/// Space: O(n * L).
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return length of shortest transformation sequence.
        pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
            let mut word_set: HashSet<String> = word_list.into_iter().collect();
            if !word_set.contains(&end_word) {
                return 0;
            }

            let mut queue = VecDeque::new();
            queue.push_back((begin_word, 1));

            while let Some((word, depth)) = queue.pop_front() {
                let bytes = word.into_bytes();
                for i in 0..bytes.len() {
                    for c in b'a'..=b'z' {
                        if c == bytes[i] {
                            continue;
                        }
                        let mut next = bytes.clone();
                        next[i] = c;
                        let next_word = String::from_utf8(next).expect("valid ascii");
                        if next_word == end_word {
                            return depth + 1;
                        }
                        if word_set.remove(&next_word) {
                            queue.push_back((next_word, depth + 1));
                        }
                    }
                }
            }
            0
        }
    }
}

/// AI solution — bidirectional BFS for faster convergence.
///
/// Time: O(n * L * 26) but typically much faster in practice.
/// Space: O(n * L).
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return shortest sequence length using bidirectional BFS.
        pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
            let mut word_set: HashSet<String> = word_list.into_iter().collect();
            if !word_set.contains(&end_word) {
                return 0;
            }

            let mut front: HashSet<String> = HashSet::new();
            let mut back: HashSet<String> = HashSet::new();
            front.insert(begin_word);
            back.insert(end_word.clone());
            word_set.remove(&end_word);
            let mut depth = 1;

            while !front.is_empty() && !back.is_empty() {
                depth += 1;
                // Always expand the smaller frontier
                if front.len() > back.len() {
                    std::mem::swap(&mut front, &mut back);
                }
                let mut next_front = HashSet::new();
                for word in &front {
                    let bytes = word.as_bytes();
                    for i in 0..bytes.len() {
                        for c in b'a'..=b'z' {
                            if c == bytes[i] {
                                continue;
                            }
                            let mut next = bytes.to_vec();
                            next[i] = c;
                            let next_word = String::from_utf8(next).expect("valid ascii");
                            if back.contains(&next_word) {
                                return depth;
                            }
                            if word_set.remove(&next_word) {
                                next_front.insert(next_word);
                            }
                        }
                    }
                }
                front = next_front;
            }
            0
        }
    }
}

fn main() {
    let words: Vec<String> = vec!["hot", "dot", "dog", "lot", "log", "cog"]
        .into_iter()
        .map(String::from)
        .collect();
    let my = my_solution::Solution::ladder_length("hit".into(), "cog".into(), words.clone());
    let ai = ai_solution::Solution::ladder_length("hit".into(), "cog".into(), words);
    println!("ladder length: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    fn to_words(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    macro_rules! test_ladder {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::to_words;

                #[test]
                fn test_example1() {
                    let words = to_words(&["hot", "dot", "dog", "lot", "log", "cog"]);
                    assert_eq!(
                        <$solution>::ladder_length("hit".into(), "cog".into(), words),
                        5,
                    );
                }

                #[test]
                fn test_no_path() {
                    let words = to_words(&["hot", "dot", "dog", "lot", "log"]);
                    assert_eq!(
                        <$solution>::ladder_length("hit".into(), "cog".into(), words),
                        0,
                    );
                }

                #[test]
                fn test_direct() {
                    let words = to_words(&["hot"]);
                    assert_eq!(
                        <$solution>::ladder_length("hit".into(), "hot".into(), words),
                        2,
                    );
                }

                #[test]
                fn test_end_not_in_list() {
                    let words = to_words(&["hot"]);
                    assert_eq!(
                        <$solution>::ladder_length("hit".into(), "cog".into(), words),
                        0,
                    );
                }

                #[test]
                fn test_longer_chain() {
                    let words = to_words(&["hot", "dot", "lot", "dof", "lof", "cog", "cof"]);
                    let result = <$solution>::ladder_length("hit".into(), "cog".into(), words);
                    assert!(result > 0);
                }
            }
        };
    }

    test_ladder!(my, crate::my_solution::Solution);
    test_ladder!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 10_000;
        let words: Vec<String> = vec!["hot", "dot", "dog", "lot", "log", "cog"]
            .into_iter()
            .map(String::from)
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::ladder_length(
                std::hint::black_box("hit".into()),
                std::hint::black_box("cog".into()),
                std::hint::black_box(words.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::ladder_length(
                std::hint::black_box("hit".into()),
                std::hint::black_box("cog".into()),
                std::hint::black_box(words.clone()),
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
            println!("~tied (ratio: {ratio:.2})");
        }
    }
}
