// LeetCode 1544: Make The String Great [Easy]
// https://leetcode.com/problems/make-the-string-great/
//
// Given a string s of lower and upper case English letters.
//
// A good string is a string which doesn't have two adjacent characters s[i]
// and s[i + 1] where:
//   - s[i] is a lower-case letter and s[i + 1] is the same letter but in
//     upper-case or vice-versa.
//
// In other words, the string is good if there is no pair of adjacent
// characters that are the same letter but different case.
//
// To make the string good, you can choose two adjacent characters that make
// the string bad and remove them. You can keep doing this until the string
// becomes good.
//
// Return the string after making it good. The answer is guaranteed to be
// unique under the given constraints.
//
// Notice that an empty string is also good.
//
// Examples:
//   s = "leEeetcode" -> "leetcode"
//   s = "abBAcC"     -> ""
//   s = "s"          -> "s"
//
// Constraints:
//   - 1 <= s.length <= 100
//   - s contains only lower and upper case English letters
//
// Run:  cargo run --example lc_1544_make_the_string_great
// Test: cargo test --example lc_1544_make_the_string_great
// Bench: cargo test --example lc_1544_make_the_string_great bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Remove adjacent characters that are same letter but different case.
        ///
        /// # Arguments
        ///
        /// * `s` - Input string of mixed-case English letters
        ///
        /// # Returns
        ///
        /// The "good" string after all bad pairs are removed
        pub fn make_good(s: String) -> String {
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
        /// Remove adjacent characters that are same letter but different case.
        ///
        /// # Arguments
        ///
        /// * `s` - Input string of mixed-case English letters
        ///
        /// # Returns
        ///
        /// The "good" string after all bad pairs are removed
        pub fn make_good(s: String) -> String {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(&str, &str)> = vec![
        ("leEeetcode", "leetcode"),
        ("abBAcC", ""),
        ("s", "s"),
    ];

    println!("--- My Solution ---");
    for (s, expected) in &test_cases {
        let result = my_solution::Solution::make_good(s.to_string());
        println!("  \"{s}\" -> \"{result}\" (expected \"{expected}\")");
    }

    println!("--- AI Solution ---");
    for (s, expected) in &test_cases {
        let result = ai_solution::Solution::make_good(s.to_string());
        println!("  \"{s}\" -> \"{result}\" (expected \"{expected}\")");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_make_good {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(
                        Solution::make_good("leEeetcode".to_string()),
                        "leetcode"
                    );
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(Solution::make_good("abBAcC".to_string()), "");
                }

                #[test]
                fn test_single_char() {
                    assert_eq!(Solution::make_good("s".to_string()), "s");
                }

                #[test]
                fn test_already_good() {
                    assert_eq!(Solution::make_good("abc".to_string()), "abc");
                }

                #[test]
                fn test_all_removed() {
                    assert_eq!(Solution::make_good("aA".to_string()), "");
                }

                #[test]
                fn test_cascading() {
                    assert_eq!(Solution::make_good("mMaA".to_string()), "");
                }

                #[test]
                fn test_mixed_case_no_conflict() {
                    assert_eq!(Solution::make_good("aAbB".to_string()), "");
                }
            }
        };
    }

    test_make_good!(my_tests, crate::my_solution::Solution);
    test_make_good!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<&str> = vec!["leEeetcode", "abBAcC", "s", "abc"];

        let start = Instant::now();
        for _ in 0..iterations {
            for s in &inputs {
                std::hint::black_box(crate::my_solution::Solution::make_good(
                    std::hint::black_box(s.to_string()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for s in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::make_good(
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
