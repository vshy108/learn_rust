// LeetCode 49: Group Anagrams [Medium]
// https://leetcode.com/problems/group-anagrams/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 13.842us
//   AI solution avg: 12.605us
//   Ratio: ~tied
//
// Given an array of strings `strs`, group the anagrams together.
// You can return the answer in any order.
//
// Example 1: strs = ["eat","tea","tan","ate","nat","bat"]
//            -> [["bat"],["nat","tan"],["ate","eat","tea"]]
// Example 2: strs = [""] -> [[""]]
// Example 3: strs = ["a"] -> [["a"]]
//
// Constraints:
//   1 <= strs.length <= 10^4
//   0 <= strs[i].length <= 100
//   strs[i] consists of lowercase English letters
//
// Run:  cargo run --example lc_0049_group_anagrams
// Test: cargo test --example lc_0049_group_anagrams
// Bench: cargo test --example lc_0049_group_anagrams bench -- --nocapture

/// My solution — sort each word as the hash key.
///
/// For each string, sort its characters to create a canonical key.
/// Group strings that share the same sorted key.
///
/// Time: O(n * k log k) where n = number of strings, k = max string length.
/// Space: O(n * k) for the HashMap.
mod my_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Group anagrams from the input strings.
        ///
        /// # Arguments
        /// * `strs` - vector of strings
        ///
        /// # Returns
        /// Groups of anagrams
        pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
            let mut map: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
            for s in strs {
                // Sort bytes of the string to create a canonical key
                let mut key: Vec<u8> = s.bytes().collect();
                key.sort_unstable();
                map.entry(key).or_default().push(s);
            }
            map.into_values().collect()
        }
    }
}

/// AI solution — character-count array as key (avoids sorting).
///
/// Use a fixed [u8; 26] frequency array as the key. This avoids
/// the O(k log k) sort per string, bringing per-string cost to O(k).
///
/// Time: O(n * k) where n = number of strings, k = max string length.
/// Space: O(n * k) for the HashMap.
mod ai_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Group anagrams from the input strings.
        ///
        /// # Arguments
        /// * `strs` - vector of strings
        ///
        /// # Returns
        /// Groups of anagrams
        pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
            let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();
            for s in strs {
                let mut count = [0u8; 26];
                for b in s.bytes() {
                    count[(b - b'a') as usize] += 1;
                }
                map.entry(count).or_default().push(s);
            }
            map.into_values().collect()
        }
    }
}

fn main() {
    let strs: Vec<String> = ["eat", "tea", "tan", "ate", "nat", "bat"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let my = my_solution::Solution::group_anagrams(strs.clone());
    let ai = ai_solution::Solution::group_anagrams(strs);
    println!("my:{my:?}");
    println!("ai:{ai:?}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_group_anagrams {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                fn sorted_groups(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
                    for g in &mut groups {
                        g.sort();
                    }
                    groups.sort();
                    groups
                }

                fn to_strings(v: &[&str]) -> Vec<String> {
                    v.iter().map(|s| s.to_string()).collect()
                }

                #[test]
                fn test_basic() {
                    let strs = to_strings(&["eat", "tea", "tan", "ate", "nat", "bat"]);
                    let result = sorted_groups(<$solution>::group_anagrams(strs));
                    let expected = sorted_groups(vec![
                        to_strings(&["ate", "eat", "tea"]),
                        to_strings(&["nat", "tan"]),
                        to_strings(&["bat"]),
                    ]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_empty_string() {
                    let strs = to_strings(&[""]);
                    let result = <$solution>::group_anagrams(strs);
                    assert_eq!(result, vec![to_strings(&[""])]);
                }

                #[test]
                fn test_single() {
                    let strs = to_strings(&["a"]);
                    let result = <$solution>::group_anagrams(strs);
                    assert_eq!(result, vec![to_strings(&["a"])]);
                }

                #[test]
                fn test_no_anagrams() {
                    let strs = to_strings(&["abc", "def", "ghi"]);
                    let result = sorted_groups(<$solution>::group_anagrams(strs));
                    assert_eq!(result.len(), 3);
                    for g in &result {
                        assert_eq!(g.len(), 1);
                    }
                }

                #[test]
                fn test_all_anagrams() {
                    let strs = to_strings(&["abc", "bca", "cab"]);
                    let result = <$solution>::group_anagrams(strs);
                    assert_eq!(result.len(), 1);
                    assert_eq!(result[0].len(), 3);
                }
            }
        };
    }

    test_group_anagrams!(my, crate::my_solution::Solution);
    test_group_anagrams!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let strs: Vec<String> = [
            "eat", "tea", "tan", "ate", "nat", "bat", "listen", "silent", "hello", "world", "abc",
            "bca", "cab", "xyz", "zyx",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::group_anagrams(
                std::hint::black_box(strs.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::group_anagrams(
                std::hint::black_box(strs.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg ({:.0}ms total)",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            my_duration.as_millis(),
        );
        println!(
            "AI solution: {:.3}us avg ({:.0}ms total)",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            ai_duration.as_millis(),
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("~tied (ratio: {ratio:.2})");
        }
    }
}
