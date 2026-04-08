// LeetCode 394: Decode String [Medium]
// https://leetcode.com/problems/decode-string/
//
// Benchmark: 500,000 iterations (2026-04-08)
//   My solution (iterative stack):     avg 553ns
//   AI solution (recursive descent):   avg 383ns
//   -> AI solution is 1.44x faster
//
// Given an encoded string, return its decoded string.
// Rule: k[encoded_string] means encoded_string is repeated k times.
// Input is always valid (no extra spaces, well-formed brackets).
// Digits only appear as repeat numbers, not inside encoded_string.
//
// Examples:
//   "3[a]2[bc]"    -> "aaabcbc"
//   "3[a2[c]]"     -> "accaccacc"
//   "2[abc]3[cd]ef" -> "abcabccdcdcdef"
//
// Constraints:
//   - 1 <= s.length <= 30
//   - s consists of lowercase letters, digits, and '[]'
//   - All integers in s are in range [1, 300]
//
// Run:  cargo run --example lc_0394_decode_string
// Test: cargo test --example lc_0394_decode_string
// Bench: cargo test --example lc_0394_decode_string bench -- --nocapture

/// My solution module — stack-based iterative decoding.
///
/// Use two stacks: one for repeat counts, one for accumulated strings.
/// On '[', push current count and string, reset both.
/// On ']', pop and repeat the current string, prepend the saved prefix.
///
/// Time:  O(n * max_k) where n = output length
/// Space: O(n) for the stacks and intermediate strings
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Decode the encoded string.
        ///
        /// # Arguments
        ///
        /// * `s` - Encoded string like "3[a2[c]]"
        ///
        /// # Returns
        ///
        /// The decoded string
        pub fn decode_string(s: String) -> String {
            let mut count_stack: Vec<usize> = Vec::new();
            let mut string_stack: Vec<String> = Vec::new();
            let mut num = 0usize;
            let mut current = String::new();

            for ch in s.chars() {
                if ch.is_ascii_digit() {
                    num = num * 10 + (ch as usize - '0' as usize);
                } else if ch == '[' {
                    // Save current state and start fresh
                    count_stack.push(num);
                    string_stack.push(current.clone());
                    num = 0;
                    current.clear();
                } else if ch == ']' {
                    // Pop and repeat
                    let repeat = count_stack.pop().expect("matching '[' guarantees a count");
                    let mut prefix = string_stack
                        .pop()
                        .expect("matching '[' guarantees a saved string");
                    prefix.push_str(&current.repeat(repeat));
                    current = prefix;
                } else {
                    current.push(ch);
                }
            }
            current
        }
    }
}

/// AI solution module — recursive descent parser.
///
/// Parse the string with an index pointer. When we hit a digit, parse
/// the full number, consume '[', recurse to decode the inner string,
/// consume ']', and repeat the result. Letters are appended directly.
///
/// Time:  O(n * max_k)
/// Space: O(d) recursion depth where d = max nesting
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Decode the encoded string.
        ///
        /// # Arguments
        ///
        /// * `s` - Encoded string like "3[a2[c]]"
        ///
        /// # Returns
        ///
        /// The decoded string
        pub fn decode_string(s: String) -> String {
            let bytes = s.as_bytes();
            let mut i = 0;
            Self::decode(bytes, &mut i)
        }

        /// Recursive helper that decodes from position `i`.
        fn decode(bytes: &[u8], i: &mut usize) -> String {
            let mut result = String::new();

            while *i < bytes.len() && bytes[*i] != b']' {
                if bytes[*i].is_ascii_digit() {
                    // Parse the full number
                    let mut num = 0usize;
                    while *i < bytes.len() && bytes[*i].is_ascii_digit() {
                        num = num * 10 + (bytes[*i] - b'0') as usize;
                        *i += 1;
                    }
                    *i += 1; // skip '['
                    let inner = Self::decode(bytes, i);
                    *i += 1; // skip ']'
                    // Repeat inner string `num` times
                    for _ in 0..num {
                        result.push_str(&inner);
                    }
                } else {
                    result.push(bytes[*i] as char);
                    *i += 1;
                }
            }
            result
        }
    }
}

fn main() {
    let cases = [
        ("3[a]2[bc]", "aaabcbc"),
        ("3[a2[c]]", "accaccacc"),
        ("2[abc]3[cd]ef", "abcabccdcdcdef"),
    ];

    println!("--- My Solution ---");
    for (input, expected) in &cases {
        let result = my_solution::Solution::decode_string(input.to_string());
        println!("  \"{input}\" -> \"{result}\" (expected \"{expected}\")");
    }

    println!("--- AI Solution ---");
    for (input, expected) in &cases {
        let result = ai_solution::Solution::decode_string(input.to_string());
        println!("  \"{input}\" -> \"{result}\" (expected \"{expected}\")");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_decode {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert_eq!(Solution::decode_string("3[a]2[bc]".to_string()), "aaabcbc");
                }

                #[test]
                fn test_example2() {
                    assert_eq!(Solution::decode_string("3[a2[c]]".to_string()), "accaccacc");
                }

                #[test]
                fn test_example3() {
                    assert_eq!(
                        Solution::decode_string("2[abc]3[cd]ef".to_string()),
                        "abcabccdcdcdef"
                    );
                }

                #[test]
                fn test_no_brackets() {
                    assert_eq!(Solution::decode_string("abc".to_string()), "abc");
                }

                #[test]
                fn test_single_repeat() {
                    assert_eq!(Solution::decode_string("1[a]".to_string()), "a");
                }

                #[test]
                fn test_large_repeat() {
                    assert_eq!(Solution::decode_string("10[a]".to_string()), "aaaaaaaaaa");
                }
            }
        };
    }

    test_decode!(my_tests, crate::my_solution::Solution);
    test_decode!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 500_000_u32;
        let input = "3[a2[b3[c]]]".to_string();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::decode_string(
                std::hint::black_box(input.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::decode_string(
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
