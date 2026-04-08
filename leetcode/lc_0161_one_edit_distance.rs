// LeetCode 161: One Edit Distance [Medium] (Premium)
// https://leetcode.com/problems/one-edit-distance/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.170us
//   AI solution avg: 0.134us
//   Ratio: AI 1.27x faster
//
// Given two strings s and t, return true if they are both one edit distance
// apart, otherwise return false.
//
// One edit distance means exactly one of:
//   - Insert exactly one character into s to get t
//   - Delete exactly one character from s to get t
//   - Replace exactly one character in s to get t
//
// Example 1: s = "ab", t = "acb" -> true (insert 'c')
// Example 2: s = "cab", t = "ad" -> false
//
// Constraints:
//   - 0 <= s.length, t.length <= 10^4
//   - s and t consist of lowercase letters, uppercase letters, and digits
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0100-0199/0161.One%20Edit%20Distance/README_EN.md
//
// Run:  cargo run --example lc_0161_one_edit_distance
// Test: cargo test --example lc_0161_one_edit_distance
// Bench: cargo test --example lc_0161_one_edit_distance bench -- --nocapture

/// My solution — case analysis (translated from Python in doocs/leetcode).
///
/// Ensure m >= n (swap if needed). If m - n > 1, return false.
/// Scan chars; on first mismatch: if same length, compare remaining
/// substrings; if different length, compare s[i+1..] with t[i..].
/// If no mismatch found, return true only if m == n + 1.
///
/// Time: O(n) where n = min(s.len(), t.len())
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return true if s and t are exactly one edit apart.
        pub fn is_one_edit_distance(s: String, t: String) -> bool {
            let (s, t) = if s.len() >= t.len() { (s, t) } else { (t, s) };
            let (m, n) = (s.len(), t.len());

            if m - n > 1 {
                return false;
            }

            let sb = s.as_bytes();
            let tb = t.as_bytes();

            for i in 0..n {
                if sb[i] != tb[i] {
                    // Same length: compare rest after this position
                    // Different length: skip one char in s
                    return if m == n {
                        sb[i + 1..] == tb[i + 1..]
                    } else {
                        sb[i + 1..] == tb[i..]
                    };
                }
            }
            // All matched so far — valid only if s has one extra char
            m == n + 1
        }
    }
}

/// AI solution — single-pass with index tracking.
///
/// Walk both strings with two pointers. On the first mismatch,
/// advance only the longer string's pointer (or both if equal length).
/// If a second mismatch is found, return false. At the end, check
/// that exactly one edit was performed.
///
/// Time: O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return true if s and t are exactly one edit apart.
        pub fn is_one_edit_distance(s: String, t: String) -> bool {
            let sb = s.as_bytes();
            let tb = t.as_bytes();
            let (m, n) = (sb.len(), tb.len());

            if (m as i32 - n as i32).unsigned_abs() as usize > 1 {
                return false;
            }

            let mut i = 0;
            let mut j = 0;
            let mut edits = 0;

            while i < m && j < n {
                if sb[i] != tb[j] {
                    edits += 1;
                    if edits > 1 {
                        return false;
                    }
                    if m > n {
                        // Delete from s: advance only i
                        i += 1;
                        continue;
                    } else if n > m {
                        // Insert into s (= delete from t): advance only j
                        j += 1;
                        continue;
                    }
                    // Same length: replace — advance both (fall through)
                }
                i += 1;
                j += 1;
            }

            // Account for trailing extra character
            edits += (m - i) + (n - j);
            edits == 1
        }
    }
}

fn main() {
    let cases = [
        ("ab", "acb", true),
        ("cab", "ad", false),
        ("", "a", true),
        ("a", "a", false),
    ];
    for (s, t, expected) in &cases {
        let my = my_solution::Solution::is_one_edit_distance(s.to_string(), t.to_string());
        let ai = ai_solution::Solution::is_one_edit_distance(s.to_string(), t.to_string());
        println!("{s:?} vs {t:?}: my={my}, ai={ai} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_one_edit {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_insert() {
                    assert!(<$solution>::is_one_edit_distance("ab".into(), "acb".into()));
                }

                #[test]
                fn test_not_one_edit() {
                    assert!(!<$solution>::is_one_edit_distance(
                        "cab".into(),
                        "ad".into()
                    ));
                }

                #[test]
                fn test_empty_to_one() {
                    assert!(<$solution>::is_one_edit_distance("".into(), "a".into()));
                }

                #[test]
                fn test_same_string() {
                    assert!(!<$solution>::is_one_edit_distance("a".into(), "a".into()));
                }

                #[test]
                fn test_replace() {
                    assert!(<$solution>::is_one_edit_distance(
                        "abc".into(),
                        "adc".into()
                    ));
                }

                #[test]
                fn test_delete() {
                    assert!(<$solution>::is_one_edit_distance("abc".into(), "ab".into()));
                }

                #[test]
                fn test_both_empty() {
                    assert!(!<$solution>::is_one_edit_distance("".into(), "".into()));
                }

                #[test]
                fn test_length_diff_two() {
                    assert!(!<$solution>::is_one_edit_distance("a".into(), "abc".into()));
                }
            }
        };
    }

    test_one_edit!(my, crate::my_solution::Solution);
    test_one_edit!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let s = "abcdefghijklmnopqrstuvwxyz".to_string();
        let t = "abcdefghijklmnXpqrstuvwxyz".to_string();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::is_one_edit_distance(
                std::hint::black_box(s.clone()),
                std::hint::black_box(t.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::is_one_edit_distance(
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
