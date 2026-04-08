// Benchmark: 500,000 iterations (2026-04-08)
// My solution:  avg 231ns  | Direct iterative (i,j) symmetry check
// AI solution:  avg 291ns  | Byte-grid with row/col length pre-check
// -> My solution is 1.26x faster
//
// LeetCode 422: Valid Word Square [Easy] (Premium)
// https://leetcode.com/problems/valid-word-square/
//
// Given an array of strings `words`, return true if it forms a valid word
// square. A sequence of strings forms a valid word square if the k-th row
// and column read the same string, where 0 <= k < max(numRows, numColumns).
//
// Examples:
//   ["abcd","bnrt","crmy","dtye"] -> true
//   ["abcd","bnrt","crm","dt"]    -> true
//   ["ball","area","read","lady"] -> false (3rd row "read" vs 3rd col "lead")
//
// Constraints:
//   1 <= words.length <= 500
//   1 <= words[i].length <= 500
//   words[i] consists of only lowercase English letters
//
// Run:  cargo run --example lc_0422_valid_word_square
// Test: cargo test --example lc_0422_valid_word_square
// Bench: cargo test --example lc_0422_valid_word_square bench -- --nocapture

/// My solution module — iterative check (translated from Python).
///
/// For each position (i, j), verify words[i][j] == words[j][i].
/// Handle out-of-bounds as inequality.
///
/// Time:  O(n * max_word_len) where n = words.len()
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Check if words form a valid word square.
        ///
        /// # Arguments
        ///
        /// * `words` - Vector of strings
        ///
        /// # Returns
        ///
        /// `true` if the k-th row and k-th column match for all k
        pub fn valid_word_square(words: Vec<String>) -> bool {
            let m = words.len();
            for (i, w) in words.iter().enumerate() {
                for (j, &c) in w.as_bytes().iter().enumerate() {
                    // Check bounds: j must be a valid row index,
                    // and i must be within that row's length
                    if j >= m
                        || i >= words[j].len()
                        || c != words[j].as_bytes()[i]
                    {
                        return false;
                    }
                }
            }
            true
        }
    }
}

/// AI solution module — byte-grid transpose check.
///
/// Convert words to a Vec<&[u8]> for faster indexing, then check
/// the upper triangle only: for each (i, j) where j > i, verify
/// symmetry. Also verify that row lengths match column lengths
/// (i.e., words[i].len() == number of rows that have a char at col i).
///
/// Time:  O(n * max_word_len)
/// Space: O(n) for the byte slice references
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Check if words form a valid word square.
        ///
        /// # Arguments
        ///
        /// * `words` - Vector of strings
        ///
        /// # Returns
        ///
        /// `true` if the k-th row and k-th column match for all k
        pub fn valid_word_square(words: Vec<String>) -> bool {
            let grid: Vec<&[u8]> =
                words.iter().map(|w| w.as_bytes()).collect();
            let m = grid.len();

            for i in 0..m {
                let row_len = grid[i].len();
                // The i-th column length = number of rows whose length > i
                let col_len = grid.iter().filter(|r| r.len() > i).count();
                if row_len != col_len {
                    return false;
                }
                for j in 0..row_len {
                    if j >= m || i >= grid[j].len() {
                        return false;
                    }
                    if grid[i][j] != grid[j][i] {
                        return false;
                    }
                }
            }
            true
        }
    }
}

fn main() {
    let cases: Vec<(Vec<&str>, bool)> = vec![
        (vec!["abcd", "bnrt", "crmy", "dtye"], true),
        (vec!["abcd", "bnrt", "crm", "dt"], true),
        (vec!["ball", "area", "read", "lady"], false),
    ];

    println!("--- My Solution ---");
    for (words, expected) in &cases {
        let result = my_solution::Solution::valid_word_square(
            words.iter().map(|s| s.to_string()).collect(),
        );
        println!("  {words:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (words, expected) in &cases {
        let result = ai_solution::Solution::valid_word_square(
            words.iter().map(|s| s.to_string()).collect(),
        );
        println!("  {words:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_square {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert!(Solution::valid_word_square(
                        vec![
                            "abcd".to_string(),
                            "bnrt".to_string(),
                            "crmy".to_string(),
                            "dtye".to_string(),
                        ],
                    ));
                }

                #[test]
                fn test_example2_ragged() {
                    assert!(Solution::valid_word_square(
                        vec![
                            "abcd".to_string(),
                            "bnrt".to_string(),
                            "crm".to_string(),
                            "dt".to_string(),
                        ],
                    ));
                }

                #[test]
                fn test_example3_false() {
                    assert!(!Solution::valid_word_square(
                        vec![
                            "ball".to_string(),
                            "area".to_string(),
                            "read".to_string(),
                            "lady".to_string(),
                        ],
                    ));
                }

                #[test]
                fn test_single_word() {
                    assert!(Solution::valid_word_square(
                        vec!["a".to_string()],
                    ));
                }

                #[test]
                fn test_asymmetric_lengths() {
                    // Row 0 has 2 chars but only 1 row exists -> col 1 length mismatch
                    assert!(!Solution::valid_word_square(
                        vec!["ab".to_string()],
                    ));
                }
            }
        };
    }

    test_square!(my_tests, crate::my_solution::Solution);
    test_square!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 500_000_u32;
        let words: Vec<String> =
            vec!["abcd", "bnrt", "crmy", "dtye"]
                .into_iter()
                .map(String::from)
                .collect();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::valid_word_square(
                    std::hint::black_box(words.clone()),
                ),
            );
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::valid_word_square(
                    std::hint::black_box(words.clone()),
                ),
            );
        }
        let ai_duration = start.elapsed();

        let my_avg = my_duration / iterations;
        let ai_avg = ai_duration / iterations;

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution:  {my_duration:.2?} (avg {my_avg:.2?})");
        println!("AI solution:  {ai_duration:.2?} (avg {ai_avg:.2?})");

        if my_avg <= ai_avg {
            let ratio = ai_avg.as_nanos() as f64
                / my_avg.as_nanos().max(1) as f64;
            println!("-> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_avg.as_nanos() as f64
                / ai_avg.as_nanos().max(1) as f64;
            println!("-> AI solution is {ratio:.2}x faster");
        }
    }
}
