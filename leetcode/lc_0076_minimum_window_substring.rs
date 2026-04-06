// LeetCode 76: Minimum Window Substring [Hard]
// https://leetcode.com/problems/minimum-window-substring/
//
// Given two strings s and t of lengths m and n respectively, return the
// minimum window substring of s such that every character in t (including
// duplicates) is included in the window. If there is no such substring,
// return the empty string "".
//
// The testcases will be generated such that the answer is unique.
//
// Examples:
//   s = "ADOBECODEBANC", t = "ABC" -> "BANC"
//   s = "a", t = "a"               -> "a"
//   s = "a", t = "aa"              -> ""
//
// Constraints:
//   - m == s.length, n == t.length
//   - 1 <= m, n <= 10^5
//   - s and t consist of uppercase and lowercase English letters
//
// Follow up: Could you find an algorithm that runs in O(m + n) time?
//
// Run:  cargo run --example lc_0076_minimum_window_substring
// Test: cargo test --example lc_0076_minimum_window_substring
// Bench: cargo test --example lc_0076_minimum_window_substring bench -- --nocapture
//
// --- Benchmark (100,000 iterations, 2026-03-27) ---
//   My solution:  3.166s total (avg 31.664us)
//   AI solution:  146.2ms total (avg 1.462us)
//   -> AI solution is 21.65x faster
//   Reason: Fixed [i32; 128] arrays replace HashMap — eliminates hashing,
//           bucket lookup, and heap allocation; direct array indexing is
//           cache-friendly and branch-predictor-friendly.
//
// --- Rust concepts used ---
//   - HashMap entry API: or_insert(0) for default-or-mutate pattern
//   - if let Some(&val) = map.get(&key) for combined existence check + binding
//   - std::str::from_utf8() to convert &[u8] back to &str
//   - Fixed-size array [i32; 128] as O(1) lookup table replacing HashMap
//   (Previously covered: as_bytes lc_0005, sliding window two-pointer lc_0003)
//
// --- Reference: faster single-counter approach (LeetCode top submission) ---
//   Uses ONE array + ONE total counter instead of two arrays + formed/unique:
//
//   let mut total = t.len() as i32;
//   let mut counts: [i32; 128] = [0; 128];
//   for &b in t.as_bytes() { counts[b as usize] += 1; }
//
//   // Expand: if counts[r] > 0 { total -= 1; } counts[r] -= 1;
//   // Contract: counts[l] += 1; if counts[l] > 0 { total += 1; }
//   // Window valid when: total == 0
//
//   Why faster:
//   - Single array merges need + window (counts starts at t's freq, goes
//     negative as chars enter window). Halves memory reads per iteration.
//   - Compare-to-zero (`counts[r] > 0`) is cheaper than equality check
//     (`window[rb] == need[rb]`) — branch predictor handles zero-tests faster.
//   - One integer `total` (remaining needed chars) replaces `formed`/`unique`
//     pair — fewer comparisons in inner loop.
//   - Tradeoff: original used `chars().collect::<Vec<char>>()` (heap alloc)
//     and `.iter().collect()` for result. Our as_bytes() is zero-copy but
//     per-iteration savings from single-counter outweigh this overhead.

/// My solution module — HashMap-based sliding window.
///
/// Time:  O(m + n)  — each pointer advances at most m times, HashMap ops amortized O(1)
/// Space: O(k)      — k = number of distinct chars in t (two HashMaps)
mod my_solution {
    pub struct Solution;

    use std::collections::HashMap;

    impl Solution {
        /// Find the minimum window substring of `s` containing all chars of `t`.
        ///
        /// # Arguments
        ///
        /// * `s` - The source string to search in
        /// * `t` - The target string whose characters must be covered
        ///
        /// # Returns
        ///
        /// The minimum window substring, or "" if no valid window exists
        pub fn min_window(s: String, t: String) -> String {
            let t = t.as_bytes();
            let s = s.as_bytes();
            let mut need: HashMap<u8, i32> = HashMap::new();
            // Clippy: originally `for (_, &char) in t.iter().enumerate()` —
            // unused_enumerate lint flags enumerate() when the index is discarded `_`.
            for &char in t.iter() {
                // Ruby: need[char] += 1 — Hash.new(0) auto-defaults missing keys to 0.
                // Rust: HashMap has no IndexMut — can't do need[&char] += 1.
                // Use entry API: or_insert(0) inserts 0 if key is missing,
                // returns &mut i32 either way, then * dereferences to increment.
                // A: Hashmap cannot give default value 0, need use entry
                *need.entry(char).or_insert(0) += 1;
            }

            let required = need.len();
            let mut window: HashMap<u8, i32> = HashMap::new();
            let mut formed = 0;
            let mut left = 0;
            let mut right = 0;
            let mut min_len = usize::MAX;
            let mut min_left = 0;

            while right < s.len() {
                let char = s[right];
                *window.entry(char).or_insert(0) += 1;

                // Ruby: need.key?(char) — checks if key exists in hash.
                // Rust: HashMap has contains_key(&key), but here we
                // only need the value, so need.get(&char) returns Option<&i32>.
                // Ruby: window[char] == need[char] — direct hash indexing.
                // Rust: HashMap[] panics on missing key. Use get() which
                // returns Option, then unwrap_or(&0) for a safe default.
                if let Some(&required_count) = need.get(&char) {
                    if window[&char] == required_count {
                        formed += 1;
                    }
                }

                while left <= right && formed == required {
                    let current_len = right - left + 1;
                    if current_len < min_len {
                        min_len = current_len;
                        min_left = left;
                    }

                    let left_char = s[left];
                    *window.entry(left_char).or_insert(0) -= 1;

                    if let Some(&required_count) = need.get(&left_char) {
                        if window[&left_char] < required_count {
                            formed -= 1;
                        }
                    }

                    left += 1;
                }

                right += 1
            }

            if min_len == usize::MAX {
                return String::new();
            };

            // Ruby: s[min_left, min_len] — substring from index, length
            // Rust: s is &[u8] (from as_bytes()), so s[range] returns &[u8], not &str.
            // &[u8] doesn't implement Display/ToString. Use std::str::from_utf8()
            // to convert the byte slice back to &str (safe since input is ASCII).
            // .unwrap() is safe here because the original String was valid UTF-8.
            // &[u8] cannot to_string()
            // Clippy: originally `min_len as usize` — identity_conversion lint
            // flags casting usize to usize (min_len was changed from i32 to usize).
            std::str::from_utf8(&s[min_left..min_left + min_len])
                .unwrap()
                .to_string()
        }
    }
}

/// AI solution module — array-based sliding window.
///
/// Uses `[i32; 128]` fixed arrays instead of HashMap for O(1) direct
/// indexing with no hashing overhead. Same algorithmic approach but
/// faster constant factor due to cache-friendly contiguous memory.
///
/// Time:  O(m + n)  — each pointer advances at most m times
/// Space: O(1)      — two fixed 128-element arrays (ASCII range)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the minimum window substring of `s` containing all chars of `t`.
        ///
        /// # Arguments
        ///
        /// * `s` - The source string to search in
        /// * `t` - The target string whose characters must be covered
        ///
        /// # Returns
        ///
        /// The minimum window substring, or "" if no valid window exists
        pub fn min_window(s: String, t: String) -> String {
            let s = s.as_bytes();
            let t = t.as_bytes();

            // Fixed-size arrays — 128 covers all ASCII chars.
            // Direct index lookup: no hashing, no bucket collisions.
            let mut need = [0i32; 128];
            let mut window = [0i32; 128];
            let mut unique = 0u32; // distinct chars in t

            for &b in t {
                if need[b as usize] == 0 {
                    unique += 1;
                }
                need[b as usize] += 1;
            }

            let mut formed = 0u32;
            let mut left = 0;
            let mut min_len = usize::MAX;
            let mut min_left = 0;

            for right in 0..s.len() {
                let rb = s[right] as usize;
                window[rb] += 1;

                // Only count chars that appear in t
                if need[rb] > 0 && window[rb] == need[rb] {
                    formed += 1;
                }

                // Contract window from the left while all chars are satisfied
                while formed == unique {
                    let current_len = right - left + 1;
                    if current_len < min_len {
                        min_len = current_len;
                        min_left = left;
                    }

                    let lb = s[left] as usize;
                    window[lb] -= 1;
                    if need[lb] > 0 && window[lb] < need[lb] {
                        formed -= 1;
                    }
                    left += 1;
                }
            }

            if min_len == usize::MAX {
                String::new()
            } else {
                std::str::from_utf8(&s[min_left..min_left + min_len])
                    .unwrap()
                    .to_string()
            }
        }
    }
}

fn main() {
    let test_cases: Vec<(&str, &str, &str)> = vec![
        ("ADOBECODEBANC", "ABC", "BANC"),
        ("a", "a", "a"),
        ("a", "aa", ""),
    ];

    println!("--- My Solution ---");
    for (s, t, expected) in &test_cases {
        let result = my_solution::Solution::min_window(s.to_string(), t.to_string());
        println!("  s=\"{s}\", t=\"{t}\" -> \"{result}\" (expected \"{expected}\")");
    }

    println!("--- AI Solution ---");
    for (s, t, expected) in &test_cases {
        let result = ai_solution::Solution::min_window(s.to_string(), t.to_string());
        println!("  s=\"{s}\", t=\"{t}\" -> \"{result}\" (expected \"{expected}\")");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_min_window {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(
                        Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
                        "BANC"
                    );
                }

                #[test]
                fn test_exact_match() {
                    assert_eq!(Solution::min_window("a".to_string(), "a".to_string()), "a");
                }

                #[test]
                fn test_no_valid_window() {
                    assert_eq!(Solution::min_window("a".to_string(), "aa".to_string()), "");
                }

                #[test]
                fn test_t_equals_s() {
                    assert_eq!(
                        Solution::min_window("abc".to_string(), "abc".to_string()),
                        "abc"
                    );
                }

                #[test]
                fn test_single_char_repeated() {
                    assert_eq!(
                        Solution::min_window("aa".to_string(), "aa".to_string()),
                        "aa"
                    );
                }

                #[test]
                fn test_window_at_end() {
                    assert_eq!(
                        Solution::min_window("bba".to_string(), "ab".to_string()),
                        "ba"
                    );
                }

                #[test]
                fn test_no_overlap() {
                    assert_eq!(Solution::min_window("xyz".to_string(), "a".to_string()), "");
                }
            }
        };
    }

    test_min_window!(my_tests, crate::my_solution::Solution);
    test_min_window!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<(&str, &str)> = vec![
            ("ADOBECODEBANC", "ABC"),
            ("a", "a"),
            ("a", "aa"),
            ("abcdefghijklmnop", "fmp"),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for &(s, t) in &inputs {
                std::hint::black_box(crate::my_solution::Solution::min_window(
                    std::hint::black_box(s.to_string()),
                    std::hint::black_box(t.to_string()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &(s, t) in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::min_window(
                    std::hint::black_box(s.to_string()),
                    std::hint::black_box(t.to_string()),
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
