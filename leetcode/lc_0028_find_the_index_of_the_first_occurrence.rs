// LeetCode 28: Find the Index of the First Occurrence in a String [Easy]
// https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/
//
// Given two strings needle and haystack, return the index of the first
// occurrence of needle in haystack, or -1 if needle is not part of haystack.
//
// Examples:
//   haystack = "sadbutsad", needle = "sad"   -> 0
//   haystack = "leetcode",  needle = "leeto" -> -1
//
// Constraints:
//   - 1 <= haystack.length, needle.length <= 10^4
//   - haystack and needle consist of only lowercase English characters
//
// Run:  cargo run --example lc_0028_find_the_index_of_the_first_occurrence
// Test: cargo test --example lc_0028_find_the_index_of_the_first_occurrence
// Bench: cargo test --example lc_0028_find_the_index_of_the_first_occurrence bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the index of the first occurrence of `needle` in `haystack`.
        ///
        /// # Arguments
        ///
        /// * `haystack` - The string to search in
        /// * `needle` - The string to search for
        ///
        /// # Returns
        ///
        /// Index of the first occurrence of `needle`, or -1 if not found
        pub fn str_str(haystack: String, needle: String) -> i32 {
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
        /// Find the index of the first occurrence of `needle` in `haystack`.
        ///
        /// # Arguments
        ///
        /// * `haystack` - The string to search in
        /// * `needle` - The string to search for
        ///
        /// # Returns
        ///
        /// Index of the first occurrence of `needle`, or -1 if not found
        pub fn str_str(haystack: String, needle: String) -> i32 {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(&str, &str, i32)> = vec![
        ("sadbutsad", "sad", 0),
        ("leetcode", "leeto", -1),
    ];

    println!("--- My Solution ---");
    for (haystack, needle, expected) in &test_cases {
        let result =
            my_solution::Solution::str_str(haystack.to_string(), needle.to_string());
        println!("  haystack=\"{haystack}\", needle=\"{needle}\" -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (haystack, needle, expected) in &test_cases {
        let result =
            ai_solution::Solution::str_str(haystack.to_string(), needle.to_string());
        println!("  haystack=\"{haystack}\", needle=\"{needle}\" -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_str_str {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_found_at_start() {
                    assert_eq!(
                        Solution::str_str("sadbutsad".to_string(), "sad".to_string()),
                        0
                    );
                }

                #[test]
                fn test_not_found() {
                    assert_eq!(
                        Solution::str_str("leetcode".to_string(), "leeto".to_string()),
                        -1
                    );
                }

                #[test]
                fn test_found_in_middle() {
                    assert_eq!(
                        Solution::str_str("hello".to_string(), "ll".to_string()),
                        2
                    );
                }

                #[test]
                fn test_exact_match() {
                    assert_eq!(
                        Solution::str_str("abc".to_string(), "abc".to_string()),
                        0
                    );
                }

                #[test]
                fn test_single_char_found() {
                    assert_eq!(
                        Solution::str_str("a".to_string(), "a".to_string()),
                        0
                    );
                }

                #[test]
                fn test_needle_longer_than_haystack() {
                    assert_eq!(
                        Solution::str_str("ab".to_string(), "abc".to_string()),
                        -1
                    );
                }

                #[test]
                fn test_found_at_end() {
                    assert_eq!(
                        Solution::str_str("abcdef".to_string(), "def".to_string()),
                        3
                    );
                }
            }
        };
    }

    test_str_str!(my_tests, crate::my_solution::Solution);
    test_str_str!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<(&str, &str)> = vec![
            ("sadbutsad", "sad"),
            ("leetcode", "leeto"),
            ("hello", "ll"),
            ("mississippi", "issip"),
            ("aaaaaaaab", "aaab"),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for &(h, n) in &inputs {
                std::hint::black_box(crate::my_solution::Solution::str_str(
                    std::hint::black_box(h.to_string()),
                    std::hint::black_box(n.to_string()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &(h, n) in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::str_str(
                    std::hint::black_box(h.to_string()),
                    std::hint::black_box(n.to_string()),
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
