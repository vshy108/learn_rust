// Benchmark: 2,000,000 iterations (2026-04-08)
// My solution:  avg 75ns   | Two-pointer index-based simulation
// AI solution:  avg 79ns   | Iterator peekable() digit parsing
// -> Tied (My 1.05x)
//
// LeetCode 408: Valid Word Abbreviation [Easy] (Premium)
// https://leetcode.com/problems/valid-word-abbreviation/
//
// A string can be abbreviated by replacing any number of non-adjacent,
// non-empty substrings with their lengths. The lengths must not have
// leading zeros.
//
// Given a string `word` and an abbreviation `abbr`, return whether the
// string matches the given abbreviation.
//
// Examples:
//   word = "internationalization", abbr = "i12iz4n" -> true
//   word = "apple", abbr = "a2e" -> false
//
// Constraints:
//   1 <= word.length <= 20
//   word consists of only lowercase English letters
//   1 <= abbr.length <= 10
//   abbr consists of lowercase English letters and digits
//   All integers in abbr fit in a 32-bit integer
//
// Run:  cargo run --example lc_0408_valid_word_abbreviation
// Test: cargo test --example lc_0408_valid_word_abbreviation
// Bench: cargo test --example lc_0408_valid_word_abbreviation bench -- --nocapture

/// My solution module — two-pointer simulation (translated from Python).
///
/// Walk through `word` with pointer `i` and `abbr` with pointer `j`.
/// If abbr[j] is a digit, accumulate a number `x` (reject leading zeros).
/// If abbr[j] is a letter, advance `i` by `x`, reset `x`, and compare chars.
///
/// Time:  O(m + n) where m = word.len(), n = abbr.len()
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Check if `abbr` is a valid abbreviation of `word`.
        ///
        /// # Arguments
        ///
        /// * `word` - The original word
        /// * `abbr` - The abbreviation to validate
        ///
        /// # Returns
        ///
        /// `true` if `abbr` is a valid abbreviation of `word`
        pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
            let wb = word.as_bytes();
            let ab = abbr.as_bytes();
            let (m, n) = (wb.len(), ab.len());
            let mut i = 0_usize; // pointer into word
            let mut j = 0_usize; // pointer into abbr
            let mut x = 0_usize; // accumulated number

            while i < m && j < n {
                if ab[j].is_ascii_digit() {
                    // Leading zero check: "0" as a standalone number is invalid
                    if ab[j] == b'0' && x == 0 {
                        return false;
                    }
                    x = x * 10 + (ab[j] - b'0') as usize;
                } else {
                    // Skip `x` characters in word, then compare letters
                    i += x;
                    x = 0;
                    if i >= m || wb[i] != ab[j] {
                        return false;
                    }
                    i += 1;
                }
                j += 1;
            }

            // After the loop, `i + x` should equal `m` (all chars consumed)
            // and `j` should equal `n` (all abbr chars consumed)
            i + x == m && j == n
        }
    }
}

/// AI solution module — iterator-based approach.
///
/// Uses `abbr.bytes()` with `peekable()` to parse digits in a group,
/// avoiding manual index tracking for the abbreviation pointer.
///
/// Time:  O(m + n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Check if `abbr` is a valid abbreviation of `word`.
        ///
        /// # Arguments
        ///
        /// * `word` - The original word
        /// * `abbr` - The abbreviation to validate
        ///
        /// # Returns
        ///
        /// `true` if `abbr` is a valid abbreviation of `word`
        pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
            let wb = word.as_bytes();
            let mut i = 0_usize;
            let mut iter = abbr.bytes().peekable();

            while let Some(&b) = iter.peek() {
                if b.is_ascii_digit() {
                    // Leading zero is invalid
                    if b == b'0' {
                        return false;
                    }
                    // Parse full number from consecutive digits
                    let mut num = 0_usize;
                    while let Some(&d) = iter.peek() {
                        if !d.is_ascii_digit() {
                            break;
                        }
                        num = num * 10 + (d - b'0') as usize;
                        iter.next();
                    }
                    i += num;
                } else {
                    // Letter: must match word[i]
                    if i >= wb.len() || wb[i] != b {
                        return false;
                    }
                    i += 1;
                    iter.next();
                }
            }

            i == wb.len()
        }
    }
}

fn main() {
    let cases = [
        ("internationalization", "i12iz4n", true),
        ("apple", "a2e", false),
    ];

    println!("--- My Solution ---");
    for (word, abbr, expected) in &cases {
        let result = my_solution::Solution::valid_word_abbreviation(
            word.to_string(),
            abbr.to_string(),
        );
        println!("  \"{word}\", \"{abbr}\" -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (word, abbr, expected) in &cases {
        let result = ai_solution::Solution::valid_word_abbreviation(
            word.to_string(),
            abbr.to_string(),
        );
        println!("  \"{word}\", \"{abbr}\" -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_abbr {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert!(Solution::valid_word_abbreviation(
                        "internationalization".to_string(),
                        "i12iz4n".to_string(),
                    ));
                }

                #[test]
                fn test_example2() {
                    assert!(!Solution::valid_word_abbreviation(
                        "apple".to_string(),
                        "a2e".to_string(),
                    ));
                }

                #[test]
                fn test_full_number() {
                    // "substitution" -> "12"
                    assert!(Solution::valid_word_abbreviation(
                        "substitution".to_string(),
                        "12".to_string(),
                    ));
                }

                #[test]
                fn test_no_abbreviation() {
                    assert!(Solution::valid_word_abbreviation(
                        "hello".to_string(),
                        "hello".to_string(),
                    ));
                }

                #[test]
                fn test_leading_zero() {
                    // "02" has a leading zero -> invalid
                    assert!(!Solution::valid_word_abbreviation(
                        "hi".to_string(),
                        "02".to_string(),
                    ));
                }

                #[test]
                fn test_single_char() {
                    assert!(Solution::valid_word_abbreviation(
                        "a".to_string(),
                        "1".to_string(),
                    ));
                }

                #[test]
                fn test_mismatch_length() {
                    assert!(!Solution::valid_word_abbreviation(
                        "abc".to_string(),
                        "4".to_string(),
                    ));
                }
            }
        };
    }

    test_abbr!(my_tests, crate::my_solution::Solution);
    test_abbr!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 2_000_000_u32;
        let word = "internationalization".to_string();
        let abbr = "i12iz4n".to_string();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::valid_word_abbreviation(
                    std::hint::black_box(word.clone()),
                    std::hint::black_box(abbr.clone()),
                ),
            );
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::valid_word_abbreviation(
                    std::hint::black_box(word.clone()),
                    std::hint::black_box(abbr.clone()),
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
