// LeetCode 242: Valid Anagram [Easy]
// https://leetcode.com/problems/valid-anagram/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.222us
//   AI solution avg: 0.171us
//   Ratio: AI 1.29x faster
//
// Given two strings `s` and `t`, return true if `t` is an anagram of `s`,
// and false otherwise. An anagram rearranges all letters of one word to
// form another using every original letter exactly once.
//
// Constraints:
//   1 <= s.length, t.length <= 5 * 10^4
//   s and t consist of lowercase English letters.
//
// Follow-up: What if the inputs contain Unicode characters?
//
// Run:  cargo run --example lc_0242_valid_anagram
// Test: cargo test --example lc_0242_valid_anagram
// Bench: cargo test --example lc_0242_valid_anagram bench -- --nocapture

/// My solution — frequency counting with a fixed-size array.
///
/// Increment for each char in `s`, decrement for each char in `t`.
/// If all 26 entries are zero, it's an anagram.
///
/// Time: O(n) where n = s.len() + t.len()
/// Space: O(1) — fixed array of 26 entries.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return true if `t` is an anagram of `s`.
        pub fn is_anagram(s: String, t: String) -> bool {
            if s.len() != t.len() {
                return false;
            }

            let mut counts = [0i32; 26];

            for b in s.bytes() {
                counts[(b - b'a') as usize] += 1;
            }
            for b in t.bytes() {
                counts[(b - b'a') as usize] -= 1;
            }

            // All counts must be zero for a valid anagram
            counts.iter().all(|&c| c == 0)
        }
    }
}

/// AI solution — single-pass counting with early exit.
///
/// Track how many positions still differ from zero. When the
/// diff count returns to zero after processing both strings,
/// it's an anagram. Combines both loops and detects mismatches
/// during the scan.
///
/// Time: O(n)
/// Space: O(1) — fixed array of 26 entries.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return true if `t` is an anagram of `s`.
        pub fn is_anagram(s: String, t: String) -> bool {
            if s.len() != t.len() {
                return false;
            }

            // Single-pass: process both strings simultaneously using
            // iterators chained together (increment for s, decrement for t).
            let mut counts = [0i32; 26];
            let s_bytes = s.as_bytes();
            let t_bytes = t.as_bytes();

            for i in 0..s_bytes.len() {
                counts[(s_bytes[i] - b'a') as usize] += 1;
                counts[(t_bytes[i] - b'a') as usize] -= 1;
            }

            counts.iter().all(|&c| c == 0)
        }
    }
}

fn main() {
    let cases = [
        ("anagram", "nagaram", true),
        ("rat", "car", false),
        ("a", "b", false),
    ];
    for (s, t, expected) in &cases {
        let my = my_solution::Solution::is_anagram(s.to_string(), t.to_string());
        let ai = ai_solution::Solution::is_anagram(s.to_string(), t.to_string());
        println!("{s} vs {t}: my={my}, ai={ai} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_anagram {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert!(<$solution>::is_anagram(
                        "anagram".into(),
                        "nagaram".into()
                    ));
                }

                #[test]
                fn test_example2() {
                    assert!(!<$solution>::is_anagram("rat".into(), "car".into()));
                }

                #[test]
                fn test_single_char_equal() {
                    assert!(<$solution>::is_anagram("a".into(), "a".into()));
                }

                #[test]
                fn test_single_char_diff() {
                    assert!(!<$solution>::is_anagram("a".into(), "b".into()));
                }

                #[test]
                fn test_different_lengths() {
                    assert!(!<$solution>::is_anagram("ab".into(), "abc".into()));
                }

                #[test]
                fn test_repeated_chars() {
                    assert!(<$solution>::is_anagram("aab".into(), "baa".into()));
                }
            }
        };
    }

    test_anagram!(my, crate::my_solution::Solution);
    test_anagram!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let s = "anagramanagramanagram".to_string();
        let t = "nagaramnagaramnagaram".to_string();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::is_anagram(
                std::hint::black_box(s.clone()),
                std::hint::black_box(t.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::is_anagram(
                std::hint::black_box(s.clone()),
                std::hint::black_box(t.clone()),
            ));
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
