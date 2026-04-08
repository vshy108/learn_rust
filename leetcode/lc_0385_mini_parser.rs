// LeetCode 385: Mini Parser [Medium]
// https://leetcode.com/problems/mini-parser/
//
// Given a string s representing the serialization of a nested list, implement
// a parser to deserialize it and return the deserialized NestedInteger.
//
// Each element is either an integer or a list whose elements may also be
// integers or other lists.
//
// Examples:
//   s = "324"             -> 324 (a single integer)
//   s = "[123,[456,[789]]]" -> [123,[456,[789]]]
//
// Constraints:
//   - 1 <= s.length <= 5 * 10^4
//   - s consists of digits, '[]', '-', and ','
//   - s is the serialization of a valid NestedInteger
//   - All values are in the range [-10^6, 10^6]
//
// Note: LeetCode provides a NestedInteger interface. We define our own enum
// to make the problem self-contained in Rust.
//
// Run:  cargo run --example lc_0385_mini_parser
// Test: cargo test --example lc_0385_mini_parser
// Bench: cargo test --example lc_0385_mini_parser bench -- --nocapture
//
// --- Benchmark (100k iterations, 2026-04-08) ---
// My solution:  263.24ms (avg 2.63us)
// AI solution:  147.21ms (avg 1.47us)
// -> AI solution is 1.79x faster
// Reason: The stack-based iterative parser avoids recursive function call
//   overhead and substring slicing allocations — it walks the byte array
//   once, building numbers in-place and pushing/popping a stack.

/// Our NestedInteger type — either a single integer or a list of NestedIntegers.
#[derive(Debug, Clone, PartialEq)]
enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

impl std::fmt::Display for NestedInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NestedInteger::Int(v) => write!(f, "{v}"),
            NestedInteger::List(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{item}")?;
                }
                write!(f, "]")
            }
        }
    }
}

/// My solution module — recursive descent parser.
///
/// If the string doesn't start with '[', parse it as a bare integer.
/// Otherwise, strip the outer brackets, split on top-level commas
/// (tracking bracket depth), and recursively parse each substring.
///
/// Time:  O(n) — each character is visited once across all recursion levels
/// Space: O(n) — recursion depth proportional to nesting depth, plus output
mod my_solution {
    use crate::NestedInteger;

    pub struct Solution;

    impl Solution {
        /// Deserialize a string into a NestedInteger.
        ///
        /// # Arguments
        ///
        /// * `s` - Serialized nested integer string
        ///
        /// # Returns
        ///
        /// The deserialized NestedInteger
        pub fn deserialize(s: String) -> NestedInteger {
            Self::parse(s.as_bytes())
        }

        /// Recursive parser operating on a byte slice.
        fn parse(s: &[u8]) -> NestedInteger {
            if s.is_empty() || s == b"[]" {
                return NestedInteger::List(Vec::new());
            }
            // If it doesn't start with '[', it's a bare integer
            if s[0] != b'[' {
                // Safe: s is valid ASCII digits/minus
                let val: i32 = std::str::from_utf8(s)
                    .expect("valid UTF-8")
                    .parse()
                    .expect("valid integer");
                return NestedInteger::Int(val);
            }
            // Strip outer brackets: s[1..len-1]
            let inner = &s[1..s.len() - 1];
            let mut result = Vec::new();
            let mut depth = 0i32;
            let mut start = 0;

            for i in 0..inner.len() {
                match inner[i] {
                    b'[' => depth += 1,
                    b']' => depth -= 1,
                    // Split at top-level commas (depth == 0)
                    b',' if depth == 0 => {
                        result.push(Self::parse(&inner[start..i]));
                        start = i + 1;
                    }
                    _ => {}
                }
            }
            // Don't forget the last segment
            if start < inner.len() {
                result.push(Self::parse(&inner[start..]));
            }
            NestedInteger::List(result)
        }
    }
}

/// AI solution module — stack-based iterative parser.
///
/// Walks the string character by character:
///   - '[' -> push a new List onto the stack
///   - ']' or ',' -> if a number was being built, push it into the
///     top-of-stack list; on ']', pop and merge into parent
///   - digit/'-' -> accumulate the current number
///
/// This avoids recursion and substring allocation entirely.
///
/// Time:  O(n)
/// Space: O(n) — stack depth proportional to nesting, plus output
mod ai_solution {
    use crate::NestedInteger;

    pub struct Solution;

    impl Solution {
        /// Deserialize a string into a NestedInteger.
        ///
        /// # Arguments
        ///
        /// * `s` - Serialized nested integer string
        ///
        /// # Returns
        ///
        /// The deserialized NestedInteger
        pub fn deserialize(s: String) -> NestedInteger {
            let bytes = s.as_bytes();

            // If it doesn't start with '[', it's a bare integer
            if bytes[0] != b'[' {
                let val: i32 = s.parse().expect("valid integer");
                return NestedInteger::Int(val);
            }

            let mut stack: Vec<NestedInteger> = Vec::new();
            let mut num: i32 = 0;
            let mut neg = false;
            // Tracks whether we're in the middle of parsing a number
            let mut has_num = false;

            for (i, &b) in bytes.iter().enumerate() {
                match b {
                    b'-' => neg = true,
                    b'0'..=b'9' => {
                        num = num * 10 + (b - b'0') as i32;
                        has_num = true;
                    }
                    b'[' => {
                        stack.push(NestedInteger::List(Vec::new()));
                    }
                    b',' | b']' => {
                        // If we just finished a number, add it to top of stack
                        if has_num {
                            let val = if neg { -num } else { num };
                            // Push the integer into the list at the top of stack
                            if let Some(NestedInteger::List(items)) = stack.last_mut() {
                                items.push(NestedInteger::Int(val));
                            }
                            num = 0;
                            neg = false;
                            has_num = false;
                        }
                        // On ']', pop and merge into parent (if not the outermost)
                        if b == b']' && stack.len() > 1 {
                            let top = stack.pop().expect("stack non-empty");
                            if let Some(NestedInteger::List(parent)) = stack.last_mut() {
                                parent.push(top);
                            }
                        }
                    }
                    _ => {
                        // Only digits, brackets, commas, and '-' in valid input
                        // (ignore i to suppress unused-variable warning in
                        // the outer enumerate)
                        let _ = i;
                    }
                }
            }

            // The final result is the single element left on the stack
            stack.pop().expect("stack should have exactly one element")
        }
    }
}

fn main() {
    let cases = ["324", "[123,[456,[789]]]", "[-1]", "[1,[2,[3]],4]"];

    println!("--- My Solution ---");
    for s in &cases {
        let result = my_solution::Solution::deserialize(s.to_string());
        println!("  \"{s}\" -> {result}");
    }

    println!("--- AI Solution ---");
    for s in &cases {
        let result = ai_solution::Solution::deserialize(s.to_string());
        println!("  \"{s}\" -> {result}");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_deserialize {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use crate::NestedInteger;
                use $sol_path as Solution;

                #[test]
                fn test_single_integer() {
                    assert_eq!(
                        Solution::deserialize("324".to_string()),
                        NestedInteger::Int(324)
                    );
                }

                #[test]
                fn test_negative_integer() {
                    assert_eq!(
                        Solution::deserialize("-5".to_string()),
                        NestedInteger::Int(-5)
                    );
                }

                #[test]
                fn test_nested_list() {
                    let result = Solution::deserialize("[123,[456,[789]]]".to_string());
                    let expected = NestedInteger::List(vec![
                        NestedInteger::Int(123),
                        NestedInteger::List(vec![
                            NestedInteger::Int(456),
                            NestedInteger::List(vec![NestedInteger::Int(789)]),
                        ]),
                    ]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_empty_list() {
                    assert_eq!(
                        Solution::deserialize("[]".to_string()),
                        NestedInteger::List(Vec::new())
                    );
                }

                #[test]
                fn test_flat_list() {
                    let result = Solution::deserialize("[1,2,3]".to_string());
                    let expected = NestedInteger::List(vec![
                        NestedInteger::Int(1),
                        NestedInteger::Int(2),
                        NestedInteger::Int(3),
                    ]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_single_element_list() {
                    let result = Solution::deserialize("[42]".to_string());
                    let expected = NestedInteger::List(vec![NestedInteger::Int(42)]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_negative_in_list() {
                    let result = Solution::deserialize("[-1]".to_string());
                    let expected = NestedInteger::List(vec![NestedInteger::Int(-1)]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_mixed_nesting() {
                    let result = Solution::deserialize("[1,[2,[3]],4]".to_string());
                    let expected = NestedInteger::List(vec![
                        NestedInteger::Int(1),
                        NestedInteger::List(vec![
                            NestedInteger::Int(2),
                            NestedInteger::List(vec![NestedInteger::Int(3)]),
                        ]),
                        NestedInteger::Int(4),
                    ]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_zero() {
                    assert_eq!(
                        Solution::deserialize("0".to_string()),
                        NestedInteger::Int(0)
                    );
                }
            }
        };
    }

    test_deserialize!(my_tests, crate::my_solution::Solution);
    test_deserialize!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        // Moderately nested input for benchmarking
        let input = "[1,[2,[3,[4,[5,[6,[7,[8,[9,10]]]]]]]]]".to_string();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::deserialize(
                std::hint::black_box(input.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::deserialize(
                std::hint::black_box(input.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution:  {:.2?} (avg {:.2?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "AI solution:  {:.2?} (avg {:.2?})",
            ai_duration,
            ai_duration / iterations
        );
        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.0 {
            println!("-> AI solution is {ratio:.2}x faster");
        } else {
            println!("-> My solution is {:.2}x faster", 1.0 / ratio);
        }
    }
}
