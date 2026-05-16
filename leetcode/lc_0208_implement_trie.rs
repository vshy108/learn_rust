// LeetCode 208: Implement Trie (Prefix Tree) [Medium]
// https://leetcode.com/problems/implement-trie-prefix-tree/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 23.503us
//   AI solution avg: 9.570us
//   Ratio: AI 2.46x faster
//
// Implement a Trie with insert, search, and startsWith methods.
// - insert(word): inserts word into the trie
// - search(word): returns true if word is in the trie
// - starts_with(prefix): returns true if any word starts with prefix
//
// Constraints: 1 <= word.length, prefix.length <= 2000
//   word and prefix consist only of lowercase English letters.
//   At most 3*10^4 calls to insert, search, starts_with.
//
// Run:  cargo run --example lc_0208_implement_trie
// Test: cargo test --example lc_0208_implement_trie
// Bench: cargo test --example lc_0208_implement_trie bench -- --nocapture

/// My solution — HashMap-based trie: each TrieNode owns a
/// HashMap<u8, Box<TrieNode>> for children and an is_end flag.
///
/// Time: O(n) per insert/search/startsWith where n is the word length.
/// Space: O(n * m) total where m is the number of inserted words.
mod my_solution {
    use std::collections::HashMap;

    /// HashMap-based trie node.
    struct TrieNode {
        children: HashMap<u8, Box<TrieNode>>,
        is_end: bool,
    }

    impl TrieNode {
        fn new() -> Self {
            TrieNode {
                children: HashMap::new(),
                is_end: false,
            }
        }
    }

    pub struct Trie {
        root: TrieNode,
    }

    impl Trie {
        /// Create a new empty Trie.
        pub fn new() -> Self {
            Trie {
                root: TrieNode::new(),
            }
        }

        /// Insert a word into the trie.
        pub fn insert(&mut self, word: String) {
            let mut node = &mut self.root;
            for b in word.bytes() {
                node = node
                    .children
                    .entry(b)
                    .or_insert_with(|| Box::new(TrieNode::new()));
            }
            node.is_end = true;
        }

        /// Return true if the word is in the trie.
        pub fn search(&self, word: String) -> bool {
            self.find_node(&word).is_some_and(|n| n.is_end)
        }

        /// Return true if any word in the trie starts with the given prefix.
        pub fn starts_with(&self, prefix: String) -> bool {
            self.find_node(&prefix).is_some()
        }

        fn find_node(&self, key: &str) -> Option<&TrieNode> {
            let mut node = &self.root;
            for b in key.bytes() {
                match node.children.get(&b) {
                    Some(child) => node = child,
                    None => return None,
                }
            }
            Some(node)
        }
    }
}

/// AI solution — array-based trie with 26 children per node.
///
/// Each node has an array of 26 optional children (a-z) and a boolean
/// marking end-of-word.
///
/// Time: O(m) per operation where m is word length.
/// Space: O(total characters * 26) for children arrays.
mod ai_solution {
    /// A trie node with 26 children slots.
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
    }

    pub struct Trie {
        root: TrieNode,
    }

    impl Trie {
        /// Create a new empty Trie.
        pub fn new() -> Self {
            Trie {
                root: TrieNode::new(),
            }
        }

        /// Insert a word into the trie.
        pub fn insert(&mut self, word: String) {
            let mut node = &mut self.root;
            for b in word.bytes() {
                let idx = (b - b'a') as usize;
                node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
            }
            node.is_end = true;
        }

        /// Return true if the word is in the trie.
        pub fn search(&self, word: String) -> bool {
            self.find_node(&word).is_some_and(|n| n.is_end)
        }

        /// Return true if any word starts with the given prefix.
        pub fn starts_with(&self, prefix: String) -> bool {
            self.find_node(&prefix).is_some()
        }

        /// Traverse the trie following the given key, returning the final node.
        fn find_node(&self, key: &str) -> Option<&TrieNode> {
            let mut node = &self.root;
            for b in key.bytes() {
                let idx = (b - b'a') as usize;
                match &node.children[idx] {
                    Some(child) => node = child,
                    None => return None,
                }
            }
            Some(node)
        }
    }
}

fn main() {
    let mut my_trie = my_solution::Trie::new();
    my_trie.insert("test".into());
    println!("my search: {}", my_trie.search("test".into()));
    println!("my prefix: {}", my_trie.starts_with("te".into()));

    let mut trie = ai_solution::Trie::new();
    trie.insert("apple".into());
    println!("search apple: {}", trie.search("apple".into()));
    println!("search app: {}", trie.search("app".into()));
    println!("starts_with app: {}", trie.starts_with("app".into()));
    trie.insert("app".into());
    println!("search app: {}", trie.search("app".into()));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_my_solution() {
        let mut trie = crate::my_solution::Trie::new();
        trie.insert("apple".into());
        assert!(trie.search("apple".into()));
        assert!(!trie.search("app".into()));
        assert!(trie.starts_with("app".into()));
        trie.insert("app".into());
        assert!(trie.search("app".into()));
    }

    #[test]
    fn test_ai_solution() {
        let mut trie = crate::ai_solution::Trie::new();
        trie.insert("apple".into());
        assert!(trie.search("apple".into()));
        assert!(!trie.search("app".into()));
        assert!(trie.starts_with("app".into()));
        trie.insert("app".into());
        assert!(trie.search("app".into()));
    }

    #[test]
    fn test_ai_not_found() {
        let mut trie = crate::ai_solution::Trie::new();
        trie.insert("hello".into());
        assert!(!trie.search("world".into()));
        assert!(!trie.starts_with("wor".into()));
    }

    #[test]
    fn test_ai_empty_trie() {
        let trie = crate::ai_solution::Trie::new();
        assert!(!trie.search("a".into()));
        assert!(!trie.starts_with("a".into()));
    }

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let words: Vec<String> = (0..20)
            .map(|i| {
                // Generate only lowercase-letter words for the array-based trie
                let suffix: String = (0..3)
                    .map(|j| (b'a' + ((i * 7 + j) % 26) as u8) as char)
                    .collect();
                format!("word{suffix}test")
            })
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut trie = crate::my_solution::Trie::new();
            for w in &words {
                trie.insert(w.clone());
            }
            for w in &words {
                std::hint::black_box(trie.search(w.clone()));
                std::hint::black_box(trie.starts_with(w[..4].to_string()));
            }
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut trie = crate::ai_solution::Trie::new();
            for w in &words {
                trie.insert(w.clone());
            }
            for w in &words {
                std::hint::black_box(trie.search(w.clone()));
                std::hint::black_box(trie.starts_with(w[..4].to_string()));
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
