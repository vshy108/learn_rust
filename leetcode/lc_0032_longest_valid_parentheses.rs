// LeetCode 32: Longest Valid Parentheses [Hard]
// https://leetcode.com/problems/longest-valid-parentheses/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 514ns
//   AI solution avg: 427ns
//   Ratio: AI solution is 1.20x faster
//
// Given a string containing just '(' and ')', return the length of the
// longest valid (well-formed) parentheses substring.
//
// Example 1: s = "(()" -> 2  (the valid substring is "()")
// Example 2: s = ")()())" -> 4  (the valid substring is "()()")
// Example 3: s = "" -> 0
//
// Constraints:
//   0 <= s.length <= 3 * 10^4
//   s[i] is '(' or ')'
//
// Run:  cargo run --example lc_0032_longest_valid_parentheses
// Test: cargo test --example lc_0032_longest_valid_parentheses
// Bench: cargo test --example lc_0032_longest_valid_parentheses bench -- --nocapture

/// My solution — stack-based approach.
///
/// Push indices onto a stack. Push the index of '(' directly. On ')',
/// pop the stack; if the stack becomes empty, push the current index
/// as a new base. Otherwise, the current valid length is `i - stack.top`.
///
/// Time: O(n) — single pass.
/// Space: O(n) — stack can hold up to n indices.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the length of the longest valid parentheses substring.
        ///
        /// # Arguments
        /// * `s` - string of '(' and ')' characters
        ///
        /// # Returns
        /// Length of the longest valid substring.
        pub fn longest_valid_parentheses(s: String) -> i32 {
            let bytes = s.as_bytes();
            // Stack stores indices; initialize with -1 as a sentinel base.
            let mut stack: Vec<i32> = vec![-1];
            let mut max_len: i32 = 0;

            for (i, &b) in bytes.iter().enumerate() {
                let i = i as i32;
                if b == b'(' {
                    stack.push(i);
                } else {
                    stack.pop(); // pop the matching '(' or the base
                    if stack.is_empty() {
                        // No matching '(' — push current index as new base.
                        stack.push(i);
                    } else {
                        // Valid length = current index - top of stack.
                        let len = i - stack.last().unwrap();
                        if len > max_len {
                            max_len = len;
                        }
                    }
                }
            }

            max_len
        }
    }
}

/// AI-optimized solution — two-pass counter approach (O(1) space).
///
/// Pass left-to-right: count open/close parens. When equal, record length.
/// When close > open, reset both. Then pass right-to-left similarly but
/// reset when open > close.
///
/// Time: O(n) — two passes.
/// Space: O(1) — only counter variables.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the length of the longest valid parentheses substring.
        ///
        /// # Arguments
        /// * `s` - string of '(' and ')' characters
        ///
        /// # Returns
        /// Length of the longest valid substring.
        pub fn longest_valid_parentheses(s: String) -> i32 {
            let bytes = s.as_bytes();
            let mut max_len: i32 = 0;

            // Left-to-right pass.
            let mut open = 0i32;
            let mut close = 0i32;
            for &b in bytes {
                if b == b'(' {
                    open += 1;
                } else {
                    close += 1;
                }
                if open == close {
                    let len = open + close;
                    if len > max_len {
                        max_len = len;
                    }
                } else if close > open {
                    // Too many ')' — reset.
                    open = 0;
                    close = 0;
                }
            }

            // Right-to-left pass (catches cases like "((()" missed above).
            open = 0;
            close = 0;
            for &b in bytes.iter().rev() {
                if b == b'(' {
                    open += 1;
                } else {
                    close += 1;
                }
                if open == close {
                    let len = open + close;
                    if len > max_len {
                        max_len = len;
                    }
                } else if open > close {
                    // Too many '(' — reset.
                    open = 0;
                    close = 0;
                }
            }

            max_len
        }
    }
}

fn main() {
    let cases = ["(()", ")()())", "", "()()", "(()()", "(()"];
    for s in &cases {
        println!(
            "\"{s}\" -> my:{}, ai:{}",
            my_solution::Solution::longest_valid_parentheses(s.to_string()),
            ai_solution::Solution::longest_valid_parentheses(s.to_string()),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_longest_valid_parentheses {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(Solution::longest_valid_parentheses("(()".into()), 2);
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(Solution::longest_valid_parentheses(")()())".into()), 4);
                }

                #[test]
                fn test_empty() {
                    assert_eq!(Solution::longest_valid_parentheses("".into()), 0);
                }

                #[test]
                fn test_all_open() {
                    assert_eq!(Solution::longest_valid_parentheses("((((".into()), 0);
                }

                #[test]
                fn test_all_close() {
                    assert_eq!(Solution::longest_valid_parentheses("))))".into()), 0);
                }

                #[test]
                fn test_perfect() {
                    assert_eq!(Solution::longest_valid_parentheses("()()".into()), 4);
                }

                #[test]
                fn test_nested() {
                    assert_eq!(Solution::longest_valid_parentheses("(())".into()), 4);
                }

                #[test]
                fn test_complex() {
                    assert_eq!(
                        Solution::longest_valid_parentheses("()(()".into()),
                        2
                    );
                }

                #[test]
                fn test_long_valid() {
                    assert_eq!(
                        Solution::longest_valid_parentheses("(()())".into()),
                        6
                    );
                }

                #[test]
                fn test_leading_close() {
                    assert_eq!(
                        Solution::longest_valid_parentheses(")()()((".into()),
                        4
                    );
                }
            }
        };
    }

    test_longest_valid_parentheses!(my_tests, crate::my_solution::Solution);
    test_longest_valid_parentheses!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        // Mix of valid and invalid sections.
        let s = ")()())(()())(()()(())))(()".to_string();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::longest_valid_parentheses(
                std::hint::black_box(s.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::longest_valid_parentheses(
                std::hint::black_box(s.clone()),
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
