// LeetCode 20: Valid Parentheses [Easy]
// https://leetcode.com/problems/valid-parentheses/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 98.222µs
//   AI solution avg: 99.710µs
//   Ratio: ~tied (1.02x)
//
// Given a string s containing just the characters '(', ')', '{', '}', '['
// and ']', determine if the input string is valid.
//
// A string is valid if:
//   1. Open brackets must be closed by the same type of brackets.
//   2. Open brackets must be closed in the correct order.
//   3. Every close bracket has a corresponding open bracket of the same type.
//
// Example 1: s = "()"     -> true
// Example 2: s = "()[]{}" -> true
// Example 3: s = "(]"     -> false
// Example 4: s = "([])"   -> true
//
// Constraints:
//   1 <= s.length <= 10^4
//   s consists of parentheses only '()[]{}'
//
// Run:  cargo run --example lc_0020_valid_parentheses
// Test: cargo test --example lc_0020_valid_parentheses
// Bench: cargo test --example lc_0020_valid_parentheses bench -- --nocapture

/// My solution — stack-based with match.
///
/// Push opening brackets onto a stack. For each closing bracket, pop the
/// stack and check if it matches. If the stack is non-empty at the end or
/// a mismatch is found, return false.
///
/// Time: O(n) — single pass through the string.
/// Space: O(n) — stack can hold up to n/2 opening brackets.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Check if the parentheses string is valid.
        ///
        /// # Arguments
        /// * `s` - string containing only '(', ')', '{', '}', '[', ']'
        ///
        /// # Returns
        /// `true` if brackets are balanced and correctly nested.
        pub fn is_valid(s: String) -> bool {
            let mut stack: Vec<u8> = Vec::with_capacity(s.len() / 2);

            for b in s.bytes() {
                match b {
                    b'(' | b'[' | b'{' => stack.push(b),
                    b')' => {
                        if stack.pop() != Some(b'(') {
                            return false;
                        }
                    }
                    b']' => {
                        if stack.pop() != Some(b'[') {
                            return false;
                        }
                    }
                    b'}' => {
                        if stack.pop() != Some(b'{') {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }

            stack.is_empty()
        }
    }
}

/// AI-optimized solution — stack with closing-bracket push.
///
/// Instead of pushing opening brackets and comparing on close, push the
/// *expected closing bracket* when an opening bracket is seen. On a closing
/// bracket, just check if the stack top equals the current character. This
/// avoids a multi-arm match on the closing side.
///
/// Time: O(n) — single pass.
/// Space: O(n) — same stack usage.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Check if the parentheses string is valid.
        ///
        /// # Arguments
        /// * `s` - string containing only '(', ')', '{', '}', '[', ']'
        ///
        /// # Returns
        /// `true` if brackets are balanced and correctly nested.
        pub fn is_valid(s: String) -> bool {
            // Early exit: odd-length strings can never be balanced.
            if s.len() % 2 != 0 {
                return false;
            }

            let mut stack: Vec<u8> = Vec::with_capacity(s.len() / 2);

            for b in s.bytes() {
                match b {
                    // Push the expected closing bracket.
                    b'(' => stack.push(b')'),
                    b'[' => stack.push(b']'),
                    b'{' => stack.push(b'}'),
                    // For closing brackets, the top of stack should match.
                    _ => {
                        if stack.pop() != Some(b) {
                            return false;
                        }
                    }
                }
            }

            stack.is_empty()
        }
    }
}

fn main() {
    let cases = ["()", "()[]{}", "(]", "([])", "{[]}"];
    for s in &cases {
        println!(
            "is_valid(\"{s}\") = my:{}, ai:{}",
            my_solution::Solution::is_valid(s.to_string()),
            ai_solution::Solution::is_valid(s.to_string()),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_is_valid {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_simple_pair() {
                    assert!(Solution::is_valid("()".into()));
                }

                #[test]
                fn test_multiple_types() {
                    assert!(Solution::is_valid("()[]{}".into()));
                }

                #[test]
                fn test_mismatch() {
                    assert!(!Solution::is_valid("(]".into()));
                }

                #[test]
                fn test_nested() {
                    assert!(Solution::is_valid("([])".into()));
                }

                #[test]
                fn test_deeply_nested() {
                    assert!(Solution::is_valid("{[()]}".into()));
                }

                #[test]
                fn test_unclosed() {
                    assert!(!Solution::is_valid("(".into()));
                }

                #[test]
                fn test_extra_close() {
                    assert!(!Solution::is_valid(")".into()));
                }

                #[test]
                fn test_wrong_order() {
                    assert!(!Solution::is_valid("([)]".into()));
                }

                #[test]
                fn test_empty_like_single() {
                    // s.length >= 1 per constraint, but odd-length is invalid.
                    assert!(!Solution::is_valid("((".into()));
                }
            }
        };
    }

    test_is_valid!(my_tests, crate::my_solution::Solution);
    test_is_valid!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        // Deeply nested valid string.
        let s: String = "(".repeat(5000) + &")".repeat(5000);

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::is_valid(
                std::hint::black_box(s.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::is_valid(
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
