// LeetCode 159: Longest Substring with At Most Two Distinct Characters [Medium] (Premium)
// https://leetcode.com/problems/longest-substring-with-at-most-two-distinct-characters/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 12.093us
//   AI solution avg: 1.396us
//   Ratio: AI 8.66x faster
//
// Given a string s, return the length of the longest substring that
// contains at most two distinct characters.
//
// Example 1: s = "eceba" -> 3 (substring "ece")
// Example 2: s = "ccaabbb" -> 5 (substring "aabbb")
//
// Constraints:
//   - 1 <= s.length <= 10^5
//   - s consists of English letters
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0100-0199/0159.Longest%20Substring%20with%20At%20Most%20Two%20Distinct%20Characters/README_EN.md
//
// Run:  cargo run --example lc_0159_longest_substring_two_distinct
// Test: cargo test --example lc_0159_longest_substring_two_distinct
// Bench: cargo test --example lc_0159_longest_substring_two_distinct bench -- --nocapture

/// My solution — sliding window with HashMap (translated from Python in doocs/leetcode).
///
/// Maintain a window [j..=i] with a HashMap counting character frequencies.
/// When the map has more than 2 keys, shrink from the left by decrementing
/// counts and removing zero-count entries.
///
/// Time: O(n) — each character enters and leaves the window at most once.
/// Space: O(1) — the HashMap holds at most 3 entries.
mod my_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Return length of longest substring with at most 2 distinct chars.
        pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {
            let bytes = s.as_bytes();
            let mut cnt: HashMap<u8, i32> = HashMap::new();
            let mut ans = 0;
            let mut j = 0;

            for (i, &c) in bytes.iter().enumerate() {
                *cnt.entry(c).or_insert(0) += 1;
                // Shrink window until at most 2 distinct chars
                while cnt.len() > 2 {
                    let left = bytes[j];
                    let count = cnt.get_mut(&left).unwrap();
                    *count -= 1;
                    if *count == 0 {
                        cnt.remove(&left);
                    }
                    j += 1;
                }
                ans = ans.max(i - j + 1);
            }
            ans as i32
        }
    }
}

/// AI solution — sliding window with fixed-size array.
///
/// Since the input consists of English letters only, use a [i32; 128]
/// array for O(1) lookups instead of HashMap. Track distinct count
/// manually to avoid scanning the array.
///
/// Time: O(n)
/// Space: O(1) — fixed 128-entry array.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return length of longest substring with at most 2 distinct chars.
        pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {
            let bytes = s.as_bytes();
            let mut counts = [0i32; 128];
            let mut distinct = 0;
            let mut ans = 0;
            let mut j = 0;

            for i in 0..bytes.len() {
                let c = bytes[i] as usize;
                if counts[c] == 0 {
                    distinct += 1;
                }
                counts[c] += 1;

                while distinct > 2 {
                    let left = bytes[j] as usize;
                    counts[left] -= 1;
                    if counts[left] == 0 {
                        distinct -= 1;
                    }
                    j += 1;
                }
                ans = ans.max(i - j + 1);
            }
            ans as i32
        }
    }
}

fn main() {
    let cases = [("eceba", 3), ("ccaabbb", 5), ("a", 1)];
    for (s, expected) in &cases {
        let my = my_solution::Solution::length_of_longest_substring_two_distinct(s.to_string());
        let ai = ai_solution::Solution::length_of_longest_substring_two_distinct(s.to_string());
        println!("{s}: my={my}, ai={ai} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_two_distinct {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(
                        <$solution>::length_of_longest_substring_two_distinct("eceba".into()),
                        3,
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        <$solution>::length_of_longest_substring_two_distinct("ccaabbb".into()),
                        5,
                    );
                }

                #[test]
                fn test_single_char() {
                    assert_eq!(
                        <$solution>::length_of_longest_substring_two_distinct("a".into()),
                        1,
                    );
                }

                #[test]
                fn test_two_chars() {
                    assert_eq!(
                        <$solution>::length_of_longest_substring_two_distinct("aabb".into()),
                        4,
                    );
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(
                        <$solution>::length_of_longest_substring_two_distinct("aaaa".into()),
                        4,
                    );
                }
            }
        };
    }

    test_two_distinct!(my, crate::my_solution::Solution);
    test_two_distinct!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        // Repeating pattern with 3+ distinct chars to exercise the window shrinking
        let s: String = "abcabc".repeat(100);

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::length_of_longest_substring_two_distinct(
                    std::hint::black_box(s.clone()),
                ),
            );
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::length_of_longest_substring_two_distinct(
                    std::hint::black_box(s.clone()),
                ),
            );
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else if ai_avg < my_avg {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        } else {
            println!("  Ratio: ~tied");
        }
    }
}
