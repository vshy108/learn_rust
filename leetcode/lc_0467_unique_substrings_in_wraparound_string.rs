// LeetCode 467: Unique Substrings in Wraparound String [Medium]
// https://leetcode.com/problems/unique-substrings-in-wraparound-string/
//
// We define the string base as "...zabcdefghijklmnopqrstuvwxyzabc..." (infinite
// wraparound of lowercase alphabet). Given a string s, return the number of
// unique non-empty substrings of s that are present in base.
//
// A substring is in base if it's a contiguous sequence of consecutive letters
// with wraparound (z -> a).
//
// Examples:
//   "a"     -> 1
//   "cac"   -> 2  ("c", "a")
//   "zab"   -> 6  ("z","a","b","za","ab","zab")
//
// Constraints:
//   1 <= s.len() <= 10^5
//   s consists of lowercase English letters only
//
// Run:  cargo run --example lc_0467_unique_substrings_in_wraparound_string
// Test: cargo test --example lc_0467_unique_substrings_in_wraparound_string
// Bench: cargo test --example lc_0467_unique_substrings_in_wraparound_string bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 66ns
//   AI solution: avg 67ns
//   Ratio: 0.99x (same algorithm)

/// My solution module — DP with max length ending at each char.
///
/// Time:  O(n)
/// Space: O(1) — 26-element array

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_substring_in_wrapround_string(s: String) -> i32 {
            let bytes = s.as_bytes();
            let mut max_len = [0i32; 26];
            let mut cur_len = 0i32;
            for i in 0..bytes.len() {
                let c = (bytes[i] - b'a') as usize;
                if i > 0 && (bytes[i] as i32 - bytes[i - 1] as i32 + 26) % 26 == 1 {
                    cur_len += 1;
                } else {
                    cur_len = 1;
                }
                max_len[c] = max_len[c].max(cur_len);
            }
            max_len.iter().sum()
        }
    }
}

/// AI solution module — track longest consecutive run ending at each letter.
///
/// Key insight: for a consecutive run of length k ending at char c, there are
/// k unique substrings ending at c that are valid. We keep the maximum run
/// length ending at each of the 26 letters, then sum them all.
///
/// Time:  O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_substring_in_wrapround_string(s: String) -> i32 {
            let bytes = s.as_bytes();
            // max_len[c] = longest consecutive wraparound run ending at letter c
            let mut max_len = [0i32; 26];
            let mut cur_len = 0i32;

            for i in 0..bytes.len() {
                let c = (bytes[i] - b'a') as usize;
                if i > 0 && (bytes[i] as i32 - bytes[i - 1] as i32 + 26) % 26 == 1 {
                    cur_len += 1;
                } else {
                    cur_len = 1;
                }
                // Keep maximum — avoids counting duplicates
                max_len[c] = max_len[c].max(cur_len);
            }

            max_len.iter().sum()
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_a() {
            assert_eq!(
                Solution::find_substring_in_wrapround_string("a".to_string()),
                1
            );
        }
        #[test]
        fn test_cac() {
            assert_eq!(
                Solution::find_substring_in_wrapround_string("cac".to_string()),
                2
            );
        }
        #[test]
        fn test_zab() {
            assert_eq!(
                Solution::find_substring_in_wrapround_string("zab".to_string()),
                6
            );
        }
    };
}

#[cfg(test)]
mod tests_my {
    solution_tests!(my_solution);
}
#[cfg(test)]
mod tests_ai {
    solution_tests!(ai_solution);
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(String) -> i32,
{
    let iterations = 500_000;
    let input = "zabcdefghijklmnopqrstuvwxyzabc".to_string();
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(input.clone())));
    }
    let elapsed = start.elapsed();
    println!(
        "{}: {} iterations in {:?} (avg {:?})",
        name,
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution(
        "My solution",
        my_solution::Solution::find_substring_in_wrapround_string,
    );
    bench_solution(
        "AI solution",
        ai_solution::Solution::find_substring_in_wrapround_string,
    );
}

fn main() {
    println!(
        "{}",
        ai_solution::Solution::find_substring_in_wrapround_string("zab".to_string())
    );
}
