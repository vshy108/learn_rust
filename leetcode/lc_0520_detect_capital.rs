// LeetCode 520: Detect Capital [Easy]
// https://leetcode.com/problems/detect-capital/
//
// We define the usage of capitals in a word to be right when one of the
// following cases holds:
//   - All letters are capitals, like "USA"
//   - All letters are not capitals, like "leetcode"
//   - Only the first letter is capital, like "Google"
//
// Given a string word, return true if the usage of capitals is right.
//
// Examples:
//   "USA"  -> true
//   "FlaG" -> false
//
// Constraints:
//   1 <= word.length <= 100
//   word consists of lowercase and uppercase English letters
//
// Run:  cargo run --example lc_0520_detect_capital
// Test: cargo test --example lc_0520_detect_capital
// Bench: cargo test --example lc_0520_detect_capital bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 41ns
//   AI solution: avg 26ns
//   AI is 1.58x faster
//   Reason: Pattern-matching on the first two chars short-circuits early,
//   avoiding a full scan of all characters to count uppercase letters.

/// Count uppercase letters — valid if 0, all, or 1 (and it's the first).
///
/// Adapted from doocs/leetcode Python solution.
///
/// Time:  O(n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Check if the word uses capitals correctly.
        ///
        /// # Arguments
        /// * `word` - input word of English letters
        ///
        /// # Returns
        /// true if capital usage is correct
        pub fn detect_capital_use(word: String) -> bool {
            let cnt = word.chars().filter(|c| c.is_uppercase()).count();
            // All uppercase, all lowercase, or only first letter uppercase
            cnt == 0
                || cnt == word.len()
                || (cnt == 1 && word.as_bytes()[0].is_ascii_uppercase())
        }
    }
}

/// Pattern-matching approach using string methods.
///
/// Directly check three valid patterns: all uppercase, all lowercase,
/// or first char uppercase + rest lowercase.
///
/// Time:  O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Check if the word uses capitals correctly.
        ///
        /// # Arguments
        /// * `word` - input word of English letters
        ///
        /// # Returns
        /// true if capital usage is correct
        pub fn detect_capital_use(word: String) -> bool {
            let bytes = word.as_bytes();
            let n = bytes.len();
            if n <= 1 {
                return true;
            }
            // Check the case of the first two characters to determine the pattern
            let first_upper = bytes[0].is_ascii_uppercase();
            let second_upper = bytes[1].is_ascii_uppercase();

            if first_upper && second_upper {
                // All remaining must be uppercase: "USA..."
                bytes[2..].iter().all(|b| b.is_ascii_uppercase())
            } else if first_upper {
                // First is upper, second is lower -> rest must be lower: "Google..."
                bytes[2..].iter().all(|b| b.is_ascii_lowercase())
            } else {
                // First is lower -> all must be lower: "leetcode..."
                // (second_upper would make it invalid like "fLaG")
                !second_upper && bytes[2..].iter().all(|b| b.is_ascii_lowercase())
            }
        }
    }
}

fn main() {
    let words = vec!["USA", "FlaG", "leetcode", "Google", "g", "A"];

    for w in &words {
        let s = w.to_string();
        println!(
            "my  \"{w}\": {}",
            my_solution::Solution::detect_capital_use(s.clone())
        );
        println!(
            "ai  \"{w}\": {}",
            ai_solution::Solution::detect_capital_use(s)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! capital_tests {
        ($mod:ident, $solution:path) => {
            mod $mod {
                use super::*;

                #[test]
                fn test_all_upper() {
                    assert!(<$solution>::detect_capital_use("USA".to_string()));
                }

                #[test]
                fn test_invalid() {
                    assert!(!<$solution>::detect_capital_use("FlaG".to_string()));
                }

                #[test]
                fn test_all_lower() {
                    assert!(<$solution>::detect_capital_use("leetcode".to_string()));
                }

                #[test]
                fn test_title_case() {
                    assert!(<$solution>::detect_capital_use("Google".to_string()));
                }

                #[test]
                fn test_single_lower() {
                    assert!(<$solution>::detect_capital_use("g".to_string()));
                }

                #[test]
                fn test_single_upper() {
                    assert!(<$solution>::detect_capital_use("A".to_string()));
                }

                #[test]
                fn test_two_upper() {
                    assert!(<$solution>::detect_capital_use("AB".to_string()));
                }

                #[test]
                fn test_lower_then_upper() {
                    assert!(!<$solution>::detect_capital_use("fF".to_string()));
                }

                #[test]
                fn test_mixed_invalid() {
                    assert!(!<$solution>::detect_capital_use("mL".to_string()));
                }
            }
        };
    }

    capital_tests!(my_tests, my_solution::Solution);
    capital_tests!(ai_tests, ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let word = "GoogleChrome".to_string();
        let iterations = 500_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(my_solution::Solution::detect_capital_use(
                std::hint::black_box(word.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(ai_solution::Solution::detect_capital_use(
                std::hint::black_box(word.clone()),
            ));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur / iterations;
        let ai_avg = ai_dur / iterations;
        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("  My solution (count):   {my_avg:?} avg");
        println!("  AI solution (pattern): {ai_avg:?} avg");
        if ai_avg < my_avg {
            println!(
                "  AI is {:.2}x faster",
                my_avg.as_nanos() as f64 / ai_avg.as_nanos() as f64
            );
        } else {
            println!(
                "  My is {:.2}x faster",
                ai_avg.as_nanos() as f64 / my_avg.as_nanos() as f64
            );
        }
    }
}
