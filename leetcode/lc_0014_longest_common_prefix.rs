// LeetCode 14: Longest Common Prefix [Easy]
// https://leetcode.com/problems/longest-common-prefix/
//
// Write a function to find the longest common prefix string amongst an
// array of strings. If there is no common prefix, return an empty string "".
//
// Examples:
//   strs = ["flower","flow","flight"] -> "fl"
//   strs = ["dog","racecar","car"]    -> ""
//
// Constraints:
//   - 1 <= strs.length <= 200
//   - 0 <= strs[i].length <= 200
//   - strs[i] consists of only lowercase English letters if non-empty
//
// Run:  cargo run --example lc_0014_longest_common_prefix
// Test: cargo test --example lc_0014_longest_common_prefix
// Bench: cargo test --example lc_0014_longest_common_prefix bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the longest common prefix among an array of strings.
        ///
        /// # Arguments
        ///
        /// * `strs` - Array of strings
        ///
        /// # Returns
        ///
        /// The longest common prefix string
        pub fn longest_common_prefix(strs: Vec<String>) -> String {
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
        /// Find the longest common prefix among an array of strings.
        ///
        /// # Arguments
        ///
        /// * `strs` - Array of strings
        ///
        /// # Returns
        ///
        /// The longest common prefix string
        pub fn longest_common_prefix(strs: Vec<String>) -> String {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<&str>, &str)> = vec![
        (vec!["flower", "flow", "flight"], "fl"),
        (vec!["dog", "racecar", "car"], ""),
    ];

    println!("--- My Solution ---");
    for (strs, expected) in &test_cases {
        let input: Vec<String> = strs.iter().map(|s| s.to_string()).collect();
        let result = my_solution::Solution::longest_common_prefix(input);
        println!("  {strs:?} -> \"{result}\" (expected \"{expected}\")");
    }

    println!("--- AI Solution ---");
    for (strs, expected) in &test_cases {
        let input: Vec<String> = strs.iter().map(|s| s.to_string()).collect();
        let result = ai_solution::Solution::longest_common_prefix(input);
        println!("  {strs:?} -> \"{result}\" (expected \"{expected}\")");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_lcp {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                fn to_vec(strs: &[&str]) -> Vec<String> {
                    strs.iter().map(|s| s.to_string()).collect()
                }

                #[test]
                fn test_example_1() {
                    assert_eq!(
                        Solution::longest_common_prefix(to_vec(&["flower", "flow", "flight"])),
                        "fl"
                    );
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(
                        Solution::longest_common_prefix(to_vec(&["dog", "racecar", "car"])),
                        ""
                    );
                }

                #[test]
                fn test_single_string() {
                    assert_eq!(Solution::longest_common_prefix(to_vec(&["alone"])), "alone");
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(
                        Solution::longest_common_prefix(to_vec(&["abc", "abc", "abc"])),
                        "abc"
                    );
                }

                #[test]
                fn test_empty_string_in_array() {
                    assert_eq!(Solution::longest_common_prefix(to_vec(&["", "abc"])), "");
                }

                #[test]
                fn test_single_char_prefix() {
                    assert_eq!(
                        Solution::longest_common_prefix(to_vec(&["ab", "ac", "ad"])),
                        "a"
                    );
                }
            }
        };
    }

    test_lcp!(my_tests, crate::my_solution::Solution);
    test_lcp!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<Vec<String>> = vec![
            vec!["flower", "flow", "flight"]
                .into_iter()
                .map(String::from)
                .collect(),
            vec!["dog", "racecar", "car"]
                .into_iter()
                .map(String::from)
                .collect(),
            vec!["abc", "abc", "abc"]
                .into_iter()
                .map(String::from)
                .collect(),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for strs in &inputs {
                std::hint::black_box(crate::my_solution::Solution::longest_common_prefix(
                    std::hint::black_box(strs.clone()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for strs in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::longest_common_prefix(
                    std::hint::black_box(strs.clone()),
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
