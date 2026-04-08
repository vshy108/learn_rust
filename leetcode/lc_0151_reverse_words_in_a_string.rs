// LeetCode 151: Reverse Words in a String [Medium]
// https://leetcode.com/problems/reverse-words-in-a-string/
//
// Given an input string s, reverse the order of the words. A word is
// defined as a sequence of non-space characters. The words in s will be
// separated by at least one space. Return a string of the words in
// reverse order concatenated by a single space.
//
// Note: s may contain leading or trailing spaces or multiple spaces
// between two words. The returned string should only have a single space
// separating the words. Do not include any extra spaces.
//
// Examples:
//   "the sky is blue"  -> "blue is sky the"
//   "  hello world  "  -> "world hello"
//   "a good   example" -> "example good a"
//
// Constraints:
//   - 1 <= s.length <= 10^4
//   - s contains English letters (upper/lower-case), digits, and spaces ' '
//   - There is at least one word in s
//
// Run:  cargo run --example lc_0151_reverse_words_in_a_string
// Test: cargo test --example lc_0151_reverse_words_in_a_string
// Bench: cargo test --example lc_0151_reverse_words_in_a_string bench -- --nocapture
//
// --- Benchmark (100,000 iterations, 2026-04-04) ---
// My solution: avg 4.518us
// AI solution: avg 2.482us
// Ratio: AI solution is 1.82x faster
// Reason: Byte-level manipulation avoids iterator + Vec<&str> + join overhead;
//         direct in-place reversal with a single compacted Vec<u8> is cheaper
//         than split_whitespace -> collect -> join which allocates a Vec + String.
/// My solution module.
///
/// Iterator chain: split on whitespace, reverse, rejoin.
/// `split_whitespace` handles trim + collapse in one step.
///
/// Time:  O(n) - single pass to split, reverse, and join
/// Space: O(n) - Vec<&str> of word slices + output String
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Reverse the order of words in a string.
        ///
        /// # Arguments
        ///
        /// * `s` - Input string with words separated by spaces
        ///
        /// # Returns
        ///
        /// Words in reverse order, separated by single spaces
        pub fn reverse_words(s: String) -> String {
            // split_whitespace: splits on any whitespace, auto-trims edges
            //   and collapses multiple spaces (handles all edge cases)
            // .rev(): lazily reverses the iterator (zero allocation)
            // .collect::<Vec<_>>(): materialises into Vec<&str>
            //   Required because .join() is a slice method, not an iterator method.
            //   Turbofish ::<Vec<_>> tells the compiler which collection type;
            //   _ lets Rust infer the element type (&str) from context.
            // .join(" "): concatenates with single-space separators
            s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
        }
    }
}

/// AI solution module.
///
/// Classic "reverse-all then reverse-each-word" byte manipulation.
/// Step 1: compact the string (trim + collapse spaces) into a byte vec.
/// Step 2: reverse the entire byte vec.
/// Step 3: reverse each individual word back to correct order.
///
/// Time:  O(n) - three linear passes over the bytes
/// Space: O(n) - compacted byte vec (at most input length)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Reverse the order of words in a string.
        ///
        /// Uses byte-level manipulation: compact whitespace, reverse all
        /// bytes, then reverse each word back to correct spelling.
        ///
        /// # Arguments
        ///
        /// * `s` - Input string with words separated by spaces
        ///
        /// # Returns
        ///
        /// Words in reverse order, separated by single spaces
        pub fn reverse_words(s: String) -> String {
            // Step 1: compact — trim edges + collapse multiple spaces
            //   Single pass: copy non-space bytes, insert exactly one
            //   space between words, skip leading/trailing/extra spaces.
            let mut bytes = Vec::with_capacity(s.len());
            let mut in_word = false;
            for &b in s.as_bytes() {
                if b == b' ' {
                    if in_word {
                        // End of word: push separator (may be trimmed later)
                        bytes.push(b' ');
                        in_word = false;
                    }
                    // Extra spaces: skip (collapsed)
                } else {
                    bytes.push(b);
                    in_word = true;
                }
            }
            // Remove trailing space (if input ended with spaces)
            if bytes.last() == Some(&b' ') {
                bytes.pop();
            }

            // Step 2: reverse entire byte array
            //   "blue is sky the" -> "eht yks si eulb"
            bytes.reverse();

            // Step 3: reverse each word in place to fix spelling
            //   "eht yks si eulb" -> "the sky is blue"... wait, that
            //   undoes it. Since we compacted first, the words are
            //   already in reversed order after step 2. Reversing each
            //   word fixes the letter order within each word.
            let n = bytes.len();
            let mut start = 0;
            for i in 0..=n {
                // Word boundary: space or end of array
                if i == n || bytes[i] == b' ' {
                    bytes[start..i].reverse();
                    start = i + 1;
                }
            }

            // Safety: input was valid UTF-8 ASCII (letters, digits, spaces);
            // reversing preserves validity
            String::from_utf8(bytes).expect("input was valid UTF-8")
        }
    }
}

fn main() {
    let test_cases: Vec<(&str, &str)> = vec![
        ("the sky is blue", "blue is sky the"),
        ("  hello world  ", "world hello"),
        ("a good   example", "example good a"),
    ];

    println!("--- My Solution ---");
    for (s, expected) in &test_cases {
        let result = my_solution::Solution::reverse_words(s.to_string());
        println!("  \"{s}\" -> \"{result}\" (expected \"{expected}\")");
    }

    println!("--- AI Solution ---");
    for (s, expected) in &test_cases {
        let result = ai_solution::Solution::reverse_words(s.to_string());
        println!("  \"{s}\" -> \"{result}\" (expected \"{expected}\")");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_reverse_words {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_basic() {
                    assert_eq!(
                        Solution::reverse_words("the sky is blue".to_string()),
                        "blue is sky the"
                    );
                }

                #[test]
                fn test_leading_trailing_spaces() {
                    assert_eq!(
                        Solution::reverse_words("  hello world  ".to_string()),
                        "world hello"
                    );
                }

                #[test]
                fn test_multiple_spaces() {
                    assert_eq!(
                        Solution::reverse_words("a good   example".to_string()),
                        "example good a"
                    );
                }

                #[test]
                fn test_single_word() {
                    assert_eq!(Solution::reverse_words("hello".to_string()), "hello");
                }

                #[test]
                fn test_single_word_with_spaces() {
                    assert_eq!(Solution::reverse_words("  hello  ".to_string()), "hello");
                }

                #[test]
                fn test_two_words() {
                    assert_eq!(Solution::reverse_words("ab cd".to_string()), "cd ab");
                }

                #[test]
                fn test_single_char_words() {
                    assert_eq!(Solution::reverse_words("a b c".to_string()), "c b a");
                }
            }
        };
    }

    test_reverse_words!(my_tests, crate::my_solution::Solution);
    test_reverse_words!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<&str> = vec![
            "the sky is blue",
            "  hello world  ",
            "a good   example",
            "single",
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for &s in &inputs {
                std::hint::black_box(crate::my_solution::Solution::reverse_words(
                    std::hint::black_box(s.to_string()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &s in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::reverse_words(
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
