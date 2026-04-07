// LeetCode 211: Design Add and Search Words Data Structure [Medium]
// https://leetcode.com/problems/design-add-and-search-words-data-structure/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 2.078us
//   AI solution avg: 7.147us
//   Ratio: My 3.44x faster
//
// Design a data structure WordDictionary that supports:
// - addWord(word) — adds a word to the data structure
// - search(word) — returns true if any previously added word matches,
//   where '.' can match any single letter
//
// Constraints: 1 <= word.length <= 25, word in lowercase a-z and '.',
//              at most 10^4 calls to addWord/search
//
// Run:  cargo run --example lc_0211_design_add_and_search_words
// Test: cargo test --example lc_0211_design_add_and_search_words
// Bench: cargo test --example lc_0211_design_add_and_search_words bench -- --nocapture

/// My solution — store words in a Vec grouped by length; brute-force
/// character comparison with '.' wildcard.
///
/// Time: addWord O(1), search O(n * m) where n = words of same length, m = word length.
/// Space: O(total characters stored).
mod my_solution {
    use std::collections::HashMap;

    /// Words grouped by length for faster filtering.
    pub struct WordDictionary {
        words_by_len: HashMap<usize, Vec<String>>,
    }

    impl WordDictionary {
        pub fn new() -> Self {
            Self {
                words_by_len: HashMap::new(),
            }
        }

        /// Add a word to the dictionary.
        pub fn add_word(&mut self, word: String) {
            self.words_by_len
                .entry(word.len())
                .or_default()
                .push(word);
        }

        /// Search with '.' wildcard support by brute-force comparison.
        pub fn search(&self, word: String) -> bool {
            let Some(candidates) = self.words_by_len.get(&word.len()) else {
                return false;
            };
            let pattern: Vec<u8> = word.bytes().collect();
            candidates.iter().any(|w| {
                w.bytes()
                    .zip(pattern.iter())
                    .all(|(c, &p)| p == b'.' || c == p)
            })
        }
    }
}

/// AI solution — trie with DFS for '.' wildcard matching.
///
/// Each node has 26 children (one per letter). When searching and
/// encountering '.', recursively try all non-null children.
///
/// Time: addWord O(m), search O(26^d * m) worst case with many dots,
///       but typically much faster.
/// Space: O(total characters * 26) for trie nodes.
mod ai_solution {
    /// A trie node with 26 optional children and an end-of-word flag.
    struct TrieNode {
        children: [Option<Box<TrieNode>>; 26],
        is_end: bool,
    }

    impl TrieNode {
        fn new() -> Self {
            Self {
                children: std::array::from_fn(|_| None),
                is_end: false,
            }
        }
    }

    pub struct WordDictionary {
        root: TrieNode,
    }

    impl WordDictionary {
        pub fn new() -> Self {
            Self {
                root: TrieNode::new(),
            }
        }

        /// Insert a word into the trie.
        pub fn add_word(&mut self, word: String) {
            let mut node = &mut self.root;
            for b in word.bytes() {
                let idx = (b - b'a') as usize;
                node = node.children[idx]
                    .get_or_insert_with(|| Box::new(TrieNode::new()));
            }
            node.is_end = true;
        }

        /// Search the trie with '.' wildcard using DFS.
        pub fn search(&self, word: String) -> bool {
            let bytes: Vec<u8> = word.bytes().collect();
            Self::dfs(&self.root, &bytes)
        }

        /// Recursive DFS: at each position, if '.', try all children.
        fn dfs(node: &TrieNode, pattern: &[u8]) -> bool {
            if pattern.is_empty() {
                return node.is_end;
            }
            let ch = pattern[0];
            if ch == b'.' {
                // Wildcard: try every existing child
                node.children.iter().any(|child| {
                    child
                        .as_ref()
                        .is_some_and(|c| Self::dfs(c, &pattern[1..]))
                })
            } else {
                let idx = (ch - b'a') as usize;
                node.children[idx]
                    .as_ref()
                    .is_some_and(|c| Self::dfs(c, &pattern[1..]))
            }
        }
    }
}

fn main() {
    let mut my = my_solution::WordDictionary::new();
    my.add_word("bad".to_string());
    my.add_word("dad".to_string());
    println!("my search('.ad') = {}", my.search(".ad".to_string()));

    let mut ai = ai_solution::WordDictionary::new();
    ai.add_word("bad".to_string());
    ai.add_word("dad".to_string());
    println!("ai search('.ad') = {}", ai.search(".ad".to_string()));
}

#[cfg(test)]
mod tests {
    macro_rules! test_word_dict {
        ($mod_name:ident, $dict:path) => {
            mod $mod_name {
                #[test]
                fn test_example() {
                    let mut d = <$dict>::new();
                    d.add_word("bad".to_string());
                    d.add_word("dad".to_string());
                    d.add_word("mad".to_string());
                    assert!(!d.search("pad".to_string()));
                    assert!(d.search("bad".to_string()));
                    assert!(d.search(".ad".to_string()));
                    assert!(d.search("b..".to_string()));
                }

                #[test]
                fn test_no_match() {
                    let mut d = <$dict>::new();
                    d.add_word("hello".to_string());
                    assert!(!d.search("world".to_string()));
                    assert!(!d.search("hell".to_string()));
                }

                #[test]
                fn test_all_dots() {
                    let mut d = <$dict>::new();
                    d.add_word("abc".to_string());
                    assert!(d.search("...".to_string()));
                    assert!(!d.search("..".to_string()));
                }

                #[test]
                fn test_single_char() {
                    let mut d = <$dict>::new();
                    d.add_word("a".to_string());
                    assert!(d.search(".".to_string()));
                    assert!(d.search("a".to_string()));
                    assert!(!d.search("b".to_string()));
                }

                #[test]
                fn test_duplicate_add() {
                    let mut d = <$dict>::new();
                    d.add_word("word".to_string());
                    d.add_word("word".to_string());
                    assert!(d.search("word".to_string()));
                    assert!(d.search("w..d".to_string()));
                }
            }
        };
    }

    test_word_dict!(my, crate::my_solution::WordDictionary);
    test_word_dict!(ai, crate::ai_solution::WordDictionary);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let words = vec![
            "apple", "banana", "cherry", "date", "elder", "fig", "grape",
            "hello", "world", "rust", "code", "test", "bench", "trie",
        ];
        let queries = vec![
            "apple", "b.nana", "ch...y", "....", "hello", ".ust", "co.e",
        ];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut d = crate::my_solution::WordDictionary::new();
            for &w in &words {
                d.add_word(std::hint::black_box(w.to_string()));
            }
            for &q in &queries {
                std::hint::black_box(d.search(q.to_string()));
            }
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut d = crate::ai_solution::WordDictionary::new();
            for &w in &words {
                d.add_word(std::hint::black_box(w.to_string()));
            }
            for &q in &queries {
                std::hint::black_box(d.search(q.to_string()));
            }
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        }
    }
}
