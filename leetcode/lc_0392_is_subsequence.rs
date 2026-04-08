// LeetCode 392: Is Subsequence [Easy]
// https://leetcode.com/problems/is-subsequence/
//
// Given two strings s and t, return true if s is a subsequence of t,
// or false otherwise.
//
// A subsequence of a string is a new string that is formed from the
// original string by deleting some (can be none) of the characters
// without disturbing the relative order of the remaining characters.
//
// Examples:
//   s = "abc", t = "ahbgdc" -> true
//   s = "axc", t = "ahbgdc" -> false
//
// Constraints:
//   - 0 <= s.length <= 100
//   - 0 <= t.length <= 10^4
//   - s and t consist only of lowercase English letters
//
// Follow up: If there are lots of incoming s's (say s1, s2, ..., sk
// where k >= 10^9), and you want to check one by one to see if t has
// its subsequence, how would you optimize?
//
// Run:  cargo run --example lc_0392_is_subsequence
// Test: cargo test --example lc_0392_is_subsequence
// Bench: cargo test --example lc_0392_is_subsequence bench -- --nocapture
//
// --- Benchmark (100_000 iterations, 2026-04-07) ---
// My solution:  73.27ms (avg 732ns)
// AI solution:  85.69ms (avg 856ns)
// -> My solution is 1.17x faster
// Reason: index-based as_bytes() approach avoids the Option<u8> wrapping
//   overhead per iteration that the .bytes() iterator incurs.

/// My solution module — two-pointer scan.
///
/// Time:  O(n) where n = t.len()
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return `true` if `s` is a subsequence of `t`.
        ///
        /// # Arguments
        ///
        /// * `s` - The potential subsequence
        /// * `t` - The source string
        ///
        /// # Returns
        ///
        /// `true` if `s` is a subsequence of `t`, `false` otherwise
        pub fn is_subsequence(s: String, t: String) -> bool {
            if s.is_empty() {
                return true;
            }
            let s = s.as_bytes();
            let t = t.as_bytes();
            let mut i = 0;
            for &byte in t {
                if s[i] == byte {
                    i += 1;
                    if i == s.len() {
                        return true;
                    }
                }
            }
            false
        }
    }
}

/// AI solution module — iterator-based approach.
///
/// Uses `t.as_bytes().iter()` with a mutable iterator over `s` bytes,
/// advancing the `s` iterator only on a match.
///
/// Time:  O(n) where n = t.len()
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return `true` if `s` is a subsequence of `t`.
        ///
        /// # Arguments
        ///
        /// * `s` - The potential subsequence
        /// * `t` - The source string
        ///
        /// # Returns
        ///
        /// `true` if `s` is a subsequence of `t`, `false` otherwise
        pub fn is_subsequence(s: String, t: String) -> bool {
            // Create a peekable iterator over s bytes; advance only on match.
            let mut s_iter = s.bytes();
            let mut current = s_iter.next();
            for byte in t.bytes() {
                if let Some(target) = current {
                    if byte == target {
                        current = s_iter.next();
                    }
                } else {
                    break; // already matched all of s
                }
            }
            // current is None when all bytes of s have been matched
            current.is_none()
        }
    }
}

fn main() {
    let test_cases: Vec<(&str, &str, bool)> =
        vec![("abc", "ahbgdc", true), ("axc", "ahbgdc", false)];

    println!("--- My Solution ---");
    for (s, t, expected) in &test_cases {
        let result = my_solution::Solution::is_subsequence(s.to_string(), t.to_string());
        println!("  s=\"{s}\", t=\"{t}\" -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (s, t, expected) in &test_cases {
        let result = ai_solution::Solution::is_subsequence(s.to_string(), t.to_string());
        println!("  s=\"{s}\", t=\"{t}\" -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_is_subsequence {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_is_subsequence() {
                    assert!(Solution::is_subsequence(
                        "abc".to_string(),
                        "ahbgdc".to_string()
                    ));
                }

                #[test]
                fn test_not_subsequence() {
                    assert!(!Solution::is_subsequence(
                        "axc".to_string(),
                        "ahbgdc".to_string()
                    ));
                }

                #[test]
                fn test_empty_s() {
                    assert!(Solution::is_subsequence(
                        "".to_string(),
                        "ahbgdc".to_string()
                    ));
                }

                #[test]
                fn test_empty_both() {
                    assert!(Solution::is_subsequence("".to_string(), "".to_string()));
                }

                #[test]
                fn test_s_longer_than_t() {
                    assert!(!Solution::is_subsequence(
                        "abcdef".to_string(),
                        "abc".to_string()
                    ));
                }

                #[test]
                fn test_equal_strings() {
                    assert!(Solution::is_subsequence(
                        "abc".to_string(),
                        "abc".to_string()
                    ));
                }

                #[test]
                fn test_single_char_present() {
                    assert!(Solution::is_subsequence("b".to_string(), "abc".to_string()));
                }
            }
        };
    }

    test_is_subsequence!(my_tests, crate::my_solution::Solution);
    test_is_subsequence!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<(&str, &str)> = vec![
            ("abc", "ahbgdc"),
            ("axc", "ahbgdc"),
            ("", "ahbgdc"),
            ("ace", "abcde"),
            ("aaaaaa", "bbaaaa"),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for &(s, t) in &inputs {
                std::hint::black_box(crate::my_solution::Solution::is_subsequence(
                    std::hint::black_box(s.to_string()),
                    std::hint::black_box(t.to_string()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &(s, t) in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::is_subsequence(
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
