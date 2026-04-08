// Benchmark: 500,000 iterations (2025-04-08)
// My solution:  avg 2.02us  | Sliding window with shrink
// AI solution:  avg 2.02us  | Non-shrinking window (only grow/slide)
// -> Tied (1.00x)
//
// LeetCode 424: Longest Repeating Character Replacement [Medium]
// https://leetcode.com/problems/longest-repeating-character-replacement/
//
// Given a string s and an integer k, you can choose any character and change it
// to any other uppercase English character. You can perform this at most k times.
// Return the length of the longest substring containing the same letter after
// performing the above operations.
//
// Examples:
//   s = "ABAB", k = 2 -> 4  (Replace both 'A's or both 'B's)
//   s = "AABABBA", k = 1 -> 4  ("AABA" -> change B at index 3)
//
// Constraints:
//   1 <= s.length <= 10^5
//   s consists of only uppercase English letters
//   0 <= k <= s.length
//
// Run:  cargo run --example lc_0424_longest_repeating_character_replacement
// Test: cargo test --example lc_0424_longest_repeating_character_replacement
// Bench: cargo test --example lc_0424_longest_repeating_character_replacement bench -- --nocapture

/// My solution module — standard sliding window with shrinking.
///
/// Maintain a window [left..right]. Track frequency of each character
/// in the window. The number of characters to replace = window_size - max_freq.
/// If this exceeds k, shrink the window from the left.
///
/// Time:  O(n)  -- each element enters/leaves window at most once
/// Space: O(1)  -- 26-element frequency array
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the longest substring with at most k replacements.
        ///
        /// # Arguments
        ///
        /// * `s` - Input string of uppercase letters
        /// * `k` - Maximum number of character replacements
        ///
        /// # Returns
        ///
        /// Length of the longest valid substring
        pub fn character_replacement(s: String, k: i32) -> i32 {
            let bytes = s.as_bytes();
            let n = bytes.len();
            let k = k as usize;
            let mut freq = [0_usize; 26];
            let mut max_freq = 0_usize;
            let mut left = 0_usize;
            let mut result = 0_usize;

            for right in 0..n {
                let idx = (bytes[right] - b'A') as usize;
                freq[idx] += 1;
                // Update max_freq with the new character's count
                max_freq = max_freq.max(freq[idx]);

                // Window is invalid if we need to replace more than k chars
                let window_size = right - left + 1;
                if window_size - max_freq > k {
                    // Shrink from left by 1
                    let left_idx = (bytes[left] - b'A') as usize;
                    freq[left_idx] -= 1;
                    left += 1;
                }

                result = result.max(right - left + 1);
            }

            result as i32
        }
    }
}

/// AI solution module — non-shrinking sliding window optimization.
///
/// Same idea, but never shrinks the window — only grows or slides.
/// The window size never decreases, which is safe because we only
/// care about the maximum valid window ever found. When the window
/// is invalid, we slide both left and right by 1 (maintaining size).
///
/// Note: max_freq may be stale after shrinking, but that's fine
/// because a stale max_freq only means we might keep an invalid
/// window the same size — we never miss a valid larger window.
///
/// Time:  O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the longest substring with at most k replacements.
        ///
        /// # Arguments
        ///
        /// * `s` - Input string of uppercase letters
        /// * `k` - Maximum number of character replacements
        ///
        /// # Returns
        ///
        /// Length of the longest valid substring
        pub fn character_replacement(s: String, k: i32) -> i32 {
            let bytes = s.as_bytes();
            let n = bytes.len();
            let k = k as usize;
            let mut freq = [0_usize; 26];
            let mut max_freq = 0_usize;
            let mut left = 0_usize;

            for right in 0..n {
                let idx = (bytes[right] - b'A') as usize;
                freq[idx] += 1;
                max_freq = max_freq.max(freq[idx]);

                // If window is invalid, slide it forward (don't shrink)
                if (right - left + 1) - max_freq > k {
                    let left_idx = (bytes[left] - b'A') as usize;
                    freq[left_idx] -= 1;
                    left += 1;
                    // Note: we don't recalculate max_freq here
                    // A stale max_freq keeps the window the same size
                    // until a larger valid window is found
                }
            }

            // The final window size is the answer
            (n - left) as i32
        }
    }
}

fn main() {
    let cases = [("ABAB", 2, 4), ("AABABBA", 1, 4)];

    println!("--- My Solution ---");
    for (s, k, expected) in &cases {
        let result = my_solution::Solution::character_replacement(s.to_string(), *k);
        println!("  \"{s}\", k={k} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (s, k, expected) in &cases {
        let result = ai_solution::Solution::character_replacement(s.to_string(), *k);
        println!("  \"{s}\", k={k} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_replacement {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert_eq!(Solution::character_replacement("ABAB".to_string(), 2,), 4,);
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        Solution::character_replacement("AABABBA".to_string(), 1,),
                        4,
                    );
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(Solution::character_replacement("AAAA".to_string(), 2,), 4,);
                }

                #[test]
                fn test_k_zero() {
                    assert_eq!(Solution::character_replacement("ABCDE".to_string(), 0,), 1,);
                }

                #[test]
                fn test_single_char() {
                    assert_eq!(Solution::character_replacement("A".to_string(), 0,), 1,);
                }

                #[test]
                fn test_k_equals_length() {
                    assert_eq!(Solution::character_replacement("ABCDE".to_string(), 5,), 5,);
                }
            }
        };
    }

    test_replacement!(my_tests, crate::my_solution::Solution);
    test_replacement!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 500_000_u32;
        // Build a large input string with mixed characters
        let s: String = (0..1000).map(|i| (b'A' + (i % 4) as u8) as char).collect();
        let k = 100;

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::character_replacement(
                std::hint::black_box(s.clone()),
                std::hint::black_box(k),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::character_replacement(
                std::hint::black_box(s.clone()),
                std::hint::black_box(k),
            ));
        }
        let ai_duration = start.elapsed();

        let my_avg = my_duration / iterations;
        let ai_avg = ai_duration / iterations;

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution:  {my_duration:.2?} (avg {my_avg:.2?})");
        println!("AI solution:  {ai_duration:.2?} (avg {ai_avg:.2?})");

        if my_avg <= ai_avg {
            let ratio = ai_avg.as_nanos() as f64 / my_avg.as_nanos().max(1) as f64;
            println!("-> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_avg.as_nanos() as f64 / ai_avg.as_nanos().max(1) as f64;
            println!("-> AI solution is {ratio:.2}x faster");
        }
    }
}
