// LeetCode 472: Concatenated Words [Hard]
// https://leetcode.com/problems/concatenated-words/
//
// Given an array of strings words (without duplicates), return all the
// concatenated words. A concatenated word is comprised entirely of at
// least two shorter words in the given array.
//
// Examples:
//   ["cat","cats","catsdogcats","dog","dogcatsdog","hippopotamuses","rat","ratcatdogcat"]
//   -> ["catsdogcats","dogcatsdog","ratcatdogcat"]
//
// Constraints:
//   1 <= words.length <= 10^4
//   1 <= words[i].length <= 30
//   words[i] consists of only lowercase English letters
//   All strings are unique
//
// Run:  cargo run --example lc_0472_concatenated_words
// Test: cargo test --example lc_0472_concatenated_words
// Bench: cargo test --example lc_0472_concatenated_words bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 3.383us (HashSet + DP word break)
//   AI solution: avg 3.475us (Trie + DFS)
//   Ratio: ~1.0x

/// My solution module — HashSet + DFS word break.
///
/// Sort words by length, then for each word check if it can be segmented
/// into words already in the set (word break problem). If not, add it.
///
/// Time:  O(n * log(n) + n * L^2) where L = max word length
/// Space: O(n * L)

#[allow(dead_code)]
mod my_solution {
    use std::collections::HashSet;

    pub struct Solution;
    impl Solution {
        pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
            let mut words = words;
            // Sort by length so shorter words are processed first
            words.sort_by_key(|w| w.len());

            let mut word_set: HashSet<String> = HashSet::new();
            let mut result = Vec::new();

            for word in &words {
                if word.is_empty() {
                    continue;
                }
                if Self::can_form(word, &word_set) {
                    result.push(word.clone());
                }
                word_set.insert(word.clone());
            }
            result
        }

        /// Check if `word` can be formed by concatenating words in `dict`.
        fn can_form(word: &str, dict: &HashSet<String>) -> bool {
            if dict.is_empty() {
                return false;
            }
            let n = word.len();
            // dp[i] = true if word[0..i] can be segmented
            let mut dp = vec![false; n + 1];
            dp[0] = true;

            for i in 1..=n {
                for j in 0..i {
                    if dp[j] && dict.contains(&word[j..i]) {
                        dp[i] = true;
                        break;
                    }
                }
            }
            dp[n]
        }
    }
}

/// AI solution module — Trie + DFS (doocs approach).
///
/// Build a trie from all words sorted by length. For each word,
/// DFS through the trie to see if it can be split into existing words.
///
/// Time:  O(n * log(n) + n * L^2)
/// Space: O(n * L)
mod ai_solution {
    pub struct Solution;

    /// Trie node for efficient prefix lookup.
    struct TrieNode {
        children: [Option<Box<TrieNode>>; 26],
        is_end: bool,
    }

    impl TrieNode {
        fn new() -> Self {
            TrieNode {
                children: Default::default(),
                is_end: false,
            }
        }

        fn insert(&mut self, word: &str) {
            let mut node = self;
            for b in word.bytes() {
                let idx = (b - b'a') as usize;
                node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
            }
            node.is_end = true;
        }
    }

    impl Solution {
        pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
            let mut words = words;
            words.sort_by_key(|w| w.len());

            let mut trie = TrieNode::new();
            let mut result = Vec::new();

            for word in &words {
                if word.is_empty() {
                    continue;
                }
                if Self::dfs(word.as_bytes(), &trie) {
                    result.push(word.clone());
                }
                trie.insert(word);
            }
            result
        }

        /// DFS to check if the word can be formed by concatenation.
        fn dfs(word: &[u8], trie: &TrieNode) -> bool {
            if word.is_empty() {
                return true;
            }

            let mut node = trie;
            for (i, &b) in word.iter().enumerate() {
                let idx = (b - b'a') as usize;
                match &node.children[idx] {
                    Some(child) => {
                        node = child;
                        if node.is_end && Self::dfs(&word[i + 1..], trie) {
                            return true;
                        }
                    }
                    None => return false,
                }
            }
            false
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_example_1() {
            let words: Vec<String> = vec![
                "cat",
                "cats",
                "catsdogcats",
                "dog",
                "dogcatsdog",
                "hippopotamuses",
                "rat",
                "ratcatdogcat",
            ]
            .into_iter()
            .map(String::from)
            .collect();
            let mut result = Solution::find_all_concatenated_words_in_a_dict(words);
            result.sort();
            assert_eq!(result, vec!["catsdogcats", "dogcatsdog", "ratcatdogcat"]);
        }
        #[test]
        fn test_example_2() {
            let words: Vec<String> = vec!["cat", "dog", "catdog"]
                .into_iter()
                .map(String::from)
                .collect();
            assert_eq!(
                Solution::find_all_concatenated_words_in_a_dict(words),
                vec!["catdog"]
            );
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
    F: Fn(Vec<String>) -> Vec<String>,
{
    let iterations = 500_000;
    let words: Vec<String> = vec![
        "cat",
        "cats",
        "catsdogcats",
        "dog",
        "dogcatsdog",
        "hippopotamuses",
        "rat",
        "ratcatdogcat",
    ]
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
    bench_solution(
        "My solution",
        my_solution::Solution::find_all_concatenated_words_in_a_dict,
    );
    bench_solution(
        "AI solution",
        ai_solution::Solution::find_all_concatenated_words_in_a_dict,
    );
}

fn main() {
    let words: Vec<String> = vec!["cat", "cats", "catsdogcats", "dog", "dogcatsdog"]
        .into_iter()
        .map(String::from)
        .collect();
    println!(
        "{:?}",
        ai_solution::Solution::find_all_concatenated_words_in_a_dict(words)
    );
}
