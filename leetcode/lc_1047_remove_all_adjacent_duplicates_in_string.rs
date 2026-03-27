// LeetCode 1047: Remove All Adjacent Duplicates In String [Easy]
// https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/
//
// You are given a string s consisting of lowercase English letters. A
// duplicate removal consists of choosing two adjacent and equal letters
// and removing them.
//
// We repeatedly make duplicate removals on s until we no longer can.
//
// Return the final string after all such duplicate removals have been made.
// It can be proven that the answer is unique.
//
// Examples:
//   s = "abbaca" -> "ca"
//   s = "azxxzy" -> "ay"
//
// Constraints:
//   - 1 <= s.length <= 10^5
//   - s consists of lowercase English letters
//
// Run:  cargo run --example lc_1047_remove_all_adjacent_duplicates_in_string
// Test: cargo test --example lc_1047_remove_all_adjacent_duplicates_in_string
// Bench: cargo test --example lc_1047_remove_all_adjacent_duplicates_in_string bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Remove all adjacent duplicate pairs repeatedly until none remain.
        ///
        /// # Arguments
        ///
        /// * `s` - Input string of lowercase English letters
        ///
        /// # Returns
        ///
        /// The final string after all duplicate removals
        pub fn remove_duplicates(s: String) -> String {
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
        /// Remove all adjacent duplicate pairs repeatedly until none remain.
        ///
        /// # Arguments
        ///
        /// * `s` - Input string of lowercase English letters
        ///
        /// # Returns
        ///
        /// The final string after all duplicate removals
        pub fn remove_duplicates(s: String) -> String {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(&str, &str)> = vec![("abbaca", "ca"), ("azxxzy", "ay")];

    println!("--- My Solution ---");
    for (s, expected) in &test_cases {
        let result = my_solution::Solution::remove_duplicates(s.to_string());
        println!("  \"{s}\" -> \"{result}\" (expected \"{expected}\")");
    }

    println!("--- AI Solution ---");
    for (s, expected) in &test_cases {
        let result = ai_solution::Solution::remove_duplicates(s.to_string());
        println!("  \"{s}\" -> \"{result}\" (expected \"{expected}\")");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_remove_duplicates {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(
                        Solution::remove_duplicates("abbaca".to_string()),
                        "ca"
                    );
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(
                        Solution::remove_duplicates("azxxzy".to_string()),
                        "ay"
                    );
                }

                #[test]
                fn test_no_duplicates() {
                    assert_eq!(
                        Solution::remove_duplicates("abc".to_string()),
                        "abc"
                    );
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(
                        Solution::remove_duplicates("aa".to_string()),
                        ""
                    );
                }

                #[test]
                fn test_single_char() {
                    assert_eq!(
                        Solution::remove_duplicates("a".to_string()),
                        "a"
                    );
                }

                #[test]
                fn test_cascading_removal() {
                    assert_eq!(
                        Solution::remove_duplicates("aababaab".to_string()),
                        "ba"
                    );
                }

                #[test]
                fn test_entire_string_removed() {
                    assert_eq!(
                        Solution::remove_duplicates("abba".to_string()),
                        ""
                    );
                }
            }
        };
    }

    test_remove_duplicates!(my_tests, crate::my_solution::Solution);
    test_remove_duplicates!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<&str> = vec!["abbaca", "azxxzy", "abc", "aababaab"];

        let start = Instant::now();
        for _ in 0..iterations {
            for s in &inputs {
                std::hint::black_box(crate::my_solution::Solution::remove_duplicates(
                    std::hint::black_box(s.to_string()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for s in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::remove_duplicates(
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
