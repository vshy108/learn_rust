// LeetCode 439: Ternary Expression Parser [Medium]
// https://leetcode.com/problems/ternary-expression-parser/
//
// Given a string expression representing arbitrarily nested ternary
// expressions, evaluate the expression and return the result as a string.
// The expression consists of digits (0-9), 'T' (true), 'F' (false), '?',
// and ':'. The ternary operator groups right-to-left (like in most
// languages), and all digits are single-digit numbers.
//
// Examples:
//   "T?2:3"        -> "2"
//   "F?1:T?4:5"    -> "4"
//   "T?T?F:5:3"    -> "F"
//
// Constraints:
//   5 <= expression.length <= 10^4
//   expression consists of digits, 'T', 'F', '?', and ':'
//   expression is a valid ternary expression and each number is one digit
//
// Run:  cargo run --example lc_0439_ternary_expression_parser
// Test: cargo test --example lc_0439_ternary_expression_parser
// Bench: cargo test --example lc_0439_ternary_expression_parser bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 6.677us
//   AI solution: avg 22.96us
//   Ratio: 0.29x (stack much faster than recursion)

/// Time: O(n), Space: O(n)
/// Stack-based approach: scan the expression right-to-left. Push every
/// character that is not '?' or ':'. When we hit '?', the character
/// immediately to the left is the condition (T/F). Pop the two top
/// values from the stack (true-branch and false-branch) and push back
/// whichever the condition selects.
#[allow(dead_code)]
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Evaluate a nested ternary expression string.
        ///
        /// # Arguments
        /// * `expression` - A valid ternary expression string
        ///
        /// # Returns
        /// The evaluated result as a single-character `String`
        pub fn parse_ternary(expression: String) -> String {
            let bytes = expression.as_bytes();
            let mut stack: Vec<u8> = Vec::with_capacity(bytes.len());

            // Scan right-to-left; skip ':' characters
            let mut i = bytes.len();
            while i > 0 {
                i -= 1;
                let ch = bytes[i];
                if ch == b':' {
                    // Separator -- skip it
                    continue;
                }
                if ch == b'?' {
                    // The condition is immediately to the left of '?'
                    i -= 1;
                    let condition = bytes[i];
                    // Top of stack is the true-branch, next is false-branch
                    let if_true = stack.pop().expect("stack has true-branch");
                    let if_false = stack.pop().expect("stack has false-branch");
                    stack.push(if condition == b'T' { if_true } else { if_false });
                } else {
                    // Digit or 'T'/'F' literal -- push as-is
                    stack.push(ch);
                }
            }

            // Exactly one element remains: the final result
            String::from(stack[0] as char)
        }
    }
}

/// Time: O(n), Space: O(n) (recursion stack depth proportional to nesting)
/// Recursive descent parser. We maintain a shared index into the byte
/// slice. At each level we read the next character; if followed by '?',
/// we parse the true-branch and false-branch recursively, then return
/// whichever the condition selects.
#[allow(dead_code)]
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Evaluate a nested ternary expression string.
        ///
        /// # Arguments
        /// * `expression` - A valid ternary expression string
        ///
        /// # Returns
        /// The evaluated result as a single-character `String`
        pub fn parse_ternary(expression: String) -> String {
            let bytes = expression.as_bytes();
            let mut pos = 0usize;
            let result = Self::parse(bytes, &mut pos);
            String::from(result as char)
        }

        /// Recursively parse starting at `pos`.
        ///
        /// Reads a value (digit / T / F). If the next character is '?',
        /// it recursively parses the true-branch and false-branch,
        /// consuming the '?' and ':' delimiters along the way.
        fn parse(bytes: &[u8], pos: &mut usize) -> u8 {
            // Read the current value (condition or terminal)
            let value = bytes[*pos];
            *pos += 1;

            // If no '?' follows, this is a terminal value
            if *pos >= bytes.len() || bytes[*pos] != b'?' {
                return value;
            }

            // Consume '?'
            *pos += 1;
            // Parse the true-branch
            let if_true = Self::parse(bytes, pos);
            // Consume ':'
            *pos += 1;
            // Parse the false-branch
            let if_false = Self::parse(bytes, pos);

            if value == b'T' { if_true } else { if_false }
        }
    }
}

fn main() {
    println!("LeetCode 439: Ternary Expression Parser");
    let cases = vec![("T?2:3", "2"), ("F?1:T?4:5", "4"), ("T?T?F:5:3", "F")];
    for (expr, expected) in &cases {
        let r1 = my_solution::Solution::parse_ternary(expr.to_string());
        let r2 = ai_solution::Solution::parse_ternary(expr.to_string());
        println!("  \"{expr}\" => my=\"{r1}\", ai=\"{r2}\" (expected \"{expected}\")");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_both {
        ($name:ident, $expr:expr, $expected:expr) => {
            mod $name {
                use super::*;
                #[test]
                fn my() {
                    assert_eq!(
                        my_solution::Solution::parse_ternary($expr.to_string()),
                        $expected
                    );
                }
                #[test]
                fn ai() {
                    assert_eq!(
                        ai_solution::Solution::parse_ternary($expr.to_string()),
                        $expected
                    );
                }
            }
        };
    }

    test_both!(example1, "T?2:3", "2");
    test_both!(example2, "F?1:T?4:5", "4");
    test_both!(example3, "T?T?F:5:3", "F");
    test_both!(single_true, "T?1:2", "1");
    test_both!(single_false, "F?1:2", "2");
    test_both!(nested_both, "T?T?1:2:F?3:4", "1");
    test_both!(deep_right, "F?0:F?0:T?1:0", "1");
    test_both!(all_digits, "T?9:0", "9");
    test_both!(triple_nested, "T?T?T?1:2:3:4", "1");
    test_both!(false_chain, "F?1:F?2:F?3:4", "4");
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(String) -> String,
{
    let iterations = 500_000;
    // Build a deeply nested expression: T?T?T?...1:0...:0:0
    let mut expr = String::with_capacity(10_000);
    let depth = 2_500;
    for _ in 0..depth {
        expr.push_str("T?");
    }
    expr.push('1');
    for _ in 0..depth {
        expr.push_str(":0");
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(expr.clone())));
    }
    let elapsed = start.elapsed();
    println!(
        "{name}: {iterations} iterations in {elapsed:?} (avg {:?})",
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::parse_ternary);
    bench_solution("AI solution", ai_solution::Solution::parse_ternary);
}
