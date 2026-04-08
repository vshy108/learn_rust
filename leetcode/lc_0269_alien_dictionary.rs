// LeetCode 269: Alien Dictionary [Hard] (Premium)
// https://leetcode.com/problems/alien-dictionary/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 0.997us
//   AI solution avg: 1.372us
//   Ratio: My 1.38x faster
//
// Given a list of words sorted in an alien language's dictionary, derive
// the order of letters. Return "" if the order is invalid.
//
// Example 1: words = ["wrt","wrf","er","ett","rftt"] -> "wertf"
// Example 2: words = ["z","x"]                       -> "zx"
// Example 3: words = ["z","x","z"]                   -> ""
//
// Constraints:
//   - 1 <= words.length <= 100
//   - 1 <= words[i].length <= 100
//   - words[i] consists of only lowercase English letters
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0269.Alien%20Dictionary/README_EN.md
//
// Run:  cargo run --example lc_0269_alien_dictionary
// Test: cargo test --example lc_0269_alien_dictionary
// Bench: cargo test --example lc_0269_alien_dictionary bench -- --nocapture

/// My solution — BFS topological sort with adjacency matrix (from doocs).
///
/// Compare adjacent words to extract ordering constraints (edges).
/// Build a 26x26 adjacency matrix and compute in-degrees. BFS from
/// nodes with in-degree 0. If output length < unique char count, cycle.
///
/// Time: O(n * m + V + E) where V=26, E=edges
/// Space: O(V^2) for adjacency matrix
mod my_solution {
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// Return the alien alphabet order, or "" if invalid.
        pub fn alien_order(words: Vec<String>) -> String {
            let mut g = [[false; 26]; 26];
            let mut s = [false; 26]; // which characters exist
            let mut cnt = 0usize;
            let n = words.len();

            // Register characters from all words except last (done below)
            for i in 0..n - 1 {
                for &b in words[i].as_bytes() {
                    let c = (b - b'a') as usize;
                    if !s[c] {
                        cnt += 1;
                        s[c] = true;
                    }
                    if cnt == 26 {
                        break;
                    }
                }
                // Compare words[i] and words[i+1] char by char
                let w1 = words[i].as_bytes();
                let w2 = words[i + 1].as_bytes();
                let m = w1.len();
                let mut found_diff = false;
                for j in 0..m {
                    if j >= w2.len() {
                        // "abc" before "ab" -> invalid
                        return String::new();
                    }
                    let c1 = (w1[j] - b'a') as usize;
                    let c2 = (w2[j] - b'a') as usize;
                    if c1 == c2 {
                        continue;
                    }
                    if g[c2][c1] {
                        // Contradictory edge
                        return String::new();
                    }
                    g[c1][c2] = true;
                    found_diff = true;
                    break;
                }
                // If no diff found and w1 is longer, handled above
                let _ = found_diff;
            }
            // Register characters from last word
            for &b in words[n - 1].as_bytes() {
                let c = (b - b'a') as usize;
                if !s[c] {
                    cnt += 1;
                    s[c] = true;
                }
                if cnt == 26 {
                    break;
                }
            }

            // Compute in-degrees
            let mut indegree = [0i32; 26];
            for i in 0..26 {
                for j in 0..26 {
                    if i != j && s[i] && s[j] && g[i][j] {
                        indegree[j] += 1;
                    }
                }
            }

            // BFS topological sort
            let mut q = VecDeque::new();
            for i in 0..26 {
                if s[i] && indegree[i] == 0 {
                    q.push_back(i);
                }
            }
            let mut ans = String::new();
            while let Some(t) = q.pop_front() {
                ans.push((t as u8 + b'a') as char);
                for i in 0..26 {
                    if i != t && s[i] && g[t][i] {
                        indegree[i] -= 1;
                        if indegree[i] == 0 {
                            q.push_back(i);
                        }
                    }
                }
            }
            if ans.len() < cnt { String::new() } else { ans }
        }
    }
}

/// AI solution — DFS topological sort with HashMap adjacency list.
///
/// Build adjacency list from word comparisons. Use DFS with 3-color
/// marking (unvisited, in-progress, done) to detect cycles and produce
/// reverse topological order.
///
/// Time: O(n * m + V + E)
/// Space: O(V + E)
mod ai_solution {
    use std::collections::{HashMap, HashSet};

    pub struct Solution;

    impl Solution {
        /// Return the alien alphabet order, or "" if invalid.
        pub fn alien_order(words: Vec<String>) -> String {
            // Collect all unique characters
            let mut chars: HashSet<u8> = HashSet::new();
            for w in &words {
                for &b in w.as_bytes() {
                    chars.insert(b);
                }
            }

            // Build adjacency list from adjacent word comparisons
            let mut adj: HashMap<u8, Vec<u8>> = HashMap::new();
            for c in &chars {
                adj.entry(*c).or_default();
            }
            for i in 0..words.len() - 1 {
                let w1 = words[i].as_bytes();
                let w2 = words[i + 1].as_bytes();
                // Check prefix case: "abc" before "ab" is invalid
                if w1.len() > w2.len() && w1.starts_with(w2) {
                    return String::new();
                }
                for j in 0..w1.len().min(w2.len()) {
                    if w1[j] != w2[j] {
                        adj.entry(w1[j]).or_default().push(w2[j]);
                        break;
                    }
                }
            }

            // DFS topological sort with cycle detection
            // 0 = unvisited, 1 = in-progress, 2 = done
            let mut state: HashMap<u8, u8> = HashMap::new();
            let mut result: Vec<u8> = Vec::new();
            let mut has_cycle = false;

            fn dfs(
                node: u8,
                adj: &HashMap<u8, Vec<u8>>,
                state: &mut HashMap<u8, u8>,
                result: &mut Vec<u8>,
                has_cycle: &mut bool,
            ) {
                if *has_cycle {
                    return;
                }
                state.insert(node, 1); // in-progress
                if let Some(neighbors) = adj.get(&node) {
                    for &next in neighbors {
                        match state.get(&next).copied().unwrap_or(0) {
                            1 => {
                                *has_cycle = true;
                                return;
                            }
                            0 => dfs(next, adj, state, result, has_cycle),
                            _ => {} // already done
                        }
                    }
                }
                state.insert(node, 2); // done
                result.push(node);
            }

            // Sort characters for deterministic output
            let mut sorted_chars: Vec<u8> = chars.into_iter().collect();
            sorted_chars.sort_unstable();
            for &c in &sorted_chars {
                if state.get(&c).copied().unwrap_or(0) == 0 {
                    dfs(c, &adj, &mut state, &mut result, &mut has_cycle);
                    if has_cycle {
                        return String::new();
                    }
                }
            }
            result.reverse();
            String::from_utf8(result).unwrap()
        }
    }
}

fn main() {
    let words: Vec<String> = ["wrt", "wrf", "er", "ett", "rftt"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    println!("my: {}", my_solution::Solution::alien_order(words.clone()));
    println!("ai: {}", ai_solution::Solution::alien_order(words));
}

#[cfg(test)]
mod tests {
    macro_rules! words {
        ($($w:expr),*) => {
            vec![$($w.to_string()),*]
        };
    }

    macro_rules! test_alien_dict {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Sol;

                /// Verify result is a valid topological order
                fn is_valid_order(words: &[String], order: &str) -> bool {
                    if order.is_empty() {
                        return false;
                    }
                    let mut pos = [0usize; 26];
                    for (i, b) in order.bytes().enumerate() {
                        pos[(b - b'a') as usize] = i;
                    }
                    // Check every adjacent pair respects the order
                    for i in 0..words.len() - 1 {
                        let w1 = words[i].as_bytes();
                        let w2 = words[i + 1].as_bytes();
                        let mut valid = false;
                        for j in 0..w1.len().min(w2.len()) {
                            if w1[j] != w2[j] {
                                if pos[(w1[j] - b'a') as usize] >= pos[(w2[j] - b'a') as usize] {
                                    return false;
                                }
                                valid = true;
                                break;
                            }
                        }
                        if !valid && w1.len() > w2.len() {
                            return false;
                        }
                    }
                    true
                }

                #[test]
                fn test_example_1() {
                    let w = words!["wrt", "wrf", "er", "ett", "rftt"];
                    let r = Sol::alien_order(w.clone());
                    assert!(is_valid_order(&w, &r), "got: {r}");
                }

                #[test]
                fn test_example_2() {
                    let w = words!["z", "x"];
                    let r = Sol::alien_order(w.clone());
                    assert!(is_valid_order(&w, &r), "got: {r}");
                }

                #[test]
                fn test_example_3_cycle() {
                    assert_eq!(Sol::alien_order(words!["z", "x", "z"]), "");
                }

                #[test]
                fn test_prefix_invalid() {
                    // "abc" before "ab" is invalid
                    assert_eq!(Sol::alien_order(words!["abc", "ab"]), "");
                }

                #[test]
                fn test_single_word() {
                    let w = words!["abc"];
                    let r = Sol::alien_order(w);
                    // Should contain a, b, c in some order
                    assert_eq!(r.len(), 3);
                }

                #[test]
                fn test_same_words() {
                    let w = words!["a", "a"];
                    let r = Sol::alien_order(w.clone());
                    assert_eq!(r, "a");
                }
            }
        };
    }

    test_alien_dict!(my, crate::my_solution::Solution);
    test_alien_dict!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let words: Vec<String> = ["wrt", "wrf", "er", "ett", "rftt"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::alien_order(words.clone()));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::alien_order(words.clone()));
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
