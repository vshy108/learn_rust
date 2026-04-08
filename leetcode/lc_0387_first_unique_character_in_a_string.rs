// LeetCode 387: First Unique Character in a String [Easy]
// https://leetcode.com/problems/first-unique-character-in-a-string/
//
// Benchmark: 100,000 iterations (2025-04-08)
//   My solution (two-pass count):     avg 9.78us
//   AI solution (single-pass + scan): avg 6.61us
//   -> AI solution is 1.48x faster
//
// Given a string s, find the first non-repeating character in it and return
// its index. If it does not exist, return -1.
//
// Examples:
//   s = "leetcode"      -> 0  ('l' is first unique)
//   s = "loveleetcode"  -> 2  ('v' is first unique)
//   s = "aabb"          -> -1 (no unique character)
//
// Constraints:
//   - 1 <= s.length <= 10^5
//   - s consists of only lowercase English letters
//
// Run:  cargo run --example lc_0387_first_unique_character_in_a_string
// Test: cargo test --example lc_0387_first_unique_character_in_a_string
// Bench: cargo test --example lc_0387_first_unique_character_in_a_string bench -- --nocapture

/// My solution module — two-pass frequency count.
///
/// Pass 1: count frequency of each character in a [i32; 26] array.
/// Pass 2: find the first character with count == 1.
///
/// Time:  O(n)
/// Space: O(1) — fixed 26-element array
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the index of the first non-repeating character.
        ///
        /// # Arguments
        ///
        /// * `s` - Input string of lowercase English letters
        ///
        /// # Returns
        ///
        /// Index of first unique character, or -1 if none exists
        pub fn first_uniq_char(s: String) -> i32 {
            let mut counts = [0i32; 26];
            for b in s.bytes() {
                counts[(b - b'a') as usize] += 1;
            }
            for (i, b) in s.bytes().enumerate() {
                if counts[(b - b'a') as usize] == 1 {
                    return i as i32;
                }
            }
            -1
        }
    }
}

/// AI solution module — single-pass recording first occurrence index.
///
/// Store (count, first_index) for each of the 26 characters. After one
/// pass, scan the 26 entries to find the smallest first_index with count 1.
/// Avoids a second pass over the (potentially large) string.
///
/// Time:  O(n + 26) = O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the index of the first non-repeating character.
        ///
        /// # Arguments
        ///
        /// * `s` - Input string of lowercase English letters
        ///
        /// # Returns
        ///
        /// Index of first unique character, or -1 if none exists
        pub fn first_uniq_char(s: String) -> i32 {
            // (count, first_index) for each letter
            let mut info = [(0u32, 0usize); 26];
            for (i, b) in s.bytes().enumerate() {
                let idx = (b - b'a') as usize;
                info[idx].0 += 1;
                if info[idx].0 == 1 {
                    info[idx].1 = i;
                }
            }
            // Find the smallest first_index among characters with count == 1
            let mut result = -1i32;
            let mut min_idx = usize::MAX;
            for &(count, first) in &info {
                if count == 1 && first < min_idx {
                    min_idx = first;
                    result = first as i32;
                }
            }
            result
        }
    }
}

fn main() {
    let cases = [("leetcode", 0), ("loveleetcode", 2), ("aabb", -1)];

    println!("--- My Solution ---");
    for (s, expected) in &cases {
        let result = my_solution::Solution::first_uniq_char(s.to_string());
        println!("  \"{s}\" -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (s, expected) in &cases {
        let result = ai_solution::Solution::first_uniq_char(s.to_string());
        println!("  \"{s}\" -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_first_uniq {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
                }

                #[test]
                fn test_example2() {
                    assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
                }

                #[test]
                fn test_example3() {
                    assert_eq!(Solution::first_uniq_char("aabb".to_string()), -1);
                }

                #[test]
                fn test_single_char() {
                    assert_eq!(Solution::first_uniq_char("z".to_string()), 0);
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(Solution::first_uniq_char("aaaa".to_string()), -1);
                }

                #[test]
                fn test_last_unique() {
                    assert_eq!(Solution::first_uniq_char("aabbc".to_string()), 4);
                }
            }
        };
    }

    test_first_uniq!(my_tests, crate::my_solution::Solution);
    test_first_uniq!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        // All 26 letters repeated — no unique char, exercises full string
        let s: String = "abcdefghijklmnopqrstuvwxyz".repeat(400) + "z";

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::first_uniq_char(
                std::hint::black_box(s.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::first_uniq_char(
                std::hint::black_box(s.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution:  {:.2?} (avg {:.2?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "AI solution:  {:.2?} (avg {:.2?})",
            ai_duration,
            ai_duration / iterations
        );
        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.0 {
            println!("-> AI solution is {ratio:.2}x faster");
        } else {
            println!("-> My solution is {:.2}x faster", 1.0 / ratio);
        }
    }
}
