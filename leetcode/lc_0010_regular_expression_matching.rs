// LeetCode 10: Regular Expression Matching [Hard]
// https://leetcode.com/problems/regular-expression-matching/
//
// Given an input string s and a pattern p, implement regular expression
// matching with support for '.' and '*' where:
//   - '.' Matches any single character.
//   - '*' Matches zero or more of the preceding element.
//
// The matching should cover the entire input string (not partial).
//
// Examples:
//   s = "aa",  p = "a"     -> false  (pattern "a" doesn't match "aa")
//   s = "aa",  p = "a*"    -> true   ("a*" means zero or more 'a's)
//   s = "ab",  p = ".*"    -> true   (".*" means zero or more of any char)
//
// Constraints:
//   - 1 <= s.length <= 20
//   - 1 <= p.length <= 20
//   - s contains only lowercase English letters
//   - p contains only lowercase English letters, '.', and '*'
//   - It is guaranteed for each appearance of '*', there will be a
//     previous valid character to match
//
// Run:  cargo run --example lc_0010_regular_expression_matching
// Test: cargo test --example lc_0010_regular_expression_matching
// Bench: cargo test --example lc_0010_regular_expression_matching bench -- --nocapture
//
// --- Benchmark (100k iterations, 2026-03-27) ---
//   My solution (2D bottom-up DP):       984.8ms avg (9.848us/iter)
//   AI solution (1D space-optimized DP): 555.9ms avg (5.558us/iter)
//   AI solution is 1.77x faster
//
// --- Rust concepts used ---
//
// vec![vec![false; n + 1]; m + 1]:
//   Creates a 2D Vec<Vec<bool>>. Equivalent to Ruby's
//   `Array.new(m + 1) { Array.new(n + 1, false) }`.
//   Inner vec is cloned (safe for Copy types like bool).
//
// as_bytes() for indexing:
//   `String`/`&str` can't be indexed by usize (UTF-8 multi-byte).
//   `as_bytes()` gives `&[u8]` with O(1) indexing. Compare with
//   byte literals: `b'*'`, `b'.'`.
//
// |= (bitwise OR assign) for bool:
//   Rust has no `||=` operator. `dp[i][j] |= val` is equivalent
//   to `dp[i][j] = dp[i][j] || val` for booleans.
//
// 1D DP space optimization:
//   When row `i` only depends on row `i-1` and current row,
//   compress 2D table to 1D. Save the "diagonal" value in a
//   `prev` variable before overwriting. O(n) vs O(m*n) space.

/// My solution module -- 2D bottom-up DP.
///
/// `dp[i][j]` = does `s[0..i]` match `p[0..j]`?
/// Three cases per cell: exact/dot match, star match, or no match.
///
/// Time:  O(m * n) -- fill every cell in the m x n table
/// Space: O(m * n) -- full 2D DP table
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Determine if string `s` matches pattern `p` with '.' and '*'.
        ///
        /// # Arguments
        ///
        /// * `s` - The input string (lowercase English letters only)
        /// * `p` - The pattern (lowercase letters, '.', '*')
        ///
        /// # Returns
        ///
        /// `true` if `s` fully matches `p`, `false` otherwise
        pub fn is_match(s: String, p: String) -> bool {
            // Convert to byte slices for O(1) indexing.
            // `String` can't be indexed by usize (UTF-8 multi-byte).
            let s = s.as_bytes();
            let p = p.as_bytes();
            let m = s.len();
            let n = p.len();

            // 2D DP table: dp[i][j] = does s[0..i] match p[0..j]?
            // Ruby: `Array.new(m + 1) { Array.new(n + 1, false) }`
            let mut dp = vec![vec![false; n + 1]; m + 1];
            // Empty string matches empty pattern.
            dp[0][0] = true;

            // Initialize row 0: empty string vs pattern.
            // Only "x*" patterns can match empty string (zero occurrences).
            // e.g., "a*b*c*" matches "" because each pair matches zero chars.
            for j in 1..=n {
                if p[j - 1] == b'*' && j >= 2 {
                    dp[0][j] = dp[0][j - 2];
                }
            }

            // Fill DP table row by row.
            for i in 1..=m {
                for j in 1..=n {
                    if p[j - 1] == b'.' || p[j - 1] == s[i - 1] {
                        // Current chars match (or '.') -> inherit diagonal.
                        dp[i][j] = dp[i - 1][j - 1];
                    } else if p[j - 1] == b'*' && j >= 2 {
                        // Zero occurrences of preceding element.
                        dp[i][j] = dp[i][j - 2];

                        // One or more occurrences: preceding element must
                        // match current char. Rust uses `|=` instead of
                        // Ruby's `||=` (no `||=` for booleans in Rust).
                        if p[j - 2] == b'.' || p[j - 2] == s[i - 1] {
                            dp[i][j] |= dp[i - 1][j];
                        }
                    } else {
                        dp[i][j] = false;
                    }
                }
            }

            dp[m][n]
        }
    }
}

/// AI solution module — space-optimized 1D DP.
///
/// Same DP logic as my_solution, but compressed from a 2D table
/// `dp[m+1][n+1]` into a single row `dp[n+1]`. This works because
/// each row `i` only depends on the previous row `i-1` and the
/// current row (for `dp[i][j-2]`). A `prev` variable tracks the
/// diagonal value `dp[i-1][j-1]`.
///
/// Time:  O(m * n) — same as 2D DP
/// Space: O(n) — single row instead of full m x n table
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Determine if string `s` matches pattern `p` with '.' and '*'.
        ///
        /// Uses a 1D DP array where `dp[j]` = "does `s[0..i]` match
        /// `p[0..j]`?". Updated in-place row by row, with `prev`
        /// saving the old `dp[j-1]` (the diagonal) before overwrite.
        ///
        /// # Arguments
        ///
        /// * `s` - The input string (lowercase English letters only)
        /// * `p` - The pattern (lowercase letters, '.', '*')
        ///
        /// # Returns
        ///
        /// `true` if `s` fully matches `p`, `false` otherwise
        pub fn is_match(s: String, p: String) -> bool {
            let s = s.as_bytes();
            let p = p.as_bytes();
            let m = s.len();
            let n = p.len();

            // Single row: dp[j] = does s[0..i] match p[0..j]?
            let mut dp = vec![false; n + 1];
            dp[0] = true;

            // Initialize for empty string (row 0).
            // "x*" can match zero chars, so dp[j] = dp[j-2] if p[j-1] == '*'.
            for j in 1..=n {
                if p[j - 1] == b'*' && j >= 2 {
                    dp[j] = dp[j - 2];
                }
            }

            // Fill row by row. `prev` holds dp[i-1][j-1] (the diagonal).
            for i in 1..=m {
                // `prev` starts as dp[i-1][0]. For i >= 1, empty pattern
                // can't match non-empty string, so dp[0] becomes false.
                let mut prev = dp[0];
                dp[0] = false;

                for j in 1..=n {
                    // Save dp[i-1][j] before we overwrite it.
                    let temp = dp[j];

                    if p[j - 1] == b'.' || p[j - 1] == s[i - 1] {
                        // Chars match: inherit diagonal dp[i-1][j-1].
                        dp[j] = prev;
                    } else if p[j - 1] == b'*' && j >= 2 {
                        // Zero occurrences: dp[i][j-2] (already updated).
                        dp[j] = dp[j - 2];
                        // One+ occurrences: dp[i-1][j] (saved as `temp`).
                        if p[j - 2] == b'.' || p[j - 2] == s[i - 1] {
                            dp[j] |= temp;
                        }
                    } else {
                        dp[j] = false;
                    }

                    // Advance diagonal: next iteration's prev = current dp[i-1][j].
                    prev = temp;
                }
            }

            dp[n]
        }
    }
}

fn main() {
    let test_cases: Vec<(&str, &str, bool)> =
        vec![("aa", "a", false), ("aa", "a*", true), ("ab", ".*", true)];

    println!("--- My Solution ---");
    for (s, p, expected) in &test_cases {
        let result = my_solution::Solution::is_match(s.to_string(), p.to_string());
        println!("  s=\"{s}\", p=\"{p}\" -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (s, p, expected) in &test_cases {
        let result = ai_solution::Solution::is_match(s.to_string(), p.to_string());
        println!("  s=\"{s}\", p=\"{p}\" -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    /// Macro that generates the same set of tests for any Solution type.
    macro_rules! test_regex {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_no_match_single_char() {
                    assert!(!Solution::is_match("aa".to_string(), "a".to_string()));
                }

                #[test]
                fn test_star_zero_or_more() {
                    assert!(Solution::is_match("aa".to_string(), "a*".to_string()));
                }

                #[test]
                fn test_dot_star() {
                    assert!(Solution::is_match("ab".to_string(), ".*".to_string()));
                }

                #[test]
                fn test_exact_match() {
                    assert!(Solution::is_match("abc".to_string(), "abc".to_string()));
                }

                #[test]
                fn test_dot_matches_any() {
                    assert!(Solution::is_match("abc".to_string(), "a.c".to_string()));
                }

                #[test]
                fn test_star_zero_occurrences() {
                    // "b*" can match zero 'b's, leaving "ac" to match "ac"
                    assert!(Solution::is_match("ac".to_string(), "ab*c".to_string()));
                }

                #[test]
                fn test_star_multiple_occurrences() {
                    assert!(Solution::is_match("aab".to_string(), "c*a*b".to_string()));
                }

                #[test]
                fn test_complex_pattern() {
                    assert!(!Solution::is_match(
                        "mississippi".to_string(),
                        "mis*is*p*.".to_string()
                    ));
                }

                #[test]
                fn test_empty_string_empty_pattern() {
                    assert!(Solution::is_match("".to_string(), "".to_string()));
                }

                #[test]
                fn test_empty_string_star_pattern() {
                    // "a*" matches zero 'a's -> empty string
                    assert!(Solution::is_match("".to_string(), "a*".to_string()));
                }

                #[test]
                fn test_empty_string_multi_star() {
                    assert!(Solution::is_match("".to_string(), "a*b*c*".to_string()));
                }

                #[test]
                fn test_single_char_match() {
                    assert!(Solution::is_match("a".to_string(), "a".to_string()));
                }

                #[test]
                fn test_single_char_no_match() {
                    assert!(!Solution::is_match("a".to_string(), "b".to_string()));
                }

                #[test]
                fn test_dot_star_long_string() {
                    assert!(Solution::is_match("abcdef".to_string(), ".*".to_string()));
                }

                #[test]
                fn test_trailing_star_pattern() {
                    assert!(Solution::is_match("aaa".to_string(), "a*a".to_string()));
                }

                #[test]
                fn test_multiple_dots() {
                    assert!(Solution::is_match("abc".to_string(), "...".to_string()));
                    assert!(!Solution::is_match("ab".to_string(), "...".to_string()));
                }
            }
        };
    }

    // Run the SAME tests against both solutions.
    test_regex!(my_tests, crate::my_solution::Solution);
    test_regex!(ai_tests, crate::ai_solution::Solution);

    /// Compares the wall-clock time of both solutions over many iterations.
    ///
    /// Run with:
    ///   cargo test --example lc_0010_regular_expression_matching bench -- --nocapture
    ///
    /// `std::hint::black_box` prevents the compiler from optimizing away
    /// the function call, ensuring we measure real execution time.
    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<(&str, &str)> = vec![
            ("aa", "a"),
            ("aa", "a*"),
            ("ab", ".*"),
            ("aab", "c*a*b"),
            ("mississippi", "mis*is*p*."),
            ("", "a*b*c*"),
            ("abcdef", ".*"),
            ("aaa", "a*a"),
        ];

        // --- Benchmark my_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            for &(s, p) in &inputs {
                std::hint::black_box(crate::my_solution::Solution::is_match(
                    std::hint::black_box(s.to_string()),
                    std::hint::black_box(p.to_string()),
                ));
            }
        }
        let my_duration = start.elapsed();

        // --- Benchmark ai_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            for &(s, p) in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::is_match(
                    std::hint::black_box(s.to_string()),
                    std::hint::black_box(p.to_string()),
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
