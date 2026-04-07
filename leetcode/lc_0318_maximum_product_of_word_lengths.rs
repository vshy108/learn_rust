// LeetCode 318. Maximum Product of Word Lengths
// https://leetcode.com/problems/maximum-product-of-word-lengths/
//
// Benchmark results (100_000 iterations, release mode, 2025-01-XX):
//   my_solution  avg: 0.425us
//   ai_solution  avg: 0.739us
//   Ratio: my_solution 1.74x faster

/// Given string array words, return max length(word[i]) * length(word[j])
/// where the two words share no common letters. Return 0 if no such pair.
///
/// Example 1:
///   Input: words = ["abcw","baz","foo","bar","xtfn","abcdef"]
///   Output: 16  ("abcw" and "xtfn")
///
/// Example 2:
///   Input: words = ["a","ab","abc","d","cd","bcd","abcd"]
///   Output: 4  ("ab" and "cd")
///
/// Example 3:
///   Input: words = ["a","aa","aaa","aaaa"]
///   Output: 0
///
/// Constraints:
///   - 2 <= words.length <= 1000
///   - 1 <= words[i].length <= 1000
///   - words[i] consists only of lowercase English letters

/// # my_solution: Bitmask + all pairs
/// Time: O(n^2 + L) where L = total chars, n = words count
/// Space: O(n) for bitmask array
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find max product of lengths of two words with no common letters.
        ///
        /// Represent each word as a 26-bit mask. Two words share no letters
        /// iff their masks AND to 0. Check all pairs.
        ///
        /// # Arguments
        /// * `words` - Array of lowercase strings
        ///
        /// # Returns
        /// Maximum product of lengths, or 0
        pub fn max_product(words: Vec<String>) -> i32 {
            let n = words.len();
            let masks: Vec<u32> = words
                .iter()
                .map(|w| {
                    w.bytes().fold(0u32, |m, b| m | (1 << (b - b'a')))
                })
                .collect();

            let mut ans = 0i32;
            for i in 0..n {
                for j in 0..i {
                    if masks[i] & masks[j] == 0 {
                        let prod = words[i].len() as i32 * words[j].len() as i32;
                        ans = ans.max(prod);
                    }
                }
            }
            ans
        }
    }
}

/// # ai_solution: Bitmask with HashMap dedup
/// Time: O(n^2 + L) worst case, but fewer comparisons when masks collide
/// Space: O(n) for the HashMap
mod ai_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Find max product using bitmask dedup via HashMap.
        ///
        /// Multiple words can map to the same bitmask (e.g. "ab" and "ba").
        /// Store only the max length for each unique mask. Then compare
        /// distinct masks — fewer comparisons when many words share masks.
        ///
        /// # Arguments
        /// * `words` - Array of lowercase strings
        ///
        /// # Returns
        /// Maximum product of lengths, or 0
        pub fn max_product(words: Vec<String>) -> i32 {
            let mut mask_map: HashMap<u32, usize> = HashMap::new();

            for w in &words {
                let m = w.bytes().fold(0u32, |acc, b| acc | (1 << (b - b'a')));
                let entry = mask_map.entry(m).or_insert(0);
                *entry = (*entry).max(w.len());
            }

            let masks: Vec<(u32, usize)> = mask_map.into_iter().collect();
            let mut ans = 0usize;

            for i in 0..masks.len() {
                for j in 0..i {
                    if masks[i].0 & masks[j].0 == 0 {
                        ans = ans.max(masks[i].1 * masks[j].1);
                    }
                }
            }

            ans as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::Solution as AiSolution;
    use super::my_solution::Solution as MySolution;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    fn run_tests(solve: fn(Vec<String>) -> i32) {
        // Example 1
        assert_eq!(
            solve(s(&["abcw", "baz", "foo", "bar", "xtfn", "abcdef"])),
            16
        );

        // Example 2
        assert_eq!(
            solve(s(&["a", "ab", "abc", "d", "cd", "bcd", "abcd"])),
            4
        );

        // Example 3
        assert_eq!(solve(s(&["a", "aa", "aaa", "aaaa"])), 0);

        // Two disjoint words
        assert_eq!(solve(s(&["abc", "xyz"])), 9);

        // Minimum case: all overlap
        assert_eq!(solve(s(&["ab", "ba"])), 0);

        // Single char words
        assert_eq!(solve(s(&["a", "b"])), 1);
    }

    #[test]
    fn test_my_solution() {
        run_tests(MySolution::max_product);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(AiSolution::max_product);
    }
}

fn main() {
    use std::hint::black_box;
    use std::time::Instant;

    let words: Vec<String> = ["abcw", "baz", "foo", "bar", "xtfn", "abcdef"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    println!("Input: words = {:?}", words);
    println!(
        "Output (my): {}",
        my_solution::Solution::max_product(words.clone())
    );
    println!(
        "Output (ai): {}",
        ai_solution::Solution::max_product(words.clone())
    );

    let iterations = 100_000;
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(my_solution::Solution::max_product(black_box(words.clone())));
    }
    let my_dur = start.elapsed();

    let start = Instant::now();
    for _ in 0..iterations {
        black_box(ai_solution::Solution::max_product(black_box(words.clone())));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    println!("\nBenchmark ({iterations} iterations, release mode):");
    println!("  my_solution  avg: {my_avg:.3}us");
    println!("  ai_solution  avg: {ai_avg:.3}us");
    if my_avg < ai_avg {
        println!("  my_solution is {:.2}x faster", ai_avg / my_avg);
    } else {
        println!("  ai_solution is {:.2}x faster", my_avg / ai_avg);
    }
}
