// LeetCode 516: Longest Palindromic Subsequence [Medium]
// https://leetcode.com/problems/longest-palindromic-subsequence/
//
// Given a string s, find the longest palindromic subsequence's length in s.
// A subsequence is derived by deleting some or no elements without changing
// the order of the remaining elements.
//
// Examples:
//   "bbbab" -> 4   (longest palindromic subsequence is "bbbb")
//   "cbbd"  -> 2   (longest palindromic subsequence is "bb")
//
// Constraints:
//   1 <= s.length <= 1000
//   s consists only of lowercase English letters
//
// Run:  cargo run --example lc_0516_longest_palindromic_subsequence
// Test: cargo test --example lc_0516_longest_palindromic_subsequence
// Bench: cargo test --example lc_0516_longest_palindromic_subsequence bench -- --nocapture
//
// First benchmark (100K iterations, 2026-04-08):
//   My solution: avg 1.33us
//   AI solution: avg 178ns
//   AI is 7.48x faster
//   Reason: 1D DP eliminates O(n^2) heap allocation for the 2D table and
//   achieves dramatically better cache locality with a single contiguous row.

/// 2D DP — f[i][j] = length of longest palindromic subsequence in s[i..=j].
///
/// Adapted from doocs/leetcode Python solution.
/// If s[i] == s[j], then f[i][j] = f[i+1][j-1] + 2.
/// Otherwise, f[i][j] = max(f[i+1][j], f[i][j-1]).
/// Enumerate i from large to small, j from small to large.
///
/// Time:  O(n^2)
/// Space: O(n^2)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the length of the longest palindromic subsequence.
        ///
        /// # Arguments
        /// * `s` - input string of lowercase English letters
        ///
        /// # Returns
        /// Length of the longest palindromic subsequence
        pub fn longest_palindrome_subseq(s: String) -> i32 {
            let bytes = s.as_bytes();
            let n = bytes.len();
            // f[i][j] = LPS length for s[i..=j]
            let mut f = vec![vec![0i32; n]; n];
            // Base case: single characters are palindromes of length 1
            for i in 0..n {
                f[i][i] = 1;
            }
            // Fill DP table bottom-up (i decreasing, j increasing)
            for i in (0..n).rev() {
                for j in (i + 1)..n {
                    if bytes[i] == bytes[j] {
                        f[i][j] = f[i + 1][j - 1] + 2;
                    } else {
                        f[i][j] = f[i + 1][j].max(f[i][j - 1]);
                    }
                }
            }
            f[0][n - 1]
        }
    }
}

/// Space-optimized 1D DP — only keep one row and a variable for the
/// diagonal element.
///
/// Since f[i][j] depends on f[i+1][j-1], f[i+1][j], and f[i][j-1], we
/// can compress to a single row by iterating carefully and saving the
/// previous diagonal value.
///
/// Time:  O(n^2)
/// Space: O(n)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the length of the longest palindromic subsequence.
        ///
        /// # Arguments
        /// * `s` - input string of lowercase English letters
        ///
        /// # Returns
        /// Length of the longest palindromic subsequence
        pub fn longest_palindrome_subseq(s: String) -> i32 {
            let bytes = s.as_bytes();
            let n = bytes.len();
            // dp[j] represents f[i][j] for the current row i
            let mut dp = vec![0i32; n];
            // Process rows from bottom to top
            for i in (0..n).rev() {
                dp[i] = 1; // f[i][i] = 1
                // prev holds the old dp[j-1] before we overwrite it,
                // which corresponds to f[i+1][j-1]
                let mut prev = 0;
                for j in (i + 1)..n {
                    let temp = dp[j]; // save f[i+1][j] before overwrite
                    if bytes[i] == bytes[j] {
                        dp[j] = prev + 2;
                    } else {
                        // dp[j] = max(f[i+1][j], f[i][j-1])
                        // f[i+1][j] is current dp[j] (temp), f[i][j-1] is dp[j-1]
                        dp[j] = temp.max(dp[j - 1]);
                    }
                    prev = temp; // update prev for the next iteration
                }
            }
            dp[n - 1]
        }
    }
}

fn main() {
    let s1 = "bbbab".to_string();
    let s2 = "cbbd".to_string();

    println!(
        "my  \"bbbab\": {}",
        my_solution::Solution::longest_palindrome_subseq(s1.clone())
    );
    println!(
        "ai  \"bbbab\": {}",
        ai_solution::Solution::longest_palindrome_subseq(s1)
    );
    println!(
        "my  \"cbbd\": {}",
        my_solution::Solution::longest_palindrome_subseq(s2.clone())
    );
    println!(
        "ai  \"cbbd\": {}",
        ai_solution::Solution::longest_palindrome_subseq(s2)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! lps_tests {
        ($mod:ident, $solution:path) => {
            mod $mod {
                use super::*;

                #[test]
                fn test_bbbab() {
                    assert_eq!(
                        <$solution>::longest_palindrome_subseq("bbbab".to_string()),
                        4
                    );
                }

                #[test]
                fn test_cbbd() {
                    assert_eq!(
                        <$solution>::longest_palindrome_subseq("cbbd".to_string()),
                        2
                    );
                }

                #[test]
                fn test_single_char() {
                    assert_eq!(<$solution>::longest_palindrome_subseq("a".to_string()), 1);
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(
                        <$solution>::longest_palindrome_subseq("aaaa".to_string()),
                        4
                    );
                }

                #[test]
                fn test_already_palindrome() {
                    assert_eq!(
                        <$solution>::longest_palindrome_subseq("racecar".to_string()),
                        7
                    );
                }

                #[test]
                fn test_no_repeats() {
                    // Each character is unique, so LPS = 1
                    assert_eq!(
                        <$solution>::longest_palindrome_subseq("abcde".to_string()),
                        1
                    );
                }

                #[test]
                fn test_two_chars_same() {
                    assert_eq!(<$solution>::longest_palindrome_subseq("aa".to_string()), 2);
                }

                #[test]
                fn test_two_chars_diff() {
                    assert_eq!(<$solution>::longest_palindrome_subseq("ab".to_string()), 1);
                }
            }
        };
    }

    lps_tests!(my_tests, my_solution::Solution);
    lps_tests!(ai_tests, ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let s = "abcbdabdacbdabcbda".to_string();
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(my_solution::Solution::longest_palindrome_subseq(
                std::hint::black_box(s.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(ai_solution::Solution::longest_palindrome_subseq(
                std::hint::black_box(s.clone()),
            ));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur / iterations;
        let ai_avg = ai_dur / iterations;
        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("  My solution (2D DP): {my_avg:?} avg");
        println!("  AI solution (1D DP): {ai_avg:?} avg");
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
