// LeetCode 345: Reverse Vowels of a String [Easy]
// https://leetcode.com/problems/reverse-vowels-of-a-string/
//
// Given a string s, reverse only all the vowels in the string and
// return it.
//
// The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear
// in both lower and upper cases, more than once.
//
// Examples:
//   s = "IcesCreAm" -> "AcesCreIm"
//   s = "leetcode"  -> "leotcede"
//
// Constraints:
//   - 1 <= s.length <= 3 * 10^5
//   - s consists of printable ASCII characters
//
// Run:  cargo run --example lc_0345_reverse_vowels_of_a_string
// Test: cargo test --example lc_0345_reverse_vowels_of_a_string
// Bench: cargo test --example lc_0345_reverse_vowels_of_a_string bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Reverse only the vowels in string `s`.
        ///
        /// # Arguments
        ///
        /// * `s` - The input string (printable ASCII)
        ///
        /// # Returns
        ///
        /// A new string with vowels reversed in position
        pub fn reverse_vowels(s: String) -> String {
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
        /// Reverse only the vowels in string `s`.
        ///
        /// # Arguments
        ///
        /// * `s` - The input string (printable ASCII)
        ///
        /// # Returns
        ///
        /// A new string with vowels reversed in position
        pub fn reverse_vowels(s: String) -> String {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(&str, &str)> = vec![
        ("IceCreAm", "AceCreIm"),
        ("leetcode", "leotcede"),
    ];

    println!("--- My Solution ---");
    for (s, expected) in &test_cases {
        let result = my_solution::Solution::reverse_vowels(s.to_string());
        println!("  \"{s}\" -> \"{result}\" (expected \"{expected}\")");
    }

    println!("--- AI Solution ---");
    for (s, expected) in &test_cases {
        let result = ai_solution::Solution::reverse_vowels(s.to_string());
        println!("  \"{s}\" -> \"{result}\" (expected \"{expected}\")");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_reverse_vowels {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_mixed_case_vowels() {
                    assert_eq!(
                        Solution::reverse_vowels("IceCreAm".to_string()),
                        "AceCreIm"
                    );
                }

                #[test]
                fn test_lowercase() {
                    assert_eq!(
                        Solution::reverse_vowels("leetcode".to_string()),
                        "leotcede"
                    );
                }

                #[test]
                fn test_no_vowels() {
                    assert_eq!(
                        Solution::reverse_vowels("bcdfg".to_string()),
                        "bcdfg"
                    );
                }

                #[test]
                fn test_all_vowels() {
                    assert_eq!(
                        Solution::reverse_vowels("aeiou".to_string()),
                        "uoiea"
                    );
                }

                #[test]
                fn test_single_char() {
                    assert_eq!(
                        Solution::reverse_vowels("a".to_string()),
                        "a"
                    );
                }

                #[test]
                fn test_uppercase_vowels() {
                    assert_eq!(
                        Solution::reverse_vowels("AEIOU".to_string()),
                        "UOIEA"
                    );
                }

                #[test]
                fn test_single_vowel() {
                    assert_eq!(
                        Solution::reverse_vowels("hello".to_string()),
                        "holle"
                    );
                }
            }
        };
    }

    test_reverse_vowels!(my_tests, crate::my_solution::Solution);
    test_reverse_vowels!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<&str> = vec![
            "IceCreAm",
            "leetcode",
            "aeiou",
            "bcdfg",
            "The quick brown fox jumps over the lazy dog",
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for &s in &inputs {
                std::hint::black_box(crate::my_solution::Solution::reverse_vowels(
                    std::hint::black_box(s.to_string()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &s in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::reverse_vowels(
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
