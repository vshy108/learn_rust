// LeetCode 58: Length of Last Word [Easy]
// https://leetcode.com/problems/length-of-last-word/
//
// Given a string s consisting of words and spaces, return the length of
// the last word in the string.
//
// A word is a maximal substring consisting of non-space characters only.
//
// Examples:
//   s = "Hello World"                    -> 5
//   s = "   fly me   to   the moon  "   -> 4
//   s = "luffy is still joyboy"          -> 6
//
// Constraints:
//   - 1 <= s.length <= 10^4
//   - s consists of only English letters and spaces ' '
//   - There will be at least one word in s
//
// Run:  cargo run --example lc_0058_length_of_last_word
// Test: cargo test --example lc_0058_length_of_last_word
// Bench: cargo test --example lc_0058_length_of_last_word bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the length of the last word in the string.
        ///
        /// # Arguments
        ///
        /// * `s` - A string consisting of words and spaces
        ///
        /// # Returns
        ///
        /// The length of the last word
        pub fn length_of_last_word(s: String) -> i32 {
            todo!()
        }
    }
}

/// AI solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the length of the last word in the string.
        ///
        /// # Arguments
        ///
        /// * `s` - A string consisting of words and spaces
        ///
        /// # Returns
        ///
        /// The length of the last word
        pub fn length_of_last_word(s: String) -> i32 {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(&str, i32)> = vec![
        ("Hello World", 5),
        ("   fly me   to   the moon  ", 4),
        ("luffy is still joyboy", 6),
    ];

    println!("--- My Solution ---");
    for (s, expected) in &test_cases {
        let result = my_solution::Solution::length_of_last_word(s.to_string());
        println!("  \"{s}\" -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (s, expected) in &test_cases {
        let result = ai_solution::Solution::length_of_last_word(s.to_string());
        println!("  \"{s}\" -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_last_word {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_two_words() {
                    assert_eq!(
                        Solution::length_of_last_word("Hello World".to_string()),
                        5
                    );
                }

                #[test]
                fn test_trailing_spaces() {
                    assert_eq!(
                        Solution::length_of_last_word(
                            "   fly me   to   the moon  ".to_string()
                        ),
                        4
                    );
                }

                #[test]
                fn test_no_trailing_spaces() {
                    assert_eq!(
                        Solution::length_of_last_word(
                            "luffy is still joyboy".to_string()
                        ),
                        6
                    );
                }

                #[test]
                fn test_single_word() {
                    assert_eq!(
                        Solution::length_of_last_word("hello".to_string()),
                        5
                    );
                }

                #[test]
                fn test_single_char() {
                    assert_eq!(
                        Solution::length_of_last_word("a".to_string()),
                        1
                    );
                }

                #[test]
                fn test_leading_spaces_single_word() {
                    assert_eq!(
                        Solution::length_of_last_word("   word".to_string()),
                        4
                    );
                }

                #[test]
                fn test_single_char_with_spaces() {
                    assert_eq!(
                        Solution::length_of_last_word("  a  ".to_string()),
                        1
                    );
                }
            }
        };
    }

    test_last_word!(my_tests, crate::my_solution::Solution);
    test_last_word!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<&str> = vec![
            "Hello World",
            "   fly me   to   the moon  ",
            "luffy is still joyboy",
            "a",
            "   word   ",
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for &s in &inputs {
                std::hint::black_box(crate::my_solution::Solution::length_of_last_word(
                    std::hint::black_box(s.to_string()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &s in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::length_of_last_word(
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
