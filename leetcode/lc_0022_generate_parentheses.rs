// LeetCode 22: Generate Parentheses [Medium]
// https://leetcode.com/problems/generate-parentheses/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 275.104µs
//   AI solution avg: 133.320µs
//   Ratio: AI solution is 2.06x faster
//
// Given n pairs of parentheses, write a function to generate all combinations
// of well-formed parentheses.
//
// Example 1: n = 3 -> ["((()))","(()())","(())()","()(())","()()()"]
// Example 2: n = 1 -> ["()"]
//
// Constraints:
//   1 <= n <= 8
//
// Run:  cargo run --example lc_0022_generate_parentheses
// Test: cargo test --example lc_0022_generate_parentheses
// Bench: cargo test --example lc_0022_generate_parentheses bench -- --nocapture

/// My solution — backtracking.
///
/// Build strings character-by-character, tracking how many open/close
/// parens have been placed. Add '(' if open_count < n, add ')' if
/// close_count < open_count.
///
/// Time: O(4^n / sqrt(n)) — the n-th Catalan number of valid strings.
/// Space: O(n) — recursion depth is 2n (the length of each result).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Generate all valid parentheses combinations of `n` pairs.
        ///
        /// # Arguments
        /// * `n` - number of parenthesis pairs (1..=8)
        ///
        /// # Returns
        /// A vector of all valid combinations.
        pub fn generate_parenthesis(n: i32) -> Vec<String> {
            let n = n as usize;
            let mut result = Vec::new();
            let mut current = String::with_capacity(n * 2);
            // Recursive backtracking helper.
            Self::backtrack(&mut result, &mut current, 0, 0, n);
            result
        }

        /// Recursively build valid parentheses strings.
        ///
        /// # Arguments
        /// * `result` - accumulator for completed strings
        /// * `current` - string being built
        /// * `open` - number of '(' placed so far
        /// * `close` - number of ')' placed so far
        /// * `n` - target pair count
        fn backtrack(
            result: &mut Vec<String>,
            current: &mut String,
            open: usize,
            close: usize,
            n: usize,
        ) {
            // Base case: string is complete when length reaches 2*n.
            if current.len() == n * 2 {
                result.push(current.clone());
                return;
            }

            // Can add '(' if we haven't used all n opens.
            if open < n {
                current.push('(');
                Self::backtrack(result, current, open + 1, close, n);
                current.pop(); // undo for backtracking
            }

            // Can add ')' only if it won't exceed the number of opens.
            if close < open {
                current.push(')');
                Self::backtrack(result, current, open, close + 1, n);
                current.pop(); // undo for backtracking
            }
        }
    }
}

/// AI-optimized solution — byte-level backtracking.
///
/// Same logic but operates on a fixed-size `Vec<u8>` buffer instead of
/// `String`, avoiding repeated push/pop and UTF-8 overhead. Converts to
/// `String` only once per complete result.
///
/// Time: O(4^n / sqrt(n)) — same Catalan-number bound.
/// Space: O(n) — fixed buffer of 2n bytes plus recursion stack.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Generate all valid parentheses combinations of `n` pairs.
        ///
        /// # Arguments
        /// * `n` - number of parenthesis pairs (1..=8)
        ///
        /// # Returns
        /// A vector of all valid combinations.
        pub fn generate_parenthesis(n: i32) -> Vec<String> {
            let n = n as usize;
            let mut result = Vec::new();
            // Pre-allocate buffer; we write into it by index.
            let mut buf = vec![0u8; n * 2];
            Self::backtrack(&mut result, &mut buf, 0, 0, 0, n);
            result
        }

        /// Recursively build valid parentheses using a byte buffer.
        ///
        /// # Arguments
        /// * `result` - accumulator for completed strings
        /// * `buf` - pre-allocated byte buffer
        /// * `pos` - current write position in buffer
        /// * `open` - number of '(' placed
        /// * `close` - number of ')' placed
        /// * `n` - target pair count
        fn backtrack(
            result: &mut Vec<String>,
            buf: &mut [u8],
            pos: usize,
            open: usize,
            close: usize,
            n: usize,
        ) {
            if pos == n * 2 {
                // SAFETY: buf only contains b'(' and b')', both valid ASCII.
                // Using from_utf8_unchecked avoids a validation scan.
                let s = unsafe { String::from_utf8_unchecked(buf.to_vec()) };
                result.push(s);
                return;
            }

            if open < n {
                buf[pos] = b'(';
                Self::backtrack(result, buf, pos + 1, open + 1, close, n);
            }

            if close < open {
                buf[pos] = b')';
                Self::backtrack(result, buf, pos + 1, open, close + 1, n);
            }
        }
    }
}

fn main() {
    for n in 1..=4 {
        let my = my_solution::Solution::generate_parenthesis(n);
        let ai = ai_solution::Solution::generate_parenthesis(n);
        println!("n={n}: my:{my:?}, ai:{ai:?}");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_generate_parenthesis {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_n1() {
                    let mut result = Solution::generate_parenthesis(1);
                    result.sort();
                    assert_eq!(result, vec!["()"]);
                }

                #[test]
                fn test_n2() {
                    let mut result = Solution::generate_parenthesis(2);
                    result.sort();
                    assert_eq!(result, vec!["(())", "()()"]);
                }

                #[test]
                fn test_n3() {
                    let mut result = Solution::generate_parenthesis(3);
                    result.sort();
                    assert_eq!(
                        result,
                        vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
                    );
                }

                #[test]
                fn test_n4_count() {
                    // Catalan(4) = 14 valid combinations.
                    let result = Solution::generate_parenthesis(4);
                    assert_eq!(result.len(), 14);
                }

                #[test]
                fn test_n8_count() {
                    // Catalan(8) = 1430 valid combinations.
                    let result = Solution::generate_parenthesis(8);
                    assert_eq!(result.len(), 1430);
                }

                #[test]
                fn test_all_valid() {
                    // Verify every string produced for n=5 is well-formed.
                    let results = Solution::generate_parenthesis(5);
                    for s in &results {
                        assert_eq!(s.len(), 10);
                        let mut depth = 0i32;
                        for c in s.chars() {
                            match c {
                                '(' => depth += 1,
                                ')' => depth -= 1,
                                _ => panic!("unexpected char"),
                            }
                            assert!(depth >= 0, "unbalanced: {s}");
                        }
                        assert_eq!(depth, 0, "unbalanced: {s}");
                    }
                }

                #[test]
                fn test_no_duplicates() {
                    let results = Solution::generate_parenthesis(5);
                    let mut sorted = results.clone();
                    sorted.sort();
                    sorted.dedup();
                    assert_eq!(sorted.len(), results.len());
                }
            }
        };
    }

    test_generate_parenthesis!(my_tests, crate::my_solution::Solution);
    test_generate_parenthesis!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        let n = 8; // Max constraint value — 1430 results per call.

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::generate_parenthesis(
                std::hint::black_box(n),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::generate_parenthesis(
                std::hint::black_box(n),
            ));
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
