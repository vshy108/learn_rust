// LeetCode 320. Generalized Abbreviation (Premium)
// https://leetcode.com/problems/generalized-abbreviation/
//
// Benchmark results (100_000 iterations, release mode, 2025-01-XX):
//   my_solution  avg: 1.765us
//   ai_solution  avg: 4.258us
//   Ratio: my_solution 2.41x faster

/// A word's generalized abbreviation replaces non-overlapping, non-adjacent
/// substrings with their lengths. Return all possible abbreviations.
///
/// Example 1:
///   Input: word = "word"
///   Output: ["4","3d","2r1","2rd","1o2","1o1d","1or1","1ord",
///            "w3","w2d","w1r1","w1rd","wo2","wo1d","wor1","word"]
///
/// Example 2:
///   Input: word = "a"
///   Output: ["1","a"]
///
/// Constraints:
///   - 1 <= word.length <= 15
///   - word consists of only lowercase English letters

/// # my_solution: Binary enumeration
/// Time: O(n * 2^n) — enumerate all 2^n bitmasks, build string in O(n)
/// Space: O(n) for each abbreviation string
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Generate all abbreviations via binary enumeration.
        ///
        /// Each bit in a mask of length n represents a character: 1 = abbreviate
        /// (count it), 0 = keep the character. Convert consecutive 1-bits into
        /// a number.
        ///
        /// # Arguments
        /// * `word` - Input word
        ///
        /// # Returns
        /// All possible generalized abbreviations
        pub fn generate_abbreviations(word: String) -> Vec<String> {
            let n = word.len();
            let bytes = word.as_bytes();
            let mut ans = Vec::with_capacity(1 << n);

            for mask in 0..(1u32 << n) {
                let mut s = String::new();
                let mut cnt = 0u32;

                for j in 0..n {
                    if (mask >> j) & 1 == 1 {
                        cnt += 1;
                    } else {
                        if cnt > 0 {
                            s.push_str(&cnt.to_string());
                            cnt = 0;
                        }
                        s.push(bytes[j] as char);
                    }
                }
                if cnt > 0 {
                    s.push_str(&cnt.to_string());
                }
                ans.push(s);
            }

            ans
        }
    }
}

/// # ai_solution: DFS/backtracking
/// Time: O(n * 2^n)
/// Space: O(n) recursion stack
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Generate all abbreviations via DFS/backtracking.
        ///
        /// At each position i, either:
        /// 1. Keep word[i] and recurse on i+1
        /// 2. Abbreviate j-i chars (for j in i+1..=n), then keep word[j] (if j<n),
        ///    and recurse on j+1
        ///
        /// # Arguments
        /// * `word` - Input word
        ///
        /// # Returns
        /// All possible generalized abbreviations
        pub fn generate_abbreviations(word: String) -> Vec<String> {
            let bytes = word.as_bytes();
            let n = bytes.len();
            dfs(bytes, n, 0)
        }
    }

    /// Recursively generate abbreviations starting from index i.
    fn dfs(word: &[u8], n: usize, i: usize) -> Vec<String> {
        if i >= n {
            return vec![String::new()];
        }

        let mut ans = Vec::new();

        // Option 1: keep word[i], recurse on i+1
        for s in dfs(word, n, i + 1) {
            let mut abbr = String::new();
            abbr.push(word[i] as char);
            abbr.push_str(&s);
            ans.push(abbr);
        }

        // Option 2: abbreviate word[i..j) for j in i+1..=n
        for j in (i + 1)..=n {
            for s in dfs(word, n, j + 1) {
                let mut abbr = String::new();
                abbr.push_str(&(j - i).to_string());
                if j < n {
                    abbr.push(word[j] as char);
                }
                abbr.push_str(&s);
                ans.push(abbr);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::Solution as AiSolution;
    use super::my_solution::Solution as MySolution;

    fn run_tests(solve: fn(String) -> Vec<String>) {
        // Example 1: "word" -> 16 abbreviations
        let mut result = solve("word".to_string());
        result.sort();
        let mut expected = vec![
            "4", "3d", "2r1", "2rd", "1o2", "1o1d", "1or1", "1ord", "w3", "w2d", "w1r1", "w1rd",
            "wo2", "wo1d", "wor1", "word",
        ];
        expected.sort();
        assert_eq!(result, expected);

        // Example 2: "a" -> ["1", "a"]
        let mut result = solve("a".to_string());
        result.sort();
        assert_eq!(result, vec!["1", "a"]);

        // Two chars
        let mut result = solve("ab".to_string());
        result.sort();
        let mut expected = vec!["2", "1b", "a1", "ab"];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_my_solution() {
        run_tests(MySolution::generate_abbreviations);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(AiSolution::generate_abbreviations);
    }
}

fn main() {
    use std::hint::black_box;
    use std::time::Instant;

    let word = "word".to_string();
    println!("Input: word = \"{}\"", word);
    let my_res = my_solution::Solution::generate_abbreviations(word.clone());
    println!("Output (my): {:?} ({} items)", my_res, my_res.len());
    let ai_res = ai_solution::Solution::generate_abbreviations(word.clone());
    println!("Output (ai): {:?} ({} items)", ai_res, ai_res.len());

    let iterations = 100_000;
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(my_solution::Solution::generate_abbreviations(black_box(
            word.clone(),
        )));
    }
    let my_dur = start.elapsed();

    let start = Instant::now();
    for _ in 0..iterations {
        black_box(ai_solution::Solution::generate_abbreviations(black_box(
            word.clone(),
        )));
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
