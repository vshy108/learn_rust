// LeetCode 76: Minimum Window Substring [Hard]
// https://leetcode.com/problems/minimum-window-substring/
//
// Given two strings s and t of lengths m and n respectively, return the
// minimum window substring of s such that every character in t (including
// duplicates) is included in the window. If there is no such substring,
// return the empty string "".
//
// The testcases will be generated such that the answer is unique.
//
// Examples:
//   s = "ADOBECODEBANC", t = "ABC" -> "BANC"
//   s = "a", t = "a"               -> "a"
//   s = "a", t = "aa"              -> ""
//
// Constraints:
//   - m == s.length, n == t.length
//   - 1 <= m, n <= 10^5
//   - s and t consist of uppercase and lowercase English letters
//
// Follow up: Could you find an algorithm that runs in O(m + n) time?
//
// Run:  cargo run --example lc_0076_minimum_window_substring
// Test: cargo test --example lc_0076_minimum_window_substring
// Bench: cargo test --example lc_0076_minimum_window_substring bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the minimum window substring of `s` containing all chars of `t`.
        ///
        /// # Arguments
        ///
        /// * `s` - The source string to search in
        /// * `t` - The target string whose characters must be covered
        ///
        /// # Returns
        ///
        /// The minimum window substring, or "" if no valid window exists
        pub fn min_window(s: String, t: String) -> String {
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
        /// Find the minimum window substring of `s` containing all chars of `t`.
        ///
        /// # Arguments
        ///
        /// * `s` - The source string to search in
        /// * `t` - The target string whose characters must be covered
        ///
        /// # Returns
        ///
        /// The minimum window substring, or "" if no valid window exists
        pub fn min_window(s: String, t: String) -> String {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(&str, &str, &str)> = vec![
        ("ADOBECODEBANC", "ABC", "BANC"),
        ("a", "a", "a"),
        ("a", "aa", ""),
    ];

    println!("--- My Solution ---");
    for (s, t, expected) in &test_cases {
        let result = my_solution::Solution::min_window(s.to_string(), t.to_string());
        println!("  s=\"{s}\", t=\"{t}\" -> \"{result}\" (expected \"{expected}\")");
    }

    println!("--- AI Solution ---");
    for (s, t, expected) in &test_cases {
        let result = ai_solution::Solution::min_window(s.to_string(), t.to_string());
        println!("  s=\"{s}\", t=\"{t}\" -> \"{result}\" (expected \"{expected}\")");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_min_window {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(
                        Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
                        "BANC"
                    );
                }

                #[test]
                fn test_exact_match() {
                    assert_eq!(Solution::min_window("a".to_string(), "a".to_string()), "a");
                }

                #[test]
                fn test_no_valid_window() {
                    assert_eq!(Solution::min_window("a".to_string(), "aa".to_string()), "");
                }

                #[test]
                fn test_t_equals_s() {
                    assert_eq!(
                        Solution::min_window("abc".to_string(), "abc".to_string()),
                        "abc"
                    );
                }

                #[test]
                fn test_single_char_repeated() {
                    assert_eq!(
                        Solution::min_window("aa".to_string(), "aa".to_string()),
                        "aa"
                    );
                }

                #[test]
                fn test_window_at_end() {
                    assert_eq!(
                        Solution::min_window("bba".to_string(), "ab".to_string()),
                        "ba"
                    );
                }

                #[test]
                fn test_no_overlap() {
                    assert_eq!(Solution::min_window("xyz".to_string(), "a".to_string()), "");
                }
            }
        };
    }

    test_min_window!(my_tests, crate::my_solution::Solution);
    test_min_window!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<(&str, &str)> = vec![
            ("ADOBECODEBANC", "ABC"),
            ("a", "a"),
            ("a", "aa"),
            ("abcdefghijklmnop", "fmp"),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for &(s, t) in &inputs {
                std::hint::black_box(crate::my_solution::Solution::min_window(
                    std::hint::black_box(s.to_string()),
                    std::hint::black_box(t.to_string()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &(s, t) in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::min_window(
                    std::hint::black_box(s.to_string()),
                    std::hint::black_box(t.to_string()),
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
