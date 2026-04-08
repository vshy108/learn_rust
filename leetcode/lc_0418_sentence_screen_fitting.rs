// Benchmark: 100,000 iterations (2026-04-08)
// My solution:  avg 26.05us | Concatenated string, direct row simulation
// AI solution:  avg 104.10us| Memoized cursor delta per sentence offset
// -> My solution is 4.00x faster
//
// LeetCode 418: Sentence Screen Fitting [Medium] (Premium)
// https://leetcode.com/problems/sentence-screen-fitting/
//
// Given a rows x cols screen and a sentence represented as a list of strings,
// return the number of times the given sentence can be fitted on the screen.
//
// The order of words must remain unchanged. A word cannot be split across lines.
// A single space separates consecutive words on a line.
//
// Examples:
//   sentence = ["hello","world"], rows = 2, cols = 8 -> 1
//   sentence = ["a","bcd","e"], rows = 3, cols = 6 -> 2
//   sentence = ["i","had","apple","pie"], rows = 4, cols = 5 -> 1
//
// Constraints:
//   1 <= sentence.length <= 100
//   1 <= sentence[i].length <= 10
//   sentence[i] consists of lowercase English letters
//   1 <= rows, cols <= 2 * 10^4
//
// Run:  cargo run --example lc_0418_sentence_screen_fitting
// Test: cargo test --example lc_0418_sentence_screen_fitting
// Bench: cargo test --example lc_0418_sentence_screen_fitting bench -- --nocapture

/// My solution module — concatenated string simulation (translated from Python).
///
/// Join all words with spaces into one repeating unit `s` (with trailing space).
/// Use a cursor `cur` that advances by `cols` each row. If we land on a space,
/// great — advance by 1. Otherwise, backtrack until we find a space boundary.
/// The answer is `cur / s.len()`.
///
/// Time:  O(rows * max_word_len) — backtracking is bounded by longest word
/// Space: O(total_sentence_length)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Count how many times the sentence fits on the screen.
        ///
        /// # Arguments
        ///
        /// * `sentence` - List of words forming the sentence
        /// * `rows` - Number of rows on the screen
        /// * `cols` - Number of columns on the screen
        ///
        /// # Returns
        ///
        /// Number of complete sentence repetitions that fit
        pub fn words_typing(sentence: Vec<String>, rows: i32, cols: i32) -> i32 {
            // Join words with spaces and add trailing space:
            // "hello world " — the space after the last word lets us
            // treat the sentence as a repeating circular buffer
            let s: String = sentence.join(" ") + " ";
            let sb = s.as_bytes();
            let m = sb.len();
            let cols = cols as usize;
            let mut cur = 0_usize;

            for _ in 0..rows {
                cur += cols;
                if sb[cur % m] == b' ' {
                    // Landed exactly on a space boundary — skip it
                    cur += 1;
                } else {
                    // Backtrack until we find a space (word boundary)
                    while cur > 0 && sb[(cur - 1) % m] != b' ' {
                        cur -= 1;
                    }
                }
            }

            (cur / m) as i32
        }
    }
}

/// AI solution module — memoized cursor advancement.
///
/// Same concatenated-string idea as my_solution, but precomputes
/// the cursor delta for each possible starting position within the
/// repeating sentence string. Since starting position is `cur % m`,
/// there are only `m` distinct states. This avoids redundant
/// backtracking across rows that start at the same sentence offset.
///
/// Time:  O(m * max_word_len + rows) — memo build + simulation
/// Space: O(m) for the memo table
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Count how many times the sentence fits on the screen.
        ///
        /// # Arguments
        ///
        /// * `sentence` - List of words forming the sentence
        /// * `rows` - Number of rows on the screen
        /// * `cols` - Number of columns on the screen
        ///
        /// # Returns
        ///
        /// Number of complete sentence repetitions that fit
        pub fn words_typing(sentence: Vec<String>, rows: i32, cols: i32) -> i32 {
            let s: String = sentence.join(" ") + " ";
            let sb = s.as_bytes();
            let m = sb.len();
            let cols = cols as usize;

            // memo[r] = effective cursor advance when a row starts at
            // sentence offset `r` (i.e., cur % m == r)
            let mut memo = vec![0_usize; m];
            for r in 0..m {
                let mut end = r + cols;
                if sb[end % m] == b' ' {
                    end += 1;
                } else {
                    // Backtrack to a word boundary
                    while end > r && sb[(end - 1) % m] != b' ' {
                        end -= 1;
                    }
                }
                memo[r] = end - r;
            }

            let mut cur = 0_usize;
            for _ in 0..rows {
                cur += memo[cur % m];
            }

            (cur / m) as i32
        }
    }
}

fn main() {
    let cases: Vec<(Vec<&str>, i32, i32, i32)> = vec![
        (vec!["hello", "world"], 2, 8, 1),
        (vec!["a", "bcd", "e"], 3, 6, 2),
        (vec!["i", "had", "apple", "pie"], 4, 5, 1),
    ];

    println!("--- My Solution ---");
    for (sentence, rows, cols, expected) in &cases {
        let result = my_solution::Solution::words_typing(
            sentence.iter().map(|s| s.to_string()).collect(),
            *rows,
            *cols,
        );
        println!("  {sentence:?}, rows={rows}, cols={cols} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (sentence, rows, cols, expected) in &cases {
        let result = ai_solution::Solution::words_typing(
            sentence.iter().map(|s| s.to_string()).collect(),
            *rows,
            *cols,
        );
        println!("  {sentence:?}, rows={rows}, cols={cols} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_fitting {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert_eq!(
                        Solution::words_typing(
                            vec!["hello".to_string(), "world".to_string(),],
                            2,
                            8,
                        ),
                        1,
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        Solution::words_typing(
                            vec!["a".to_string(), "bcd".to_string(), "e".to_string(),],
                            3,
                            6,
                        ),
                        2,
                    );
                }

                #[test]
                fn test_example3() {
                    assert_eq!(
                        Solution::words_typing(
                            vec![
                                "i".to_string(),
                                "had".to_string(),
                                "apple".to_string(),
                                "pie".to_string(),
                            ],
                            4,
                            5,
                        ),
                        1,
                    );
                }

                #[test]
                fn test_single_word() {
                    // "a " = 2 chars. cols=6 fits "a a a " = 3 per row.
                    // 3 rows * 3 = 9 sentences.
                    assert_eq!(Solution::words_typing(vec!["a".to_string()], 3, 6,), 9,);
                }

                #[test]
                fn test_exact_fit() {
                    // "a b" fits exactly in cols=3
                    assert_eq!(
                        Solution::words_typing(vec!["a".to_string(), "b".to_string()], 2, 3,),
                        2,
                    );
                }
            }
        };
    }

    test_fitting!(my_tests, crate::my_solution::Solution);
    test_fitting!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let sentence: Vec<String> = vec!["i", "had", "apple", "pie"]
            .into_iter()
            .map(String::from)
            .collect();
        let rows = 20000;
        let cols = 20000;

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::words_typing(
                std::hint::black_box(sentence.clone()),
                std::hint::black_box(rows),
                std::hint::black_box(cols),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::words_typing(
                std::hint::black_box(sentence.clone()),
                std::hint::black_box(rows),
                std::hint::black_box(cols),
            ));
        }
        let ai_duration = start.elapsed();

        let my_avg = my_duration / iterations;
        let ai_avg = ai_duration / iterations;

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution:  {my_duration:.2?} (avg {my_avg:.2?})");
        println!("AI solution:  {ai_duration:.2?} (avg {ai_avg:.2?})");

        if my_avg <= ai_avg {
            let ratio = ai_avg.as_nanos() as f64 / my_avg.as_nanos().max(1) as f64;
            println!("-> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_avg.as_nanos() as f64 / ai_avg.as_nanos().max(1) as f64;
            println!("-> AI solution is {ratio:.2}x faster");
        }
    }
}
