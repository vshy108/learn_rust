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
//
// --- Rust concepts used ---
// - `into_iter().collect::<String>()`: consumes Vec<String> and
//   concatenates all elements into a single owned String
// - Multi-index pointer walk: (array_idx, byte_idx) pairs to compare
//   two Vec<String>s byte-by-byte without allocating joined strings
// - `str::is_empty()` guard: skip zero-length strings to avoid
//   out-of-bounds on `as_bytes()[0]`
//   (Previously covered: .as_bytes() -> lc_1047, into_iter() -> lc_0151)
//
// --- Benchmark (100_000 iterations, 2026-04-07) ---
// My solution:  239.59ms (avg 2.395us)
// AI solution:  217.97ms (avg 2.179us)
// -> AI solution is 1.10x faster
// Reason: O(1) space byte-by-byte walk avoids the allocation overhead
//   of joining both arrays into Strings.

/// My solution module — join and compare.
///
/// Time:  O(n + m) where n, m = total chars in word1, word2
/// Space: O(n + m) for the joined strings
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
            let s1: String = word1.into_iter().collect();
            let s2: String = word2.into_iter().collect();
            s1 == s2
        }
    }
}

/// AI solution module — byte-by-byte comparison without joining.
///
/// Walks both arrays simultaneously using (array_index, char_index)
/// pointers, comparing one byte at a time without allocating.
///
/// Time:  O(min(n, m))
/// Space: O(1)
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
            let mut i1 = 0usize; // index into word1 array
            let mut j1 = 0usize; // index into word1[i1] bytes
            let mut i2 = 0usize;
            let mut j2 = 0usize;
            // Skip empty strings in each array to avoid out-of-bounds
            while i1 < word1.len() && word1[i1].is_empty() {
                i1 += 1;
            }
            while i2 < word2.len() && word2[i2].is_empty() {
                i2 += 1;
            }
            loop {
                let done1 = i1 >= word1.len();
                let done2 = i2 >= word2.len();
                if done1 && done2 {
                    return true;
                }
                if done1 || done2 {
                    return false;
                }
                let b1 = word1[i1].as_bytes()[j1];
                let b2 = word2[i2].as_bytes()[j2];
                if b1 != b2 {
                    return false;
                }
                j1 += 1;
                if j1 >= word1[i1].len() {
                    i1 += 1;
                    j1 = 0;
                    // Skip empty strings after advancing
                    while i1 < word1.len() && word1[i1].is_empty() {
                        i1 += 1;
                    }
                }
                j2 += 1;
                if j2 >= word2[i2].len() {
                    i2 += 1;
                    j2 = 0;
                    while i2 < word2.len() && word2[i2].is_empty() {
                        i2 += 1;
                    }
                }
            }
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
