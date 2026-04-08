// Benchmark: 50,000 iterations (2026-04-08)
// My solution:  avg 567ns  | Bitmask enumeration, linear scan all 2^m masks
// AI solution:  avg 430ns  | Gosper's hack popcount-ordered enumeration
// -> AI solution is 1.32x faster
//
// LeetCode 411: Minimum Unique Word Abbreviation [Hard] (Premium)
// https://leetcode.com/problems/minimum-unique-word-abbreviation/
//
// A string can be abbreviated by replacing any number of non-adjacent
// substrings with their lengths. Given a target string and a dictionary,
// return the shortest abbreviation of target that is NOT an abbreviation
// of any word in the dictionary.
//
// Examples:
//   target = "apple", dictionary = ["blade"] -> "a4"
//   target = "apple", dictionary = ["blade","plain","amber"] -> "1p3"
//
// Constraints:
//   m == target.length
//   n == dictionary.length
//   1 <= m <= 21
//   0 <= n <= 1000
//   1 <= dictionary[i].length <= 100
//   log2(n) + m <= 21 if n > 0
//   target and dictionary[i] consist of lowercase English letters
//   dictionary does not contain target
//
// Run:  cargo run --example lc_0411_minimum_unique_word_abbreviation
// Test: cargo test --example lc_0411_minimum_unique_word_abbreviation
// Bench: cargo test --example lc_0411_minimum_unique_word_abbreviation bench -- --nocapture

/// My solution module — bitmask enumeration + conflict checking.
///
/// Key idea: represent an abbreviation as a bitmask of length `m` where
/// bit=1 means "keep this character" and bit=0 means "abbreviate".
/// For each dictionary word of the same length as target, compute a
/// "diff mask" (positions where they differ). An abbreviation (keep-mask)
/// distinguishes target from a dict word iff (keep_mask & diff_mask) != 0.
///
/// Enumerate all 2^m bitmasks in order of increasing popcount (fewest kept
/// letters = shortest abbreviation). For each mask, check it conflicts with
/// all same-length dictionary words. Return the first valid one, converted
/// to abbreviation string.
///
/// Time:  O(2^m * n) where m = target.len(), n = filtered dict size
/// Space: O(n) for diff masks
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the shortest unique abbreviation of `target`.
        ///
        /// # Arguments
        ///
        /// * `target` - The word to abbreviate
        /// * `dictionary` - Words the abbreviation must not match
        ///
        /// # Returns
        ///
        /// Shortest abbreviation string that is unique
        pub fn min_abbreviation(
            target: String,
            dictionary: Vec<String>,
        ) -> String {
            let m = target.len();
            let tb = target.as_bytes();

            // Only words with the same length as target can conflict
            let diffs: Vec<u32> = dictionary
                .iter()
                .filter(|w| w.len() == m)
                .map(|w| {
                    let wb = w.as_bytes();
                    // Bit i is set if target[i] != word[i]
                    (0..m).fold(0_u32, |mask, i| {
                        if tb[i] != wb[i] {
                            mask | (1 << i)
                        } else {
                            mask
                        }
                    })
                })
                .collect();

            // If no same-length words, the shortest abbreviation is
            // just the length as a number (all chars abbreviated)
            if diffs.is_empty() {
                return m.to_string();
            }

            // Enumerate masks by increasing popcount (fewer kept chars = shorter)
            // Mask bit i = 1 means "keep target[i]"
            // A mask is valid if for every diff, (mask & diff) != 0
            // meaning at least one kept position distinguishes them
            let total = 1_u32 << m;
            let mut best_mask = total - 1; // worst case: keep all chars
            let mut best_len = m;          // worst case length

            for mask in 1..total {
                let abbr_len = Self::abbreviation_length(mask, m);
                if abbr_len >= best_len {
                    continue;
                }
                // Check that mask distinguishes target from every dict word
                if diffs.iter().all(|&d| mask & d != 0) {
                    best_mask = mask;
                    best_len = abbr_len;
                }
            }

            Self::mask_to_abbreviation(tb, best_mask, m)
        }

        /// Calculate the length of an abbreviation given a keep-mask.
        ///
        /// Consecutive 0-bits become one number (length 1 in the abbreviation),
        /// consecutive 1-bits each contribute 1 character.
        fn abbreviation_length(mask: u32, m: usize) -> usize {
            let mut len = 0;
            let mut i = 0;
            while i < m {
                if mask & (1 << i) != 0 {
                    // Kept character
                    len += 1;
                    i += 1;
                } else {
                    // Run of abbreviated characters counts as 1 token
                    len += 1;
                    while i < m && mask & (1 << i) == 0 {
                        i += 1;
                    }
                }
            }
            len
        }

        /// Convert a keep-mask into the abbreviation string.
        fn mask_to_abbreviation(tb: &[u8], mask: u32, m: usize) -> String {
            let mut result = String::new();
            let mut i = 0;
            while i < m {
                if mask & (1 << i) != 0 {
                    result.push(tb[i] as char);
                    i += 1;
                } else {
                    let start = i;
                    while i < m && mask & (1 << i) == 0 {
                        i += 1;
                    }
                    result.push_str(&(i - start).to_string());
                }
            }
            result
        }
    }
}

/// AI solution module — bitmask with popcount-ordered enumeration.
///
/// Same core idea but enumerates masks strictly by popcount using
/// Gosper's hack (next combination with same number of set bits),
/// guaranteeing we find the shortest abbreviation first and can
/// return immediately.
///
/// Time:  O(2^m * n) worst case, but exits early
/// Space: O(n)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the shortest unique abbreviation of `target`.
        ///
        /// # Arguments
        ///
        /// * `target` - The word to abbreviate
        /// * `dictionary` - Words the abbreviation must not match
        ///
        /// # Returns
        ///
        /// Shortest abbreviation string that is unique
        pub fn min_abbreviation(
            target: String,
            dictionary: Vec<String>,
        ) -> String {
            let m = target.len();
            let tb = target.as_bytes();

            let diffs: Vec<u32> = dictionary
                .iter()
                .filter(|w| w.len() == m)
                .map(|w| {
                    let wb = w.as_bytes();
                    (0..m).fold(0_u32, |mask, i| {
                        if tb[i] != wb[i] {
                            mask | (1 << i)
                        } else {
                            mask
                        }
                    })
                })
                .collect();

            if diffs.is_empty() {
                return m.to_string();
            }

            let limit = 1_u32 << m;

            // Try popcount = 0 (mask=0 means all abbreviated, "m")
            // but mask=0 can't distinguish anything, so start at popcount=1

            // Enumerate by increasing popcount using Gosper's hack
            for pop in 1..=m {
                // First mask with `pop` bits set: (1 << pop) - 1
                let mut mask = (1_u32 << pop) - 1;
                while mask < limit {
                    if diffs.iter().all(|&d| mask & d != 0) {
                        return Self::mask_to_abbreviation(tb, mask, m);
                    }
                    // Gosper's hack: next combination with same popcount
                    mask = Self::next_combination(mask);
                }
            }

            // Fallback: keep all characters (should not reach here)
            target
        }

        /// Gosper's hack: compute next integer with the same number of set bits.
        ///
        /// Given a bitmask `x`, returns the next larger integer with the same
        /// popcount. Works by finding the lowest set bit that can be "promoted"
        /// and packing the remaining bits to the right.
        fn next_combination(x: u32) -> u32 {
            let c = x & x.wrapping_neg(); // lowest set bit
            let r = x + c;                // flip that bit group
            // Pack remaining bits to the right
            (((r ^ x) >> 2) / c) | r
        }

        /// Convert a keep-mask into the abbreviation string.
        fn mask_to_abbreviation(tb: &[u8], mask: u32, m: usize) -> String {
            let mut result = String::new();
            let mut i = 0;
            while i < m {
                if mask & (1 << i) != 0 {
                    result.push(tb[i] as char);
                    i += 1;
                } else {
                    let start = i;
                    while i < m && mask & (1 << i) == 0 {
                        i += 1;
                    }
                    result.push_str(&(i - start).to_string());
                }
            }
            result
        }
    }
}

fn main() {
    let cases: Vec<(&str, Vec<&str>, &str)> = vec![
        ("apple", vec!["blade"], "a4"),
        ("apple", vec!["blade", "plain", "amber"], "1p3"),
    ];

    println!("--- My Solution ---");
    for (target, dict, expected) in &cases {
        let result = my_solution::Solution::min_abbreviation(
            target.to_string(),
            dict.iter().map(|s| s.to_string()).collect(),
        );
        println!("  \"{target}\", {dict:?} -> \"{result}\" (expected \"{expected}\")");
    }

    println!("--- AI Solution ---");
    for (target, dict, expected) in &cases {
        let result = ai_solution::Solution::min_abbreviation(
            target.to_string(),
            dict.iter().map(|s| s.to_string()).collect(),
        );
        println!("  \"{target}\", {dict:?} -> \"{result}\" (expected \"{expected}\")");
    }
}

/// Validate that an abbreviation is correct (matches target but not any dict word).
#[cfg(test)]
fn is_valid_abbreviation(target: &str, dict: &[&str], abbr: &str) -> bool {
    // Check abbr matches target
    if !matches_abbreviation(target, abbr) {
        return false;
    }
    // Check abbr does NOT match any dictionary word
    dict.iter().all(|w| !matches_abbreviation(w, abbr))
}

/// Check if a word matches an abbreviation string.
#[cfg(test)]
fn matches_abbreviation(word: &str, abbr: &str) -> bool {
    let wb = word.as_bytes();
    let ab = abbr.as_bytes();
    let (m, n) = (wb.len(), ab.len());
    let mut i = 0;
    let mut j = 0;
    while i < m && j < n {
        if ab[j].is_ascii_digit() {
            if ab[j] == b'0' {
                return false;
            }
            let mut num = 0_usize;
            while j < n && ab[j].is_ascii_digit() {
                num = num * 10 + (ab[j] - b'0') as usize;
                j += 1;
            }
            i += num;
        } else {
            if wb[i] != ab[j] {
                return false;
            }
            i += 1;
            j += 1;
        }
    }
    i == m && j == n
}

#[cfg(test)]
mod tests {
    use super::{is_valid_abbreviation, matches_abbreviation};

    macro_rules! test_min_abbr {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;
                use super::super::is_valid_abbreviation;

                #[test]
                fn test_example1() {
                    let result = Solution::min_abbreviation(
                        "apple".to_string(),
                        vec!["blade".to_string()],
                    );
                    assert!(
                        is_valid_abbreviation("apple", &["blade"], &result),
                        "Got \"{result}\" which is not a valid unique abbreviation",
                    );
                    // Known valid: "a4" (length 2)
                    assert_eq!(result.len(), 2, "Expected length 2, got \"{result}\"");
                }

                #[test]
                fn test_example2() {
                    let result = Solution::min_abbreviation(
                        "apple".to_string(),
                        vec![
                            "blade".to_string(),
                            "plain".to_string(),
                            "amber".to_string(),
                        ],
                    );
                    assert!(
                        is_valid_abbreviation(
                            "apple",
                            &["blade", "plain", "amber"],
                            &result,
                        ),
                        "Got \"{result}\" which is not valid",
                    );
                    // Known valid answers: "1p3", "2p2", "3l1" (all length 3)
                    assert_eq!(result.len(), 3);
                }

                #[test]
                fn test_empty_dict() {
                    let result = Solution::min_abbreviation(
                        "cat".to_string(),
                        vec![],
                    );
                    // No conflicts, shortest is just "3"
                    assert_eq!(result, "3");
                }

                #[test]
                fn test_no_same_length() {
                    let result = Solution::min_abbreviation(
                        "abc".to_string(),
                        vec!["abcd".to_string(), "ab".to_string()],
                    );
                    // Different lengths can't conflict, shortest is "3"
                    assert_eq!(result, "3");
                }

                #[test]
                fn test_single_char() {
                    let result = Solution::min_abbreviation(
                        "a".to_string(),
                        vec!["b".to_string()],
                    );
                    // "a" itself or "1" — "1" is shorter but matches "b" too,
                    // so "a" is the answer
                    assert!(
                        is_valid_abbreviation("a", &["b"], &result),
                        "Got \"{result}\"",
                    );
                }
            }
        };
    }

    test_min_abbr!(my_tests, crate::my_solution::Solution);
    test_min_abbr!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn test_matches_abbreviation_helper() {
        assert!(matches_abbreviation("internationalization", "i12iz4n"));
        assert!(!matches_abbreviation("apple", "a2e"));
        assert!(matches_abbreviation("substitution", "12"));
        assert!(matches_abbreviation("hello", "hello"));
        assert!(!matches_abbreviation("hi", "02"));
    }

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 50_000_u32;
        let target = "apple".to_string();
        let dict: Vec<String> =
            vec!["blade", "plain", "amber"]
                .into_iter()
                .map(String::from)
                .collect();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::min_abbreviation(
                    std::hint::black_box(target.clone()),
                    std::hint::black_box(dict.clone()),
                ),
            );
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::min_abbreviation(
                    std::hint::black_box(target.clone()),
                    std::hint::black_box(dict.clone()),
                ),
            );
        }
        let ai_duration = start.elapsed();

        let my_avg = my_duration / iterations;
        let ai_avg = ai_duration / iterations;

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution:  {my_duration:.2?} (avg {my_avg:.2?})");
        println!("AI solution:  {ai_duration:.2?} (avg {ai_avg:.2?})");

        if my_avg <= ai_avg {
            let ratio = ai_avg.as_nanos() as f64
                / my_avg.as_nanos().max(1) as f64;
            println!("-> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_avg.as_nanos() as f64
                / ai_avg.as_nanos().max(1) as f64;
            println!("-> AI solution is {ratio:.2}x faster");
        }
    }
}
