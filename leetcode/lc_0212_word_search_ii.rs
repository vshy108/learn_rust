// LeetCode 212: Word Search II [Hard]
// https://leetcode.com/problems/word-search-ii/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 2.844us
//   AI solution avg: 2.177us
//   Ratio: AI 1.31x faster
//
// Given an m x n board of characters and a list of words, return all words
// that can be constructed from sequentially adjacent cells (horizontally or
// vertically). The same cell may not be used more than once per word.
//
// Constraints: m,n in [1..12], board[i][j] is lowercase English letter,
//              1 <= words.length <= 3*10^4, 1 <= words[i].length <= 10,
//              words[i] consists of lowercase English letters,
//              all words[i] are unique
//
// Run:  cargo run --example lc_0212_word_search_ii
// Test: cargo test --example lc_0212_word_search_ii
// Bench: cargo test --example lc_0212_word_search_ii bench -- --nocapture

/// My solution — for each word, run independent DFS backtracking on the
/// board (like Word Search I repeated for every word).
///
/// Time: O(W * m * n * 4^L) where W = number of words, L = max word length.
/// Space: O(m * n) for the visited board per DFS call.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return all words from the list that exist on the board.
        pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
            let m = board.len();
            let n = board[0].len();
            let mut result = Vec::new();

            for word in &words {
                if Self::word_exists(&board, word, m, n) {
                    result.push(word.clone());
                }
            }
            result
        }

        /// Check if a single word exists on the board via DFS backtracking.
        fn word_exists(board: &[Vec<char>], word: &str, m: usize, n: usize) -> bool {
            let chars: Vec<char> = word.chars().collect();
            let mut visited = vec![vec![false; n]; m];

            for r in 0..m {
                for c in 0..n {
                    if board[r][c] == chars[0]
                        && Self::dfs(board, &chars, 0, r, c, m, n, &mut visited)
                    {
                        return true;
                    }
                }
            }
            false
        }

        // Clippy warns about >7 params, but DFS needs board, word chars, current
        // index, row/col, dimensions, and visited state — no clean way to reduce
        // without a context struct that would hurt readability for this pattern.
        #[allow(clippy::too_many_arguments)]
        fn dfs(
            board: &[Vec<char>],
            chars: &[char],
            idx: usize,
            r: usize,
            c: usize,
            m: usize,
            n: usize,
            visited: &mut Vec<Vec<bool>>,
        ) -> bool {
            if idx == chars.len() {
                return true;
            }
            if r >= m || c >= n || visited[r][c] || board[r][c] != chars[idx] {
                return false;
            }
            visited[r][c] = true;
            // Explore four directions
            let found = (r > 0 && Self::dfs(board, chars, idx + 1, r - 1, c, m, n, visited))
                || Self::dfs(board, chars, idx + 1, r + 1, c, m, n, visited)
                || (c > 0 && Self::dfs(board, chars, idx + 1, r, c - 1, m, n, visited))
                || Self::dfs(board, chars, idx + 1, r, c + 1, m, n, visited);
            visited[r][c] = false; // backtrack
            found
        }
    }
}

/// AI solution — build a trie from all words, then DFS the board once
/// guided by the trie structure. Prunes branches where no word prefix
/// matches, dramatically reducing redundant searches.
///
/// Time: O(m * n * 4^L) with heavy pruning from the trie.
/// Space: O(total chars in words) for trie + O(m * n) for board marking.
mod ai_solution {
    /// Trie node with 26 children and an optional completed word.
    struct TrieNode {
        children: [Option<Box<TrieNode>>; 26],
        // Store the word at leaf nodes for easy collection
        word: Option<String>,
    }

    impl TrieNode {
        fn new() -> Self {
            Self {
                children: std::array::from_fn(|_| None),
                word: None,
            }
        }
    }

    pub struct Solution;

    impl Solution {
        /// Return all words from the list that exist on the board.
        pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
            // Build trie from all words
            let mut root = TrieNode::new();
            for w in &words {
                let mut node = &mut root;
                for &b in w.as_bytes() {
                    let idx = (b - b'a') as usize;
                    node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
                }
                node.word = Some(w.clone());
            }

            let m = board.len();
            let n = board[0].len();
            let mut board = board;
            let mut result = Vec::new();

            for r in 0..m {
                for c in 0..n {
                    Self::dfs(&mut board, &mut root, r, c, m, n, &mut result);
                }
            }
            result
        }

        fn dfs(
            board: &mut Vec<Vec<char>>,
            node: &mut TrieNode,
            r: usize,
            c: usize,
            m: usize,
            n: usize,
            result: &mut Vec<String>,
        ) {
            if r >= m || c >= n {
                return;
            }
            let ch = board[r][c];
            if ch == '#' {
                return; // already visited in this path
            }
            let idx = (ch as u8 - b'a') as usize;
            let Some(child) = node.children[idx].as_mut() else {
                return; // no trie branch for this character
            };

            // If this trie node marks a complete word, collect it
            if let Some(word) = child.word.take() {
                result.push(word);
                // Don't return — there may be longer words sharing this prefix
            }

            // Mark cell as visited
            board[r][c] = '#';
            if r > 0 {
                Self::dfs(board, child, r - 1, c, m, n, result);
            }
            if r + 1 < m {
                Self::dfs(board, child, r + 1, c, m, n, result);
            }
            if c > 0 {
                Self::dfs(board, child, r, c - 1, m, n, result);
            }
            if c + 1 < n {
                Self::dfs(board, child, r, c + 1, m, n, result);
            }
            board[r][c] = ch; // backtrack

            // Prune: if child has no remaining children and no word, remove it
            if child.children.iter().all(|c| c.is_none()) && child.word.is_none() {
                node.children[idx] = None;
            }
        }
    }
}

/// Helper to sort results for order-independent comparison.
fn sorted(mut v: Vec<String>) -> Vec<String> {
    v.sort();
    v
}

fn main() {
    let board = vec![
        vec!['o', 'a', 'a', 'n'],
        vec!['e', 't', 'a', 'e'],
        vec!['i', 'h', 'k', 'r'],
        vec!['i', 'f', 'l', 'v'],
    ];
    let words: Vec<String> = vec!["oath", "pea", "eat", "rain"]
        .into_iter()
        .map(String::from)
        .collect();

    let my = sorted(my_solution::Solution::find_words(
        board.clone(),
        words.clone(),
    ));
    let ai = sorted(ai_solution::Solution::find_words(board, words));
    println!("my: {my:?}");
    println!("ai: {ai:?}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_word_search {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::super::sorted;

                fn s(v: Vec<&str>) -> Vec<String> {
                    v.into_iter().map(String::from).collect()
                }

                #[test]
                fn test_example1() {
                    let board = vec![
                        vec!['o', 'a', 'a', 'n'],
                        vec!['e', 't', 'a', 'e'],
                        vec!['i', 'h', 'k', 'r'],
                        vec!['i', 'f', 'l', 'v'],
                    ];
                    let words = s(vec!["oath", "pea", "eat", "rain"]);
                    let result = sorted(<$solution>::find_words(board, words));
                    assert_eq!(result, vec!["eat", "oath"]);
                }

                #[test]
                fn test_example2() {
                    let board = vec![vec!['a', 'b'], vec!['c', 'd']];
                    let words = s(vec!["abcb"]);
                    let result = <$solution>::find_words(board, words);
                    assert!(result.is_empty());
                }

                #[test]
                fn test_single_cell() {
                    let board = vec![vec!['a']];
                    let words = s(vec!["a", "b"]);
                    let result = sorted(<$solution>::find_words(board, words));
                    assert_eq!(result, vec!["a"]);
                }

                #[test]
                fn test_no_match() {
                    let board = vec![vec!['a', 'b'], vec!['c', 'd']];
                    let words = s(vec!["xyz", "mnop"]);
                    let result = <$solution>::find_words(board, words);
                    assert!(result.is_empty());
                }

                #[test]
                fn test_all_match() {
                    let board = vec![vec!['a', 'b'], vec!['c', 'd']];
                    let words = s(vec!["ab", "cd", "ac"]);
                    let result = sorted(<$solution>::find_words(board, words));
                    assert_eq!(result, vec!["ab", "ac", "cd"]);
                }

                #[test]
                fn test_shared_prefix() {
                    let board = vec![
                        vec!['o', 'a', 'a', 'n'],
                        vec!['e', 't', 'a', 'e'],
                        vec!['i', 'h', 'k', 'r'],
                        vec!['i', 'f', 'l', 'v'],
                    ];
                    let words = s(vec!["oath", "oat"]);
                    let result = sorted(<$solution>::find_words(board, words));
                    assert_eq!(result, vec!["oat", "oath"]);
                }
            }
        };
    }

    test_word_search!(my, crate::my_solution::Solution);
    test_word_search!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let board = vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];
        let words: Vec<String> = vec!["oath", "pea", "eat", "rain", "oat", "eta"]
            .into_iter()
            .map(String::from)
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::find_words(
                std::hint::black_box(board.clone()),
                std::hint::black_box(words.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::find_words(
                std::hint::black_box(board.clone()),
                std::hint::black_box(words.clone()),
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
        } else {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        }
    }
}
