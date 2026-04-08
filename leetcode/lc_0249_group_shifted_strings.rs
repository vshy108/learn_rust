// LeetCode 249: Group Shifted Strings [Medium] (Premium)
// https://leetcode.com/problems/group-shifted-strings/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 2.663us
//   AI solution avg: 2.348us
//   Ratio: AI 1.13x faster
//
// Group strings that belong to the same shifting sequence.
// A string can be shifted right (a->b, z->a) or left (b->a, a->z).
// e.g. "abc" and "bcd" and "xyz" all belong to the same group.
//
// Example 1:
//   Input: ["abc","bcd","acef","xyz","az","ba","a","z"]
//   Output: [["acef"],["a","z"],["abc","bcd","xyz"],["az","ba"]]
//
// Example 2:
//   Input: ["a"]
//   Output: [["a"]]
//
// Constraints:
//   - 1 <= strings.length <= 200
//   - 1 <= strings[i].length <= 50
//   - strings[i] consists of lowercase English letters
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0249.Group%20Shifted%20Strings/README_EN.md
//
// Run:  cargo run --example lc_0249_group_shifted_strings
// Test: cargo test --example lc_0249_group_shifted_strings
// Bench: cargo test --example lc_0249_group_shifted_strings bench -- --nocapture

/// My solution — normalize by shifting first char to 'a' (translated from Python in doocs).
///
/// For each string, compute the shift needed to bring its first character
/// to 'a', apply that shift to all characters (wrapping mod 26), and use
/// the normalized string as the hash key.
///
/// Time: O(L) where L = total length of all strings
/// Space: O(L)
mod my_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Group shifted strings together.
        pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
            let mut g: HashMap<String, Vec<String>> = HashMap::new();
            for s in strings {
                let bytes = s.as_bytes();
                // Shift amount to normalize first char to 'a'
                let diff = bytes[0] - b'a';
                let key: String = bytes
                    .iter()
                    .map(|&c| {
                        let mut shifted = c.wrapping_sub(diff);
                        // Wrap around if we went below 'a'
                        if shifted < b'a' {
                            shifted += 26;
                        }
                        shifted as char
                    })
                    .collect();
                g.entry(key).or_default().push(s);
            }
            g.into_values().collect()
        }
    }
}

/// AI solution — use difference tuple as key.
///
/// Instead of normalizing the string, compute the sequence of
/// character-to-character differences (mod 26) as the grouping key.
/// This avoids string allocation for the key — uses a Vec<u8> instead.
///
/// Time: O(L)
/// Space: O(L)
mod ai_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Group shifted strings together.
        pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
            let mut g: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
            for s in strings {
                let bytes = s.as_bytes();
                // Key = sequence of differences between consecutive chars mod 26
                // Single-char strings all map to empty vec (same group)
                let key: Vec<u8> = bytes
                    .windows(2)
                    .map(|w| (w[1] as i16 - w[0] as i16).rem_euclid(26) as u8)
                    .collect();
                g.entry(key).or_default().push(s);
            }
            g.into_values().collect()
        }
    }
}

fn main() {
    let strs: Vec<String> = vec!["abc", "bcd", "acef", "xyz", "az", "ba", "a", "z"]
        .into_iter()
        .map(String::from)
        .collect();
    let mut my_result = my_solution::Solution::group_strings(strs.clone());
    for g in &mut my_result {
        g.sort();
    }
    my_result.sort_by(|a, b| a[0].cmp(&b[0]));
    println!("my: {my_result:?}");

    let mut ai_result = ai_solution::Solution::group_strings(strs);
    for g in &mut ai_result {
        g.sort();
    }
    ai_result.sort_by(|a, b| a[0].cmp(&b[0]));
    println!("ai: {ai_result:?}");
}

#[cfg(test)]
mod tests {
    /// Normalize groups for comparison: sort within each group,
    /// then sort groups by their first element.
    fn normalize(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for g in &mut groups {
            g.sort();
        }
        groups.sort_by(|a, b| a[0].cmp(&b[0]));
        groups
    }

    fn make(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    macro_rules! test_group_shifted {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::{make, normalize};
                use $solution as Sol;

                #[test]
                fn test_example_1() {
                    let result = normalize(Sol::group_strings(make(&[
                        "abc", "bcd", "acef", "xyz", "az", "ba", "a", "z",
                    ])));
                    let expected = normalize(vec![
                        make(&["acef"]),
                        make(&["a", "z"]),
                        make(&["abc", "bcd", "xyz"]),
                        make(&["az", "ba"]),
                    ]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_example_2() {
                    let result = normalize(Sol::group_strings(make(&["a"])));
                    assert_eq!(result, vec![make(&["a"])]);
                }

                #[test]
                fn test_all_same_shift() {
                    let result = normalize(Sol::group_strings(make(&["a", "b", "c", "z"])));
                    let expected = normalize(vec![make(&["a", "b", "c", "z"])]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_wrap_around() {
                    // "az" has diff 25, "ba" has diff 25 (wrapping) -> same group
                    let result = normalize(Sol::group_strings(make(&["az", "ba"])));
                    assert_eq!(result, vec![make(&["az", "ba"])]);
                }

                #[test]
                fn test_different_lengths() {
                    let result = normalize(Sol::group_strings(make(&["ab", "abc"])));
                    let expected = normalize(vec![make(&["ab"]), make(&["abc"])]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_empty_like() {
                    // Single chars all group together
                    let result = normalize(Sol::group_strings(make(&["a", "m", "z"])));
                    assert_eq!(result, vec![make(&["a", "m", "z"])]);
                }
            }
        };
    }

    test_group_shifted!(my, crate::my_solution::Solution);
    test_group_shifted!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let strs: Vec<String> = vec![
            "abc", "bcd", "acef", "xyz", "az", "ba", "a", "z", "def", "efg", "bdeg", "yza", "by",
            "cb",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::group_strings(strs.clone()));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::group_strings(strs.clone()));
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
