// LeetCode 3: Longest Substring Without Repeating Characters
// https://leetcode.com/problems/longest-substring-without-repeating-characters/
//
// Given a string `s`, find the length of the longest substring without
// repeating characters.
//
// A "substring" is a contiguous sequence of characters within the string.
//
// Examples:
//   "abcabcbb" -> 3   (substring "abc")
//   "bbbbb"    -> 1   (substring "b")
//   "pwwkew"   -> 3   (substring "wke")
//   ""         -> 0   (empty string)
//
// Constraints:
//   - 0 <= s.len() <= 50_000
//   - s consists of English letters, digits, symbols, and spaces
//
// Run:  cargo run --example lc_0003_longest_substring_without_repeating
// Test: cargo test --example lc_0003_longest_substring_without_repeating
// Bench: cargo test --example lc_0003_longest_substring_without_repeating bench -- --nocapture
//
// --- Benchmark (1k iterations, 2026-03-26) ---
//   My solution (HashMap):  4.002ms avg (4.0s total)
//   AI solution (array):    104.925us avg (105ms total)
//   AI solution is 38.14x faster
//
// --- Rust concepts used ---
//
// Sliding window:
//   A two-pointer technique where `left` and `right` define a window into
//   the string. Expand `right` to include new characters; shrink `left`
//   when a duplicate is found. The window always contains unique characters.
//
// HashMap / HashSet for tracking:
//   Use a HashMap<char, usize> to store the last-seen index of each
//   character, or a HashSet<char> to track which characters are in the
//   current window.
//
// String iteration:
//   `s.chars()` yields `char` values (Unicode scalar values).
//   `s.char_indices()` yields `(usize, char)` — the byte index and char.
//   Since this problem uses ASCII-range characters, byte index == char index.
//
// .max() and cmp::max:
//   `std::cmp::max(a, b)` returns the larger of two values.
//   Useful for tracking the running maximum window size.

/// My solution module — implement your solution here.
mod my_solution {
    use std::collections::HashMap;
    pub struct Solution;

    impl Solution {
        /// Find the length of the longest substring without repeating
        /// characters.
        ///
        /// # Arguments
        ///
        /// * `s` - The input string to search
        ///
        /// # Returns
        ///
        /// Length of the longest substring with all unique characters
        pub fn length_of_longest_substring(s: String) -> i32 {
            // A: initialize HashMap
            // A: use HashMap<char, usize> not HashMap<&char, usize> —
            //    `c` from char_indices() is a temp loop variable that dies
            //    each iteration, so storing &c would be a dangling reference.
            //    `char` is Copy (4 bytes), so storing by value is free.
            let mut last_seen: HashMap<char, usize> = HashMap::new();
            let mut left = 0;
            let mut max_len = 0;

            // A: String to char iteration with index
            for (i, c) in s.char_indices() {
                // i = position, c = character
                if let Some(&previous_last_seen) = last_seen.get(&c) && previous_last_seen >= left {
                    left = previous_last_seen + 1;
                }

                last_seen.insert(c, i);
                // A: get max value between 2 integers, float cannot std::cmp::max
                // max_len = max_len.max(i - left + 1);
                max_len = std::cmp::max(max_len, (i - left + 1) as usize);
            }

            max_len as i32
        }
    }
}

/// AI solution — array-based sliding window.
///
/// Key difference from my_solution:
///   my_solution uses HashMap<char, usize> for last-seen tracking.
///   This solution uses a fixed [usize; 128] array indexed by ASCII value.
///   Array indexing is O(1) with no hashing overhead; the entire array
///   fits in a few cache lines, so lookups are faster in practice.
///
/// Tradeoff:
///   - Faster constant factor (no hashing, cache-friendly)
///   - Only works for ASCII (128 entries). For full Unicode, a HashMap
///     would be needed.
///   - Same big-O: O(n) time, O(1) space (fixed 128-element array)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the length of the longest substring without repeating
        /// characters.
        ///
        /// # Arguments
        ///
        /// * `s` - The input string to search
        ///
        /// # Returns
        ///
        /// Length of the longest substring with all unique characters
        pub fn length_of_longest_substring(s: String) -> i32 {
            // Fixed-size array for ASCII chars (0..127).
            // Stores last-seen index + 1 so that 0 means "never seen".
            // This avoids needing Option or a sentinel like usize::MAX.
            // `0_usize` is a type-suffixed literal — the `_usize` suffix
            // tells the compiler this 0 is of type `usize` (not i32, u8,
            // etc.). Equivalent to `0 as usize` or `0usize`.
            // `[value; N]` creates an array of N elements, all set to value.
            // A: provide type in the value, 0_usize
            let mut last_seen = [0_usize; 128];
            let mut left = 0_usize;
            let mut max_len = 0_i32;

            // `as_bytes()` returns &[u8] — the raw bytes of the string.
            // Faster than `chars()` which must decode variable-width
            // UTF-8 (1-4 bytes per char). For ASCII input, each char
            // is exactly 1 byte, so byte index == char index and the
            // byte value (0..127) can index directly into the array.
            // Would NOT work for multi-byte Unicode (e.g. emoji).
            for (i, &b) in s.as_bytes().iter().enumerate() {
                // Cast u8 byte to usize for array indexing (Rust requires
                // usize for indexes). Each ASCII char has a numeric code:
                // e.g. 'a' = 97, 'z' = 122, ' ' = 32. This byte value
                // becomes the slot in last_seen[128], so last_seen[97]
                // tracks when we last saw 'a' — no hashing needed.
                let idx = b as usize;
                // If this char was seen at or after `left`, shrink the
                // window. last_seen[idx] stores (previous index + 1),
                // so comparing against left works directly.
                if last_seen[idx] > left {
                    left = last_seen[idx];
                }
                // Store index + 1 so 0 remains the "unseen" sentinel.
                last_seen[idx] = i + 1;
                let window = (i - left + 1) as i32;
                if window > max_len {
                    max_len = window;
                }
            }

            max_len
        }
    }
}

fn main() {
    let test_cases = [
        ("abcabcbb", 3),
        ("bbbbb", 1),
        ("pwwkew", 3),
        ("", 0),
    ];

    println!("--- My Solution ---");
    for (input, expected) in &test_cases {
        let result = my_solution::Solution::length_of_longest_substring(
            input.to_string(),
        );
        println!(
            "  \"{input}\" -> {result} (expected {expected})"
        );
    }

    println!("--- AI Solution ---");
    for (input, expected) in &test_cases {
        let result = ai_solution::Solution::length_of_longest_substring(
            input.to_string(),
        );
        println!(
            "  \"{input}\" -> {result} (expected {expected})"
        );
    }
}

#[cfg(test)]
mod tests {
    /// Macro that generates the same set of tests for any Solution type.
    macro_rules! test_longest_substring {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_basic() {
                    // "abc" is the longest substring without repeating
                    assert_eq!(
                        Solution::length_of_longest_substring(
                            "abcabcbb".to_string()
                        ),
                        3
                    );
                }

                #[test]
                fn test_all_same() {
                    // Every character is 'b', so longest unique is just "b"
                    assert_eq!(
                        Solution::length_of_longest_substring(
                            "bbbbb".to_string()
                        ),
                        1
                    );
                }

                #[test]
                fn test_repeat_in_middle() {
                    // "wke" is the longest (not "pwke" — must be contiguous)
                    assert_eq!(
                        Solution::length_of_longest_substring(
                            "pwwkew".to_string()
                        ),
                        3
                    );
                }

                #[test]
                fn test_empty_string() {
                    assert_eq!(
                        Solution::length_of_longest_substring(
                            "".to_string()
                        ),
                        0
                    );
                }

                #[test]
                fn test_single_char() {
                    assert_eq!(
                        Solution::length_of_longest_substring(
                            "a".to_string()
                        ),
                        1
                    );
                }

                #[test]
                fn test_all_unique() {
                    // Entire string is the answer
                    assert_eq!(
                        Solution::length_of_longest_substring(
                            "abcdef".to_string()
                        ),
                        6
                    );
                }

                #[test]
                fn test_with_spaces_and_symbols() {
                    // "b a" has length 3 (space counts as a character)
                    assert_eq!(
                        Solution::length_of_longest_substring(
                            "ab a".to_string()
                        ),
                        3
                    );
                }

                #[test]
                fn test_repeat_at_end() {
                    // "abcd" then 'a' repeats — longest is "abcd" = 4
                    assert_eq!(
                        Solution::length_of_longest_substring(
                            "abcda".to_string()
                        ),
                        4
                    );
                }
            }
        };
    }

    // Run the SAME tests against both solutions.
    test_longest_substring!(my_tests, crate::my_solution::Solution);

    test_longest_substring!(ai_tests, crate::ai_solution::Solution);

    /// Benchmark both solutions side by side.
    ///
    /// Builds a long string with repeated patterns to stress-test the
    /// sliding window. Use:
    /// `cargo test --example lc_0003_longest_substring_without_repeating bench -- --nocapture`
    ///
    /// `std::hint::black_box` prevents the compiler from optimizing away
    /// the function call, ensuring we measure real execution time.
    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 1_000;
        // Build a ~10k char string with repeating ASCII pattern to
        // exercise the sliding window across many duplicates.
        let input: String = (0..10_000)
            .map(|i| (b'a' + (i % 26) as u8) as char)
            .collect();

        // --- Benchmark my_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::length_of_longest_substring(
                    std::hint::black_box(input.clone()),
                ),
            );
        }
        let my_duration = start.elapsed();

        // --- Benchmark ai_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::length_of_longest_substring(
                    std::hint::black_box(input.clone()),
                ),
            );
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
            let ratio = ai_duration.as_nanos() as f64
                / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64
                / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
