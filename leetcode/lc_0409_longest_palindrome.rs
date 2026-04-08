// LeetCode 409: Longest Palindrome [Easy]
// https://leetcode.com/problems/longest-palindrome/
//
// Given a string `s` which consists of lowercase or uppercase letters, return
// the length of the longest palindrome that can be built with those letters.
//
// Letters are case sensitive, for example, "Aa" is not considered a palindrome.
//
// Examples:
//   s = "abccccdd" -> 7  (e.g. "dccaccd")
//   s = "a"        -> 1
//
// Constraints:
//   - 1 <= s.length <= 2000
//   - s consists of lowercase and/or uppercase English letters only
//
// Run:  cargo run --example lc_0409_longest_palindrome
// Test: cargo test --example lc_0409_longest_palindrome
// Bench: cargo test --example lc_0409_longest_palindrome bench -- --nocapture
//
// --- Benchmark (100_000 iterations, 2026-04-07) ---
// My solution:  824.51ms (avg 8.24us)
// AI solution:  451.44ms (avg 4.51us)
// -> AI solution is 1.83x faster
// Reason: fixed-size array avoids HashMap heap allocation and hash
//   computation; branchless `has_odd as i32` saves a conditional.

/// My solution module — HashMap frequency counting.
///
/// Time:  O(n)
/// Space: O(k) where k = number of distinct characters
mod my_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Return the length of the longest palindrome buildable from `s`.
        ///
        /// # Arguments
        ///
        /// * `s` - A string of lowercase/uppercase English letters
        ///
        /// # Returns
        ///
        /// Length of the longest palindrome that can be built
        pub fn longest_palindrome(s: String) -> i32 {
            let mut count: HashMap<u8, i32> = HashMap::new();
            for &byte in s.as_bytes() {
                *count.entry(byte).or_insert(0) += 1;
            }
            let mut length = 0;
            let mut odd_found = false;
            for &freq in count.values() {
                // Take the biggest even chunk from each frequency
                length += freq / 2 * 2;
                if freq % 2 != 0 {
                    odd_found = true;
                }
            }
            // One odd-count char can sit in the center
            if odd_found {
                length += 1;
            }
            length
        }
    }
}

/// AI solution module — fixed-size array counting (no HashMap).
///
/// Uses a `[i32; 128]` array indexed by ASCII byte value, avoiding
/// HashMap overhead entirely. Only ASCII letters appear per constraints.
///
/// Time:  O(n)
/// Space: O(1) (128-element fixed array)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the length of the longest palindrome buildable from `s`.
        ///
        /// # Arguments
        ///
        /// * `s` - A string of lowercase/uppercase English letters
        ///
        /// # Returns
        ///
        /// Length of the longest palindrome that can be built
        pub fn longest_palindrome(s: String) -> i32 {
            // Fixed-size array avoids heap allocation and hash overhead.
            let mut freq = [0i32; 128];
            for &b in s.as_bytes() {
                freq[b as usize] += 1;
            }
            let mut length = 0;
            let mut has_odd = false;
            for &f in &freq {
                length += f / 2 * 2;
                if f % 2 != 0 {
                    has_odd = true;
                }
            }
            length + has_odd as i32
        }
    }
}

fn main() {
    let test_cases: Vec<(&str, i32)> =
        vec![("abccccdd", 7), ("a", 1), ("bb", 2), ("Aa", 1), ("ccc", 3)];

    println!("--- My Solution ---");
    for (s, expected) in &test_cases {
        let result = my_solution::Solution::longest_palindrome(s.to_string());
        println!("  \"{s}\" -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (s, expected) in &test_cases {
        let result = ai_solution::Solution::longest_palindrome(s.to_string());
        println!("  \"{s}\" -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_longest_palindrome {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(Solution::longest_palindrome("abccccdd".to_string()), 7);
                }

                #[test]
                fn test_single_char() {
                    assert_eq!(Solution::longest_palindrome("a".to_string()), 1);
                }

                #[test]
                fn test_two_same() {
                    assert_eq!(Solution::longest_palindrome("bb".to_string()), 2);
                }

                #[test]
                fn test_case_sensitive() {
                    assert_eq!(Solution::longest_palindrome("Aa".to_string()), 1);
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(Solution::longest_palindrome("ccc".to_string()), 3);
                }

                #[test]
                fn test_all_unique() {
                    assert_eq!(Solution::longest_palindrome("abcdef".to_string()), 1);
                }

                #[test]
                fn test_full_palindrome() {
                    assert_eq!(Solution::longest_palindrome("aabbcc".to_string()), 6);
                }
            }
        };
    }

    test_longest_palindrome!(my_tests, crate::my_solution::Solution);
    test_longest_palindrome!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<&str> = vec!["abccccdd", "a", "bb", "Aa", "ccc", "aabbcc"];

        let start = Instant::now();
        for _ in 0..iterations {
            for s in &inputs {
                std::hint::black_box(crate::my_solution::Solution::longest_palindrome(
                    std::hint::black_box(s.to_string()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for s in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::longest_palindrome(
                    std::hint::black_box(s.to_string()),
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
