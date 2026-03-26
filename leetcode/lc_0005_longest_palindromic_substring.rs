// LeetCode 5: Longest Palindromic Substring [Medium]
// https://leetcode.com/problems/longest-palindromic-substring/
//
// Given a string `s`, return the longest palindromic substring in `s`.
//
// A "palindrome" reads the same forwards and backwards.
// A "substring" is a contiguous sequence of characters within the string.
//
// Examples:
//   "babad" -> "bab"  (or "aba" — both are valid)
//   "cbbd"  -> "bb"
//
// Constraints:
//   - 1 <= s.len() <= 1000
//   - s consists of only digits and English letters
//
// Run:  cargo run --example lc_0005_longest_palindromic_substring
// Test: cargo test --example lc_0005_longest_palindromic_substring
// Bench: cargo test --example lc_0005_longest_palindromic_substring bench -- --nocapture

/// My solution module — implement your solution here.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the longest palindromic substring in `s`.
        ///
        /// # Arguments
        ///
        /// * `s` - The input string to search
        ///
        /// # Returns
        ///
        /// The longest palindromic substring
        pub fn longest_palindrome(s: String) -> String {
            todo!()
        }
    }
}

/// AI solution module — will be filled in after benchmarking.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the longest palindromic substring in `s`.
        ///
        /// # Arguments
        ///
        /// * `s` - The input string to search
        ///
        /// # Returns
        ///
        /// The longest palindromic substring
        pub fn longest_palindrome(s: String) -> String {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(&str, &[&str])> = vec![
        ("babad", &["bab", "aba"]),
        ("cbbd", &["bb"]),
        ("a", &["a"]),
        ("ac", &["a", "c"]),
    ];

    println!("--- My Solution ---");
    for (input, expected) in &test_cases {
        let result = my_solution::Solution::longest_palindrome(
            input.to_string(),
        );
        println!(
            "  \"{input}\" -> \"{result}\" (expected one of {expected:?})"
        );
    }

    println!("--- AI Solution ---");
    for (input, expected) in &test_cases {
        let result = ai_solution::Solution::longest_palindrome(
            input.to_string(),
        );
        println!(
            "  \"{input}\" -> \"{result}\" (expected one of {expected:?})"
        );
    }
}

#[cfg(test)]
mod tests {
    /// Macro that generates the same set of tests for any Solution type.
    macro_rules! test_palindrome {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                /// Helper: check the result is a valid palindrome
                /// substring of `s` with the expected length.
                fn is_palindrome(s: &str) -> bool {
                    let bytes = s.as_bytes();
                    let len = bytes.len();
                    for i in 0..len / 2 {
                        if bytes[i] != bytes[len - 1 - i] {
                            return false;
                        }
                    }
                    true
                }

                /// Helper: verify the result is a palindromic substring
                /// of `input` that matches one of the `expected` values.
                fn assert_valid(input: &str, expected: &[&str]) {
                    let result = Solution::longest_palindrome(
                        input.to_string(),
                    );
                    assert!(
                        expected.contains(&result.as_str()),
                        "input=\"{input}\", got \"{result}\", \
                         expected one of {expected:?}"
                    );
                    assert!(
                        is_palindrome(&result),
                        "\"{result}\" is not a palindrome"
                    );
                    assert!(
                        input.contains(&result),
                        "\"{result}\" is not a substring of \"{input}\""
                    );
                }

                #[test]
                fn test_basic_odd() {
                    // "babad" -> "bab" or "aba"
                    assert_valid("babad", &["bab", "aba"]);
                }

                #[test]
                fn test_basic_even() {
                    // "cbbd" -> "bb"
                    assert_valid("cbbd", &["bb"]);
                }

                #[test]
                fn test_single_char() {
                    assert_valid("a", &["a"]);
                }

                #[test]
                fn test_two_different_chars() {
                    // "ac" -> "a" or "c" (each single char is a palindrome)
                    assert_valid("ac", &["a", "c"]);
                }

                #[test]
                fn test_two_same_chars() {
                    assert_valid("bb", &["bb"]);
                }

                #[test]
                fn test_entire_string_palindrome() {
                    assert_valid("racecar", &["racecar"]);
                }

                #[test]
                fn test_palindrome_at_end() {
                    // "aacabdkacaa" -> "aca" (multiple valid answers)
                    let result = Solution::longest_palindrome(
                        "aacabdkacaa".to_string(),
                    );
                    assert!(is_palindrome(&result));
                    assert!("aacabdkacaa".contains(&result));
                    assert!(result.len() >= 3); // at least "aca"
                }

                #[test]
                fn test_all_same_chars() {
                    assert_valid("aaaa", &["aaaa"]);
                }

                #[test]
                fn test_longer_input() {
                    // "forgeeksskeegfor" -> "geeksskeeg"
                    assert_valid(
                        "forgeeksskeegfor",
                        &["geeksskeeg"],
                    );
                }
            }
        };
    }

    // Run the SAME tests against both solutions.
    test_palindrome!(my_tests, crate::my_solution::Solution);
    test_palindrome!(ai_tests, crate::ai_solution::Solution);

    /// Compares the wall-clock time of both solutions over many iterations.
    ///
    /// Run with:
    ///   cargo test --example lc_0005_longest_palindromic_substring bench -- --nocapture
    ///
    /// `std::hint::black_box` prevents the compiler from optimizing away
    /// the function call, ensuring we measure real execution time.
    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        // Use a moderately sized string with a known palindrome to exercise
        // a realistic workload.
        let input = "forgeeksskeegfor".to_string();

        // --- Benchmark my_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::longest_palindrome(
                    std::hint::black_box(input.clone()),
                ),
            );
        }
        let my_duration = start.elapsed();

        // --- Benchmark ai_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::longest_palindrome(
                    std::hint::black_box(input.clone()),
                ),
            );
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
            let ratio = ai_duration.as_nanos() as f64
                / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64
                / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
