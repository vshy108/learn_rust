// LeetCode 1662: Check If Two String Arrays are Equivalent [Easy]
// https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/
//
// Given two string arrays word1 and word2, return true if the two arrays
// represent the same string, and false otherwise.
//
// A string is represented by an array if the array elements concatenated
// in order forms the string.
//
// Examples:
//   word1 = ["ab","c"],   word2 = ["a","bc"]      -> true
//   word1 = ["a","cb"],   word2 = ["ab","c"]       -> false
//   word1 = ["abc","d","defg"], word2 = ["abcddefg"] -> true
//
// Constraints:
//   - 1 <= word1.length, word2.length <= 10^3
//   - 1 <= word1[i].length, word2[i].length <= 10^3
//   - 1 <= sum(word1[i].length), sum(word2[i].length) <= 10^3
//   - word1[i] and word2[i] consist of lowercase letters
//
// Run:  cargo run --example lc_1662_check_if_two_string_arrays_are_equivalent
// Test: cargo test --example lc_1662_check_if_two_string_arrays_are_equivalent
// Bench: cargo test --example lc_1662_check_if_two_string_arrays_are_equivalent bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Check if two string arrays represent the same string.
        ///
        /// # Arguments
        ///
        /// * `word1` - First array of strings
        /// * `word2` - Second array of strings
        ///
        /// # Returns
        ///
        /// true if concatenated arrays are equal
        pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
            // str1 = word1.join
            // str2 = word2.join
            // str1 == str2
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
        /// Check if two string arrays represent the same string.
        ///
        /// # Arguments
        ///
        /// * `word1` - First array of strings
        /// * `word2` - Second array of strings
        ///
        /// # Returns
        ///
        /// true if concatenated arrays are equal
        pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
            todo!()
        }
    }
}

/// Helper: convert a slice of &str into Vec<String>.
fn to_string_vec(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

fn main() {
    let test_cases: Vec<(Vec<&str>, Vec<&str>, bool)> = vec![
        (vec!["ab", "c"], vec!["a", "bc"], true),
        (vec!["a", "cb"], vec!["ab", "c"], false),
        (vec!["abc", "d", "defg"], vec!["abcddefg"], true),
    ];

    println!("--- My Solution ---");
    for (w1, w2, expected) in &test_cases {
        let result =
            my_solution::Solution::array_strings_are_equal(to_string_vec(w1), to_string_vec(w2));
        println!("  {w1:?}, {w2:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (w1, w2, expected) in &test_cases {
        let result =
            ai_solution::Solution::array_strings_are_equal(to_string_vec(w1), to_string_vec(w2));
        println!("  {w1:?}, {w2:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    use super::to_string_vec;

    macro_rules! test_array_strings_are_equal {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::to_string_vec;
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert!(Solution::array_strings_are_equal(
                        to_string_vec(&["ab", "c"]),
                        to_string_vec(&["a", "bc"]),
                    ));
                }

                #[test]
                fn test_example_2() {
                    assert!(!Solution::array_strings_are_equal(
                        to_string_vec(&["a", "cb"]),
                        to_string_vec(&["ab", "c"]),
                    ));
                }

                #[test]
                fn test_example_3() {
                    assert!(Solution::array_strings_are_equal(
                        to_string_vec(&["abc", "d", "defg"]),
                        to_string_vec(&["abcddefg"]),
                    ));
                }

                #[test]
                fn test_single_char_each() {
                    assert!(Solution::array_strings_are_equal(
                        to_string_vec(&["a"]),
                        to_string_vec(&["a"]),
                    ));
                }

                #[test]
                fn test_different_lengths() {
                    assert!(!Solution::array_strings_are_equal(
                        to_string_vec(&["a", "b"]),
                        to_string_vec(&["a", "b", "c"]),
                    ));
                }

                #[test]
                fn test_empty_strings() {
                    assert!(Solution::array_strings_are_equal(
                        to_string_vec(&["", "a"]),
                        to_string_vec(&["a"]),
                    ));
                }

                #[test]
                fn test_many_single_chars() {
                    assert!(Solution::array_strings_are_equal(
                        to_string_vec(&["a", "b", "c"]),
                        to_string_vec(&["abc"]),
                    ));
                }
            }
        };
    }

    test_array_strings_are_equal!(my_tests, crate::my_solution::Solution);
    test_array_strings_are_equal!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<(Vec<&str>, Vec<&str>)> = vec![
            (vec!["ab", "c"], vec!["a", "bc"]),
            (vec!["a", "cb"], vec!["ab", "c"]),
            (vec!["abc", "d", "defg"], vec!["abcddefg"]),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for (w1, w2) in &inputs {
                std::hint::black_box(crate::my_solution::Solution::array_strings_are_equal(
                    std::hint::black_box(to_string_vec(w1)),
                    std::hint::black_box(to_string_vec(w2)),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for (w1, w2) in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::array_strings_are_equal(
                    std::hint::black_box(to_string_vec(w1)),
                    std::hint::black_box(to_string_vec(w2)),
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
