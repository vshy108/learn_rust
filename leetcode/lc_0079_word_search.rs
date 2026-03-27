// LeetCode 79: Word Search [Medium]
// https://leetcode.com/problems/word-search/
//
// Given an m x n grid of characters board and a string word, return true
// if word exists in the grid.
//
// The word can be constructed from letters of sequentially adjacent cells,
// where adjacent cells are horizontally or vertically neighboring. The same
// letter cell may not be used more than once.
//
// Examples:
//   board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]],
//   word = "ABCCED" -> true
//
//   board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]],
//   word = "SEE" -> true
//
//   board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]],
//   word = "ABCB" -> false
//
// Constraints:
//   - m == board.length
//   - n = board[i].length
//   - 1 <= m, n <= 6
//   - 1 <= word.length <= 15
//   - board and word consists of only lowercase and uppercase English letters
//
// Run:  cargo run --example lc_0079_word_search
// Test: cargo test --example lc_0079_word_search
// Bench: cargo test --example lc_0079_word_search bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Determine if `word` exists in the `board` grid.
        ///
        /// # Arguments
        ///
        /// * `board` - A 2D grid of characters
        /// * `word` - The word to search for
        ///
        /// # Returns
        ///
        /// `true` if the word can be formed by adjacent cells, `false` otherwise
        pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
            todo!()
        }
    }
}

/// AI solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Determine if `word` exists in the `board` grid.
        ///
        /// # Arguments
        ///
        /// * `board` - A 2D grid of characters
        /// * `word` - The word to search for
        ///
        /// # Returns
        ///
        /// `true` if the word can be formed by adjacent cells, `false` otherwise
        pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
            todo!()
        }
    }
}

fn main() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let test_cases: Vec<(&str, bool)> = vec![("ABCCED", true), ("SEE", true), ("ABCB", false)];

    println!("--- My Solution ---");
    for (word, expected) in &test_cases {
        let result = my_solution::Solution::exist(board.clone(), word.to_string());
        println!("  word=\"{word}\" -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (word, expected) in &test_cases {
        let result = ai_solution::Solution::exist(board.clone(), word.to_string());
        println!("  word=\"{word}\" -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_exist {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                fn default_board() -> Vec<Vec<char>> {
                    vec![
                        vec!['A', 'B', 'C', 'E'],
                        vec!['S', 'F', 'C', 'S'],
                        vec!['A', 'D', 'E', 'E'],
                    ]
                }

                #[test]
                fn test_abcced() {
                    assert!(Solution::exist(default_board(), "ABCCED".to_string()));
                }

                #[test]
                fn test_see() {
                    assert!(Solution::exist(default_board(), "SEE".to_string()));
                }

                #[test]
                fn test_abcb_no_reuse() {
                    assert!(!Solution::exist(default_board(), "ABCB".to_string()));
                }

                #[test]
                fn test_single_char_found() {
                    assert!(Solution::exist(vec![vec!['A']], "A".to_string()));
                }

                #[test]
                fn test_single_char_not_found() {
                    assert!(!Solution::exist(vec![vec!['A']], "B".to_string()));
                }

                #[test]
                fn test_word_longer_than_board() {
                    assert!(!Solution::exist(vec![vec!['A']], "AB".to_string()));
                }

                #[test]
                fn test_full_board_path() {
                    // Snake through entire 2x2 board
                    assert!(Solution::exist(
                        vec![vec!['A', 'B'], vec!['D', 'C']],
                        "ABCD".to_string()
                    ));
                }
            }
        };
    }

    test_exist!(my_tests, crate::my_solution::Solution);
    test_exist!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let words: Vec<&str> = vec!["ABCCED", "SEE", "ABCB", "ADEE"];

        let start = Instant::now();
        for _ in 0..iterations {
            for &w in &words {
                std::hint::black_box(crate::my_solution::Solution::exist(
                    std::hint::black_box(board.clone()),
                    std::hint::black_box(w.to_string()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &w in &words {
                std::hint::black_box(crate::ai_solution::Solution::exist(
                    std::hint::black_box(board.clone()),
                    std::hint::black_box(w.to_string()),
                ));
            }
        }
        let ai_duration = start.elapsed();

        println!("\n=== Benchmark ({iterations} iterations) ===");
        println!(
            "  My solution: {:?} (avg {:?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "  AI solution: {:?} (avg {:?})",
            ai_duration,
            ai_duration / iterations
        );

        if my_duration < ai_duration {
            let ratio = ai_duration.as_nanos() as f64 / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64 / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
