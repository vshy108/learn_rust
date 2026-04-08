// LeetCode 425: Word Squares [Hard]
// https://leetcode.com/problems/word-squares/
//
// Given an array of unique strings `words`, return all word squares you can
// build. The same word can be used multiple times. A sequence of strings forms
// a valid word square if the k-th row and k-th column read the same string.
//
// Examples:
//   ["area","lead","wall","lady","ball"]
//   -> [["ball","area","lead","lady"], ["wall","area","lead","lady"]]
//
//   ["abat","baba","atan","atal"]
//   -> [["baba","abat","baba","atal"], ["baba","abat","baba","atan"]]
//
// Constraints:
//   1 <= words.length <= 1000
//   1 <= words[i].length <= 4
//   All words[i] have the same length
//   words[i] consists of only lowercase English letters
//   All words[i] are unique
//
// Run:  cargo run --example lc_0425_word_squares
// Test: cargo test --example lc_0425_word_squares
// Bench: cargo test --example lc_0425_word_squares bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 6.16us  (HashMap prefix lookup + backtrack)
//   AI solution: avg 4.285us (Trie + backtrack)
//   Ratio: 1.44x (AI 1.4x faster -- trie prefix lookup is more efficient)

/// My solution module — HashMap prefix lookup + backtracking.
///
/// Precompute a HashMap mapping every prefix to the list of word indices
/// that share that prefix, then backtrack to build word squares.
///
/// Time:  O(n * L + n * 26^L) worst case, but heavily pruned in practice
/// Space: O(n * L) for prefix map

#[allow(dead_code)]
mod my_solution {
    use std::collections::HashMap;

    pub struct Solution;
    impl Solution {
        pub fn word_squares(words: Vec<String>) -> Vec<Vec<String>> {
            let word_len = words[0].len();

            // Build prefix -> list of word indices
            let mut prefix_map: HashMap<String, Vec<usize>> = HashMap::new();
            for (i, w) in words.iter().enumerate() {
                for end in 0..=w.len() {
                    prefix_map.entry(w[..end].to_string()).or_default().push(i);
                }
            }

            let mut ans: Vec<Vec<String>> = Vec::new();
            let mut square: Vec<String> = Vec::with_capacity(word_len);

            for w in &words {
                square.push(w.clone());
                Self::backtrack(&words, &prefix_map, word_len, &mut square, &mut ans);
                square.pop();
            }
            ans
        }

        fn backtrack(
            words: &[String],
            prefix_map: &HashMap<String, Vec<usize>>,
            word_len: usize,
            square: &mut Vec<String>,
            ans: &mut Vec<Vec<String>>,
        ) {
            if square.len() == word_len {
                ans.push(square.clone());
                return;
            }

            // Build the required prefix for the next row:
            // column `idx` of each existing row gives us the prefix
            let idx = square.len();
            let prefix: String = square
                .iter()
                .map(|w| {
                    // Safe: all words have the same length >= word_len
                    w.as_bytes()[idx] as char
                })
                .collect();

            if let Some(candidates) = prefix_map.get(&prefix) {
                for &i in candidates {
                    square.push(words[i].clone());
                    Self::backtrack(words, prefix_map, word_len, square, ans);
                    square.pop();
                }
            }
        }
    }
}

/// AI solution module — Trie + backtracking (doocs approach).
///
/// Build a trie storing word indices at each node, then use DFS
/// to construct word squares by looking up the required prefix.
///
/// Time:  O(n * L + n * 26^L) worst case
/// Space: O(n * L) for trie
mod ai_solution {
    pub struct Solution;

    /// Trie node stores children and indices of words passing through.
    struct TrieNode {
        children: [Option<Box<TrieNode>>; 26],
        /// Indices of words that have this prefix.
        word_indices: Vec<usize>,
    }

    impl TrieNode {
        fn new() -> Self {
            TrieNode {
                children: Default::default(),
                word_indices: Vec::new(),
            }
        }

        /// Insert word index into the trie.
        fn insert(&mut self, word: &str, idx: usize) {
            let mut node = self;
            for b in word.bytes() {
                let c = (b - b'a') as usize;
                node = node.children[c].get_or_insert_with(|| Box::new(TrieNode::new()));
                node.word_indices.push(idx);
            }
        }

        /// Return all word indices matching the given prefix.
        fn search(&self, prefix: &[u8]) -> &[usize] {
            let mut node = self;
            for &b in prefix {
                let c = (b - b'a') as usize;
                match &node.children[c] {
                    Some(child) => node = child,
                    None => return &[],
                }
            }
            &node.word_indices
        }
    }

    impl Solution {
        pub fn word_squares(words: Vec<String>) -> Vec<Vec<String>> {
            let word_len = words[0].len();

            // Build trie
            let mut trie = TrieNode::new();
            for (i, w) in words.iter().enumerate() {
                trie.insert(w, i);
            }

            let mut ans: Vec<Vec<String>> = Vec::new();
            let mut square: Vec<String> = Vec::with_capacity(word_len);

            for w in &words {
                square.push(w.clone());
                Self::dfs(&words, &trie, word_len, &mut square, &mut ans);
                square.pop();
            }
            ans
        }

        fn dfs(
            words: &[String],
            trie: &TrieNode,
            word_len: usize,
            square: &mut Vec<String>,
            ans: &mut Vec<Vec<String>>,
        ) {
            if square.len() == word_len {
                ans.push(square.clone());
                return;
            }

            // Build prefix from column `idx` of current square rows
            let idx = square.len();
            let prefix: Vec<u8> = square.iter().map(|w| w.as_bytes()[idx]).collect();

            let candidates = trie.search(&prefix);
            for &i in candidates {
                square.push(words[i].clone());
                Self::dfs(words, trie, word_len, square, ans);
                square.pop();
            }
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;

        fn sorted(mut vv: Vec<Vec<String>>) -> Vec<Vec<String>> {
            vv.sort();
            vv
        }

        #[test]
        fn test_example_1() {
            let words: Vec<String> = vec!["area", "lead", "wall", "lady", "ball"]
                .into_iter()
                .map(String::from)
                .collect();
            let result = sorted(Solution::word_squares(words));
            let expected = sorted(vec![
                vec!["ball", "area", "lead", "lady"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                vec!["wall", "area", "lead", "lady"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
            ]);
            assert_eq!(result, expected);
        }
        #[test]
        fn test_example_2() {
            let words: Vec<String> = vec!["abat", "baba", "atan", "atal"]
                .into_iter()
                .map(String::from)
                .collect();
            let result = sorted(Solution::word_squares(words));
            let expected = sorted(vec![
                vec!["baba", "abat", "baba", "atal"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                vec!["baba", "abat", "baba", "atan"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
            ]);
            assert_eq!(result, expected);
        }
        #[test]
        fn test_single_char() {
            let words: Vec<String> = vec!["a", "b"].into_iter().map(String::from).collect();
            let result = sorted(Solution::word_squares(words));
            let expected = sorted(vec![
                vec!["a"].into_iter().map(String::from).collect(),
                vec!["b"].into_iter().map(String::from).collect(),
            ]);
            assert_eq!(result, expected);
        }
    };
}

#[cfg(test)]
mod tests_my {
    solution_tests!(my_solution);
}
#[cfg(test)]
mod tests_ai {
    solution_tests!(ai_solution);
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(Vec<String>) -> Vec<Vec<String>>,
{
    let iterations = 500_000;
    let words: Vec<String> = vec!["area", "lead", "wall", "lady", "ball"]
        .into_iter()
        .map(String::from)
        .collect();
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(words.clone())));
    }
    let elapsed = start.elapsed();
    println!(
        "{}: {} iterations in {:?} (avg {:?})",
        name,
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::word_squares);
    bench_solution("AI solution", ai_solution::Solution::word_squares);
}

fn main() {
    let words: Vec<String> = vec!["area", "lead", "wall", "lady", "ball"]
        .into_iter()
        .map(String::from)
        .collect();
    let result = ai_solution::Solution::word_squares(words);
    for square in &result {
        for row in square {
            println!("{}", row);
        }
        println!();
    }
}
