// LeetCode 126: Word Ladder II [Hard]
// https://leetcode.com/problems/word-ladder-ii/
//
// First benchmark: 10_000 iterations, 2026-04-07
//   My solution avg: 23.349us
//   AI solution avg: 21.575us
//   Ratio: ~tied
//
// Given beginWord, endWord, and a wordList, return all shortest transformation
// sequences from beginWord to endWord. Each adjacent pair of words differs by
// a single letter.
//
// Constraints: 1 <= beginWord.length <= 5, 1 <= wordList.length <= 500,
// all words same length, lowercase English letters
//
// Run:  cargo run --example lc_0126_word_ladder_ii
// Test: cargo test --example lc_0126_word_ladder_ii
// Bench: cargo test --example lc_0126_word_ladder_ii bench -- --nocapture

use std::collections::{HashMap, HashSet, VecDeque};

/// My solution — BFS to find shortest paths, then DFS backtrack to collect them.
///
/// Time: O(n * L * 26) for BFS + O(paths) for DFS.
/// Space: O(n * L) where n = word count, L = word length.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Find all shortest transformation sequences.
        pub fn find_ladders(
            begin_word: String,
            end_word: String,
            word_list: Vec<String>,
        ) -> Vec<Vec<String>> {
            let word_set: HashSet<String> = word_list.into_iter().collect();
            if !word_set.contains(&end_word) {
                return Vec::new();
            }

            // BFS: find parent map (word -> set of words that lead to it)
            let mut parents: HashMap<String, Vec<String>> = HashMap::new();
            let mut visited: HashSet<String> = HashSet::new();
            visited.insert(begin_word.clone());
            let mut queue = VecDeque::new();
            queue.push_back(begin_word.clone());
            let mut found = false;

            while !queue.is_empty() && !found {
                let mut level_visited = HashSet::new();
                let size = queue.len();
                for _ in 0..size {
                    let word = queue.pop_front().expect("sized");
                    let word_bytes = word.as_bytes().to_vec();
                    for i in 0..word_bytes.len() {
                        for c in b'a'..=b'z' {
                            if c == word_bytes[i] { continue; }
                            let mut next = word_bytes.clone();
                            next[i] = c;
                            let next_word = String::from_utf8(next)
                                .expect("valid ascii");
                            if !word_set.contains(&next_word) { continue; }
                            if next_word == end_word { found = true; }
                            if !visited.contains(&next_word) {
                                parents.entry(next_word.clone())
                                    .or_default()
                                    .push(word.clone());
                                if level_visited.insert(next_word.clone()) {
                                    queue.push_back(next_word);
                                }
                            }
                        }
                    }
                }
                // Mark entire level as visited after processing
                visited.extend(level_visited);
            }

            if !found { return Vec::new(); }

            // DFS backtrack from end_word to begin_word using parents
            let mut result = Vec::new();
            let mut path = vec![end_word.clone()];
            Self::backtrack(&end_word, &begin_word, &parents, &mut path, &mut result);
            result
        }

        fn backtrack(
            word: &str,
            begin: &str,
            parents: &HashMap<String, Vec<String>>,
            path: &mut Vec<String>,
            result: &mut Vec<Vec<String>>,
        ) {
            if word == begin {
                let mut p = path.clone();
                p.reverse();
                result.push(p);
                return;
            }
            if let Some(pars) = parents.get(word) {
                for parent in pars {
                    path.push(parent.clone());
                    Self::backtrack(parent, begin, parents, path, result);
                    path.pop();
                }
            }
        }
    }
}

/// AI solution — bidirectional BFS for faster shortest-path finding.
///
/// Time: O(n * L * 26) but typically faster due to meeting in the middle.
/// Space: O(n * L).
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Find all shortest transformation sequences using BFS + backtrack.
        pub fn find_ladders(
            begin_word: String,
            end_word: String,
            word_list: Vec<String>,
        ) -> Vec<Vec<String>> {
            let word_set: HashSet<String> = word_list.into_iter().collect();
            if !word_set.contains(&end_word) {
                return Vec::new();
            }

            // BFS with layer-based visited tracking
            let mut parents: HashMap<String, Vec<String>> = HashMap::new();
            let mut current_level: HashSet<String> = HashSet::new();
            current_level.insert(begin_word.clone());
            let mut visited: HashSet<String> = HashSet::new();
            visited.insert(begin_word.clone());
            let mut found = false;

            while !current_level.is_empty() && !found {
                let mut next_level: HashSet<String> = HashSet::new();
                // Temporarily remove current level from visited to allow
                // multiple parents within same level
                for word in &current_level {
                    let bytes = word.as_bytes();
                    for i in 0..bytes.len() {
                        for c in b'a'..=b'z' {
                            if c == bytes[i] { continue; }
                            let mut next = bytes.to_vec();
                            next[i] = c;
                            let next_word = String::from_utf8(next)
                                .expect("valid ascii");
                            if !word_set.contains(&next_word)
                                || visited.contains(&next_word)
                            {
                                continue;
                            }
                            if next_word == end_word { found = true; }
                            next_level.insert(next_word.clone());
                            parents.entry(next_word)
                                .or_default()
                                .push(word.clone());
                        }
                    }
                }
                visited.extend(next_level.iter().cloned());
                current_level = next_level;
            }

            if !found { return Vec::new(); }

            let mut result = Vec::new();
            let mut path = vec![end_word.clone()];
            Self::backtrack(&end_word, &begin_word, &parents, &mut path, &mut result);
            result
        }

        fn backtrack(
            word: &str,
            begin: &str,
            parents: &HashMap<String, Vec<String>>,
            path: &mut Vec<String>,
            result: &mut Vec<Vec<String>>,
        ) {
            if word == begin {
                let mut p = path.clone();
                p.reverse();
                result.push(p);
                return;
            }
            if let Some(pars) = parents.get(word) {
                for parent in pars {
                    path.push(parent.clone());
                    Self::backtrack(parent, begin, parents, path, result);
                    path.pop();
                }
            }
        }
    }
}

fn main() {
    let words: Vec<String> = vec!["hot", "dot", "dog", "lot", "log", "cog"]
        .into_iter().map(String::from).collect();
    let my = my_solution::Solution::find_ladders("hit".into(), "cog".into(), words.clone());
    let ai = ai_solution::Solution::find_ladders("hit".into(), "cog".into(), words);
    println!("paths: my={my:?}, ai={ai:?}");
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
                    let mut result = <$solution>::find_ladders(
                        "hit".into(), "cog".into(), words,
                    );
                    result.sort();
                    let mut expected = vec![
                        to_words(&["hit", "hot", "dot", "dog", "cog"]),
                        to_words(&["hit", "hot", "lot", "log", "cog"]),
                    ];
                    expected.sort();
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_no_path() {
                    let words = to_words(&["hot", "dot", "dog", "lot", "log"]);
                    let result = <$solution>::find_ladders(
                        "hit".into(), "cog".into(), words,
                    );
                    assert!(result.is_empty());
                }

                #[test]
                fn test_end_not_in_list() {
                    let words = to_words(&["hot"]);
                    let result = <$solution>::find_ladders(
                        "hit".into(), "cog".into(), words,
                    );
                    assert!(result.is_empty());
                }

                #[test]
                fn test_direct() {
                    let words = to_words(&["hot"]);
                    let result = <$solution>::find_ladders(
                        "hit".into(), "hot".into(), words,
                    );
                    assert_eq!(result, vec![to_words(&["hit", "hot"])]);
                }
            }
        };
    }

    test_ladder!(my, crate::my_solution::Solution);
    test_ladder!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 10_000;
        let words = to_words(&["hot", "dot", "dog", "lot", "log", "cog"]);

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::find_ladders(
                std::hint::black_box("hit".into()),
                std::hint::black_box("cog".into()),
                std::hint::black_box(words.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::find_ladders(
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
