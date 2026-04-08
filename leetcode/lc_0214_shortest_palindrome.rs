// LeetCode 214: Shortest Palindrome [Hard]
// https://leetcode.com/problems/shortest-palindrome/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.636us
//   AI solution avg: 1.800us
//   Ratio: My 2.83x faster
//
// Given a string s, find the shortest palindrome you can create by adding
// characters only in front of s. Equivalently, find the longest palindromic
// prefix of s, then prepend the reverse of the remaining suffix.
//
// Constraints: 0 <= s.length <= 5*10^4, s consists of lowercase English
//              letters only
//
// Run:  cargo run --example lc_0214_shortest_palindrome
// Test: cargo test --example lc_0214_shortest_palindrome
// Bench: cargo test --example lc_0214_shortest_palindrome bench -- --nocapture

/// My solution — try each prefix length from longest to shortest,
/// check if it is a palindrome, then prepend the reverse of the suffix.
///
/// Time: O(n^2) — for each candidate prefix, O(n) palindrome check.
/// Space: O(n) for the result string.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the shortest palindrome formed by prepending to `s`.
        pub fn shortest_palindrome(s: String) -> String {
            let bytes = s.as_bytes();
            let n = bytes.len();
            if n <= 1 {
                return s;
            }

            // Find the longest palindromic prefix by checking from the
            // full string down to length 1. Once found, reverse the suffix
            // and prepend it.
            let mut longest_pal_end = 0; // exclusive end of longest palindromic prefix
            for end in (1..=n).rev() {
                if Self::is_palindrome(&bytes[..end]) {
                    longest_pal_end = end;
                    break;
                }
            }

            let suffix = &s[longest_pal_end..];
            let reversed_suffix: String = suffix.chars().rev().collect();
            format!("{reversed_suffix}{s}")
        }

        /// Check if a byte slice is a palindrome.
        fn is_palindrome(b: &[u8]) -> bool {
            let (mut lo, mut hi) = (0, b.len() - 1);
            while lo < hi {
                if b[lo] != b[hi] {
                    return false;
                }
                lo += 1;
                hi -= 1;
            }
            true
        }
    }
}

/// AI solution — KMP failure-function approach.
///
/// Build the string `s + '#' + reverse(s)`. Compute the KMP failure
/// (partial match) table. The last value in the table gives the length
/// of the longest palindromic prefix of s, because it is the longest
/// proper prefix of `s` that is also a suffix of `reverse(s)`.
///
/// Time: O(n) — linear KMP failure function computation.
/// Space: O(n) for the concatenated string and failure table.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the shortest palindrome formed by prepending to `s`.
        pub fn shortest_palindrome(s: String) -> String {
            if s.len() <= 1 {
                return s;
            }

            let rev: String = s.chars().rev().collect();
            // Separator '#' prevents the failure function from crossing
            // the boundary between s and reverse(s).
            let combined = format!("{s}#{rev}");
            let bytes = combined.as_bytes();
            let n = bytes.len();

            // Build KMP failure table (longest proper prefix which is
            // also a suffix, for each position).
            let mut fail = vec![0usize; n];
            let mut k = 0;
            for i in 1..n {
                while k > 0 && bytes[k] != bytes[i] {
                    k = fail[k - 1];
                }
                if bytes[k] == bytes[i] {
                    k += 1;
                }
                fail[i] = k;
            }

            // The last value in fail is the length of the longest
            // palindromic prefix of s.
            let pal_len = fail[n - 1];
            let suffix = &s[pal_len..];
            let reversed_suffix: String = suffix.chars().rev().collect();
            format!("{reversed_suffix}{s}")
        }
    }
}

fn main() {
    let cases = vec!["aacecaaa", "abcd", ""];
    for &s in &cases {
        let my = my_solution::Solution::shortest_palindrome(s.to_string());
        let ai = ai_solution::Solution::shortest_palindrome(s.to_string());
        println!("{s:>12} -> my={my}, ai={ai}");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_palindrome {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(
                        <$solution>::shortest_palindrome("aacecaaa".to_string()),
                        "aaacecaaa"
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        <$solution>::shortest_palindrome("abcd".to_string()),
                        "dcbabcd"
                    );
                }

                #[test]
                fn test_empty() {
                    assert_eq!(<$solution>::shortest_palindrome("".to_string()), "");
                }

                #[test]
                fn test_single_char() {
                    assert_eq!(<$solution>::shortest_palindrome("a".to_string()), "a");
                }

                #[test]
                fn test_already_palindrome() {
                    assert_eq!(<$solution>::shortest_palindrome("aba".to_string()), "aba");
                }

                #[test]
                fn test_two_chars() {
                    assert_eq!(<$solution>::shortest_palindrome("ab".to_string()), "bab");
                }

                #[test]
                fn test_repeated_char() {
                    assert_eq!(<$solution>::shortest_palindrome("aaaa".to_string()), "aaaa");
                }
            }
        };
    }

    test_palindrome!(my, crate::my_solution::Solution);
    test_palindrome!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        // Build a string where only the first char is a palindromic prefix,
        // forcing both algorithms to do real work.
        let s: String = (0..200).map(|i| (b'a' + (i % 26)) as char).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::shortest_palindrome(
                std::hint::black_box(s.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::shortest_palindrome(
                std::hint::black_box(s.clone()),
            ));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        }
    }
}
